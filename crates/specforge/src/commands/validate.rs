use std::collections::HashMap;

use crate::cli::ValidateArgs;
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{EvidenceIr, StatementClass, VisualObservationKind};
use crate::ir::intent::IntentIr;
use crate::ir::semantic::SemanticIr;
use crate::ir::source::{DiagramKind, SourceIr};

pub fn run(args: ValidateArgs) -> Result<()> {
    // Auto-detect stage from artifact JSON `stage` field.
    let raw = std::fs::read_to_string(&args.artifact)
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
            let ir = SourceIr::load_from_path(&args.artifact)?;
            validate_source_ir(&ir);
        }
        IrStage::EvidenceIr => {
            let ir = EvidenceIr::load_from_path(&args.artifact)?;
            validate_evidence_ir(&ir);
        }
        IrStage::SemanticIr => {
            let ir = SemanticIr::load_from_path(&args.artifact)?;
            validate_semantic_ir(&ir);
        }
        IrStage::IntentIr => {
            let ir = IntentIr::load_from_path(&args.artifact)?;
            validate_intent_ir(&ir);
        }
    }

    Ok(())
}

fn validate_source_ir(ir: &SourceIr) {
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
}

fn validate_evidence_ir(ir: &EvidenceIr) {
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
}

fn validate_semantic_ir(ir: &SemanticIr) {
    println!("command: validate");
    println!("stage: semantic_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    println!("=== Interface / Signal Coverage ===");
    let total_signals: usize = ir.interfaces.iter().map(|i| i.signal_records.len()).sum();
    let with_direction: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| s.direction_hint.is_some())
        .count();
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
        .filter(|s| s.direction_hint.is_some() && s.width_hint.is_some())
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
    println!("  with_direction: {with_direction} ({dir_pct}%)");
    println!("  with_width: {with_width} ({w_pct}%)");
    println!("  fully_typed (direction+width): {fully_typed} ({ft_pct}%)");
    println!();

    println!("=== Semantic Records ===");
    println!("  actors: {}", ir.actors.len());
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
}

fn validate_intent_ir(ir: &IntentIr) {
    println!("command: validate");
    println!("stage: intent_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    println!("=== Signal Coverage ===");
    let total_signals: usize = ir.interfaces.iter().map(|i| i.signal_records.len()).sum();
    let with_direction: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| s.direction_hint.is_some())
        .count();
    let with_width: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| s.width_hint.is_some())
        .count();
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
    println!("  total_signal_records: {total_signals}");
    println!("  with_direction: {with_direction} ({dir_pct}%)");
    println!("  with_width: {with_width} ({w_pct}%)");
    println!();

    println!("=== Intent Records ===");
    println!("  actors: {}", ir.actors.len());
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

    println!("=== Quality Score ===");
    let sig_score = dir_pct.min(100);
    let has_states = !ir.regular_states.is_empty();
    let has_enums = !ir.symbol_definitions.is_empty();
    let has_constraints = ir.signal_constraints.len() + ir.conditional_rules.len();
    let has_system_contract = ir.system_contract.is_some();
    println!("  signal_direction_coverage: {dir_pct}%");
    println!("  signal_width_coverage: {w_pct}%");
    println!("  has_encoding_enums: {has_enums}");
    println!("  has_state_machine: {has_states}");
    println!("  has_system_contract: {has_system_contract}");
    println!("  structured_nlp_constraints: {has_constraints}");
    println!("  residual_decisions: {}", ir.residual_decisions.len());
    println!();

    let score = (sig_score as f64
        + if has_enums { 20.0 } else { 0.0 }
        + if has_states { 20.0 } else { 0.0 }
        + if has_system_contract { 10.0 } else { 0.0 }
        + (has_constraints.min(20) as f64))
        / 1.7;
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
}
