pub mod cli;
mod commands;
pub mod error;
pub mod ir;

use cli::{Cli, Commands};
use error::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Inspect(args) => commands::inspect::run(args),
        Commands::Ingest(args) => commands::ingest::run(args),
        Commands::Evidence(args) => commands::evidence::run(args),
        Commands::Semantic(args) => commands::semantic::run(args),
        Commands::Intent(args) => commands::intent::run(args),
        Commands::Adapt(args) => commands::adapt::run(args),
        Commands::Enrich(args) => commands::enrich::run(args),
        Commands::Validate(args) => commands::validate::run(args),
        Commands::NlpEnrich(args) => commands::nlp_enrich::run(args),
    }
}
