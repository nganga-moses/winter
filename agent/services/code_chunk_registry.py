import ast
from pathlib import Path
from typing import List, Dict, Optional, Union

import tiktoken


class CodeChunk:
    def __init__(self, file_path: str, chunk_type: str, name: str, start_line: int, end_line: int,
                 docstring: Optional[str], token_count: int):
        self.file_path = file_path
        self.chunk_type = chunk_type  # "function" or "class"
        self.name = name
        self.start_line = start_line
        self.end_line = end_line
        self.docstring = docstring or ""
        self.token_count = token_count

    def to_dict(self):
        return {
            "file": self.file_path,
            "type": self.chunk_type,
            "name": self.name,
            "start_line": self.start_line,
            "end_line": self.end_line,
            "docstring": self.docstring.strip(),
            "token_count": self.token_count
        }


class CodeChunkRegistry:
    def __init__(self, project_path: str):
        self.project_path = Path(project_path)
        self.registry: List[CodeChunk] = []

    def _estimate_tokens(self, text: str) -> int:
        try:
            encoding = tiktoken.encoding_for_model("gpt-3.5-turbo")
            return len(encoding.encode(text))
        except Exception:
            return len(text.split())  # Fallback for count approximation

    def scan(self):
        for py_file in self.project_path.rglob("*.py"):
            if "venv" in str(py_file):  # Skip virtual environments
                continue
            self._parse_file(py_file)

    def _parse_file(self, file_path: Path):
        try:
            with open(file_path, "r", encoding="utf-8") as f:
                source = f.read()
            tree = ast.parse(source)
        except Exception as e:
            print(f"[WARN] Failed to parse {file_path}: {e}")
            return

        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                self._register_function(file_path, node)
            elif isinstance(node, ast.ClassDef):
                self._register_class(file_path, node)

    def _register_function(self, file_path: Path, node: Union[ast.FunctionDef, ast.AsyncFunctionDef]):
        start_line = node.lineno
        end_line = self._get_end_line(node)
        docstring = ast.get_docstring(node)
        name = node.name

        source_lines = self._get_source_lines(file_path, start_line, end_line)
        token_count = self._estimate_tokens(source_lines)

        chunk = CodeChunk(
            file_path=str(file_path.relative_to(self.project_path)),
            chunk_type="function",
            name=name,
            start_line=start_line,
            end_line=end_line,
            docstring=docstring,
            token_count=token_count
        )
        self.registry.append(chunk)

    def _register_class(self, file_path: Path, node: ast.ClassDef):
        start_line = node.lineno
        end_line = self._get_end_line(node)
        docstring = ast.get_docstring(node)
        name = node.name
        source_lines = self._get_source_lines(file_path, start_line, end_line)
        token_count = self._estimate_tokens(source_lines)

        chunk = CodeChunk(
            file_path=str(file_path.relative_to(self.project_path)),
            chunk_type="class",
            name=name,
            start_line=start_line,
            end_line=end_line,
            docstring=docstring,
            token_count=token_count
        )
        self.registry.append(chunk)

    def _get_source_lines(self, file_path: Path, start_line: int, end_line: int) -> str:
        try:
            with open(file_path, "r", encoding="utf-8") as f:
                lines = f.readlines()
            return "".join(lines[start_line - 1:end_line])
        except Exception as e:
            print(f"[WARN] Failed to read source for {file_path}: {e}")
            return ""

    def _get_end_line(self, node: ast.AST) -> int:
        if hasattr(node, 'body') and isinstance(node.body, list) and node.body:
            last = node.body[-1]
            if isinstance(last, ast.AST):
                return self._get_end_line(last)
        return getattr(node, 'lineno', -1)

    def to_list(self) -> List[Dict]:
        return [chunk.to_dict() for chunk in self.registry]

    def print_summary(self):
        print(f"[✓] Indexed {len(self.registry)} code chunks:")
        for chunk in self.registry:
            print(f" - {chunk.chunk_type}: {chunk.name} ({chunk.file_path}:{chunk.start_line}-{chunk.end_line})")
