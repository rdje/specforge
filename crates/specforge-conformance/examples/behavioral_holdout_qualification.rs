use std::env;
use std::path::PathBuf;

use specforge_conformance::behavioral_genericity::{
    HeldOutQualificationRequest, qualify_held_out_population,
};
use specforge_conformance::error::{AppError, Result};

fn main() -> Result<()> {
    let mut arguments = env::args().skip(1);
    let output_root = arguments.next().ok_or_else(|| {
        AppError::InvalidStageArtifact(
            "usage: behavioral_holdout_qualification <repo-relative-output-root> <production-revision> [transform-seed]".to_string(),
        )
    })?;
    let production_revision = arguments.next().ok_or_else(|| {
        AppError::InvalidStageArtifact(
            "behavioral held-out qualification requires a full production revision".to_string(),
        )
    })?;
    let transform_seed = arguments
        .next()
        .map(|value| {
            value.parse::<u64>().map_err(|_| {
                AppError::InvalidStageArtifact(
                    "behavioral held-out transform seed must be an unsigned integer".to_string(),
                )
            })
        })
        .transpose()?
        .unwrap_or(0x6d11_f111_u64);
    if arguments.next().is_some() {
        return Err(AppError::InvalidStageArtifact(
            "behavioral held-out qualification received unexpected arguments".to_string(),
        ));
    }
    let report = qualify_held_out_population(&HeldOutQualificationRequest {
        output_root: PathBuf::from(output_root),
        prior_memory: PathBuf::from("generated/prior_memory/corpus_memory.json"),
        production_revision,
        transform_seed,
    })?;
    println!(
        "behavioral holdout: {} documents / {} attempts; pass={} fail={} unmeasurable={} invalid={}",
        report.coverage.declared_documents.len(),
        report.coverage.declared_attempts,
        report.coverage.pass_attempts,
        report.coverage.fail_attempts,
        report.coverage.unmeasurable_attempts,
        report.coverage.invalid_attempts,
    );
    Ok(())
}
