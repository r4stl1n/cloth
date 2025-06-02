mod commands;
mod integrations;
mod managers;
mod utils;
mod consts;
mod agentic;

use crate::commands::pattern;
use crate::commands::{configc, openwebui};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Config related command
    Config {
        #[command(subcommand)]
        commands: configc::ConfigCommands,
    },

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
        Commands::Config { commands } => configc::execute(commands),
        Commands::OWUI { commands } => openwebui::execute(commands),
        Commands::Pattern {
            pattern_directory,
            commands,
        } => pattern::execute(pattern_directory.clone(), commands),
    }
}
