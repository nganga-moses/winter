import os
import json
import networkx as nx
from agent.services.dependency_graph import DependencyGraph
from agent.storage.project_manager import ProjectManager
import matplotlib.pyplot as plt


class DependencyGraphAgent:
    def __init__(self, project: ProjectManager):
        self.project = project
        self.graph_dir = os.path.join(project.path, "dependency")
        os.makedirs(self.graph_dir, exist_ok=True)

        # Lazily loaded graphs
        self.import_graph = None
        self.call_graph = None
        self.inheritance_graph = None

    def execute(self):
        yield "🔍 Building dependency graphs..."
        graph = DependencyGraph(self.project.path)
        graph.build()

        yield "✅ Dependency graphs built."
        self._save_graph(graph.import_graph, "import_graph.json")
        self._save_graph(graph.call_graph, "call_graph.json")
        self._save_graph(graph.inheritance_graph, "inheritance_graph.json")

        yield "📁 Graphs saved to project folder."

    def _save_graph(self, nx_graph, filename: str):
        path = os.path.join(self.graph_dir, filename)
        data = nx.node_link_data(nx_graph)
        with open(path, "w") as f:
            json.dump(data, f, indent=2)

    def _load_graph(self, filename: str):
        path = os.path.join(self.graph_dir, filename)
        if os.path.exists(path):
            with open(path) as f:
                data = json.load(f)
                return nx.node_link_graph(data)
        return nx.DiGraph()

    def _ensure_loaded(self):
        if self.import_graph is None:
            self.import_graph = self._load_graph("import_graph.json")
        if self.call_graph is None:
            self.call_graph = self._load_graph("call_graph.json")
        if self.inheritance_graph is None:
            self.inheritance_graph = self._load_graph("inheritance_graph.json")

    def find_affected_files(self, symbol: str) -> list:
        """
        Given a class/function name, return all files that are affected
        via calls, inheritance or imports.
        """
        self._ensure_loaded()
        affected_files = set()

        # From call graph
        for caller, callee in self.call_graph.edges():
            if callee == symbol:
                affected_files.add(caller.split("::")[0])

        # From inheritance
        for parent, child in self.inheritance_graph.edges():
            if parent == symbol:
                affected_files.add(child.split("::")[0])

        # From imports
        for source, target in self.import_graph.edges():
            if symbol in target:
                affected_files.add(source)

        return sorted(affected_files)

    def visualize_graph(self, graph_type: str):
        self._ensure_loaded()
        graph_map = {
            "import": self.import_graph,
            "call": self.call_graph,
            "inheritance": self.inheritance_graph,
        }
        graph = graph_map.get(graph_type)
        if graph is None:
            raise ValueError(f"Unknown graph type: {graph_type}")

        plt.figure(figsize=(12, 8))
        pos = nx.spring_layout(graph, seed=42)
        nx.draw(graph, pos, with_labels=True, node_color='lightblue', edge_color='gray', font_size=8, node_size=700)
        filename = os.path.join(self.graph_dir, f"{graph_type}.png")
        plt.savefig(filename)
        plt.close()
        return filename

    def print_hotspots(self, top_n: int = 10):
        graph = DependencyGraph(self.project.path)
        graph.build()

        print("🔥 Dependency Hotspots:")
        hotspots = graph.find_hotspots(top_n=top_n)

        for category, items in hotspots.items():
            print(f"\n📌 {category.replace('_', ' ').title()}")
            for item in items:
                print(f"   - {item}")

    def load_all_graphs(self):
        return {
            "import": self._load_graph("import_graph.json"),
            "call": self._load_graph("call_graph.json"),
            "inheritance": self._load_graph("inheritance_graph.json"),
        }

    def find_hotspots(self, graph_type="call", top_n=5):
        graphs = self.load_all_graphs()
        graph = graphs.get(graph_type)
        if not graph:
            return []

        degrees = graph.in_degree()  # in-degree = how many times this node is used/imported/called
        sorted_nodes = sorted(degrees, key=lambda x: x[1], reverse=True)
        return sorted_nodes[:top_n]
