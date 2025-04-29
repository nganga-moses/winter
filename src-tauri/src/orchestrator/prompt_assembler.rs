use crate::agents::orchestrator::context::AgentContext;
use crate::agents::orchestrator::types::AgentTask;
use serde_json::Value::String;
use std::fmt::Write;

/// Builds structured prompts for agents that need LLM assistance
/// Will be extended later with context injection, summarization etc
pub struct PromptAssembler;

impl PromptAssembler {
    pub fn build_prompt(task: &AgentTask, ctx: &AgentContext) -> String {
        let mut prompt = String::new();

        //Header
        writeln!(prompt, "You are the {} agent.", task.task_type).ok();
        writeln!(prompt, " Your goal is to complete the following task:\n").ok();
        writeln!(prompt, "{}", task.payload).ok();

        // Include memory context
        if let Some(goal_id) = &task.context.goal_id {
            if let Some(memory) = ctx.session.load(goal_id) {
                writeln!(prompt, "\nRelevant memory:\n{}", memory).ok();
            }
        }
        writeln!(
            prompt,
            "\nPlease reason step-by-step and output structured results."
        )
        .ok();

        prompt
    }
}
