use eyre::Result;

#[must_use]
pub fn get_input_or_stdin(query: Option<String>) -> String {
    query.unwrap_or_else(|| {
        let stdin = std::io::stdin();
        let mut lines = String::new();
        for line in stdin.lines() {
            if let Ok(line) = line {
                lines.push_str(&line);
                lines.push('\n');
            }
        }
        lines.trim_end().to_string()
    })
}

#[must_use]
pub fn extract_text(text_input: &str, prefix: &str, suffix: &str) -> Result<String> {

    if let Some(start) = text_input.find(prefix) {
        if let Some(end) = text_input[start + prefix.len()..].find(suffix) {
            return Ok(text_input[start + prefix.len()..start + prefix.len() + end].to_string());
        }
    }

    Err(eyre::eyre!("Could not find prefix '{}' and suffix '{}' in completion", prefix, suffix))
}