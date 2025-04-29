use crate::agents::architecture_agent::ArchitectureAgent;
use crate::agents::codegen_agent::CodegenAgent;
use crate::agents::critique_agent::CritiqueAgent;
use crate::agents::deployment_agent::DeploymentAgent;
use crate::agents::doc_agent::DocAgent;
use crate::agents::hello_agent::HelloAgent;
use crate::orchestrator::orchestrator::Orchestrator;
use crate::orchestrator::types::{AgentCard, Capability, ExecutionMode, SkillGraph};
use crate::agents::planner_agent::PlannerAgent;
use crate::agents::refactor_agent::RefactorAgent;
use crate::agents::repo_agent::RepoAgent;
use crate::agents::requirements_agent::RequirementsAgent;
use crate::agents::scaffold_agent::ScaffoldAgent;
use crate::agents::security_agent::SecurityAgent;
use crate::agents::test_agent::TestAgent;
use crate::tools::echo_tool::EchoTool;
use crate::tools::llm_planner::LLMPlannerTool;
use crate::tools::registry::ToolRegistry;

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
        PlannerAgent::card(),
        Box::new(PlannerAgent),
    );
    orchestrator.register_agent(
        CritiqueAgent::card(),
        Box::new(CritiqueAgent),
    );
    orchestrator.register_agent(
        RequirementsAgent::card(),
        Box::new(RequirementsAgent)
    );
    orchestrator.register_agent(
        ArchitectureAgent::card(),
        Box::new(ArchitectureAgent)
    );
    orchestrator.register_agent(
        CodegenAgent::card(),
        Box::new(CodegenAgent)
    );
    orchestrator.register_agent(
        TestAgent::card(),
        Box::new(TestAgent)
    );
    orchestrator.register_agent(
        RefactorAgent::card(),
        Box::new(RefactorAgent)
    );
    orchestrator.register_agent(
        DocAgent::card(),
        Box::new(DocAgent)
    );
    orchestrator.register_agent(
        ScaffoldAgent::card(),
        Box::new(ScaffoldAgent)
    );
    orchestrator.register_agent(
        DeploymentAgent::card(),
        Box::new(DeploymentAgent)
    );
    orchestrator.register_agent(
        SecurityAgent::card(),
        Box::new(SecurityAgent)
    );
    orchestrator.register_agent(
        RepoAgent::card(),
        Box::new(RepoAgent)
    );
}
pub fn register_all_tools(tool_registry: &mut ToolRegistry) {
    tool_registry.register_tool(Box::new(EchoTool));
    tool_registry.register_tool(Box::new(LLMPlannerTool));
}