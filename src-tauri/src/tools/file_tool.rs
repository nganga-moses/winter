use async_trait::async_trait;
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::path::Path;
use crate::orchestrator::protocol::{ToolReturn, ToolStatus};
use crate::agents::tools::tool::Tool;

pub struct FileTool;

#[async_trait]
impl Tool for FileTool{
    fn name(&self) -> &'static str {
        "FileTool"
    }

    fn description(&self) -> &'static str {
        "Reads from and writes to the file system"
    }

    async fn run(&self, input: Value) -> Result<ToolReturn, String> {
        let action = input["action"].as_str().unwrap_or("read");
        let path = input["path"].as_str().ok_or("Missing path")?;

        match action {
            "read" => {
                let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
                Ok(ToolReturn{
                    result: json!({"content": content}),
                    status: ToolStatus::Success,
                    trace: Some(vec![format!("Read from file: {}", path)]),
                })
            },
            "write" => {
                let content = input["content"].as_str().ok_or("Missing content")?;
                fs::write(path, content).map_err(|e| e.to_string())?;
                Ok(ToolReturn{
                    result: json!({"message": "File written successfully"}),
                    status: ToolStatus::Success,
                    trace: Some(vec![format!("Wrote to file: {}", path)]),
                })
            },
            _ =>Err("Unsupported action".into())
        }
    }
}