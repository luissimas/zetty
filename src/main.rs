use std::io;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use zetty::{config::Config, create_note};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new note
    New {
        /// The name of the new note
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    human_panic::setup_panic!();
    let cli = Cli::parse();
    let log_level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::WARN
    };
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(log_level)
        .init();

    let config = Config::from_system().context("Could not load configuration from system")?;

    match cli.command {
        Commands::New { name } => {
            let name = name.unwrap_or_else(|| {
                inquire::Text::new("Note name:")
                    .prompt()
                    .expect("Could not get user input")
            });
            create_note(&config, &name)?;
        }
    }

    Ok(())
}

#[test]
fn test_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
