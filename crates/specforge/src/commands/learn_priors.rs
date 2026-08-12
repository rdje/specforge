use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use crate::cli::LearnPriorsArgs;
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::corpus_cluster::{
    DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD, DocumentFingerprint, derive_extraction_profiles,
    document_fingerprint,
};
use crate::ir::evidence::EvidenceIr;
use crate::ir::evidence::{SignalSemanticHintSourceKind, SignalSemanticTag};
use crate::ir::intent::IntentIr;
use crate::ir::prior_memory::{
    ActorTaxonomyPriorRecord, ActorTaxonomyRole, CorpusMemory, CorpusMemoryUpdatePolicyRecord,
    ExtractionProfileExtractorSupportRecord, ExtractionProfilePriorRecord, NegativeKnowledgeKind,
    NegativeKnowledgePriorRecord, PriorScope, PriorSourceArtifactRecord,
    SemanticModalityReliabilityPriorRecord, SemanticPhrasePriorRecord, TableShapePriorRecord,
    TemporalPhrasePriorRecord, VisualMotifPriorRecord,
    interface_signal_conflict_negative_knowledge_pattern, is_meaningful_actor_term,
    is_meaningful_prior_phrase, normalize_actor_term, normalize_prior_phrase,
    normalize_table_header_signature, residual_decision_negative_knowledge_pattern,
    signal_connectivity_conflict_negative_knowledge_pattern,
    signal_polarity_conflict_negative_knowledge_pattern,
    signal_semantic_conflict_negative_knowledge_pattern,
    temporal_value_conflict_negative_knowledge_pattern,
};
use crate::ir::semantic::SemanticIr;
use crate::ir::semantic::{
    ActorRelativeDirection, InterfaceSignalSemanticObservationRecord, InterfaceSignalSemanticRole,
    SemanticGroundingStrength, TemporalPredicateRecord,
};
use crate::ir::source::{AutomationConfidence, ValidationFindingSeverity, ValidationReportRecord};
use crate::ir::source::{DiagramKind, SourceIr, StructuredTableRecord, TableKind, VisualAssetKind};
use crate::persisted_path::{PersistedPathOrigin, resolve_existing, resolve_repository_output};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ActorTaxonomyPriorKey {
    normalized_actor_term: String,
    taxonomy_role: String,
    prior_scope: PriorScope,
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
    prior_scope: PriorScope,
    source_kind: String,
}

#[derive(Debug, Clone)]
struct SemanticPriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
    strongest_grounding_strength: SemanticGroundingStrength,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SemanticModalityReliabilityPriorKey {
    role: String,
    prior_scope: PriorScope,
    source_kind: String,
}

#[derive(Debug, Clone)]
struct SemanticModalityReliabilityPriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
    strongest_grounding_strength: SemanticGroundingStrength,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TemporalPriorKey {
    normalized_phrase: String,
    prior_scope: PriorScope,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TableShapePriorKey {
    normalized_header_signature: String,
    table_kind: String,
    prior_scope: PriorScope,
}

#[derive(Debug, Clone)]
struct TableShapePriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct VisualMotifPriorKey {
    normalized_caption_phrase: Option<String>,
    diagram_kind: String,
    asset_kind: String,
    prior_scope: PriorScope,
}

#[derive(Debug, Clone)]
struct VisualMotifPriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct NegativeKnowledgePriorKey {
    knowledge_kind: String,
    normalized_pattern: String,
    prior_scope: PriorScope,
}

#[derive(Debug, Clone)]
struct NegativeKnowledgePriorAccumulator {
    supporting_document_keys: BTreeSet<String>,
    strongest_automation_confidence: AutomationConfidence,
}

pub fn run(args: LearnPriorsArgs) -> Result<()> {
    let output_path = args.output;
    let mut source_artifacts = Vec::new();
    let mut actor_taxonomy_priors =
        BTreeMap::<ActorTaxonomyPriorKey, ActorTaxonomyPriorAccumulator>::new();
    let mut semantic_priors = BTreeMap::<SemanticPriorKey, SemanticPriorAccumulator>::new();
    let mut semantic_modality_reliability_priors = BTreeMap::<
        SemanticModalityReliabilityPriorKey,
        SemanticModalityReliabilityPriorAccumulator,
    >::new();
    let mut temporal_priors = BTreeMap::<TemporalPriorKey, TemporalPriorAccumulator>::new();
    let mut table_shape_priors = BTreeMap::<TableShapePriorKey, TableShapePriorAccumulator>::new();
    let mut visual_motif_priors =
        BTreeMap::<VisualMotifPriorKey, VisualMotifPriorAccumulator>::new();
    let mut negative_knowledge_priors =
        BTreeMap::<NegativeKnowledgePriorKey, NegativeKnowledgePriorAccumulator>::new();
    let mut fingerprint_documents: Vec<(String, DocumentFingerprint)> = Vec::new();

    for artifact in &args.artifacts {
        let artifact_path = resolve_existing(artifact, PersistedPathOrigin::RepositoryOwned)?;
        let intent_ir = IntentIr::load_from_path(&artifact_path)?;
        if !matches!(intent_ir.stage, IrStage::IntentIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an IntentIR document before learning priors",
                artifact_path.display()
            )));
        }

        let prior_scope = PriorScope::Global;
        let learning_gate = assess_intent_for_learning(&intent_ir.validation_reports);

        source_artifacts.push(PriorSourceArtifactRecord {
            artifact_path: artifact_path.clone(),
            document_key: intent_ir.document_identity.document_key.clone(),
            display_name: intent_ir.document_identity.display_name.clone(),
            prior_scope,
            overall_score: learning_gate.report.and_then(|report| report.overall_score),
            grade: learning_gate.report.and_then(|report| report.grade.clone()),
            accepted_for_learning: learning_gate.accepted,
            skip_reason: learning_gate.skip_reason.clone(),
        });

        if !learning_gate.accepted {
            continue;
        }

        harvest_actor_taxonomy_priors(&intent_ir, prior_scope, &mut actor_taxonomy_priors);
        harvest_semantic_priors(&intent_ir, prior_scope, &mut semantic_priors);
        harvest_semantic_modality_reliability_priors(
            &intent_ir,
            prior_scope,
            &mut semantic_modality_reliability_priors,
        );
        harvest_temporal_priors(&intent_ir, prior_scope, &mut temporal_priors);
        harvest_table_shape_priors(&intent_ir, prior_scope, &mut table_shape_priors);
        harvest_visual_motif_priors(&intent_ir, prior_scope, &mut visual_motif_priors);
        harvest_negative_knowledge_priors(&intent_ir, prior_scope, &mut negative_knowledge_priors);
        // CORPUS-PATTERN-REUSE.3b.2: the extraction-profile harvest clusters the accepted
        // documents' derived fingerprints, which live on the persisted EvidenceIR. A document
        // whose evidence artifact cannot be reloaded simply contributes no fingerprint —
        // honest absence, like the table-shape/visual-motif harvests above.
        if let Some(evidence_ir) = load_evidence_ir_for_learning(&intent_ir) {
            fingerprint_documents.push((
                intent_ir.document_identity.document_key.clone(),
                document_fingerprint(&evidence_ir),
            ));
        }
    }

    let corpus_memory = CorpusMemory {
        schema_version: crate::ir::prior_memory::CORPUS_MEMORY_SCHEMA_VERSION,
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
        semantic_modality_reliability_priors: materialize_semantic_modality_reliability_priors(
            semantic_modality_reliability_priors,
        ),
        temporal_phrase_priors: materialize_temporal_priors(temporal_priors),
        table_shape_priors: materialize_table_shape_priors(table_shape_priors),
        visual_motif_priors: materialize_visual_motif_priors(visual_motif_priors),
        negative_knowledge_priors: materialize_negative_knowledge_priors(negative_knowledge_priors),
        extraction_profile_priors: materialize_extraction_profile_priors(&fingerprint_documents),
    };

    let pretty_json = corpus_memory.to_pretty_json()?;
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
        "semantic_modality_reliability_priors: {}",
        corpus_memory.semantic_modality_reliability_priors.len()
    );
    println!(
        "temporal_phrase_priors: {}",
        corpus_memory.temporal_phrase_priors.len()
    );
    println!(
        "table_shape_priors: {}",
        corpus_memory.table_shape_priors.len()
    );
    println!(
        "visual_motif_priors: {}",
        corpus_memory.visual_motif_priors.len()
    );
    println!(
        "negative_knowledge_priors: {}",
        corpus_memory.negative_knowledge_priors.len()
    );
    println!(
        "extraction_profile_priors: {}",
        corpus_memory.extraction_profile_priors.len()
    );

    // PRIOR-DECAY: surface cross-document contradictions (same key, conflicting
    // values) the accrete-only harvest never revises. Advisory-only — reported,
    // never auto-resolved.
    let contested_priors = corpus_memory.contested_priors();
    println!("contested_priors: {}", contested_priors.len());
    for contested in &contested_priors {
        let competing = contested
            .competing_values
            .iter()
            .map(|value| format!("{}={}", value.value, value.support_count))
            .collect::<Vec<_>>()
            .join(" vs ");
        println!(
            "  contested {} [{:?}] '{}': {competing} (strongest: {})",
            contested.family.as_str(),
            contested.prior_scope,
            contested.key,
            contested.strongest_value,
        );
    }

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

    let output_path = resolve_repository_output(&output_path)?;
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
    prior_scope: PriorScope,
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
        if !is_meaningful_actor_term(&normalized_actor_term) {
            continue;
        }

        let key = ActorTaxonomyPriorKey {
            normalized_actor_term,
            taxonomy_role: taxonomy_role.as_str().to_string(),
            prior_scope,
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
    prior_scope: PriorScope,
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
                prior_scope,
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

fn harvest_semantic_modality_reliability_priors(
    intent_ir: &IntentIr,
    prior_scope: PriorScope,
    semantic_modality_reliability_priors: &mut BTreeMap<
        SemanticModalityReliabilityPriorKey,
        SemanticModalityReliabilityPriorAccumulator,
    >,
) {
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

        for source_kind in signal
            .semantic_observations
            .iter()
            .filter(|observation| observation_matches_role(observation, consensus.role))
            .filter(|observation| {
                !matches!(
                    observation.source_kind,
                    SignalSemanticHintSourceKind::AliasGroundedProseStatement
                )
            })
            .map(|observation| observation.source_kind)
            .collect::<BTreeSet<_>>()
        {
            let key = SemanticModalityReliabilityPriorKey {
                role: consensus.role.as_str().to_string(),
                prior_scope,
                source_kind: source_kind.as_str().to_string(),
            };
            let entry = semantic_modality_reliability_priors
                .entry(key)
                .or_insert_with(|| SemanticModalityReliabilityPriorAccumulator {
                    supporting_document_keys: BTreeSet::new(),
                    strongest_automation_confidence: consensus.automation_confidence,
                    strongest_grounding_strength: consensus.grounding_strength,
                });
            entry
                .supporting_document_keys
                .insert(intent_ir.document_identity.document_key.clone());
            if automation_confidence_rank(consensus.automation_confidence)
                > automation_confidence_rank(entry.strongest_automation_confidence)
            {
                entry.strongest_automation_confidence = consensus.automation_confidence;
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
    prior_scope: PriorScope,
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
            prior_scope,
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
            prior_scope,
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
            prior_scope,
            &signal_names,
            &actor_names,
            temporal_priors,
        );
    }
}

fn harvest_table_shape_priors(
    intent_ir: &IntentIr,
    prior_scope: PriorScope,
    table_shape_priors: &mut BTreeMap<TableShapePriorKey, TableShapePriorAccumulator>,
) {
    let Some(source_ir) = load_source_ir_for_learning(intent_ir) else {
        return;
    };
    harvest_table_shape_priors_from_source_ir(
        &source_ir,
        &intent_ir.document_identity.document_key,
        prior_scope,
        table_shape_priors,
    );
}

fn load_evidence_ir_for_learning(intent_ir: &IntentIr) -> Option<EvidenceIr> {
    let semantic_ir = SemanticIr::load_from_path(&intent_ir.semantic_ir_path).ok()?;
    if !matches!(semantic_ir.stage, IrStage::SemanticIr) {
        return None;
    }
    let evidence_ir = EvidenceIr::load_from_path(&semantic_ir.evidence_ir_path).ok()?;
    if !matches!(evidence_ir.stage, IrStage::EvidenceIr) {
        return None;
    }
    Some(evidence_ir)
}

fn load_source_ir_for_learning(intent_ir: &IntentIr) -> Option<SourceIr> {
    let evidence_ir = load_evidence_ir_for_learning(intent_ir)?;
    let source_ir = SourceIr::load_from_path(&evidence_ir.source_ir_path).ok()?;
    if !matches!(source_ir.stage, IrStage::SourceIr) {
        return None;
    }
    Some(source_ir)
}

fn harvest_table_shape_priors_from_source_ir(
    source_ir: &SourceIr,
    document_key: &str,
    prior_scope: PriorScope,
    table_shape_priors: &mut BTreeMap<TableShapePriorKey, TableShapePriorAccumulator>,
) {
    for table in &source_ir.structured_tables {
        if matches!(table.table_kind, TableKind::Unknown) {
            continue;
        }
        let Some(normalized_header_signature) = normalize_table_header_signature(table) else {
            continue;
        };

        let key = TableShapePriorKey {
            normalized_header_signature,
            table_kind: table_kind_key(table.table_kind).to_string(),
            prior_scope,
        };
        let entry = table_shape_priors
            .entry(key)
            .or_insert_with(|| TableShapePriorAccumulator {
                supporting_document_keys: BTreeSet::new(),
                strongest_automation_confidence: infer_table_shape_prior_confidence(table),
            });
        entry
            .supporting_document_keys
            .insert(document_key.to_string());
        let confidence = infer_table_shape_prior_confidence(table);
        if automation_confidence_rank(confidence)
            > automation_confidence_rank(entry.strongest_automation_confidence)
        {
            entry.strongest_automation_confidence = confidence;
        }
    }
}

fn harvest_visual_motif_priors(
    intent_ir: &IntentIr,
    prior_scope: PriorScope,
    visual_motif_priors: &mut BTreeMap<VisualMotifPriorKey, VisualMotifPriorAccumulator>,
) {
    let Some(source_ir) = load_source_ir_for_learning(intent_ir) else {
        return;
    };
    let signal_names = collect_signal_names(intent_ir);
    let actor_names = collect_actor_names(intent_ir);
    harvest_visual_motif_priors_from_source_ir(
        &source_ir,
        &intent_ir.document_identity.document_key,
        prior_scope,
        &signal_names,
        &actor_names,
        visual_motif_priors,
    );
}

fn harvest_visual_motif_priors_from_source_ir(
    source_ir: &SourceIr,
    document_key: &str,
    prior_scope: PriorScope,
    signal_names: &BTreeSet<String>,
    actor_names: &BTreeSet<String>,
    visual_motif_priors: &mut BTreeMap<VisualMotifPriorKey, VisualMotifPriorAccumulator>,
) {
    for visual_asset in &source_ir.visual_assets {
        if matches!(visual_asset.diagram_kind, DiagramKind::Unknown)
            && matches!(visual_asset.asset_kind, VisualAssetKind::Unknown)
        {
            continue;
        }

        let normalized_caption_phrase = visual_asset.caption_text.as_deref().and_then(|caption| {
            let normalized = normalize_prior_phrase(caption, signal_names, actor_names);
            is_meaningful_prior_phrase(&normalized).then_some(normalized)
        });
        if normalized_caption_phrase.is_none()
            && matches!(visual_asset.diagram_kind, DiagramKind::Unknown)
        {
            continue;
        }

        let confidence = infer_visual_motif_prior_confidence(
            visual_asset.diagram_kind,
            normalized_caption_phrase.is_some(),
        );
        let key = VisualMotifPriorKey {
            normalized_caption_phrase,
            diagram_kind: diagram_kind_key(visual_asset.diagram_kind).to_string(),
            asset_kind: visual_asset_kind_key(visual_asset.asset_kind).to_string(),
            prior_scope,
        };
        let entry = visual_motif_priors
            .entry(key)
            .or_insert_with(|| VisualMotifPriorAccumulator {
                supporting_document_keys: BTreeSet::new(),
                strongest_automation_confidence: confidence,
            });
        entry
            .supporting_document_keys
            .insert(document_key.to_string());
        if automation_confidence_rank(confidence)
            > automation_confidence_rank(entry.strongest_automation_confidence)
        {
            entry.strongest_automation_confidence = confidence;
        }
    }
}

fn harvest_negative_knowledge_priors(
    intent_ir: &IntentIr,
    prior_scope: PriorScope,
    negative_knowledge_priors: &mut BTreeMap<
        NegativeKnowledgePriorKey,
        NegativeKnowledgePriorAccumulator,
    >,
) {
    for conflict in &intent_ir.signal_semantic_conflicts {
        let Some(normalized_pattern) =
            signal_semantic_conflict_negative_knowledge_pattern(conflict)
        else {
            continue;
        };
        harvest_negative_knowledge_pattern(
            NegativeKnowledgeKind::SignalSemanticConflict,
            normalized_pattern,
            conflict.automation_confidence,
            &intent_ir.document_identity.document_key,
            prior_scope,
            negative_knowledge_priors,
        );
    }

    for conflict in &intent_ir.temporal_conflicts {
        let Some(normalized_pattern) = temporal_value_conflict_negative_knowledge_pattern(conflict)
        else {
            continue;
        };
        harvest_negative_knowledge_pattern(
            NegativeKnowledgeKind::TemporalValueConflict,
            normalized_pattern,
            conflict.automation_confidence,
            &intent_ir.document_identity.document_key,
            prior_scope,
            negative_knowledge_priors,
        );
    }

    for conflict in &intent_ir.signal_polarity_conflicts {
        let Some(normalized_pattern) =
            signal_polarity_conflict_negative_knowledge_pattern(conflict)
        else {
            continue;
        };
        harvest_negative_knowledge_pattern(
            NegativeKnowledgeKind::SignalPolarityConflict,
            normalized_pattern,
            conflict.automation_confidence,
            &intent_ir.document_identity.document_key,
            prior_scope,
            negative_knowledge_priors,
        );
    }

    for conflict in &intent_ir.interface_signal_conflicts {
        let Some(normalized_pattern) =
            interface_signal_conflict_negative_knowledge_pattern(conflict)
        else {
            continue;
        };
        harvest_negative_knowledge_pattern(
            NegativeKnowledgeKind::InterfaceSignalConflict,
            normalized_pattern,
            conflict.automation_confidence,
            &intent_ir.document_identity.document_key,
            prior_scope,
            negative_knowledge_priors,
        );
    }

    for conflict in &intent_ir.signal_connectivity_conflicts {
        let Some(normalized_pattern) =
            signal_connectivity_conflict_negative_knowledge_pattern(conflict)
        else {
            continue;
        };
        harvest_negative_knowledge_pattern(
            NegativeKnowledgeKind::SignalConnectivityConflict,
            normalized_pattern,
            conflict.automation_confidence,
            &intent_ir.document_identity.document_key,
            prior_scope,
            negative_knowledge_priors,
        );
    }

    for residual in &intent_ir.residual_decisions {
        let Some(normalized_pattern) = residual_decision_negative_knowledge_pattern(residual)
        else {
            continue;
        };
        harvest_negative_knowledge_pattern(
            NegativeKnowledgeKind::ResidualDecision,
            normalized_pattern,
            residual.automation_confidence,
            &intent_ir.document_identity.document_key,
            prior_scope,
            negative_knowledge_priors,
        );
    }
}

fn harvest_negative_knowledge_pattern(
    knowledge_kind: NegativeKnowledgeKind,
    normalized_pattern: String,
    automation_confidence: AutomationConfidence,
    document_key: &str,
    prior_scope: PriorScope,
    negative_knowledge_priors: &mut BTreeMap<
        NegativeKnowledgePriorKey,
        NegativeKnowledgePriorAccumulator,
    >,
) {
    if normalized_pattern.trim().is_empty() {
        return;
    }

    let key = NegativeKnowledgePriorKey {
        knowledge_kind: knowledge_kind.as_str().to_string(),
        normalized_pattern,
        prior_scope,
    };
    let entry =
        negative_knowledge_priors
            .entry(key)
            .or_insert_with(|| NegativeKnowledgePriorAccumulator {
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
            prior_scope: key.prior_scope,
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
            prior_scope: key.prior_scope,
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
            prior_scope: key.prior_scope,
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

fn materialize_table_shape_priors(
    table_shape_priors: BTreeMap<TableShapePriorKey, TableShapePriorAccumulator>,
) -> Vec<TableShapePriorRecord> {
    table_shape_priors
        .into_iter()
        .enumerate()
        .map(|(index, (key, accumulator))| TableShapePriorRecord {
            prior_id: format!("table_shape_prior_{:04}", index + 1),
            normalized_header_signature: key.normalized_header_signature,
            table_kind: parse_table_kind(&key.table_kind),
            prior_scope: key.prior_scope,
            support_count: accumulator.supporting_document_keys.len(),
            supporting_document_keys: accumulator.supporting_document_keys.into_iter().collect(),
            strongest_automation_confidence: accumulator.strongest_automation_confidence,
        })
        .collect()
}

fn materialize_semantic_modality_reliability_priors(
    semantic_modality_reliability_priors: BTreeMap<
        SemanticModalityReliabilityPriorKey,
        SemanticModalityReliabilityPriorAccumulator,
    >,
) -> Vec<SemanticModalityReliabilityPriorRecord> {
    semantic_modality_reliability_priors
        .into_iter()
        .enumerate()
        .map(
            |(index, (key, accumulator))| SemanticModalityReliabilityPriorRecord {
                prior_id: format!("semantic_modality_reliability_prior_{:04}", index + 1),
                role: parse_semantic_role(&key.role),
                prior_scope: key.prior_scope,
                source_kind: parse_semantic_source_kind(&key.source_kind),
                support_count: accumulator.supporting_document_keys.len(),
                supporting_document_keys: accumulator
                    .supporting_document_keys
                    .into_iter()
                    .collect(),
                strongest_automation_confidence: accumulator.strongest_automation_confidence,
                strongest_grounding_strength: accumulator.strongest_grounding_strength,
            },
        )
        .collect()
}

fn materialize_visual_motif_priors(
    visual_motif_priors: BTreeMap<VisualMotifPriorKey, VisualMotifPriorAccumulator>,
) -> Vec<VisualMotifPriorRecord> {
    visual_motif_priors
        .into_iter()
        .enumerate()
        .map(|(index, (key, accumulator))| VisualMotifPriorRecord {
            prior_id: format!("visual_motif_prior_{:04}", index + 1),
            normalized_caption_phrase: key.normalized_caption_phrase,
            diagram_kind: parse_diagram_kind(&key.diagram_kind),
            asset_kind: parse_visual_asset_kind(&key.asset_kind),
            prior_scope: key.prior_scope,
            support_count: accumulator.supporting_document_keys.len(),
            supporting_document_keys: accumulator.supporting_document_keys.into_iter().collect(),
            strongest_automation_confidence: accumulator.strongest_automation_confidence,
        })
        .collect()
}

fn materialize_negative_knowledge_priors(
    negative_knowledge_priors: BTreeMap<
        NegativeKnowledgePriorKey,
        NegativeKnowledgePriorAccumulator,
    >,
) -> Vec<NegativeKnowledgePriorRecord> {
    negative_knowledge_priors
        .into_iter()
        .enumerate()
        .map(|(index, (key, accumulator))| NegativeKnowledgePriorRecord {
            prior_id: format!("negative_knowledge_prior_{:04}", index + 1),
            knowledge_kind: parse_negative_knowledge_kind(&key.knowledge_kind),
            normalized_pattern: key.normalized_pattern,
            prior_scope: key.prior_scope,
            support_count: accumulator.supporting_document_keys.len(),
            supporting_document_keys: accumulator.supporting_document_keys.into_iter().collect(),
            strongest_automation_confidence: accumulator.strongest_automation_confidence,
        })
        .collect()
}

/// Materialize the 8th prior family (`CORPUS-PATTERN-REUSE.3b.2`): cluster the accepted
/// documents' derived fingerprints at the same default threshold the `corpus-cluster`
/// command uses, then persist one advisory profile per MULTI-member cluster — a cluster
/// of one carries no reusable cross-document pattern, so singletons are never harvested.
/// Deterministic: `derive_extraction_profiles` is key-sorted and the per-profile fields
/// (signature, fired extractors, members) are already deterministically ordered.
fn materialize_extraction_profile_priors(
    fingerprint_documents: &[(String, DocumentFingerprint)],
) -> Vec<ExtractionProfilePriorRecord> {
    derive_extraction_profiles(
        fingerprint_documents,
        DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD,
    )
    .into_iter()
    .filter(|profile| profile.members.len() >= 2)
    .enumerate()
    .map(|(index, profile)| ExtractionProfilePriorRecord {
        prior_id: format!("extraction_profile_prior_{:04}", index + 1),
        cluster_signature: profile.cluster_signature.into_iter().collect(),
        fired_extractors: profile
            .fired_extractors
            .into_iter()
            .map(|support| ExtractionProfileExtractorSupportRecord {
                extractor_name: support.extractor_name,
                member_support: support.member_support,
            })
            .collect(),
        support_count: profile.members.len(),
        supporting_document_keys: profile.members,
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

#[expect(
    clippy::too_many_arguments,
    reason = "prior harvesting keeps the source evidence, document scope, and accumulator explicit"
)]
fn harvest_temporal_language_phrase(
    source_text: &str,
    cycle_window: Option<crate::ir::semantic::CycleWindowRecord>,
    actor_grounded: bool,
    handshake_completion: bool,
    automation_confidence: AutomationConfidence,
    document_key: &str,
    prior_scope: PriorScope,
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
        prior_scope,
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

fn parse_table_kind(value: &str) -> TableKind {
    match value {
        "signal_description" => TableKind::SignalDescription,
        "encoding" => TableKind::Encoding,
        "register_map" => TableKind::RegisterMap,
        "timing_parameter" => TableKind::TimingParameter,
        "feature_matrix" => TableKind::FeatureMatrix,
        _ => TableKind::Unknown,
    }
}

fn parse_diagram_kind(value: &str) -> DiagramKind {
    match value {
        "timing_diagram" => DiagramKind::TimingDiagram,
        "state_machine_diagram" => DiagramKind::StateMachineDiagram,
        "block_diagram" => DiagramKind::BlockDiagram,
        "register_bitfield" => DiagramKind::RegisterBitfield,
        "truth_table" => DiagramKind::TruthTable,
        "flow_chart" => DiagramKind::FlowChart,
        _ => DiagramKind::Unknown,
    }
}

fn parse_visual_asset_kind(value: &str) -> VisualAssetKind {
    match value {
        "figure" => VisualAssetKind::Figure,
        "diagram" => VisualAssetKind::Diagram,
        "chart" => VisualAssetKind::Chart,
        "table_region" => VisualAssetKind::TableRegion,
        "formula_region" => VisualAssetKind::FormulaRegion,
        "screenshot" => VisualAssetKind::Screenshot,
        _ => VisualAssetKind::Unknown,
    }
}

fn parse_negative_knowledge_kind(value: &str) -> NegativeKnowledgeKind {
    match value {
        "signal_polarity_conflict" => NegativeKnowledgeKind::SignalPolarityConflict,
        "temporal_value_conflict" => NegativeKnowledgeKind::TemporalValueConflict,
        "interface_signal_conflict" => NegativeKnowledgeKind::InterfaceSignalConflict,
        "signal_connectivity_conflict" => NegativeKnowledgeKind::SignalConnectivityConflict,
        "residual_decision" => NegativeKnowledgeKind::ResidualDecision,
        _ => NegativeKnowledgeKind::SignalSemanticConflict,
    }
}

fn table_kind_key(table_kind: TableKind) -> &'static str {
    match table_kind {
        TableKind::SignalDescription => "signal_description",
        TableKind::Encoding => "encoding",
        TableKind::RegisterMap => "register_map",
        TableKind::TimingParameter => "timing_parameter",
        TableKind::FeatureMatrix => "feature_matrix",
        TableKind::Unknown => "unknown",
    }
}

fn diagram_kind_key(diagram_kind: DiagramKind) -> &'static str {
    match diagram_kind {
        DiagramKind::TimingDiagram => "timing_diagram",
        DiagramKind::StateMachineDiagram => "state_machine_diagram",
        DiagramKind::BlockDiagram => "block_diagram",
        DiagramKind::RegisterBitfield => "register_bitfield",
        DiagramKind::TruthTable => "truth_table",
        DiagramKind::FlowChart => "flow_chart",
        DiagramKind::Unknown => "unknown",
    }
}

fn visual_asset_kind_key(asset_kind: VisualAssetKind) -> &'static str {
    match asset_kind {
        VisualAssetKind::Figure => "figure",
        VisualAssetKind::Diagram => "diagram",
        VisualAssetKind::Chart => "chart",
        VisualAssetKind::TableRegion => "table_region",
        VisualAssetKind::FormulaRegion => "formula_region",
        VisualAssetKind::Screenshot => "screenshot",
        VisualAssetKind::Unknown => "unknown",
    }
}

fn infer_table_shape_prior_confidence(table: &StructuredTableRecord) -> AutomationConfidence {
    if table
        .caption_text
        .as_deref()
        .map(|caption| !caption.trim().is_empty())
        .unwrap_or(false)
    {
        AutomationConfidence::High
    } else if !table.header_rows.is_empty() {
        AutomationConfidence::Medium
    } else {
        AutomationConfidence::Low
    }
}

fn infer_visual_motif_prior_confidence(
    diagram_kind: DiagramKind,
    has_caption_phrase: bool,
) -> AutomationConfidence {
    if !matches!(diagram_kind, DiagramKind::Unknown) && has_caption_phrase {
        AutomationConfidence::High
    } else if !matches!(diagram_kind, DiagramKind::Unknown) {
        AutomationConfidence::Medium
    } else {
        AutomationConfidence::Low
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
    token.parse::<u32>().ok().or(match token {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use crate::ir::intent::{
        IntentActor, IntentArtifactLayout, IntentDocumentIdentity, IntentIdentity,
    };
    use crate::ir::semantic::{
        ActorPortRecord, CycleWindowRecord, InterfaceRecord, InterfaceSignalDirection,
        InterfaceSignalRecord, InterfaceSignalSemanticArbitrationRecord,
        InterfaceSignalSemanticConsensusRecord, InterfaceSignalSemanticObservationRecord,
        SemanticArbitrationDecisionBasis, TemporalRuleRecord,
    };
    use crate::ir::source::{
        DiagramKind, RelationKind, SourceIr, StructuredTableCellRecord, StructuredTableRecord,
        TableKind, ValidationFindingRecord, ValidationMetricRecord, VisualAsset, VisualAssetKind,
    };

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
            infrastructure_signals: Vec::new(),
            interface_signal_conflicts: Vec::new(),
            signal_connectivity_conflicts: Vec::new(),
            signal_polarities: Vec::new(),
            signal_polarity_conflicts: Vec::new(),
            signal_semantic_conflicts: Vec::new(),
            interfaces: Vec::new(),
            serial_frame_fields: Vec::new(),
            protocol_operations: Vec::new(),
            protocol_states: Vec::new(),
            interface_edge_timings: Vec::new(),
            system_contract: None,
            behaviors: Vec::new(),
            constraints: Vec::new(),
            assumptions: Vec::new(),
            regular_states: Vec::new(),
            state_transitions: Vec::new(),
            symbol_definitions: Vec::new(),
            control_blocks: Vec::new(),
            explicit_modules: Vec::new(),
            explicit_tops: Vec::new(),
            register_records: Vec::new(),
            timing_constraints: Vec::new(),
            temporal_rules: Vec::new(),
            actor_contracts: Vec::new(),
            constrained_extraction_stats: None,
            protocol_graph: Default::default(),
            fidelity_findings: Vec::new(),
            temporal_conflicts: Vec::new(),
            signal_constraints: Vec::new(),
            conditional_rules: Vec::new(),
            transactions: Vec::new(),
            actor_drive_relations: Vec::new(),
            actor_sample_relations: Vec::new(),
            actor_trigger_relations: Vec::new(),
            actor_temporal_dependencies: Vec::new(),
            temporal_invariants: Vec::new(),
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
                resolved_polarity: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                    candidate_count: 1,
                    leading_role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                    leading_evidence_weight: 3,
                    leading_prior_reliability_adjustment: 0,
                    leading_arbitration_weight: 3,
                    runner_up_role: None,
                    runner_up_evidence_weight: None,
                    runner_up_prior_reliability_adjustment: None,
                    runner_up_arbitration_weight: None,
                    margin_over_runner_up: None,
                    decisive: true,
                    decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
                }),
                resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeReadyLike),
                semantic_grounding_strength: Some(SemanticGroundingStrength::CrossModality),
                semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                    role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                    grounding_strength: SemanticGroundingStrength::CrossModality,
                    supporting_source_kinds: vec![SignalSemanticHintSourceKind::ProseStatement],
                    supporting_observation_count: 2,
                    automation_confidence: AutomationConfidence::High,
                    prior_reliability_adjustment: 0,
                    prior_guided: false,
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
                supporting_table_ids: Vec::new(),
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
        let mut semantic_modality_reliability_priors = BTreeMap::new();
        let mut temporal_priors = BTreeMap::new();
        harvest_semantic_priors(&intent_ir, PriorScope::Global, &mut semantic_priors);
        harvest_semantic_modality_reliability_priors(
            &intent_ir,
            PriorScope::Global,
            &mut semantic_modality_reliability_priors,
        );
        harvest_temporal_priors(&intent_ir, PriorScope::Global, &mut temporal_priors);

        let semantic_records = materialize_semantic_priors(semantic_priors);
        let semantic_modality_reliability_records =
            materialize_semantic_modality_reliability_priors(semantic_modality_reliability_priors);
        let temporal_records = materialize_temporal_priors(temporal_priors);

        assert_eq!(semantic_records.len(), 1);
        assert_eq!(
            semantic_records[0].normalized_phrase,
            "<signal> can accept the transfer"
        );
        assert_eq!(semantic_records[0].prior_scope, PriorScope::Global);
        assert_eq!(semantic_records[0].support_count, 1);
        assert!(!semantic_records[0].supporting_document_keys.is_empty());
        assert_eq!(
            semantic_records[0].strongest_automation_confidence,
            AutomationConfidence::High
        );
        assert_eq!(semantic_modality_reliability_records.len(), 1);
        assert_eq!(
            semantic_modality_reliability_records[0].role,
            InterfaceSignalSemanticRole::HandshakeReadyLike
        );
        assert_eq!(
            semantic_modality_reliability_records[0].source_kind,
            SignalSemanticHintSourceKind::ProseStatement
        );
        assert_eq!(
            semantic_modality_reliability_records[0].strongest_grounding_strength,
            SemanticGroundingStrength::CrossModality
        );
        assert_eq!(
            semantic_modality_reliability_records[0].strongest_automation_confidence,
            AutomationConfidence::High
        );
        assert!(
            !semantic_modality_reliability_records[0]
                .supporting_document_keys
                .is_empty()
        );

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
        assert_eq!(
            temporal_records[0].strongest_automation_confidence,
            AutomationConfidence::High
        );
        assert!(!temporal_records[0].supporting_document_keys.is_empty());
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
                    resolved_polarity: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                        leading_evidence_weight: 3,
                        leading_prior_reliability_adjustment: 0,
                        leading_arbitration_weight: 3,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        runner_up_prior_reliability_adjustment: None,
                        runner_up_arbitration_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                        decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
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
                        prior_reliability_adjustment: 0,
                        prior_guided: false,
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
                    supporting_table_ids: vec!["table_1".to_string()],
                    automation_confidence: AutomationConfidence::High,
                },
                InterfaceSignalRecord {
                    signal_name: "PREADY".to_string(),
                    direction_hint: Some(InterfaceSignalDirection::Input),
                    width_hint: None,
                    resolved_polarity: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                        leading_evidence_weight: 3,
                        leading_prior_reliability_adjustment: 0,
                        leading_arbitration_weight: 3,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        runner_up_prior_reliability_adjustment: None,
                        runner_up_arbitration_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                        decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
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
                        prior_reliability_adjustment: 0,
                        prior_guided: false,
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
                    supporting_table_ids: Vec::new(),
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
        harvest_actor_taxonomy_priors(&intent_ir, PriorScope::Global, &mut actor_taxonomy_priors);

        let records = materialize_actor_taxonomy_priors(actor_taxonomy_priors);
        assert_eq!(records.len(), 2);
        assert!(
            records
                .iter()
                .all(|record| !record.supporting_document_keys.is_empty())
        );
        assert!(records.iter().any(|record| {
            record.normalized_actor_term == "requester"
                && record.taxonomy_role == ActorTaxonomyRole::RequesterLike
                && record.strongest_automation_confidence == AutomationConfidence::High
        }));
        assert!(records.iter().any(|record| {
            record.normalized_actor_term == "completer"
                && record.taxonomy_role == ActorTaxonomyRole::CompleterLike
                && record.strongest_grounding_strength == SemanticGroundingStrength::CrossModality
                && record.strongest_automation_confidence == AutomationConfidence::High
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
        harvest_actor_taxonomy_priors(&intent_ir, PriorScope::Global, &mut actor_taxonomy_priors);

        let records = materialize_actor_taxonomy_priors(actor_taxonomy_priors);
        assert_eq!(records.len(), 1);
        assert!(!records[0].supporting_document_keys.is_empty());
        assert_eq!(records[0].normalized_actor_term, "requester");
        assert_eq!(records[0].taxonomy_role, ActorTaxonomyRole::RequesterLike);
        assert_eq!(
            records[0].strongest_grounding_strength,
            SemanticGroundingStrength::SingleSource
        );
        assert_eq!(
            records[0].strongest_automation_confidence,
            AutomationConfidence::Medium
        );
    }

    #[test]
    fn learn_priors_skips_payload_like_actor_terms() {
        let mut intent_ir = base_intent_ir("doc_payload_actor", "doc_payload_actor");
        intent_ir.actors = vec![IntentActor {
            actor_id: "actor_payload".to_string(),
            actor_name: Some("control information".to_string()),
            responsibilities: Vec::new(),
            supporting_actor_ids: Vec::new(),
        }];
        intent_ir.interfaces = vec![InterfaceRecord {
            interface_id: "if_payload".to_string(),
            signals: vec!["TVALID".to_string()],
            signal_records: vec![InterfaceSignalRecord {
                signal_name: "TVALID".to_string(),
                direction_hint: Some(InterfaceSignalDirection::Output),
                width_hint: None,
                resolved_polarity: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                    candidate_count: 1,
                    leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    leading_evidence_weight: 3,
                    leading_prior_reliability_adjustment: 0,
                    leading_arbitration_weight: 3,
                    runner_up_role: None,
                    runner_up_evidence_weight: None,
                    runner_up_prior_reliability_adjustment: None,
                    runner_up_arbitration_weight: None,
                    margin_over_runner_up: None,
                    decisive: true,
                    decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
                }),
                resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeValidLike),
                semantic_grounding_strength: Some(SemanticGroundingStrength::SingleSource),
                semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                    role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    grounding_strength: SemanticGroundingStrength::SingleSource,
                    supporting_source_kinds: vec![SignalSemanticHintSourceKind::ProseStatement],
                    supporting_observation_count: 1,
                    automation_confidence: AutomationConfidence::Medium,
                    prior_reliability_adjustment: 0,
                    prior_guided: false,
                    alias_dependent: false,
                }),
                semantic_observations: vec![InterfaceSignalSemanticObservationRecord {
                    semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                    source_kind: SignalSemanticHintSourceKind::ProseStatement,
                    source_text: "Control information indicates a transfer request.".to_string(),
                    supporting_statement_ids: vec!["stmt_payload".to_string()],
                    supporting_table_ids: Vec::new(),
                    supporting_visual_evidence_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::Medium,
                }],
                supporting_statement_ids: Vec::new(),
                supporting_table_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            }],
            supporting_statement_ids: Vec::new(),
        }];
        intent_ir.actor_ports = vec![ActorPortRecord {
            actor_id: "actor_payload".to_string(),
            actor_name: "control information".to_string(),
            signal_name: "TVALID".to_string(),
            direction: ActorRelativeDirection::Output,
            relation_basis: vec![RelationKind::Drives],
            width_hint: None,
            source_statement_ids: vec!["stmt_payload".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }];

        let mut actor_taxonomy_priors = BTreeMap::new();
        harvest_actor_taxonomy_priors(&intent_ir, PriorScope::Global, &mut actor_taxonomy_priors);

        let records = materialize_actor_taxonomy_priors(actor_taxonomy_priors);
        assert!(
            records.is_empty(),
            "payload-like actor terms must not be harvested into actor-taxonomy priors, got: {:?}",
            records
        );
    }

    #[test]
    fn learn_priors_harvests_table_shape_priors_from_source_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source_path = tempdir.path().join("table_shape_fixture.md");
        fs::write(&source_path, "# Table Shape Fixture\n")?;

        let mut source_ir = SourceIr::build(&source_path, &tempdir.path().join("generated"))?;
        source_ir.structured_tables = vec![StructuredTableRecord {
            table_id: "table_0001".to_string(),
            asset_id: "table_0001".to_string(),
            page_id: Some("page_0001".to_string()),
            caption_text: Some("Table 1 Interface signals".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                StructuredTableCellRecord {
                    text: "Name".to_string(),
                    row_span: 1,
                    col_span: 1,
                    is_header: true,
                },
                StructuredTableCellRecord {
                    text: "Direction".to_string(),
                    row_span: 1,
                    col_span: 1,
                    is_header: true,
                },
                StructuredTableCellRecord {
                    text: "Width".to_string(),
                    row_span: 1,
                    col_span: 1,
                    is_header: true,
                },
            ]],
            body_rows: Vec::new(),
            row_count: 0,
            col_count: 3,
        }];

        let mut table_shape_priors = BTreeMap::new();
        harvest_table_shape_priors_from_source_ir(
            &source_ir,
            "fixture_doc",
            PriorScope::Global,
            &mut table_shape_priors,
        );

        let records = materialize_table_shape_priors(table_shape_priors);
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].normalized_header_signature,
            "name | direction | width"
        );
        assert_eq!(records[0].table_kind, TableKind::SignalDescription);
        assert_eq!(records[0].support_count, 1);
        assert!(!records[0].supporting_document_keys.is_empty());
        assert_eq!(
            records[0].strongest_automation_confidence,
            AutomationConfidence::High
        );

        Ok(())
    }

    #[test]
    fn learn_priors_harvests_visual_motif_and_negative_knowledge_priors() -> Result<()> {
        let tempdir = tempdir()?;
        let source_path = tempdir.path().join("visual_motif_fixture.md");
        fs::write(&source_path, "# Visual Motif Fixture\n")?;

        let mut source_ir = SourceIr::build(&source_path, &tempdir.path().join("generated"))?;
        source_ir.visual_assets = vec![VisualAsset {
            asset_id: "asset_timing".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("XREQ timing diagram".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: DiagramKind::TimingDiagram,
        }];

        let mut intent_ir = base_intent_ir("visual_negative_doc", "visual_negative_doc");
        intent_ir.interfaces = vec![InterfaceRecord {
            interface_id: "if_visual".to_string(),
            signals: vec!["XREQ".to_string()],
            signal_records: Vec::new(),
            supporting_statement_ids: Vec::new(),
        }];
        intent_ir.signal_semantic_conflicts =
            vec![crate::ir::evidence::SignalSemanticConflictRecord {
                conflict_id: "semantic_conflict_0001".to_string(),
                signal_name: "XREQ".to_string(),
                observations: vec![
                    crate::ir::evidence::SignalSemanticConflictObservationRecord {
                        semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                        source_kind: SignalSemanticHintSourceKind::SignalDescriptionTable,
                        source_text: "XREQ launches a request.".to_string(),
                        supporting_statement_ids: vec!["stmt_table".to_string()],
                        supporting_table_ids: vec!["table_0001".to_string()],
                        supporting_visual_evidence_ids: Vec::new(),
                    },
                    crate::ir::evidence::SignalSemanticConflictObservationRecord {
                        semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                        source_kind: SignalSemanticHintSourceKind::VisualCaption,
                        source_text: "XREQ can accept the transfer.".to_string(),
                        supporting_statement_ids: Vec::new(),
                        supporting_table_ids: Vec::new(),
                        supporting_visual_evidence_ids: vec!["visual_0001".to_string()],
                    },
                ],
                automation_confidence: AutomationConfidence::Medium,
            }];

        let signal_names = collect_signal_names(&intent_ir);
        let actor_names = collect_actor_names(&intent_ir);
        let mut visual_motif_priors = BTreeMap::new();
        harvest_visual_motif_priors_from_source_ir(
            &source_ir,
            &intent_ir.document_identity.document_key,
            PriorScope::Global,
            &signal_names,
            &actor_names,
            &mut visual_motif_priors,
        );

        let mut negative_knowledge_priors = BTreeMap::new();
        harvest_negative_knowledge_priors(
            &intent_ir,
            PriorScope::Global,
            &mut negative_knowledge_priors,
        );

        let visual_records = materialize_visual_motif_priors(visual_motif_priors);
        let negative_records = materialize_negative_knowledge_priors(negative_knowledge_priors);

        assert_eq!(visual_records.len(), 1);
        assert_eq!(visual_records[0].diagram_kind, DiagramKind::TimingDiagram);
        assert_eq!(visual_records[0].asset_kind, VisualAssetKind::Diagram);
        assert_eq!(
            visual_records[0].normalized_caption_phrase.as_deref(),
            Some("<signal> timing diagram")
        );
        assert_eq!(
            visual_records[0].strongest_automation_confidence,
            AutomationConfidence::High
        );
        assert!(!visual_records[0].supporting_document_keys.is_empty());

        assert_eq!(negative_records.len(), 1);
        assert_eq!(
            negative_records[0].knowledge_kind,
            NegativeKnowledgeKind::SignalSemanticConflict
        );
        assert_eq!(
            negative_records[0].normalized_pattern,
            "signal_semantic_conflict:signal_description_table:handshake_valid_like|visual_caption:handshake_ready_like"
        );
        assert_eq!(negative_records[0].support_count, 1);
        assert!(!negative_records[0].supporting_document_keys.is_empty());
        assert_eq!(
            negative_records[0].strongest_automation_confidence,
            AutomationConfidence::Medium
        );

        Ok(())
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
                resolved_polarity: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                    candidate_count: 1,
                    leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    leading_evidence_weight: 1,
                    leading_prior_reliability_adjustment: 0,
                    leading_arbitration_weight: 1,
                    runner_up_role: None,
                    runner_up_evidence_weight: None,
                    runner_up_prior_reliability_adjustment: None,
                    runner_up_arbitration_weight: None,
                    margin_over_runner_up: None,
                    decisive: true,
                    decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
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
                    prior_reliability_adjustment: 0,
                    prior_guided: false,
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
                supporting_table_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            }],
            supporting_statement_ids: Vec::new(),
        }];

        let mut semantic_priors = BTreeMap::new();
        harvest_semantic_priors(&accepted_intent, PriorScope::Global, &mut semantic_priors);
        assert!(semantic_priors.is_empty());
    }

    #[test]
    fn learn_priors_materializes_extraction_profile_priors_from_multi_member_clusters() {
        // Two register-shaped documents cluster; the constraint-shaped document stays a
        // singleton and must NOT become a profile (a cluster of one carries no reusable
        // cross-document pattern). Fired-extractor support is the UNION with honest
        // per-member counts, mirroring `derive_extraction_profiles`.
        let reg_a: DocumentFingerprint = [
            "shape:registers:b2",
            "shape:protocol_states:b0",
            "fired:registers.field_table",
            "fired:registers.prose",
        ]
        .iter()
        .map(|t| t.to_string())
        .collect();
        let reg_b: DocumentFingerprint = [
            "shape:registers:b2",
            "shape:protocol_states:b0",
            "fired:registers.field_table",
        ]
        .iter()
        .map(|t| t.to_string())
        .collect();
        let proto: DocumentFingerprint = ["shape:signal_constraints:b2", "shape:registers:b0"]
            .iter()
            .map(|t| t.to_string())
            .collect();
        let documents = vec![
            ("doc_reg_a".to_string(), reg_a),
            ("doc_proto".to_string(), proto),
            ("doc_reg_b".to_string(), reg_b),
        ];

        let records = materialize_extraction_profile_priors(&documents);

        assert_eq!(records.len(), 1, "only the multi-member cluster persists");
        let record = &records[0];
        assert_eq!(record.prior_id, "extraction_profile_prior_0001");
        assert_eq!(record.support_count, 2);
        assert_eq!(
            record.supporting_document_keys,
            vec!["doc_reg_a", "doc_reg_b"]
        );
        // The signature is the shared intersection — the partially-fired prose extractor
        // is absent from it but preserved in the union with member support 1.
        assert!(
            record
                .cluster_signature
                .contains(&"fired:registers.field_table".to_string())
        );
        assert!(
            !record
                .cluster_signature
                .contains(&"fired:registers.prose".to_string())
        );
        assert_eq!(
            record.fired_extractors,
            vec![
                ExtractionProfileExtractorSupportRecord {
                    extractor_name: "registers.field_table".to_string(),
                    member_support: 2,
                },
                ExtractionProfileExtractorSupportRecord {
                    extractor_name: "registers.prose".to_string(),
                    member_support: 1,
                },
            ]
        );
    }

    #[test]
    fn extraction_profile_priors_empty_when_no_cluster_has_two_members() {
        let a: DocumentFingerprint = ["shape:registers:b2"]
            .iter()
            .map(|t| t.to_string())
            .collect();
        let b: DocumentFingerprint = ["shape:signal_constraints:b2"]
            .iter()
            .map(|t| t.to_string())
            .collect();
        let documents = vec![("doc_a".to_string(), a), ("doc_b".to_string(), b)];
        assert!(materialize_extraction_profile_priors(&documents).is_empty());
    }

    #[test]
    fn run_rejects_missing_artifact_path() {
        let args = crate::cli::LearnPriorsArgs {
            artifacts: vec![std::path::PathBuf::from("/nonexistent/artifact.json")],
            output: std::path::PathBuf::from("/tmp/out.json"),
            dry_run: true,
        };
        let result = run(args);
        assert!(result.is_err());
    }

    #[test]
    fn run_rejects_non_intent_ir_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        // Create an IntentIR JSON with a non-IntentIr stage (SemanticIr) to trigger the stage check
        let artifact_path = tempdir.path().join("intent_ir.json");
        let non_intent = serde_json::json!({
            "schema_version": 1,
            "stage": "semantic_ir",
            "semantic_ir_path": "/tmp/semantic_ir.json",
            "artifact_layout": {
                "artifact_root": "/tmp",
                "intent_ir_path": "/tmp/intent_ir.json"
            },
            "document_identity": {
                "document_key": "test_doc",
                "display_name": "Test Doc"
            },
            "intent_identity": {
                "intent_id": "intent",
                "summary": "summary"
            },
            "actors": [],
            "actor_signal_relations": [],
            "actor_ports": [],
            "signal_connectivity": [],
            "infrastructure_signals": [],
            "interface_signal_conflicts": [],
            "signal_connectivity_conflicts": [],
            "signal_polarities": [],
            "signal_polarity_conflicts": [],
            "signal_semantic_conflicts": [],
            "interfaces": [],
            "system_contract": null,
            "behaviors": [],
            "constraints": [],
            "assumptions": [],
            "regular_states": [],
            "state_transitions": [],
            "symbol_definitions": [],
            "control_blocks": [],
            "explicit_modules": [],
            "explicit_tops": [],
            "register_records": [],
            "timing_constraints": [],
            "temporal_rules": [],
            "temporal_conflicts": [],
            "signal_constraints": [],
            "conditional_rules": [],
            "residual_decisions": [],
            "validation_reports": []
        });
        fs::write(&artifact_path, serde_json::to_string_pretty(&non_intent)?)?;

        let args = crate::cli::LearnPriorsArgs {
            artifacts: vec![artifact_path],
            output: tempdir.path().join("out.json"),
            dry_run: true,
        };
        let result = run(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("IntentIR"));
        Ok(())
    }

    #[test]
    fn learn_priors_writes_repository_relative_source_artifact_paths() -> Result<()> {
        let tempdir = crate::project_data::tempdir()?;
        let semantic_ir_path = tempdir
            .path()
            .join("generated/semantic_ir/spec/semantic_ir.json");
        fs::create_dir_all(semantic_ir_path.parent().expect("semantic parent"))?;
        fs::write(&semantic_ir_path, b"{}")?;

        let mut intent_ir = base_intent_ir("spec", "Spec");
        intent_ir.semantic_ir_path = semantic_ir_path;
        intent_ir.artifact_layout = IntentArtifactLayout {
            artifact_root: tempdir.path().join("generated/intent_ir/spec"),
            intent_ir_path: tempdir
                .path()
                .join("generated/intent_ir/spec/intent_ir.json"),
        };
        intent_ir.write_to_disk()?;

        let output = tempdir
            .path()
            .join("generated/prior_memory/corpus_memory.json");
        run(crate::cli::LearnPriorsArgs {
            artifacts: vec![intent_ir.artifact_layout.intent_ir_path.clone()],
            output: output.clone(),
            dry_run: false,
        })?;

        let stored = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(output)?)?;
        let stored_path = stored["source_artifacts"][0]["artifact_path"]
            .as_str()
            .expect("learned source artifact path");
        assert!(std::path::Path::new(stored_path).is_relative());
        assert!(stored_path.ends_with("generated/intent_ir/spec/intent_ir.json"));
        Ok(())
    }

    #[test]
    fn assess_intent_for_learning_rejects_non_intent_ir_report() {
        let validation_reports = vec![ValidationReportRecord {
            report_id: "report_1".to_string(),
            validated_stage: IrStage::SemanticIr,
            artifact_fingerprint: "fingerprint".to_string(),
            summary: "ok".to_string(),
            overall_score: Some(80),
            grade: Some("GOOD".to_string()),
            metrics: Vec::new(),
            findings: Vec::new(),
        }];

        let assessment = assess_intent_for_learning(&validation_reports);
        assert!(!assessment.accepted);
        assert!(
            assessment
                .skip_reason
                .as_deref()
                .unwrap()
                .contains("instead of intent_ir")
        );
    }

    #[test]
    fn harvest_actor_taxonomy_priors_upgrades_confidence_when_higher() {
        let mut intent_ir = base_intent_ir("doc_upgrade", "doc_upgrade");
        intent_ir.actors = vec![IntentActor {
            actor_id: "actor_requester".to_string(),
            actor_name: Some("Requester".to_string()),
            responsibilities: Vec::new(),
            supporting_actor_ids: Vec::new(),
        }];
        intent_ir.interfaces = vec![InterfaceRecord {
            interface_id: "if_1".to_string(),
            signals: vec!["PSEL".to_string()],
            signal_records: vec![InterfaceSignalRecord {
                signal_name: "PSEL".to_string(),
                direction_hint: Some(InterfaceSignalDirection::Output),
                width_hint: None,
                resolved_polarity: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                    candidate_count: 1,
                    leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    leading_evidence_weight: 3,
                    leading_prior_reliability_adjustment: 0,
                    leading_arbitration_weight: 3,
                    runner_up_role: None,
                    runner_up_evidence_weight: None,
                    runner_up_prior_reliability_adjustment: None,
                    runner_up_arbitration_weight: None,
                    margin_over_runner_up: None,
                    decisive: true,
                    decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
                }),
                resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeValidLike),
                semantic_grounding_strength: Some(SemanticGroundingStrength::MultiSource),
                semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                    role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    grounding_strength: SemanticGroundingStrength::MultiSource,
                    supporting_source_kinds: vec![
                        SignalSemanticHintSourceKind::SignalDescriptionTable,
                    ],
                    supporting_observation_count: 1,
                    automation_confidence: AutomationConfidence::High,
                    prior_reliability_adjustment: 0,
                    prior_guided: false,
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
                supporting_table_ids: vec!["table_1".to_string()],
                automation_confidence: AutomationConfidence::High,
            }],
            supporting_statement_ids: Vec::new(),
        }];
        // First port: Medium confidence
        intent_ir.actor_ports = vec![ActorPortRecord {
            actor_id: "actor_requester".to_string(),
            actor_name: "Requester".to_string(),
            signal_name: "PSEL".to_string(),
            direction: ActorRelativeDirection::Output,
            relation_basis: vec![RelationKind::Drives],
            width_hint: None,
            source_statement_ids: vec!["stmt_psel".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }];

        let mut actor_taxonomy_priors = BTreeMap::new();
        harvest_actor_taxonomy_priors(&intent_ir, PriorScope::Global, &mut actor_taxonomy_priors);

        let records = materialize_actor_taxonomy_priors(actor_taxonomy_priors);
        assert_eq!(records.len(), 1);
        // The consensus has High confidence which should upgrade from Medium
        assert_eq!(
            records[0].strongest_automation_confidence,
            AutomationConfidence::High
        );
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
                    resolved_polarity: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeValidLike,
                        leading_evidence_weight: 1,
                        leading_prior_reliability_adjustment: 0,
                        leading_arbitration_weight: 1,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        runner_up_prior_reliability_adjustment: None,
                        runner_up_arbitration_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                        decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
                    }),
                    resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeValidLike),
                    semantic_grounding_strength: Some(SemanticGroundingStrength::SingleSource),
                    semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                        role: InterfaceSignalSemanticRole::HandshakeValidLike,
                        grounding_strength: SemanticGroundingStrength::SingleSource,
                        supporting_source_kinds: vec![SignalSemanticHintSourceKind::ProseStatement],
                        supporting_observation_count: 1,
                        automation_confidence: AutomationConfidence::Medium,
                        prior_reliability_adjustment: 0,
                        prior_guided: false,
                        alias_dependent: false,
                    }),
                    semantic_observations: Vec::new(),
                    supporting_statement_ids: Vec::new(),
                    supporting_table_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::Medium,
                },
                InterfaceSignalRecord {
                    signal_name: "XREADY".to_string(),
                    direction_hint: Some(InterfaceSignalDirection::Output),
                    width_hint: None,
                    resolved_polarity: None,
                    semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                    semantic_candidates: Vec::new(),
                    semantic_arbitration: Some(InterfaceSignalSemanticArbitrationRecord {
                        candidate_count: 1,
                        leading_role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                        leading_evidence_weight: 1,
                        leading_prior_reliability_adjustment: 0,
                        leading_arbitration_weight: 1,
                        runner_up_role: None,
                        runner_up_evidence_weight: None,
                        runner_up_prior_reliability_adjustment: None,
                        runner_up_arbitration_weight: None,
                        margin_over_runner_up: None,
                        decisive: true,
                        decision_basis: SemanticArbitrationDecisionBasis::SingleCandidate,
                    }),
                    resolved_semantic_role: Some(InterfaceSignalSemanticRole::HandshakeReadyLike),
                    semantic_grounding_strength: Some(SemanticGroundingStrength::SingleSource),
                    semantic_consensus: Some(InterfaceSignalSemanticConsensusRecord {
                        role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                        grounding_strength: SemanticGroundingStrength::SingleSource,
                        supporting_source_kinds: vec![SignalSemanticHintSourceKind::ProseStatement],
                        supporting_observation_count: 1,
                        automation_confidence: AutomationConfidence::Medium,
                        prior_reliability_adjustment: 0,
                        prior_guided: false,
                        alias_dependent: false,
                    }),
                    semantic_observations: Vec::new(),
                    supporting_statement_ids: Vec::new(),
                    supporting_table_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::Medium,
                },
            ],
            supporting_statement_ids: Vec::new(),
        }];

        let mut actor_taxonomy_priors = BTreeMap::new();
        harvest_actor_taxonomy_priors(&intent_ir, PriorScope::Global, &mut actor_taxonomy_priors);
        assert!(actor_taxonomy_priors.is_empty());
    }
}
