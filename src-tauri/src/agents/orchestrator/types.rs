use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
/// Shared Types

#[derive(Debug, Clone)]
pub struct AgentTaskContext{
    pub origin: String,
    pub goal_id: Option<String>,
    pub parent_task_id: Option<String>,
    pub retry_of: Option<String>,
    pub revision_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentTask{
    pub task_id: String, // Unique identifier for orchestration and traceability
    pub task_type: String, // e.g., "code_gen", "clarification", "review"
    pub payload: String,
    pub context: AgentTaskContext,
    pub status: TaskStatus,
}

#[derive(Debug, Clone,PartialEq, Eq, Hash)]
pub enum Capability{
    CodeGen,
    Planning,
    Evaluation,
    FileAccess,
    GitOps,
    Search,
    Research,
    Reasoning,
    Clarification,
    Greeting
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum  ExecutionMode{
    Simulate,
    DryRun,
    Execute,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentCard{
    pub id: String,
    pub description: String,
    pub skills: SkillGraph,
    pub input_schema: String, // Optional JSON schema
    pub output_schema: String, // Optional JSON schema
    pub default_execution: ExecutionMode,
}
/// Enables skill matching, dependency resolution, and graph traversal
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct SkillGraph {
    pub root: Capability,
    pub subskills: Vec<Capability>,
    /// NOTE: This will be refactored in Phase 4 to a HashMap-based graph for introspection,
    /// subskill matching, and agent chaining. This simplified version keeps capability
    /// matching clean while Winter operates in single-layer mode.
}
#[derive(Debug,Clone)]
pub enum TaskStatus{
    Pending,
    Running,
    Succeeded,
    Failed { reason: String},
    Retried { previous_id: String},
}
pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}