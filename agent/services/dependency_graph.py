import ast
import json
import os
from pathlib import Path
from typing import Dict, List, Set, Tuple
import networkx as nx
from networkx.readwrite import json_graph


class DependencyGraph:
    def __init__(self, root_path: str):
        self.root_path = Path(root_path)
        self.import_graph = nx.DiGraph()
        self.call_graph = nx.DiGraph()
        self.inheritance_graph = nx.DiGraph()

    def build(self):
        for py_file in self.root_path.rglob("*.py"):
            if "venv" in str(py_file): continue
            self._analyze_file(py_file)

    def _analyze_file(self, file_path: Path):
        rel_path = str(file_path.relative_to(self.root_path))
        try:
            with open(file_path, "r", encoding="utf-8") as f:
                source = f.read()
            tree = ast.parse(source)
        except Exception as e:
            print(f"[WARN] Failed to parse {rel_path}: {e}")
            return

        imports = self._extract_imports(tree)
        for imp in imports:
            self.import_graph.add_edge(rel_path, imp)

        function_map = self._get_functions(tree)
        calls = self._extract_calls(tree, rel_path, function_map)
        for caller, callee in calls:
            self.call_graph.add_edge(caller, callee)

        classes = self._extract_inheritance(tree, rel_path)
        for base, child in classes:
            self.inheritance_graph.add_edge(base, child)

    def _extract_imports(self, tree: ast.AST) -> List[str]:
        imports = []
        for node in ast.walk(tree):
            if isinstance(node, ast.ImportFrom) and node.module:
                imports.append(node.module.replace(".", "/") + ".py")
            elif isinstance(node, ast.Import):
                for alias in node.names:
                    imports.append(alias.name.replace(".", "/") + ".py")
        return imports

    def _get_functions(self, tree: ast.AST) -> Dict[str, Tuple[int, int]]:
        func_map = {}
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                func_map[node.name] = (node.lineno, self._get_end_line(node))
        return func_map

    def _extract_calls(self, tree: ast.AST, rel_path: str, function_map: Dict[str, Tuple[int, int]]) -> List[
        Tuple[str, str]]:
        calls = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Call):
                if hasattr(node.func, 'id'):  # direct function call
                    caller = f"{rel_path}::unknown"
                    callee = node.func.id
                    calls.append((caller, callee))
                elif isinstance(node.func, ast.Attribute):
                    callee = node.func.attr
                    calls.append((f"{rel_path}::unknown", callee))
        return calls

    def _extract_inheritance(self, tree: ast.AST, rel_path: str) -> List[Tuple[str, str]]:
        edges = []
        for node in ast.walk(tree):
            if isinstance(node, ast.ClassDef):
                child = f"{rel_path}::{node.name}"
                for base in node.bases:
                    if isinstance(base, ast.Name):
                        parent = base.id
                        edges.append((parent, child))
        return edges

    def _get_end_line(self, node: ast.AST) -> int:
        if hasattr(node, 'body') and isinstance(node.body, list) and node.body:
            last = node.body[-1]
            return self._get_end_line(last)
        return getattr(node, 'lineno', -1)

    def print_summary(self):
        print(f"[✓] Imports: {self.import_graph.number_of_edges()} edges")
        print(f"[✓] Function Calls: {self.call_graph.number_of_edges()} edges")
        print(f"[✓] Inheritance: {self.inheritance_graph.number_of_edges()} edges")

    def find_most_called(self, top_n: int = 10) -> List[Tuple[str, str]]:
        """Top N most-called functions/classes"""
        degrees = self.call_graph.in_degree()
        return sorted(degrees, key=lambda x: x[1], reverse=True)[:top_n]

    def find_largest_modules(self, top_n: int = 5) -> List[Tuple[str, int]]:
        count = {}
        for node in self.call_graph.nodes:
            file = node.split("::")[0]
            count[file] = count.get(file, 0) + 1
        return sorted(count.items(), key=lambda x: x[1], reverse=True)[:top_n]

    def find_affected_files(self, symbol: str) -> Set[str]:
        """Given a symbol, return files that depend on it (e.g., call or inherit from it)"""
        affected = set()
        for node in self.call_graph.nodes:
            if symbol in node:
                affected.add(node.split("::")[0])
        for edge in self.inheritance_graph.edges:
            if symbol in edge:
                affected.update([s.split("::")[0] for s in edge])
        return affected

    def save_all(self, output_path: str):
        os.makedirs(output_path, exist_ok=True)
        self._save_graph(self.import_graph, os.path.join(output_path, "import_graph.json"))
        self._save_graph(self.call_graph, os.path.join(output_path, "call_graph.json"))
        self._save_graph(self.inheritance_graph, os.path.join(output_path, "inheritance_graph.json"))

    def load_all(self, input_path: str):
        self.import_graph = self._load_graph(os.path.join(input_path, "import_graph.json"))
        self.call_graph = self._load_graph(os.path.join(input_path, "call_graph.json"))
        self.inheritance_graph = self._load_graph(os.path.join(input_path, "inheritance_graph.json"))

    def _save_graph(self, graph: nx.DiGraph, path: str):
        data = json_graph.node_link_data(graph)
        with open(path, "w") as f:
            json.dump(data, f, indent=2)

    def _load_graph(self, path: str) -> nx.DiGraph:
        if not os.path.exists(path):
            return nx.DiGraph()
        with open(path, "r") as f:
            data = json.load(f)
        return json_graph.node_link_graph(data)

    def find_related_to(self, file_path: str) -> dict:
        """
        Given a file (relative path), return a dictionary of related nodes in:
        - Imports (both imported and importers)
        - Function calls (callers and callees)
        - Inheritance (parents and children)
        """
        related = {
            "imports": set(),
            "callers": set(),
            "callees": set(),
            "parents": set(),
            "children": set()
        }

        # --- Import Graph ---
        for u, v in self.import_graph.edges():
            if u == file_path:
                related["imports"].add(v)
            if v == file_path:
                related["imports"].add(u)

        # --- Call Graph ---
        for u, v in self.call_graph.edges():
            if file_path in u:
                related["callees"].add(v)
            if file_path in v:
                related["callers"].add(u)

        # --- Inheritance Graph ---
        for u, v in self.inheritance_graph.edges():
            if file_path in v:
                related["parents"].add(u)
            if file_path in u:
                related["children"].add(v)

        # Convert sets to sorted lists
        return {k: sorted(v) for k, v in related.items() if v}

    def find_hotspots(self, top_n: int = 10) -> dict:
        hotspots = {}

        # 🔥 Most called functions/classes (based on incoming edges)
        if hasattr(self.call_graph, "in_degree"):
            call_degrees = self.call_graph.in_degree()
            sorted_calls = sorted(call_degrees, key=lambda x: x[1], reverse=True)
            hotspots["most_called"] = [f"{n} ({d} calls)" for n, d in sorted_calls[:top_n]]

        # 🔗 Most imported modules (based on incoming edges)
        if hasattr(self.import_graph, "in_degree"):
            import_degrees = self.import_graph.in_degree()
            sorted_imports = sorted(import_degrees, key=lambda x: x[1], reverse=True)
            hotspots["most_imported"] = [f"{n} ({d} imports)" for n, d in sorted_imports[:top_n]]

        # 👑 Base classes (inherited from most often)
        if hasattr(self.inheritance_graph, "in_degree"):
            inh_degrees = self.inheritance_graph.in_degree()
            sorted_inheritance = sorted(inh_degrees, key=lambda x: x[1], reverse=True)
            hotspots["most_inherited"] = [f"{n} ({d} children)" for n, d in sorted_inheritance[:top_n]]

        return hotspots