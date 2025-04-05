
import os
from git import Repo, GitCommandError
from typing import List


class GitAgent:
    def __init__(self, repo_path: str):
        self.repo_path = repo_path
        self.repo = self._load_or_init_repo()

    def _load_or_init_repo(self) -> Repo:
        if not os.path.exists(os.path.join(self.repo_path, ".git")):
            print("🆕 Initializing Git repository...")
            return Repo.init(self.repo_path)
        return Repo(self.repo_path)

    def init_repo(self):
        if not os.path.exists(os.path.join(self.repo_path, ".git")):
            Repo.init(self.repo_path)
            return "✅ Git repository initialized."
        return "✅ Git already initialized."

    def stage_changes(self, paths: List[str] = ["."]) -> str:
        try:
            self.repo.index.add(paths)
            return f"✅ Staged files: {paths}"
        except GitCommandError as e:
            return f"❌ Failed to stage files: {e}"

    def commit(self, message: str) -> str:
        try:
            if self.repo.is_dirty(index=True, working_tree=True, untracked_files=True):
                self.repo.index.commit(message)
                return f"✅ Commit created: {message}"
            return "⚠️ No changes to commit."
        except GitCommandError as e:
            return f"❌ Commit failed: {e}"

    def push(self, remote_name: str = "origin", branch: str = "main") -> str:
        try:
            remote = self.repo.remote(name=remote_name)
            remote.push(refspec=branch)
            return f"🚀 Pushed to {remote_name}/{branch}"
        except Exception as e:
            return f"❌ Push failed: {e}"

    def get_diff(self) -> str:
        return self.repo.git.diff()

    def get_history(self, max_count: int = 5) -> str:
        return self.repo.git.log(f"-{max_count}", "--oneline")

    def status(self) -> str:
        return self.repo.git.status()
