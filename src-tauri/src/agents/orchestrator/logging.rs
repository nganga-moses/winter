use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};
use uuid::Timestamp;
use crate::agents::orchestrator::protocol::{AgentResponse};
use crate::agents::orchestrator::types::{AgentTask,TaskStatus};

#[derive(Serialize, Deserialize)]
pub struct TaskLogEntry{
    pub task: AgentTask,
    pub response: AgentResponse,
    pub timestamp: u64,
}

pub fn log_task_to_disk(task: &AgentTask, response: &AgentResponse){
    let task_id = &task.task_id;
    let logs_dir = get_logs_dir().join("tasks");
    let _=fs::create_dir_all(&logs_dir);

    let path = logs_dir.join(format!("{}.json", task_id));
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let log = TaskLogEntry{
        task: task.clone(),
        response: response.clone(),
        timestamp,
    };

    let content = serde_json::to_string(&log).unwrap_or_else(|_| "{}".to_string());
    let _ =fs::write(path, content);
}