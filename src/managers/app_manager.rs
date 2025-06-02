use crate::consts::prompts::PATTERN_OUTPUT_FORMAT_PROMPT;
use crate::integrations::openwebui::openwebui_service::OpenWebUIService;
use crate::utils::config::Config;
use crate::utils::text_extraction::{extract_text, get_input_or_stdin};
use eyre::Result;
use std::collections::HashMap;
use std::fs;

pub struct AppManager {
    config: Config,
    patterns: HashMap<String, String>,
    patterns_dir: String,

    owui_client: OpenWebUIService,
}

impl AppManager {
    pub fn new(pattern_directory: Option<String>) -> Result<AppManager> {
        let config_struct = Config::load_configuration_struct();

        let patterns_directory = pattern_directory
            .or_else(|| {
                if !config_struct.patterns_dir.is_empty() {
                    Some(config_struct.patterns_dir.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| String::from("./patterns"));

        tracing::debug!("patterns dir: {}", config_struct.patterns_dir);
        tracing::debug!("owui base url: {}", config_struct.owui_base_url);
        tracing::debug!(
            "owui auth token len: {}",
            config_struct.owui_auth_token.len()
        );

        let mut app_manager = AppManager {
            config: config_struct.clone(),
            patterns: HashMap::new(),
            patterns_dir: patterns_directory,
            owui_client: OpenWebUIService::new(
                config_struct.owui_base_url.as_str(),
                config_struct.owui_auth_token.as_str(),
            ),
        };

        app_manager.load_patterns()?;

        Ok(app_manager)
    }

    fn load_patterns(&mut self) -> Result<()> {
        let entries = fs::read_dir(self.patterns_dir.clone())?;

        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            if let Ok(path) = entry.path().into_os_string().into_string() {
                                self.patterns.insert(file_name, path);
                            }
                        }
                    }
                }
            }
        }

        tracing::debug!("loaded {} patterns", self.patterns.len());

        Ok(())
    }

    fn read_pattern(&mut self, pattern_name: &str) -> Result<String> {
        if let Some(pattern_path) = self.patterns.get(pattern_name) {
            let content = fs::read_to_string(format!("{}/pattern.md", pattern_path))?;
            Ok(content)
        } else {
            Err(eyre::eyre!("pattern '{}' not found", pattern_name))
        }
    }

    pub fn list_patterns(&mut self) {
        for (pattern_name, pattern_path) in &self.patterns {
            tracing::debug!("pattern: {} -> {}", pattern_name, pattern_path);
        }
    }

    pub fn view_pattern(&mut self, pattern_name: &str) {
        match self.read_pattern(pattern_name) {
            Ok(content) => {
                tracing::debug!("Pattern '{}'\n\n{}", pattern_name, content);
            }
            Err(e) => tracing::error!("Failed to read pattern: {}", e),
        }
    }

    pub fn process_pattern(
        &mut self,
        model_name: Option<String>,
        pattern_name: &str,
        query: Option<String>,
    ) -> Result<String> {
        // Attempt to load the pattern
        let pattern_data = self.read_pattern(pattern_name)?;

        // Get the input for the query
        let input = get_input_or_stdin(query.to_owned());

        if input.is_empty() {
            return Err(eyre::eyre!("input is empty"));
        }
        
        let model = model_name.unwrap_or_else(|| self.config.model_name.clone());

        let completion = self.owui_client.completion(
            &model,
            format!(
                "{}\n{}",
                pattern_data.as_str(),
                PATTERN_OUTPUT_FORMAT_PROMPT
            )
            .as_str(),
            input.as_str(),
        )?;

        let Ok(extracted_text) = extract_text(completion.as_str(), "<--OUTPUT-->", "<!!OUTPUT!!>")
        else {
            return Err(eyre::eyre!("failed to extract output from completion"));
        };

        Ok(extracted_text)
    }

    pub fn process_raw(
        &mut self,
        model_name: Option<String>,
        prompt: &str,
        query: Option<String>,
    ) -> Result<String> {

        // Get the input for the query
        let input = get_input_or_stdin(query.to_owned());

        if input.is_empty() {
            return Err(eyre::eyre!("input is empty"));
        }

        let model = model_name.unwrap_or_else(|| self.config.model_name.clone());

        let completion = self.owui_client.completion(
            &model,
            format!("{}\n{}", prompt, PATTERN_OUTPUT_FORMAT_PROMPT).as_str(),
            input.as_str(),
        )?;

        let Ok(extracted_text) = extract_text(completion.as_str(), "<--OUTPUT-->", "<!!OUTPUT!!>")
        else {
            return Err(eyre::eyre!("failed to extract output from completion"));
        };

        Ok(extracted_text)
    }
}
