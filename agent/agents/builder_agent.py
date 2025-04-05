import os
import json
import difflib

from agent.agents.dependency_graph_agent import DependencyGraphAgent
from agent.agents.utils.git_utils import init_git_repo, commit_all_changes

from agent.services.context_manager import ContextManager
from agent.llm.llm_router import LLMRouter
from agent.llm.models.prompter import PromptAssembler
from agent.services.dependency_graph import DependencyGraph
from agent.storage.project_manager import ProjectManager
from agent.agents.reviewer_agent import ReviewerAgent


class BuilderAgent:
    def __init__(self, project: ProjectManager, llm_mode: str = "local", model: str = "mistral"):
        self.project = project
        self.context_manager = ContextManager(project)
        self.llm = LLMRouter(mode=llm_mode, model=model)
        self.prompter = PromptAssembler(role="builder")
        self.reviewer = ReviewerAgent(project, llm_mode, model)
        self.dependency_graph = DependencyGraph(project.id)

    def execute(self, task: str, file_hint: str = None, preview: bool = False, loop_mode: bool = False):
        if loop_mode:
            yield from self._run_feedback_loop(preview)
        else:
            yield from self._run_single_task(task, file_hint, preview)

    def _run_single_task(self, task: str, file_hint: str, preview: bool):
        yield f"🧠 Starting task: {task}"

        context = self.context_manager.get_context_for_task(task, file_hint)

        if file_hint:
            related = self.dependency_graph.find_related_to(file_hint)
            context["related"] = related

        context["hotspots"] = self._inject_hotspots_into_context(self.project)

        prompt = self.prompter.assemble(task, context)
        response_stream = self.llm.complete(prompt, stream=True)

        file_path = file_hint or "generated_code.py"
        generated_code = ""

        yield f"🧱 Generating code for `{file_path}`..."
        for chunk in response_stream:
            generated_code += chunk
            if preview:
                yield chunk

        # 🔍 Generate diff before overwriting
        diff = self.project.generate_diff(file_path, generated_code)

        # 💾 Write the new file
        self.project.write_file(file_path, generated_code)
        yield f"✅ Code written to `{file_path}`"

        # Commit and push changes via Git
        yield from self._git_commit_and_push(file_path, task)

        # 💬 Save to chat
        self.project.save_chat(task, [
            {"role": "user", "content": task},
            {"role": "agent", "content": f"(code written to {file_path})"}
        ])

        # 🔁 Pass diff to reviewer
        if not preview and diff.strip():
            yield "status:reviewing"
            yield "🔍 Reviewing changes..."
            for chunk in self.reviewer.execute(task, diff=diff, file_hint=file_path, preview=preview):
                yield chunk

    def _run_feedback_loop(self, preview: bool):
        yield "🔄 Entering Code Review mode..."
        queue_path = os.path.join(self.project.path, "feedback_queue.json")

        if not os.path.exists(queue_path):
            yield "🟡 No pending code reviews found."
            return

        with open(queue_path, "r") as f:
            queue = json.load(f)

        if not queue:
            yield "🟢 Code Review queue is empty. Nothing to fix."
            return

        while queue:
            task_item = queue.pop(0)
            task = task_item["task"]
            file_hint = task_item.get("file_hint")

            yield f"🧠 Addressing Code Review feedback: {task}"
            yield from self._run_single_task(task, file_hint, preview)

        with open(queue_path, "w") as f:
            json.dump(queue, f, indent=2)

        yield "✅ Code Review complete."

    def _generate_diff(self, old: str, new: str, file_path: str):
        old_lines = old.splitlines() if old else []
        new_lines = new.splitlines()

        diff = difflib.unified_diff(
            old_lines, new_lines, fromfile=f"{file_path} (old)", tofile=f"{file_path} (new)", lineterm=""
        )
        return "\n".join(diff)

    def _git_commit_and_push(self, file_path: str, task: str):
        try:
            init_git_repo(self.project.path)  # Only initializes if not already
            commit_all_changes(self.project.path, f"🔨 Update: {task}")
            yield "🚀 Changes committed locally."
        except Exception as e:
            yield f"❌ Git commit error: {e}"

    def _inject_hotspots_into_context(self,project: ProjectManager):
        agent = DependencyGraphAgent(project)
        top = agent.find_hotspots("call", top_n=5)
        return [f"{node} (called {count}x)" for node, count in top]