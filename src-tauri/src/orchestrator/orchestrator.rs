use crate::memory::planner_memory::{PlannerMemory, PlannerMemoryEntry};
use crate::memory::runtime_memory::DesignDecision;
use crate::memory::session_memory::SessionMemoryHandle;
use crate::memory::task_memory::TaskMemoryHandle;
use crate::memory::{
    global_memory::GlobalMemoryHandle, project_memory::ProjectMemory,
    session_memory::SessionMemory, task_memory::TaskMemory,
};
use crate::orchestrator::context::AgentContext;
use crate::orchestrator::feedback::load_feedback_queue;
use crate::orchestrator::hash::calculate_plan_hash;
use crate::orchestrator::protocol::{
    AgentError, AgentOutput, AgentResponse, PlannerOutput,
};
use crate::orchestrator::registry::{AgentHandler, AgentMetadata, AgentRegistry};
use crate::orchestrator::task_index::{append_to_task_index, TaskIndexEntry};
use crate::orchestrator::task_log::write_task_log;
use crate::orchestrator::timeline::{append_timeline_event, TimelineEvent};
use crate::orchestrator::types::{
    now_timestamp, AgentCard, AgentTask, AgentTaskContext, Capability, TaskStatus,
};
use crate::tools::registry::ToolRegistry;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::{uuid, Uuid};

const MAX_RETRIES: usize = 3;
const MAX_PLANNER_REVISIONS: u8 = 3;
const PLANNER_RETRY_THRESHOLD: u8 = 7;

pub struct Orchestrator {
    registry: AgentRegistry,
    tool_registry: Arc<ToolRegistry>,
    pub task_memory: TaskMemory,
    pub session_memory: SessionMemory,
    pub project_memory: ProjectMemoryHandle,
    pub global_memory: GlobalMemoryHandle,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            registry: AgentRegistry::new(),
            tool_registry: Arc::new(ToolRegistry::new()),
            task_memory: TaskMemory::new(),
            session_memory: SessionMemory::new(),
            project_memory: ProjectMemory::default(),
            global_memory: GlobalMemoryHandle::default(),
        }
    }
    /// register Agents into the orchestrator
    pub fn register_agent(
        &mut self,
        card: AgentCard,
        handler: Box<dyn AgentHandler + Send + Sync>,
    ) {
        self.registry.register(card, handler);
    }
    pub fn execute_task_graph(
        &self,
        task_graph: Vec<AgentTask>,
        ctx: AgentContext,
    ) -> AgentResponse {
        for task in task_graph {
            println!("Executing planner-subtask: {}", task.task_type);

            let result = self.handle(task.clone(), ctx.clone());
            match result {
                AgentResponse::Error(_) => {
                    println!("Halting chain due to failure");
                    return result;
                }
                AgentResponse::Success(_) => {
                    println!("Chained task succeeded");
                }
            }
        }
        AgentResponse::success("All planner tasks executed", "orchestrator")
    }

    /// Routes an AgentTask to the appropriate agent by capability
    pub fn handle(&self, mut task: AgentTask, mut ctx: AgentContext) -> AgentResponse {
        task.status = TaskStatus::Running;
        let task_id = task.task_id.clone();
        let task_type = task.task_type.clone();

        let Ok(capability) = task.task_type.parse::<Capability>() else {
            task.status = TaskStatus::Failed {
                reason: "Unknown task type or capability.".into(),
            };
            return AgentResponse::error("Unknown task type or capability.", false);
        };

        let Some(agent) = self.registry.find_agent_for_task(&capability) else {
            task.status = TaskStatus::Failed {
                reason: "No agent available for this task.".into(),
            };
            let _ = ctx.task.lock().unwrap().save(
                &task_id,
                &format!("Failed: no agent for capability {}", task_type),
            );
            return AgentResponse::error("No agent available for this task", false);
        };

        ctx.tool_registry = self.tool_registry.clone();

        //Execute the Agent task
        let response = agent.handle_task(task.clone(), ctx.clone());

        // Check if it's a planner output and requires critique before running sub-tasks
        if let AgentResponse::Success(output) = &response {
            if capability == Capability::Planning {
                if let Ok(plan) = serde_json::from_value::<PlannerOutput>(output.clone()) {
                    println!("[Orchestrator] Plan received, routing to CritiqueAgent...");

                    let critique_task = AgentTask {
                        task_id: uuid::Uuid::new_v4().to_string(),
                        task_type: "evaluation".to_string(),
                        payload: serde_json::to_string(&output).unwrap_or_default(),
                        context: task.context.clone(),
                        status: TaskStatus::Pending,
                    };

                    let critique_response = self.handle(critique_task.clone(), ctx.clone());

                    response = match critique_response {
                        AgentResponse::Success(eval_output) => {
                            let score = eval_output.score.unwrap_or(10);
                            let revision = task.context.revision_id.unwrap_or(0);

                            if score < PLANNER_RETRY_THRESHOLD && revision < MAX_PLANNER_REVISIONS {
                                println!("[Orchestrator] Critique score {score} < threshold. Retrying Planner...");

                                let mut retry_task = task.clone();
                                retry_task.task_id = uuid::Uuid::new_v4().to_string();
                                retry_task.context.retry_of = Some(task.task_id.clone());
                                retry_task.context.revision_id = Some(revision + 1);
                                retry_task.status = TaskStatus::Pending;

                                return self.handle(retry_task, ctx);
                            }

                            println!("[Orchestrator] Critique approved. Executing plan...");
                            let entry = PlannerMemoryEntry {
                                plan_id: plan.plan_id.clone(),
                                goal_id: task
                                    .context
                                    .goal_id
                                    .clone()
                                    .unwrap_or_else(|| "unknown".to_string()),
                                score: eval_output.score,
                                status: TaskStatus::Succeeded,
                                feedback_tags: None,
                                revision_id: plan.revision_id,
                                plan_hash: Some(calculate_plan_hash(&plan.task_graph)),
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            };
                            ctx.planner_memory.add_entry(&entry.goal_id, entry);

                            let decision = DesignDecision {
                                id: format!("plan-{}", plan.plan_id),
                                summary: format!(
                                    "Planner used {:?} strategy with score {}",
                                    plan.strategy_used,
                                    eval_output.score.unwrap_or_default()
                                ),
                                made_by: "PlannerAgent".into(),
                                rationale: plan.feedback_notes.unwrap_or_else(|| "N/A".into()),
                                timestamp: now_timestamp().to_string(),
                            };
                            ctx.project.lock().unwrap().write_decision(decision);

                            self.execute_task_graph(plan.task_graph, ctx.clone())
                        }
                        AgentResponse::Error(err) => {
                            println!("[Orchestrator] Plan rejected: {:?}", err.reason);
                            AgentResponse::Error(err)
                        }
                    };
                } else {
                    response = AgentResponse::error("Planner returned invalid task graph", false);
                }
            }
        }

        // Handle dynamically emitted subtasks from any agent
        if let AgentResponse::Success(output) = &response {
            if let Some(subtasks) = &output.subtasks {
                println!("↪ Executing {} chained subtasks", subtasks.len());

                for subtask in subtasks {
                    let mut enriched = subtask.clone();
                    enriched.context.parent_task_id = Some(task.task_id.clone());
                    enriched.context.goal_id =
                        enriched.context.goal_id.or(task.context.goal_id.clone());
                    let result = self.handle(enriched, ctx.clone());
                    println!("↪ Subtask result: {:?}", result);
                }
            }
        }

        // Update status
        task.status = match &response {
            AgentResponse::Success(_) => TaskStatus::Succeeded,
            AgentResponse::Error(err) => TaskStatus::Failed {
                reason: err.reason.clone(),
            },
        };

        log_task_result(&task, &response, ctx.task.clone());

        // Write persistent task log to disk
        if let Err(e) = write_task_log(&task, &response) {
            eprintln!("[warn] Failed to write disk task log: {e}");
        }

        // Save to session memory
        if let AgentResponse::Success(output) = &response {
            if let Ok(json) = serde_json::to_string(&output) {
                ctx.session.save(&task.task_id, &json);
            }
        }

        let index_entry = TaskIndexEntry {
            task_id: task.task_id.clone(),
            agent_id: agent.card.id.clone(),
            task_type: task.task_type.clone(),
            status: format!("{:?}", task.status),
            goal_id: task.context.goal_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            revision_id: task.context.revision_id,
        };

        let _ = append_to_task_index(&index_entry);
        append_timeline_event(
            &task.context.goal_id.clone().unwrap_or("unknown".into()),
            TimelineEvent::Task {
                task_id: task.task_id.clone(),
                task_type: task.task_type.clone(),
                status: format!("{:?}", task.status),
                agent_id: agent.card.id.clone(),
                timestamp: now_timestamp(),
            },
        );
        response
    }
    pub fn execute_reviewed_plan(
        &self,
        planner_output: PlannerOutput,
        ctx: AgentContext,
    ) -> AgentResponse {
        println!("Executing task graph from Planner...");
        self.execute_task_graph(planner_output.task_graph, ctx)
    }
    pub fn process_feedback_queue(
        &self,
        task: TaskMemoryHandle,
        session: SessionMemoryHandle,
        project: ProjectMemoryHandle,
        global: GlobalMemoryHandle,
    ) {
        let Ok(queue) = load_feedback_queue() else {
            println!("⚠️ Could not load feedback queue.");
            return;
        };

        for item in queue {
            if !item.retry_recommended {
                continue;
            }

            let retry_key = format!("retries_for:{}", item.task_id);
            let retry_count = task
                .lock()
                .unwrap()
                .load(&retry_key)
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            if retry_count >= MAX_RETRIES {
                let msg = format!("Retry limit reached ({MAX_RETRIES}). Task aborted.");
                println!("⚠️ {}", msg);

                task.lock()
                    .unwrap()
                    .save(&format!("retry_skipped:{}", item.task_id), &msg);

                append_to_task_index(TaskIndexEntry {
                    task_id: item.task_id.clone(),
                    status: "RetrySkipped".into(),
                    agent_id: "Unknown".into(),
                    goal_id: item
                        .original_task
                        .context
                        .goal_id
                        .clone()
                        .unwrap_or_default(),
                    timestamp: now_timestamp(),
                    task_type: "".to_string(),
                    revision_id: None,
                })
                .expect("error appending Index entry");

                continue;
            }

            let mut retry_task = item.original_task.clone();
            retry_task.task_id = uuid::Uuid::new_v4().to_string();
            retry_task.context.retry_of = Some(item.task_id.clone());
            retry_task.status = TaskStatus::Pending;

            task.lock()
                .unwrap()
                .save(&retry_key, &(retry_count + 1).to_string());

            println!(
                "🔁 Retrying task: {} (attempt #{})",
                retry_task.task_id,
                retry_count + 1
            );

            let ctx = AgentContext {
                task: task.clone(),
                session: session.clone(),
                project: project.clone(),
                global: global.clone(),
                tool_registry: self.tool_registry.clone(),
                planner_memory: PlannerMemory::new(),
            };

            let response = self.handle(retry_task.clone(), ctx);

            println!("✅ Retry result: {:?}", response);
        }
    }
    /// Returns a list of all available agent cards
    pub fn available_agents(&self) -> Vec<AgentCard> {
        self.registry.all_cards()
    }
}
pub fn log_task_result(
    task: &AgentTask,
    response: &AgentResponse,
    task_memory: Arc<Mutex<TaskMemory>>,
) {
    let task_id = &task.task_id;
    let log = match response {
        AgentResponse::Success(AgentOutput {
            content,
            execution_time_ms,
            tool_invocations,
            trace,
            evaluation_notes,
            ..
        }) => {
            format!(
                "✅ Task Succeeded\nType: {}\nOutput: {}\nTime: {}ms\nTools: {:?}\nTrace: {:?}\nNotes: {:?}",
                task.task_type,
                content,
                execution_time_ms,
                tool_invocations,
                trace,
                evaluation_notes
            )
        }

        AgentResponse::Error(AgentError {
            reason,
            retryable,
            log_trace,
        }) => {
            format!(
                "❌ Task Failed\nType: {}\nReason: {}\nRetryable: {}\nTrace: {:?}",
                task.task_type, reason, retryable, log_trace
            )
        }
    };

    let _ = task_memory.lock().unwrap().save(task_id, &log);
}
