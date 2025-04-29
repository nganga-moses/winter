use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;
use uuid::Uuid;
use crate::memory::project_memory::DesignDecision;
use crate::orchestrator::context::AgentContext;
use crate::orchestrator::protocol::AgentResponse;
use crate::orchestrator::registry::AgentHandler;
use crate::orchestrator::types::{AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph};
use crate::orchestrator::utils::write_json_to_project_file;

pub struct ArchitectureAgent;

impl ArchitectureAgent {
    pub fn card()-> AgentCard{
        AgentCard {
            id: "architecture".into(),
            description: "Designs the software architecture for the project".to_string(),
            skills: SkillGraph {
                root: Capability::Architecture,
                subskills: vec![],
            },
            input_schema: "RequirementList".into(),
            output_schema: "ArchitecturePlan".into(),
            default_execution: ExecutionMode::Simulate,
        }
    }
}

impl AgentHandler for ArchitectureAgent {
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[ArchitectureAgent] Generating architecture for task: {}", task.task_id);

        // Simulated LLM response
        let architecture_plan = json!({
            "type": "modular",
            "components": ["AuthService", "UserService", "Frontend", "Database"],
            "tech_stack": {
                "backend": "Rust + Axum",
                "frontend": "React",
                "db": "PostgreSQL"
            },
            "summary": "Use modular architecture with clean separation of concerns",
            "rationale": "Based on project scope and team expertise in Rust and React"
        });

        // Extract simulated fields for decision logging
        let summary = architecture_plan["summary"].as_str().unwrap_or("Unknown summary");
        let rationale = architecture_plan["rationale"].as_str().unwrap_or("No rationale provided");

        ctx.project.set_architecture(&architecture_plan.to_string());

        let decision = DesignDecision{
            id: Uuid::new_v4().to_string(),
            summary: summary.to_string(),
            made_by: "ArchitectureAgent".to_string(),
            rationale: rationale.to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
        };

        ctx.project.lock().unwrap().write_decisions(decision.clone());

        let output = json!({
            "architecture": &serde_json::to_string(&architecture_plan).unwrap_or_default(),
            "summary": decision.summary,
            "rationale": decision.rationale
        });

        write_json_to_project_file("architecture.json", &output).ok();

        AgentResponse::success(
            &output.to_string(),
            "ArchitectureAgent")
    }
}