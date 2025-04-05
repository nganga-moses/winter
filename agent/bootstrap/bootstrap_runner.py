import os
import subprocess
from context.code_chunk_registry import CodeChunkRegistry
from context.dependency_graph import DependencyGraph
from context.memory_store.memory_sqlite import MemorySqlite
from llm.llm_router import LLMRouter


class BootstrapRunner:
    def __init__(self, repo_url_or_path: str, db_path: str = "winter_memory.db", model: str = "mistral",
                 mode: str = "local"):
        self.repo_path = self._clone_repo_if_needed(repo_url_or_path)
        self.registry = CodeChunkRegistry(self.repo_path)
        self.graph = DependencyGraph(self.repo_path)
        self.memory = MemorySqlite(db_path)
        self.llm = LLMRouter(model=model, mode=mode)

    def _clone_repo_if_needed(self, path_or_url: str) -> str:
        if path_or_url.startswith("http"):
            repo_name = path_or_url.split("/")[-1].replace(".git", "")
            target_path = f"./repos/{repo_name}"
            if not os.path.exists(target_path):
                subprocess.run(["git", "clone", path_or_url, target_path], check=True)
            return target_path
        return path_or_url

    def run(self):
        print(f"🚀 Bootstrapping: {self.repo_path}")
        self.registry.scan()
        self.graph.build()
        print("🔗 Saving dependency graphs...")
        self.graph.save_all(os.path.join(self.repo_path, "dependency"))

        chunks = self.registry.to_list()
        func_doc_summary = "\n".join([f"{c['file']}::{c['name']}: {c['docstring']}" for c in chunks if c['docstring']])

        prompt = f"""
        You are analyzing a codebase. Below is a list of function/class docstrings:
        
        {func_doc_summary}
        
        ___
        
        From this, extract:
        1. A list of project requirements/features (1 per line)
        2. Coding conventions or patterns used
        3. Any architectural modules or layers you can infer
        
        Format as:
        ## Requirements
        - ...
        ## Rules
        - ...
        ## Architecture
        - ...
        """

        print("🤖 Agent is analyzing the codebase...")
        result = ""
        for chunk in self.llm.complete(prompt, stream=True):
            print(chunk,end="",flush=True)
            result += chunk

    def _parse_and_store(self, raw_output: str):
        current = None
        for line in raw_output.splitlines():
            line = line.strip()
            if line.startswith("## "):
                current = line[3:].lower()
            elif current == "requirements" and line.startswith("- "):
                self.memory.add_requirement(line[2:])
            elif current == "rules" and line.startswith("- "):
                self.memory.add_rule("convention",line[2:])
            elif current == "architecture" and line.startswith("- "):
                self.memory.add_architecture(module=line[2:], layer=None, dependencies=None)
