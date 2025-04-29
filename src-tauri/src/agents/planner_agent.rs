use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;
use sha2::digest::consts::U32;
use uuid::Uuid;
use crate::memory::planner_memory::PlannerMemoryEntry;
use crate::orchestrator::protocol::{AgentResponse, AgentOutput, PlannerOutput};
use crate::orchestrator::types::{AgentCard, AgentTask, AgentTaskContext, Capability, ExecutionMode, SkillGraph, TaskStatus};
use crate::orchestrator::context::AgentContext;
use crate::orchestrator::feedback::{load_feedback_queue, load_plan_feedback_queue, PlanFeedbackAction};
use crate::orchestrator::planning::meta_planner::{HeuristicMetaPlanner, MetaPlanner};
use crate::orchestrator::registry::AgentHandler;


const MAX_REVISIONS: i32 = 3;

pub struct PlannerAgent;

impl PlannerAgent {
    pub fn card()-> AgentCard {
        AgentCard {
            id: "planner".to_string(),
            description: "Generates multi-task plans for agents based on a high-level goal".to_string(),
            skills: SkillGraph {
                root: Capability::Planning,
                subskills: vec![Capability::Evaluation],
            },
            input_schema: "Goal".to_string(),
            output_schema: "PlannerOutput".to_string(),
            default_execution: ExecutionMode::Simulate,
        }
    }
    pub fn new() -> Self{
        PlannerAgent
    }
}

impl AgentHandler for PlannerAgent{
    fn handle_task(&self, task: AgentTask, _ctx: AgentContext) -> AgentResponse {
        println!("[PlannerAgent] Planning next steps...");

        // Step 1: Extract project goal
        let goal = match task.payload.get("goal") {
            Some(g) => g.as_str().unwrap_or("").to_string(),
            None => {
                return AgentResponse::error("PlannerAgent received task without a goal field.");
            }
        };

        // Step 2: Create a RequirementsAgent task
        let req_task = AgentTask {
            task_id: format!("requirements_{}", uuid::Uuid::new_v4()),
            task_type: Capability::Requirements,
            payload: json!({
                "goal": goal
            }),
            context: _ctx,
            status: TaskStatus::Pending,
        };

        // Step 3: Return new task as output
        let output = json!({
            "tasks": [req_task]
        });

        AgentResponse::success(&output.to_string(), "planner_agent")
    }
    
}
