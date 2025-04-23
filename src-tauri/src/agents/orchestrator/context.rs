use crate::agents::memory::{
    task_memory::TaskMemoryHandle,
    session_memory::SessionMemoryHandle,
    project_memory::ProjectMemoryHandle,
    global_memory::GlobalMemoryHandle,
};
use crate::agents::memory::planner_memory::PlannerMemory;
use crate::agents::tools::registry::ToolRegistry;

#[derive(Clone)]
pub struct AgentContext {
    pub task: TaskMemoryHandle,
    pub session: SessionMemoryHandle,
    pub project: ProjectMemoryHandle,
    pub global: GlobalMemoryHandle,
    pub tool_registry: std::sync::Arc<ToolRegistry>,
    pub planner_memory: PlannerMemory,
}