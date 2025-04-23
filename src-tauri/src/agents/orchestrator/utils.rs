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