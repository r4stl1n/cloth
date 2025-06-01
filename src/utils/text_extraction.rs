use std::io;
use std::io::Write;

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

pub fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?; // Ensure prompt is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Remove trailing newline
    Ok(input.trim().to_string())
}
