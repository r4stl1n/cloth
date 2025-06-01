use crate::utils::env_utils::get_env_or_default;
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub model_name: String,
    pub owui_base_url: String,
    pub owui_auth_token: String,
}

impl Config {
    /// Load the configuration struct
    pub fn load_configuration_struct() -> Config {
        let config = Config::load();

        match config {
            Ok(cfg) => {
                tracing::debug!("config file loaded");
                Config {
                    model_name: cfg.model_name,
                    owui_base_url: cfg.owui_base_url,
                    owui_auth_token: cfg.owui_auth_token,
                }
            }
            Err(_) => {
                tracing::warn!("config file not found, loading from env");
                Config {
                    model_name: String::new(),
                    owui_base_url: get_env_or_default("OWUI_BASE_URL", "http://localhost:3000"),
                    owui_auth_token: get_env_or_default("OWUI_AUTH_TOKEN", ""),
                }
            }
        }
    }

    /// Load configuration from ~/.config/cloth/config.json
    fn load() -> Result<Self> {
        let config_path =
            Self::get_config_path().context("Failed to determine config file path")?;

        if !config_path.exists() {
            return Err(eyre::eyre!(
                "Config file not found: {}",
                config_path.display()
            ));
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        Ok(config)
    }

    /// Get the path to the config file (~/.config/cloth/config.json)
    pub fn get_config_path() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| eyre::eyre!("Could not determine home directory"))?;

        Ok(home_dir.join(".config").join("cloth").join("config.json"))
    }
}
