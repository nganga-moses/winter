use crate::agents::critique_agent::CritiqueAgent;
use crate::agents::hello_agent::HelloAgent;
use crate::agents::orchestrator::orchestrator::Orchestrator;
use crate::agents::orchestrator::types::{AgentCard, Capability, ExecutionMode, SkillGraph};
use crate::agents::planner_agent::PlannerAgent;
use crate::agents::tools::echo_tool::EchoTool;
use crate::agents::tools::llm_planner::LLMPlannerTool;
use crate::agents::tools::registry::ToolRegistry;

pub fn register_all_agents(orchestrator: &mut Orchestrator){
    orchestrator.register_agent(
        AgentCard{
            id: "hello".to_string(),
            description: "Greets the user".to_string(),
            input_schema: "text".to_string(),
            output_schema: "text".to_string(),
            default_execution: ExecutionMode::Simulate,
            skills: SkillGraph {
                root: Capability::Greeting,
                subskills: vec![]
            },
        },
        Box::new(HelloAgent),
    );
    orchestrator.register_agent(
        PlannerAgent.card(),
        Box::new(PlannerAgent),
    );
    orchestrator.register_agent(
        CritiqueAgent.card(),
        Box::new(CritiqueAgent),
    );
}
pub fn register_all_tools(tool_registry: &mut ToolRegistry) {
    tool_registry.register_tool(Box::new(EchoTool));
    tool_registry.register_tool(Box::new(LLMPlannerTool));
}