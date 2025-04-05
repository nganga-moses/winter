# agent/storage/models.py

from pydantic import BaseModel, Field
from typing import Literal
import uuid

class ProjectMeta(BaseModel):
    id: str = Field(default_factory=lambda: f"project-{uuid.uuid4().hex[:8]}")
    name: str
    description: str
    instructions: str
    privacy: Literal["public", "private"] = "private"