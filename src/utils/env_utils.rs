use std::env;

#[must_use]
pub fn get_env_or_default(env_val: &str, default_val: &str) -> String {
    match env::var(env_val) {
        Ok(d) => d,
        Err(_) => default_val.to_string(),
    }
}
