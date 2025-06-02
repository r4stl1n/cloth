mod agentic;
mod commands;
mod consts;
mod integrations;
mod managers;
mod utils;

use crate::commands::{agenticc, pattern};
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
    /// Agentic functionality
    Agentic {
        /// Team Directory
        #[arg(long, global = true)]
        teams_directory: Option<String>,

        #[command(subcommand)]
        commands: agenticc::AgenticCommands,
    },

    /// Config management
    Config {
        #[command(subcommand)]
        commands: configc::ConfigCommands,
    },

    /// OpenWebUi functions
    OWUI {
        #[command(subcommand)]
        commands: openwebui::OpenWebUiCommands,
    },

    /// Pattern interactions
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
        Commands::Agentic {
            teams_directory,
            commands,
        } => agenticc::execute(teams_directory.clone(), commands),
        Commands::Config { commands } => configc::execute(commands),
        Commands::OWUI { commands } => openwebui::execute(commands),
        Commands::Pattern {
            pattern_directory,
            commands,
        } => pattern::execute(pattern_directory.clone(), commands),
    }
}
