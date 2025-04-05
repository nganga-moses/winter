import json
from agent.llm.llm_router import LLMRouter
from agent.llm.models.prompter import PromptAssembler
from agent.services.context_manager import ContextManager
from agent.storage.project_manager import ProjectManager


class RequirementsAgent:
    def __init__(self, project: ProjectManager, llm_mode: str = "local", model: str = "mistral"):
        self.project = project
        self.context = ContextManager(project)
        self.llm = LLMRouter(mode=llm_mode, model=model)
        self.prompter = PromptAssembler(role="requirements")

    def execute(self, task: str, preview: bool = False, thread_id: str = None):
        existing = self.project.load_requirements()
        if existing:
            yield {
                "text": "🧠 Retrieving existing requirements...",
                "phase": "requirements",
                "threadId": thread_id
            }
            yield from self._summarize(existing, thread_id)
            return

        # Step 1: Get raw draft from prompt
        context = self.context.get_context_for_task(task)
        prompt = self.prompter.assemble(task, context)

        yield {
            "text": "🧠 Generating raw requirements...",
            "phase": "requirements",
            "threadId": thread_id
        }

        agent_reply = ""
        for chunk in self.llm.complete(prompt, stream=True):
            agent_reply += chunk
            yield {
                "text": chunk,
                "phase": "requirements",
                "threadId": thread_id
            }

        # Step 2: Ask LLM to extract structured list
        yield {
            "text": "🔍 Structuring requirements...",
            "phase": "requirements",
            "threadId": thread_id
        }

        list_prompt = f"""
        Extract a JSON list of requirements from the text below. Each item should have a title and a short description.

        ###
        {agent_reply.strip()}
        ###
        Output format:
        [
            {{
                "title": "...",
                "description": "..."
            }},
            ...
        ]
        """
        final_json = self.llm.complete(list_prompt).strip()

        try:
            parsed = json.loads(final_json)
            if isinstance(parsed, list):
                if not preview:
                    self.project.save_requirements(parsed)
                    self.project.save_chat(task, [
                        {"role": "user", "content": task},
                        {"role": "agent", "content": agent_reply},
                        {"role": "agent", "content": json.dumps(parsed, indent=2)}
                    ])

                yield {
                    "text": "✅ Structured requirements saved.",
                    "phase": "requirements",
                    "threadId": thread_id,
                    "summary": task[:50]
                }

                if not preview:
                    yield {
                        "text": "[🔁 Moving to Architecture Generation...]",
                        "phase": "requirements",
                        "threadId": thread_id
                    }

                yield from self._summarize(parsed, thread_id)

            else:
                raise ValueError("Parsed output is not a list")

        except Exception as e:
            yield {
                "text": f"❌ Failed to structure requirements: {str(e)}",
                "phase": "requirements",
                "threadId": thread_id
            }

    def _summarize(self, reqs: list, thread_id: str = None):
        for i, r in enumerate(reqs, 1):
            text = f"**{i}. {r.get('title', 'Untitled')}**\n- {r.get('description', '')}"
            yield {
                "text": text,
                "phase": "requirements",
                "threadId": thread_id
            }