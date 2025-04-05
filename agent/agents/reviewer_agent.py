import os
import json
import re

from agent.agents.dependency_graph_agent import DependencyGraphAgent
from agent.services.context_manager import ContextManager
from agent.llm.llm_router import LLMRouter
from agent.llm.models.prompter import PromptAssembler
from agent.services.dependency_graph import DependencyGraph
from agent.storage.project_manager import ProjectManager


class ReviewerAgent:
    def __init__(self, project: ProjectManager, llm_mode: str = "local", model: str = "mistral"):
        self.project = project
        self.context_manager = ContextManager(project)
        self.llm = LLMRouter(mode=llm_mode, model=model)
        self.dependency_graph = DependencyGraph(project.path)
        self.dependency_graph.load_all(project.path)
        self.prompter = PromptAssembler(role="reviewer")

    def execute(self, task: str, diff: str = "", file_hint: str = None, preview: bool = False):
        yield f"🔍 Starting code review for `{file_hint or 'unknown file'}`..."

        context = self.context_manager.get_context_for_task(task, file_hint)

        if file_hint:
            related = self.dependency_graph.find_related_to(file_hint)
            context["related"] = related

        context["hotspots"] = self._inject_hotspots_into_context(self.project)

        prompt = self.prompter.assemble(task, context, diff or "")

        feedback = ""
        for chunk in self.llm.complete(prompt, stream=True):
            feedback += chunk
            if preview:
                yield chunk  # stream feedback text if preview is requested

        # Save diff file for record
        if diff:
            self.project.save_diff(task, diff)

        # Save the feedback into chat history
        self.project.save_chat(task, [
            {"role": "user", "content": diff or task},
            {"role": "agent", "content": feedback}
        ])

        # Parse actionable feedback (very naive approach for now)
        extracted = self._parse_structured_feedback(feedback)
        if extracted:
            self.project.add_to_feedback_queue(extracted)

        yield "✅ Code review complete."

    def _parse_structured_feedback(self, text: str):
        pattern = r"- \[ \] (Fix|Improve): (.+?) \(file: (.+?)\)"
        matches = re.findall(pattern, text)
        return [{"task": f"{kind}: {desc}", "file_hint": path} for kind, desc, path in matches]



    def _inject_hotspots_into_context(self, project: ProjectManager):
        agent = DependencyGraphAgent(project)
        top = agent.find_hotspots("call", top_n=5)
        return [f"{node} (called {count}x)" for node, count in top]