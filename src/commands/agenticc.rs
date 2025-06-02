use clap::Subcommand;
use std::process::exit;

use crate::managers::agentic_manager::AgenticManager;

#[derive(Subcommand, Debug)]
pub enum AgenticCommands {
    /// Process an agentic task
    Process {
        /// Name of the model
        #[clap(long)]
        model: Option<String>,

        /// Team
        #[clap(long)]
        team: String,

        /// Task to process
        #[clap(long)]
        task: Option<String>,
    },
}

pub fn execute(teams_directory: Option<String>, args: &AgenticCommands) {
    let mut agentic_manager = match AgenticManager::new(teams_directory) {
        Ok(app) => app,
        Err(e) => {
            tracing::error!("failed to start agentic_manager: {e}");
            exit(1)
        }
    };

    match args {
        AgenticCommands::Process { model, team, task } => {
            match agentic_manager.process_task(model.clone(), team, task.clone()) {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("agentic processing error: {}", e);
                }
            }
        }
    }
}
