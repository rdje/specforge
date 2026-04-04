use std::path::PathBuf;

use serde::Serialize;

use crate::cli::{ConvergeArgs, EnrichArgs, NlpEnrichArgs, VlmProviderArg};
use crate::commands::{enrich, nlp_enrich};
use crate::error::{AppError, Result};
use crate::ir::adapters::{AdapterArtifact, AdapterLoweringStatus, AdapterTarget};
use crate::ir::evidence::{EvidenceIr, StatementClass, VisualObservationKind};
use crate::ir::intent::IntentIr;
use crate::ir::semantic::SemanticIr;
use crate::ir::source::{DiagramKind, SourceIr};

fn source_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("source_ir")
}

fn evidence_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("evidence_ir")
}

fn semantic_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("semantic_ir")
}

fn intent_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("intent_ir")
}

fn adapter_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("adapters")
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
    println!(
        "next_step_hint: run `specforge validate {}` for a stage-aware report",
        report.paths.intent_ir_path.display()
    );

    Ok(())
}

fn run_convergence(args: ConvergeArgs) -> Result<ConvergenceReport> {
    if args.max_iterations == 0 {
        return Err(AppError::InvalidStageArtifact(
            "converge requires --max-iterations >= 1".to_string(),
        ));
    }

    let target: AdapterTarget = args.target.into();
    let mut source_ir = SourceIr::build(&args.source, &source_artifact_base_root())?;
    source_ir.materialize()?;
    source_ir.write_to_disk()?;
    let paths = PipelineArtifactPaths::from_source_ir(&source_ir, target)?;

    println!("command: converge");
    println!("mode: execute");
    println!("source: {}", args.source.display());
    println!("document_key: {}", paths.document_key);
    println!("target: {}", target.as_str());
    println!("max_iterations: {}", args.max_iterations);
    println!("vlm_provider: {}", provider_name(args.vlm_provider));
    println!("nlp_provider: {}", provider_name(args.nlp_provider));

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

        let evidence_ir = EvidenceIr::build(&paths.source_ir_path, &evidence_artifact_base_root())?;
        evidence_ir.write_to_disk()?;
        println!(
            "evidence_fact_count: {}",
            evidence_ir.extracted_statements.len()
                + evidence_ir.signal_constraints.len()
                + evidence_ir.conditional_rules.len()
                + evidence_ir.actor_signal_relations.len()
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

        let semantic_ir =
            SemanticIr::build(&paths.evidence_ir_path, &semantic_artifact_base_root())?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(&paths.semantic_ir_path, &intent_artifact_base_root())?;
        intent_ir.write_to_disk()?;

        let adapter_artifact =
            AdapterArtifact::build(&paths.intent_ir_path, target, &adapter_artifact_base_root())?;
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
                return Ok(ConvergenceReport {
                    converged: true,
                    passes_run: pass,
                    final_snapshot: snapshot,
                    paths,
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
    fn from_source_ir(source_ir: &SourceIr, target: AdapterTarget) -> Result<Self> {
        let document_key = source_ir.document_identity.document_key.clone();
        Self {
            source_ir_path: absolute_artifact_path(&source_ir.artifact_layout.source_ir_path)?,
            evidence_ir_path: evidence_artifact_base_root()
                .join(&document_key)
                .join("evidence_ir.json"),
            semantic_ir_path: semantic_artifact_base_root()
                .join(&document_key)
                .join("semantic_ir.json"),
            intent_ir_path: intent_artifact_base_root()
                .join(&document_key)
                .join("intent_ir.json"),
            adapter_artifact_path: adapter_artifact_base_root()
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
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

#[derive(Debug, Clone)]
struct ConvergenceReport {
    converged: bool,
    passes_run: usize,
    final_snapshot: KnowledgeSnapshot,
    paths: PipelineArtifactPaths,
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
    init_assignments: usize,
    regular_states: usize,
    state_transitions: usize,
    decision_tree_fragments: usize,
    symbol_definitions: usize,
    control_blocks: usize,
    explicit_modules: usize,
    explicit_tops: usize,
    register_records: usize,
    timing_constraints: usize,
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
            init_assignments: ir.init_assignments.len(),
            regular_states: ir.regular_states.len(),
            state_transitions: ir.state_transitions.len(),
            decision_tree_fragments: ir.decision_tree_fragments.len(),
            symbol_definitions: ir.symbol_definitions.len(),
            control_blocks: ir.control_blocks.len(),
            explicit_modules: ir.explicit_modules.len(),
            explicit_tops: ir.explicit_tops.len(),
            register_records: ir.register_records.len(),
            timing_constraints: ir.timing_constraints.len(),
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
            + self.init_assignments
            + self.regular_states
            + self.state_transitions
            + self.decision_tree_fragments
            + self.symbol_definitions
            + self.control_blocks
            + self.explicit_modules
            + self.explicit_tops
            + self.register_records
            + self.timing_constraints
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
    init_assignments: usize,
    regular_states: usize,
    state_transitions: usize,
    decision_tree_fragments: usize,
    symbol_definitions: usize,
    control_blocks: usize,
    explicit_modules: usize,
    explicit_tops: usize,
    register_records: usize,
    timing_constraints: usize,
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
            init_assignments: ir.init_assignments.len(),
            regular_states: ir.regular_states.len(),
            state_transitions: ir.state_transitions.len(),
            decision_tree_fragments: ir.decision_tree_fragments.len(),
            symbol_definitions: ir.symbol_definitions.len(),
            control_blocks: ir.control_blocks.len(),
            explicit_modules: ir.explicit_modules.len(),
            explicit_tops: ir.explicit_tops.len(),
            register_records: ir.register_records.len(),
            timing_constraints: ir.timing_constraints.len(),
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
            + self.init_assignments
            + self.regular_states
            + self.state_transitions
            + self.decision_tree_fragments
            + self.symbol_definitions
            + self.control_blocks
            + self.explicit_modules
            + self.explicit_tops
            + self.register_records
            + self.timing_constraints
            + self.signal_constraints
            + self.conditional_rules
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct AdapterSnapshot {
    lowering_status: AdapterLoweringStatus,
    residual_decisions: usize,
    signal_inventory: usize,
    decision_tree_candidates: usize,
    state_candidates: usize,
    transition_candidates: usize,
    module_candidates: usize,
    top_candidates: usize,
    renderable: bool,
}

impl AdapterSnapshot {
    fn from_artifact(artifact: &AdapterArtifact) -> Self {
        let (
            signal_inventory,
            decision_tree_candidates,
            state_candidates,
            transition_candidates,
            module_candidates,
            top_candidates,
            renderable,
        ) = if let Some(fsm) = artifact.fsm.as_ref() {
            (
                fsm.signal_inventory.len(),
                fsm.decision_tree_candidates.len(),
                fsm.state_candidates.len(),
                fsm.transition_candidates.len(),
                fsm.module_candidates.len(),
                fsm.top_candidates.len(),
                fsm.renderability.is_renderable,
            )
        } else {
            (0, 0, 0, 0, 0, 0, false)
        };

        Self {
            lowering_status: artifact.lowering_status,
            residual_decisions: artifact.residual_decisions.len(),
            signal_inventory,
            decision_tree_candidates,
            state_candidates,
            transition_candidates,
            module_candidates,
            top_candidates,
            renderable,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::cli::AdapterTargetArg;
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

        let cwd_before = std::env::current_dir()?;
        std::env::set_current_dir(tempdir.path())?;

        let result = run_convergence(ConvergeArgs {
            source: source.clone(),
            target: AdapterTargetArg::Fsm,
            max_iterations: 4,
            vlm_provider: VlmProviderArg::Skip,
            vlm_model: None,
            nlp_provider: VlmProviderArg::Ollama,
            nlp_model: Some("mock".to_string()),
            nlp_max_sentences: 0,
        });

        std::env::set_current_dir(cwd_before)?;
        unsafe { std::env::remove_var("SPECFORGE_VLM_HELPER") };

        let report = result?;
        assert!(report.converged);
        assert_eq!(report.passes_run, 2);

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
}
