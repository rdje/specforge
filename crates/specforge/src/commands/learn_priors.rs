use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::LearnPriorsArgs;
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{SignalSemanticHintSourceKind, SignalSemanticTag};
use crate::ir::intent::IntentIr;
use crate::ir::prior_memory::{
    ActorTaxonomyPriorRecord, ActorTaxonomyRole, CorpusMemory, CorpusMemoryUpdatePolicyRecord,
    PriorSourceArtifactRecord, ProtocolFamily, SemanticPhrasePriorRecord,
    TemporalPhrasePriorRecord, is_meaningful_prior_phrase, normalize_actor_term,
    normalize_prior_phrase,
};
use crate::ir::semantic::{
    ActorRelativeDirection, InterfaceSignalSemanticObservationRecord, InterfaceSignalSemanticRole,
    SemanticGroundingStrength, TemporalPredicateRecord,
};
use crate::ir::source::{AutomationConfidence, ValidationFindingSeverity, ValidationReportRecord};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ActorTaxonomyPriorKey {
    normalized_actor_term: String,
    taxonomy_role: String,
    protocol_family: String,
}

#[derive(Debug, Clone)]
struct ActorTaxonomyPriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
    strongest_grounding_strength: SemanticGroundingStrength,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SemanticPriorKey {
    normalized_phrase: String,
    role: String,
    protocol_family: String,
    source_kind: String,
}

#[derive(Debug, Clone)]
struct SemanticPriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
    strongest_grounding_strength: SemanticGroundingStrength,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TemporalPriorKey {
    normalized_phrase: String,
    protocol_family: String,
    min_cycles: Option<u32>,
    max_cycles: Option<u32>,
    actor_grounded: bool,
    handshake_completion: bool,
}

#[derive(Debug, Clone)]
struct TemporalPriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
}

pub fn run(args: LearnPriorsArgs) -> Result<()> {
    let output_path = args.output;
    let mut source_artifacts = Vec::new();
    let mut actor_taxonomy_priors =
        BTreeMap::<ActorTaxonomyPriorKey, ActorTaxonomyPriorAccumulator>::new();
    let mut semantic_priors = BTreeMap::<SemanticPriorKey, SemanticPriorAccumulator>::new();
    let mut temporal_priors = BTreeMap::<TemporalPriorKey, TemporalPriorAccumulator>::new();

    for artifact in &args.artifacts {
        let artifact_path = canonicalize_existing_path(artifact)?;
        let intent_ir = IntentIr::load_from_path(&artifact_path)?;
        if !matches!(intent_ir.stage, IrStage::IntentIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an IntentIR document before learning priors",
                artifact_path.display()
            )));
        }

        let protocol_family = ProtocolFamily::infer(
            &intent_ir.document_identity.document_key,
            &intent_ir.document_identity.display_name,
        );
        let learning_gate = assess_intent_for_learning(&intent_ir.validation_reports);

        source_artifacts.push(PriorSourceArtifactRecord {
            artifact_path: artifact_path.clone(),
            document_key: intent_ir.document_identity.document_key.clone(),
            display_name: intent_ir.document_identity.display_name.clone(),
            protocol_family,
            overall_score: learning_gate.report.and_then(|report| report.overall_score),
            grade: learning_gate.report.and_then(|report| report.grade.clone()),
            accepted_for_learning: learning_gate.accepted,
            skip_reason: learning_gate.skip_reason.clone(),
        });

        if !learning_gate.accepted {
            continue;
        }

        harvest_actor_taxonomy_priors(&intent_ir, protocol_family, &mut actor_taxonomy_priors);
        harvest_semantic_priors(&intent_ir, protocol_family, &mut semantic_priors);
        harvest_temporal_priors(&intent_ir, protocol_family, &mut temporal_priors);
    }

    let corpus_memory = CorpusMemory {
        schema_version: 2,
        update_policy: CorpusMemoryUpdatePolicyRecord {
            advisory_only: true,
            requires_validated_intent_ir: true,
            rejects_error_findings: true,
            excludes_alias_dependent_semantic_consensus: true,
            local_grounding_required_for_canonical_promotion: true,
        },
        source_artifacts,
        actor_taxonomy_priors: materialize_actor_taxonomy_priors(actor_taxonomy_priors),
        semantic_phrase_priors: materialize_semantic_priors(semantic_priors),
        temporal_phrase_priors: materialize_temporal_priors(temporal_priors),
    };

    let pretty_json = serde_json::to_string_pretty(&corpus_memory)?;
    println!("command: learn-priors");
    println!("artifacts_requested: {}", args.artifacts.len());
    println!(
        "artifacts_accepted: {}",
        corpus_memory
            .source_artifacts
            .iter()
            .filter(|artifact| artifact.accepted_for_learning)
            .count()
    );
    println!(
        "actor_taxonomy_priors: {}",
        corpus_memory.actor_taxonomy_priors.len()
    );
    println!(
        "semantic_phrase_priors: {}",
        corpus_memory.semantic_phrase_priors.len()
    );
    println!(
        "temporal_phrase_priors: {}",
        corpus_memory.temporal_phrase_priors.len()
    );

    for artifact in corpus_memory
        .source_artifacts
        .iter()
        .filter(|artifact| !artifact.accepted_for_learning)
    {
        if let Some(skip_reason) = &artifact.skip_reason {
            println!(
                "skipped_artifact: {} ({skip_reason})",
                artifact.artifact_path.display()
            );
        }
    }

    if args.dry_run {
        println!("{pretty_json}");
        return Ok(());
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, pretty_json)?;
    println!("output_path: {}", output_path.display());

    Ok(())
}

#[derive(Debug, Clone)]
struct LearningGateAssessment<'a> {
    accepted: bool,
    report: Option<&'a ValidationReportRecord>,
    skip_reason: Option<String>,
}

fn assess_intent_for_learning(
    validation_reports: &[ValidationReportRecord],
) -> LearningGateAssessment<'_> {
    let Some(report) = validation_reports.first() else {
        return LearningGateAssessment {
            accepted: false,
            report: None,
            skip_reason: Some("missing persisted validation report".to_string()),
        };
    };

    if !matches!(report.validated_stage, IrStage::IntentIr) {
        return LearningGateAssessment {
            accepted: false,
            report: Some(report),
            skip_reason: Some(format!(
                "latest validation report is for {} instead of intent_ir",
                report.validated_stage.as_str()
            )),
        };
    }

    if report
        .findings
        .iter()
        .any(|finding| matches!(finding.severity, ValidationFindingSeverity::Error))
    {
        return LearningGateAssessment {
            accepted: false,
            report: Some(report),
            skip_reason: Some("validation report contains error findings".to_string()),
        };
    }

    LearningGateAssessment {
        accepted: true,
        report: Some(report),
        skip_reason: None,
    }
}

fn harvest_actor_taxonomy_priors(
    intent_ir: &IntentIr,
    protocol_family: ProtocolFamily,
    actor_taxonomy_priors: &mut BTreeMap<ActorTaxonomyPriorKey, ActorTaxonomyPriorAccumulator>,
) {
    let signal_records_by_name = intent_ir
        .interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .map(|signal| (signal.signal_name.to_ascii_lowercase(), signal))
        .collect::<BTreeMap<_, _>>();
    let conflicted_signals = intent_ir
        .signal_semantic_conflicts
        .iter()
        .map(|conflict| conflict.signal_name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let mut actor_names = intent_ir
        .actor_ports
        .iter()
        .map(|port| port.actor_name.clone())
        .collect::<BTreeSet<_>>();
    for actor in &intent_ir.actors {
        if let Some(actor_name) = actor
            .actor_name
            .clone()
            .or_else(|| actor_name_from_actor_id(&actor.actor_id))
        {
            actor_names.insert(actor_name);
        }
    }

    for actor_name in actor_names {
        let inferred = infer_actor_taxonomy_role(
            intent_ir,
            &actor_name,
            &signal_records_by_name,
            &conflicted_signals,
        )
        .or_else(|| {
            infer_actor_taxonomy_role_from_term(&actor_name).map(|taxonomy_role| {
                (
                    taxonomy_role,
                    AutomationConfidence::Medium,
                    SemanticGroundingStrength::SingleSource,
                )
            })
        });
        let Some((taxonomy_role, automation_confidence, grounding_strength)) = inferred else {
            continue;
        };

        let normalized_actor_term = normalize_actor_term(&actor_name);
        if normalized_actor_term.is_empty() {
            continue;
        }

        let key = ActorTaxonomyPriorKey {
            normalized_actor_term,
            taxonomy_role: taxonomy_role.as_str().to_string(),
            protocol_family: protocol_family.as_str().to_string(),
        };
        let entry =
            actor_taxonomy_priors
                .entry(key)
                .or_insert_with(|| ActorTaxonomyPriorAccumulator {
                    supporting_document_keys: BTreeSet::new(),
                    strongest_automation_confidence: automation_confidence,
                    strongest_grounding_strength: grounding_strength,
                });
        entry
            .supporting_document_keys
            .insert(intent_ir.document_identity.document_key.clone());
        if automation_confidence_rank(automation_confidence)
            > automation_confidence_rank(entry.strongest_automation_confidence)
        {
            entry.strongest_automation_confidence = automation_confidence;
        }
        if semantic_grounding_strength_rank(grounding_strength)
            > semantic_grounding_strength_rank(entry.strongest_grounding_strength)
        {
            entry.strongest_grounding_strength = grounding_strength;
        }
    }
}

fn harvest_semantic_priors(
    intent_ir: &IntentIr,
    protocol_family: ProtocolFamily,
    semantic_priors: &mut BTreeMap<SemanticPriorKey, SemanticPriorAccumulator>,
) {
    let signal_names = collect_signal_names(intent_ir);
    let actor_names = collect_actor_names(intent_ir);
    let conflicted_signals = intent_ir
        .signal_semantic_conflicts
        .iter()
        .map(|conflict| conflict.signal_name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();

    for signal in intent_ir
        .interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
    {
        if conflicted_signals.contains(&signal.signal_name.to_ascii_lowercase()) {
            continue;
        }

        let Some(consensus) = &signal.semantic_consensus else {
            continue;
        };
        if consensus.alias_dependent {
            continue;
        }
        let Some(arbitration) = &signal.semantic_arbitration else {
            continue;
        };
        if !arbitration.decisive {
            continue;
        }

        for observation in signal
            .semantic_observations
            .iter()
            .filter(|observation| observation_matches_role(observation, consensus.role))
            .filter(|observation| {
                !matches!(
                    observation.source_kind,
                    SignalSemanticHintSourceKind::AliasGroundedProseStatement
                )
            })
        {
            let normalized_phrase =
                normalize_prior_phrase(&observation.source_text, &signal_names, &actor_names);
            if !is_meaningful_prior_phrase(&normalized_phrase) {
                continue;
            }

            let key = SemanticPriorKey {
                normalized_phrase,
                role: consensus.role.as_str().to_string(),
                protocol_family: protocol_family.as_str().to_string(),
                source_kind: observation.source_kind.as_str().to_string(),
            };
            let entry = semantic_priors
                .entry(key)
                .or_insert_with(|| SemanticPriorAccumulator {
                    supporting_document_keys: BTreeSet::new(),
                    strongest_automation_confidence: observation.automation_confidence,
                    strongest_grounding_strength: consensus.grounding_strength,
                });
            entry
                .supporting_document_keys
                .insert(intent_ir.document_identity.document_key.clone());
            if automation_confidence_rank(observation.automation_confidence)
                > automation_confidence_rank(entry.strongest_automation_confidence)
            {
                entry.strongest_automation_confidence = observation.automation_confidence;
            }
            if semantic_grounding_strength_rank(consensus.grounding_strength)
                > semantic_grounding_strength_rank(entry.strongest_grounding_strength)
            {
                entry.strongest_grounding_strength = consensus.grounding_strength;
            }
        }
    }
}

fn harvest_temporal_priors(
    intent_ir: &IntentIr,
    protocol_family: ProtocolFamily,
    temporal_priors: &mut BTreeMap<TemporalPriorKey, TemporalPriorAccumulator>,
) {
    let signal_names = collect_signal_names(intent_ir);
    let actor_names = collect_actor_names(intent_ir);
    let conflicted_rule_ids = intent_ir
        .temporal_conflicts
        .iter()
        .flat_map(|conflict| conflict.supporting_rule_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let alias_dependent_handshake = intent_ir.assumptions.iter().any(|assumption| {
        assumption.assumption_id == "assumption_alias_dependent_handshake_completion"
    });

    for rule in &intent_ir.temporal_rules {
        if conflicted_rule_ids.contains(&rule.rule_id) {
            continue;
        }

        let actor_grounded = temporal_rule_is_actor_grounded(rule);
        let handshake_completion = temporal_rule_has_handshake_completion(rule);
        if alias_dependent_handshake && handshake_completion {
            continue;
        }

        if rule.cycle_window.is_none() && !handshake_completion {
            continue;
        }

        let normalized_phrase =
            normalize_prior_phrase(&rule.source_text, &signal_names, &actor_names);
        if !is_meaningful_prior_phrase(&normalized_phrase) {
            continue;
        }

        let key = TemporalPriorKey {
            normalized_phrase,
            protocol_family: protocol_family.as_str().to_string(),
            min_cycles: rule
                .cycle_window
                .as_ref()
                .and_then(|window| window.min_cycles),
            max_cycles: rule
                .cycle_window
                .as_ref()
                .and_then(|window| window.max_cycles),
            actor_grounded,
            handshake_completion,
        };
        let entry = temporal_priors
            .entry(key)
            .or_insert_with(|| TemporalPriorAccumulator {
                supporting_document_keys: BTreeSet::new(),
                strongest_automation_confidence: rule.automation_confidence,
            });
        entry
            .supporting_document_keys
            .insert(intent_ir.document_identity.document_key.clone());
        if automation_confidence_rank(rule.automation_confidence)
            > automation_confidence_rank(entry.strongest_automation_confidence)
        {
            entry.strongest_automation_confidence = rule.automation_confidence;
        }
    }

    for constraint in &intent_ir.signal_constraints {
        if has_temporal_conflict_support(
            &intent_ir.temporal_conflicts,
            &constraint.supporting_statement_ids,
        ) {
            continue;
        }

        harvest_temporal_language_phrase(
            &constraint.source_text,
            extract_cycle_window_from_text(&constraint.source_text),
            false,
            false,
            constraint.automation_confidence,
            &intent_ir.document_identity.document_key,
            protocol_family,
            &signal_names,
            &actor_names,
            temporal_priors,
        );
    }

    for rule in &intent_ir.conditional_rules {
        if has_temporal_conflict_support(
            &intent_ir.temporal_conflicts,
            &rule.supporting_statement_ids,
        ) {
            continue;
        }

        harvest_temporal_language_phrase(
            &rule.source_text,
            extract_cycle_window_from_text(&rule.source_text),
            false,
            false,
            rule.automation_confidence,
            &intent_ir.document_identity.document_key,
            protocol_family,
            &signal_names,
            &actor_names,
            temporal_priors,
        );
    }
}

fn materialize_semantic_priors(
    semantic_priors: BTreeMap<SemanticPriorKey, SemanticPriorAccumulator>,
) -> Vec<SemanticPhrasePriorRecord> {
    semantic_priors
        .into_iter()
        .enumerate()
        .map(|(index, (key, accumulator))| SemanticPhrasePriorRecord {
            prior_id: format!("semantic_phrase_prior_{:04}", index + 1),
            normalized_phrase: key.normalized_phrase,
            role: parse_semantic_role(&key.role),
            protocol_family: parse_protocol_family(&key.protocol_family),
            source_kind: parse_semantic_source_kind(&key.source_kind),
            support_count: accumulator.supporting_document_keys.len(),
            supporting_document_keys: accumulator.supporting_document_keys.into_iter().collect(),
            strongest_automation_confidence: accumulator.strongest_automation_confidence,
            strongest_grounding_strength: accumulator.strongest_grounding_strength,
        })
        .collect()
}

fn materialize_actor_taxonomy_priors(
    actor_taxonomy_priors: BTreeMap<ActorTaxonomyPriorKey, ActorTaxonomyPriorAccumulator>,
) -> Vec<ActorTaxonomyPriorRecord> {
    actor_taxonomy_priors
        .into_iter()
        .enumerate()
        .map(|(index, (key, accumulator))| ActorTaxonomyPriorRecord {
            prior_id: format!("actor_taxonomy_prior_{:04}", index + 1),
            normalized_actor_term: key.normalized_actor_term,
            taxonomy_role: parse_actor_taxonomy_role(&key.taxonomy_role),
            protocol_family: parse_protocol_family(&key.protocol_family),
            support_count: accumulator.supporting_document_keys.len(),
            supporting_document_keys: accumulator.supporting_document_keys.into_iter().collect(),
            strongest_automation_confidence: accumulator.strongest_automation_confidence,
            strongest_grounding_strength: accumulator.strongest_grounding_strength,
        })
        .collect()
}

fn materialize_temporal_priors(
    temporal_priors: BTreeMap<TemporalPriorKey, TemporalPriorAccumulator>,
) -> Vec<TemporalPhrasePriorRecord> {
    temporal_priors
        .into_iter()
        .enumerate()
        .map(|(index, (key, accumulator))| TemporalPhrasePriorRecord {
            prior_id: format!("temporal_phrase_prior_{:04}", index + 1),
            normalized_phrase: key.normalized_phrase,
            protocol_family: parse_protocol_family(&key.protocol_family),
            cycle_window: if key.min_cycles.is_none() && key.max_cycles.is_none() {
                None
            } else {
                Some(crate::ir::semantic::CycleWindowRecord {
                    min_cycles: key.min_cycles,
                    max_cycles: key.max_cycles,
                })
            },
            actor_grounded: key.actor_grounded,
            handshake_completion: key.handshake_completion,
            support_count: accumulator.supporting_document_keys.len(),
            supporting_document_keys: accumulator.supporting_document_keys.into_iter().collect(),
            strongest_automation_confidence: accumulator.strongest_automation_confidence,
        })
        .collect()
}

fn observation_matches_role(
    observation: &InterfaceSignalSemanticObservationRecord,
    role: InterfaceSignalSemanticRole,
) -> bool {
    observation.semantic_tags.iter().any(|tag| {
        matches!(
            (tag, role),
            (
                SignalSemanticTag::HandshakeValidLike,
                InterfaceSignalSemanticRole::HandshakeValidLike
            ) | (
                SignalSemanticTag::HandshakeReadyLike,
                InterfaceSignalSemanticRole::HandshakeReadyLike
            )
        )
    })
}

fn collect_signal_names(intent_ir: &IntentIr) -> BTreeSet<String> {
    let mut signal_names = BTreeSet::new();
    for interface in &intent_ir.interfaces {
        for signal_name in &interface.signals {
            signal_names.insert(signal_name.clone());
        }
        for signal in &interface.signal_records {
            signal_names.insert(signal.signal_name.clone());
        }
    }
    for relation in &intent_ir.actor_signal_relations {
        signal_names.insert(relation.signal_name.clone());
    }
    for port in &intent_ir.actor_ports {
        signal_names.insert(port.signal_name.clone());
    }
    signal_names
}

fn collect_actor_names(intent_ir: &IntentIr) -> BTreeSet<String> {
    let mut actor_names = BTreeSet::new();
    for actor in &intent_ir.actors {
        if let Some(actor_name) = &actor.actor_name {
            actor_names.insert(actor_name.clone());
        }
    }
    for relation in &intent_ir.actor_signal_relations {
        actor_names.insert(relation.actor_name.clone());
    }
    for port in &intent_ir.actor_ports {
        actor_names.insert(port.actor_name.clone());
    }
    actor_names
}

fn actor_name_from_actor_id(actor_id: &str) -> Option<String> {
    actor_id
        .strip_prefix("actor_")
        .map(|suffix| suffix.replace('_', " "))
        .filter(|name| !name.trim().is_empty())
}

fn temporal_rule_is_actor_grounded(rule: &crate::ir::semantic::TemporalRuleRecord) -> bool {
    rule.antecedents
        .iter()
        .chain(rule.consequents.iter())
        .any(|predicate| {
            matches!(
                predicate,
                TemporalPredicateRecord::ActorDrivesSignal { .. }
                    | TemporalPredicateRecord::ActorMaintainsSignalStable { .. }
                    | TemporalPredicateRecord::ActorSamplesSignal { .. }
            )
        })
}

fn temporal_rule_has_handshake_completion(rule: &crate::ir::semantic::TemporalRuleRecord) -> bool {
    rule.antecedents
        .iter()
        .chain(rule.consequents.iter())
        .any(|predicate| matches!(predicate, TemporalPredicateRecord::HandshakeComplete { .. }))
}

fn harvest_temporal_language_phrase(
    source_text: &str,
    cycle_window: Option<crate::ir::semantic::CycleWindowRecord>,
    actor_grounded: bool,
    handshake_completion: bool,
    automation_confidence: AutomationConfidence,
    document_key: &str,
    protocol_family: ProtocolFamily,
    signal_names: &BTreeSet<String>,
    actor_names: &BTreeSet<String>,
    temporal_priors: &mut BTreeMap<TemporalPriorKey, TemporalPriorAccumulator>,
) {
    let normalized_phrase = normalize_prior_phrase(source_text, signal_names, actor_names);
    if !is_meaningful_prior_phrase(&normalized_phrase) {
        return;
    }

    let key = TemporalPriorKey {
        normalized_phrase,
        protocol_family: protocol_family.as_str().to_string(),
        min_cycles: cycle_window.as_ref().and_then(|window| window.min_cycles),
        max_cycles: cycle_window.as_ref().and_then(|window| window.max_cycles),
        actor_grounded,
        handshake_completion,
    };
    let entry = temporal_priors
        .entry(key)
        .or_insert_with(|| TemporalPriorAccumulator {
            supporting_document_keys: BTreeSet::new(),
            strongest_automation_confidence: automation_confidence,
        });
    entry
        .supporting_document_keys
        .insert(document_key.to_string());
    if automation_confidence_rank(automation_confidence)
        > automation_confidence_rank(entry.strongest_automation_confidence)
    {
        entry.strongest_automation_confidence = automation_confidence;
    }
}

fn has_temporal_conflict_support(
    conflicts: &[crate::ir::semantic::TemporalConflictRecord],
    supporting_statement_ids: &[String],
) -> bool {
    conflicts.iter().any(|conflict| {
        conflict
            .supporting_statement_ids
            .iter()
            .any(|statement_id| supporting_statement_ids.contains(statement_id))
    })
}

fn parse_protocol_family(value: &str) -> ProtocolFamily {
    match value {
        "amba_apb" => ProtocolFamily::AmbaApb,
        "amba_ahb" => ProtocolFamily::AmbaAhb,
        "amba_axi" => ProtocolFamily::AmbaAxi,
        "amba_generic" => ProtocolFamily::AmbaGeneric,
        _ => ProtocolFamily::Unknown,
    }
}

fn parse_actor_taxonomy_role(value: &str) -> ActorTaxonomyRole {
    match value {
        "completer_like" => ActorTaxonomyRole::CompleterLike,
        _ => ActorTaxonomyRole::RequesterLike,
    }
}

fn parse_semantic_role(value: &str) -> InterfaceSignalSemanticRole {
    match value {
        "handshake_ready_like" => InterfaceSignalSemanticRole::HandshakeReadyLike,
        _ => InterfaceSignalSemanticRole::HandshakeValidLike,
    }
}

fn parse_semantic_source_kind(value: &str) -> SignalSemanticHintSourceKind {
    match value {
        "signal_description_table" => SignalSemanticHintSourceKind::SignalDescriptionTable,
        "alias_grounded_prose_statement" => {
            SignalSemanticHintSourceKind::AliasGroundedProseStatement
        }
        "visual_caption" => SignalSemanticHintSourceKind::VisualCaption,
        "vlm_timing_diagram_annotation" => SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation,
        _ => SignalSemanticHintSourceKind::ProseStatement,
    }
}

fn infer_actor_taxonomy_role(
    intent_ir: &IntentIr,
    actor_name: &str,
    signal_records_by_name: &BTreeMap<String, &crate::ir::semantic::InterfaceSignalRecord>,
    conflicted_signals: &BTreeSet<String>,
) -> Option<(
    ActorTaxonomyRole,
    AutomationConfidence,
    SemanticGroundingStrength,
)> {
    let mut inferred_roles = BTreeSet::new();
    let mut strongest_automation_confidence = AutomationConfidence::Low;
    let mut strongest_grounding_strength = SemanticGroundingStrength::SingleSource;

    for port in intent_ir.actor_ports.iter().filter(|port| {
        port.actor_name.eq_ignore_ascii_case(actor_name)
            && matches!(
                port.direction,
                ActorRelativeDirection::Output | ActorRelativeDirection::InOut
            )
            && port
                .relation_basis
                .iter()
                .any(|basis| matches!(basis, crate::ir::source::RelationKind::Drives))
    }) {
        if conflicted_signals.contains(&port.signal_name.to_ascii_lowercase()) {
            continue;
        }

        let Some(signal) = signal_records_by_name.get(&port.signal_name.to_ascii_lowercase())
        else {
            continue;
        };
        let Some(consensus) = &signal.semantic_consensus else {
            continue;
        };
        if consensus.alias_dependent {
            continue;
        }
        let Some(arbitration) = &signal.semantic_arbitration else {
            continue;
        };
        if !arbitration.decisive {
            continue;
        }

        let Some(role) = actor_taxonomy_role_from_semantic_role(consensus.role) else {
            continue;
        };
        inferred_roles.insert(role);

        if automation_confidence_rank(port.automation_confidence)
            > automation_confidence_rank(strongest_automation_confidence)
        {
            strongest_automation_confidence = port.automation_confidence;
        }
        if automation_confidence_rank(consensus.automation_confidence)
            > automation_confidence_rank(strongest_automation_confidence)
        {
            strongest_automation_confidence = consensus.automation_confidence;
        }
        if semantic_grounding_strength_rank(consensus.grounding_strength)
            > semantic_grounding_strength_rank(strongest_grounding_strength)
        {
            strongest_grounding_strength = consensus.grounding_strength;
        }
    }

    if inferred_roles.len() != 1 {
        return None;
    }

    Some((
        inferred_roles.into_iter().next()?,
        strongest_automation_confidence,
        strongest_grounding_strength,
    ))
}

fn actor_taxonomy_role_from_semantic_role(
    role: InterfaceSignalSemanticRole,
) -> Option<ActorTaxonomyRole> {
    match role {
        InterfaceSignalSemanticRole::HandshakeValidLike => Some(ActorTaxonomyRole::RequesterLike),
        InterfaceSignalSemanticRole::HandshakeReadyLike => Some(ActorTaxonomyRole::CompleterLike),
    }
}

fn infer_actor_taxonomy_role_from_term(actor_name: &str) -> Option<ActorTaxonomyRole> {
    let normalized = normalize_actor_term(actor_name);
    let tokens = normalized.split_whitespace().collect::<BTreeSet<_>>();
    let requester_like_tokens = [
        "requester",
        "requestor",
        "initiator",
        "manager",
        "master",
        "producer",
        "source",
        "transmitter",
        "tx",
    ];
    let completer_like_tokens = [
        "completer",
        "subordinate",
        "responder",
        "receiver",
        "consumer",
        "target",
        "slave",
        "rx",
    ];

    let requester_like = requester_like_tokens
        .iter()
        .any(|token| tokens.contains(token));
    let completer_like = completer_like_tokens
        .iter()
        .any(|token| tokens.contains(token));

    match (requester_like, completer_like) {
        (true, false) => Some(ActorTaxonomyRole::RequesterLike),
        (false, true) => Some(ActorTaxonomyRole::CompleterLike),
        _ => None,
    }
}

fn extract_cycle_window_from_text(text: &str) -> Option<crate::ir::semantic::CycleWindowRecord> {
    let normalized = text.to_ascii_lowercase();
    let tokens = normalized
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        return None;
    }

    for index in 0..tokens.len() {
        if tokens[index] != "between" {
            continue;
        }
        let Some(min_cycles) = tokens
            .get(index + 1)
            .copied()
            .and_then(parse_cycle_count_value)
        else {
            continue;
        };
        let and_index = if tokens.get(index + 2) == Some(&"and") {
            index + 2
        } else {
            continue;
        };
        let Some(max_cycles) = tokens
            .get(and_index + 1)
            .copied()
            .and_then(parse_cycle_count_value)
        else {
            continue;
        };
        if tokens[and_index + 2..]
            .iter()
            .any(|token| *token == "cycle" || *token == "cycles")
        {
            return Some(crate::ir::semantic::CycleWindowRecord {
                min_cycles: Some(min_cycles),
                max_cycles: Some(max_cycles),
            });
        }
    }

    for index in 0..tokens.len() {
        let Some(count) = tokens
            .get(index + 1)
            .copied()
            .and_then(parse_cycle_count_value)
        else {
            continue;
        };
        let trailing_mentions_cycles = tokens[index + 2..]
            .iter()
            .take(3)
            .any(|token| *token == "cycle" || *token == "cycles");
        if !trailing_mentions_cycles {
            continue;
        }

        match tokens[index] {
            "within" => {
                return Some(crate::ir::semantic::CycleWindowRecord {
                    min_cycles: None,
                    max_cycles: Some(count),
                });
            }
            "after" | "for" => {
                return Some(crate::ir::semantic::CycleWindowRecord {
                    min_cycles: Some(count),
                    max_cycles: Some(count),
                });
            }
            _ => {}
        }
    }

    for index in 0..tokens.len().saturating_sub(2) {
        if tokens[index] == "at" && tokens[index + 1] == "least" {
            let Some(count) = parse_cycle_count_value(tokens[index + 2]) else {
                continue;
            };
            if tokens[index + 3..]
                .iter()
                .take(3)
                .any(|token| *token == "cycle" || *token == "cycles")
            {
                return Some(crate::ir::semantic::CycleWindowRecord {
                    min_cycles: Some(count),
                    max_cycles: None,
                });
            }
        }
        if tokens[index] == "at" && tokens[index + 1] == "most" {
            let Some(count) = parse_cycle_count_value(tokens[index + 2]) else {
                continue;
            };
            if tokens[index + 3..]
                .iter()
                .take(3)
                .any(|token| *token == "cycle" || *token == "cycles")
            {
                return Some(crate::ir::semantic::CycleWindowRecord {
                    min_cycles: None,
                    max_cycles: Some(count),
                });
            }
        }
        if tokens[index] == "no"
            && tokens[index + 1] == "more"
            && tokens.get(index + 2) == Some(&"than")
        {
            let Some(count) = tokens
                .get(index + 3)
                .copied()
                .and_then(parse_cycle_count_value)
            else {
                continue;
            };
            if tokens[index + 4..]
                .iter()
                .take(3)
                .any(|token| *token == "cycle" || *token == "cycles")
            {
                return Some(crate::ir::semantic::CycleWindowRecord {
                    min_cycles: None,
                    max_cycles: Some(count),
                });
            }
        }
    }

    let single_cycle_phrases = [
        ["next", "cycle"].as_slice(),
        ["next", "clock", "cycle"].as_slice(),
        ["following", "cycle"].as_slice(),
        ["subsequent", "cycle"].as_slice(),
        ["next", "tick"].as_slice(),
        ["following", "tick"].as_slice(),
        ["subsequent", "tick"].as_slice(),
        ["next", "rising", "edge"].as_slice(),
        ["following", "rising", "edge"].as_slice(),
        ["subsequent", "rising", "edge"].as_slice(),
    ];
    if single_cycle_phrases
        .iter()
        .any(|phrase| contains_token_phrase(&tokens, phrase))
    {
        return Some(crate::ir::semantic::CycleWindowRecord {
            min_cycles: Some(1),
            max_cycles: Some(1),
        });
    }

    None
}

fn contains_token_phrase(tokens: &[&str], phrase: &[&str]) -> bool {
    if phrase.is_empty() || tokens.len() < phrase.len() {
        return false;
    }
    tokens.windows(phrase.len()).any(|window| {
        window
            .iter()
            .zip(phrase.iter())
            .all(|(lhs, rhs)| lhs == rhs)
    })
}

fn parse_cycle_count_value(token: &str) -> Option<u32> {
    token.parse::<u32>().ok().or_else(|| match token {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        _ => None,
    })
}

fn automation_confidence_rank(confidence: AutomationConfidence) -> u8 {
    match confidence {
        AutomationConfidence::Low => 0,
        AutomationConfidence::Medium => 1,
        AutomationConfidence::High => 2,
    }
}

fn semantic_grounding_strength_rank(strength: SemanticGroundingStrength) -> u8 {
    match strength {
        SemanticGroundingStrength::SingleSource => 0,
        SemanticGroundingStrength::MultiSource => 1,
        SemanticGroundingStrength::CrossModality => 2,
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
    use super::*;
    use crate::ir::intent::{
        IntentActor, IntentArtifactLayout, IntentDocumentIdentity, IntentIdentity,
    };
    use crate::ir::semantic::{
        ActorPortRecord, CycleWindowRecord, InterfaceRecord, InterfaceSignalDirection,
        InterfaceSignalRecord, InterfaceSignalSemanticArbitrationRecord,
        InterfaceSignalSemanticConsensusRecord, InterfaceSignalSemanticObservationRecord,
        TemporalRuleRecord,
    };
    use crate::ir::source::{RelationKind, ValidationFindingRecord, ValidationMetricRecord};

    fn base_intent_ir(document_key: &str, display_name: &str) -> IntentIr {
        IntentIr {
            schema_version: 1,
            stage: IrStage::IntentIr,
            semantic_ir_path: PathBuf::from("/tmp/semantic_ir.json"),
            artifact_layout: IntentArtifactLayout {
                artifact_root: PathBuf::from("/tmp"),
                intent_ir_path: PathBuf::from("/tmp/intent_ir.json"),
            },
            document_identity: IntentDocumentIdentity {
                document_key: document_key.to_string(),
                display_name: display_name.to_string(),
            },
            intent_identity: IntentIdentity {
                intent_id: "intent".to_string(),
                summary: "summary".to_string(),
            },
            actors: vec![IntentActor {
                actor_id: "actor_1".to_string(),
                actor_name: Some("Manager".to_string()),
                responsibilities: Vec::new(),
                supporting_actor_ids: Vec::new(),
            }],
            actor_signal_relations: Vec::new(),
            actor_ports: Vec::new(),
            signal_connectivity: Vec::new(),
            interface_signal_conflicts: Vec::new(),
            signal_connectivity_conflicts: Vec::new(),
            signal_polarity_conflicts: Vec::new(),
            signal_semantic_conflicts: Vec::new(),
            interfaces: Vec::new(),
            system_contract: None,
            behaviors: Vec::new(),
            constraints: Vec::new(),
            assumptions: Vec::new(),
            init_assignments: Vec::new(),
            regular_states: Vec::new(),
            state_transitions: Vec::new(),
            decision_tree_fragments: Vec::new(),
            symbol_definitions: Vec::new(),
            control_blocks: Vec::new(),
            explicit_modules: Vec::new(),
            explicit_tops: Vec::new(),
            register_records: Vec::new(),
            timing_constraints: Vec::new(),
            temporal_rules: Vec::new(),
            temporal_conflicts: Vec::new(),
            signal_constraints: Vec::new(),
            conditional_rules: Vec::new(),
            residual_decisions: Vec::new(),
            validation_reports: vec![ValidationReportRecord {
                report_id: "report_1".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "good".to_string(),
                overall_score: Some(95),
                grade: Some("EXCELLENT".to_string()),
                metrics: vec![ValidationMetricRecord {
                    name: "score".to_string(),
                    value: "95".to_string(),
                }],
                findings: Vec::new(),
            }],
        }
    }

    #[test]
    fn normalize_prior_phrase_replaces_signal_and_actor_terms() {
        let mut signals = BTreeSet::new();
        signals.insert("AWVALID".to_string());
        let mut actors = BTreeSet::new();
        actors.insert("Manager".to_string());

        let normalized = normalize_prior_phrase(
            "Manager asserts AWVALID on the next cycle.",
            &signals,
            &actors,
        );

        assert_eq!(normalized, "<actor> asserts <signal> on the next cycle");
    }

    #[test]
    fn learn_priors_harvests_semantic_and_temporal_priors() {
        let mut intent_ir = base_intent_ir(
            "ihi0022_l_2025_08_amba_axi_protocol_specification",
            "IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification",
        );
        intent_ir.interfaces = vec![InterfaceRecord {
            interface_id: "if_1".to_string(),
            signals: vec!["AWREADY".to_string(), "AWVALID".to_string()],
            signal_records: vec![InterfaceSignalRecord {
                signal_name: "AWREADY".to_string(),
                direction_hint: Some(InterfaceSignalDirection::Input),
                width_hint: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                    candidate_count: 1,
                    leading_role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                    leading_evidence_weight: 3,
                    runner_up_role: None,
                    runner_up_evidence_weight: None,
                    margin_over_runner_up: None,
                    decisive: true,
                }),
                resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeReadyLike),
                semantic_grounding_strength: Some(SemanticGroundingStrength::CrossModality),
                semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                    role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                    grounding_strength: SemanticGroundingStrength::CrossModality,
                    supporting_source_kinds: vec![SignalSemanticHintSourceKind::ProseStatement],
                    supporting_observation_count: 2,
                    automation_confidence: AutomationConfidence::High,
                    alias_dependent: false,
                }),
                semantic_observations: vec![InterfaceSignalSemanticObservationRecord {
                    semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                    source_kind: SignalSemanticHintSourceKind::ProseStatement,
                    source_text: "AWREADY can accept the transfer.".to_string(),
                    supporting_statement_ids: vec!["stmt_1".to_string()],
                    supporting_table_ids: Vec::new(),
                    supporting_visual_evidence_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::High,
                }],
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::High,
            }],
            supporting_statement_ids: Vec::new(),
        }];
        intent_ir.temporal_rules = vec![TemporalRuleRecord {
            rule_id: "rule_1".to_string(),
            clock_signal: Some("ACLK".to_string()),
            edge: crate::ir::semantic::ClockEdge::Rising,
            antecedents: Vec::new(),
            consequents: vec![TemporalPredicateRecord::ActorDrivesSignal {
                actor_name: "Manager".to_string(),
                signal_name: "AWREADY".to_string(),
                phase: crate::ir::semantic::TickPhase::PostTick,
            }],
            cycle_window: Some(CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(1),
            }),
            source_text: "AWREADY must be asserted on the next cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_2".to_string()],
            automation_confidence: AutomationConfidence::High,
        }];

        let mut semantic_priors = BTreeMap::new();
        let mut temporal_priors = BTreeMap::new();
        harvest_semantic_priors(&intent_ir, ProtocolFamily::AmbaAxi, &mut semantic_priors);
        harvest_temporal_priors(&intent_ir, ProtocolFamily::AmbaAxi, &mut temporal_priors);

        let semantic_records = materialize_semantic_priors(semantic_priors);
        let temporal_records = materialize_temporal_priors(temporal_priors);

        assert_eq!(semantic_records.len(), 1);
        assert_eq!(
            semantic_records[0].normalized_phrase,
            "<signal> can accept the transfer"
        );
        assert_eq!(semantic_records[0].protocol_family, ProtocolFamily::AmbaAxi);
        assert_eq!(semantic_records[0].support_count, 1);

        assert_eq!(temporal_records.len(), 1);
        assert_eq!(
            temporal_records[0].normalized_phrase,
            "<signal> must be asserted on the next cycle"
        );
        assert_eq!(
            temporal_records[0].cycle_window,
            Some(CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(1),
            })
        );
        assert!(temporal_records[0].actor_grounded);
    }

    #[test]
    fn learn_priors_harvests_actor_taxonomy_priors() {
        let mut intent_ir = base_intent_ir(
            "ihi0024_d_2021_04_amba_apb_protocol_specification",
            "IHI0024_D_2021-04_AMBA_APB_Protocol_Specification",
        );
        intent_ir.actors = vec![
            IntentActor {
                actor_id: "actor_requester".to_string(),
                actor_name: Some("Requester".to_string()),
                responsibilities: Vec::new(),
                supporting_actor_ids: Vec::new(),
            },
            IntentActor {
                actor_id: "actor_completer".to_string(),
                actor_name: Some("Completer".to_string()),
                responsibilities: Vec::new(),
                supporting_actor_ids: Vec::new(),
            },
        ];
        intent_ir.interfaces = vec![InterfaceRecord {
            interface_id: "if_1".to_string(),
            signals: vec!["PSEL".to_string(), "PREADY".to_string()],
            signal_records: vec![
                InterfaceSignalRecord {
                    signal_name: "PSEL".to_string(),
                    direction_hint: Some(InterfaceSignalDirection::Output),
                    width_hint: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                        leading_evidence_weight: 3,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                    }),
                    resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeValidLike),
                    semantic_grounding_strength: Some(SemanticGroundingStrength::SingleSource),
                    semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                        role: InterfaceSignalSemanticRole::HandshakeValidLike,
                        grounding_strength: SemanticGroundingStrength::SingleSource,
                        supporting_source_kinds: vec![
                            SignalSemanticHintSourceKind::SignalDescriptionTable,
                        ],
                        supporting_observation_count: 1,
                        automation_confidence: AutomationConfidence::High,
                        alias_dependent: false,
                    }),
                    semantic_observations: vec![InterfaceSignalSemanticObservationRecord {
                        semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                        source_kind: SignalSemanticHintSourceKind::SignalDescriptionTable,
                        source_text: "Initiates the transfer request.".to_string(),
                        supporting_statement_ids: vec!["stmt_psel".to_string()],
                        supporting_table_ids: vec!["table_1".to_string()],
                        supporting_visual_evidence_ids: Vec::new(),
                        automation_confidence: AutomationConfidence::High,
                    }],
                    supporting_statement_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::High,
                },
                InterfaceSignalRecord {
                    signal_name: "PREADY".to_string(),
                    direction_hint: Some(InterfaceSignalDirection::Input),
                    width_hint: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                        leading_evidence_weight: 3,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                    }),
                    resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeReadyLike),
                    semantic_grounding_strength: Some(SemanticGroundingStrength::CrossModality),
                    semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                        role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                        grounding_strength: SemanticGroundingStrength::CrossModality,
                        supporting_source_kinds: vec![
                            SignalSemanticHintSourceKind::ProseStatement,
                            SignalSemanticHintSourceKind::VisualCaption,
                        ],
                        supporting_observation_count: 2,
                        automation_confidence: AutomationConfidence::High,
                        alias_dependent: false,
                    }),
                    semantic_observations: vec![InterfaceSignalSemanticObservationRecord {
                        semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                        source_kind: SignalSemanticHintSourceKind::ProseStatement,
                        source_text: "Signals that the completer can accept the transfer."
                            .to_string(),
                        supporting_statement_ids: vec!["stmt_pready".to_string()],
                        supporting_table_ids: Vec::new(),
                        supporting_visual_evidence_ids: Vec::new(),
                        automation_confidence: AutomationConfidence::High,
                    }],
                    supporting_statement_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::High,
                },
            ],
            supporting_statement_ids: Vec::new(),
        }];
        intent_ir.actor_ports = vec![
            ActorPortRecord {
                actor_id: "actor_requester".to_string(),
                actor_name: "Requester".to_string(),
                signal_name: "PSEL".to_string(),
                direction: ActorRelativeDirection::Output,
                relation_basis: vec![RelationKind::Drives],
                width_hint: None,
                source_statement_ids: vec!["stmt_psel".to_string()],
                automation_confidence: AutomationConfidence::High,
            },
            ActorPortRecord {
                actor_id: "actor_completer".to_string(),
                actor_name: "Completer".to_string(),
                signal_name: "PREADY".to_string(),
                direction: ActorRelativeDirection::Output,
                relation_basis: vec![RelationKind::Drives],
                width_hint: None,
                source_statement_ids: vec!["stmt_pready".to_string()],
                automation_confidence: AutomationConfidence::High,
            },
        ];

        let mut actor_taxonomy_priors = BTreeMap::new();
        harvest_actor_taxonomy_priors(
            &intent_ir,
            ProtocolFamily::AmbaApb,
            &mut actor_taxonomy_priors,
        );

        let records = materialize_actor_taxonomy_priors(actor_taxonomy_priors);
        assert_eq!(records.len(), 2);
        assert!(records.iter().any(|record| {
            record.normalized_actor_term == "requester"
                && record.taxonomy_role == ActorTaxonomyRole::RequesterLike
        }));
        assert!(records.iter().any(|record| {
            record.normalized_actor_term == "completer"
                && record.taxonomy_role == ActorTaxonomyRole::CompleterLike
                && record.strongest_grounding_strength == SemanticGroundingStrength::CrossModality
        }));
    }

    #[test]
    fn learn_priors_harvests_actor_taxonomy_from_actor_identity_terms() {
        let mut intent_ir = base_intent_ir("doc_actor_terms", "doc_actor_terms");
        intent_ir.actors = vec![IntentActor {
            actor_id: "actor_requester".to_string(),
            actor_name: None,
            responsibilities: vec![
                "semantic role inferred around `requester` evidence".to_string(),
            ],
            supporting_actor_ids: vec!["actor_requester".to_string()],
        }];

        let mut actor_taxonomy_priors = BTreeMap::new();
        harvest_actor_taxonomy_priors(
            &intent_ir,
            ProtocolFamily::Unknown,
            &mut actor_taxonomy_priors,
        );

        let records = materialize_actor_taxonomy_priors(actor_taxonomy_priors);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].normalized_actor_term, "requester");
        assert_eq!(records[0].taxonomy_role, ActorTaxonomyRole::RequesterLike);
        assert_eq!(
            records[0].strongest_grounding_strength,
            SemanticGroundingStrength::SingleSource
        );
    }

    #[test]
    fn learn_priors_skips_alias_dependent_consensus_and_error_reports() {
        let mut intent_ir = base_intent_ir("doc", "doc");
        intent_ir.validation_reports[0]
            .findings
            .push(ValidationFindingRecord {
                finding_id: "error_1".to_string(),
                severity: ValidationFindingSeverity::Error,
                category: "test".to_string(),
                summary: "bad".to_string(),
                related_ids: Vec::new(),
            });

        let assessment = assess_intent_for_learning(&intent_ir.validation_reports);
        assert!(!assessment.accepted);

        let mut accepted_intent = base_intent_ir("doc2", "doc2");
        accepted_intent.interfaces = vec![InterfaceRecord {
            interface_id: "if_1".to_string(),
            signals: vec!["XREQ".to_string()],
            signal_records: vec![InterfaceSignalRecord {
                signal_name: "XREQ".to_string(),
                direction_hint: None,
                width_hint: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                    candidate_count: 1,
                    leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    leading_evidence_weight: 1,
                    runner_up_role: None,
                    runner_up_evidence_weight: None,
                    margin_over_runner_up: None,
                    decisive: true,
                }),
                resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeValidLike),
                semantic_grounding_strength: Some(SemanticGroundingStrength::SingleSource),
                semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                    role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    grounding_strength: SemanticGroundingStrength::SingleSource,
                    supporting_source_kinds: vec![
                        SignalSemanticHintSourceKind::AliasGroundedProseStatement,
                    ],
                    supporting_observation_count: 1,
                    automation_confidence: AutomationConfidence::Medium,
                    alias_dependent: true,
                }),
                semantic_observations: vec![InterfaceSignalSemanticObservationRecord {
                    semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                    source_kind: SignalSemanticHintSourceKind::AliasGroundedProseStatement,
                    source_text: "request phase".to_string(),
                    supporting_statement_ids: vec!["stmt_1".to_string()],
                    supporting_table_ids: Vec::new(),
                    supporting_visual_evidence_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::Medium,
                }],
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            }],
            supporting_statement_ids: Vec::new(),
        }];

        let mut semantic_priors = BTreeMap::new();
        harvest_semantic_priors(
            &accepted_intent,
            ProtocolFamily::Unknown,
            &mut semantic_priors,
        );
        assert!(semantic_priors.is_empty());
    }

    #[test]
    fn learn_priors_skips_ambiguous_actor_taxonomy_roles() {
        let mut intent_ir = base_intent_ir("doc3", "doc3");
        intent_ir.actors = vec![IntentActor {
            actor_id: "actor_1".to_string(),
            actor_name: Some("Hybrid".to_string()),
            responsibilities: Vec::new(),
            supporting_actor_ids: vec!["actor_1".to_string()],
        }];
        intent_ir.actor_ports = vec![
            ActorPortRecord {
                actor_id: "actor_1".to_string(),
                actor_name: "Hybrid".to_string(),
                signal_name: "XVALID".to_string(),
                direction: ActorRelativeDirection::Output,
                relation_basis: vec![RelationKind::Drives],
                width_hint: None,
                source_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            },
            ActorPortRecord {
                actor_id: "actor_1".to_string(),
                actor_name: "Hybrid".to_string(),
                signal_name: "XREADY".to_string(),
                direction: ActorRelativeDirection::Output,
                relation_basis: vec![RelationKind::Drives],
                width_hint: None,
                source_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            },
        ];
        intent_ir.interfaces = vec![InterfaceRecord {
            interface_id: "if_1".to_string(),
            signals: vec!["XVALID".to_string(), "XREADY".to_string()],
            signal_records: vec![
                InterfaceSignalRecord {
                    signal_name: "XVALID".to_string(),
                    direction_hint: Some(InterfaceSignalDirection::Output),
                    width_hint: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                        leading_evidence_weight: 1,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                    }),
                    resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeValidLike),
                    semantic_grounding_strength: Some(SemanticGroundingStrength::SingleSource),
                    semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                        role: InterfaceSignalSemanticRole::HandshakeValidLike,
                        grounding_strength: SemanticGroundingStrength::SingleSource,
                        supporting_source_kinds: vec![SignalSemanticHintSourceKind::ProseStatement],
                        supporting_observation_count: 1,
                        automation_confidence: AutomationConfidence::Medium,
                        alias_dependent: false,
                    }),
                    semantic_observations: Vec::new(),
                    supporting_statement_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::Medium,
                },
                InterfaceSignalRecord {
                    signal_name: "XREADY".to_string(),
                    direction_hint: Some(InterfaceSignalDirection::Output),
                    width_hint: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                        leading_evidence_weight: 1,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                    }),
                    resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeReadyLike),
                    semantic_grounding_strength: Some(SemanticGroundingStrength::SingleSource),
                    semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                        role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                        grounding_strength: SemanticGroundingStrength::SingleSource,
                        supporting_source_kinds: vec![SignalSemanticHintSourceKind::ProseStatement],
                        supporting_observation_count: 1,
                        automation_confidence: AutomationConfidence::Medium,
                        alias_dependent: false,
                    }),
                    semantic_observations: Vec::new(),
                    supporting_statement_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::Medium,
                },
            ],
            supporting_statement_ids: Vec::new(),
        }];

        let mut actor_taxonomy_priors = BTreeMap::new();
        harvest_actor_taxonomy_priors(
            &intent_ir,
            ProtocolFamily::Unknown,
            &mut actor_taxonomy_priors,
        );
        assert!(actor_taxonomy_priors.is_empty());
    }
}
