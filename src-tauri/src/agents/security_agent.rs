use serde_json::json;
use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::protocol::AgentResponse;
use crate::agents::orchestrator::registry::AgentHandler;
use crate::agents::orchestrator::types::{AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph};

pub struct SecurityAgent;

impl SecurityAgent {
    pub fn card() -> AgentCard {
        AgentCard {
            id: "security".into(),
            description: "Analyzes and recommends security best practices".into(),
            skills: SkillGraph {
                root: Capability::Security,
                subskills: vec![],
            },
            input_schema: "CodePatch".into(),
            output_schema: "SecurityReview".into(),
            default_execution: ExecutionMode::Simulate,
        }
    }
}

impl AgentHandler for SecurityAgent {
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[SecurityAgent] Performing simulated security scan...");

        if let Some(tool) = ctx.tool_registry.get("CodeScanTool") {
            let input = json!({
                "scope": "entire_project",
                "focus": "vulnerabilities",
            });

            let result = futures::executor::block_on(tool.run(input));

            match result {
                Ok(tool_return) if tool_return.status.is_success() => {
                    let output = json!({
                        "scan": tool_return.result,
                        "summary": "Security scan completed with no critical issues"
                    });

                    AgentResponse::success(&output.to_string(), "SecurityAgent")
                },
                Ok(tool_return) => {
                    let output = json!({
                        "warning": tool_return.result,
                        "summary": "Security scan completed with warnings"
                    });

                    AgentResponse::success(&output.to_string(), "SecurityAgent")
                },
                Err(e) => {
                    AgentResponse::error(&format!("Security tool error: {e}"), false)
                }
            }
        } else {
            AgentResponse::error("CodeScanTool not found in registry", false)
        }
    }
}