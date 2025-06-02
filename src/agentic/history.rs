use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Default, Deserialize)]
pub struct History {
    context: Vec<Message>,
}

impl History {
    pub fn new(system_prompt: &str) -> History {
        let mut temp = History {
            context: Vec::new(),
        };

        temp.add(Message {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        });

        temp
    }

    pub fn print_history(&self) -> String {
        let mut output = String::new();

        for message in &self.context {
            output.push_str(&format!(
                "Role: {}\nContent: {}\n\n",
                message.role, message.content
            ));
        }

        output
    }

    pub fn print_latest_history(&self) -> String {
        let mut output = String::new();

        if let Some(message) = self.context.last() {
            output.push_str(&format!(
                "Role: {}\nContent: {}\n\n",
                message.role, message.content
            ));
        }

        output
    }

    pub fn add(&mut self, message: Message) {
        self.context.push(message);
    }

    pub fn remove_latest(&mut self) {
        if !self.context.is_empty() {
            self.context.pop();
        }
    }
}
