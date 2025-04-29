use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;
use crate::orchestrator::context::AgentContext;
use crate::orchestrator::prompt_assembler::PromptAssembler;
use crate::orchestrator::protocol::AgentResponse;
use crate::tools::llm_tool::LLMTool;
use crate::memory::project_memory::DesignDecisions;
use crate::orchestrator::registry::AgentHandler;
use crate::orchestrator::types::{AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph};

pub struct RequirementsAgent;

impl RequirementsAgent{
    pub fn card()-> AgentCard{
        AgentCard{
            id: "requirements".to_string(),
            description: "Extracts and refines user requirements from input".to_string(),
            skills: SkillGraph {
                root: Capability::Requirements,
                subskills: vec![],
            },
            input_schema: "UserGoal".to_string(),
            output_schema: "RequirementList".to_string(),
            default_execution: ExecutionMode::Simulate,
        }
    }
    pub fn new() -> Self{
        RequirementsAgent
    }
}

impl AgentHandler for RequirementsAgent{
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[RequirementsAgent] Starting requirements extraction.....");

        // 1. Assemble prompt
        let prompt_result = PromptAssembler::assembler("requirements_agent",&task, &ctx.project);
        if let Err(err) = prompt_result{
            return AgentResponse::error(&format!("Prompt Assembly failed: {}", err), false);
        }
        let prompt = prompt_result.unwrap();

        // 2. Query LLM
        let llm_tool = LLMTool::new();
        let query_result = llm_tool.query(prompt);

        if let Err(err) = query_result {
            return AgentResponse::error(&format!("LLM query failed: {}", err),false);
        }
        let llm_output = query_result.unwrap();

        println!("[RequirementsAgent] Received output from LLM.");

        // 3. Save to project memory
        if let Err(err) = ctx.project.save_to_project_memory("requirements.json", &llm_output) {
            return AgentResponse::error(&format!("Saving requirements failed: {}", err), false);
        }

        // 4. Log a design decision for memory enrichment
        let decision = DesignDecision{
            id: Uuid::new_v4().to_string(),
            summary: summary.to_string(),
            made_by: "RequirementsAgent".to_string(),
            rationale: rationale.to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
        };

        // Log into ProjectMemory
        ctx.project.write_decision(decision);


        // Return the requirements as agent output
        AgentResponse::success("Requirements generated and saved".to_string(), "RequirementsAgent")
    }
}