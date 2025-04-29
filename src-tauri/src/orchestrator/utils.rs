use crate::agents::memory::task_memory::TaskMemory;
use crate::agents::orchestrator::types::AgentTask;

pub fn retry_depth(task: &AgentTask, memory: &TaskMemory) -> usize{
    let mut depth = 0;
    let mut current = task.context.retry_of.clone();
    while let Some(prev_id) = current{
        depth +=1;
        current = memory.load(&prev_id)
            .and_then(|val| serde_json::from_str::<AgentTask>(&val).ok())
            .and_then(|t| t.context.retry_of);
    }
    depth
}
pub fn write_json_to_project_file(path: &str, data: &serde_json::Value) -> std::io::Result<()> {
    let base = dirs::home_dir().unwrap().join("WinterData/project");
    std::fs::create_dir_all(&base)?;
    let full = base.join(path);
    std::fs::write(full, serde_json::to_string_pretty(data)?)?;
    Ok(())
}