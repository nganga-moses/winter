# cli/main.py
import typer

from agent.cli import hotspots
app = typer.Typer()
app.add_typer(hotspots.app, name="graph")