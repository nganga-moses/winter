use std::collections::HashMap;
use super::tool::Tool;

/// Provides capability for composable utilities eg ShellTol, GitTool, SearchTool
pub struct ToolRegistry{
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new()-> Self{
        Self {
            tools: HashMap::new()
        }
    }
    pub fn register_tool(&mut self, tool: Box<dyn Tool>){
        self.tools.insert(tool.name().to_string(), tool);
    }
    pub fn get(&self, name: &str)-> Option<&Box<dyn Tool>>{
        self.tools.get(name)
    }
    pub fn all(&self) -> Vec<String>{
        self.tools.keys().cloned().collect()
    }
}