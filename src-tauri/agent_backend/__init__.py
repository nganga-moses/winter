from fastapi import FastAPI
from fastapi.responses import StreamingResponse
from pydantic import BaseModel
from leo_core import LeoCore

app = FastAPI()
leo = LeoCore()

class ChatInput(BaseModel):
    message: str
    project_id: str

@app.post("/chat")
async def chat(req: ChatInput):
    def generate():
        for chunk in leo.stream_response(req.message, req.project_id):
            yield chunk
    return StreamingResponse(generate(), media_type="text/plain")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="127.0.0.1", port=5100)