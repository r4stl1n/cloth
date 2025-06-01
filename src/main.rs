mod commands;
mod managers;
mod integrations;
mod utils;

use clap::{Parser, Subcommand};
use crate::commands::openwebui;
use crate::commands::pattern;


#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

    /// OpenWebUi related commands
    OWUI {

        #[command(subcommand)]
        commands: openwebui::OpenWebUiCommands,
    },

    /// Pattern-related commands
    Pattern {

        #[arg(long, global = true)]
        pattern_directory: Option<String>,

        #[command(subcommand)]
        commands: pattern::PatternCommands,
    },
}

fn main() {

    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    let args = Args::parse();

    match &args.cmd {

        Commands::OWUI {
           commands
        } => {
            openwebui::execute(commands)
        },

        Commands::Pattern { pattern_directory, commands  } => {
            pattern::execute(pattern_directory.clone(), commands)
        }

    }
}