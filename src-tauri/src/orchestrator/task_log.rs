use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::agents::orchestrator::protocol::{AgentResponse};
use serde::{Deserialize, Serialize};
use crate::agents::orchestrator::types::AgentTask;

#[derive(Serialize, Deserialize)]
pub struct TaskLogEntry {
    pub timestamp: u64,
    pub task: AgentTask,
    pub response: AgentResponse,
}

pub fn write_task_log(task: &AgentTask, response: AgentResponse) -> std::io::Result<()> {
    let log_entry = TaskLogEntry {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        task: task.clone(),
        response: response.clone(),
    };

    let data = serde_json::to_string_pretty(&log_entry)?;
    let folder = dirs::home_dir()
        .expect("No home dir")
        .join("WinterData/logs/tasks");
    fs::create_dir_all(&folder)?;

    let path: PathBuf = folder.join(format!("{}.json", task.task_id));
    fs::write(path, data)?;
    Ok(())
}
