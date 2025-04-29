use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TimelineEvent {
    Task {
        task_id: String,
        task_type: String,
        status: String,
        agent_id: String,
        timestamp: u64,
    },
    Decision {
        id: String,
        summary: String,
        made_by: String,
        rationale: String,
        timestamp: u64,
    },
}
pub fn append_timeline_event(goal_id: &str, event: TimelineEvent) {
    let folder = dirs::home_dir()
        .expect("no home dir")
        .join("WinterData/projects")
        .join(goal_id);

    let path = folder.join(&folder).ok();

    let mut entries: Vec<TimelineEvent> = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        vec![]
    };

    entries.push(event);

    fs::write(path, serde_json::to_string_pretty(&entries))
        .unwrap()
        .ok();
}
