import logging
import os
import sys

from fastapi import FastAPI
from starlette.middleware.cors import CORSMiddleware
from contextlib import asynccontextmanager

from agent.routes import agent_routes


@asynccontextmanager
async def lifespan(app: FastAPI):
    logger.info("Starting up Winter Backend...")
    yield
    logger.info("Shutting down Winter Backend...")


app = FastAPI(lifespan=lifespan)

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Allows all origins. Change to specific origins in production.
    allow_credentials=True,
    allow_methods=["*"],  # Allows all HTTP methods (e.g., GET, POST, PUT).
    allow_headers=["*"],  # Allows all headers.
)
# Include the API router
app.include_router(agent_routes.router)


def get_port():
    for arg in sys.argv:
        if arg.startswith("--port="):
            return int(arg.split("=")[1])
    return int(os.environ.get("PORT", 6144))


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=get_port(), reload=False)
