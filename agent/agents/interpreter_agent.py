import json
from agent.llm.llm_router import LLMRouter


class ChatInterpreter:
    def __init__(self, model: str = "mistral", mode: str = "local"):
        self.llm = LLMRouter(model=model, mode=mode)

    def interpret(self, message: str) -> dict:
        prompt = self._build_prompt(message)
        raw = self.llm.complete(prompt)

        # If streaming, collect full string
        if hasattr(raw, "__iter__") and not isinstance(raw, str):
            full_response = "".join(chunk for chunk in raw)
        else:
            full_response = raw

        try:
            return json.loads(full_response)
        except json.JSONDecodeError as e:
            return {
                "intent": "unknown",
                "error": f"Failed to parse JSON: {e}",
                "raw": full_response.strip()
            }

    def _build_prompt(self, message: str) -> str:
        return f"""
            You are an intelligent message interpreter for an autonomous software engineer.
            
            Classify the user's message below into one of the following intents, and extract a structured payload for downstream processing.
            
            ## Supported Intents:
            
            - initialize_project: User wants to build something new (e.g., "I want to build a school system")
            - add_requirement: User describes a new feature or goal
            - refine_requirement: User provides clarification or refinement
            - update_architecture: User describes a structural or system-level change
            - feedback_loop: Response to a prior task/code/review (e.g., "That looks good, but add X")
            - report_error: User encountered an issue or exception
            - ask_for_help: User needs guidance
            - bootstrap: User wants to analyze an existing codebase (e.g., from GitHub or a local path) to reverse-engineer its requirements, architecture, and rules.
            - unknown: Fallback if nothing matches
            
            ---
            
            ## User Message:
            {message}
            
            ---
            
            ## Respond with JSON like:
            
            For `initialize_project`:
            {
              "intent": "initialize_project",
              "project": {
                "name": "School Management System",
                "description": "Handles students, teachers, grades, fees, and reports."
              }
            }
            
            For `add_requirement`:
            {
              "intent": "add_requirement",
              "requirement": {
                "title": "Add payments",
                "description": "Enable students to pay fees via Stripe."
              }
            }
            
            For `refine_requirement`:
            {
              "intent": "refine_requirement",
              "clarification": "Also allow parents to view payment history."
            }
            
            For `update_architecture`:
            {
              "intent": "update_architecture",
              "context": "We need to move the form engine to a separate microservice."
            }
            
            For `feedback_loop`:
            {
              "intent": "feedback_loop",
              "note": "Looks good, but update the validation logic."
            }
            
            For `report_error`:
            {
              "intent": "report_error",
              "error": {
                "message": "ModuleNotFoundError: No module named 'flask'",
                "context": "when starting the backend"
              }
            }
            
            For `ask_for_help`:
            {
              "intent": "ask_for_help",
              "question": "How do I add a new endpoint?"
            }
            
            For `bootstrap`:
            {
              "intent": "bootstrap",
              "bootstrap": {
                "repo": "https://github.com/example/project.git",
                "instructions": "Focus on backend structure and database layer."
              }
            }
            
            Only output a valid JSON object. No markdown, no commentary, no code fences.

        """
