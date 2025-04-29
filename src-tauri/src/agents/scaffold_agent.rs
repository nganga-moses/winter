use serde_json::json;
use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::protocol::AgentResponse;
use crate::agents::orchestrator::registry::AgentHandler;
use crate::agents::orchestrator::types::{AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph};

pub struct ScaffoldAgent;

impl ScaffoldAgent {
    pub fn card() -> AgentCard {
        AgentCard {
            id: "scaffold".into(),
            description: "Bootstraps new project or component structures".into(),
            skills: SkillGraph {
                root: Capability::Scaffolding,
                subskills: vec![],
            },
            input_schema: "ArchitecturePlan".into(),
            output_schema: "FileStructure".into(),
            default_execution: ExecutionMode::Execute,
        }
    }
}

impl AgentHandler for ScaffoldAgent {
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[ScaffoldAgent] Writing stubs for architecture...");

        // Simulated file stubs
        let files = vec![
            ("src/services/auth.rs", "// TODO: Implement AuthService"),
            ("src/services/events.rs", "// TODO: Implement EventService"),
            ("src/main.rs", "fn main() { println!(\"App bootstrap\"); }")
        ];

        let mut written = vec![];

        for (path, content) in files {
            if let Some(tool) = ctx.tool_registry.get("FileTool") {
                let result = futures::executor::block_on(tool.run(json!({
                    "action": "write",
                    "path": path,
                    "content": content
                })));

                match result {
                    Ok(tool_return) if tool_return.status.is_success() => {
                        written.push(path.to_string());
                    },
                    Ok(tool_return) => {
                        println!("[ScaffoldAgent] Write warning: {:?}", tool_return.status);
                    },
                    Err(e) => {
                        println!("[ScaffoldAgent] Tool error: {}", e);
                    }
                }
            }
        }

        let output = json!({
            "written": written,
            "summary": "Scaffolded initial files based on architecture"
        });

        AgentResponse::success(&output.to_string(), "ScaffoldAgent")
    }
}