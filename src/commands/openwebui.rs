use crate::integrations::openwebui::openwebui_service::OpenWebUIService;
use clap::Subcommand;
use crate::utils::text_extraction::get_input_or_stdin;

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
    let mut openwebui_clinet = OpenWebUIService::new();

    match args {
        OpenWebUiCommands::ListModels {} => match openwebui_clinet.print_models() {
            Ok(()) => {}
            Err(e) => tracing::error!("failed to get models {e}"),
        },

        OpenWebUiCommands::Completion { model, query } => {

            let input = get_input_or_stdin(query.to_owned());

            match openwebui_clinet.complete(model,input.as_str()) {
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
