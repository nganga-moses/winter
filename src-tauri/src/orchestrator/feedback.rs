use crate::agents::orchestrator::protocol::EvaluationNote;
use crate::agents::orchestrator::types::AgentTask;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

const FEEDBACK_PATH: &str = "WinterData/feedback_queue.json";
const PLAN_QUEUE_PATH: &str = "WinterData/plan_feedback_queue.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeedbackItem {
    pub task_id: String,
    pub original_task: AgentTask,
    pub evaluation_notes: String,
    pub score: Option<u8>,
    pub retry_recommended: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanFeedbackItem {
    pub plan_id: String,
    pub goal_id: String,
    pub notes: Vec<EvaluationNote>,
    pub score: Option<u8>,
    pub action: PlanFeedbackAction,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanFeedbackAction {
    Revise,
    Replan,
    Reject,
}
pub fn write_feedback_item(item: &FeedbackItem) -> std::io::Result<()> {
    let path = dirs::home_dir().unwrap().join(FEEDBACK_PATH);
    let mut list = load_feedback_queue().unwrap_or_default();

    list.push(item.clone());
    let json = serde_json::to_string_pretty(&list)?;
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, json)?;
    Ok(())
}
pub fn load_feedback_queue() -> std::io::Result<Vec<FeedbackItem>> {
    let path = dirs::home_dir().unwrap().join(FEEDBACK_PATH);
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let items = serde_json::from_reader(reader)?;
    Ok(items)
}
pub fn load_plan_feedback_queue() -> std::io::Result<Vec<PlanFeedbackItem>> {
    let path = PathBuf::from(dirs::home_dir().unwrap()).join(PLAN_QUEUE_PATH);
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(path)?;
    let items: Vec<PlanFeedbackItem> = serde_json::from_str(&data)?;
    Ok(items)
}
pub fn write_plan_feedback_item(item: &PlanFeedbackItem) -> std::io::Result<()> {
    let mut queue = load_plan_feedback_queue().unwrap_or_default();
    queue.push(item.clone());

    let path = PathBuf::from(dirs::home_dir().unwrap()).join(PLAN_QUEUE_PATH);
    fs::create_dir_all(path.parent().unwrap())?;
    let data = serde_json::to_string_pretty(&queue)?;
    fs::write(path, data)
}
