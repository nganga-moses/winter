use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::protocol::{AgentOutput, AgentResponse};
use crate::agents::orchestrator::registry::{AgentHandler};
use crate::agents::orchestrator::types::{AgentTask, AgentTaskContext, TaskStatus};

pub struct HelloAgent;

impl AgentHandler for HelloAgent {
    fn handle_task(&self, task: AgentTask, mut ctx: AgentContext) -> AgentResponse {
       let reply = format!("👋 Hello! You said: {}", task.payload);

        //Emit a follow up task
        let follow_up = AgentTask{
            task_id: uuid::Uuid::new_v4().to_string(),
            task_type: "echo".to_string(),
            payload: r#"{"message: "This is a subtask from HelloAgent"}"#.into(),
            context: AgentTaskContext{
                origin: "hello".into(),
                goal_id: task.context.goal_id.clone(),
                parent_task_id: Some(task.task_id.clone()),
                retry_of:None,
            },
            status: TaskStatus::Pending,
        };

        AgentResponse::Success(AgentOutput{
            content: serde_json::json!({"message": reply}),
            execution_time_ms: 5,
            tool_invocations: vec![],
            trace: None,
            evaluation_notes: None,
            score: None,
            produced_by: "HelloAgent".into(),
            planned_by: None,
            subtasks: Some(vec![follow_up]),
        })
    }
}