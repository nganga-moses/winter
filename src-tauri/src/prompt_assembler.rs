use crate::memory::project_memory::ProjectMemory;
use crate::orchestrator::types::AgentTask;
use anyhow::{Context, Result};
use std::fs;

/// Responsible for assembling the final prompt to be sent to the LLM.
pub struct PromptAssembler;

impl PromptAssembler {
    /// Assemble the full prompt from file + memory _ task.
    pub fn assemble(agent_id: &str, task: &AgentTask, memory: &ProjectMemory) -> Result<String> {
        // 1. Load static agent prompt
        let prompt_path = format!("prompts/{}.txt", agent_id);
        let static_prompt = fs::read_to_string(&prompt_path)
            .with_context(|| format!("Failed to load static prompt: {}", prompt_path))?;

        // 2. Pull relevant memory context
        let context = Self::gather_context(agent_id, task, memory)?;

        // 3. Add task payload
        let task_input = format!("{:#}", task.payload);

        // 4. Assemble the final prompt
        let final_prompt = format!(
            "### Agent Role Prompt\n{}\n### Context\n{}\n\n### Task Input\n{}\n",
            static_prompt.trim(),
            context.trim(),
            task_input.trim(),
        );

        Ok(final_prompt)
    }
}

/// Gather memory based on agent type.
fn gather_context(agent_id: &str, _task: &AgentTask, memory: &ProjectMemory) -> Result<String> {
    match agent_id {
        "requirements_agent" => {
            let goals = memory.read("project_memory/goals.md").unwrap_or_default();
            let rules = memory.read("project/design_rules.md").unwrap_or_default();
            Ok(format!("{}\n\n{}", goals, rules))
        }
        "architecture_agent" => {
            let requirements = memory
                .read("project_memory/requirements.json")
                .unwrap_or_default();
            let rules = memory
                .read("project_memory/design_rules.md")
                .unwrap_or_default();
            Ok(format!("{}\n\n{}", requirements, rules))
        }
        "codegen_agent" => {
            let architecture = memory.summarize("project_memory/architecture.yaml", "module")?;
            let rules = memory
                .read("project_memory/design_rules.md")
                .unwrap_or_default();
            Ok(format!("{}\n\n{}", architecture, rules))
        }
        "critique_agent" => {
            // Critique works on task input directly (e.g file or output to review)
            Ok("Review the following output carefully.".to_string())
        }
        "refactor_agent" => {
            let rules = memory
                .read("project_memory/design_rules.md")
                .unwrap_or_default();
            Ok(rules)
        }
        "test_agent" => {
            let requirements = memory.summarize("project_memory/requirements.json", "testing")?;
            Ok(requirements)
        }
        "deployment_agent" => {
            let architecture = memory.summarize("project_memory/architecture.yaml", "infra")?;
            Ok(architecture)
        }
        _ => {
            Ok("".to_string()) // Default: no extra context
        }
    }
}
