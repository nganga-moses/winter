use crate::orchestrator::context::AgentContext;
use crate::orchestrator::protocol::AgentResponse;
use crate::orchestrator::registry::AgentHandler;
use crate::orchestrator::types::{
    AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph,
};
use winter_ui_lib::prompt_assembler::PromptAssembler;
use winter_ui_lib::tools::llm_tool::LLMTool;

pub struct CritiqueAgent;

impl CritiqueAgent {
    pub fn card() -> AgentCard {
        AgentCard {
            id: "critique".into(),
            description: "Evaluates agent responses and adds feedback notes".into(),
            skills: SkillGraph {
                root: Capability::Critique,
                subskills: vec![],
            },
            input_schema: "AgentOutput".into(),
            output_schema: "CriticList".into(),
            default_execution: ExecutionMode::Simulate,
        }
    }
    pub fn new() -> Self {
        CritiqueAgent
    }
}

impl AgentHandler for CritiqueAgent {
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[CritiqueAgent] Starting Reviewing...");

        // 1. Assemble prompt
        let prompt_result = PromptAssembler::assemble("critique_agent", &task, &ctx.project);
        if let Err(err) = prompt_result {
            return AgentResponse::error(&format!("Prompt assembly failed: {}", err), false);
        }
        let prompt = prompt_result.unwrap();

        // 2. Query LLM
        let llm_tool = LLMTool::new();
        let query_result = llm_tool.query(prompt);

        if let Err(err) = query_result {
            return AgentResponse::error(&format!("LLM query failed: {}", err), false);
        }
        let llm_output = query_result.unwrap();

        println!("[CritiqueAgent] Received feedback from Model.");

        // 3. Save feedback to project feedback memory
        if let Err(err) = ctx
            .project
            .save_to_project_memory("feedback_queue.json", &llm_output)
        {
            return AgentResponse::error(
                &format!("Saving critique feedback failed: {}", err),
                false,
            );
        }

        AgentResponse::success(
            "Critique generated and saved.".to_string().to_string(),
            "critique_agent",
        )
    }
}
