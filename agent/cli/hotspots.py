# cli/hotspots.py
import typer
from agent.agents.dependency_graph_agent import DependencyGraphAgent
from agent.storage.project_manager import ProjectManager

app = typer.Typer()

@app.command()
def find(
    project_path: str = typer.Argument(..., help="Path to the project folder"),
    graph_type: str = typer.Option("call", help="Graph type: call, import, inheritance"),
    top_n: int = typer.Option(10, help="How many top nodes to return")
):
    """Show hotspot files or functions in a dependency graph."""
    project = ProjectManager(project_path)
    agent = DependencyGraphAgent(project)
    results = agent.find_hotspots(graph_type, top_n)

    print(f"\n🔥 Top {top_n} hotspots in '{graph_type}' graph:\n")
    for node, count in results:
        print(f"{node}  →  referenced {count}x")