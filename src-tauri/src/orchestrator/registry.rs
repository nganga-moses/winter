use  std::collections::HashMap;
use crate::orchestrator::protocol::AgentResponse;
use crate::orchestrator::types::{AgentCard, AgentTask, Capability};
use crate::orchestrator::context::AgentContext;

/// Trait implemented by all agent handlers
pub trait AgentHandler{
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse;
}

/// Full metadata about a registered Agent
pub struct AgentMetadata{
    pub card: AgentCard,
    pub handler: Box<dyn AgentHandler + Send + Sync>
}

/// Stores all agents that can be routed to
pub struct AgentRegistry{
    agents: Vec<AgentMetadata>,
}

impl AgentRegistry{
    pub fn new()-> Self{
        Self{
            agents: vec![]
        }
    }
    pub fn register(&mut self,agent_card: AgentCard, handler: Box<dyn AgentHandler + Send + Sync>){
        self.agents.push(AgentMetadata{card:agent_card, handler});
    }
    /// Structured and deterministic capability matching
    pub fn find_agent_for_task(&self, task_type: &Capability)-> Option<&AgentMetadata>{
        self.agents
            .iter()
            .find(|agent| {
                let skills =&agent.card.skills;
                skills.root == *task_type || skills.subskills.contains(task_type)
            })
    }
    pub fn all_cards(&self)-> Vec<AgentCard> {
        self.agents.iter().map(|m| m.card.clone()).collect()
    }

}