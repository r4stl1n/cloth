use crate::utils::config::Config;
use crate::utils::text_extraction::get_user_input;
use clap::Subcommand;
use std::fs;

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    // List all supported patterns
    View {},

    // View a specific pattern
    Setup {},
}

pub fn execute(args: &ConfigCommands) {

    match args {
        ConfigCommands::View {} => {
            let config_path = match Config::get_config_path() {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("failed to get config path: {e}");
                    return;
                }
            };

            tracing::info!(
                "reading config file from: {}",
                config_path.to_string_lossy().to_string()
            );

            let content = match fs::read_to_string(config_path.to_string_lossy().to_string()) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("failed to read file: {e}");
                    return;
                }
            };

            tracing::info!("Config Contents:\n\n{}", content);
        }
        ConfigCommands::Setup {} => {
            let config_path = match Config::get_config_path() {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("failed to get config path: {e}");
                    return;
                }
            };

            tracing::info!(
                "writing config file to: {}",
                config_path.to_string_lossy().to_string()
            );

            let Ok(base_url) = get_user_input("OpenWebUI URL > ") else {
                tracing::error!("failed to read user input");
                return;
            };

            let Ok(auth_token) = get_user_input("OpenWebUI Auth Token > ") else {
                tracing::error!("failed to read user input");
                return;
            };

            let Ok(model) = get_user_input("Model > ") else {
                tracing::error!("failed to read user input");
                return;
            };

            if let Some(parent) = config_path.parent() {
                match fs::create_dir_all(parent) {
                    Ok(()) => {}
                    Err(e) => {
                        tracing::error!(
                            "failed to create directory path: {} - {e}",
                            parent.to_string_lossy().to_string()
                        );
                        return;
                    }
                }
            }

            let config_data = Config {
                model_name: model,
                owui_base_url: base_url,
                owui_auth_token: auth_token,
            };

            let Ok(json_data) = serde_json::to_string_pretty(&config_data) else {
                tracing::error!("failed to marshall config data");
                return;
            };

            // Create the file (this will overwrite if it exists)
            match fs::write(&config_path, json_data) {
                Ok(()) => tracing::info!("config file wrote"),
                Err(e) => tracing::error!("failed to write config {e}"),
            }
        }
    }
}
