use async_trait::async_trait;
use serde_json::{json, Value};
use crate::orchestrator::protocol::{ToolReturn, ToolStatus};
use crate::tools::tool::Tool;

pub struct LLMPlannerTool;

#[async_trait]
impl Tool for LLMPlannerTool {
    fn name(&self) -> &'static str {
        "llm_planner"
    }

    fn description(&self) -> &'static str {
        "Generates a multi-step plan based on a goal using a language model"
    }

    async fn run(&self, input: Value) -> Result<ToolReturn, String> {
        // A stub for now, we will add an LLM call later
        println!("[LLMPlannerTool] Simulating plan for: {}", input);

        let simulated_plan = json!({
            "task_graph": [
                { "task_type": "clarify", "payload": "Clarify goal"},
                { "task_type": "design", "payload": "Propose architecture"}
            ]
        });

        Ok(ToolReturn {
            result: simulated_plan,
            status: ToolStatus::Success,
            trace: Some(vec!["stub::plan_generated".into()]),
        })
    }
}
