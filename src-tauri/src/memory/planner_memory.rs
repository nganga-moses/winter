use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlannerMemoryEntry {
    pub plan_id: String,
    pub goal_id: Option<String>,
    pub score: Option<u8>,
    pub status: String,
    pub feedback_tags: Option<Vec<String>>,
    pub revision_id: Option<u32>,
    pub plan_hash: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Default, Clone)]
pub struct PlannerMemory {
    inner: Arc<Mutex<HashMap<String, Vec<PlannerMemoryEntry>>>>,
}
impl PlannerMemory {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn add_entry(&self, goal_id: &str, entry: PlannerMemoryEntry) {
        let mut memory = self.inner.lock().unwrap();
        memory.entry(goal_id.to_string()).or_default().push(entry);
    }
    pub fn get_history(&self, goal_id: &str) -> Option<Vec<PlannerMemoryEntry>> {
        self.inner.lock().unwrap().get(goal_id).cloned()
    }
    pub fn handle(&self) -> Arc<Mutex<HashMap<String, Vec<PlannerMemoryEntry>>>> {
        Arc::clone(&self.inner)
    }
}
pub fn log_planner_memory_entry(entry: &PlannerMemoryEntry) -> std::io::Result<()> {
    let folder = dirs::home_dir()
        .expect("No home dir")
        .join("WinterData/memory");
    create_dir_all(&folder)?;

    let file_path: PathBuf = folder.join("planner_memory.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let line = serde_json::to_string(entry)?;
    writeln!(file, "{}", line)?;

    Ok(())
}

pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
