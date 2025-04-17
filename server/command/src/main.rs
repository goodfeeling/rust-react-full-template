mod launcher;
mod config;

use clap::{Parser, Subcommand};
use config::conf;
use tokio::runtime::Runtime;
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// run the project
    Run { config: Option<String> },
    // migrate the project
    Migrate { config: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Run { config } => {
            _ = config;
            // Create a Tokio runtime
            let rt = Runtime::new().unwrap();
            rt.block_on(launcher::run())
        },
        Commands::Migrate { config } => {
            _ = config;
        }
    }
}
