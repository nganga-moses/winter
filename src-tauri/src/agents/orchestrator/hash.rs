use sha2::{Digest, Sha256};
use serde_json;
use crate::agents::orchestrator::types::AgentTask;

pub fn calculate_plan_hash(task_graph: &Vec<AgentTask>) -> String {
    let serialized = serde_json::to_string(task_graph).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
}