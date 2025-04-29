#[derive(Debug, Clone)]
pub struct AgentJobCard{
    pub name: String,
    pub description: String,
    pub input_type: String,
    pub out_type: String,
}
impl AgentJobCard{
    pub fn new(name: &str, description: &str, input_type: &str, output_type: &str)->Self{
        Self{
            name: name.into(),
            description: description.into(),
            input_type: input_type.into(),
        }
    }
}