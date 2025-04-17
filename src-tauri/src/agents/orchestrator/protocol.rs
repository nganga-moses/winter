use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentMessage{
    Request{
        sender: String,
        receiver: String,
        payload: String,
    },
    Response{
        sender: String,
        payload: String,
    },
}