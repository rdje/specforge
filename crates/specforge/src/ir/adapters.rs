use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::intent::{IntentDocumentIdentity, IntentIr};
use crate::ir::isf_ir::IsfIr;
use crate::ir::semantic::SymbolDefinitionKind;
use crate::ir::source::ResidualDecisionPacket;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterTarget {
    Isf,
}

impl AdapterTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Isf => "isf",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStatus {
    Implemented,
    Planned,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterPlan {
    pub target: AdapterTarget,
    pub required_input_stage: IrStage,
    pub status: AdapterStatus,
    pub notes: Vec<String>,
}

pub fn default_adapter_plans() -> Vec<AdapterPlan> {
    vec![
        AdapterPlan {
            target: AdapterTarget::Isf,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Implemented,
            notes: vec![
                "Intent Scheduling Format (.isf) adapter — emits high-level scheduling intent".to_string(),
                "FSMGen consumes .isf and owns scheduling, .fsm, and HDL downstream; SpecForge does not do cycle scheduling".to_string(),
            ],
        },
    ]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterLoweringStatus {
    Renderable,
    Blocked,
}

impl AdapterLoweringStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Renderable => "renderable",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterArtifact {
    pub stage: IrStage,
    pub schema_version: u32,
    pub target: AdapterTarget,
    pub required_input_stage: IrStage,
    pub intent_ir_path: PathBuf,
    pub artifact_layout: AdapterArtifactLayout,
    pub adapter_identity: AdapterIdentity,
    pub document_identity: IntentDocumentIdentity,
    pub lowering_status: AdapterLoweringStatus,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_reports: Vec<crate::ir::source::ValidationReportRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isf: Option<IsfAdapterArtifact>,
}

impl AdapterArtifact {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn build(
        intent_ir_path: &Path,
        target: AdapterTarget,
        artifact_base_root: &Path,
    ) -> Result<Self> {
        let intent_ir_path = canonicalize_existing_path(intent_ir_path)?;
        let raw_artifact = fs::read_to_string(&intent_ir_path)?;
        let stage_probe: StageProbe = serde_json::from_str(&raw_artifact)?;

        if !matches!(stage_probe.stage, IrStage::IntentIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an IntentIR document before building an adapter artifact",
                intent_ir_path.display()
            )));
        }

        let intent_ir: IntentIr = serde_json::from_str(&raw_artifact)?;

        match target {
            AdapterTarget::Isf => {
                build_isf_adapter_artifact(&intent_ir, &intent_ir_path, artifact_base_root)
            }
        }
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        fs::write(
            &self.artifact_layout.adapter_artifact_path,
            self.to_pretty_json()?,
        )?;

        if let (Some(path), Some(text)) = (
            self.artifact_layout.emitted_target_path.as_ref(),
            self.rendered_target_text(),
        ) {
            fs::write(path, text)?;
        }

        Ok(())
    }

    fn rendered_target_text(&self) -> Option<String> {
        match &self.isf {
            Some(isf) if isf.is_renderable => Some(isf.source_text.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterArtifactLayout {
    pub artifact_root: PathBuf,
    pub adapter_artifact_path: PathBuf,
    pub emitted_target_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterIdentity {
    pub adapter_id: String,
    pub summary: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IsfAdapterArtifact {
    pub actor_name: String,
    pub source_text: String,
    pub is_renderable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocking_reasons: Vec<String>,
    pub signal_count: usize,
    pub transaction_count: usize,
    pub rule_count: usize,
    pub constant_count: usize,
    pub enum_count: usize,
    pub storage_count: usize,
}
fn derive_isf_actor_name(intent_ir: &IntentIr) -> String {
    if let Some(actor) = intent_ir.actors.first()
        && let Some(name) = &actor.actor_name
        && !name.is_empty()
    {
        return name.replace([' ', '-', '.'], "_").to_lowercase();
    }
    intent_ir
        .document_identity
        .document_key
        .replace([' ', '-', '.'], "_")
        .to_lowercase()
}

fn count_isf_signals(intent_ir: &IntentIr) -> usize {
    let mut seen = BTreeSet::new();
    for iface in &intent_ir.interfaces {
        for sig in &iface.signal_records {
            seen.insert(&sig.signal_name);
        }
    }
    seen.len()
}

// ISF renderability policy (R6-ISF-ADAPTER.4 decision).
//
// ISF lowering blocks on exactly two conditions:
//   1. no signals declared in any interface, and
//   2. no behavioral content (temporal rules, conditional rules, signal
//      constraints, or control blocks).
//
// Missing per-signal direction or width is *deliberately not* a blocker:
// `IsfIr::from_intent_ir` defaults an unknown direction to `output` and an
// unknown width to `1`. This is intentional and is the key policy difference
// from the stricter `.fsm` adapter, which blocks on missing/conflicting
// direction or width. The rationale: FSMGen performs the cycle scheduling for
// `.isf`, and ISF accepts a default direction/width, so blocking on those
// would over-restrict otherwise-honest lowering without improving downstream
// correctness. The `.fsm` path stays strict because it must not fabricate
// target syntax; the `.isf` path can safely default and let FSMGen schedule.
fn assess_isf_renderability(intent_ir: &IntentIr) -> (bool, Vec<String>) {
    let mut reasons: Vec<String> = Vec::new();

    let signal_count = count_isf_signals(intent_ir);
    if signal_count == 0 {
        reasons.push("no signals declared in interface".to_string());
    } else {
        // Direction and width missing are informational only — the ISF IR
        // defaults them to output / width 1 so emission can proceed (see the
        // ISF renderability policy comment above).
    }

    let has_behavior = !intent_ir.temporal_rules.is_empty()
        || !intent_ir.conditional_rules.is_empty()
        || !intent_ir.signal_constraints.is_empty()
        || !intent_ir.control_blocks.is_empty();
    if !has_behavior {
        reasons.push(
            "no behavioral content (temporal rules, conditional rules, signal constraints, or control blocks)"
                .to_string(),
        );
    }

    (reasons.is_empty(), reasons)
}

fn build_isf_adapter_artifact(
    intent_ir: &IntentIr,
    intent_ir_path: &Path,
    artifact_base_root: &Path,
) -> Result<AdapterArtifact> {
    let artifact_root = artifact_base_root
        .join(AdapterTarget::Isf.as_str())
        .join(&intent_ir.document_identity.document_key);
    let adapter_artifact_path = artifact_root.join("adapter.json");

    let actor_name = derive_isf_actor_name(intent_ir);
    // Build the typed ISF model once so the emitted source text and the
    // temporal residual decisions come from the exact same lowering pass
    // (ISF-TEMPORAL-LOWERING.2.3 — the residual set must reflect what the
    // emitter actually did, not a re-derived guess).
    let isf_model = IsfIr::from_intent_ir(intent_ir, &actor_name);
    let source_text = isf_model.render();
    let (is_renderable, blocking_reasons) = assess_isf_renderability(intent_ir);

    let signal_count = count_isf_signals(intent_ir);
    // ISF-TEMPORAL-LOWERING.2.4: report what the emitter actually renders,
    // not a blind IntentIR-derived guess. The old
    // `transaction_count = temporal_rules.len() + cb_tx_count` and
    // `rule_count = conditional_rules + signal_constraints` counted surfaces
    // the emitter ignores (the entire temporal_rules set pre-.2.2/.2.3, and
    // control_blocks that may not all lower) and missed temporal-synthesized
    // transactions and temporal `(rule …)`. Derive both from the single ISF
    // model built above so the metric == emitted content.
    let transaction_count = isf_model.emitted_transaction_count();
    let rule_count = isf_model.emitted_rule_count();
    let constant_count = intent_ir
        .symbol_definitions
        .iter()
        .filter(|s| {
            matches!(
                s.kind,
                SymbolDefinitionKind::Constant
                    | SymbolDefinitionKind::Define
                    | SymbolDefinitionKind::Param
            )
        })
        .count();
    let enum_count = intent_ir
        .symbol_definitions
        .iter()
        .filter(|s| matches!(s.kind, SymbolDefinitionKind::Enum))
        .count();
    let storage_count = intent_ir.register_records.len();

    let emitted_target_path = if is_renderable {
        Some(artifact_root.join(format!("{}.isf", actor_name)))
    } else {
        None
    };

    let lowering_status = if is_renderable {
        AdapterLoweringStatus::Renderable
    } else {
        AdapterLoweringStatus::Blocked
    };

    let adapter_identity = AdapterIdentity {
        adapter_id: format!(
            "adapter_{}_{}",
            AdapterTarget::Isf.as_str(),
            intent_ir.document_identity.document_key
        ),
        summary: format!(
            ".isf lowering artifact for {}",
            intent_ir.document_identity.display_name
        ),
    };

    let isf = IsfAdapterArtifact {
        actor_name: actor_name.clone(),
        source_text,
        is_renderable,
        blocking_reasons,
        signal_count,
        transaction_count,
        rule_count,
        constant_count,
        enum_count,
        storage_count,
    };

    // Temporal rules with no representable supported ISF construct are
    // preserved as explicit residual decisions (ISF-TEMPORAL-LOWERING.2.3
    // mapping #4) so a dropped temporal obligation is visible in the
    // artifact rather than silently lost; syntax is never fabricated.
    let mut residual_decisions = intent_ir.residual_decisions.clone();
    residual_decisions.extend(isf_model.temporal_residuals().iter().cloned());

    Ok(AdapterArtifact {
        stage: IrStage::IsfAdapter,
        schema_version: 1,
        target: AdapterTarget::Isf,
        required_input_stage: IrStage::IntentIr,
        intent_ir_path: intent_ir_path.to_path_buf(),
        artifact_layout: AdapterArtifactLayout {
            artifact_root,
            adapter_artifact_path,
            emitted_target_path,
        },
        adapter_identity,
        document_identity: intent_ir.document_identity.clone(),
        lowering_status,
        residual_decisions,
        validation_reports: vec![],
        isf: Some(isf),
    })
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}
#[derive(Debug, Deserialize)]
struct StageProbe {
    stage: IrStage,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::adapters::{AdapterArtifact, AdapterTarget};
    use crate::ir::evidence::EvidenceIr;
    use crate::ir::intent::IntentIr;
    use crate::ir::semantic::SemanticIr;
    use crate::ir::source::SourceIr;

    fn build_intent_ir_from_markdown(
        base: &Path,
        source_name: &str,
        markdown: &str,
    ) -> Result<IntentIr> {
        let source = base.join(source_name);
        let source_artifact_base = base.join("generated").join("source_ir");
        let evidence_artifact_base = base.join("generated").join("evidence_ir");
        let semantic_artifact_base = base.join("generated").join("semantic_ir");
        let intent_artifact_base = base.join("generated").join("intent_ir");

        fs::write(&source, markdown)?;

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
        Ok(intent_ir)
    }

    // --- ISF adapter ---

    // Self-contained ISF spec: a renderable ISF needs >=1 signal plus
    // behavioral content. Built through the generic markdown -> IntentIR
    // pipeline so the ISF tests stay self-contained.
    const ISF_ADAPTER_TEST_SPEC: &str = concat!(
        "# ISF Spec\n",
        "Clock clk.\n\n",
        "Reset rst_n is asynchronous active low.\n\n",
        "Signal DATA_IN is input width 8.\n\n",
        "Signal GO is input width 1.\n\n",
        "Signal ACC is output width 8.\n\n",
        "Init ACC = 8'0.\n\n",
        "Block accumulate when GO: ACC <- DATA_IN.\n",
    );

    #[test]
    fn isf_adapter_emits_valid_s_expression_source() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir =
            build_intent_ir_from_markdown(tempdir.path(), "isf_spec.md", ISF_ADAPTER_TEST_SPEC)?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;

        let isf = artifact.isf.expect("ISF artifact must be populated");
        assert!(
            isf.is_renderable,
            "expected renderable ISF artifact: {:?}",
            isf.blocking_reasons
        );
        assert!(isf.blocking_reasons.is_empty());

        assert!(isf.signal_count > 0, "must carry signal inventory");
        assert!(
            isf.transaction_count > 0 || isf.rule_count > 0,
            "should have behavioral content"
        );

        let source = &isf.source_text;
        assert!(source.starts_with("(actor"), "must start with actor form");
        assert!(source.ends_with(")"), "must end with closing paren");
        assert!(source.contains("(clock "), "must declare clock");
        assert!(source.contains("(interface"), "must declare interface");
        assert!(
            source.contains("(input ") || source.contains("(output "),
            "must have signal directions"
        );
        assert!(
            source.contains("(transaction ") || source.contains("(rule "),
            "must have behavioral content"
        );

        assert!(artifact.artifact_layout.emitted_target_path.is_some());
        assert!(
            artifact
                .artifact_layout
                .emitted_target_path
                .unwrap()
                .to_string_lossy()
                .ends_with(".isf")
        );

        Ok(())
    }

    // ISF-TEMPORAL-LOWERING.2.4 regression: every reported behavioral count
    // MUST equal what the emitter actually rendered — never a blind
    // IntentIR-derived guess that counts a surface the emitter ignores.
    #[test]
    fn isf_adapter_counts_equal_emitted_content() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir =
            build_intent_ir_from_markdown(tempdir.path(), "isf_count.md", ISF_ADAPTER_TEST_SPEC)?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;
        let isf = artifact.isf.expect("ISF artifact must be populated");
        let src = &isf.source_text;

        // Top-level transaction/rule forms render at a 2-space indent
        // (`\n  (transaction `/`\n  (rule `); nested clauses are deeper.
        let emitted_txns = src.matches("\n  (transaction ").count();
        let emitted_rules = src.matches("\n  (rule ").count();

        assert_eq!(
            isf.transaction_count, emitted_txns,
            "transaction_count must equal emitted (transaction …) forms\n{src}"
        );
        assert_eq!(
            isf.rule_count, emitted_rules,
            "rule_count must equal emitted (rule …) forms\n{src}"
        );
        Ok(())
    }

    #[test]
    fn isf_adapter_blocks_when_no_signals() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(tempdir.path(), "empty.md", "# Empty\n")?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;

        let isf = artifact.isf.expect("ISF artifact must be populated");
        assert!(!isf.is_renderable);
        assert!(!isf.blocking_reasons.is_empty());
        assert!(artifact.artifact_layout.emitted_target_path.is_none());

        Ok(())
    }

    #[test]
    fn isf_output_passes_fsmgen_strict_validation() -> Result<()> {
        let tempdir = tempdir()?;

        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_strict_spec.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;

        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;

        let isf = artifact.isf.expect("ISF artifact must be populated");
        assert!(
            isf.is_renderable,
            "ISF must be renderable: {:?}",
            isf.blocking_reasons
        );

        let isf_path = tempdir.path().join("test_output.isf");
        fs::write(&isf_path, &isf.source_text)?;
        eprintln!("=== ISF source text ===\n{}\n=== END ===", isf.source_text);

        let output = crate::ir::run_fsmgen_strict_check(&isf_path);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let check_result: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!(
                "fsmgen did not produce valid JSON.\nstdout: {}\nstderr: {}\nparse error: {}",
                stdout, stderr, e
            );
        });

        let success = check_result["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);

        let diag_count = check_result["diagnostic_summary"]["diagnostic_count"]
            .as_i64()
            .unwrap_or(-1);

        if !success && let Some(diags) = check_result["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(no message)")
                );
            }
        }

        assert!(
            success,
            "FSMGen strict validation failed with {} diagnostics",
            diag_count
        );
        assert_eq!(
            diag_count, 0,
            "expected 0 FSMGen diagnostics, got {}",
            diag_count
        );

        Ok(())
    }

    // ISF-TEMPORAL-LOWERING.3 end-to-end regression: a temporal-rules-
    // bearing IntentIR produced through the generic markdown pipeline must
    // yield non-empty ISF temporal behavior, the artifact metric must match
    // the emitted content, and the emitted `.isf` must pass fsmgen strict.
    const ISF_TEMPORAL_E2E_SPEC: &str = concat!(
        "# Temporal Spec\n",
        "Clock clk.\n\n",
        "Reset rst_n is asynchronous active low.\n\n",
        "Signal PSEL is input width 1.\n\n",
        "Signal PREADY is output width 1.\n\n",
        "Signal CS_N is output width 1.\n\n",
        "PREADY must be asserted within 2 cycles.\n\n",
        "CS_N must be asserted.\n",
    );

    #[test]
    fn isf_temporal_rules_reach_isf_end_to_end() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_temporal_e2e.md",
            ISF_TEMPORAL_E2E_SPEC,
        )?;

        // The generic markdown pipeline must have recovered temporal rules
        // (this regression is meaningless otherwise).
        let loaded = IntentIr::load_from_path(&intent_ir.artifact_layout.intent_ir_path)?;
        assert!(
            !loaded.temporal_rules.is_empty(),
            "pipeline produced no temporal_rules — spec/regression is moot"
        );

        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;
        let isf = artifact
            .isf
            .clone()
            .expect("ISF artifact must be populated");
        let src = &isf.source_text;
        eprintln!(
            "=== temporal_rules={} ===\n{src}\n=== residuals={} ===",
            loaded.temporal_rules.len(),
            artifact.residual_decisions.len()
        );

        // Non-empty ISF temporal behavior: at least one temporal-derived
        // construct OR an explicit residual decision — never silent loss.
        let has_contract = src.contains("\n    (contract ");
        let has_temporal_rule = src.contains("(rule temporal_");
        let has_temporal_residual = artifact
            .residual_decisions
            .iter()
            .any(|p| p.packet_id.starts_with("isf_temporal_unrepresentable_"));
        assert!(
            has_contract || has_temporal_rule || has_temporal_residual,
            "temporal_rules were silently dropped (no contract, no temporal rule, no residual)\n{src}"
        );

        // Metric == emitted content (ISF-TEMPORAL-LOWERING.2.4 invariant
        // holds in the temporal path too).
        assert_eq!(
            isf.transaction_count,
            src.matches("\n  (transaction ").count(),
            "transaction_count must equal emitted transactions\n{src}"
        );
        assert_eq!(
            isf.rule_count,
            src.matches("\n  (rule ").count(),
            "rule_count must equal emitted rules\n{src}"
        );

        // Emitted `.isf` still passes fsmgen strict.
        let isf_path = tempdir.path().join("temporal_e2e.isf");
        fs::write(&isf_path, src)?;
        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        let success = check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);
        if !success && let Some(diags) = check["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(none)")
                );
            }
        }
        assert!(
            success,
            "emitted temporal `.isf` was rejected by fsmgen strict"
        );

        Ok(())
    }
}
