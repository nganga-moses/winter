use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::protocol::AgentResponse;
use crate::agents::orchestrator::registry::AgentHandler;
use crate::agents::orchestrator::types::{AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph};

pub struct DocAgent;

impl DocAgent {
    pub fn card() -> AgentCard {
        AgentCard {
            id: "doc".into(),
            description: "Generates or updates documentation for the project".into(),
            skills: SkillGraph {
                root: Capability::Documentation,
                subskills: vec![],
            },
            input_schema: "CodePatch".into(),
            output_schema: "DocSummary".into(),
            default_execution: ExecutionMode::Simulate,
        }
    }
}

impl AgentHandler for DocAgent {
    fn handle_task(&self, task: AgentTask, _ctx: AgentContext) -> AgentResponse {
        println!("[DocAgent] Stub handling task: {}", task.task_id);
        AgentResponse::success("Stub: documentation generated", "DocAgent")
    }
}