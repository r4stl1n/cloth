use crate::utils::text_extraction::extract_text;
use eyre::Result;
use serde_derive::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct RBOPResponse {
    pub thought: String,
    pub action: String,
    pub data: String,
    pub data2: String,
}

impl RBOPResponse {
    pub fn from_completion(completion: &str) -> Result<RBOPResponse> {
        let thought = extract_text(completion, "<--THOUGHT-->", "<!!THOUGHT!!>")?;
        let action = extract_text(completion, "<--ACTION-->", "<!!ACTION!!>")?;
        let data = extract_text(completion, "<--DATA-->", "<!!DATA!!>")?;
        let data2 = extract_text(completion, "<--DATA2-->", "<!!DATA2!!>").unwrap_or(String::new());

        Ok(RBOPResponse {
            thought,
            action,
            data,
            data2,
        })
    }

    pub fn print_result(&self, agent_name: &str) {
        tracing::info!("{} - [Thought]: {}", agent_name, self.thought);
        tracing::info!("{} - [Action]: {}", agent_name, self.action);
        tracing::info!("{} - [Data]: {}", agent_name, self.data);
        tracing::info!("{} - [Data2]: {}", agent_name, self.data2);
    }
}
