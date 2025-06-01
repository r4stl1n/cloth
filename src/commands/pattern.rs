use crate::managers::app_manager::AppManager;
use clap::Subcommand;
use std::process::exit;

#[derive(Subcommand, Debug)]
pub enum PatternCommands {
    // List all supported patterns
    List {},

    // View a specific pattern
    View {
        /// Name of the pattern
        #[clap(long)]
        pattern: String,
    },

    // Process a pattern
    Process {
        /// Name of the model
        #[clap(long)]
        model: Option<String>,

        /// Name of the pattern
        #[clap(long)]
        name: String,

        /// Query to process
        #[clap(long)]
        query: Option<String>,
    },
}

pub fn execute(pattern_directory: Option<String>, args: &PatternCommands) {
    let pattern_dir = pattern_directory.unwrap_or_else(|| String::from("./patterns"));

    let mut app_manager = match AppManager::new(pattern_dir.as_str()) {
        Ok(app) => app,
        Err(e) => {
            tracing::error!("failed to start app_manager: {e}");
            exit(1)
        }
    };

    match args {
        PatternCommands::List {} => app_manager.list_patterns(),

        PatternCommands::View { pattern } => app_manager.view_pattern(pattern.as_str()),

        PatternCommands::Process { model, name, query } => {
            match app_manager.process_pattern(model.clone(), name, query.clone()) {
                Ok(data) => {
                    println!("{}", data)
                }
                Err(e) => {
                    tracing::error!("failed to run pattern: {}", e)
                }
            }
        }
    }
}
