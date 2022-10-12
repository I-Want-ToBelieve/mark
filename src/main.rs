mod commands;
mod config;
mod schema;

use clap::{Parser, Subcommand};
use commands::{apply, backup};
use confy::ConfyError;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Backup {},
    /// ccc
    Apply {},
}

fn main() -> Result<(), ConfyError> {
    let cli = Cli::parse();
    let cfg = config::get()?;

    match &cli.command {
        Some(Commands::Backup {}) => {
            backup::backup(&cfg).expect("can't exec backup");
        }
        Some(Commands::Apply {}) => {
            apply::apply(&cfg).expect("can't exec apply");
        }
        None => {}
    }

    Ok(())
}
