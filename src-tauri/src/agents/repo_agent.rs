use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::protocol::AgentResponse;
use crate::agents::orchestrator::registry::AgentHandler;
use crate::agents::orchestrator::types::{AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph};

pub struct RepoAgent;

impl RepoAgent {
    pub fn card() -> AgentCard {
        AgentCard {
            id: "repo".into(),
            description: "Clones, audits, and initializes project context from external repos".into(),
            skills: SkillGraph {
                root: Capability::RepoAnalysis,
                subskills: vec![Capability::CodeGen, Capability::Documentation],
            },
            input_schema: "GitUrl".into(),
            output_schema: "ProjectContext".into(),
            default_execution: ExecutionMode::Execute,
        }
    }
}

impl AgentHandler for RepoAgent {
    fn handle_task(&self, task: AgentTask, _ctx: AgentContext) -> AgentResponse {
        println!("[RepoAgent] Stub handling task: {}", task.task_id);
        AgentResponse::success("Stub: repo analyzed and project bootstrapped", "RepoAgent")
    }
}