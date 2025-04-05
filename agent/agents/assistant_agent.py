import json
import os
from typing import Optional, List

from agent.llm.llm_router import LLMRouter
from agent.llm.models.prompter import PromptAssembler
from agent.storage.project_manager import ProjectManager
from PyPDF2 import PdfReader, PdfFileReader

ASSISTANT_PATH = "./winter-data/assistant"
HISTORY_FILE = os.path.join(ASSISTANT_PATH, "history,json")


class AssistantAgent:
    def __init__(self, project: ProjectManager = None, llm_mode="local", model="mistral"):
        os.makedirs(ASSISTANT_PATH, exist_ok=True)
        if not os.path.exists(HISTORY_FILE):
            with open(HISTORY_FILE, "w") as f:
                json.dump([], f)
        self.project = project or None
        self.llm = LLMRouter(mode=llm_mode, model=model)
        self.prompter = PromptAssembler(role="assistant")

    def _load_history(self, project_id: Optional[str] = None):
        if project_id:
            path = os.path.join(self.project.path, "assistant_history.json")
        else:
            path = HISTORY_FILE

        if os.path.exists(path):
            with open(path, "r") as f:
                return json.loads(f.read())
        return []

    def _save_to_history(self, messages: List[dict], project_id: Optional[str] = None):
        if project_id:
            path = os.path.join(self.project.path, "assistant_history.json")
        else:
            path = HISTORY_FILE

        history = self._load_history(project_id)
        history += messages[-20:]

        with open(path, "w") as f:
            json.dump(history, f, indent=2)

    def execute(self, user_input: str,project_id: Optional[str], preview: bool = False):
        messages = [{"role": "user", "content": user_input}]
        history = self._load_history(project_id)[-20:]  # Most recent messages only

        # 1. Format chat context
        chat_context = "\n".join([f"{m['role']}: {m['content']}" for m in history])

        # 2. Load uploaded files
        upload_context = self._load_uploaded_context()

        # 3. Build structured context for PromptAssembler
        context = {
            "uploads": upload_context or "[None]",
            "chat": chat_context or "[No conversation history]"
        }

        # 4. Assemble prompt (assistant role handles chat + files internally)
        prompt = self.prompter.assemble(task=user_input, context=context)

        yield "status:thinking"
        response = ""
        for chunk in self.llm.complete(prompt, stream=True):
            response += chunk
            if preview:
                yield chunk

        if not preview:
            self._save_to_history(messages + [{"role": "assistant", "content": response}],project_id)
            yield response

    def _load_uploaded_context(self):
        context_blobs = []

        for filename in os.listdir("./winter-data/assistant/uploads"):
            path = os.path.join(f"./winter-data/assistant/uploads", filename)
            ext = os.path.splitext(filename)[-1]

            try:
                if ext == ".pdf":
                    reader = PdfFileReader(path)
                    text = "\n".join(page.extract_text() for page in reader.pages)
                    context_blobs.append(f"[PDF: {filename}]\n{text.strip()[:2000]}")
                else:
                    with open(path, "r", encoding="utf-8") as f:
                        text = f.read()
                        context_blobs.append(f"[{filename}]\n{text.strip()[:2000]}")
            except Exception as e:
                print(f"⚠️ Failed to read {filename}: {e}")
        return "\n\n".join(context_blobs)
