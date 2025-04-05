from fastapi import FastAPI, Request
from agent.routes import agent_routes

app = FastAPI()

app.include_router(agent_routes.router)