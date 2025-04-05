import requests
import openai
import os
from typing import Generator


class LLMRouter:
    def __init__(self, mode: str = "local", model: str = "mistral"):
        self.mode = mode
        self.model = model

        # Cloud
        self.openai_key = os.getenv("OPENAI_API_KEY", "")
        openai.api_key = self.openai_key

        if self.mode not in ("local", "cloud"):
            raise ValueError("mode must be 'local' or 'cloud'")

    def complete(self, prompt: str, stream: bool = True) -> Generator[str, None, None]:
        if self.mode == "local":
            return self._ollama_stream(prompt)
        else:
            return self._openai_stream(prompt)

    def _ollama_stream(self, prompt: str) -> Generator[str, None, None]:
        url = "http://localhost:11434/api/chat"
        response = requests.post(url, json={
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": True
        }, stream=True)

        for line in response.iter_lines():
            if not line:
                continue
            try:
                content = line.decode().split("data: ")[-1]
                if content == "[DONE]":
                    break
                yield eval(content)["message"]["content"]
            except Exception as e:
                print(f"[stream error] {e}")

    def _openai_stream(self, prompt: str) -> Generator[str, None, None]:
        response = openai.ChatCompletion.create(
            model=self.model,
            messages=[{"role": "user", "content": prompt}],
            stream=True,
        )
        for chunk in response:
            content = chunk["choices"][0]["delta"].get("content", "")
            yield content
