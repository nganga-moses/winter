from typing import List, Dict, Optional

import tiktoken

from agent.services.code_chunk_registry import CodeChunkRegistry
from agent.services.dependency_graph import DependencyGraph
from agent.storage.project_manager import ProjectManager


class ContextManager:
    def __init__(
            self,
            project: ProjectManager,
            db_path: str = "winter_memory.db",
            max_tokens: int = 2000,
            chunk_buffer: int = 250,
    ):
        self.project = project
        self.max_tokens = max_tokens
        self.chunk_buffer = chunk_buffer
        self.tokenizer = tiktoken.encoding_for_model("gpt-3.5-tubo")

        # Core Engines
        self.chunk_registry = CodeChunkRegistry(project.path)
        self.chunk_registry.scan()

        self.dependency_graph = DependencyGraph(project.path)
        self.dependency_graph.build()



    def _estimate_tokens(self, text: str) -> int:
        try:
            return len(self.tokenizer.encode(text))
        except Exception:
            return len(text.split())  # Fallback



    def get_context_for_task(self, task: str, file_hint: Optional[str] = None) -> Dict:
        requirements = self.project.load_requirements()
        rules = self.project.load_rules()
        architecture = self.project.load_architecture()

       # 1.  Prioritize code chunks near file_hint or by name match
        chunks = self.chunk_registry.to_list()

        # Sort by file_hint first if provided(rudimentary relevance)
        if file_hint:
            chunks.sort(key=lambda c:0 if file_hint in c["file"] else 1)
        selected_chunks = []

        # Token budget filtering
        token_total = 0
        for chunk in chunks:
            if file_hint and file_hint not in chunk["file"]:
                continue  # prioritize only related files first
            estimated_tokens = chunk["token_count"]
            if token_total + estimated_tokens <= self.max_tokens - self.chunk_buffer:
                selected_chunks.append(chunk)
                token_total += estimated_tokens
            else:
                break  # don't overflow token limit

        # Fallback:: add more services from unrelated files if there is room
        for chunk in chunks:
            if chunk in selected_chunks:
                continue
            estimated_tokens = self.estimate_tokens(chunk)
            if token_total + estimated_tokens <= self.max_tokens - self.chunk_buffer:
                selected_chunks.append(chunk)
                token_total += estimated_tokens
            else:
                break
        # Memory sections (dynamic allocation)
        remaining_tokens = self.max_tokens - token_total

        def add_memory(label: str, data: List[str], max_share: float)-> List[str]:
            selected = []
            current_tokens = 0
            max_tokens = int(remaining_tokens * max_share)
            for entry in data:
                tokens = self._estimate_tokens(entry)
                if current_tokens + tokens <= max_tokens:
                    selected.append(entry)
                    current_tokens += tokens
                else:
                    break
            return selected


        selected_reqs = add_memory("requirements",requirements, 0.5)
        selected_rules = add_memory("rules",rules,0.3)
        selected_arch = add_memory("architecture",architecture, 0.2)

        final_token_count = token_total + sum(
            self._estimate_tokens(x) for x in selected_reqs + selected_rules + selected_arch
        )

        # 4. Return assembled payload
        return {
            "task": task,
            "code_chunks": selected_chunks,
            "requirements": selected_reqs,
            "rules": selected_rules,
            "architecture": selected_arch,
            "estimated_tokens": final_token_count
        }

    def estimate_tokens(self, chunk: dict)->int:
        # Estimate: function signature + docstring only
        code_len = len(chunk.get("name",""))+ len(chunk.get("docstring",""))
        return code_len // 4 + 10 # ~ 4 chars per token + reasonable buffer
    def debug_print(self, payload: Dict):
        print(f"\n[🧠 ContextManager] Task: {payload['task']}")
        print(f"🧱 Code Chunks: {len(payload['code_chunks'])}")
        print(f"📜 Requirements: {len(payload['requirements'])}")
        print(f"⚙️  Rules: {len(payload['rules'])}")
        print(f"🧩 Architecture: {len(payload['architecture'])}")
        print(f"📊 Estimated Tokens: {payload['estimated_tokens']}")