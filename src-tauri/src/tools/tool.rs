use async_trait::async_trait;
use serde_json::Value;
use serde::{Serialize,Deserialize};


/// Tools take structured JSON input and return structured JSON output.
#[async_trait]
pub trait Tool: Send + Sync{
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn run(&self, input: Value) -> ToolReturn;
}

