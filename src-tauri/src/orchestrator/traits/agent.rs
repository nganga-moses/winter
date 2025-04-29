use crate::agents::orchestrator::protocol::AgentResponse;
use crate::agents::orchestrator::types::{AgentTask};

pub trait Agent {
    fn id(&self) -> &'static str;
    fn get_card(&self)->AgentCard;
    fn can_handle(&self, task: &AgentTask) -> bool;
    async fn handle(&self, task: AgentTask) -> AgentResponse;
}