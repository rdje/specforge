use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ir::evidence::SignalSemanticHintSourceKind;
use crate::ir::semantic::{
    CycleWindowRecord, InterfaceSignalSemanticRole, SemanticGroundingStrength,
};
use crate::ir::source::AutomationConfidence;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolFamily {
    AmbaApb,
    AmbaAhb,
    AmbaAxi,
    AmbaGeneric,
    Unknown,
}

impl ProtocolFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AmbaApb => "amba_apb",
            Self::AmbaAhb => "amba_ahb",
            Self::AmbaAxi => "amba_axi",
            Self::AmbaGeneric => "amba_generic",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CorpusMemory {
    pub schema_version: u32,
    pub update_policy: CorpusMemoryUpdatePolicyRecord,
    #[serde(default)]
    pub source_artifacts: Vec<PriorSourceArtifactRecord>,
    #[serde(default)]
    pub actor_taxonomy_priors: Vec<ActorTaxonomyPriorRecord>,
    #[serde(default)]
    pub semantic_phrase_priors: Vec<SemanticPhrasePriorRecord>,
    #[serde(default)]
    pub temporal_phrase_priors: Vec<TemporalPhrasePriorRecord>,
}

impl CorpusMemory {
    pub fn semantic_phrase_priors_for(
        &self,
        protocol_family: Option<ProtocolFamily>,
        source_kind: Option<SignalSemanticHintSourceKind>,
        role: Option<InterfaceSignalSemanticRole>,
    ) -> Vec<&SemanticPhrasePriorRecord> {
        self.semantic_phrase_priors
            .iter()
            .filter(|prior| {
                protocol_family
                    .map(|expected| prior.protocol_family == expected)
                    .unwrap_or(true)
                    && source_kind
                        .map(|expected| prior.source_kind == expected)
                        .unwrap_or(true)
                    && role.map(|expected| prior.role == expected).unwrap_or(true)
            })
            .collect()
    }

    pub fn actor_taxonomy_priors_for(
        &self,
        protocol_family: Option<ProtocolFamily>,
        role: Option<ActorTaxonomyRole>,
    ) -> Vec<&ActorTaxonomyPriorRecord> {
        self.actor_taxonomy_priors
            .iter()
            .filter(|prior| {
                protocol_family
                    .map(|expected| prior.protocol_family == expected)
                    .unwrap_or(true)
                    && role
                        .map(|expected| prior.taxonomy_role == expected)
                        .unwrap_or(true)
            })
            .collect()
    }

    pub fn temporal_phrase_priors_for(
        &self,
        protocol_family: Option<ProtocolFamily>,
        requires_cycle_window: bool,
        actor_grounded: Option<bool>,
    ) -> Vec<&TemporalPhrasePriorRecord> {
        self.temporal_phrase_priors
            .iter()
            .filter(|prior| {
                protocol_family
                    .map(|expected| prior.protocol_family == expected)
                    .unwrap_or(true)
                    && (!requires_cycle_window || prior.cycle_window.is_some())
                    && actor_grounded
                        .map(|expected| prior.actor_grounded == expected)
                        .unwrap_or(true)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CorpusMemoryUpdatePolicyRecord {
    pub advisory_only: bool,
    pub requires_validated_intent_ir: bool,
    pub rejects_error_findings: bool,
    pub excludes_alias_dependent_semantic_consensus: bool,
    pub local_grounding_required_for_canonical_promotion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PriorSourceArtifactRecord {
    pub artifact_path: PathBuf,
    pub document_key: String,
    pub display_name: String,
    pub protocol_family: ProtocolFamily,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<String>,
    pub accepted_for_learning: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ActorTaxonomyRole {
    RequesterLike,
    CompleterLike,
}

impl ActorTaxonomyRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequesterLike => "requester_like",
            Self::CompleterLike => "completer_like",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorTaxonomyPriorRecord {
    pub prior_id: String,
    pub normalized_actor_term: String,
    pub taxonomy_role: ActorTaxonomyRole,
    pub protocol_family: ProtocolFamily,
    pub support_count: usize,
    #[serde(default)]
    pub supporting_document_keys: Vec<String>,
    pub strongest_automation_confidence: AutomationConfidence,
    pub strongest_grounding_strength: SemanticGroundingStrength,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticPhrasePriorRecord {
    pub prior_id: String,
    pub normalized_phrase: String,
    pub role: InterfaceSignalSemanticRole,
    pub protocol_family: ProtocolFamily,
    pub source_kind: SignalSemanticHintSourceKind,
    pub support_count: usize,
    #[serde(default)]
    pub supporting_document_keys: Vec<String>,
    pub strongest_automation_confidence: AutomationConfidence,
    pub strongest_grounding_strength: SemanticGroundingStrength,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalPhrasePriorRecord {
    pub prior_id: String,
    pub normalized_phrase: String,
    pub protocol_family: ProtocolFamily,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_window: Option<CycleWindowRecord>,
    pub actor_grounded: bool,
    pub handshake_completion: bool,
    pub support_count: usize,
    #[serde(default)]
    pub supporting_document_keys: Vec<String>,
    pub strongest_automation_confidence: AutomationConfidence,
}
