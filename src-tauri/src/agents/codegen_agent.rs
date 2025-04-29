use tauri::utils::acl::capability::CapabilityFile::Capability;
use crate::orchestrator::context::AgentContext;
use crate::orchestrator::protocol::AgentResponse;
use crate::orchestrator::registry::AgentHandler;
use crate::orchestrator::types::{AgentCard, AgentTask, ExecutionMode, SkillGraph};

pub struct CodegenAgent;

impl CodegenAgent{
    pub fn card()-> AgentCard{
        AgentCard {
            id: "codegen".to_string(),
            description: "Generates code based on design and requirements".to_string(),
            skills: SkillGraph {
                root: Capability::CodeGen,
                subskills:vec![],
            },
            input_schema: "ArchitecturePlan".into(),
            output_schema: "CodePatch".into(),
            default_execution: ExecutionMode::Simulate,
        }
    }
}
impl AgentHandler for CodegenAgent{
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[CodegenAgent] Stub handling task: {}", task.task_id);
        AgentResponse::success("Stub: code generated","CodegenAgent")
    }
}