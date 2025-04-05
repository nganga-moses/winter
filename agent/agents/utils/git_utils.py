import os
import subprocess
from typing import List, Optional
from git import Repo, InvalidGitRepositoryError, GitCommandError


def run_git_command(project_path: str, args: List[str]) -> str:
    result = subprocess.run(
        ["git"] + args,
        cwd=project_path,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode != 0:
        raise Exception(f"Git error: {result.stderr.strip()}")
    return result.stdout.strip()


def init_git_repo(project_path: str):
    git_path = os.path.join(project_path, ".git")
    if not os.path.exists(git_path):
        run_git_command(project_path, ["init"])
        run_git_command(project_path, ["config", "user.name", "Winter"])
        run_git_command(project_path, ["config", "user.email", "winter@localhost"])
        run_git_command(project_path, ["add", "."])
        run_git_command(project_path, ["commit", "-m", "Initial Commit"])


def commit_all_changes(project_path: str, message: str):
    run_git_command(project_path, ["add", "."])
    run_git_command(project_path, ["commit", "-m", message])


def get_commit_history(project_path: str, limit: int = 10) -> List[dict]:
    log_format = "--pretty=format:%h|%an|%ad|%s"
    raw = run_git_command(project_path, ["log", f"-{limit}", log_format])
    return [
        {
            "hash": line.split("|")[0],
            "author": line.split("|")[1],
            "date": line.split("|")[2],
            "message": line.split("|")[3]
        }
        for line in raw.splitlines()
    ]


def get_commit_diff(project_path: str, commit_hash: Optional[str] = None) -> str:
    if commit_hash:
        return run_git_command(project_path, ["show", commit_hash])
    else:
        return run_git_command(project_path, ["diff", "HEAD^", "HEAD"])
def list_branches(project_path: str) -> List[str]:
    raw = run_git_command(project_path, ["branch", "--list"])
    return [line.strip().lstrip("* ").strip() for line in raw.splitlines()]


def switch_branch(project_path: str, branch_name: str):
    try:
        # Try to switch
        run_git_command(project_path, ["checkout", branch_name])
    except Exception:
        # If it doesn't exist, create it
        run_git_command(project_path, ["checkout", "-b", branch_name])


def rollback_to_commit(project_path: str, commit_hash: str):
    run_git_command(project_path, ["reset", "--hard", commit_hash])