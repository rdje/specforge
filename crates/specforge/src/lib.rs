pub mod cli;
mod commands;
pub use specforge_conformance::eval;
pub use specforge_core::{error, llm_text, persisted_path, project_data, provider};

/// Compatibility facade over the one-way core and downstream conformance crates.
pub mod ir {
    pub use specforge_conformance::ir::{
        completeness, source_to_intent_eval, source_to_intent_replay, trajectory,
    };
    pub use specforge_core::ir::*;
}

#[cfg(test)]
pub(crate) mod test_support;

use cli::{Cli, Commands};
use error::Result;

pub fn run(cli: Cli) -> Result<()> {
    project_data::prepare()?;
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
        Commands::CorpusCluster(args) => commands::corpus_cluster::run(args),
        Commands::KgBench(args) => commands::kg_bench::run(args),
        Commands::LearnPriors(args) => commands::learn_priors::run(args),
        Commands::CorpusKb(args) => commands::corpus_kb::run(args),
        Commands::Clean(args) => commands::clean::run(args),
        Commands::NlpEnrich(args) => commands::nlp_enrich::run(args),
        Commands::ExtractContracts(args) => commands::extract_contracts::run(args),
        Commands::SignalResolve(args) => commands::signal_resolve::run(args),
        Commands::EvalExtraction(args) => commands::eval_extraction::run(args),
        Commands::NliVerify(args) => commands::nli_verify::run(args),
        Commands::GritsConsensus(args) => commands::grits_consensus::run(args),
        Commands::EntityType(args) => commands::entity_type::run(args),
        Commands::ExtractConditions(args) => commands::extract_conditions::run(args),
        Commands::ExtractConstraintsLlm(args) => commands::extract_constraints_llm::run(args),
        Commands::AuditExtraction(args) => commands::audit_extraction::run(args),
        Commands::RecoverRegisterBits(args) => commands::recover_register_bits::run(args),
    }
}
