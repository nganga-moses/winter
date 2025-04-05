import os
import json
from storage.project_manager import ProjectManager

class WorkspaceManager:
    def __init__(self, root_path: str="winter-data"):
        self.root_path = root_path
        os.makedirs(self.root_path, exist_ok=True)
        self._init_workspace()

    def __init_workspace(self):
        workspace_file = os.path.join(self.root_path, "workspace.json")
        if not os.path.exists(workspace_file):
            with open(workspace_file, "w") as f:
                json.dump({"projects": []}, f)

    def list_projects(self):
        projects_dir = os.path.join(self.root_path, "projects")
        if not os.path.exists(projects_dir):
            return []
        return [d for d in os.listdir(projects_dir) if os.path.isdir(os.path.join(projects_dir))]

    def create_project(self, name: str)-> ProjectManager:
        slug = name.lower().replace(" ","-")
        path = os.path.join(self.root_path, "projects",slug)
        os.makedirs(path, exist_ok=True)

        metadata_path = os.path.join(path, "metadata.json")
        if not os.path.exists(metadata_path):
            with open(metadata_path, "w") as f:
                json.dump({"name": name}, f)
        return ProjectManager(path)

    def load_project(self, slug: str) -> ProjectManager:
        path = os.path.join(self.root_path, "projects", slug)
        return ProjectManager(path)