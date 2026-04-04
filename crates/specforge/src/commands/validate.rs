use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::ValidateArgs;
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{EvidenceIr, StatementClass, VisualObservationKind};
use crate::ir::intent::IntentIr;
use crate::ir::semantic::{ActorPortRecord, ActorRelativeDirection, SemanticIr};
use crate::ir::source::{
    AutomationConfidence, DiagramKind, SourceIr, ValidationFindingRecord,
    ValidationFindingSeverity, ValidationMetricRecord, ValidationReportRecord, WidthHint,
};

pub fn run(args: ValidateArgs) -> Result<()> {
    // Auto-detect stage from artifact JSON `stage` field.
    let raw = fs::read_to_string(&args.artifact)
        .map_err(|_| AppError::MissingPath(args.artifact.clone()))?;

    #[derive(serde::Deserialize)]
    struct StageProbe {
        stage: IrStage,
    }
    let probe: StageProbe = serde_json::from_str(&raw).map_err(|e| {
        AppError::InvalidStageArtifact(format!(
            "cannot determine stage from {}: {e}",
            args.artifact.display()
        ))
    })?;

    match probe.stage {
        IrStage::SourceIr => {
            let mut ir = SourceIr::load_from_path(&args.artifact)?;
            let report = validate_source_ir(&ir, source_ir_fingerprint(&ir)?);
            persist_source_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
        IrStage::EvidenceIr => {
            let mut ir = EvidenceIr::load_from_path(&args.artifact)?;
            let report = validate_evidence_ir(&ir, evidence_ir_fingerprint(&ir)?);
            persist_evidence_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
        IrStage::SemanticIr => {
            let mut ir = SemanticIr::load_from_path(&args.artifact)?;
            let report = validate_semantic_ir(&ir, semantic_ir_fingerprint(&ir)?);
            persist_semantic_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
        IrStage::IntentIr => {
            let mut ir = IntentIr::load_from_path(&args.artifact)?;
            let report = validate_intent_ir(&ir, intent_ir_fingerprint(&ir)?);
            persist_intent_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
    }

    Ok(())
}

fn stable_fingerprint(text: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn source_ir_fingerprint(ir: &SourceIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn evidence_ir_fingerprint(ir: &EvidenceIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn semantic_ir_fingerprint(ir: &SemanticIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn intent_ir_fingerprint(ir: &IntentIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn validation_report_path_for(artifact_path: &Path) -> Result<PathBuf> {
    let artifact_dir = artifact_path.parent().ok_or_else(|| {
        AppError::InvalidStageArtifact(format!(
            "cannot derive validation report path for {}",
            artifact_path.display()
        ))
    })?;
    Ok(artifact_dir.join("validation_report.json"))
}

fn metric(name: &str, value: impl Into<String>) -> ValidationMetricRecord {
    ValidationMetricRecord {
        name: name.to_string(),
        value: value.into(),
    }
}

fn finding(
    finding_id: &str,
    severity: ValidationFindingSeverity,
    category: &str,
    summary: impl Into<String>,
    related_ids: Vec<String>,
) -> ValidationFindingRecord {
    ValidationFindingRecord {
        finding_id: finding_id.to_string(),
        severity,
        category: category.to_string(),
        summary: summary.into(),
        related_ids,
    }
}

fn graph_direction_signal_names(actor_ports: &[ActorPortRecord]) -> BTreeSet<String> {
    actor_ports
        .iter()
        .filter(|port| !matches!(port.direction, ActorRelativeDirection::Unknown))
        .map(|port| port.signal_name.clone())
        .collect()
}

fn resolved_direction_counts<'a>(
    signal_records: impl IntoIterator<Item = &'a crate::ir::semantic::InterfaceSignalRecord>,
    graph_direction_signals: &BTreeSet<String>,
) -> (usize, usize, usize) {
    let mut resolved = 0usize;
    let mut graph = 0usize;
    let mut compat = 0usize;

    for signal in signal_records {
        let graph_has_direction = graph_direction_signals.contains(&signal.signal_name);
        let compat_has_direction = signal.direction_hint.is_some();
        if graph_has_direction || compat_has_direction {
            resolved += 1;
        }
        if graph_has_direction {
            graph += 1;
        }
        if compat_has_direction {
            compat += 1;
        }
    }

    (resolved, graph, compat)
}

fn backannotate_report(target: &mut Vec<ValidationReportRecord>, report: &ValidationReportRecord) {
    target.clear();
    target.push(report.clone());
}

fn write_validation_report_sidecar(
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    let report_path = validation_report_path_for(artifact_path)?;
    fs::write(report_path, serde_json::to_string_pretty(report)?)?;
    Ok(())
}

fn print_validation_findings(report: &ValidationReportRecord) {
    println!("=== Validation Findings ===");
    println!("  count: {}", report.findings.len());
    if report.findings.is_empty() {
        println!("  none");
    } else {
        for finding in &report.findings {
            println!(
                "  - [{}:{}] {}",
                finding.severity.as_str(),
                finding.category,
                finding.summary
            );
            if !finding.related_ids.is_empty() {
                println!("    related_ids: {}", finding.related_ids.join(", "));
            }
        }
    }
    println!();
}

fn print_validation_backannotation(
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    let report_path = validation_report_path_for(artifact_path)?;
    println!("=== Validation Backannotation ===");
    println!("  artifact_path: {}", artifact_path.display());
    println!("  validation_report_path: {}", report_path.display());
    println!("  artifact_fingerprint: {}", report.artifact_fingerprint);
    Ok(())
}

fn persist_source_validation(
    ir: &mut SourceIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn persist_evidence_validation(
    ir: &mut EvidenceIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn persist_semantic_validation(
    ir: &mut SemanticIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn persist_intent_validation(
    ir: &mut IntentIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn validate_source_ir(ir: &SourceIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: source_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    println!("=== Document Profile ===");
    println!("pages: {}", ir.page_artifacts.len());
    println!("visual_assets: {}", ir.visual_assets.len());
    println!("structured_tables: {}", ir.structured_tables.len());
    println!("content_elements: {}", ir.content_elements.len());
    println!("document_sections: {}", ir.document_sections.len());
    if let Some(profile) = &ir.document_profile {
        if let Some(title) = &profile.title {
            println!("document_title: {title}");
        }
    }
    println!();

    println!("=== Table Classification ===");
    let mut table_kinds: HashMap<&str, usize> = HashMap::new();
    for table in &ir.structured_tables {
        *table_kinds
            .entry(format!("{:?}", table.table_kind).leak())
            .or_insert(0) += 1;
    }
    for (kind, count) in sorted_by_value(&table_kinds) {
        println!("  {kind}: {count}");
    }
    println!();

    println!("=== Diagram Classification ===");
    let timing_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::TimingDiagram))
        .count();
    let state_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::StateMachineDiagram))
        .count();
    let block_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::BlockDiagram))
        .count();
    let unknown_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::Unknown))
        .count();
    println!("  timing_diagram: {timing_count}");
    println!("  state_machine_diagram: {state_count}");
    println!("  block_diagram: {block_count}");
    println!("  unknown: {unknown_count}");
    let classified = timing_count + state_count + block_count;
    let diagram_coverage = if ir.visual_assets.is_empty() {
        0.0
    } else {
        classified as f64 / ir.visual_assets.len() as f64 * 100.0
    };
    println!("  diagram_classification_coverage: {diagram_coverage:.0}%");
    println!();

    println!("=== VLM Readiness ===");
    let vlm_enriched = ir
        .visual_assets
        .iter()
        .filter(|a| {
            a.note.as_deref().map(|n| {
                n.starts_with("vlm_timing_diagram_extraction:")
                    || n.starts_with("vlm_state_machine_extraction:")
            }) == Some(true)
        })
        .count();
    let vlm_ready = timing_count + state_count;
    println!(
        "  figures_ready_for_vlm: {vlm_ready} (timing: {timing_count}, state_machine: {state_count})"
    );
    println!("  figures_already_enriched: {vlm_enriched}");
    if vlm_ready > 0 && vlm_enriched == 0 {
        println!(
            "  hint: run `specforge enrich <source-ir> --vlm-provider ollama` to enrich {vlm_ready} diagrams"
        );
    }
    println!();

    println!("=== Section Classification ===");
    let mut section_kinds: HashMap<&str, usize> = HashMap::new();
    for s in &ir.document_sections {
        *section_kinds
            .entry(format!("{:?}", s.section_kind).leak())
            .or_insert(0) += 1;
    }
    for (kind, count) in sorted_by_value(&section_kinds) {
        println!("  {kind}: {count}");
    }

    println!();
    println!("=== Residual Decisions ===");
    println!("  count: {}", ir.residual_decisions.len());

    let figures_ready_for_vlm = timing_count + state_count;
    let mut findings = Vec::new();
    if figures_ready_for_vlm > 0 && vlm_enriched == 0 {
        findings.push(finding(
            "source_vlm_enrichment_missing",
            ValidationFindingSeverity::Warning,
            "visual_enrichment",
            format!(
                "{figures_ready_for_vlm} classified timing/state diagrams are still missing VLM enrichment"
            ),
            ir.visual_assets
                .iter()
                .filter(|asset| {
                    matches!(
                        asset.diagram_kind,
                        DiagramKind::TimingDiagram | DiagramKind::StateMachineDiagram
                    )
                })
                .map(|asset| asset.asset_id.clone())
                .take(6)
                .collect(),
        ));
    }
    if unknown_count > 0 {
        findings.push(finding(
            "source_unknown_diagrams_remaining",
            ValidationFindingSeverity::Info,
            "diagram_classification",
            format!("{unknown_count} visual assets remain unclassified"),
            ir.visual_assets
                .iter()
                .filter(|asset| matches!(asset.diagram_kind, DiagramKind::Unknown))
                .map(|asset| asset.asset_id.clone())
                .take(6)
                .collect(),
        ));
    }
    if !ir.residual_decisions.is_empty() {
        findings.push(finding(
            "source_residual_decisions_present",
            ValidationFindingSeverity::Warning,
            "residual_decisions",
            format!(
                "SourceIR still carries {} residual decision packet(s)",
                ir.residual_decisions.len()
            ),
            ir.residual_decisions
                .iter()
                .map(|packet| packet.packet_id.clone())
                .collect(),
        ));
    }

    let report = ValidationReportRecord {
        report_id: format!("validation_source_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::SourceIr,
        artifact_fingerprint,
        summary: format!(
            "SourceIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: None,
        grade: None,
        metrics: vec![
            metric("pages", ir.page_artifacts.len().to_string()),
            metric("visual_assets", ir.visual_assets.len().to_string()),
            metric("structured_tables", ir.structured_tables.len().to_string()),
            metric("content_elements", ir.content_elements.len().to_string()),
            metric("document_sections", ir.document_sections.len().to_string()),
            metric("timing_diagrams", timing_count.to_string()),
            metric("state_machine_diagrams", state_count.to_string()),
            metric("block_diagrams", block_count.to_string()),
            metric("unknown_diagrams", unknown_count.to_string()),
            metric(
                "diagram_classification_coverage_pct",
                format!("{diagram_coverage:.0}"),
            ),
            metric("figures_ready_for_vlm", figures_ready_for_vlm.to_string()),
            metric("figures_already_enriched", vlm_enriched.to_string()),
            metric(
                "residual_decisions",
                ir.residual_decisions.len().to_string(),
            ),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn validate_evidence_ir(ir: &EvidenceIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: evidence_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    println!("=== Statement Classification ===");
    let total = ir.extracted_statements.len();
    let mut classes: HashMap<&str, usize> = HashMap::new();
    for s in &ir.extracted_statements {
        let name = match s.class {
            StatementClass::SignalValueConstraint => "signal_value_constraint",
            StatementClass::TimingConstraint => "timing_constraint",
            StatementClass::ConditionalRule => "conditional_rule",
            StatementClass::NormativeStatement => "normative_statement",
            StatementClass::DerivedRule => "derived_rule",
            StatementClass::LocalDesignDecision => "local_design_decision",
            StatementClass::ExplicitAbstraction => "explicit_abstraction",
            StatementClass::SourceFact => "source_fact",
            StatementClass::Unknown => "unknown",
        };
        *classes.entry(name).or_insert(0) += 1;
    }
    println!("  total_statements: {total}");
    for (class, count) in sorted_by_value(&classes) {
        let pct = if total > 0 { count * 100 / total } else { 0 };
        println!("  {class}: {count} ({pct}%)");
    }
    let non_fact = total - classes.get("source_fact").copied().unwrap_or(0);
    let nlp_coverage = if total > 0 { non_fact * 100 / total } else { 0 };
    println!("  nlp_coverage (non-source_fact): {nlp_coverage}%");
    println!();

    println!("=== Structured Extraction ===");
    println!(
        "  signal_constraints (Level 2 NLP): {}",
        ir.signal_constraints.len()
    );
    println!(
        "  conditional_rules (Level 2 NLP): {}",
        ir.conditional_rules.len()
    );
    println!(
        "  register_records (from tables): {}",
        ir.register_records.len()
    );
    println!(
        "  timing_constraints (from tables): {}",
        ir.timing_constraints.len()
    );
    println!();

    println!("=== VLM Observations ===");
    let mut timing_obs = 0usize;
    let mut state_obs = 0usize;
    for item in &ir.visual_evidence {
        for obs in &item.observations {
            match obs.kind {
                VisualObservationKind::TimingDiagramExtraction => timing_obs += 1,
                VisualObservationKind::StateMachineExtraction => state_obs += 1,
                _ => {}
            }
        }
    }
    println!("  timing_diagram_extractions: {timing_obs}");
    println!("  state_machine_extractions: {state_obs}");
    if timing_obs == 0 && state_obs == 0 {
        println!(
            "  hint: run `specforge enrich` then re-run `specforge evidence` to populate VLM observations"
        );
    }
    println!();

    println!("=== Visual Evidence ===");
    println!("  total: {}", ir.visual_evidence.len());
    let with_caption = ir
        .visual_evidence
        .iter()
        .filter(|e| e.caption_text.is_some())
        .count();
    println!("  with_caption: {with_caption}");

    let normative_count = classes.get("normative_statement").copied().unwrap_or(0);
    let mut findings = Vec::new();
    if total == 0 {
        findings.push(finding(
            "evidence_no_extracted_statements",
            ValidationFindingSeverity::Error,
            "statement_extraction",
            "EvidenceIR contains no extracted statements",
            Vec::new(),
        ));
    }
    if timing_obs == 0 && state_obs == 0 && !ir.visual_evidence.is_empty() {
        findings.push(finding(
            "evidence_missing_vlm_observations",
            ValidationFindingSeverity::Warning,
            "visual_enrichment",
            "Visual evidence is present, but no VLM timing/state observations were injected into EvidenceIR",
            ir.visual_evidence
                .iter()
                .map(|item| item.evidence_id.clone())
                .take(6)
                .collect(),
        ));
    }
    if ir.actor_signal_relations.is_empty()
        && (!ir.signal_constraints.is_empty() || !ir.conditional_rules.is_empty())
    {
        findings.push(finding(
            "evidence_structural_kg_missing",
            ValidationFindingSeverity::Warning,
            "knowledge_graph",
            "Behavioral evidence exists, but the structural actor-signal graph is still empty in EvidenceIR",
            Vec::new(),
        ));
    }
    if normative_count > 0 {
        findings.push(finding(
            "evidence_normative_residuals_remaining",
            ValidationFindingSeverity::Info,
            "nlp_residuals",
            format!(
                "{normative_count} normative statements remain only partially structured in EvidenceIR"
            ),
            Vec::new(),
        ));
    }

    let report = ValidationReportRecord {
        report_id: format!("validation_evidence_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::EvidenceIr,
        artifact_fingerprint,
        summary: format!(
            "EvidenceIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: None,
        grade: None,
        metrics: vec![
            metric("total_statements", total.to_string()),
            metric("nlp_coverage_pct", nlp_coverage.to_string()),
            metric(
                "signal_value_constraint_statements",
                classes
                    .get("signal_value_constraint")
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
            ),
            metric(
                "conditional_rule_statements",
                classes
                    .get("conditional_rule")
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
            ),
            metric("normative_statements", normative_count.to_string()),
            metric(
                "signal_constraints",
                ir.signal_constraints.len().to_string(),
            ),
            metric("conditional_rules", ir.conditional_rules.len().to_string()),
            metric(
                "actor_signal_relations",
                ir.actor_signal_relations.len().to_string(),
            ),
            metric("register_records", ir.register_records.len().to_string()),
            metric(
                "timing_constraints",
                ir.timing_constraints.len().to_string(),
            ),
            metric("timing_diagram_extractions", timing_obs.to_string()),
            metric("state_machine_extractions", state_obs.to_string()),
            metric(
                "visual_evidence_total",
                ir.visual_evidence.len().to_string(),
            ),
            metric("visual_evidence_with_caption", with_caption.to_string()),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn validate_semantic_ir(ir: &SemanticIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: semantic_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    println!("=== Interface / Signal Coverage ===");
    let total_signals: usize = ir.interfaces.iter().map(|i| i.signal_records.len()).sum();
    let graph_direction_signals = graph_direction_signal_names(&ir.actor_ports);
    let (with_direction, with_graph_direction, with_compat_direction_hint) =
        resolved_direction_counts(
            ir.interfaces.iter().flat_map(|i| i.signal_records.iter()),
            &graph_direction_signals,
        );
    let with_width: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| s.width_hint.is_some())
        .count();
    let fully_typed = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| {
            s.width_hint.is_some()
                && (graph_direction_signals.contains(&s.signal_name) || s.direction_hint.is_some())
        })
        .count();
    println!("  total_signal_records: {total_signals}");
    let dir_pct = if total_signals > 0 {
        with_direction * 100 / total_signals
    } else {
        0
    };
    let w_pct = if total_signals > 0 {
        with_width * 100 / total_signals
    } else {
        0
    };
    let ft_pct = if total_signals > 0 {
        fully_typed * 100 / total_signals
    } else {
        0
    };
    let graph_dir_pct = if total_signals > 0 {
        with_graph_direction * 100 / total_signals
    } else {
        0
    };
    let compat_dir_pct = if total_signals > 0 {
        with_compat_direction_hint * 100 / total_signals
    } else {
        0
    };
    println!("  with_resolved_direction: {with_direction} ({dir_pct}%)");
    println!("  with_graph_direction: {with_graph_direction} ({graph_dir_pct}%)");
    println!("  with_compat_direction_hint: {with_compat_direction_hint} ({compat_dir_pct}%)");
    println!("  with_width: {with_width} ({w_pct}%)");
    println!("  fully_typed (resolved_direction+width): {fully_typed} ({ft_pct}%)");
    println!();

    println!("=== Semantic Records ===");
    println!("  actors: {}", ir.actors.len());
    println!(
        "  actor_signal_relations: {}",
        ir.actor_signal_relations.len()
    );
    println!("  actor_ports: {}", ir.actor_ports.len());
    println!("  signal_connectivity: {}", ir.signal_connectivity.len());
    println!("  interfaces: {}", ir.interfaces.len());
    println!("  invariants: {}", ir.invariants.len());
    println!(
        "  symbol_definitions (enums+consts): {}",
        ir.symbol_definitions.len()
    );
    println!("  regular_states: {}", ir.regular_states.len());
    println!("  state_transitions: {}", ir.state_transitions.len());
    println!("  register_records: {}", ir.register_records.len());
    println!("  timing_constraints: {}", ir.timing_constraints.len());
    println!(
        "  signal_constraints (Level 2 NLP): {}",
        ir.signal_constraints.len()
    );
    println!(
        "  conditional_rules (Level 2 NLP): {}",
        ir.conditional_rules.len()
    );
    println!();

    println!("=== System Contract ===");
    if let Some(sc) = &ir.system_contract {
        println!("  clock_signal: {}", sc.clock_signal);
        println!("  reset_signal: {}", sc.reset_signal);
        println!("  reset_kind: {:?}", sc.reset_kind);
        println!("  automation_confidence: {:?}", sc.automation_confidence);
    } else {
        println!("  ABSENT — no explicit Clock/Reset declarations found");
    }
    println!();

    println!("=== Residual Decisions ===");
    println!("  count: {}", ir.residual_decisions.len());
    for rd in &ir.residual_decisions {
        println!("  - [{}] {}", rd.packet_id, rd.question);
    }

    let missing_producer_signals: Vec<String> = ir
        .signal_connectivity
        .iter()
        .filter(|record| record.producer_actor_ids.is_empty())
        .map(|record| record.signal_name.clone())
        .collect();
    let missing_consumer_signals: Vec<String> = ir
        .signal_connectivity
        .iter()
        .filter(|record| record.consumer_actor_ids.is_empty())
        .map(|record| record.signal_name.clone())
        .collect();
    let missing_graph_direction_count = total_signals.saturating_sub(with_graph_direction);
    let missing_compat_direction_count = total_signals.saturating_sub(with_compat_direction_hint);

    let mut findings = Vec::new();
    if !ir.actor_signal_relations.is_empty() && ir.actor_ports.is_empty() {
        findings.push(finding(
            "semantic_actor_ports_missing",
            ValidationFindingSeverity::Error,
            "knowledge_graph",
            "SemanticIR carries actor-signal relations but failed to synthesize actor-relative ports",
            Vec::new(),
        ));
    }
    if !missing_producer_signals.is_empty() {
        findings.push(finding(
            "semantic_connectivity_missing_producer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in SemanticIR connectivity have no resolved producer actor",
                missing_producer_signals.len()
            ),
            missing_producer_signals.iter().take(8).cloned().collect(),
        ));
    }
    if !missing_consumer_signals.is_empty() {
        findings.push(finding(
            "semantic_connectivity_missing_consumer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in SemanticIR connectivity have no resolved consumer actor",
                missing_consumer_signals.len()
            ),
            missing_consumer_signals.iter().take(8).cloned().collect(),
        ));
    }
    if !ir.actor_ports.is_empty() && missing_graph_direction_count > 0 {
        findings.push(finding(
            "semantic_graph_direction_coverage_incomplete",
            ValidationFindingSeverity::Info,
            "knowledge_graph",
            format!(
                "{missing_graph_direction_count} interface signal record(s) still lack graph-derived direction coverage"
            ),
            Vec::new(),
        ));
    }
    if missing_compat_direction_count > 0 {
        findings.push(finding(
            "semantic_compat_direction_hints_incomplete",
            ValidationFindingSeverity::Info,
            "compatibility_surface",
            format!(
                "{missing_compat_direction_count} interface signal record(s) still lack flat compatibility direction hints"
            ),
            Vec::new(),
        ));
    }
    if !ir.residual_decisions.is_empty() {
        findings.push(finding(
            "semantic_residual_decisions_present",
            ValidationFindingSeverity::Warning,
            "residual_decisions",
            format!(
                "SemanticIR still carries {} residual decision packet(s)",
                ir.residual_decisions.len()
            ),
            ir.residual_decisions
                .iter()
                .map(|packet| packet.packet_id.clone())
                .collect(),
        ));
    }

    let report = ValidationReportRecord {
        report_id: format!("validation_semantic_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::SemanticIr,
        artifact_fingerprint,
        summary: format!(
            "SemanticIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: None,
        grade: None,
        metrics: vec![
            metric("total_signal_records", total_signals.to_string()),
            metric("with_resolved_direction", with_direction.to_string()),
            metric("with_graph_direction", with_graph_direction.to_string()),
            metric(
                "with_compat_direction_hint",
                with_compat_direction_hint.to_string(),
            ),
            metric("with_width", with_width.to_string()),
            metric("fully_typed", fully_typed.to_string()),
            metric("actors", ir.actors.len().to_string()),
            metric(
                "actor_signal_relations",
                ir.actor_signal_relations.len().to_string(),
            ),
            metric("actor_ports", ir.actor_ports.len().to_string()),
            metric(
                "signal_connectivity",
                ir.signal_connectivity.len().to_string(),
            ),
            metric("interfaces", ir.interfaces.len().to_string()),
            metric("invariants", ir.invariants.len().to_string()),
            metric(
                "symbol_definitions",
                ir.symbol_definitions.len().to_string(),
            ),
            metric("regular_states", ir.regular_states.len().to_string()),
            metric("state_transitions", ir.state_transitions.len().to_string()),
            metric("register_records", ir.register_records.len().to_string()),
            metric(
                "timing_constraints",
                ir.timing_constraints.len().to_string(),
            ),
            metric(
                "signal_constraints",
                ir.signal_constraints.len().to_string(),
            ),
            metric("conditional_rules", ir.conditional_rules.len().to_string()),
            metric(
                "residual_decisions",
                ir.residual_decisions.len().to_string(),
            ),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn validate_intent_ir(ir: &IntentIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: intent_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    // Layer E: count declared signals as High OR Medium confidence.
    // High = from structured signal description tables (most authoritative).
    // Medium = from Tier 2 KG actor-signal relation extraction from prose.
    // Low = heuristic co-mention noise — excluded from coverage metrics.
    let declared_signals: Vec<_> = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| !matches!(s.automation_confidence, AutomationConfidence::Low))
        .collect();
    let heuristic_signals: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| matches!(s.automation_confidence, AutomationConfidence::Low))
        .count();

    println!("=== Signal Coverage ===");
    let declared_count = declared_signals.len();
    let graph_direction_signals = graph_direction_signal_names(&ir.actor_ports);
    let (with_direction, with_graph_direction, with_compat_direction_hint) =
        resolved_direction_counts(declared_signals.iter().copied(), &graph_direction_signals);
    // Both numeric and parametric widths count as "known" — parametric means the
    // integrator will set the value (e.g. ADDR_WIDTH=32) at instantiation time.
    let with_numeric_width = declared_signals
        .iter()
        .filter(|s| matches!(s.width_hint, Some(WidthHint::Numeric(_))))
        .count();
    let with_parametric_width = declared_signals
        .iter()
        .filter(|s| matches!(s.width_hint, Some(WidthHint::Parametric(_))))
        .count();
    let with_width = with_numeric_width + with_parametric_width;
    let dir_pct = if declared_count > 0 {
        with_direction * 100 / declared_count
    } else {
        0
    };
    let w_pct = if declared_count > 0 {
        with_width * 100 / declared_count
    } else {
        0
    };
    let graph_dir_pct = if declared_count > 0 {
        with_graph_direction * 100 / declared_count
    } else {
        0
    };
    let compat_dir_pct = if declared_count > 0 {
        with_compat_direction_hint * 100 / declared_count
    } else {
        0
    };
    println!("  declared_signal_records: {declared_count}");
    println!("  heuristic_signal_records (excluded from coverage): {heuristic_signals}");
    println!("  with_resolved_direction: {with_direction} ({dir_pct}%)");
    println!("  with_graph_direction: {with_graph_direction} ({graph_dir_pct}%)");
    println!("  with_compat_direction_hint: {with_compat_direction_hint} ({compat_dir_pct}%)");
    println!(
        "  with_width: {with_width} ({w_pct}%) [{with_numeric_width} numeric, {with_parametric_width} parametric]"
    );
    println!();

    println!("=== Intent Records ===");
    println!("  actors: {}", ir.actors.len());
    println!(
        "  actor_signal_relations: {}",
        ir.actor_signal_relations.len()
    );
    println!("  actor_ports: {}", ir.actor_ports.len());
    println!("  signal_connectivity: {}", ir.signal_connectivity.len());
    println!("  behaviors: {}", ir.behaviors.len());
    println!("  constraints: {}", ir.constraints.len());
    println!("  assumptions: {}", ir.assumptions.len());
    println!("  symbol_definitions: {}", ir.symbol_definitions.len());
    println!("  regular_states: {}", ir.regular_states.len());
    println!("  state_transitions: {}", ir.state_transitions.len());
    println!("  register_records: {}", ir.register_records.len());
    println!("  timing_constraints: {}", ir.timing_constraints.len());
    println!("  signal_constraints: {}", ir.signal_constraints.len());
    println!("  conditional_rules: {}", ir.conditional_rules.len());
    println!();

    // Layer E: spec-type-aware quality scoring.
    //
    // Points breakdown (total max = 100):
    //   Signal direction coverage (declared only): 0–25 pts
    //   Signal width coverage    (declared only):  0–10 pts
    //   NLP constraint richness  (sig + cond):     0–30 pts  (capped at 30 constraints)
    //   Encoding enum definitions:                 0–15 pts
    //   Register map records:                      0–5  pts
    //   Timing constraint records:                 0–5  pts
    //   State machine (FSM specs):                 0–5  pts  (bonus, not penalised if absent)
    //   System contract (clock/reset):             0–5  pts  (bonus, not penalised if absent)
    //
    // The FSM/contract bonuses are additive rather than penalties so that bus protocol
    // specs like AHB (which have no FSM or explicit clock declaration) are scored on the
    // merit of what they *do* contain rather than penalised for spec-appropriate omissions.
    let has_states = !ir.regular_states.is_empty();
    let has_enums = !ir.symbol_definitions.is_empty();
    let has_constraints = ir.signal_constraints.len() + ir.conditional_rules.len();
    let has_system_contract = ir.system_contract.is_some();
    let has_registers = !ir.register_records.is_empty();
    let has_timing = !ir.timing_constraints.is_empty();

    let dir_score = dir_pct as f64 * 0.25; // 0–25
    let width_score = w_pct as f64 * 0.10; // 0–10
    let constraint_score = has_constraints.min(30) as f64; // 0–30
    let enum_score = if has_enums { 15.0_f64 } else { 0.0 }; // 0–15
    let register_score = if has_registers { 5.0_f64 } else { 0.0 }; // 0–5
    let timing_score = if has_timing { 5.0_f64 } else { 0.0 }; // 0–5
    let fsm_score = if has_states { 5.0_f64 } else { 0.0 }; // 0–5  (bonus)
    let contract_score = if has_system_contract { 5.0_f64 } else { 0.0 }; // 0–5  (bonus)

    let score = (dir_score
        + width_score
        + constraint_score
        + enum_score
        + register_score
        + timing_score
        + fsm_score
        + contract_score)
        .min(100.0);

    println!("=== Quality Score ===");
    println!(
        "  signal_direction_coverage: {dir_pct}% (declared signals only, graph-first with compatibility fallback)"
    );
    println!("  graph_direction_coverage: {graph_dir_pct}% (declared signals only)");
    println!("  compatibility_direction_hints: {compat_dir_pct}% (declared signals only)");
    println!("  signal_width_coverage: {w_pct}% (declared signals only)");
    println!("  has_encoding_enums: {has_enums}");
    println!("  has_register_map: {has_registers}");
    println!("  has_timing_constraints: {has_timing}");
    println!("  has_state_machine: {has_states}");
    println!("  has_system_contract: {has_system_contract}");
    println!("  structured_nlp_constraints: {has_constraints}");
    println!("  residual_decisions: {}", ir.residual_decisions.len());
    println!();
    println!("  score_breakdown:");
    println!("    signal_direction:  {dir_score:.1}/25");
    println!("    signal_width:      {width_score:.1}/10");
    println!("    nlp_constraints:   {constraint_score:.0}/30");
    println!("    encoding_enums:    {enum_score:.0}/15");
    println!("    register_map:      {register_score:.0}/5");
    println!("    timing:            {timing_score:.0}/5");
    println!("    fsm_bonus:         {fsm_score:.0}/5");
    println!("    contract_bonus:    {contract_score:.0}/5");
    let grade = match score as u32 {
        90..=100 => "EXCELLENT",
        70..=89 => "GOOD",
        50..=69 => "ADEQUATE",
        30..=49 => "NEEDS IMPROVEMENT",
        _ => "INCOMPLETE",
    };
    println!("  overall_score: {score:.0}/100 — {grade}");
    println!();

    println!("=== Residual Decisions ===");
    println!("  count: {}", ir.residual_decisions.len());
    for rd in &ir.residual_decisions {
        println!("  - [{}] {}", rd.packet_id, rd.question);
    }

    let missing_producer_signals: Vec<String> = ir
        .signal_connectivity
        .iter()
        .filter(|record| record.producer_actor_ids.is_empty())
        .map(|record| record.signal_name.clone())
        .collect();
    let missing_consumer_signals: Vec<String> = ir
        .signal_connectivity
        .iter()
        .filter(|record| record.consumer_actor_ids.is_empty())
        .map(|record| record.signal_name.clone())
        .collect();
    let missing_graph_direction_count = declared_count.saturating_sub(with_graph_direction);

    let mut findings = Vec::new();
    if !ir.actor_signal_relations.is_empty() && ir.actor_ports.is_empty() {
        findings.push(finding(
            "intent_actor_ports_missing",
            ValidationFindingSeverity::Error,
            "knowledge_graph",
            "IntentIR carries actor-signal relations but no actor-relative ports",
            Vec::new(),
        ));
    }
    if !missing_producer_signals.is_empty() {
        findings.push(finding(
            "intent_connectivity_missing_producer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in IntentIR connectivity have no resolved producer actor",
                missing_producer_signals.len()
            ),
            missing_producer_signals.iter().take(8).cloned().collect(),
        ));
    }
    if !missing_consumer_signals.is_empty() {
        findings.push(finding(
            "intent_connectivity_missing_consumer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in IntentIR connectivity have no resolved consumer actor",
                missing_consumer_signals.len()
            ),
            missing_consumer_signals.iter().take(8).cloned().collect(),
        ));
    }
    if !ir.actor_ports.is_empty() && missing_graph_direction_count > 0 {
        findings.push(finding(
            "intent_graph_direction_coverage_incomplete",
            ValidationFindingSeverity::Info,
            "knowledge_graph",
            format!(
                "{missing_graph_direction_count} declared signal record(s) still lack graph-derived direction coverage"
            ),
            Vec::new(),
        ));
    }
    if !ir.actor_ports.is_empty() && with_compat_direction_hint < declared_count {
        findings.push(finding(
            "intent_compat_direction_hints_lag_graph",
            ValidationFindingSeverity::Info,
            "compatibility_surface",
            format!(
                "{} declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist",
                declared_count.saturating_sub(with_compat_direction_hint)
            ),
            Vec::new(),
        ));
    }
    if score < 90.0 {
        findings.push(finding(
            "intent_quality_below_excellent_threshold",
            ValidationFindingSeverity::Warning,
            "quality_score",
            format!("IntentIR quality score is {score:.0}/100 ({grade})"),
            Vec::new(),
        ));
    }
    if !ir.residual_decisions.is_empty() {
        findings.push(finding(
            "intent_residual_decisions_present",
            ValidationFindingSeverity::Warning,
            "residual_decisions",
            format!(
                "IntentIR still carries {} residual decision packet(s)",
                ir.residual_decisions.len()
            ),
            ir.residual_decisions
                .iter()
                .map(|packet| packet.packet_id.clone())
                .collect(),
        ));
    }

    let report = ValidationReportRecord {
        report_id: format!("validation_intent_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::IntentIr,
        artifact_fingerprint,
        summary: format!(
            "IntentIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: Some(score.round() as u32),
        grade: Some(grade.to_string()),
        metrics: vec![
            metric("declared_signal_records", declared_count.to_string()),
            metric("heuristic_signal_records", heuristic_signals.to_string()),
            metric("with_resolved_direction", with_direction.to_string()),
            metric("with_graph_direction", with_graph_direction.to_string()),
            metric(
                "with_compat_direction_hint",
                with_compat_direction_hint.to_string(),
            ),
            metric("with_width", with_width.to_string()),
            metric("actors", ir.actors.len().to_string()),
            metric(
                "actor_signal_relations",
                ir.actor_signal_relations.len().to_string(),
            ),
            metric("actor_ports", ir.actor_ports.len().to_string()),
            metric(
                "signal_connectivity",
                ir.signal_connectivity.len().to_string(),
            ),
            metric("behaviors", ir.behaviors.len().to_string()),
            metric("constraints", ir.constraints.len().to_string()),
            metric("assumptions", ir.assumptions.len().to_string()),
            metric(
                "symbol_definitions",
                ir.symbol_definitions.len().to_string(),
            ),
            metric("regular_states", ir.regular_states.len().to_string()),
            metric("state_transitions", ir.state_transitions.len().to_string()),
            metric("register_records", ir.register_records.len().to_string()),
            metric(
                "timing_constraints",
                ir.timing_constraints.len().to_string(),
            ),
            metric(
                "signal_constraints",
                ir.signal_constraints.len().to_string(),
            ),
            metric("conditional_rules", ir.conditional_rules.len().to_string()),
            metric(
                "residual_decisions",
                ir.residual_decisions.len().to_string(),
            ),
            metric("overall_score", format!("{score:.0}")),
            metric("grade", grade.to_string()),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn sorted_by_value<'a>(map: &'a HashMap<&str, usize>) -> Vec<(&'a &'a str, &'a usize)> {
    let mut pairs: Vec<_> = map.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    pairs
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::error::Result;
    use crate::ir::evidence::EvidenceIr;
    use crate::ir::intent::IntentIr;
    use crate::ir::semantic::SemanticIr;
    use crate::ir::source::SourceIr;

    #[test]
    fn validate_source_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Spec\nSome content.\n")?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        run(ValidateArgs {
            artifact: source_ir.artifact_layout.source_ir_path,
        })
    }

    #[test]
    fn validate_source_ir_backannotates_artifact_and_writes_sidecar() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Spec\nSome content.\n")?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let artifact_path = source_ir.artifact_layout.source_ir_path.clone();
        run(ValidateArgs {
            artifact: artifact_path.clone(),
        })?;

        let reloaded = SourceIr::load_from_path(&artifact_path)?;
        assert_eq!(reloaded.validation_reports.len(), 1);
        assert_eq!(
            reloaded.validation_reports[0].validated_stage,
            IrStage::SourceIr
        );
        assert!(
            validation_report_path_for(&artifact_path)?.exists(),
            "expected stage-local validation_report.json sidecar to exist"
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(&source, "# Spec\nSome content.\n")?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;

        run(ValidateArgs {
            artifact: evidence_ir.artifact_layout.evidence_ir_path,
        })
    }

    #[test]
    fn validate_semantic_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        // Use formal module syntax so SemanticIR has something to extract.
        fs::write(
            &source,
            "# Spec\n(?module: mymod (+system (clock clk) (sreset rstn)))\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        run(ValidateArgs {
            artifact: semantic_ir.artifact_layout.semantic_ir_path,
        })
    }

    #[test]
    fn validate_intent_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            "# Spec\n(?module: mymod (+system (clock clk) (sreset rstn)))\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        intent_ir.write_to_disk()?;

        run(ValidateArgs {
            artifact: intent_ir.artifact_layout.intent_ir_path,
        })
    }

    #[test]
    fn validate_intent_ir_backannotates_current_score_into_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            "# Spec\nSignal DATA_IN is input width 8.\n\nSignal DATA_OUT is output width 8.\n\nBlock pass: DATA_OUT = DATA_IN.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        intent_ir.write_to_disk()?;

        let artifact_path = intent_ir.artifact_layout.intent_ir_path.clone();
        run(ValidateArgs {
            artifact: artifact_path.clone(),
        })?;

        let reloaded = IntentIr::load_from_path(&artifact_path)?;
        assert_eq!(reloaded.validation_reports.len(), 1);
        let report = &reloaded.validation_reports[0];
        assert_eq!(report.validated_stage, IrStage::IntentIr);
        assert!(report.overall_score.is_some());
        assert!(report.grade.is_some());
        assert!(
            validation_report_path_for(&artifact_path)?.exists(),
            "expected stage-local validation_report.json sidecar to exist"
        );

        Ok(())
    }

    fn metric_value<'a>(report: &'a ValidationReportRecord, name: &str) -> Option<&'a str> {
        report
            .metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value.as_str())
    }

    #[test]
    fn validate_intent_ir_scores_direction_from_graph_before_compat_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PADDR is input width 32.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Requester reads PREADY.\n",
                "\n",
                "The Requester drives PADDR.\n",
                "\n",
                "The Completer samples PADDR.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let with_hints_report = validate_intent_ir(&intent_ir, "with_hints".to_string());

        let mut graph_only_intent = intent_ir.clone();
        for interface in &mut graph_only_intent.interfaces {
            for signal in &mut interface.signal_records {
                signal.direction_hint = None;
            }
        }
        let graph_only_report = validate_intent_ir(&graph_only_intent, "graph_only".to_string());

        assert_eq!(
            with_hints_report.overall_score, graph_only_report.overall_score,
            "graph-derived direction coverage should preserve the IntentIR score even when flat compatibility hints are absent"
        );
        assert_eq!(
            metric_value(&graph_only_report, "with_resolved_direction"),
            Some("2")
        );
        assert_eq!(
            metric_value(&graph_only_report, "with_graph_direction"),
            Some("2")
        );
        assert_eq!(
            metric_value(&graph_only_report, "with_compat_direction_hint"),
            Some("0")
        );

        Ok(())
    }
}
