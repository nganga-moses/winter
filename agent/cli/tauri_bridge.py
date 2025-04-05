from agent.agents.builder_agent import BuilderAgent
from agent.services.context_manager import ContextManager
import os
import sys
sys.path.append(os.path.dirname(os.path.dirname(__file__)))

if __name__ == "__main__":
    task = sys.argv[1]
    agent = BuilderAgent(project_path="agent")
    result = agent.run_task(task)
    print(result)