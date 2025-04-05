import json
from agent.services.context_manager import ContextManager
from agent.llm.llm_router import LLMRouter
from agent.llm.models.prompter import PromptAssembler
from agent.storage.project_manager import ProjectManager


class ArchitectureAgent:
    def __init__(self, project: ProjectManager, llm_mode: str = "local", model: str = "mistral"):
        self.project = project
        self.context = ContextManager(project)
        self.llm = LLMRouter(mode=llm_mode, model=model)
        self.prompter = PromptAssembler(role="architect")

    def execute(self, task: str = "Generate architecture", preview: bool = False, thread_id: str = None):
        yield {
            "text": "🏗️ Generating system architecture...",
            "phase": "architecture",
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
                    "phase": "architecture",
                    "threadId": thread_id
                }

        try:
            start = output.find("{")
            end = output.rfind("}")
            parsed = json.loads(output[start:end])

            if not preview:
                self.project.save_architecture(parsed)
                self.project.save_chat(task, [
                    {"role": "user", "content": task},
                    {"role": "agent", "content": output}
                ])

            yield {
                "text": "✅ Architecture saved.",
                "phase": "architecture",
                "threadId": thread_id,
                "summary": "System structure and modules defined."
            }

            yield from self._summarize(parsed, thread_id)

        except Exception as e:
            yield {
                "text": f"❌ Failed to parse architecture: {e}",
                "phase": "architecture",
                "threadId": thread_id
            }
            if preview:
                yield {
                    "text": output,
                    "phase": "architecture",
                    "threadId": thread_id
                }

    def _summarize(self, arch: dict, thread_id: str = None):
        parts = []

        if "services" in arch:
            parts.append(f"🧩 Services: {', '.join(arch['services'])}")
        if "modules" in arch:
            parts.append(f"📦 Modules: {', '.join(arch['modules'])}")
        if "database" in arch:
            parts.append(f"🗃️ Database: {arch['database']}")
        if "notes" in arch:
            parts.append(f"📝 Notes: {arch['notes']}")

        for line in parts:
            yield {
                "text": line,
                "phase": "architecture",
                "threadId": thread_id
            }