import os
import json
from agent.storage.project_manager import ProjectManager
from agent.services.context_manager import ContextManager
from agent.llm.llm_router import LLMRouter
from agent.llm.models.prompter import PromptAssembler
from agent.agents.utils.git_utils import init_git_repo

class ScaffolderAgent:
    def __init__(self, project: ProjectManager, llm_mode: str = "local", model: str = "mistral"):
        self.project = project
        self.context = ContextManager(project)
        self.llm = LLMRouter(mode=llm_mode, model=model)
        self.prompter = PromptAssembler(role="scaffolder")

    def execute(self, task: str = "Generate folder structure", preview: bool = False, thread_id: str = None):
        yield {
            "text": "🧱 Creating project structure...",
            "phase": "scaffolding",
            "threadId": thread_id
        }

        context = self.context.get_context_for_task(task)
        prompt = self.prompter.assemble(task, context)

        output = ""
        for chunk in self.llm.complete(prompt, stream=True):
            output += chunk
            if preview:
                yield {
                    "text": chunk,
                    "phase": "scaffolding",
                    "threadId": thread_id
                }

        try:
            start = output.find("{")
            end = output.rfind("}") + 1
            structure = json.loads(output[start:end])

            if not preview:
                self.project.save_structure(structure)
                self._create_from_structure(structure)

            yield {
                "text": "✅ Structure saved & folders created.",
                "phase": "scaffolding",
                "threadId": thread_id,
                "summary": "Directory and base files ready."
            }

            yield from self._summarize(structure, thread_id)

        except Exception as e:
            yield {
                "text": f"❌ Failed to parse structure: {e}",
                "phase": "scaffolding",
                "threadId": thread_id
            }
            if preview:
                yield {
                    "text": output,
                    "phase": "scaffolding",
                    "threadId": thread_id
                }

    def _create_from_structure(self, structure: dict, base_path: str = None):
        if base_path is None:
            base_path = self.project.path

        def walk_and_create(path, node):
            for key, value in node.items():
                new_path = os.path.join(path, key)
                os.makedirs(new_path, exist_ok=True)
                if isinstance(value, dict):
                    walk_and_create(new_path, value)
                elif isinstance(value, list):
                    for f in value:
                        open(os.path.join(new_path, f), "a").close()

        walk_and_create(base_path, structure)
        init_git_repo(base_path)  # 💥 Git init happens once structure is set up

    def _summarize(self, structure: dict, thread_id: str = None):
        count_dirs = 0
        count_files = 0

        def walk(node):
            nonlocal count_dirs, count_files
            for key, value in node.items():
                count_dirs += 1
                if isinstance(value, dict):
                    walk(value)
                elif isinstance(value, list):
                    count_files += len(value)

        walk(structure)

        yield {
            "text": f"📁 {count_dirs} folders, 📄 {count_files} files created.",
            "phase": "scaffolding",
            "threadId": thread_id
        }