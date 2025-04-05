from typing import Optional

from agent.agents.dependency_graph_agent import DependencyGraphAgent
from agent.agents.project_assistant import ScopedAssistantAgent
from agent.agents.requirements_agent import RequirementsAgent
from agent.agents.architecture_agent import ArchitectureAgent
from agent.agents.scaffolder_agent import ScaffolderAgent
from agent.agents.builder_agent import BuilderAgent
from agent.agents.reviewer_agent import ReviewerAgent
from agent.agents.interpreter_agent import ChatInterpreter
from agent.services.dependency_graph import DependencyGraph
from agent.storage.project_manager import ProjectManager


class OrchestratorAgent:
    def __init__(self, project: ProjectManager):

        self.project = project
        self.requirements_agent = RequirementsAgent(project)
        self.architecture_agent = ArchitectureAgent(project)
        self.scaffolder_agent = ScaffolderAgent(project)
        self.builder_agent = BuilderAgent(project)
        self.reviewer_agent = ReviewerAgent(project)
        self.interpreter = ChatInterpreter()
        self.dependency_agent = DependencyGraphAgent(project)
        self.dependency_graph = DependencyGraph(project.path)

    def execute(self, user_message: str, preview: bool = False, thread_id: Optional[str] = None):
        yield "status:interpreting"
        interpretation = self.interpreter.interpret(user_message)

        intent = interpretation.get("intent", "unknown")
        yield "status:thinking"

        match intent:
            case "bootstrap":
                yield from self._handle_bootstrap(preview=preview)
            case "initialize_project":
                yield from self._handle_initialize_project(interpretation, preview)
            case "add_requirement":
                yield from self._handle_add_requirement(interpretation, preview)
            case "refine_requirement":
                yield from self._handle_refine_requirement(interpretation, preview)
            case "update_architecture":
                yield from self._handle_update_architecture(interpretation, preview)
            case "feedback_loop":
                yield from self._handle_feedback_loop(interpretation, preview)
            case "report_error":
                yield from self._handle_report_error(interpretation, preview)
            case "ask_for_help":
                yield from self._handle_ask_for_help(interpretation, preview)
            case _:
                yield "status:uncertain"
                yield from "🤷 Sorry, I couldn't understand your request. Try rephrasing?"

    # === INTENT HANDLERS ===

    def _handle_initialize_project(self, data: dict, preview: bool):
        project = data.get("project", {})
        name = project.get("name", "Unnamed Project")
        desc = project.get("description", "")

        yield f"📁 Starting project: **{name}**"
        yield f"📝 Description: {desc}"

        # Use description as initial task for requirements
        task = f"Project: {name}\n\n{desc}"
        yield "status:requirements"
        for chunk in self.requirements_agent.execute(task, preview=preview):
            yield chunk

    def _handle_add_requirement(self, data: dict, preview: bool):
        req = data.get("requirement", {})
        title = req.get("title", "")
        desc = req.get("description", "")
        task = f"{title}\n\n{desc}"

        yield f"status:requirements"
        yield f"📄 Adding requirement: {title}"
        for chunk in self.requirements_agent.execute(task, preview=preview):
            yield chunk

        yield f"status:architecture"
        yield "🏗️ Generating architecture..."
        for chunk in self.architecture_agent.execute(preview=preview):
            yield chunk

        yield f"status:scaffolding"
        yield "🧱 Updating project scaffold..."
        for chunk in self.scaffolder_agent.execute(preview=preview):
            yield chunk

        yield f"status:building"
        yield "💻 Building new code..."
        for chunk in self.builder_agent.execute(task, preview=preview):
            yield chunk

        yield "status:analyzing"
        yield from self.dependency_agent.execute()

        yield f"status:reviewing"
        yield "🔍 Reviewing output..."
        for chunk in self.reviewer_agent.execute(task, diff="", preview=preview):
            yield chunk

        yield f"status:feedback"
        yield "🔁 Running feedback loop..."
        for chunk in self.builder_agent.execute(task, loop_mode=True, preview=preview):
            yield chunk

    def _handle_refine_requirement(self, data: dict, preview: bool):
        note = data.get("clarification", "Additional details provided.")
        yield f"status:requirements"
        yield f"🛠 Clarifying requirement: {note}"
        for chunk in self.requirements_agent.execute(note, preview=preview):
            yield chunk

    def _handle_update_architecture(self, data: dict, preview: bool):
        context = data.get("context", "")
        yield f"status:architecture"
        yield f"🧱 Updating architecture: {context}"
        for chunk in self.architecture_agent.execute(preview=preview):
            yield chunk

    def _handle_feedback_loop(self, data: dict, preview: bool):
        note = data.get("note", "Apply Code Review feedback to previous result")
        yield f"status:feedback"
        yield f"🔁 Responding to feedback: {note}"
        # Could route to builder or reviewer
        for chunk in self.builder_agent.execute(note, preview=preview, loop_mode=True):
            yield chunk

    def _handle_report_error(self, data: dict, preview: bool):
        error = data.get("error", {})
        message = error.get("message", "Unknown error")
        file_hint = error.get("file","")
        stack = error.get("stack", None)

        yield "🚨 Error reported:"
        yield f"```\n{message}\n\nFile: {file_hint or 'unknown'}\n```"

        # 🔎 Find related files using the dependency graph
        related_files = self.dependency_graph.find_related_to(file_hint) if file_hint else []
        files_to_check = [file_hint] + related_files if file_hint else related_files

        feedback_items = [
            {"task": f"Investigate error: {message}", "file_hint": f}
            for f in files_to_check
        ]

        if feedback_items:
            self.project.add_to_feedback_queue(feedback_items)
            yield f"📥 Added {len(feedback_items)} file(s) to the feedback queue."
        else:
            yield "⚠️ No related files found. Skipping queue addition."

        yield "🛠 This error has been triaged & recovery flow will begin shortly"

        yield "🔁 Fixing reported Error..."
        for chunk in self.builder_agent.execute("Fix reported error", loop_mode=True, preview=preview):
            yield chunk

    def _handle_ask_for_help(self, data: dict, preview: bool):
        question = data.get("question", "No question provided.")
        assistant = ScopedAssistantAgent(self.project)
        yield f"status:help"
        for chunk in assistant.execute(question, preview=preview, project_id=self.project.id):
            yield chunk

    def _handle_bootstrap(self, preview: bool = False):
        yield "status:bootstrapping"
        from agent.bootstrap.bootstrap_runner import BootstrapRunner

        runner = BootstrapRunner(self.project.path)
        runner.run()
        yield "🏁 Bootstrap complete."
