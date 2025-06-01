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
