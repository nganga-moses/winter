import json
import os
import uuid

from fastapi import APIRouter, Request, UploadFile, File, HTTPException

from agent.agents.assistant_agent import AssistantAgent
from agent.agents.dependency_graph_agent import DependencyGraphAgent
from agent.agents.orchestrator_agent import OrchestratorAgent
from agent.agents.utils.git_utils import get_commit_history, get_commit_diff
from agent.storage.project_manager import ProjectManager
from fastapi.responses import StreamingResponse
from pydantic import BaseModel

router = APIRouter()

PROJECTS_DIR = "./winter-data/projects"
UPLOAD_DIR = "./winter-data/assistant/uploads"
os.makedirs(UPLOAD_DIR, exist_ok=True)

ALLOWED_EXTENSIONS = {".pdf", ".txt", ".md", ".py", ".js", ".ts", ".json"}


class ProjectCreate(BaseModel):
    name: str
    description: str
    instructions: str
    privacy: str  # "public" or "private"


@router.get("/health")
async def health_check():
    return {"status": "ok"}


@router.get("/projects")
async def list_projects():
    try:
        projects = []
        for project_id in os.listdir(PROJECTS_DIR):
            project_path = os.path.join(PROJECTS_DIR, project_id)
            meta_path = os.path.join(project_path, "meta.json")

            if os.path.isdir(project_path) and os.path.exists(meta_path):
                with open(meta_path, "r") as f:
                    try:
                        meta = json.load(f)
                        meta["id"] = project_id  # ensure ID is included
                        projects.append(meta)
                    except json.JSONDecodeError:
                        continue  # skip corrupt or partial projects

        return {"projects": projects}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/projects")
async def create_project(payload: ProjectCreate):
    project_id = f"project-{uuid.uuid4().hex[:8]}"
    project_path = os.path.join(PROJECTS_DIR, project_id)
    os.makedirs(project_path, exist_ok=True)

    meta = {
        "id": project_id,
        "name": payload.name,
        "description": payload.description,
        "instructions": payload.instructions,
        "privacy": payload.privacy
    }

    with open(os.path.join(project_path, "meta.json"), "w") as f:
        f.write(meta.__str__())

    return {"id": project_id, "message": "Project created successfully"}


@router.get("/agent/chat/{project_id}")
async def stream_chat(request: Request, project_id: str, text: str, threadId: str = None):
    project = ProjectManager(f"./winter-data/projects/{project_id}")
    agent = OrchestratorAgent(project)
    preview = False

    # Use provided threadId or create a new one
    thread_id = threadId or f"thread-{uuid.uuid4().hex[:8]}"

    def stream():
        yield f"status:interpreting\n\n"

        for chunk in agent.execute(text, preview=preview, thread_id=thread_id):
            if isinstance(chunk, dict):
                payload = {
                    "type": "message",
                    "data": chunk.get("text", ""),
                    "threadId": chunk.get("threadId", thread_id),
                    "phase": chunk.get("phase"),
                    "summary": chunk.get("summary"),
                }
                yield f"data:{json.dumps(payload)}\n\n"
            else:
                yield f"data:{chunk}\n\n"

        yield "data:[[END]]\n\n"

    return StreamingResponse(stream(), media_type="text/event-stream")


@router.post("/assistant/upload")
async def upload_file(
        file: UploadFile = File(...)):
    ext = os.path.splitext(file.filename)[-1].lower()
    if ext not in ALLOWED_EXTENSIONS:
        raise HTTPException(status_code=400, detail=f"File type {ext} not allowed")

    file_id = f"{uuid.uuid4().hex[:8]}{ext}"
    save_path = os.path.join(UPLOAD_DIR, file_id)

    with open(save_path, "wb") as f:
        content = await file.read()
        f.write(content)
    return {"id": file_id, "name": file.filename, "path": save_path, "message": "File uploaded"}


@router.get("/assistant")
async def assistant_chat(request: Request, text: str, preview: bool = False):
    agent = AssistantAgent()

    def stream():
        for chunk in agent.execute(text, preview=preview):
            if isinstance(chunk, str) and chunk.startswith("status:"):
                yield f"{chunk}\n\n"
            else:
                yield f"data: {chunk}\n\n"
        yield "data:[[END]]\n\n"

    return StreamingResponse(stream(), media_type="text/event-stream")


@router.get("/projects/{project_id}/hotspots")
def get_hotspots(project_id: str, graph: str = "call", top: int = 10):
    project = ProjectManager(f"./winter-data/projects/{project_id}")
    agent = DependencyGraphAgent(project)
    return agent.find_hotspots(graph_type=graph, top_n=top)


@router.get("/agent/gitlog/{project_id}")
async def get_git_history(project_id: str):
    project = ProjectManager(f"./winter-data/projects/{project_id}")
    return {"commits": project.get_git_history()}


@router.get("/agent/gitdiff/{project_id}/{commit_hash}")
async def get_git_diff(project_id: str, commit_hash: str):
    project = ProjectManager(f"./winter-data/projects/{project_id}")
    diff = project.get_git_diff(commit_hash)
    return {"commit": commit_hash, "diff": diff}


@router.get("/git/history/{project_id}")
def git_history(project_id: str):
    project_path = f"./winter-data/projects/{project_id}"
    return get_commit_history(project_path)


@router.get("/git/diff/{project_id}")
def git_diff(project_id: str):
    project_path = f"./winter-data/projects/{project_id}"
    return get_commit_diff(project_path)
