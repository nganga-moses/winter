import os
import json
import difflib
import subprocess
from datetime import datetime

from agent.storage.models import ProjectMeta


class ProjectManager:
    def __init__(self, project_path: str):
        self.path = project_path
        self.meta = self._load_or_create_meta()
        self.ensure_dirs()

    @property
    def id(self):
        return self.meta.get("id", os.path.basename(self.path))

    @property
    def name(self):
        return self.meta.get("name", "Unnamed Project")

    @property
    def description(self):
        return self.meta.get("description", "")

    @property
    def instructions(self):
        return self.meta.get("instructions", "")

    def _load_or_create_meta(self):
        meta_path = os.path.join(self.path, "meta.json")
        if os.path.exists(meta_path):
            with open(meta_path) as f:
                try:
                    data = json.load(f)
                    return ProjectMeta(**data)
                except Exception as e:
                    print(f"⚠️ Failed to load meta.json: {e}")

        # Fallback if meta.json is missing
        print("⚠️ No meta.json found, creating a blank one...")
        default = ProjectMeta(
            name=os.path.basename(self.path),
            description="No description",
            instructions="No instructions"
        )
        self._save_json("meta.json", default.dict())
        return default
    def ensure_dirs(self):
        for folder in ["chats", "diffs", "threads", "structure"]:
            os.makedirs(os.path.join(self.path, folder), exist_ok=True)

    # ==== File IO ====

    def write_file(self, relative_path: str, content: str):
        full_path = os.path.join(self.path, relative_path)
        os.makedirs(os.path.dirname(full_path), exist_ok=True)
        with open(full_path, "w") as f:
            f.write(content)

    def read_file(self, relative_path: str) -> str:
        full_path = os.path.join(self.path, relative_path)
        if not os.path.exists(full_path):
            return ""
        with open(full_path, "r") as f:
            return f.read()

    def file_exists(self, relative_path: str) -> bool:
        return os.path.exists(os.path.join(self.path, relative_path))

    def generate_diff(self, file_path: str, new_content: str) -> str:
        full_path = os.path.join(self.path, file_path)

        try:
            with open(full_path, "r") as f:
                old_content = f.readlines()
        except FileNotFoundError:
            old_content = []

        new_lines = new_content.splitlines(keepends=True)
        diff = difflib.unified_diff(
            old_content,
            new_lines,
            fromfile=f"a/{file_path}",
            tofile=f"b/{file_path}",
            lineterm=""
        )
        return "\n".join(diff)

    # ==== JSON Storage ====

    def _save_json(self, filename: str, data):
        with open(os.path.join(self.path, filename), "w") as f:
            json.dump(data, f, indent=2)

    def _load_json(self, filename: str):
        path = os.path.join(self.path, filename)
        if os.path.exists(path):
            with open(path, "r") as f:
                return json.load(f)
        return []

    def save_architecture(self, data):
        self._save_json("architecture.json", data)

    def save_requirements(self, data):
        self._save_json("requirements.json", data)

    def save_rules(self, data):
        self._save_json("rules.json", data)

    def save_structure(self, data):
        self._save_json("structure.json", data)

    def load_architecture(self):
        return self._load_json("architecture.json")

    def load_requirements(self):
        return self._load_json("requirements.json")

    def load_rules(self):
        return self._load_json("rules.json")

    def load_structure(self):
        return self._load_json("structure.json")

    # ==== Logs ====

    def save_chat(self, task: str, messages: list, thread_id: str = None):
        timestamp = datetime.now().strftime("%Y%m%d-%H-%M-%S")
        safe_task = task.replace(" ", "-")
        folder = os.path.join(self.path, "chats")
        filename = f"{timestamp}--{safe_task}.json"
        if thread_id:
            folder = os.path.join(self.path, "threads", thread_id)
            os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, filename), "w") as f:
            json.dump(messages, f, indent=2)

    def save_diff(self, task: str, diff: str):
        timestamp = datetime.now().strftime("%Y%m%d-%H-%M-%S")
        filename = f"{timestamp}--{task.replace(' ', '-')}.diff"
        path = os.path.join(self.path, "diffs", filename)
        with open(path, "w") as f:
            f.write(diff)

    def get_git_history(self, max_count: int = 20) -> list:
        """Returns latest git commits for this project"""
        try:
            output = subprocess.check_output(
                ["git", "log", f"--max-count={max_count}", "--pretty=format:%h|%an|%ar|%s"],
                cwd=self.path,
                stderr=subprocess.DEVNULL
            ).decode("utf-8")
            commits = []
            for line in output.strip().splitlines():
                hash_, author, rel_time, message = line.split("|", 3)
                commits.append({
                    "hash": hash_,
                    "author": author,
                    "relative_time": rel_time,
                    "message": message,
                })
            return commits
        except Exception as e:
            print(f"⚠️ Git history error: {e}")
            return []

    def get_git_diff(self, commit_hash: str) -> str:
        """Returns the diff for a given commit hash"""
        try:
            output = subprocess.check_output(
                ["git", "show", commit_hash, "--no-color"],
                cwd=self.path,
                stderr=subprocess.DEVNULL
            ).decode("utf-8")
            return output
        except Exception as e:
            print(f"⚠️ Git diff error: {e}")
            return f"Error fetching diff: {str(e)}"

    def add_to_feedback_queue(self, items: list):
        queue_path = os.path.join(self.path, "feedback_queue.json")
        queue = []

        if os.path.exists(queue_path):
            with open(queue_path, "r") as f:
                queue = json.load(f)

        queue.extend(items)

        with open(queue_path, "w") as f:
            json.dump(queue, f, indent=2)

        print(f"📥 {len(items)} feedback items added to queue.")