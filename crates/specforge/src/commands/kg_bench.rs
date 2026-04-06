use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use tempfile::tempdir;

use crate::cli::{KgBenchArgs, ValidateArgs};
use crate::commands::validate;
use crate::error::{AppError, Result};
use crate::ir::evidence::EvidenceIr;
use crate::ir::intent::{IntentAssumption, IntentIr};
use crate::ir::semantic::{
    ActorPortRecord, ActorRelativeDirection, InterfaceRecord, SemanticIr, TemporalRuleRecord,
};
use crate::ir::source::{ResidualDecisionPacket, ValidationFindingRecord, ValidationReportRecord};
use crate::ir::{intent, semantic, source};

const FIXTURE_FILE_NAME: &str = "fixture.json";

#[derive(Debug, Deserialize)]
struct KgBenchFixture {
    name: String,
    source: PathBuf,
    #[serde(default)]
    source_ir_patch: Option<SourceIrPatch>,
    #[serde(default)]
    evidence_ir_patch: Option<EvidenceIrPatch>,
    #[serde(default)]
    expectations: FixtureExpectations,
}

#[derive(Debug, Default, Deserialize)]
struct SourceIrPatch {
    #[serde(default)]
    structured_tables: Vec<crate::ir::source::StructuredTableRecord>,
    #[serde(default)]
    visual_assets: Vec<crate::ir::source::VisualAsset>,
}

#[derive(Debug, Default, Deserialize)]
struct EvidenceIrPatch {
    #[serde(default)]
    signal_alias_map: BTreeMap<String, String>,
    #[serde(default)]
    refresh_signal_semantic_hints: bool,
    #[serde(default)]
    signal_constraints: Vec<crate::ir::source::SignalConstraintRecord>,
}

#[derive(Debug, Default, Deserialize)]
struct FixtureExpectations {
    #[serde(default)]
    semantic: Option<CanonicalStageExpectations>,
    #[serde(default)]
    intent: Option<CanonicalStageExpectations>,
    #[serde(default)]
    validation: Option<ValidationExpectations>,
}

#[derive(Debug, Default, Deserialize)]
struct CanonicalStageExpectations {
    #[serde(default)]
    signal_names_include: Vec<String>,
    #[serde(default)]
    actor_ports_include: Vec<ExpectedActorPort>,
    #[serde(default)]
    residual_decision_ids_include: Vec<String>,
    #[serde(default)]
    residual_decision_ids_exclude: Vec<String>,
    #[serde(default)]
    assumption_ids_include: Vec<String>,
    #[serde(default)]
    assumption_ids_exclude: Vec<String>,
    #[serde(default)]
    resolved_semantic_role_signal_names_include: Vec<String>,
    #[serde(default)]
    resolved_semantic_role_signal_names_exclude: Vec<String>,
    #[serde(default)]
    semantic_consensus_signal_names_include: Vec<String>,
    #[serde(default)]
    semantic_consensus_signal_names_exclude: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_consensus_signal_names_include: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_consensus_signal_names_exclude: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_candidate_signal_names_include: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_candidate_signal_names_exclude: Vec<String>,
    #[serde(default)]
    semantic_candidate_signal_names_include: Vec<String>,
    #[serde(default)]
    semantic_candidate_signal_names_exclude: Vec<String>,
    #[serde(default)]
    multiple_semantic_candidate_signal_names_include: Vec<String>,
    #[serde(default)]
    multiple_semantic_candidate_signal_names_exclude: Vec<String>,
    #[serde(default)]
    semantic_arbitration_signal_names_include: Vec<String>,
    #[serde(default)]
    semantic_arbitration_signal_names_exclude: Vec<String>,
    #[serde(default)]
    decisive_semantic_arbitration_signal_names_include: Vec<String>,
    #[serde(default)]
    decisive_semantic_arbitration_signal_names_exclude: Vec<String>,
    #[serde(default)]
    non_decisive_semantic_arbitration_signal_names_include: Vec<String>,
    #[serde(default)]
    non_decisive_semantic_arbitration_signal_names_exclude: Vec<String>,
    temporal_rule_count: Option<usize>,
    temporal_rules_with_handshake_completion: Option<usize>,
    temporal_rules_with_alias_dependent_handshake_completion: Option<usize>,
    signal_polarity_conflicts: Option<usize>,
    signal_semantic_conflicts: Option<usize>,
    signal_connectivity_conflicts: Option<usize>,
    interface_signal_conflicts: Option<usize>,
    temporal_conflicts: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct ExpectedActorPort {
    actor_name: String,
    signal_name: String,
    direction: ActorRelativeDirection,
}

#[derive(Debug, Default, Deserialize)]
struct ValidationExpectations {
    #[serde(default)]
    evidence: Option<ValidationStageExpectations>,
    #[serde(default)]
    semantic: Option<ValidationStageExpectations>,
    #[serde(default)]
    intent: Option<ValidationStageExpectations>,
}

#[derive(Debug, Default, Deserialize)]
struct ValidationStageExpectations {
    #[serde(default)]
    finding_ids_include: Vec<String>,
    #[serde(default)]
    finding_ids_exclude: Vec<String>,
    #[serde(default)]
    metric_values: BTreeMap<String, String>,
}

#[derive(Debug)]
struct FixtureOutcome {
    name: String,
    fixture_path: PathBuf,
    failures: Vec<String>,
}

pub fn run(args: KgBenchArgs) -> Result<()> {
    let fixtures_root = canonicalize_existing_path(&args.fixtures_root)?;
    let fixture_paths = resolve_fixture_paths(&fixtures_root, &args.fixtures)?;

    println!("command: kg-bench");
    println!("fixtures_root: {}", fixtures_root.display());
    println!("fixtures_requested: {}", fixture_paths.len());

    let mut outcomes = Vec::new();
    for fixture_path in fixture_paths {
        let outcome = run_fixture(&fixture_path)?;
        println!(
            "fixture: {} [{}]",
            outcome.name,
            if outcome.failures.is_empty() {
                "pass"
            } else {
                "fail"
            }
        );
        for failure in &outcome.failures {
            println!("  - {failure}");
        }
        outcomes.push(outcome);
    }

    let passed = outcomes
        .iter()
        .filter(|outcome| outcome.failures.is_empty())
        .count();
    let failed = outcomes.len().saturating_sub(passed);
    println!("fixtures_passed: {passed}");
    println!("fixtures_failed: {failed}");

    if failed > 0 {
        let mut lines = vec!["kg-bench detected fixture failures:".to_string()];
        for outcome in outcomes
            .iter()
            .filter(|outcome| !outcome.failures.is_empty())
        {
            lines.push(format!(
                "- {} ({})",
                outcome.name,
                outcome.fixture_path.display()
            ));
            for failure in &outcome.failures {
                lines.push(format!("  - {failure}"));
            }
        }
        return Err(AppError::InvalidStageArtifact(lines.join("\n")));
    }

    Ok(())
}

fn run_fixture(fixture_path: &Path) -> Result<FixtureOutcome> {
    let fixture = load_fixture(fixture_path)?;
    let fixture_dir = fixture_path.parent().ok_or_else(|| {
        AppError::InvalidStageArtifact(format!(
            "fixture path {} has no parent directory",
            fixture_path.display()
        ))
    })?;
    let source_path = normalize_input_path(&fixture.source, fixture_dir);
    if !source_path.exists() {
        return Err(AppError::MissingPath(source_path));
    }

    let tempdir = tempdir()?;
    let generated_root = tempdir.path().join("generated");
    let source_ir_root = generated_root.join("source_ir");
    let evidence_ir_root = generated_root.join("evidence_ir");
    let semantic_ir_root = generated_root.join("semantic_ir");
    let intent_ir_root = generated_root.join("intent_ir");

    let mut source_ir = source::SourceIr::build(&source_path, &source_ir_root)?;
    source_ir.materialize()?;
    if let Some(patch) = fixture.source_ir_patch.as_ref() {
        source_ir
            .structured_tables
            .extend(patch.structured_tables.iter().cloned());
        source_ir
            .visual_assets
            .extend(patch.visual_assets.iter().cloned());
    }
    source_ir.write_to_disk()?;
    let mut evidence_ir =
        EvidenceIr::build(&source_ir.artifact_layout.source_ir_path, &evidence_ir_root)?;
    if let Some(patch) = fixture.evidence_ir_patch.as_ref() {
        evidence_ir.signal_alias_map.extend(
            patch
                .signal_alias_map
                .iter()
                .map(|(alias, signal_name)| (alias.clone(), signal_name.clone())),
        );
        if patch.refresh_signal_semantic_hints || !patch.signal_alias_map.is_empty() {
            evidence_ir.refresh_signal_semantic_hints()?;
        }
        evidence_ir
            .signal_constraints
            .extend(patch.signal_constraints.iter().cloned());
    }
    evidence_ir.write_to_disk()?;
    let evidence_report = if fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.evidence.as_ref())
        .is_some()
    {
        validate::run(ValidateArgs {
            artifact: evidence_ir.artifact_layout.evidence_ir_path.clone(),
        })?;
        let reloaded = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        Some(latest_validation_report(
            &reloaded.validation_reports,
            "EvidenceIR",
            &evidence_ir.artifact_layout.evidence_ir_path,
        )?)
    } else {
        None
    };
    let semantic_ir = semantic::SemanticIr::build(
        &evidence_ir.artifact_layout.evidence_ir_path,
        &semantic_ir_root,
    )?;
    semantic_ir.write_to_disk()?;
    let intent_ir = intent::IntentIr::build(
        &semantic_ir.artifact_layout.semantic_ir_path,
        &intent_ir_root,
    )?;
    intent_ir.write_to_disk()?;

    let semantic_report = if fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.semantic.as_ref())
        .is_some()
    {
        validate::run(ValidateArgs {
            artifact: semantic_ir.artifact_layout.semantic_ir_path.clone(),
        })?;
        let reloaded = SemanticIr::load_from_path(&semantic_ir.artifact_layout.semantic_ir_path)?;
        Some(latest_validation_report(
            &reloaded.validation_reports,
            "SemanticIR",
            &semantic_ir.artifact_layout.semantic_ir_path,
        )?)
    } else {
        None
    };
    let intent_report = if fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.intent.as_ref())
        .is_some()
    {
        validate::run(ValidateArgs {
            artifact: intent_ir.artifact_layout.intent_ir_path.clone(),
        })?;
        let reloaded = IntentIr::load_from_path(&intent_ir.artifact_layout.intent_ir_path)?;
        Some(latest_validation_report(
            &reloaded.validation_reports,
            "IntentIR",
            &intent_ir.artifact_layout.intent_ir_path,
        )?)
    } else {
        None
    };

    let mut failures = Vec::new();
    if let Some(expectations) = fixture.expectations.semantic.as_ref() {
        evaluate_canonical_expectations(
            "semantic",
            expectations,
            &semantic_ir.interfaces,
            &semantic_ir.actor_ports,
            &semantic_ir.residual_decisions,
            &[],
            &semantic_ir.temporal_rules,
            semantic_ir.signal_polarity_conflicts.len(),
            semantic_ir.signal_semantic_conflicts.len(),
            semantic_ir.signal_connectivity_conflicts.len(),
            semantic_ir.interface_signal_conflicts.len(),
            semantic_ir.temporal_conflicts.len(),
            &mut failures,
        );
    }
    if let Some(expectations) = fixture.expectations.intent.as_ref() {
        evaluate_canonical_expectations(
            "intent",
            expectations,
            &intent_ir.interfaces,
            &intent_ir.actor_ports,
            &intent_ir.residual_decisions,
            &intent_ir.assumptions,
            &intent_ir.temporal_rules,
            intent_ir.signal_polarity_conflicts.len(),
            intent_ir.signal_semantic_conflicts.len(),
            intent_ir.signal_connectivity_conflicts.len(),
            intent_ir.interface_signal_conflicts.len(),
            intent_ir.temporal_conflicts.len(),
            &mut failures,
        );
    }
    if let Some(expectations) = fixture.expectations.validation.as_ref() {
        if let Some(stage) = expectations.evidence.as_ref() {
            evaluate_validation_expectations(
                "evidence_validation",
                stage,
                evidence_report
                    .as_ref()
                    .expect("evidence report should exist"),
                &mut failures,
            );
        }
        if let Some(stage) = expectations.semantic.as_ref() {
            evaluate_validation_expectations(
                "semantic_validation",
                stage,
                semantic_report
                    .as_ref()
                    .expect("semantic report should exist"),
                &mut failures,
            );
        }
        if let Some(stage) = expectations.intent.as_ref() {
            evaluate_validation_expectations(
                "intent_validation",
                stage,
                intent_report.as_ref().expect("intent report should exist"),
                &mut failures,
            );
        }
    }

    Ok(FixtureOutcome {
        name: fixture.name,
        fixture_path: fixture_path.to_path_buf(),
        failures,
    })
}

fn evaluate_canonical_expectations(
    label: &str,
    expectations: &CanonicalStageExpectations,
    interfaces: &[InterfaceRecord],
    actor_ports: &[ActorPortRecord],
    residual_decisions: &[ResidualDecisionPacket],
    assumptions: &[IntentAssumption],
    temporal_rules: &[TemporalRuleRecord],
    signal_polarity_conflicts: usize,
    signal_semantic_conflicts: usize,
    signal_connectivity_conflicts: usize,
    interface_signal_conflicts: usize,
    temporal_conflicts: usize,
    failures: &mut Vec<String>,
) {
    let signal_names = interface_signal_names(interfaces);
    assert_includes(
        label,
        "signal_names_include",
        &expectations.signal_names_include,
        &signal_names,
        failures,
    );

    let resolved_role_signals = interface_signal_names_matching(interfaces, |signal| {
        signal.resolved_semantic_role.is_some()
    });
    assert_includes(
        label,
        "resolved_semantic_role_signal_names_include",
        &expectations.resolved_semantic_role_signal_names_include,
        &resolved_role_signals,
        failures,
    );
    assert_excludes(
        label,
        "resolved_semantic_role_signal_names_exclude",
        &expectations.resolved_semantic_role_signal_names_exclude,
        &resolved_role_signals,
        failures,
    );

    let semantic_consensus_signals =
        interface_signal_names_matching(interfaces, |signal| signal.semantic_consensus.is_some());
    assert_includes(
        label,
        "semantic_consensus_signal_names_include",
        &expectations.semantic_consensus_signal_names_include,
        &semantic_consensus_signals,
        failures,
    );
    assert_excludes(
        label,
        "semantic_consensus_signal_names_exclude",
        &expectations.semantic_consensus_signal_names_exclude,
        &semantic_consensus_signals,
        failures,
    );

    let alias_dependent_semantic_consensus_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_consensus
                .as_ref()
                .is_some_and(|consensus| consensus.alias_dependent)
        });
    assert_includes(
        label,
        "alias_dependent_semantic_consensus_signal_names_include",
        &expectations.alias_dependent_semantic_consensus_signal_names_include,
        &alias_dependent_semantic_consensus_signals,
        failures,
    );
    assert_excludes(
        label,
        "alias_dependent_semantic_consensus_signal_names_exclude",
        &expectations.alias_dependent_semantic_consensus_signal_names_exclude,
        &alias_dependent_semantic_consensus_signals,
        failures,
    );

    let alias_dependent_semantic_candidate_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_candidates
                .iter()
                .any(|candidate| candidate.alias_dependent)
        });
    assert_includes(
        label,
        "alias_dependent_semantic_candidate_signal_names_include",
        &expectations.alias_dependent_semantic_candidate_signal_names_include,
        &alias_dependent_semantic_candidate_signals,
        failures,
    );
    assert_excludes(
        label,
        "alias_dependent_semantic_candidate_signal_names_exclude",
        &expectations.alias_dependent_semantic_candidate_signal_names_exclude,
        &alias_dependent_semantic_candidate_signals,
        failures,
    );

    let semantic_candidate_signals = interface_signal_names_matching(interfaces, |signal| {
        !signal.semantic_candidates.is_empty()
    });
    assert_includes(
        label,
        "semantic_candidate_signal_names_include",
        &expectations.semantic_candidate_signal_names_include,
        &semantic_candidate_signals,
        failures,
    );
    assert_excludes(
        label,
        "semantic_candidate_signal_names_exclude",
        &expectations.semantic_candidate_signal_names_exclude,
        &semantic_candidate_signals,
        failures,
    );

    let multiple_semantic_candidate_signals =
        interface_signal_names_matching(interfaces, |signal| signal.semantic_candidates.len() > 1);
    assert_includes(
        label,
        "multiple_semantic_candidate_signal_names_include",
        &expectations.multiple_semantic_candidate_signal_names_include,
        &multiple_semantic_candidate_signals,
        failures,
    );
    assert_excludes(
        label,
        "multiple_semantic_candidate_signal_names_exclude",
        &expectations.multiple_semantic_candidate_signal_names_exclude,
        &multiple_semantic_candidate_signals,
        failures,
    );

    let semantic_arbitration_signals =
        interface_signal_names_matching(interfaces, |signal| signal.semantic_arbitration.is_some());
    assert_includes(
        label,
        "semantic_arbitration_signal_names_include",
        &expectations.semantic_arbitration_signal_names_include,
        &semantic_arbitration_signals,
        failures,
    );
    assert_excludes(
        label,
        "semantic_arbitration_signal_names_exclude",
        &expectations.semantic_arbitration_signal_names_exclude,
        &semantic_arbitration_signals,
        failures,
    );

    let decisive_semantic_arbitration_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| arbitration.decisive)
        });
    assert_includes(
        label,
        "decisive_semantic_arbitration_signal_names_include",
        &expectations.decisive_semantic_arbitration_signal_names_include,
        &decisive_semantic_arbitration_signals,
        failures,
    );
    assert_excludes(
        label,
        "decisive_semantic_arbitration_signal_names_exclude",
        &expectations.decisive_semantic_arbitration_signal_names_exclude,
        &decisive_semantic_arbitration_signals,
        failures,
    );

    let non_decisive_semantic_arbitration_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| !arbitration.decisive)
        });
    assert_includes(
        label,
        "non_decisive_semantic_arbitration_signal_names_include",
        &expectations.non_decisive_semantic_arbitration_signal_names_include,
        &non_decisive_semantic_arbitration_signals,
        failures,
    );
    assert_excludes(
        label,
        "non_decisive_semantic_arbitration_signal_names_exclude",
        &expectations.non_decisive_semantic_arbitration_signal_names_exclude,
        &non_decisive_semantic_arbitration_signals,
        failures,
    );

    for expected_port in &expectations.actor_ports_include {
        let found = actor_ports.iter().any(|port| {
            port.actor_name == expected_port.actor_name
                && port.signal_name == expected_port.signal_name
                && port.direction == expected_port.direction
        });
        if !found {
            failures.push(format!(
                "{label}: missing actor port `{}`:`{}`:`{}`",
                expected_port.actor_name,
                expected_port.signal_name,
                actor_relative_direction_label(expected_port.direction)
            ));
        }
    }

    let residual_ids = residual_decision_ids(residual_decisions);
    assert_includes(
        label,
        "residual_decision_ids_include",
        &expectations.residual_decision_ids_include,
        &residual_ids,
        failures,
    );
    assert_excludes(
        label,
        "residual_decision_ids_exclude",
        &expectations.residual_decision_ids_exclude,
        &residual_ids,
        failures,
    );

    let assumption_ids = assumption_id_set(assumptions);
    assert_includes(
        label,
        "assumption_ids_include",
        &expectations.assumption_ids_include,
        &assumption_ids,
        failures,
    );
    assert_excludes(
        label,
        "assumption_ids_exclude",
        &expectations.assumption_ids_exclude,
        &assumption_ids,
        failures,
    );

    assert_optional_count(
        label,
        "temporal_rule_count",
        expectations.temporal_rule_count,
        temporal_rules.len(),
        failures,
    );
    assert_optional_count(
        label,
        "temporal_rules_with_handshake_completion",
        expectations.temporal_rules_with_handshake_completion,
        temporal_rules_with_handshake_completion_count(temporal_rules),
        failures,
    );
    assert_optional_count(
        label,
        "temporal_rules_with_alias_dependent_handshake_completion",
        expectations.temporal_rules_with_alias_dependent_handshake_completion,
        temporal_rules_with_alias_dependent_handshake_completion_count(interfaces, temporal_rules),
        failures,
    );
    assert_optional_count(
        label,
        "signal_polarity_conflicts",
        expectations.signal_polarity_conflicts,
        signal_polarity_conflicts,
        failures,
    );
    assert_optional_count(
        label,
        "signal_semantic_conflicts",
        expectations.signal_semantic_conflicts,
        signal_semantic_conflicts,
        failures,
    );
    assert_optional_count(
        label,
        "signal_connectivity_conflicts",
        expectations.signal_connectivity_conflicts,
        signal_connectivity_conflicts,
        failures,
    );
    assert_optional_count(
        label,
        "interface_signal_conflicts",
        expectations.interface_signal_conflicts,
        interface_signal_conflicts,
        failures,
    );
    assert_optional_count(
        label,
        "temporal_conflicts",
        expectations.temporal_conflicts,
        temporal_conflicts,
        failures,
    );
}

fn evaluate_validation_expectations(
    label: &str,
    expectations: &ValidationStageExpectations,
    report: &ValidationReportRecord,
    failures: &mut Vec<String>,
) {
    let finding_ids = validation_finding_ids(&report.findings);
    assert_includes(
        label,
        "finding_ids_include",
        &expectations.finding_ids_include,
        &finding_ids,
        failures,
    );
    assert_excludes(
        label,
        "finding_ids_exclude",
        &expectations.finding_ids_exclude,
        &finding_ids,
        failures,
    );

    for (metric_name, expected_value) in &expectations.metric_values {
        let actual_value = validation_metric_value(report, metric_name);
        match actual_value {
            Some(actual_value) if actual_value == expected_value => {}
            Some(actual_value) => failures.push(format!(
                "{label}: expected metric `{metric_name}` to equal `{expected_value}`, but actual value was `{actual_value}`"
            )),
            None => failures.push(format!(
                "{label}: expected metric `{metric_name}` to equal `{expected_value}`, but the metric was absent"
            )),
        }
    }
}

fn load_fixture(path: &Path) -> Result<KgBenchFixture> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

fn resolve_fixture_paths(fixtures_root: &Path, requested: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut paths = if requested.is_empty() {
        discover_fixture_paths(fixtures_root)?
    } else {
        requested
            .iter()
            .map(|path| normalize_fixture_path(path, fixtures_root))
            .collect::<Vec<_>>()
    };

    if paths.is_empty() {
        return Err(AppError::InvalidStageArtifact(format!(
            "no KG benchmark fixtures found under {}",
            fixtures_root.display()
        )));
    }

    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn discover_fixture_paths(root: &Path) -> Result<Vec<PathBuf>> {
    let mut fixtures = Vec::new();
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            let fixture = path.join(FIXTURE_FILE_NAME);
            if fixture.exists() {
                fixtures.push(fixture);
                continue;
            }
            fixtures.extend(discover_fixture_paths(&path)?);
        } else if path.file_name().and_then(|name| name.to_str()) == Some(FIXTURE_FILE_NAME) {
            fixtures.push(path);
        }
    }
    Ok(fixtures)
}

fn normalize_fixture_path(path: &Path, fixtures_root: &Path) -> PathBuf {
    let absolute = normalize_input_path(path, fixtures_root);
    if absolute.is_dir() {
        absolute.join(FIXTURE_FILE_NAME)
    } else {
        absolute
    }
}

fn latest_validation_report(
    reports: &[ValidationReportRecord],
    stage_label: &str,
    artifact_path: &Path,
) -> Result<ValidationReportRecord> {
    reports.first().cloned().ok_or_else(|| {
        AppError::InvalidStageArtifact(format!(
            "{stage_label} artifact at {} does not carry a persisted validation report",
            artifact_path.display()
        ))
    })
}

fn interface_signal_names(interfaces: &[InterfaceRecord]) -> BTreeSet<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .map(|signal| signal.signal_name.clone())
        .collect()
}

fn interface_signal_names_matching(
    interfaces: &[InterfaceRecord],
    predicate: impl Fn(&crate::ir::semantic::InterfaceSignalRecord) -> bool,
) -> BTreeSet<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| predicate(signal))
        .map(|signal| signal.signal_name.clone())
        .collect()
}

fn residual_decision_ids(residual_decisions: &[ResidualDecisionPacket]) -> BTreeSet<String> {
    residual_decisions
        .iter()
        .map(|packet| packet.packet_id.clone())
        .collect()
}

fn assumption_id_set(assumptions: &[IntentAssumption]) -> BTreeSet<String> {
    assumptions
        .iter()
        .map(|assumption| assumption.assumption_id.clone())
        .collect()
}

fn validation_finding_ids(findings: &[ValidationFindingRecord]) -> BTreeSet<String> {
    findings
        .iter()
        .map(|finding| finding.finding_id.clone())
        .collect()
}

fn validation_metric_value<'a>(
    report: &'a ValidationReportRecord,
    metric_name: &str,
) -> Option<&'a str> {
    report
        .metrics
        .iter()
        .find(|metric| metric.name == metric_name)
        .map(|metric| metric.value.as_str())
}

fn temporal_rules_with_handshake_completion_count(temporal_rules: &[TemporalRuleRecord]) -> usize {
    temporal_rules
        .iter()
        .filter(|rule| {
            rule.antecedents
                .iter()
                .chain(rule.consequents.iter())
                .any(|predicate| {
                    matches!(
                        predicate,
                        crate::ir::semantic::TemporalPredicateRecord::HandshakeComplete { .. }
                    )
                })
        })
        .count()
}

fn temporal_rules_with_alias_dependent_handshake_completion_count(
    interfaces: &[InterfaceRecord],
    temporal_rules: &[TemporalRuleRecord],
) -> usize {
    let alias_dependent_signals = interface_signal_names_matching(interfaces, |signal| {
        signal
            .semantic_consensus
            .as_ref()
            .is_some_and(|consensus| consensus.alias_dependent)
    });

    if alias_dependent_signals.is_empty() {
        return 0;
    }

    temporal_rules
        .iter()
        .filter(|rule| {
            rule.antecedents
                .iter()
                .chain(rule.consequents.iter())
                .any(|predicate| {
                    matches!(
                        predicate,
                        crate::ir::semantic::TemporalPredicateRecord::HandshakeComplete {
                            valid_signal,
                            ready_signal,
                            ..
                        } if alias_dependent_signals.contains(valid_signal)
                            || alias_dependent_signals.contains(ready_signal)
                    )
                })
        })
        .count()
}

fn assert_includes(
    label: &str,
    field: &str,
    expected: &[String],
    actual: &BTreeSet<String>,
    failures: &mut Vec<String>,
) {
    for value in expected {
        if !actual.contains(value) {
            failures.push(format!(
                "{label}: expected `{field}` to include `{value}`, but actual set was {:?}",
                actual
            ));
        }
    }
}

fn assert_excludes(
    label: &str,
    field: &str,
    excluded: &[String],
    actual: &BTreeSet<String>,
    failures: &mut Vec<String>,
) {
    for value in excluded {
        if actual.contains(value) {
            failures.push(format!(
                "{label}: expected `{field}` to exclude `{value}`, but actual set was {:?}",
                actual
            ));
        }
    }
}

fn assert_optional_count(
    label: &str,
    field: &str,
    expected: Option<usize>,
    actual: usize,
    failures: &mut Vec<String>,
) {
    if let Some(expected) = expected {
        if actual != expected {
            failures.push(format!(
                "{label}: expected `{field}` = {expected}, got {actual}"
            ));
        }
    }
}

fn actor_relative_direction_label(direction: ActorRelativeDirection) -> &'static str {
    match direction {
        ActorRelativeDirection::Input => "input",
        ActorRelativeDirection::Output => "output",
        ActorRelativeDirection::InOut => "in_out",
        ActorRelativeDirection::Unknown => "unknown",
    }
}

fn normalize_input_path(path: &Path, base_root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_root.join(path)
    }
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::run;
    use crate::cli::KgBenchArgs;
    use crate::error::AppError;

    #[test]
    fn kg_bench_runs_tracked_fixtures() -> crate::error::Result<()> {
        let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("kg_quality");
        run(KgBenchArgs {
            fixtures_root,
            fixtures: Vec::new(),
        })
    }

    #[test]
    fn kg_bench_reports_fixture_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("broken_fixture");
        fs::create_dir_all(&fixture_dir)?;
        fs::write(
            fixture_dir.join("source.md"),
            "# Interface\nSignal AWVALID is input width 1.\n",
        )?;
        fs::write(
            fixture_dir.join("fixture.json"),
            serde_json::json!({
                "name": "broken_fixture",
                "source": "source.md",
                "expectations": {
                    "semantic": {
                        "resolved_semantic_role_signal_names_include": ["AWVALID"]
                    }
                }
            })
            .to_string(),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for mismatched fixture");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("broken_fixture"));
                assert!(message.contains("resolved_semantic_role_signal_names_include"));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }
}
