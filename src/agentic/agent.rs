use std::ops::Add;

use crate::agentic::history::History;
use crate::agentic::tools::tool::Tool;
use crate::consts::prompts::{
    AGENTIC_INCORRECT_OUTPUT_FORMAT_PROMPT, AGENTIC_MANAGER_PROMPT, AGENTIC_TEAM_PROMPT,
};
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Agent {
    pub role: String,
    pub goal: String,
    pub persona: String,
    pub is_manager: bool,
    #[serde(skip)]
    pub context: History,
}

impl Agent {
    pub fn from_json(json_str: &str) -> eyre::Result<Self> {
        let mut agent: Agent = serde_json::from_str(json_str)?;
        agent.context = History::new(agent.persona.as_str());
        Ok(agent)
    }

    pub fn construct_manager_prompt(
        &self,
        task: &str,
        agents_details: &str,
    ) -> Result<String, std::io::Error> {
        let agent_info = format!(
            "\nPersona: {}\nRole: {}\nGoal: {}\n",
            self.role, self.goal, self.persona
        );

        let mut prompt = AGENTIC_MANAGER_PROMPT.replace("<--INJECT_CREW_HERE-->", &agents_details);
        prompt = prompt.replace("<--INJECT_AGENT_INFO-->", &agent_info);
        prompt = prompt.add(format!("\nTask: {}\n", task).as_str());
        Ok(prompt)
    }

    pub fn construct_team_prompt(
        &self,
        task: &str,
        tools: &Vec<Box<dyn Tool>>,
    ) -> Result<String, std::io::Error> {
        let agent_info = format!(
            "\nPersona: {}\nRole: {}\nGoal: {}\n",
            self.role, self.goal, self.persona
        );

        let tool_info = tools
            .iter()
            .map(|tool| tool.get_tool_prompt())
            .collect::<Vec<_>>()
            .join("\n");

        let mut prompt = AGENTIC_TEAM_PROMPT.replace("<--INJECT_TOOLS_HERE-->", tool_info.as_str());
        prompt = prompt.replace("<--INJECT_AGENT_INFO-->", &agent_info);
        prompt = prompt.add(format!("\nTask: {}\n", task).as_str());
        Ok(prompt)
    }

    pub fn construct_invalid_format_prompt(&self) -> Result<String, std::io::Error> {
        Ok(AGENTIC_INCORRECT_OUTPUT_FORMAT_PROMPT.to_string())
    }
}
