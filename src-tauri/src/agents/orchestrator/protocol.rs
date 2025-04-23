use crate::agents::orchestrator::types::AgentTask;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Messaging structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    pub content: serde_json::Value, // Final result (e.g., code, analysis, summary, etc.)
    pub execution_time_ms: u64,
    pub tool_invocations: Vec<String>,
    pub trace: Option<Vec<String>>,
    pub evaluation_notes: Option<Vec<EvaluationNote>>,
    pub score: Option<u8>,
    pub produced_by: String,
    pub planned_by: Option<String>,
    pub subtasks: Option<Vec<AgentTask>>,
}

#[derive(Debug, Clone)]
pub struct AgentError {
    pub reason: String,
    pub retryable: bool,
    pub log_trace: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum AgentResponse {
    Success(AgentOutput),
    Error(AgentError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationLevel {
    Info,
    Warn,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationNote {
    pub note: String,
    pub level: EvaluationLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ToolStatus {
    Success,
    Warning,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolReturn {
    pub result: Value,
    pub status: ToolStatus,
    pub trace: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub name: String,
    pub input: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolInvocationLog {
    pub tool_name: String,
    pub input: Value,
    pub output: ToolReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannerOutput {
    pub task_graph: Vec<AgentTask>,     //Linear for now, DAg later
    pub score: Option<u8>,              //Set by CritiqueAgent(0-10)
    pub feedback_notes: Option<String>, // CritiqueAgent writes reasoning
    pub plan_id: String,
    pub revision_id: Option<u32>,
    pub strategy_used: PlanningStrategy,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlanningStrategy {
    ReusePlan { plan_id: String },
    ReviseLast { plan_id: String },
    GenerateFresh,
}

impl AgentResponse {
    pub fn success(content: &str, agent_id: &str) -> Self {
        AgentResponse::Success(AgentOutput {
            content: serde_json::Value::String(content.into()),
            execution_time_ms: 0,
            tool_invocations: vec![],
            trace: None,
            evaluation_notes: None,
            score: None,
            produced_by: agent_id.to_string(),
            planned_by: None,
            subtasks: None,
        })
    }
    pub fn error(reason: &str, retryable: bool) -> Self {
        AgentResponse::Error(AgentError {
            reason: reason.to_string(),
            retryable,
            log_trace: None,
        })
    }
}
