use crate::agents::tools::registry::ToolRegistry;
use crate::agents::tools::echo_tool::EchoTool;

pub fn register_all_tools(registry: &mut ToolRegistry){
    registry.register_tool(Box::new(EchoTool));
}