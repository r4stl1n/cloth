use crate::integrations::openwebui::openwebui_service::OpenWebUIService;
use crate::utils::text_extraction::get_input_or_stdin;
use eyre::Result;
use std::collections::HashMap;
use std::fs;

pub struct AppManager {
    patterns: HashMap<String, String>,
    patterns_dir: String,

    owui_client: OpenWebUIService,
}

impl AppManager {
    pub fn new(patterns_directory: &str) -> Result<AppManager> {
        let mut app_manager = AppManager {
            patterns: HashMap::new(),
            patterns_dir: patterns_directory.to_string(),

            owui_client: OpenWebUIService::new(),
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

        tracing::info!("loaded {} patterns", self.patterns.len());

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
            tracing::info!("pattern: {} -> {}", pattern_name, pattern_path);
        }
    }

    pub fn view_pattern(&mut self, pattern_name: &str) {
        match self.read_pattern(pattern_name) {
            Ok(content) => {
                tracing::info!("Pattern '{}'\n\n{}", pattern_name, content);
            }
            Err(e) => tracing::error!("Failed to read pattern: {}", e),
        }
    }

    pub fn process_pattern(
        &mut self,
        model_name: &str,
        pattern_name: &str,
        query: Option<String>,
    ) -> Result<String> {
        // Attempt to load the pattern
        let pattern_data = self.read_pattern(pattern_name)?;

        // Get the input for the query
        let input = get_input_or_stdin(query.to_owned());

        let completion = self
            .owui_client
            .complete(model_name, format!("{}\n{}", pattern_data, input).as_str())?;

        Ok(completion)
    }
}
