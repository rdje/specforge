pub mod cli;
mod commands;
pub mod error;
pub mod ir;
#[cfg(test)]
pub(crate) mod test_support;

use cli::{Cli, Commands};
use error::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Inspect(args) => commands::inspect::run(args),
        Commands::Doctor(args) => commands::doctor::run(args),
        Commands::Converge(args) => commands::converge::run(args),
        Commands::Ingest(args) => commands::ingest::run(args),
        Commands::Evidence(args) => commands::evidence::run(args),
        Commands::Semantic(args) => commands::semantic::run(args),
        Commands::Intent(args) => commands::intent::run(args),
        Commands::Adapt(args) => commands::adapt::run(args),
        Commands::Enrich(args) => commands::enrich::run(args),
        Commands::Validate(args) => commands::validate::run(args),
        Commands::ProjectValidation(args) => commands::project_validation::run(args),
        Commands::RescanPlan(args) => commands::rescan_plan::run(args),
        Commands::KgBench(args) => commands::kg_bench::run(args),
        Commands::LearnPriors(args) => commands::learn_priors::run(args),
        Commands::CorpusKb(args) => commands::corpus_kb::run(args),
        Commands::Clean(args) => commands::clean::run(args),
        Commands::NlpEnrich(args) => commands::nlp_enrich::run(args),
        Commands::ExtractContracts(args) => commands::extract_contracts::run(args),
    }
}
