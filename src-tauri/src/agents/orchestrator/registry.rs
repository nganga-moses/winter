use  std::collections::HashMap;
use super::job_card::AgentJobCard;

#[derive(Default)]
pub struct AgentRegistry{
    agents: HashMap<String, AgentJobCard>,
}

impl AgentRegistry{
    pub fn new()-> Self{
        Self{
            agents: HashMap::new()
        }
    }
    pub fn register(&mut self,card:AgentJobCard){
        self.agents.insert(card.name.clone(),card);
    }

    pub fn list(&self)-> Vec<AgentJobCard>{
        self.agents.values().collect()
    }

    pub fn get(&self, name: &str)-> Option<&AgentJobCard>{
        self.agents.get(name)
    }
}