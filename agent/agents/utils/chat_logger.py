# agent_core/utils/chat_logger.py

import os
import json
from datetime import datetime


def log_interaction(project, user_message, intent, metadata):
    logs_path = os.path.join(project.path, "chats")
    os.makedirs(logs_path, exist_ok=True)
    ts = datetime.now().strftime("%Y%m%d-%H%M%S")
    entry = {
        "timestamp": ts,
        "message": user_message,
        "intent": intent,
        "metadata": metadata
    }
    with open(os.path.join(logs_path, f"{ts}_orchestrator.json"), "w") as f:
        json.dump(entry, f, indent=2)
