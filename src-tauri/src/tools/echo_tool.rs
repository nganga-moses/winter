use serde_json::{json, Value};
use crate::orchestrator::protocol::{ToolReturn, ToolStatus};
use crate::tools::tool::{Tool};

pub struct EchoTool;

impl Tool for EchoTool{
    fn name(&self) -> &'static str{ "echo" }
    fn description(&self) -> &'static str { "Repeats whatever input is given"}

    async fn run(&self, input: Value) -> ToolReturn {
        ToolReturn{
            result: json!({"echoded": input}),
            status: ToolStatus::Success,
            trace: Some(vec!["EchoTool::run".into()]),
        }
    }
}