use std::path::PathBuf;

use serde::Serialize;

use crate::cli::{ConvergeArgs, EnrichArgs, NlpEnrichArgs, RescanPlanArgs, VlmProviderArg};
use crate::commands::{enrich, nlp_enrich, rescan_plan};
use crate::error::{AppError, Result};
use crate::ir::adapters::{AdapterArtifact, AdapterLoweringStatus, AdapterTarget};
use crate::ir::evidence::{
    EvidenceIr, InterfaceEdgeTimingRecord, ProtocolOperationRecord, ProtocolStateRecord,
    SerialFrameField, StatementClass, VisualObservationKind,
};
use crate::ir::intent::IntentIr;
use crate::ir::semantic::SemanticIr;
use crate::ir::source::{DiagramKind, SourceIr};
use crate::persisted_path::resolve_repository_output;

#[derive(Debug, Clone)]
struct PipelineArtifactRoots {
    source: PathBuf,
    evidence: PathBuf,
    semantic: PathBuf,
    intent: PathBuf,
    adapters: PathBuf,
}

impl PipelineArtifactRoots {
    fn repository_defaults() -> Result<Self> {
        Ok(Self::below(
            &crate::project_data::repository_root()?.join("generated"),
        ))
    }

    fn below(generated_root: &std::path::Path) -> Self {
        Self {
            source: generated_root.join("source_ir"),
            evidence: generated_root.join("evidence_ir"),
            semantic: generated_root.join("semantic_ir"),
            intent: generated_root.join("intent_ir"),
            adapters: generated_root.join("adapters"),
        }
    }
}

pub fn run(args: ConvergeArgs) -> Result<()> {
    let report = run_convergence(args)?;

    println!("--- convergence summary ---");
    println!("converged: {}", report.converged);
    println!("passes_run: {}", report.passes_run);
    println!("document_key: {}", report.paths.document_key);
    println!(
        "knowledge_fact_count: {}",
        report.final_snapshot.total_fact_count
    );
    println!("source_ir_path: {}", report.paths.source_ir_path.display());
    println!(
        "evidence_ir_path: {}",
        report.paths.evidence_ir_path.display()
    );
    println!(
        "semantic_ir_path: {}",
        report.paths.semantic_ir_path.display()
    );
    println!("intent_ir_path: {}", report.paths.intent_ir_path.display());
    println!(
        "adapter_artifact_path: {}",
        report.paths.adapter_artifact_path.display()
    );
    if let Some(rescan_plan) = report.rescan_plan.as_ref() {
        println!("rescan_plan_path: {}", rescan_plan.plan_path.display());
        println!(
            "rescan_plan_mode: {}",
            if rescan_plan.executed {
                "execute"
            } else {
                "dry-run"
            }
        );
        println!(
            "rescan_plan_selected_recommendations: {}",
            rescan_plan.selected_recommendations
        );
        println!(
            "rescan_plan_validated_changed: {}",
            rescan_plan.executed_validated_changed
        );
        println!(
            "rescan_plan_validated_no_change: {}",
            rescan_plan.executed_validated_no_change
        );
        println!(
            "rescan_plan_snapshot_changed: {}",
            rescan_plan.snapshot_changed
        );
        println!(
            "rescan_plan_review_required: {}",
            rescan_plan.review_required
        );
        println!(
            "rescan_plan_possible_improvement_review_required: {}",
            rescan_plan.possible_improvement_review_required
        );
        println!(
            "rescan_plan_regression_review_required: {}",
            rescan_plan.regression_review_required
        );
        println!(
            "rescan_plan_neutral_change_review_required: {}",
            rescan_plan.neutral_change_review_required
        );
        println!(
            "rescan_plan_arbitration_status: {}",
            rescan_plan.arbitration_status()
        );
    }
    if let Some(promotion) = report.promotion.as_ref() {
        println!(
            "constraint_promotion: {} (Pattern) → {} kept (LLM-primary; field constraints {}; downstream rebuilt)",
            promotion.pattern_before, promotion.kept, promotion.field_kept
        );
    }
    if let Some(gauge) = report.extraction_quality.as_ref() {
        println!(
            "extraction_quality_gauge: {}",
            crate::commands::nli_verify::gauge_summary_line(gauge)
        );
    }
    println!("--- production capability accounting ---");
    println!(
        "production_capability_count: {}",
        report.production_capabilities.len()
    );
    for capability in &report.production_capabilities {
        println!(
            "production_capability: {}",
            serde_json::to_string(capability)?
        );
    }
    println!(
        "next_step_hint: run `specforge validate {}` for a stage-aware report",
        report.paths.intent_ir_path.display()
    );

    Ok(())
}

fn run_convergence(args: ConvergeArgs) -> Result<ConvergenceReport> {
    let roots = PipelineArtifactRoots::repository_defaults()?;
    run_convergence_with_roots(args, &roots)
}

fn run_convergence_with_roots(
    args: ConvergeArgs,
    roots: &PipelineArtifactRoots,
) -> Result<ConvergenceReport> {
    if args.execute_rescan_plan && args.rescan_plan.is_none() {
        return Err(AppError::InvalidStageArtifact(
            "--execute-rescan-plan requires --rescan-plan <path>".to_string(),
        ));
    }

    // An explicit promotion opt-in that could silently do nothing is worse than an error:
    // the LLM-primary extractor needs the text provider, so fail before the long run starts.
    if args.promote_constraints_llm && matches!(args.nlp_provider, VlmProviderArg::Skip) {
        return Err(AppError::InvalidStageArtifact(
            "--promote-constraints-llm requires a live --nlp-provider (not skip)".to_string(),
        ));
    }

    if args.max_iterations == 0 {
        return Err(AppError::InvalidStageArtifact(
            "converge requires --max-iterations >= 1".to_string(),
        ));
    }

    let target: AdapterTarget = args.target.into();
    let mut source_ir = SourceIr::build(&args.source, &roots.source)?;
    source_ir.materialize()?;
    source_ir.write_to_disk()?;
    let paths = PipelineArtifactPaths::from_source_ir(&source_ir, target, roots)?;

    println!("command: converge");
    println!("mode: execute");
    println!("source: {}", args.source.display());
    println!("document_key: {}", paths.document_key);
    println!("target: {}", target.as_str());
    println!("max_iterations: {}", args.max_iterations);
    println!("vlm_provider: {}", provider_name(args.vlm_provider));
    println!("nlp_provider: {}", provider_name(args.nlp_provider));
    println!("prior_memory: {}", args.prior_memory.display());
    println!(
        "rescan_plan: {}",
        args.rescan_plan
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "disabled".to_string())
    );
    println!("execute_rescan_plan: {}", args.execute_rescan_plan);
    println!("rescan_plan_limit: {}", args.rescan_plan_limit);

    let mut previous_snapshot: Option<KnowledgeSnapshot> = None;

    for pass in 1..=args.max_iterations {
        println!("--- pipeline pass {pass} ---");

        if !matches!(args.vlm_provider, VlmProviderArg::Skip) {
            enrich::run(EnrichArgs {
                source_ir: paths.source_ir_path.clone(),
                vlm_provider: args.vlm_provider,
                vlm_model: args.vlm_model.clone(),
                classify_only: false,
                dry_run: false,
            })?;
        }

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &paths.source_ir_path,
            &roots.evidence,
            Some(args.prior_memory.as_path()),
        )?;
        evidence_ir.write_to_disk()?;
        println!(
            "evidence_fact_count: {}",
            evidence_ir.extracted_statements.len()
                + evidence_ir.signal_constraints.len()
                + evidence_ir.conditional_rules.len()
                + evidence_ir.actor_signal_relations.len()
                + evidence_ir.serial_frame_fields.len()
                + evidence_ir.protocol_operations.len()
                + evidence_ir.protocol_states.len()
                + evidence_ir.interface_edge_timings.len()
        );

        if !matches!(args.nlp_provider, VlmProviderArg::Skip) {
            nlp_enrich::run(NlpEnrichArgs {
                evidence_ir: paths.evidence_ir_path.clone(),
                vlm_provider: args.nlp_provider,
                vlm_model: args.nlp_model.clone(),
                dry_run: false,
                max_sentences: args.nlp_max_sentences,
                grounding_signals: None,
            })?;
        }

        let semantic_ir = SemanticIr::build(&paths.evidence_ir_path, &roots.semantic)?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(&paths.semantic_ir_path, &roots.intent)?;
        intent_ir.write_to_disk()?;

        let adapter_artifact =
            AdapterArtifact::build(&paths.intent_ir_path, target, &roots.adapters)?;
        adapter_artifact.write_to_disk()?;

        let snapshot = KnowledgeSnapshot::collect(&paths)?;
        println!("knowledge_fact_count: {}", snapshot.total_fact_count);
        println!(
            "adapter_lowering_status: {}",
            render_lowering_status(snapshot.adapter.lowering_status)
        );

        if let Some(previous) = previous_snapshot.as_ref() {
            if snapshot.total_fact_count < previous.total_fact_count {
                return Err(AppError::InvalidStageArtifact(format!(
                    "pipeline knowledge shrank from {} to {} on pass {}",
                    previous.total_fact_count, snapshot.total_fact_count, pass
                )));
            }

            if snapshot == *previous {
                println!("convergence: stable after pass {pass}");
                let rescan_plan = maybe_run_rescan_plan(&args, &paths, &snapshot)?;
                // Promotion (when opted in) runs BEFORE the gauge, so the standing quality
                // measurement describes the surface the artifacts actually carry.
                let promotion = maybe_promote_constraints(&args, &paths, roots)?;
                let extraction_quality = measure_extraction_quality(&args, &paths)?;
                let production_capabilities = production_capability_report(
                    &args,
                    rescan_plan.as_ref(),
                    promotion.is_some(),
                    extraction_quality.is_some(),
                );
                return Ok(ConvergenceReport {
                    converged: true,
                    passes_run: pass,
                    final_snapshot: snapshot,
                    paths,
                    rescan_plan,
                    promotion,
                    extraction_quality,
                    production_capabilities,
                });
            }
        }

        previous_snapshot = Some(snapshot);
    }

    Err(AppError::InvalidStageArtifact(format!(
        "pipeline did not converge within {} pass(es)",
        args.max_iterations
    )))
}

fn maybe_run_rescan_plan(
    args: &ConvergeArgs,
    paths: &PipelineArtifactPaths,
    stable_snapshot: &KnowledgeSnapshot,
) -> Result<Option<ConvergenceRescanPlanReport>> {
    let Some(plan_path) = args.rescan_plan.clone() else {
        return Ok(None);
    };

    println!("--- convergence rescan plan ---");
    let plan_report = rescan_plan::run_plan(RescanPlanArgs {
        plan: plan_path,
        execute: args.execute_rescan_plan,
        limit: args.rescan_plan_limit,
        document_key: Some(paths.document_key.clone()),
        prior_memory: args.prior_memory.clone(),
    })?;
    let post_rescan_snapshot = KnowledgeSnapshot::collect(paths)?;
    let snapshot_changed = post_rescan_snapshot != *stable_snapshot;
    let review_required = plan_report.review_required_count();
    let possible_improvement_review_required = plan_report
        .arbitration_verdict_count(rescan_plan::ARBITRATION_POSSIBLE_IMPROVEMENT_REVIEW_REQUIRED);
    let regression_review_required =
        plan_report.arbitration_verdict_count(rescan_plan::ARBITRATION_REGRESSION_REVIEW_REQUIRED);
    let neutral_change_review_required = plan_report
        .arbitration_verdict_count(rescan_plan::ARBITRATION_NEUTRAL_CHANGE_REVIEW_REQUIRED);
    println!("rescan_plan_snapshot_changed: {snapshot_changed}");
    println!(
        "rescan_plan_arbitration_status: {}",
        ConvergenceRescanPlanReport::arbitration_status_for(
            plan_report.execute,
            plan_report.executed_validated_changed,
            snapshot_changed,
            review_required,
        )
    );

    Ok(Some(ConvergenceRescanPlanReport {
        plan_path: plan_report.plan_path,
        executed: plan_report.execute,
        selected_recommendations: plan_report.selected_recommendations,
        executed_validated_changed: plan_report.executed_validated_changed,
        executed_validated_no_change: plan_report.executed_validated_no_change,
        snapshot_changed,
        review_required,
        possible_improvement_review_required,
        regression_review_required,
        neutral_change_review_required,
    }))
}

/// LLM-PRIMARY-PROMOTION.5 — promotion is now the DEFAULT for live-NLP converge runs. It runs
/// when a live `--nlp-provider` is used UNLESS the operator opts out with
/// `--no-promote-constraints-llm`. Provider-free runs (`--nlp-provider skip`) stay on the
/// deterministic Pattern surface by construction (so CI/kg-bench/provider-free converge are
/// untouched), and the retained `--promote-constraints-llm` flag is now redundant-but-accepted.
fn should_promote_constraints(args: &ConvergeArgs) -> bool {
    !matches!(args.nlp_provider, VlmProviderArg::Skip) && !args.no_promote_constraints_llm
}

/// LLM-PRIMARY-PROMOTION.2 — the post-stability constraint promotion: replace the final
/// EvidenceIR's Pattern `signal_constraints` with the LLM-primary grounded surface
/// ([`crate::commands::extract_constraints_llm::promote_constraints`] — typed subjects, grounded
/// conditions, condition-only-subject + permissive-frame gates, provenance-merging dedup,
/// manifest-recorded as `constraints.llm_primary`), then rebuild the downstream stages ONCE so
/// `SemanticIR`/`IntentIR`/the adapter carry the promoted surface. Runs OUTSIDE the convergence
/// loop by design: the loop's monotone fact-count guard forbids an in-loop shrink, and replacing
/// 102 Pattern records with ~50 clean ones IS a shrink (`LLM-PRIMARY-PROMOTION.1`).
fn maybe_promote_constraints(
    args: &ConvergeArgs,
    paths: &PipelineArtifactPaths,
    roots: &PipelineArtifactRoots,
) -> Result<Option<crate::commands::extract_constraints_llm::ConstraintPromotionReport>> {
    if !should_promote_constraints(args) {
        return Ok(None);
    }
    // `should_promote_constraints` already excludes `--nlp-provider skip`; defensive here so the
    // helper stays safe if called standalone.
    if matches!(args.nlp_provider, VlmProviderArg::Skip) {
        return Ok(None);
    }
    let model = args
        .nlp_model
        .clone()
        .unwrap_or_else(|| crate::ir::constraint_extract_llm::DEFAULT_EXTRACT_MODEL.to_string());
    println!("--- constraint promotion (LLM-primary, post-stability) ---");
    let report = crate::commands::extract_constraints_llm::promote_constraints(
        &paths.evidence_ir_path,
        args.nlp_provider,
        &model,
        args.nlp_max_sentences,
    )?;
    println!(
        "promotion: constraints {} (Pattern) → {} grounded → {} kept ({} sentence(s); field constraints {} → {})",
        report.pattern_before,
        report.grounded,
        report.kept,
        report.sentences,
        report.field_grounded,
        report.field_kept
    );
    // One downstream rebuild so the canonical stages carry the promoted surface.
    let semantic_ir = SemanticIr::build(&paths.evidence_ir_path, &roots.semantic)?;
    semantic_ir.write_to_disk()?;
    let intent_ir = IntentIr::build(&paths.semantic_ir_path, &roots.intent)?;
    intent_ir.write_to_disk()?;
    let adapter_artifact =
        AdapterArtifact::build(&paths.intent_ir_path, args.target.into(), &roots.adapters)?;
    adapter_artifact.write_to_disk()?;
    println!("promotion: downstream stages rebuilt (semantic → intent → adapter)");
    Ok(Some(report))
}

/// EXTRACTION-QUALITY-GAUGE.0 — the standing per-document quality measurement: after the loop
/// stabilizes (and after any rescan-plan step, so the gauge describes the FINAL artifact), run one
/// NLI pass over the persisted EvidenceIR's signal constraints and back-annotate the
/// extraction-quality gauge. Shares the exact implementation with `nli-verify`
/// ([`crate::commands::nli_verify::measure_and_persist_gauge`]). Skipped (honest `None`) when
/// `--nlp-provider skip` — the gauge needs the text LLM, and CI never depends on a live provider.
fn measure_extraction_quality(
    args: &ConvergeArgs,
    paths: &PipelineArtifactPaths,
) -> Result<Option<crate::ir::evidence::ExtractionQualityGaugeRecord>> {
    if matches!(args.nlp_provider, VlmProviderArg::Skip) {
        return Ok(None);
    }
    let model = args
        .nlp_model
        .clone()
        .unwrap_or_else(|| crate::ir::nli_verify::DEFAULT_NLI_MODEL.to_string());
    let outcome = crate::commands::nli_verify::measure_and_persist_gauge(
        &paths.evidence_ir_path,
        args.nlp_provider,
        &model,
    )?;
    println!(
        "extraction_quality_gauge: {}",
        crate::commands::nli_verify::gauge_summary_line(&outcome.record)
    );
    // A pass that labeled nothing is not a measurement (provider unreachable mid-run) —
    // report the absence instead of pretending a vacuous gauge was recorded.
    if !outcome.persisted {
        println!("extraction_quality_gauge_persisted: false (nothing labeled)");
        return Ok(None);
    }
    Ok(Some(outcome.record))
}

fn provider_name(provider: VlmProviderArg) -> &'static str {
    match provider {
        VlmProviderArg::Ollama => "ollama",
        VlmProviderArg::OpenAi => "openai",
        VlmProviderArg::LmStudio => "lmstudio",
        VlmProviderArg::Skip => "skip",
    }
}

fn render_lowering_status(status: AdapterLoweringStatus) -> &'static str {
    match status {
        AdapterLoweringStatus::Renderable => "renderable",
        AdapterLoweringStatus::Blocked => "blocked",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CapabilityParticipation {
    Integrated,
    Scheduled,
    Omitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CapabilityRunState {
    Executed,
    InspectedOnly,
    NotExecuted,
}

/// One stable, machine-readable row in the canonical production-capability ledger.
///
/// `command` is always the exact clap subcommand name. Multiple capabilities may share a command
/// when one entrypoint has materially different modes, such as ordinary IntentIR construction and
/// `intent --nli-verify` demotion. `entrypoint` names the precise operator-facing form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ProductionCapabilityRecord {
    capability_id: &'static str,
    command: &'static str,
    entrypoint: &'static str,
    participation: CapabilityParticipation,
    run_state: CapabilityRunState,
    reason: &'static str,
}

impl ProductionCapabilityRecord {
    fn new(
        capability_id: &'static str,
        command: &'static str,
        entrypoint: &'static str,
        participation: CapabilityParticipation,
        run_state: CapabilityRunState,
        reason: &'static str,
    ) -> Self {
        Self {
            capability_id,
            command,
            entrypoint,
            participation,
            run_state,
            reason,
        }
    }
}

/// Classify every production capability that can affect the canonical source-to-adapter result.
///
/// This ledger is deliberately per-run: an integrated provider-backed stage reports
/// `not_executed` when the operator disables its provider, while a capability island reports
/// `omitted` regardless of provider availability. The tests below partition the complete clap
/// subcommand surface and require every command classified as production to appear here.
fn production_capability_report(
    args: &ConvergeArgs,
    rescan_plan: Option<&ConvergenceRescanPlanReport>,
    promotion_executed: bool,
    extraction_quality_measured: bool,
) -> Vec<ProductionCapabilityRecord> {
    use CapabilityParticipation::{Integrated, Omitted, Scheduled};
    use CapabilityRunState::{Executed, InspectedOnly, NotExecuted};

    let vlm_live = !matches!(args.vlm_provider, VlmProviderArg::Skip);
    let nlp_live = !matches!(args.nlp_provider, VlmProviderArg::Skip);
    let rescan_state = match rescan_plan {
        Some(report) if report.executed => Executed,
        Some(_) => InspectedOnly,
        None => NotExecuted,
    };

    vec![
        ProductionCapabilityRecord::new(
            "source_ingest",
            "ingest",
            "ingest",
            Integrated,
            Executed,
            "converge builds, materializes, and persists SourceIR before iteration",
        ),
        ProductionCapabilityRecord::new(
            "visual_enrichment",
            "enrich",
            "enrich",
            Integrated,
            if vlm_live { Executed } else { NotExecuted },
            if vlm_live {
                "the configured VLM enrichment pass ran inside each convergence iteration"
            } else {
                "omitted for this run because --vlm-provider skip was selected"
            },
        ),
        ProductionCapabilityRecord::new(
            "deterministic_evidence_extraction",
            "evidence",
            "evidence",
            Integrated,
            Executed,
            "converge rebuilt and persisted EvidenceIR inside each iteration",
        ),
        ProductionCapabilityRecord::new(
            "nlp_constraint_enrichment",
            "nlp-enrich",
            "nlp-enrich",
            Integrated,
            if nlp_live { Executed } else { NotExecuted },
            if nlp_live {
                "the configured NLP enrichment pass ran inside each convergence iteration"
            } else {
                "omitted for this run because --nlp-provider skip was selected"
            },
        ),
        ProductionCapabilityRecord::new(
            "constrained_contract_extraction",
            "extract-contracts",
            "extract-contracts",
            Omitted,
            NotExecuted,
            "standalone extractor is not composed by converge; run it explicitly and rebuild downstream stages",
        ),
        ProductionCapabilityRecord::new(
            "actor_signal_relation_resolution",
            "signal-resolve",
            "signal-resolve",
            Omitted,
            NotExecuted,
            "standalone Tier-3 resolver is not composed by converge; run it explicitly and rebuild downstream stages",
        ),
        ProductionCapabilityRecord::new(
            "register_bit_recovery",
            "recover-register-bits",
            "recover-register-bits",
            Omitted,
            NotExecuted,
            "diagram-backed recovery is not composed by converge; run it explicitly when candidate registers and a VLM are available",
        ),
        ProductionCapabilityRecord::new(
            "condition_extraction",
            "extract-conditions",
            "extract-conditions",
            if promotion_executed {
                Integrated
            } else {
                Omitted
            },
            if promotion_executed {
                Executed
            } else {
                NotExecuted
            },
            if promotion_executed {
                "condition grounding was covered by the integrated LLM-primary constraint replacement"
            } else {
                "standalone condition repair did not run and LLM-primary replacement was not executed"
            },
        ),
        ProductionCapabilityRecord::new(
            "llm_primary_constraint_extraction",
            "extract-constraints-llm",
            "extract-constraints-llm",
            Integrated,
            if promotion_executed {
                Executed
            } else {
                NotExecuted
            },
            if promotion_executed {
                "the post-stability grounded replacement ran and downstream stages were rebuilt"
            } else if nlp_live {
                "omitted for this run because --no-promote-constraints-llm was selected"
            } else {
                "omitted for this run because --nlp-provider skip was selected"
            },
        ),
        ProductionCapabilityRecord::new(
            "semantic_projection",
            "semantic",
            "semantic",
            Integrated,
            Executed,
            "converge rebuilt and persisted SemanticIR",
        ),
        ProductionCapabilityRecord::new(
            "intent_projection",
            "intent",
            "intent",
            Integrated,
            Executed,
            "converge rebuilt and persisted canonical IntentIR",
        ),
        ProductionCapabilityRecord::new(
            "intent_nli_enforcement",
            "intent",
            "intent --nli-verify",
            Omitted,
            NotExecuted,
            "converge measures EvidenceIR extraction quality but does not invoke the separate IntentIR NLI demotion gate",
        ),
        ProductionCapabilityRecord::new(
            "adapter_lowering",
            "adapt",
            "adapt",
            Integrated,
            Executed,
            "converge rebuilt and persisted the selected adapter artifact",
        ),
        ProductionCapabilityRecord::new(
            "evidence_nli_quality_measurement",
            "nli-verify",
            "nli-verify",
            Integrated,
            if extraction_quality_measured {
                Executed
            } else {
                NotExecuted
            },
            if extraction_quality_measured {
                "the post-stability EvidenceIR quality gauge was measured and persisted"
            } else if nlp_live {
                "the configured measurement produced no labeled claims, so no gauge was persisted"
            } else {
                "omitted for this run because --nlp-provider skip was selected"
            },
        ),
        ProductionCapabilityRecord::new(
            "validation_rescan",
            "rescan-plan",
            "rescan-plan",
            Integrated,
            rescan_state,
            match rescan_state {
                Executed => "the supplied validation rescan plan executed for this document",
                InspectedOnly => {
                    "the supplied validation rescan plan was inspected without executing recommendations"
                }
                NotExecuted => "no --rescan-plan was supplied for this run",
            },
        ),
        ProductionCapabilityRecord::new(
            "artifact_validation",
            "validate",
            "validate",
            Scheduled,
            NotExecuted,
            "scheduled as the explicit next-step hint after converge; validation is not silently claimed for this run",
        ),
        ProductionCapabilityRecord::new(
            "prior_memory_learning",
            "learn-priors",
            "learn-priors",
            Scheduled,
            NotExecuted,
            "cross-document prior learning remains a separate reviewed-corpus operation; converge only consumes the configured store",
        ),
    ]
}

#[derive(Debug, Clone)]
struct PipelineArtifactPaths {
    document_key: String,
    source_ir_path: PathBuf,
    evidence_ir_path: PathBuf,
    semantic_ir_path: PathBuf,
    intent_ir_path: PathBuf,
    adapter_artifact_path: PathBuf,
}

impl PipelineArtifactPaths {
    fn from_source_ir(
        source_ir: &SourceIr,
        target: AdapterTarget,
        roots: &PipelineArtifactRoots,
    ) -> Result<Self> {
        let document_key = source_ir.document_identity.document_key.clone();
        Self {
            source_ir_path: absolute_artifact_path(&source_ir.artifact_layout.source_ir_path)?,
            evidence_ir_path: roots.evidence.join(&document_key).join("evidence_ir.json"),
            semantic_ir_path: roots.semantic.join(&document_key).join("semantic_ir.json"),
            intent_ir_path: roots.intent.join(&document_key).join("intent_ir.json"),
            adapter_artifact_path: roots
                .adapters
                .join(target.as_str())
                .join(&document_key)
                .join("adapter.json"),
            document_key,
        }
        .absolutize()
    }
}

impl PipelineArtifactPaths {
    fn absolutize(self) -> Result<Self> {
        Ok(Self {
            document_key: self.document_key,
            source_ir_path: absolute_artifact_path(&self.source_ir_path)?,
            evidence_ir_path: absolute_artifact_path(&self.evidence_ir_path)?,
            semantic_ir_path: absolute_artifact_path(&self.semantic_ir_path)?,
            intent_ir_path: absolute_artifact_path(&self.intent_ir_path)?,
            adapter_artifact_path: absolute_artifact_path(&self.adapter_artifact_path)?,
        })
    }
}

fn absolute_artifact_path(path: &std::path::Path) -> Result<PathBuf> {
    resolve_repository_output(path)
}

#[derive(Debug, Clone)]
struct ConvergenceReport {
    converged: bool,
    passes_run: usize,
    final_snapshot: KnowledgeSnapshot,
    paths: PipelineArtifactPaths,
    rescan_plan: Option<ConvergenceRescanPlanReport>,
    /// LLM-PRIMARY-PROMOTION.2: the opt-in post-stability constraint promotion report. `None`
    /// unless `--promote-constraints-llm` ran.
    promotion: Option<crate::commands::extract_constraints_llm::ConstraintPromotionReport>,
    /// EXTRACTION-QUALITY-GAUGE.0: the post-stability NLI extraction-quality gauge measured over
    /// the FINAL EvidenceIR (after any rescan-plan step and any constraint promotion) and
    /// persisted into the artifact. `None` when `--nlp-provider skip` (the gauge needs the text
    /// LLM) or when the pass labeled nothing.
    extraction_quality: Option<crate::ir::evidence::ExtractionQualityGaugeRecord>,
    /// SPEC-TO-INTENT-ALIGNMENT.2: exact per-run participation for every production command.
    /// Capability islands must remain visible as `omitted`; provider/config skips are reported as
    /// integrated capabilities whose `run_state` is `not_executed`.
    production_capabilities: Vec<ProductionCapabilityRecord>,
}

#[derive(Debug, Clone)]
struct ConvergenceRescanPlanReport {
    plan_path: PathBuf,
    executed: bool,
    selected_recommendations: usize,
    executed_validated_changed: usize,
    executed_validated_no_change: usize,
    snapshot_changed: bool,
    review_required: usize,
    possible_improvement_review_required: usize,
    regression_review_required: usize,
    neutral_change_review_required: usize,
}

impl ConvergenceRescanPlanReport {
    fn arbitration_status(&self) -> &'static str {
        Self::arbitration_status_for(
            self.executed,
            self.executed_validated_changed,
            self.snapshot_changed,
            self.review_required,
        )
    }

    fn arbitration_status_for(
        executed: bool,
        executed_validated_changed: usize,
        snapshot_changed: bool,
        review_required: usize,
    ) -> &'static str {
        if !executed {
            "dry_run_not_promoted"
        } else if review_required > 0 || executed_validated_changed > 0 || snapshot_changed {
            "changed_requires_validation_review"
        } else {
            "executed_validated_no_change"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct KnowledgeSnapshot {
    total_fact_count: usize,
    source: SourceSnapshot,
    evidence: EvidenceSnapshot,
    semantic: SemanticSnapshot,
    intent: IntentSnapshot,
    adapter: AdapterSnapshot,
}

impl KnowledgeSnapshot {
    fn collect(paths: &PipelineArtifactPaths) -> Result<Self> {
        let source = SourceIr::load_from_path(&paths.source_ir_path)?;
        let evidence = EvidenceIr::load_from_path(&paths.evidence_ir_path)?;
        let semantic = SemanticIr::load_from_path(&paths.semantic_ir_path)?;
        let intent = IntentIr::load_from_path(&paths.intent_ir_path)?;
        let adapter = AdapterArtifact::load_from_path(&paths.adapter_artifact_path)?;

        let source_snapshot = SourceSnapshot::from_ir(&source);
        let evidence_snapshot = EvidenceSnapshot::from_ir(&evidence);
        let semantic_snapshot = SemanticSnapshot::from_ir(&semantic);
        let intent_snapshot = IntentSnapshot::from_ir(&intent);
        let adapter_snapshot = AdapterSnapshot::from_artifact(&adapter);
        // Knowledge convergence is about persisted IR facts, not downstream residual work.
        // Adapter residual decisions can legitimately shrink as the IR gets better, so they
        // must not count against the monotone knowledge metric.
        let total_fact_count = source_snapshot.fact_count()
            + evidence_snapshot.fact_count()
            + semantic_snapshot.fact_count()
            + intent_snapshot.fact_count();

        Ok(Self {
            total_fact_count,
            source: source_snapshot,
            evidence: evidence_snapshot,
            semantic: semantic_snapshot,
            intent: intent_snapshot,
            adapter: adapter_snapshot,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SourceSnapshot {
    page_artifacts: usize,
    visual_assets: usize,
    structured_tables: usize,
    content_elements: usize,
    document_sections: usize,
    timing_diagrams: usize,
    state_machine_diagrams: usize,
    vlm_enriched_assets: usize,
}

impl SourceSnapshot {
    fn from_ir(ir: &SourceIr) -> Self {
        let timing_diagrams = ir
            .visual_assets
            .iter()
            .filter(|asset| matches!(asset.diagram_kind, DiagramKind::TimingDiagram))
            .count();
        let state_machine_diagrams = ir
            .visual_assets
            .iter()
            .filter(|asset| matches!(asset.diagram_kind, DiagramKind::StateMachineDiagram))
            .count();
        let vlm_enriched_assets = ir
            .visual_assets
            .iter()
            .filter(|asset| {
                asset.note.as_deref().map(|note| {
                    note.starts_with("vlm_timing_diagram_extraction:")
                        || note.starts_with("vlm_state_machine_extraction:")
                }) == Some(true)
            })
            .count();

        Self {
            page_artifacts: ir.page_artifacts.len(),
            visual_assets: ir.visual_assets.len(),
            structured_tables: ir.structured_tables.len(),
            content_elements: ir.content_elements.len(),
            document_sections: ir.document_sections.len(),
            timing_diagrams,
            state_machine_diagrams,
            vlm_enriched_assets,
        }
    }

    fn fact_count(&self) -> usize {
        self.page_artifacts
            + self.visual_assets
            + self.structured_tables
            + self.content_elements
            + self.document_sections
            + self.timing_diagrams
            + self.state_machine_diagrams
            + self.vlm_enriched_assets
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct EvidenceSnapshot {
    section_anchors: usize,
    evidence_spans: usize,
    visual_evidence: usize,
    extracted_statements: usize,
    signal_value_statements: usize,
    conditional_rule_statements: usize,
    normative_statements: usize,
    signal_constraints: usize,
    conditional_rules: usize,
    actor_signal_relations: usize,
    register_records: usize,
    timing_constraints: usize,
    serial_frame_fields: Vec<SerialFrameField>,
    protocol_operations: Vec<ProtocolOperationRecord>,
    protocol_states: Vec<ProtocolStateRecord>,
    interface_edge_timings: Vec<InterfaceEdgeTimingRecord>,
    alias_map_size: usize,
    timing_diagram_observations: usize,
    state_machine_observations: usize,
}

impl EvidenceSnapshot {
    fn from_ir(ir: &EvidenceIr) -> Self {
        let signal_value_statements = ir
            .extracted_statements
            .iter()
            .filter(|statement| matches!(statement.class, StatementClass::SignalValueConstraint))
            .count();
        let conditional_rule_statements = ir
            .extracted_statements
            .iter()
            .filter(|statement| matches!(statement.class, StatementClass::ConditionalRule))
            .count();
        let normative_statements = ir
            .extracted_statements
            .iter()
            .filter(|statement| matches!(statement.class, StatementClass::NormativeStatement))
            .count();
        let mut timing_diagram_observations = 0usize;
        let mut state_machine_observations = 0usize;
        for item in &ir.visual_evidence {
            for observation in &item.observations {
                match observation.kind {
                    VisualObservationKind::TimingDiagramExtraction => {
                        timing_diagram_observations += 1;
                    }
                    VisualObservationKind::StateMachineExtraction => {
                        state_machine_observations += 1;
                    }
                    _ => {}
                }
            }
        }

        Self {
            section_anchors: ir.section_anchors.len(),
            evidence_spans: ir.evidence_spans.len(),
            visual_evidence: ir.visual_evidence.len(),
            extracted_statements: ir.extracted_statements.len(),
            signal_value_statements,
            conditional_rule_statements,
            normative_statements,
            signal_constraints: ir.signal_constraints.len(),
            conditional_rules: ir.conditional_rules.len(),
            actor_signal_relations: ir.actor_signal_relations.len(),
            register_records: ir.register_records.len(),
            timing_constraints: ir.timing_constraints.len(),
            serial_frame_fields: ir.serial_frame_fields.clone(),
            protocol_operations: ir.protocol_operations.clone(),
            protocol_states: ir.protocol_states.clone(),
            interface_edge_timings: ir.interface_edge_timings.clone(),
            alias_map_size: ir.signal_alias_map.len(),
            timing_diagram_observations,
            state_machine_observations,
        }
    }

    fn fact_count(&self) -> usize {
        self.section_anchors
            + self.evidence_spans
            + self.visual_evidence
            + self.extracted_statements
            + self.signal_value_statements
            + self.conditional_rule_statements
            + self.signal_constraints
            + self.conditional_rules
            + self.actor_signal_relations
            + self.register_records
            + self.timing_constraints
            + self.serial_frame_fields.len()
            + self.protocol_operations.len()
            + self.protocol_states.len()
            + self.interface_edge_timings.len()
            + self.alias_map_size
            + self.timing_diagram_observations
            + self.state_machine_observations
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SemanticSnapshot {
    actors: usize,
    interfaces: usize,
    interface_signals: usize,
    phases: usize,
    invariants: usize,
    contracts: usize,
    gates: usize,
    assertions: usize,
    abstractions: usize,
    decomposition_candidates: usize,
    system_contract_present: bool,
    regular_states: usize,
    state_transitions: usize,
    symbol_definitions: usize,
    control_blocks: usize,
    explicit_modules: usize,
    explicit_tops: usize,
    register_records: usize,
    timing_constraints: usize,
    serial_frame_fields: Vec<SerialFrameField>,
    protocol_operations: Vec<ProtocolOperationRecord>,
    protocol_states: Vec<ProtocolStateRecord>,
    interface_edge_timings: Vec<InterfaceEdgeTimingRecord>,
    signal_constraints: usize,
    conditional_rules: usize,
    residual_decisions: usize,
}

impl SemanticSnapshot {
    fn from_ir(ir: &SemanticIr) -> Self {
        Self {
            actors: ir.actors.len(),
            interfaces: ir.interfaces.len(),
            interface_signals: ir
                .interfaces
                .iter()
                .map(|iface| iface.signal_records.len())
                .sum(),
            phases: ir.phases.len(),
            invariants: ir.invariants.len(),
            contracts: ir.contracts.len(),
            gates: ir.gates.len(),
            assertions: ir.assertions.len(),
            abstractions: ir.abstractions.len(),
            decomposition_candidates: ir.decomposition_candidates.len(),
            system_contract_present: ir.system_contract.is_some(),
            regular_states: ir.regular_states.len(),
            state_transitions: ir.state_transitions.len(),
            symbol_definitions: ir.symbol_definitions.len(),
            control_blocks: ir.control_blocks.len(),
            explicit_modules: ir.explicit_modules.len(),
            explicit_tops: ir.explicit_tops.len(),
            register_records: ir.register_records.len(),
            timing_constraints: ir.timing_constraints.len(),
            serial_frame_fields: ir.serial_frame_fields.clone(),
            protocol_operations: ir.protocol_operations.clone(),
            protocol_states: ir.protocol_states.clone(),
            interface_edge_timings: ir.interface_edge_timings.clone(),
            signal_constraints: ir.signal_constraints.len(),
            conditional_rules: ir.conditional_rules.len(),
            residual_decisions: ir.residual_decisions.len(),
        }
    }

    fn fact_count(&self) -> usize {
        self.actors
            + self.interfaces
            + self.interface_signals
            + self.phases
            + self.invariants
            + self.contracts
            + self.gates
            + self.assertions
            + self.abstractions
            + self.decomposition_candidates
            + usize::from(self.system_contract_present)
            + self.regular_states
            + self.state_transitions
            + self.symbol_definitions
            + self.control_blocks
            + self.explicit_modules
            + self.explicit_tops
            + self.register_records
            + self.timing_constraints
            + self.serial_frame_fields.len()
            + self.protocol_operations.len()
            + self.protocol_states.len()
            + self.interface_edge_timings.len()
            + self.signal_constraints
            + self.conditional_rules
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct IntentSnapshot {
    actors: usize,
    interfaces: usize,
    interface_signals: usize,
    behaviors: usize,
    constraints: usize,
    assumptions: usize,
    system_contract_present: bool,
    regular_states: usize,
    state_transitions: usize,
    symbol_definitions: usize,
    control_blocks: usize,
    explicit_modules: usize,
    explicit_tops: usize,
    register_records: usize,
    timing_constraints: usize,
    serial_frame_fields: Vec<SerialFrameField>,
    protocol_operations: Vec<ProtocolOperationRecord>,
    protocol_states: Vec<ProtocolStateRecord>,
    interface_edge_timings: Vec<InterfaceEdgeTimingRecord>,
    signal_constraints: usize,
    conditional_rules: usize,
    residual_decisions: usize,
}

impl IntentSnapshot {
    fn from_ir(ir: &IntentIr) -> Self {
        Self {
            actors: ir.actors.len(),
            interfaces: ir.interfaces.len(),
            interface_signals: ir
                .interfaces
                .iter()
                .map(|iface| iface.signal_records.len())
                .sum(),
            behaviors: ir.behaviors.len(),
            constraints: ir.constraints.len(),
            assumptions: ir.assumptions.len(),
            system_contract_present: ir.system_contract.is_some(),
            regular_states: ir.regular_states.len(),
            state_transitions: ir.state_transitions.len(),
            symbol_definitions: ir.symbol_definitions.len(),
            control_blocks: ir.control_blocks.len(),
            explicit_modules: ir.explicit_modules.len(),
            explicit_tops: ir.explicit_tops.len(),
            register_records: ir.register_records.len(),
            timing_constraints: ir.timing_constraints.len(),
            serial_frame_fields: ir.serial_frame_fields.clone(),
            protocol_operations: ir.protocol_operations.clone(),
            protocol_states: ir.protocol_states.clone(),
            interface_edge_timings: ir.interface_edge_timings.clone(),
            signal_constraints: ir.signal_constraints.len(),
            conditional_rules: ir.conditional_rules.len(),
            residual_decisions: ir.residual_decisions.len(),
        }
    }

    fn fact_count(&self) -> usize {
        self.actors
            + self.interfaces
            + self.interface_signals
            + self.behaviors
            + self.constraints
            + self.assumptions
            + usize::from(self.system_contract_present)
            + self.regular_states
            + self.state_transitions
            + self.symbol_definitions
            + self.control_blocks
            + self.explicit_modules
            + self.explicit_tops
            + self.register_records
            + self.timing_constraints
            + self.serial_frame_fields.len()
            + self.protocol_operations.len()
            + self.protocol_states.len()
            + self.interface_edge_timings.len()
            + self.signal_constraints
            + self.conditional_rules
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct AdapterSnapshot {
    lowering_status: AdapterLoweringStatus,
    residual_decisions: usize,
    signal_count: usize,
    transaction_count: usize,
    rule_count: usize,
    constant_count: usize,
    enum_count: usize,
    storage_count: usize,
    renderable: bool,
}

impl AdapterSnapshot {
    fn from_artifact(artifact: &AdapterArtifact) -> Self {
        let (
            signal_count,
            transaction_count,
            rule_count,
            constant_count,
            enum_count,
            storage_count,
            renderable,
        ) = if let Some(isf) = artifact.isf.as_ref() {
            (
                isf.signal_count,
                isf.transaction_count,
                isf.rule_count,
                isf.constant_count,
                isf.enum_count,
                isf.storage_count,
                isf.is_renderable,
            )
        } else {
            (0, 0, 0, 0, 0, 0, false)
        };

        Self {
            lowering_status: artifact.lowering_status,
            residual_decisions: artifact.residual_decisions.len(),
            signal_count,
            transaction_count,
            rule_count,
            constant_count,
            enum_count,
            storage_count,
            renderable,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;

    use clap::CommandFactory;
    use tempfile::tempdir;

    use super::*;
    use crate::cli::AdapterTargetArg;
    use crate::commands::project_validation::ProjectRescanPlanRecord;
    use crate::test_support::env_var_lock;

    fn write_mock_helper(dir: &std::path::Path, responses: &[(&str, &str)]) -> PathBuf {
        let script_path = dir.join("mock_converge_helper.sh");
        let mut cases = String::new();
        for (keyword, response) in responses {
            cases.push_str(&format!(
                "if echo \"$SENTENCE\" | grep -q \"{keyword}\"; then\n  echo '{response}'\n  exit 0\nfi\n"
            ));
        }
        let script = format!(
            "#!/bin/bash\nSENTENCE=\"\"\nwhile [ $# -gt 0 ]; do\n  case \"$1\" in\n    --sentence) SENTENCE=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\n{cases}echo '{{\"type\":\"none\"}}'\n"
        );
        fs::write(&script_path, script).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();
        }
        script_path
    }

    #[test]
    fn converge_rebuilds_pipeline_until_snapshot_stabilizes() -> Result<()> {
        let _lock = env_var_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        fs::write(
            &source,
            concat!(
                "# Signals\n",
                "Signal HADDR is input width 32.\n",
                "Signal HREADY is input width 1.\n",
                "\n",
                "# Protocol\n",
                "The address bus shall keep its previous value while HREADY is LOW.\n",
            ),
        )?;

        let helper = write_mock_helper(
            tempdir.path(),
            &[(
                "address bus",
                r#"{"type":"signal_constraint","subject_signal":"HADDR","constraint_kind":"must_be_stable","condition":"while HREADY is LOW","negated":false}"#,
            )],
        );
        unsafe { std::env::set_var("SPECFORGE_VLM_HELPER", &helper) };

        let rescan_plan_path = tempdir.path().join("rescan_plan.json");
        fs::write(
            &rescan_plan_path,
            serde_json::to_string_pretty(&ProjectRescanPlanRecord {
                schema_version: 2,
                generated_by: "test".to_string(),
                recommendation_count: 0,
                recommendations: Vec::new(),
            })?,
        )?;

        let roots = PipelineArtifactRoots::below(&tempdir.path().join("generated"));
        let result = run_convergence_with_roots(
            ConvergeArgs {
                source: source.clone(),
                target: AdapterTargetArg::Isf,
                max_iterations: 4,
                vlm_provider: VlmProviderArg::Skip,
                vlm_model: None,
                nlp_provider: VlmProviderArg::Ollama,
                nlp_model: Some("mock".to_string()),
                nlp_max_sentences: 0,
                promote_constraints_llm: false,
                // This test exercises the rescan-plan path and asserts the Pattern constraint
                // surface; opt out of the now-default LLM-primary promotion so it stays focused.
                no_promote_constraints_llm: true,
                prior_memory: tempdir
                    .path()
                    .join("generated")
                    .join("prior_memory")
                    .join("corpus_memory.json"),
                rescan_plan: Some(rescan_plan_path.clone()),
                execute_rescan_plan: false,
                rescan_plan_limit: 0,
            },
            &roots,
        );

        unsafe { std::env::remove_var("SPECFORGE_VLM_HELPER") };

        let report = result?;
        assert!(report.converged);
        assert_eq!(report.passes_run, 2);
        let rescan_report = report.rescan_plan.as_ref().expect("rescan plan report");
        assert_eq!(rescan_report.plan_path, rescan_plan_path);
        assert!(!rescan_report.executed);
        assert_eq!(rescan_report.selected_recommendations, 0);
        assert!(!rescan_report.snapshot_changed);
        assert_eq!(rescan_report.review_required, 0);
        assert_eq!(rescan_report.possible_improvement_review_required, 0);
        assert_eq!(rescan_report.regression_review_required, 0);
        assert_eq!(rescan_report.neutral_change_review_required, 0);
        assert_eq!(rescan_report.executed_validated_changed, 0);
        assert_eq!(rescan_report.executed_validated_no_change, 0);
        assert_eq!(rescan_report.arbitration_status(), "dry_run_not_promoted");

        let evidence = EvidenceIr::load_from_path(&report.paths.evidence_ir_path)?;
        assert_eq!(
            evidence.signal_alias_map.get("address bus"),
            Some(&"HADDR".to_string())
        );
        assert!(evidence.extracted_statements.iter().any(|statement| {
            statement.text == "The address bus shall keep its previous value while HREADY is LOW."
                && matches!(statement.class, StatementClass::SignalValueConstraint)
        }));
        assert!(evidence.signal_constraints.iter().any(|record| {
            record.subject_signal == "HADDR"
                && matches!(
                    record.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeStable
                )
                && record.condition_text.as_deref() == Some("while HREADY is LOW")
        }));

        let semantic = SemanticIr::load_from_path(&report.paths.semantic_ir_path)?;
        assert_eq!(semantic.signal_constraints.len(), 1);

        Ok(())
    }

    #[test]
    fn converge_rejects_execute_rescan_plan_without_plan() {
        let err = run_convergence(ConvergeArgs {
            source: PathBuf::from("missing.md"),
            target: AdapterTargetArg::Isf,
            max_iterations: 1,
            vlm_provider: VlmProviderArg::Skip,
            vlm_model: None,
            nlp_provider: VlmProviderArg::Skip,
            nlp_model: None,
            nlp_max_sentences: 0,
            promote_constraints_llm: false,
            no_promote_constraints_llm: false,
            prior_memory: PathBuf::from("generated/prior_memory/corpus_memory.json"),
            rescan_plan: None,
            execute_rescan_plan: true,
            rescan_plan_limit: 0,
        })
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("--execute-rescan-plan requires --rescan-plan <path>")
        );
    }

    #[test]
    fn converge_rejects_promotion_without_nlp_provider() {
        // An explicit promotion opt-in must never silently do nothing: the flag with
        // `--nlp-provider skip` errors before any source work starts.
        let err = run_convergence(ConvergeArgs {
            source: PathBuf::from("missing.md"),
            target: AdapterTargetArg::Isf,
            max_iterations: 1,
            vlm_provider: VlmProviderArg::Skip,
            vlm_model: None,
            nlp_provider: VlmProviderArg::Skip,
            nlp_model: None,
            nlp_max_sentences: 0,
            promote_constraints_llm: true,
            no_promote_constraints_llm: false,
            prior_memory: PathBuf::from("generated/prior_memory/corpus_memory.json"),
            rescan_plan: None,
            execute_rescan_plan: false,
            rescan_plan_limit: 0,
        })
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("--promote-constraints-llm requires a live --nlp-provider")
        );
    }

    /// LLM-PRIMARY-PROMOTION.5: a minimal `ConvergeArgs` for exercising the promotion-decision
    /// gate without touching the filesystem or a provider.
    fn promotion_decision_args(
        nlp_provider: VlmProviderArg,
        promote_constraints_llm: bool,
        no_promote_constraints_llm: bool,
    ) -> ConvergeArgs {
        ConvergeArgs {
            source: PathBuf::from("unused.md"),
            target: AdapterTargetArg::Isf,
            max_iterations: 1,
            vlm_provider: VlmProviderArg::Skip,
            vlm_model: None,
            nlp_provider,
            nlp_model: None,
            nlp_max_sentences: 0,
            promote_constraints_llm,
            no_promote_constraints_llm,
            prior_memory: PathBuf::from("generated/prior_memory/corpus_memory.json"),
            rescan_plan: None,
            execute_rescan_plan: false,
            rescan_plan_limit: 0,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum CliSurfaceRole {
        Orchestrator,
        Production,
        QualityOrReview,
        CorpusManagement,
        Diagnostic,
        Maintenance,
    }

    /// Complete clap-surface partition. A new subcommand fails the coverage test below until it is
    /// classified; every `Production` command must additionally appear in the converge ledger.
    const CLI_SURFACE_REGISTRY: &[(&str, CliSurfaceRole)] = &[
        ("inspect", CliSurfaceRole::Diagnostic),
        ("doctor", CliSurfaceRole::Diagnostic),
        ("converge", CliSurfaceRole::Orchestrator),
        ("ingest", CliSurfaceRole::Production),
        ("evidence", CliSurfaceRole::Production),
        ("semantic", CliSurfaceRole::Production),
        ("intent", CliSurfaceRole::Production),
        ("adapt", CliSurfaceRole::Production),
        ("enrich", CliSurfaceRole::Production),
        ("validate", CliSurfaceRole::Production),
        ("project-validation", CliSurfaceRole::QualityOrReview),
        ("rescan-plan", CliSurfaceRole::Production),
        ("corpus-cluster", CliSurfaceRole::CorpusManagement),
        ("kg-bench", CliSurfaceRole::QualityOrReview),
        ("learn-priors", CliSurfaceRole::Production),
        ("corpus-kb", CliSurfaceRole::CorpusManagement),
        ("clean", CliSurfaceRole::Maintenance),
        ("nlp-enrich", CliSurfaceRole::Production),
        ("extract-contracts", CliSurfaceRole::Production),
        ("signal-resolve", CliSurfaceRole::Production),
        ("eval-extraction", CliSurfaceRole::QualityOrReview),
        ("nli-verify", CliSurfaceRole::Production),
        ("grits-consensus", CliSurfaceRole::QualityOrReview),
        ("entity-type", CliSurfaceRole::QualityOrReview),
        ("extract-conditions", CliSurfaceRole::Production),
        ("extract-constraints-llm", CliSurfaceRole::Production),
        ("audit-extraction", CliSurfaceRole::QualityOrReview),
        ("recover-register-bits", CliSurfaceRole::Production),
    ];

    #[test]
    fn production_capability_registry_partitions_cli_and_accounts_for_every_producer() {
        let clap_commands: BTreeSet<String> = crate::cli::Cli::command()
            .get_subcommands()
            .map(|command| command.get_name().to_string())
            .collect();
        let classified_commands: BTreeSet<String> = CLI_SURFACE_REGISTRY
            .iter()
            .map(|(command, _)| (*command).to_string())
            .collect();
        assert_eq!(classified_commands.len(), CLI_SURFACE_REGISTRY.len());
        assert_eq!(classified_commands, clap_commands);

        let report = production_capability_report(
            &promotion_decision_args(VlmProviderArg::Skip, false, false),
            None,
            false,
            false,
        );
        let capability_ids: BTreeSet<&str> =
            report.iter().map(|record| record.capability_id).collect();
        assert_eq!(capability_ids.len(), report.len());
        let reported_commands: BTreeSet<&str> =
            report.iter().map(|record| record.command).collect();
        let production_commands: BTreeSet<&str> = CLI_SURFACE_REGISTRY
            .iter()
            .filter_map(|(command, role)| (*role == CliSurfaceRole::Production).then_some(*command))
            .collect();
        assert_eq!(reported_commands, production_commands);
    }

    #[test]
    fn provider_free_capability_report_names_every_current_capability_island() {
        let report = production_capability_report(
            &promotion_decision_args(VlmProviderArg::Skip, false, false),
            None,
            false,
            false,
        );
        for capability_id in [
            "constrained_contract_extraction",
            "actor_signal_relation_resolution",
            "register_bit_recovery",
            "condition_extraction",
            "intent_nli_enforcement",
        ] {
            let record = report
                .iter()
                .find(|record| record.capability_id == capability_id)
                .unwrap_or_else(|| panic!("missing capability record: {capability_id}"));
            assert_eq!(record.participation, CapabilityParticipation::Omitted);
            assert_eq!(record.run_state, CapabilityRunState::NotExecuted);
            assert!(!record.reason.is_empty());
        }
    }

    #[test]
    fn provider_free_capability_observation_is_current() -> Result<()> {
        let report = production_capability_report(
            &promotion_decision_args(VlmProviderArg::Skip, false, false),
            None,
            false,
            false,
        );
        let repository = crate::project_data::repository_root()?;
        let observation: serde_json::Value = serde_json::from_slice(&fs::read(repository.join(
            crate::test_support::trajectory_snapshot::CURRENT_CAPABILITY_OBSERVATION_PATH,
        ))?)?;
        assert_eq!(observation["schema_version"], 1);
        assert_eq!(observation["profile"]["vlm_provider"], "skip");
        assert_eq!(observation["profile"]["nlp_provider"], "skip");
        assert_eq!(
            observation["production_capabilities"],
            serde_json::to_value(report)?
        );
        Ok(())
    }

    #[test]
    fn live_promotion_reports_condition_repair_as_covered_and_serializes_stably() {
        let report = production_capability_report(
            &promotion_decision_args(VlmProviderArg::Ollama, false, false),
            None,
            true,
            true,
        );
        let condition = report
            .iter()
            .find(|record| record.capability_id == "condition_extraction")
            .expect("condition extraction record");
        assert_eq!(condition.participation, CapabilityParticipation::Integrated);
        assert_eq!(condition.run_state, CapabilityRunState::Executed);

        let serialized = serde_json::to_value(condition).expect("serialize capability");
        assert_eq!(serialized["command"], "extract-conditions");
        assert_eq!(serialized["participation"], "integrated");
        assert_eq!(serialized["run_state"], "executed");
    }

    #[test]
    fn promotion_is_default_on_for_live_nlp() {
        // The LLM-PRIMARY-PROMOTION.5 flip: a live `--nlp-provider` with no extra flags promotes
        // the LLM-primary constraint surface by default.
        assert!(should_promote_constraints(&promotion_decision_args(
            VlmProviderArg::Ollama,
            false,
            false,
        )));
    }

    #[test]
    fn promotion_stays_off_for_provider_free_runs() {
        // Provider-free converge stays on the deterministic Pattern surface by construction, so
        // CI / kg-bench / provider-free runs are untouched by the flip.
        assert!(!should_promote_constraints(&promotion_decision_args(
            VlmProviderArg::Skip,
            false,
            false,
        )));
    }

    #[test]
    fn promotion_can_be_opted_out_with_a_live_provider() {
        // `--no-promote-constraints-llm` keeps the Pattern surface even when a live provider runs.
        assert!(!should_promote_constraints(&promotion_decision_args(
            VlmProviderArg::Ollama,
            false,
            true,
        )));
    }

    #[test]
    fn artifact_base_roots_are_repository_derived() -> Result<()> {
        let roots = PipelineArtifactRoots::repository_defaults()?;
        let repository = crate::project_data::repository_root()?;
        for root in [
            roots.source,
            roots.evidence,
            roots.semantic,
            roots.intent,
            roots.adapters,
        ] {
            assert!(root.starts_with(&repository));
        }
        Ok(())
    }

    #[test]
    fn provider_name_returns_correct_values() {
        assert_eq!(provider_name(VlmProviderArg::Ollama), "ollama");
        assert_eq!(provider_name(VlmProviderArg::OpenAi), "openai");
        assert_eq!(provider_name(VlmProviderArg::LmStudio), "lmstudio");
        assert_eq!(provider_name(VlmProviderArg::Skip), "skip");
    }

    #[test]
    fn render_lowering_status_returns_correct_values() {
        assert_eq!(
            render_lowering_status(AdapterLoweringStatus::Renderable),
            "renderable"
        );
        assert_eq!(
            render_lowering_status(AdapterLoweringStatus::Blocked),
            "blocked"
        );
    }

    // --- arbitration_status_for ---

    #[test]
    fn arbitration_status_for_dry_run_not_promoted() {
        assert_eq!(
            ConvergenceRescanPlanReport::arbitration_status_for(false, 0, false, 0),
            "dry_run_not_promoted"
        );
    }

    #[test]
    fn arbitration_status_for_changed_requires_review_from_review_required() {
        assert_eq!(
            ConvergenceRescanPlanReport::arbitration_status_for(true, 0, false, 1),
            "changed_requires_validation_review"
        );
    }

    #[test]
    fn arbitration_status_for_changed_requires_review_from_executed_changed() {
        assert_eq!(
            ConvergenceRescanPlanReport::arbitration_status_for(true, 1, false, 0),
            "changed_requires_validation_review"
        );
    }

    #[test]
    fn arbitration_status_for_changed_requires_review_from_snapshot_changed() {
        assert_eq!(
            ConvergenceRescanPlanReport::arbitration_status_for(true, 0, true, 0),
            "changed_requires_validation_review"
        );
    }

    #[test]
    fn arbitration_status_for_executed_validated_no_change() {
        assert_eq!(
            ConvergenceRescanPlanReport::arbitration_status_for(true, 0, false, 0),
            "executed_validated_no_change"
        );
    }

    #[test]
    fn arbitration_status_delegates_correctly() {
        let report = ConvergenceRescanPlanReport {
            plan_path: PathBuf::from("/tmp/plan.json"),
            executed: false,
            selected_recommendations: 0,
            executed_validated_changed: 0,
            executed_validated_no_change: 0,
            snapshot_changed: false,
            review_required: 0,
            possible_improvement_review_required: 0,
            regression_review_required: 0,
            neutral_change_review_required: 0,
        };
        assert_eq!(report.arbitration_status(), "dry_run_not_promoted");
    }

    // --- fact_count ---

    fn protocol_frame_field(field_id: &str) -> SerialFrameField {
        SerialFrameField {
            field_id: field_id.to_string(),
            name: "ALPHA".to_string(),
            bit_width: Some(1),
            bit_range: Some((0, 0)),
            phase_name: Some("opening".to_string()),
            participant_drive: Some(crate::ir::evidence::ParticipantDriveRecord {
                source_actor: "initiator".to_string(),
                destination_actor: Some("recipient".to_string()),
            }),
            order: Some(0),
            response_values: Vec::new(),
            supporting_statement_ids: vec!["statement_protocol".to_string()],
        }
    }

    fn protocol_operation(operation_id: &str) -> ProtocolOperationRecord {
        ProtocolOperationRecord {
            operation_id: operation_id.to_string(),
            branch_label: Some("accepted".to_string()),
            operation_name: Some("transfer".to_string()),
            phase_count: 3,
            phase_names: vec![
                "opening".to_string(),
                "body".to_string(),
                "closing".to_string(),
            ],
            supporting_statement_ids: vec!["statement_protocol".to_string()],
        }
    }

    fn protocol_state(state_id: &str) -> ProtocolStateRecord {
        ProtocolStateRecord {
            state_id: state_id.to_string(),
            machine_name: Some("serial machine".to_string()),
            state_name: "Operating".to_string(),
            action: Some("transfers data".to_string()),
            supporting_statement_ids: vec!["statement_protocol".to_string()],
        }
    }

    fn protocol_edge_timing(timing_id: &str) -> InterfaceEdgeTimingRecord {
        InterfaceEdgeTimingRecord {
            timing_id: timing_id.to_string(),
            actor_name: "target".to_string(),
            signal_name: "DATA".to_string(),
            clock_signal: "CLK".to_string(),
            edge: crate::ir::evidence::InterfaceClockEdge::Rising,
            samples_on_edge: true,
            drive_changes_on_edge: true,
            supporting_statement_ids: vec!["statement_protocol".to_string()],
        }
    }

    #[test]
    fn protocol_only_changes_are_visible_in_every_convergence_snapshot() -> Result<()> {
        let tempdir = tempdir()?;
        let source_path = tempdir.path().join("protocol_convergence.md");
        let generated = tempdir.path().join("generated");
        fs::write(&source_path, "# Protocol convergence fixture\n")?;

        let source_ir = SourceIr::build(&source_path, &generated.join("source_ir"))?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &generated.join("evidence_ir"),
        )?;
        let empty_evidence = EvidenceSnapshot::from_ir(&evidence_ir);

        let mut second_field = protocol_frame_field("serial_field_0002");
        second_field.name = "OMEGA".to_string();
        second_field.phase_name = Some("closing".to_string());
        second_field.participant_drive = Some(crate::ir::evidence::ParticipantDriveRecord {
            source_actor: "recipient".to_string(),
            destination_actor: Some("initiator".to_string()),
        });
        second_field.order = Some(1);
        evidence_ir.serial_frame_fields =
            vec![protocol_frame_field("serial_field_0001"), second_field];
        evidence_ir.protocol_operations = vec![protocol_operation("protocol_operation_0001")];
        evidence_ir.protocol_states = vec![protocol_state("protocol_state_0001")];
        evidence_ir.interface_edge_timings =
            vec![protocol_edge_timing("interface_edge_timing_0001")];
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &generated.join("semantic_ir"),
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &generated.join("intent_ir"),
        )?;

        let evidence_snapshot = EvidenceSnapshot::from_ir(&evidence_ir);
        let semantic_snapshot = SemanticSnapshot::from_ir(&semantic_ir);
        let intent_snapshot = IntentSnapshot::from_ir(&intent_ir);
        for fields in [
            &evidence_snapshot.serial_frame_fields,
            &semantic_snapshot.serial_frame_fields,
            &intent_snapshot.serial_frame_fields,
        ] {
            assert_eq!(fields, &evidence_ir.serial_frame_fields);
        }
        for operations in [
            &evidence_snapshot.protocol_operations,
            &semantic_snapshot.protocol_operations,
            &intent_snapshot.protocol_operations,
        ] {
            assert_eq!(operations, &evidence_ir.protocol_operations);
        }
        for states in [
            &evidence_snapshot.protocol_states,
            &semantic_snapshot.protocol_states,
            &intent_snapshot.protocol_states,
        ] {
            assert_eq!(states, &evidence_ir.protocol_states);
        }
        for timings in [
            &evidence_snapshot.interface_edge_timings,
            &semantic_snapshot.interface_edge_timings,
            &intent_snapshot.interface_edge_timings,
        ] {
            assert_eq!(timings, &evidence_ir.interface_edge_timings);
        }

        let mut empty_semantic_ir = semantic_ir.clone();
        empty_semantic_ir.serial_frame_fields.clear();
        empty_semantic_ir.protocol_operations.clear();
        empty_semantic_ir.protocol_states.clear();
        empty_semantic_ir.interface_edge_timings.clear();
        let empty_semantic = SemanticSnapshot::from_ir(&empty_semantic_ir);
        let mut empty_intent_ir = intent_ir.clone();
        empty_intent_ir.serial_frame_fields.clear();
        empty_intent_ir.protocol_operations.clear();
        empty_intent_ir.protocol_states.clear();
        empty_intent_ir.interface_edge_timings.clear();
        let empty_intent = IntentSnapshot::from_ir(&empty_intent_ir);
        let protocol_fact_count = 5;
        assert_eq!(
            evidence_snapshot.fact_count() - empty_evidence.fact_count(),
            protocol_fact_count
        );
        assert_eq!(
            semantic_snapshot.fact_count() - empty_semantic.fact_count(),
            protocol_fact_count
        );
        assert_eq!(
            intent_snapshot.fact_count() - empty_intent.fact_count(),
            protocol_fact_count
        );

        let mut changed_evidence_ir = evidence_ir.clone();
        changed_evidence_ir.serial_frame_fields[0].name = "CHANGED".to_string();
        let changed_evidence = EvidenceSnapshot::from_ir(&changed_evidence_ir);
        assert_ne!(changed_evidence, evidence_snapshot);
        assert_eq!(
            changed_evidence.fact_count(),
            evidence_snapshot.fact_count()
        );

        let mut changed_semantic_ir = semantic_ir.clone();
        changed_semantic_ir.protocol_operations[0].branch_label = Some("deferred".to_string());
        let changed_semantic = SemanticSnapshot::from_ir(&changed_semantic_ir);
        assert_ne!(changed_semantic, semantic_snapshot);
        assert_eq!(
            changed_semantic.fact_count(),
            semantic_snapshot.fact_count()
        );

        let mut reordered_intent_ir = intent_ir.clone();
        reordered_intent_ir.serial_frame_fields.swap(0, 1);
        let reordered_intent = IntentSnapshot::from_ir(&reordered_intent_ir);
        assert_ne!(reordered_intent, intent_snapshot);
        assert_eq!(reordered_intent.fact_count(), intent_snapshot.fact_count());

        Ok(())
    }

    #[test]
    fn source_snapshot_fact_count_sums_all_fields() {
        let s = SourceSnapshot {
            page_artifacts: 1,
            visual_assets: 2,
            structured_tables: 3,
            content_elements: 4,
            document_sections: 5,
            timing_diagrams: 6,
            state_machine_diagrams: 7,
            vlm_enriched_assets: 8,
        };
        // 1+2+3+4+5+6+7+8 = 36
        assert_eq!(s.fact_count(), 36);
    }

    #[test]
    fn source_snapshot_fact_count_zero() {
        let s = SourceSnapshot {
            page_artifacts: 0,
            visual_assets: 0,
            structured_tables: 0,
            content_elements: 0,
            document_sections: 0,
            timing_diagrams: 0,
            state_machine_diagrams: 0,
            vlm_enriched_assets: 0,
        };
        assert_eq!(s.fact_count(), 0);
    }

    #[test]
    fn evidence_snapshot_fact_count_sums_fields() {
        let s = EvidenceSnapshot {
            section_anchors: 1,
            evidence_spans: 1,
            visual_evidence: 1,
            extracted_statements: 1,
            signal_value_statements: 1,
            conditional_rule_statements: 1,
            normative_statements: 1,
            signal_constraints: 1,
            conditional_rules: 1,
            actor_signal_relations: 1,
            register_records: 1,
            timing_constraints: 1,
            serial_frame_fields: vec![SerialFrameField {
                field_id: "serial_field_0001".to_string(),
                name: "ALPHA".to_string(),
                bit_width: None,
                bit_range: None,
                phase_name: None,
                participant_drive: None,
                order: None,
                response_values: Vec::new(),
                supporting_statement_ids: Vec::new(),
            }],
            protocol_operations: vec![ProtocolOperationRecord {
                operation_id: "protocol_operation_0001".to_string(),
                branch_label: Some("accepted".to_string()),
                operation_name: Some("transfer".to_string()),
                phase_count: 3,
                phase_names: Vec::new(),
                supporting_statement_ids: Vec::new(),
            }],
            protocol_states: vec![ProtocolStateRecord {
                state_id: "protocol_state_0001".to_string(),
                machine_name: None,
                state_name: "Operating".to_string(),
                action: None,
                supporting_statement_ids: Vec::new(),
            }],
            interface_edge_timings: vec![InterfaceEdgeTimingRecord {
                timing_id: "interface_edge_timing_0001".to_string(),
                actor_name: "target".to_string(),
                signal_name: "DATA".to_string(),
                clock_signal: "CLK".to_string(),
                edge: crate::ir::evidence::InterfaceClockEdge::Rising,
                samples_on_edge: true,
                drive_changes_on_edge: true,
                supporting_statement_ids: Vec::new(),
            }],
            alias_map_size: 1,
            timing_diagram_observations: 1,
            state_machine_observations: 1,
        };
        // 18 counted facts (normative_statements is diagnostic-only; each protocol record counts).
        assert_eq!(s.fact_count(), 18);
    }

    #[test]
    fn semantic_snapshot_fact_count_includes_system_contract_present() {
        let s = SemanticSnapshot {
            actors: 1,
            interfaces: 1,
            interface_signals: 1,
            phases: 1,
            invariants: 1,
            contracts: 1,
            gates: 1,
            assertions: 1,
            abstractions: 1,
            decomposition_candidates: 1,
            system_contract_present: true,
            regular_states: 1,
            state_transitions: 1,
            symbol_definitions: 1,
            control_blocks: 1,
            explicit_modules: 1,
            explicit_tops: 1,
            register_records: 1,
            timing_constraints: 1,
            serial_frame_fields: vec![protocol_frame_field("semantic_field")],
            protocol_operations: vec![protocol_operation("semantic_operation")],
            protocol_states: vec![protocol_state("semantic_state")],
            interface_edge_timings: vec![protocol_edge_timing("semantic_edge")],
            signal_constraints: 1,
            conditional_rules: 1,
            residual_decisions: 0,
        };
        // 25 counted facts, each 1 (system_contract_present=true → 1) = 25.
        // (was 23; ISF-ONLY-IR-PRUNE.2 removed init_assignments +
        // decision_tree_fragments from the snapshot — convergence deltas are
        // unchanged since the surface is gone from every snapshot equally)
        assert_eq!(s.fact_count(), 25);
    }

    #[test]
    fn semantic_snapshot_fact_count_system_contract_absent() {
        let s = SemanticSnapshot {
            system_contract_present: false,
            ..default_semantic_snapshot()
        };
        // system_contract_present=false → 0, all others 0 → 0
        assert_eq!(s.fact_count(), 0);
    }

    #[test]
    fn intent_snapshot_fact_count_includes_system_contract_present() {
        let s = IntentSnapshot {
            actors: 1,
            interfaces: 1,
            interface_signals: 1,
            behaviors: 1,
            constraints: 1,
            assumptions: 1,
            system_contract_present: true,
            regular_states: 1,
            state_transitions: 1,
            symbol_definitions: 1,
            control_blocks: 1,
            explicit_modules: 1,
            explicit_tops: 1,
            register_records: 1,
            timing_constraints: 1,
            serial_frame_fields: vec![protocol_frame_field("intent_field")],
            protocol_operations: vec![protocol_operation("intent_operation")],
            protocol_states: vec![protocol_state("intent_state")],
            interface_edge_timings: vec![protocol_edge_timing("intent_edge")],
            signal_constraints: 1,
            conditional_rules: 1,
            residual_decisions: 0,
        };
        // 21 counted facts, each 1 (system_contract_present=true → 1) = 21.
        // (was 19; ISF-ONLY-IR-PRUNE.2 removed init_assignments +
        // decision_tree_fragments from the snapshot — convergence deltas are
        // unchanged since the surface is gone from every snapshot equally)
        assert_eq!(s.fact_count(), 21);
    }

    fn default_semantic_snapshot() -> SemanticSnapshot {
        SemanticSnapshot {
            actors: 0,
            interfaces: 0,
            interface_signals: 0,
            phases: 0,
            invariants: 0,
            contracts: 0,
            gates: 0,
            assertions: 0,
            abstractions: 0,
            decomposition_candidates: 0,
            system_contract_present: false,
            regular_states: 0,
            state_transitions: 0,
            symbol_definitions: 0,
            control_blocks: 0,
            explicit_modules: 0,
            explicit_tops: 0,
            register_records: 0,
            timing_constraints: 0,
            serial_frame_fields: Vec::new(),
            protocol_operations: Vec::new(),
            protocol_states: Vec::new(),
            interface_edge_timings: Vec::new(),
            signal_constraints: 0,
            conditional_rules: 0,
            residual_decisions: 0,
        }
    }

    // --- absolute_artifact_path ---

    #[test]
    fn absolute_artifact_path_returns_local_absolute_path_unchanged() {
        let repository = crate::project_data::repository_root().unwrap();
        let local = repository.join("generated/test-output/file.json");
        let result = absolute_artifact_path(&local).unwrap();
        assert_eq!(result, local);
    }

    #[test]
    fn absolute_artifact_path_resolves_relative_path_at_repository() {
        let repository = crate::project_data::repository_root().unwrap();
        let rel = std::path::Path::new("relative/file.json");
        let result = absolute_artifact_path(rel).unwrap();
        assert_eq!(result, repository.join("relative/file.json"));
    }

    #[test]
    fn absolute_artifact_path_rejects_external_output() {
        let external = std::path::Path::new("/absolute/path/file.json");
        assert!(absolute_artifact_path(external).is_err());
    }
}
