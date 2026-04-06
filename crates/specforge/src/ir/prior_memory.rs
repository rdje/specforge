use std::collections::BTreeMap;
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

    pub fn infer(document_key: &str, display_name: &str) -> Self {
        let normalized = format!(
            "{} {}",
            document_key.to_ascii_lowercase(),
            display_name.to_ascii_lowercase()
        );
        if normalized.contains("axi") {
            Self::AmbaAxi
        } else if normalized.contains("ahb") {
            Self::AmbaAhb
        } else if normalized.contains("apb") {
            Self::AmbaApb
        } else if normalized.contains("amba") {
            Self::AmbaGeneric
        } else {
            Self::Unknown
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

    pub fn actor_taxonomy_role_for_term(
        &self,
        protocol_family: Option<ProtocolFamily>,
        actor_term: &str,
    ) -> Option<ActorTaxonomyRole> {
        let normalized_term = normalize_actor_term(actor_term);
        if normalized_term.is_empty() {
            return None;
        }

        self.resolve_actor_taxonomy_role(protocol_family, |prior| {
            prior.normalized_actor_term == normalized_term
        })
    }

    pub fn actor_taxonomy_role_in_text(
        &self,
        protocol_family: Option<ProtocolFamily>,
        text: &str,
    ) -> Option<ActorTaxonomyRole> {
        let normalized_text = normalize_actor_term(text);
        if normalized_text.is_empty() {
            return None;
        }

        if let Some(role) = self.actor_taxonomy_role_for_term(protocol_family, &normalized_text) {
            return Some(role);
        }

        self.resolve_actor_taxonomy_role(protocol_family, |prior| {
            normalized_text_contains_term(&normalized_text, &prior.normalized_actor_term)
        })
    }

    pub fn semantic_phrase_role_in_text(
        &self,
        protocol_family: Option<ProtocolFamily>,
        source_kind: SignalSemanticHintSourceKind,
        text: &str,
        signal_names: &std::collections::BTreeSet<String>,
        actor_names: &std::collections::BTreeSet<String>,
    ) -> Option<InterfaceSignalSemanticRole> {
        let normalized_phrase = normalize_prior_phrase(text, signal_names, actor_names);
        if !is_meaningful_prior_phrase(&normalized_phrase) {
            return None;
        }

        self.resolve_semantic_phrase_role(protocol_family, source_kind, |prior| {
            prior.normalized_phrase == normalized_phrase
                || normalized_text_contains_term(&normalized_phrase, &prior.normalized_phrase)
        })
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

    fn resolve_actor_taxonomy_role<F>(
        &self,
        protocol_family: Option<ProtocolFamily>,
        predicate: F,
    ) -> Option<ActorTaxonomyRole>
    where
        F: Fn(&ActorTaxonomyPriorRecord) -> bool,
    {
        for scope in actor_taxonomy_search_scopes(protocol_family) {
            let roles = self
                .actor_taxonomy_priors
                .iter()
                .filter(|prior| {
                    scope
                        .map(|expected| prior.protocol_family == expected)
                        .unwrap_or(true)
                })
                .filter(|prior| predicate(prior))
                .map(|prior| prior.taxonomy_role)
                .collect::<std::collections::BTreeSet<_>>();
            if roles.len() == 1 {
                return roles.into_iter().next();
            }
            if roles.len() > 1 {
                return None;
            }
        }

        None
    }

    fn resolve_semantic_phrase_role<F>(
        &self,
        protocol_family: Option<ProtocolFamily>,
        source_kind: SignalSemanticHintSourceKind,
        predicate: F,
    ) -> Option<InterfaceSignalSemanticRole>
    where
        F: Fn(&SemanticPhrasePriorRecord) -> bool,
    {
        for scope in actor_taxonomy_search_scopes(protocol_family) {
            let mut roles = self
                .semantic_phrase_priors
                .iter()
                .filter(|prior| {
                    scope
                        .map(|expected| prior.protocol_family == expected)
                        .unwrap_or(true)
                })
                .filter(|prior| prior.source_kind == source_kind)
                .filter(|prior| predicate(prior))
                .map(|prior| prior.role)
                .collect::<Vec<_>>();
            roles.sort_by_key(|role| role.as_str());
            roles.dedup();
            if roles.len() == 1 {
                return roles.first().copied();
            }
            if roles.len() > 1 {
                return None;
            }
        }

        None
    }
}

pub fn normalize_actor_term(text: &str) -> String {
    collapse_whitespace(
        text.to_ascii_lowercase()
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    ch
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .trim(),
    )
}

pub fn normalized_text_contains_term(text: &str, term: &str) -> bool {
    let text_tokens = text
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    let term_tokens = term
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();

    if text_tokens.is_empty() || term_tokens.is_empty() || term_tokens.len() > text_tokens.len() {
        return false;
    }

    text_tokens
        .windows(term_tokens.len())
        .any(|window| window == term_tokens.as_slice())
}

pub fn normalize_prior_phrase(
    text: &str,
    signal_names: &std::collections::BTreeSet<String>,
    actor_names: &std::collections::BTreeSet<String>,
) -> String {
    let mut normalized = text.to_ascii_lowercase();
    let mut replacements = signal_names
        .iter()
        .map(|signal_name| (signal_name.to_ascii_lowercase(), "<signal>"))
        .chain(
            actor_names
                .iter()
                .map(|actor_name| (actor_name.to_ascii_lowercase(), "<actor>")),
        )
        .collect::<Vec<_>>();
    replacements.sort_by(|left, right| {
        right
            .0
            .len()
            .cmp(&left.0.len())
            .then_with(|| left.0.cmp(&right.0))
    });

    let mut token_replacements = BTreeMap::new();
    for (term, placeholder) in replacements {
        if !term.contains(' ') {
            token_replacements.insert(term, placeholder);
            continue;
        }
        normalized = replace_term_with_placeholder(&normalized, &term, placeholder);
    }

    normalized = replace_single_token_terms(&normalized, &token_replacements);

    collapse_whitespace(
        normalized
            .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '<' && ch != '>')
            .replace(['\n', '\t'], " ")
            .as_str(),
    )
}

pub fn is_meaningful_prior_phrase(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty()
        && trimmed != "<signal>"
        && trimmed != "<actor>"
        && trimmed.chars().any(|ch| ch.is_ascii_lowercase())
}

fn actor_taxonomy_search_scopes(
    protocol_family: Option<ProtocolFamily>,
) -> Vec<Option<ProtocolFamily>> {
    let mut scopes = Vec::new();
    if let Some(protocol_family) =
        protocol_family.filter(|family| *family != ProtocolFamily::Unknown)
    {
        scopes.push(Some(protocol_family));
        if protocol_family != ProtocolFamily::AmbaGeneric {
            scopes.push(Some(ProtocolFamily::AmbaGeneric));
        }
    }
    scopes.push(None);
    scopes
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn replace_term_with_placeholder(text: &str, term: &str, placeholder: &str) -> String {
    if term.is_empty() {
        return text.to_string();
    }

    let bytes = text.as_bytes();
    let term_bytes = term.as_bytes();
    let mut result = String::with_capacity(text.len());
    let mut index = 0;

    while index < bytes.len() {
        let remaining = &bytes[index..];
        if remaining.len() >= term_bytes.len()
            && remaining[..term_bytes.len()].eq_ignore_ascii_case(term_bytes)
            && is_word_boundary(text, index)
            && is_word_boundary(text, index + term_bytes.len())
        {
            result.push_str(placeholder);
            index += term_bytes.len();
        } else {
            result.push(bytes[index] as char);
            index += 1;
        }
    }

    result
}

fn replace_single_token_terms(text: &str, replacements: &BTreeMap<String, &'static str>) -> String {
    if replacements.is_empty() {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    let mut token = String::new();

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            token.push(ch);
            continue;
        }

        flush_normalized_token(&mut result, &mut token, replacements);
        result.push(ch);
    }

    flush_normalized_token(&mut result, &mut token, replacements);
    result
}

fn flush_normalized_token(
    result: &mut String,
    token: &mut String,
    replacements: &BTreeMap<String, &'static str>,
) {
    if token.is_empty() {
        return;
    }

    if let Some(placeholder) = replacements.get(token) {
        result.push_str(placeholder);
    } else {
        result.push_str(token);
    }
    token.clear();
}

fn is_word_boundary(text: &str, byte_index: usize) -> bool {
    if byte_index == 0 || byte_index >= text.len() {
        return true;
    }
    !text.as_bytes()[byte_index].is_ascii_alphanumeric() && text.as_bytes()[byte_index] != b'_'
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
