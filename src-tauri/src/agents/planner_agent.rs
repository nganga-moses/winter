use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;
use sha2::digest::consts::U32;
use uuid::Uuid;
use crate::agents::memory::planner_memory::PlannerMemoryEntry;
use crate::agents::orchestrator::protocol::{AgentResponse, AgentOutput, PlannerOutput};
use crate::agents::orchestrator::types::{AgentCard, AgentTask, AgentTaskContext, Capability, ExecutionMode, SkillGraph, TaskStatus};
use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::feedback::{load_feedback_queue, load_plan_feedback_queue, PlanFeedbackAction};
use crate::agents::orchestrator::planning::meta_planner::{HeuristicMetaPlanner, MetaPlanner};
use crate::agents::orchestrator::registry::AgentHandler;

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
}

impl AgentHandler for PlannerAgent{
    fn handle_task(&self, task: AgentTask, mut ctx: AgentContext) -> AgentResponse {
        println!("[PlannerAgent] Selecting plan strategy...");

        let planner_memory = ctx.planner_memory.lock().unwrap();
        let history = planner_memory.get_history(&task.context.goal_id.clone().unwrap_or_default());
        
        let strategy = HeuristicMetaPlanner.recommend_strategy(
            &task.context.goal_id.clone().unwrap_or_default(),
            &history,
        );

        let goal_id = task.context.goal_id.clone().unwrap_or("unknown".into());
        let mut revision_id = task.context.revision_id.unwrap_or(0);

        // Check for feedback that requests a revision
        if let Ok(feedbacks) = load_plan_feedback_queue(){
            if let Some(critique) = feedbacks.iter().find(|f| f.goal_id == goal_id && f.action == PlanFeedbackAction::Revise){
                if revision_id >= MAX_REVISIONS{
                    let msg = format!("⚠️ Max revisions ({MAX_REVISIONS}) reached for goal: {goal_id}");
                    println!("{msg}");

                    ctx.planner_memory.add_entry(&goal_id, PlannerMemoryEntry{
                        plan_id: Uuid::new_v4().to_string(),
                        goal_id: Some(goal_id.clone()),
                        score: critique.score,
                        status: "Rejected".into(),
                        feedback_tags: None,
                        revision_id: Some(revision_id),
                        plan_hash: None,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    });
                    return AgentResponse::error("Max revisions hit for plan", false);
                }
                println!(" Revising plan for goal_id={goal_id} (revision {})", revision_id);
                revision_id+=1;
            }
        }

        //Placeholder plan -TODO evolve to dynamic graph planning
        let task_graph = vec![
            AgentTask{
                task_id: uuid::Uuid::new_v4().to_string(),
                task_type: "clarification".to_string(),
                payload: r#"{"question" : "What user roles should this sysyem support?"}"#.to_string(),
                context: task.context.clone(),
                status: TaskStatus::Pending,
            },
            AgentTask{
                task_id: uuid::Uuid::new_v4().to_string(),
                task_type: "architecture".to_string(),
                payload: r#"{"based_on": "clarified requirements"}"#.to_string(),
                context: task.context.clone(),
                status: TaskStatus::Pending,
            },
        ];

        let plan_id = Uuid::new_v4().to_string();

        let plan_output = PlannerOutput{
            plan_id: plan_id.clone(),
            task_graph,
            score: None,
            revision_id: Some(revision_id),
            feedback_notes: None,
            strategy_used: strategy.clone(),
        };

        AgentResponse::success(&serde_json::to_string(&plan_output).unwrap_or_default(), "planner")

    }
}