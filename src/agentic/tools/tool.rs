use eyre::Result;

pub trait Tool {
    fn name(&self) -> String;
    fn example(&self) -> String;
    fn description(&self) -> String;
    fn run(&self, data: &str) -> Result<String>;
}

impl dyn Tool {
    pub fn get_tool_prompt(&self) -> String {
        format!("{}\n{}\n{}\n ", self.name(), self.description(), self.example())
    }
}