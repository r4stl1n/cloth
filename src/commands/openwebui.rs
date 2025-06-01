use crate::integrations::openwebui::openwebui_service::OpenWebUIService;
use crate::utils::text_extraction::get_input_or_stdin;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum OpenWebUiCommands {
    // List all supported patterns
    ListModels {},

    // View a specific pattern
    Completion {
        /// Name of the model
        #[clap(long)]
        model: String,

        /// Query to complete
        #[clap(long)]
        query: Option<String>,
    },
}

pub fn execute(args: &OpenWebUiCommands) {
    let mut owui_client = OpenWebUIService::new();

    match args {
        OpenWebUiCommands::ListModels {} => match owui_client.print_models() {
            Ok(()) => {}
            Err(e) => tracing::error!("failed to get models {e}"),
        },

        OpenWebUiCommands::Completion { model, query } => {
            let input = get_input_or_stdin(query.to_owned());

            match owui_client.complete(model, input.as_str()) {
                Ok(data) => {
                    tracing::info!("completion: {}", data)
                }
                Err(e) => {
                    tracing::error!("failed to get completion: {e}")
                }
            }
        }
    }
}
