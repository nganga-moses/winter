from agent.agents.assistant_agent import AssistantAgent
from agent.storage.project_manager import ProjectManager


class ScopedAssistantAgent(AssistantAgent):
    def __init__(self, project: ProjectManager, llm_mode="local", model="mistral"):
        super().__init__(project, llm_mode, model)
