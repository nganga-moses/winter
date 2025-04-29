use serde_json::json;
use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::protocol::AgentResponse;
use crate::agents::orchestrator::registry::AgentHandler;
use crate::agents::orchestrator::types::{AgentCard, AgentTask, Capability, ExecutionMode, SkillGraph};

pub struct TestAgent;

impl TestAgent {
    pub fn card() -> AgentCard {
        AgentCard {
            id: "test".into(),
            description: "Generates and evaluates tests for code patches".into(),
            skills: SkillGraph {
                root: Capability::Testing,
                subskills: vec![],
            },
            input_schema: "CodePatch".into(),
            output_schema: "TestSuite".into(),
            default_execution: ExecutionMode::Execute,
        }
    }
}

impl AgentHandler for TestAgent {
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[TestAgent] Generating test stubs...");

        // Simulated test generation
        let modules = vec!["auth", "events"];
        let mut generated_tests = vec![];

        for module in modules {
            if let Some(tool) = ctx.tool_registry.get("TestGenTool") {
                let input = json!({
                    "module": module,
                    "style": "unit",
                    "lang": "rust"
                });

                let result = futures::executor::block_on(tool.run(input));

                match result {
                    Ok(tool_return) if tool_return.status.is_success() => {
                        generated_tests.push(json!({
                            "module": module,
                            "test": tool_return.result
                        }));
                    },
                    Ok(tool_return) => {
                        println!("[TestAgent] Warning: {:?}", tool_return.status);
                    },
                    Err(e) => {
                        println!("[TestAgent] Error generating tests for {module}: {e}");
                    }
                }
            }
        }

        let output = json!({
            "tests": generated_tests,
            "summary": "Test stubs generated for key modules"
        });

        AgentResponse::success(&output.to_string(), "TestAgent")
    }
}