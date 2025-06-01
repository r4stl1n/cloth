use crate::integrations::openwebui::openwebui_service::OpenWebUIService;
use crate::utils::config::Config;
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

    let config_struct = Config::load_configuration_struct();

    let mut owui_client = OpenWebUIService::new(config_struct.owui_base_url.as_str(),
                                                config_struct.owui_auth_token.as_str());

    tracing::debug!("owui base url: {}", config_struct.owui_base_url);
    tracing::debug!("owui auth token len: {}", config_struct.owui_auth_token.len());

    match args {
        OpenWebUiCommands::ListModels {} => match owui_client.print_models() {
            Ok(()) => {}
            Err(e) => tracing::error!("failed to get models {e}"),
        },

        OpenWebUiCommands::Completion { model, query } => {
            let input = get_input_or_stdin(query.to_owned());

            match owui_client.complete(model, input.as_str()) {
                Ok(data) => {
                    tracing::debug!("completion: {}", data)
                }
                Err(e) => {
                    tracing::error!("failed to get completion: {e}")
                }
            }
        }
    }
}
