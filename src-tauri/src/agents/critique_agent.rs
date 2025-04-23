use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::feedback::{
    write_feedback_item, write_plan_feedback_item, FeedbackItem, PlanFeedbackAction,
    PlanFeedbackItem,
};
use crate::agents::orchestrator::protocol::{
    AgentOutput, AgentResponse, EvaluationLevel, EvaluationNote, PlannerOutput,
};
use crate::agents::orchestrator::registry::AgentHandler;
use crate::agents::orchestrator::types::{
    AgentCard, AgentTask, AgentTaskContext, Capability, ExecutionMode, SkillGraph,
};

pub struct CritiqueAgent;

impl CritiqueAgent {
    pub fn card() -> AgentCard {
        AgentCard {
            id: "critique".into(),
            description: "Evaluates agent responses and adds feedback notes".into(),
            skills: SkillGraph {
                root: Capability::Evaluation,
                subskills: vec![],
            },
            input_schema: "AgentOutput".into(),
            output_schema: "EvaluationNote".into(),
            default_execution: ExecutionMode::Simulate,
        }
    }
}

impl AgentHandler for CritiqueAgent {
    fn handle_task(&self, task: AgentTask, ctx: AgentContext) -> AgentResponse {
        println!("[CritiqueAgent] Reviewing planner output...");

        //Placeholder critique review
        let parsed: Result<PlannerOutput, _> = serde_json::from_str(&task.payload);
        let Ok(plan) = parsed else {
            return AgentResponse::error("Invalid planner output payload", false);
        };

        // Placeholder evaluation logic
        let notes = vec![EvaluationNote {
            note: "Plan contains only 2 steps. Consider adding testing.".to_string(),
            level: EvaluationLevel::Warn,
        }];
        let feedback = PlanFeedbackItem {
            plan_id: plan.plan_id.clone(),
            goal_id: "".to_string(),
            notes: notes.clone(),
            score: Some(6),
            action: PlanFeedbackAction::Revise,
        };

        if let Err(e) = write_plan_feedback_item(&feedback) {
            eprintln!("Failed to write plan feedback: {}", e);
        }

        AgentResponse::Success(AgentOutput {
            content: serde_json::json!({
                "summary": "Critique complete",
                "notes": notes.iter().map(|n| &n.note).collect::<Vec<_>>(),
            }),
            execution_time_ms: 42,
            tool_invocations: vec![],
            trace: None,
            evaluation_notes: Some(notes),
            score: Some(6),
            produced_by: "CritiqueAgent".into(),
            planned_by: None,
            subtasks: None,
        })
    }
}
