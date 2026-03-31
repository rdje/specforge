pub mod cli;
mod commands;
pub mod error;
pub mod source;

use cli::{Cli, Commands};
use error::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Inspect(args) => commands::inspect::run(args),
        Commands::Ingest(args) => commands::ingest::run(args),
    }
}
