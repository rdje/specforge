use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ir::evidence::{
    SignalPolarityConflictRecord, SignalSemanticConflictRecord, SignalSemanticHintSourceKind,
};
use crate::ir::semantic::{
    CycleWindowRecord, InterfaceSignalConflictRecord, InterfaceSignalSemanticRole,
    SemanticGroundingStrength, SignalConnectivityConflictRecord, TemporalConflictRecord, TickPhase,
};
use crate::ir::source::{
    AutomationConfidence, DiagramKind, ResidualDecisionPacket, StructuredTableRecord, TableKind,
    VisualAssetKind,
};

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
    pub semantic_modality_reliability_priors: Vec<SemanticModalityReliabilityPriorRecord>,
    #[serde(default)]
    pub temporal_phrase_priors: Vec<TemporalPhrasePriorRecord>,
    #[serde(default)]
    pub table_shape_priors: Vec<TableShapePriorRecord>,
    #[serde(default)]
    pub visual_motif_priors: Vec<VisualMotifPriorRecord>,
    #[serde(default)]
    pub negative_knowledge_priors: Vec<NegativeKnowledgePriorRecord>,
    #[serde(default)]
    pub extraction_profile_priors: Vec<ExtractionProfilePriorRecord>,
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
        if !is_meaningful_actor_term(&normalized_term) {
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

    pub fn temporal_cycle_window_in_text(
        &self,
        protocol_family: Option<ProtocolFamily>,
        text: &str,
        signal_names: &std::collections::BTreeSet<String>,
        actor_names: &std::collections::BTreeSet<String>,
        actor_grounded: Option<bool>,
        handshake_completion: Option<bool>,
    ) -> Option<CycleWindowRecord> {
        let normalized_phrase = normalize_prior_phrase(text, signal_names, actor_names);
        if !is_meaningful_prior_phrase(&normalized_phrase) {
            return None;
        }

        for scope in protocol_family_exact_or_amba_generic_search_scopes(protocol_family) {
            let cycle_windows = self
                .temporal_phrase_priors
                .iter()
                .filter(|prior| prior.protocol_family == scope)
                .filter(|prior| prior.cycle_window.is_some())
                .filter(|prior| {
                    actor_grounded
                        .map(|expected| prior.actor_grounded == expected)
                        .unwrap_or(true)
                })
                .filter(|prior| {
                    handshake_completion
                        .map(|expected| prior.handshake_completion == expected)
                        .unwrap_or(true)
                })
                .filter(|prior| prior.normalized_phrase == normalized_phrase)
                .filter_map(|prior| {
                    prior
                        .cycle_window
                        .as_ref()
                        .map(|window| (window.min_cycles, window.max_cycles))
                })
                .collect::<std::collections::BTreeSet<_>>();
            if cycle_windows.len() == 1 {
                let (min_cycles, max_cycles) = cycle_windows.iter().next().copied()?;
                return Some(CycleWindowRecord {
                    min_cycles,
                    max_cycles,
                });
            }
            if cycle_windows.len() > 1 {
                return None;
            }
        }

        None
    }

    pub fn semantic_modality_reliability_bonus(
        &self,
        protocol_family: Option<ProtocolFamily>,
        role: InterfaceSignalSemanticRole,
        source_kind: SignalSemanticHintSourceKind,
    ) -> u32 {
        for scope in protocol_family_exact_or_amba_generic_search_scopes(protocol_family) {
            let bonus = self
                .semantic_modality_reliability_priors
                .iter()
                .filter(|prior| prior.protocol_family == scope)
                .filter(|prior| prior.role == role)
                .filter(|prior| prior.source_kind == source_kind)
                .map(semantic_modality_reliability_prior_bonus)
                .max();
            if let Some(bonus) = bonus.filter(|bonus| *bonus > 0) {
                return bonus;
            }
        }

        0
    }

    pub fn table_shape_priors_for(
        &self,
        protocol_family: Option<ProtocolFamily>,
        table_kind: Option<TableKind>,
    ) -> Vec<&TableShapePriorRecord> {
        self.table_shape_priors
            .iter()
            .filter(|prior| {
                protocol_family
                    .map(|expected| prior.protocol_family == expected)
                    .unwrap_or(true)
                    && table_kind
                        .map(|expected| prior.table_kind == expected)
                        .unwrap_or(true)
            })
            .collect()
    }

    pub fn visual_motif_priors_for(
        &self,
        protocol_family: Option<ProtocolFamily>,
        diagram_kind: Option<DiagramKind>,
    ) -> Vec<&VisualMotifPriorRecord> {
        self.visual_motif_priors
            .iter()
            .filter(|prior| {
                protocol_family
                    .map(|expected| prior.protocol_family == expected)
                    .unwrap_or(true)
                    && diagram_kind
                        .map(|expected| prior.diagram_kind == expected)
                        .unwrap_or(true)
            })
            .collect()
    }

    pub fn negative_knowledge_priors_for(
        &self,
        protocol_family: Option<ProtocolFamily>,
        knowledge_kind: Option<NegativeKnowledgeKind>,
    ) -> Vec<&NegativeKnowledgePriorRecord> {
        self.negative_knowledge_priors
            .iter()
            .filter(|prior| {
                protocol_family
                    .map(|expected| prior.protocol_family == expected)
                    .unwrap_or(true)
                    && knowledge_kind
                        .map(|expected| prior.knowledge_kind == expected)
                        .unwrap_or(true)
            })
            .collect()
    }

    pub fn negative_knowledge_pattern_is_known(
        &self,
        protocol_family: Option<ProtocolFamily>,
        knowledge_kind: NegativeKnowledgeKind,
        normalized_pattern: &str,
    ) -> bool {
        if normalized_pattern.trim().is_empty() {
            return false;
        }

        for scope in protocol_family_exact_or_amba_generic_search_scopes(protocol_family) {
            if self.negative_knowledge_priors.iter().any(|prior| {
                prior.protocol_family == scope
                    && prior.knowledge_kind == knowledge_kind
                    && prior.normalized_pattern == normalized_pattern
            }) {
                return true;
            }
        }

        false
    }

    pub fn diagram_kind_for_visual_caption(
        &self,
        protocol_family: Option<ProtocolFamily>,
        caption_text: &str,
        signal_names: &BTreeSet<String>,
        actor_names: &BTreeSet<String>,
    ) -> Option<DiagramKind> {
        let normalized_caption = normalize_prior_phrase(caption_text, signal_names, actor_names);
        if !is_meaningful_prior_phrase(&normalized_caption) {
            return None;
        }

        for scope in protocol_family_exact_or_amba_generic_search_scopes(protocol_family) {
            let mut diagram_kinds = self
                .visual_motif_priors
                .iter()
                .filter(|prior| prior.protocol_family == scope)
                .filter(|prior| {
                    prior.normalized_caption_phrase.as_deref() == Some(&normalized_caption)
                })
                .filter(|prior| !matches!(prior.diagram_kind, DiagramKind::Unknown))
                .map(|prior| prior.diagram_kind)
                .collect::<Vec<_>>();
            diagram_kinds.sort_by_key(|diagram_kind| diagram_kind_key(*diagram_kind));
            diagram_kinds.dedup_by_key(|diagram_kind| diagram_kind_key(*diagram_kind));
            if diagram_kinds.len() == 1 {
                return diagram_kinds.first().copied();
            }
            if diagram_kinds.len() > 1 {
                return None;
            }
        }

        None
    }

    pub fn table_kind_for_structured_table(
        &self,
        protocol_family: Option<ProtocolFamily>,
        table: &StructuredTableRecord,
    ) -> Option<TableKind> {
        let normalized_header_signature = normalize_table_header_signature(table)?;

        for scope in protocol_family_exact_or_amba_generic_search_scopes(protocol_family) {
            let mut table_kinds = self
                .table_shape_priors
                .iter()
                .filter(|prior| prior.protocol_family == scope)
                .filter(|prior| prior.normalized_header_signature == normalized_header_signature)
                .map(|prior| prior.table_kind)
                .collect::<Vec<_>>();
            table_kinds.sort_by_key(|table_kind| table_kind_key(*table_kind));
            table_kinds.dedup_by_key(|table_kind| table_kind_key(*table_kind));
            if table_kinds.len() == 1 {
                return table_kinds.first().copied();
            }
            if table_kinds.len() > 1 {
                return None;
            }
        }

        None
    }

    /// Advisory extraction-profile lookup (`CORPUS-PATTERN-REUSE.3b.2`): the learned
    /// profiles whose cluster signature the given document fingerprint fully exhibits
    /// (signature ⊆ fingerprint). The signature is the ADR-0006-safe derived structural
    /// key (see [`crate::ir::corpus_cluster`]) — never a vendor or protocol name — so
    /// this family is deliberately NOT scoped by [`ProtocolFamily`]. Profiles with an
    /// empty signature would match every document and are skipped as meaningless.
    /// Consumption stays bounded by the activate-only contract (`.3b.3`): a matched
    /// profile may only activate an opt-in extractor, never suppress a default-on one.
    pub fn extraction_profile_priors_for(
        &self,
        document_fingerprint: &BTreeSet<String>,
    ) -> Vec<&ExtractionProfilePriorRecord> {
        self.extraction_profile_priors
            .iter()
            .filter(|prior| !prior.cluster_signature.is_empty())
            .filter(|prior| {
                prior
                    .cluster_signature
                    .iter()
                    .all(|feature| document_fingerprint.contains(feature))
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
        for scope in protocol_family_exact_or_amba_generic_search_scopes(protocol_family) {
            let roles = self
                .actor_taxonomy_priors
                .iter()
                .filter(|prior| prior.protocol_family == scope)
                .filter(|prior| is_meaningful_actor_term(&prior.normalized_actor_term))
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
        for scope in protocol_family_exact_or_amba_generic_search_scopes(protocol_family) {
            let mut roles = self
                .semantic_phrase_priors
                .iter()
                .filter(|prior| prior.protocol_family == scope)
                .filter(|prior| prior.source_kind == source_kind)
                .filter(|prior| is_meaningful_prior_phrase(&prior.normalized_phrase))
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

pub fn signal_semantic_conflict_negative_knowledge_pattern(
    conflict: &SignalSemanticConflictRecord,
) -> Option<String> {
    let mut signatures = conflict
        .observations
        .iter()
        .map(|observation| {
            let mut tags = observation
                .semantic_tags
                .iter()
                .map(|tag| tag.as_str())
                .collect::<Vec<_>>();
            tags.sort();
            tags.dedup();
            format!("{}:{}", observation.source_kind.as_str(), tags.join("+"))
        })
        .filter(|signature| !signature.ends_with(':'))
        .collect::<Vec<_>>();
    signatures.sort();
    signatures.dedup();
    (signatures.len() >= 2).then(|| format!("signal_semantic_conflict:{}", signatures.join("|")))
}

pub fn temporal_value_conflict_negative_knowledge_pattern(
    conflict: &TemporalConflictRecord,
) -> Option<String> {
    let mut values = conflict
        .conflicting_values
        .iter()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    (values.len() >= 2).then(|| {
        format!(
            "temporal_value_conflict:phase={};values={}",
            tick_phase_key(conflict.phase),
            values.join("|")
        )
    })
}

pub fn signal_polarity_conflict_negative_knowledge_pattern(
    conflict: &SignalPolarityConflictRecord,
) -> Option<String> {
    let mut signatures = conflict
        .observations
        .iter()
        .map(|observation| {
            format!(
                "{}:{}",
                observation.source_kind.as_str(),
                observation.polarity.as_str()
            )
        })
        .collect::<Vec<_>>();
    signatures.sort();
    signatures.dedup();
    (signatures.len() >= 2).then(|| format!("signal_polarity_conflict:{}", signatures.join("|")))
}

pub fn interface_signal_conflict_negative_knowledge_pattern(
    conflict: &InterfaceSignalConflictRecord,
) -> Option<String> {
    Some(format!(
        "interface_signal_conflict:{}",
        conflict.conflict_kind.as_str()
    ))
}

pub fn signal_connectivity_conflict_negative_knowledge_pattern(
    conflict: &SignalConnectivityConflictRecord,
) -> Option<String> {
    Some(format!(
        "signal_connectivity_conflict:{}",
        conflict.conflict_kind.as_str()
    ))
}

pub fn residual_decision_negative_knowledge_pattern(
    residual: &ResidualDecisionPacket,
) -> Option<String> {
    let packet_id = residual.packet_id.trim();
    (!packet_id.is_empty()).then(|| format!("residual_decision:{packet_id}"))
}

fn tick_phase_key(phase: TickPhase) -> &'static str {
    match phase {
        TickPhase::PreTick => "pre_tick",
        TickPhase::PostTick => "post_tick",
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

pub fn is_meaningful_actor_term(text: &str) -> bool {
    let normalized = normalize_actor_term(text);
    if normalized.is_empty() {
        return false;
    }

    if matches!(
        normalized.as_str(),
        "input"
            | "output"
            | "inout"
            | "bidirectional"
            | "bidir"
            | "external"
            | "tie off"
            | "tieoff"
            | "reserved"
            | "n a"
            | "na"
            | "none"
            | "tbd"
            | "see note"
            | "information"
            | "control information"
            | "status information"
            | "data"
            | "payload"
            | "data bytes"
            | "control bytes"
            | "byte lanes"
            | "transfer"
            | "transaction"
            | "mixture"
            | "mixture of"
    ) {
        return false;
    }

    if normalized.contains("clock")
        || normalized.contains("reset")
        || normalized.contains("global")
        || normalized.contains("system bus")
        || normalized.contains("power")
        || normalized.contains("ground")
        || normalized.contains("supply")
        || normalized.contains("vdd")
        || normalized.contains("vss")
    {
        return false;
    }

    true
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

pub fn normalize_table_header_signature(table: &StructuredTableRecord) -> Option<String> {
    let row_signatures = table
        .header_rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| normalize_table_header_cell(&cell.text))
                .filter(|cell| !cell.is_empty())
                .collect::<Vec<_>>()
                .join(" | ")
        })
        .filter(|row| !row.is_empty())
        .collect::<Vec<_>>();

    (!row_signatures.is_empty()).then_some(row_signatures.join(" || "))
}

pub fn is_meaningful_prior_phrase(text: &str) -> bool {
    let trimmed = text.trim();
    let meaningful_terms = trimmed
        .split_whitespace()
        .map(|term| {
            term.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '<' && ch != '>')
        })
        .filter(|term| !term.is_empty() && *term != "<signal>" && *term != "<actor>")
        .count();

    !trimmed.is_empty()
        && trimmed != "<signal>"
        && trimmed != "<actor>"
        && trimmed.chars().any(|ch| ch.is_ascii_lowercase())
        && meaningful_terms >= 2
}

fn protocol_family_exact_or_amba_generic_search_scopes(
    protocol_family: Option<ProtocolFamily>,
) -> Vec<ProtocolFamily> {
    let mut scopes = Vec::new();
    if let Some(protocol_family) =
        protocol_family.filter(|family| *family != ProtocolFamily::Unknown)
    {
        scopes.push(protocol_family);
        if protocol_family != ProtocolFamily::AmbaGeneric {
            scopes.push(ProtocolFamily::AmbaGeneric);
        }
    } else {
        scopes.push(ProtocolFamily::Unknown);
    }
    scopes
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_table_header_cell(text: &str) -> String {
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

fn semantic_modality_reliability_prior_bonus(
    prior: &SemanticModalityReliabilityPriorRecord,
) -> u32 {
    if prior.support_count >= 3
        && matches!(
            prior.strongest_grounding_strength,
            SemanticGroundingStrength::CrossModality
        )
    {
        2
    } else if prior.support_count >= 2
        && !matches!(
            prior.strongest_grounding_strength,
            SemanticGroundingStrength::SingleSource
        )
    {
        1
    } else {
        0
    }
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

// --- PRIOR-DECAY: contested-prior detection (revision-on-contradiction) ---

/// Which prior family a contested key belongs to.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum ContestedPriorFamily {
    ActorTaxonomy,
    SemanticPhrase,
    TableShape,
}

impl ContestedPriorFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            ContestedPriorFamily::ActorTaxonomy => "actor_taxonomy",
            ContestedPriorFamily::SemanticPhrase => "semantic_phrase",
            ContestedPriorFamily::TableShape => "table_shape",
        }
    }
}

/// One competing value for a contested prior key, with its aggregated support.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContestedPriorValue {
    pub value: String,
    pub support_count: usize,
    pub supporting_document_keys: Vec<String>,
}

/// A prior key that two or more documents map, within one protocol family, to
/// DIFFERENT values — a cross-document contradiction the accrete-only harvest
/// never revises (the Parisi revision-on-contradiction gap; `PRIOR-DECAY`).
///
/// Advisory-only: the contest is surfaced (with the strongest-supported value as
/// a hint in `strongest_value`), never auto-resolved. `contested_priors` does not
/// mutate any stored prior or consultation behavior.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContestedPrior {
    pub family: ContestedPriorFamily,
    pub protocol_family: ProtocolFamily,
    pub key: String,
    pub competing_values: Vec<ContestedPriorValue>,
    pub strongest_value: String,
}

impl CorpusMemory {
    /// Detect **contested priors**: keys carried, within one protocol family, by
    /// two or more distinct values across the harvested corpus.
    ///
    /// Read-only — does not mutate stored priors, the harvest, or consultation.
    /// Realizes the Parisi revision-on-contradiction gap (`PRIOR-DECAY`): the
    /// accrete-only harvest never notices when a later validated document maps a
    /// key to a value that contradicts an earlier one. Covers the families with a
    /// single-expected-value-per-key (actor taxonomy, semantic phrase, table
    /// shape); the fuzzier families (temporal/modality/visual, where one key may
    /// legitimately carry several shapes) are out of scope.
    pub fn contested_priors(&self) -> Vec<ContestedPrior> {
        let mut out = Vec::new();
        out.extend(contested_in_family(
            ContestedPriorFamily::ActorTaxonomy,
            self.actor_taxonomy_priors.iter().map(|p| {
                (
                    p.protocol_family,
                    p.normalized_actor_term.clone(),
                    format!("{:?}", p.taxonomy_role),
                    p.support_count,
                    p.supporting_document_keys.clone(),
                )
            }),
        ));
        out.extend(contested_in_family(
            ContestedPriorFamily::SemanticPhrase,
            self.semantic_phrase_priors.iter().map(|p| {
                (
                    p.protocol_family,
                    p.normalized_phrase.clone(),
                    format!("{:?}", p.role),
                    p.support_count,
                    p.supporting_document_keys.clone(),
                )
            }),
        ));
        out.extend(contested_in_family(
            ContestedPriorFamily::TableShape,
            self.table_shape_priors.iter().map(|p| {
                (
                    p.protocol_family,
                    p.normalized_header_signature.clone(),
                    format!("{:?}", p.table_kind),
                    p.support_count,
                    p.supporting_document_keys.clone(),
                )
            }),
        ));
        out
    }
}

/// Group prior rows `(protocol_family, key, value, support, docs)` by
/// `(protocol_family, key)` and emit a `ContestedPrior` for any scope carrying
/// two or more distinct values. Deterministic (BTree ordering; competing values
/// sorted by support desc, ties broken by value asc).
/// Per-scope aggregation: each distinct value -> (total support, backing documents).
type ContestValueAggregates = BTreeMap<String, (usize, BTreeSet<String>)>;

fn contested_in_family<I>(family: ContestedPriorFamily, rows: I) -> Vec<ContestedPrior>
where
    I: Iterator<Item = (ProtocolFamily, String, String, usize, Vec<String>)>,
{
    let mut scopes: BTreeMap<(ProtocolFamily, String), ContestValueAggregates> = BTreeMap::new();
    for (protocol_family, key, value, support, docs) in rows {
        let value_agg = scopes
            .entry((protocol_family, key))
            .or_default()
            .entry(value)
            .or_insert((0, BTreeSet::new()));
        value_agg.0 += support;
        value_agg.1.extend(docs);
    }

    let mut out = Vec::new();
    for ((protocol_family, key), values) in scopes {
        if values.len() < 2 {
            continue; // a single value for the key is settled, not contested
        }
        let mut competing_values: Vec<ContestedPriorValue> = values
            .into_iter()
            .map(|(value, (support_count, docs))| ContestedPriorValue {
                value,
                support_count,
                supporting_document_keys: docs.into_iter().collect(),
            })
            .collect();
        competing_values.sort_by(|a, b| {
            b.support_count
                .cmp(&a.support_count)
                .then_with(|| a.value.cmp(&b.value))
        });
        let strongest_value = competing_values[0].value.clone();
        out.push(ContestedPrior {
            family,
            protocol_family,
            key,
            competing_values,
            strongest_value,
        });
    }
    out
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
pub struct SemanticModalityReliabilityPriorRecord {
    pub prior_id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableShapePriorRecord {
    pub prior_id: String,
    pub normalized_header_signature: String,
    pub table_kind: TableKind,
    pub protocol_family: ProtocolFamily,
    pub support_count: usize,
    #[serde(default)]
    pub supporting_document_keys: Vec<String>,
    pub strongest_automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualMotifPriorRecord {
    pub prior_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_caption_phrase: Option<String>,
    pub diagram_kind: DiagramKind,
    pub asset_kind: VisualAssetKind,
    pub protocol_family: ProtocolFamily,
    pub support_count: usize,
    #[serde(default)]
    pub supporting_document_keys: Vec<String>,
    pub strongest_automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum NegativeKnowledgeKind {
    SignalSemanticConflict,
    SignalPolarityConflict,
    TemporalValueConflict,
    InterfaceSignalConflict,
    SignalConnectivityConflict,
    ResidualDecision,
}

impl NegativeKnowledgeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignalSemanticConflict => "signal_semantic_conflict",
            Self::SignalPolarityConflict => "signal_polarity_conflict",
            Self::TemporalValueConflict => "temporal_value_conflict",
            Self::InterfaceSignalConflict => "interface_signal_conflict",
            Self::SignalConnectivityConflict => "signal_connectivity_conflict",
            Self::ResidualDecision => "residual_decision",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NegativeKnowledgePriorRecord {
    pub prior_id: String,
    pub knowledge_kind: NegativeKnowledgeKind,
    pub normalized_pattern: String,
    pub protocol_family: ProtocolFamily,
    pub support_count: usize,
    #[serde(default)]
    pub supporting_document_keys: Vec<String>,
    pub strongest_automation_confidence: AutomationConfidence,
}

/// One extractor strategy's support within a learned extraction-profile prior: how many
/// of the originating cluster's member documents that strategy fired on (kept ≥ 1 record).
/// The persisted twin of [`crate::ir::corpus_cluster::ProfileExtractorSupport`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtractionProfileExtractorSupportRecord {
    /// The extractor strategy name as recorded by the run manifest, e.g. `registers.field_table`.
    pub extractor_name: String,
    /// How many of the cluster's member documents this extractor fired on.
    pub member_support: usize,
}

/// An advisory per-cluster extraction profile (`CORPUS-PATTERN-REUSE.3b.2`) — the 8th
/// prior family. Harvested (never authored) by `learn-priors`: the accepted artifacts'
/// derived fingerprints are clustered ([`crate::ir::corpus_cluster`]) and each
/// multi-member cluster contributes one profile recording "what tends to work for
/// documents shaped like this". Keyed by the cluster's shared structural signature —
/// ADR-0006-safe derived feature tokens, never a vendor name — and therefore not scoped
/// by [`ProtocolFamily`] (the signature itself is the scope). Advisory-only: the consume
/// side (`.3b.3`) may only ACTIVATE an opt-in extractor on a matching document, never
/// deactivate a default-on one, so a profile adjusts where extraction looks, never what
/// it concludes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtractionProfilePriorRecord {
    pub prior_id: String,
    /// The cluster's shared structural signature (sorted derived feature tokens) — the lookup key.
    pub cluster_signature: Vec<String>,
    /// Extractor strategies that fired on ≥ 1 member, with per-member support, sorted by name.
    /// Honestly empty while the corpus' `extraction_manifest` coverage is still sparse.
    pub fired_extractors: Vec<ExtractionProfileExtractorSupportRecord>,
    /// Number of member documents in the originating cluster (always ≥ 2 — singleton
    /// clusters carry no reusable cross-document pattern and are never harvested).
    pub support_count: usize,
    #[serde(default)]
    pub supporting_document_keys: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::source::StructuredTableCellRecord;

    // is_word_boundary unit tests

    #[test]
    fn word_boundary_at_zero() {
        assert!(is_word_boundary("abc", 0));
    }

    #[test]
    fn word_boundary_at_length() {
        assert!(is_word_boundary("abc", 3));
    }

    #[test]
    fn word_boundary_beyond_length() {
        assert!(is_word_boundary("abc", 10));
    }

    #[test]
    fn word_boundary_at_alphanumeric_is_not_boundary() {
        assert!(!is_word_boundary("abc", 1));
    }

    #[test]
    fn word_boundary_at_underscore_is_not_boundary() {
        assert!(!is_word_boundary("a_b", 1));
    }

    #[test]
    fn word_boundary_at_hyphen_is_boundary() {
        assert!(is_word_boundary("a-b", 1));
    }

    #[test]
    fn word_boundary_at_space_is_boundary() {
        assert!(is_word_boundary("a b", 1));
    }

    #[test]
    fn word_boundary_zero_length_text() {
        assert!(is_word_boundary("", 0));
    }

    // NegativeKnowledgeKind::as_str unit tests

    #[test]
    fn negative_knowledge_kind_as_str_signal_semantic_conflict() {
        assert_eq!(
            NegativeKnowledgeKind::SignalSemanticConflict.as_str(),
            "signal_semantic_conflict"
        );
    }

    #[test]
    fn negative_knowledge_kind_as_str_residual_decision() {
        assert_eq!(
            NegativeKnowledgeKind::ResidualDecision.as_str(),
            "residual_decision"
        );
    }

    // table_kind_key unit tests

    #[test]
    fn table_kind_key_signal_description() {
        assert_eq!(
            table_kind_key(TableKind::SignalDescription),
            "signal_description"
        );
    }

    #[test]
    fn table_kind_key_unknown() {
        assert_eq!(table_kind_key(TableKind::Unknown), "unknown");
    }

    // diagram_kind_key unit tests

    #[test]
    fn diagram_kind_key_timing_diagram() {
        assert_eq!(
            diagram_kind_key(DiagramKind::TimingDiagram),
            "timing_diagram"
        );
    }

    #[test]
    fn diagram_kind_key_truth_table() {
        assert_eq!(diagram_kind_key(DiagramKind::TruthTable), "truth_table");
    }

    // replace_term_with_placeholder unit tests

    #[test]
    fn replace_term_at_start_of_text() {
        // Catches return String::new() and return "xyzzy" at line 813 —
        // term at index 0 where is_word_boundary(text,0)=true.
        let result = replace_term_with_placeholder("bar baz", "bar", "XXX");
        assert_eq!(result, "XXX baz");
    }

    #[test]
    fn replace_term_case_insensitive_at_start() {
        let result = replace_term_with_placeholder("BAR baz", "bar", "XXX");
        assert_eq!(result, "XXX baz");
    }

    #[test]
    fn replace_term_not_at_start_is_not_replaced() {
        // Catches &&→|| at lines 825-827 — "bar" not at index 0,
        // is_word_boundary(text,4) checks 'b' which is alphanumeric → false.
        // The term must NOT be replaced.
        let result = replace_term_with_placeholder("foo bar baz", "bar", "XXX");
        assert_eq!(result, "foo bar baz");
    }

    #[test]
    fn replace_term_does_not_replace_subword() {
        // Catches &&→|| at lines 825-826 — "bar" in "foobar" is not
        // at a word boundary, so it must NOT be replaced.
        let result = replace_term_with_placeholder("foobar baz", "bar", "XXX");
        assert_eq!(result, "foobar baz");
    }

    #[test]
    fn replace_term_empty_term_returns_original() {
        // Catches <→== at line 822 — empty term early-return path.
        let result = replace_term_with_placeholder("hello world", "", "XXX");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn replace_term_not_found_returns_original() {
        // Catches <→> at line 822 — if the loop never enters or scans
        // without matching, the result must equal the original.
        let result = replace_term_with_placeholder("hello world", "xyz", "XXX");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn replace_term_at_start_with_hyphen_prefix_not_replaced() {
        // Catches >=→< at line 824 — "bar" in "-bar" starts at index 1,
        // remaining.len()=3 >= term.len()=3, but is_word_boundary(text,1)
        // checks 'b' → alphanumeric → false → not replaced.
        let result = replace_term_with_placeholder("-bar baz", "bar", "XXX");
        assert_eq!(result, "-bar baz");
    }

    #[test]
    fn replace_term_only_first_at_start_replaced() {
        // Catches +=→-= and +=→*= at lines 830, 833 — first "bar" at index 0
        // is replaced. Second "bar" at index 4 is NOT (is_word_boundary
        // checks 'b'). Index must advance correctly.
        let result = replace_term_with_placeholder("bar bar bar", "bar", "X");
        assert_eq!(result, "X bar bar");
    }

    #[test]
    fn replace_term_not_replaced_when_no_end_boundary() {
        // Catches +→* at line 827 — "bar" at index 0 matches start boundary
        // but char after term ('b' in "barbaz") is alphanumeric → no end
        // boundary. Mutant changes index+len→index*len=0 (always boundary).
        let result = replace_term_with_placeholder("barbaz", "bar", "XXX");
        assert_eq!(result, "barbaz");
    }

    // is_meaningful_actor_term unit tests

    #[test]
    fn meaningful_actor_term_rejects_clock() {
        // Catches ||→&& at line 620 — "clock" alone must trigger return false.
        assert!(!is_meaningful_actor_term("clock signal"));
    }

    #[test]
    fn meaningful_actor_term_rejects_reset() {
        // Catches ||→&& at line 621.
        assert!(!is_meaningful_actor_term("reset controller"));
    }

    #[test]
    fn meaningful_actor_term_rejects_global() {
        // Catches ||→&& at line 622.
        assert!(!is_meaningful_actor_term("global enable"));
    }

    #[test]
    fn meaningful_actor_term_rejects_system_bus() {
        // Catches ||→&& at line 623.
        assert!(!is_meaningful_actor_term("system bus interface"));
    }

    #[test]
    fn meaningful_actor_term_rejects_power() {
        // Catches ||→&& at line 624.
        assert!(!is_meaningful_actor_term("power management"));
    }

    #[test]
    fn meaningful_actor_term_rejects_ground() {
        // Catches ||→&& at line 625.
        assert!(!is_meaningful_actor_term("ground plane"));
    }

    #[test]
    fn meaningful_actor_term_rejects_supply() {
        // Catches ||→&& at line 626.
        assert!(!is_meaningful_actor_term("supply rail"));
    }

    #[test]
    fn meaningful_actor_term_rejects_vdd() {
        // Catches ||→&& at line 627.
        assert!(!is_meaningful_actor_term("vdd rail"));
    }

    #[test]
    fn meaningful_actor_term_rejects_vss() {
        // Catches ||→&& at line 627.
        assert!(!is_meaningful_actor_term("vss rail"));
    }

    #[test]
    fn meaningful_actor_term_accepts_real_actor() {
        let result = is_meaningful_actor_term("dma engine");
        assert!(result);
    }

    // normalized_text_contains_term unit tests

    #[test]
    fn contains_term_empty_text_returns_false() {
        // Catches ||→&& at line 645 — "text_tokens.is_empty() || term_tokens..."
        assert!(!normalized_text_contains_term("", "clock"));
    }

    #[test]
    fn contains_term_empty_term_returns_false() {
        // Catches ||→&& at line 645 — "term_tokens.is_empty() || term_tokens.len() > ..."
        assert!(!normalized_text_contains_term("clock signal", ""));
    }

    #[test]
    fn contains_term_term_longer_than_text_returns_false() {
        assert!(!normalized_text_contains_term("clock", "clock signal"));
    }

    #[test]
    fn contains_term_exact_match_returns_true() {
        assert!(normalized_text_contains_term(
            "clock signal",
            "clock signal"
        ));
    }

    #[test]
    fn contains_term_window_match_returns_true() {
        assert!(normalized_text_contains_term(
            "the clock signal is",
            "clock signal"
        ));
    }

    #[test]
    fn contains_term_no_match_returns_false() {
        assert!(!normalized_text_contains_term("clock signal", "reset"));
    }

    // is_meaningful_prior_phrase unit tests

    #[test]
    fn meaningful_prior_phrase_empty_returns_false() {
        // Catches &&→|| at line 723 — empty must return false.
        assert!(!is_meaningful_prior_phrase(""));
    }

    #[test]
    fn meaningful_prior_phrase_signal_placeholder_returns_false() {
        // Catches &&→|| at line 724 — "<signal>" must not be meaningful.
        assert!(!is_meaningful_prior_phrase("<signal>"));
    }

    #[test]
    fn meaningful_prior_phrase_actor_placeholder_returns_false() {
        // Catches &&→|| at line 725 — "<actor>" must not be meaningful.
        assert!(!is_meaningful_prior_phrase("<actor>"));
    }

    #[test]
    fn meaningful_prior_phrase_no_lowercase_returns_false() {
        // Catches &&→|| at line 726 — text with no lowercase letters is not meaningful.
        assert!(!is_meaningful_prior_phrase("123 DMA VIP 456"));
    }

    #[test]
    fn meaningful_prior_phrase_only_one_meaningful_term_returns_false() {
        // Catches &&→|| at line 727 — need at least 2 meaningful terms.
        assert!(!is_meaningful_prior_phrase("clock"));
    }

    #[test]
    fn meaningful_prior_phrase_two_meaningful_terms_returns_true() {
        // Catches return true at line 714 — two meaningful terms must pass.
        assert!(is_meaningful_prior_phrase("clock signal"));
    }

    #[test]
    fn meaningful_prior_phrase_bracket_trimmed_signal_not_meaningful() {
        // Catches !=→== at lines 718 — "<signal>" with brackets trimmed
        // must still be filtered as placeholder.
        assert!(!is_meaningful_prior_phrase("<signal> something"));
    }

    // normalize_table_header_cell unit tests

    #[test]
    fn normalize_table_header_cell_preserves_underscore() {
        // Catches ==→!= at line 756 — underscore must be preserved.
        assert_eq!(normalize_table_header_cell("signal_name"), "signal_name");
    }

    #[test]
    fn normalize_table_header_cell_replaces_hyphen_with_space() {
        assert_eq!(normalize_table_header_cell("signal-name"), "signal name");
    }

    // semantic_modality_reliability_prior_bonus unit tests

    fn make_reliability_prior(
        support_count: usize,
        grounding_strength: SemanticGroundingStrength,
    ) -> SemanticModalityReliabilityPriorRecord {
        SemanticModalityReliabilityPriorRecord {
            prior_id: "test".into(),
            role: InterfaceSignalSemanticRole::HandshakeValidLike,
            protocol_family: ProtocolFamily::Unknown,
            source_kind: SignalSemanticHintSourceKind::ProseStatement,
            support_count,
            supporting_document_keys: vec![],
            strongest_automation_confidence: AutomationConfidence::Medium,
            strongest_grounding_strength: grounding_strength,
        }
    }

    #[test]
    fn reliability_bonus_cross_modality_with_support_3_returns_2() {
        let prior = make_reliability_prior(3, SemanticGroundingStrength::CrossModality);
        assert_eq!(semantic_modality_reliability_prior_bonus(&prior), 2);
    }

    #[test]
    fn reliability_bonus_multi_source_with_support_2_returns_1() {
        // Catches delete ! at line 801 — MultiSource must NOT match SingleSource.
        let prior = make_reliability_prior(2, SemanticGroundingStrength::MultiSource);
        assert_eq!(semantic_modality_reliability_prior_bonus(&prior), 1);
    }

    #[test]
    fn reliability_bonus_low_support_returns_0() {
        // Catches >=→< at line 800 — support_count < 2 must not trigger bonus 1.
        let prior = make_reliability_prior(1, SemanticGroundingStrength::MultiSource);
        assert_eq!(semantic_modality_reliability_prior_bonus(&prior), 0);
    }

    #[test]
    fn reliability_bonus_high_support_single_source_returns_0() {
        // Catches &&→|| at line 794 — support_count >= 3 but SingleSource
        // (not CrossModality) must return 0, not 2. With || the bonus 2
        // triggers on support_count alone.
        let prior = make_reliability_prior(3, SemanticGroundingStrength::SingleSource);
        assert_eq!(semantic_modality_reliability_prior_bonus(&prior), 0);
    }

    #[test]
    fn reliability_bonus_single_source_with_support_2_returns_0() {
        // Catches &&→|| at line 801 — support_count >= 2 but SingleSource
        // must return 0, not 1 (the !matches! excludes SingleSource).
        let prior = make_reliability_prior(2, SemanticGroundingStrength::SingleSource);
        assert_eq!(semantic_modality_reliability_prior_bonus(&prior), 0);
    }

    // CorpusMemory filter method unit tests
    // Helper to construct a minimal CorpusMemory for testing

    fn make_test_corpus() -> CorpusMemory {
        CorpusMemory {
            schema_version: 1,
            update_policy: CorpusMemoryUpdatePolicyRecord {
                advisory_only: false,
                requires_validated_intent_ir: false,
                rejects_error_findings: false,
                excludes_alias_dependent_semantic_consensus: false,
                local_grounding_required_for_canonical_promotion: false,
            },
            source_artifacts: vec![],
            actor_taxonomy_priors: vec![ActorTaxonomyPriorRecord {
                prior_id: "at1".into(),
                normalized_actor_term: "dma".into(),
                taxonomy_role: ActorTaxonomyRole::RequesterLike,
                protocol_family: ProtocolFamily::AmbaAxi,
                support_count: 5,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
                strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
            }],
            semantic_phrase_priors: vec![SemanticPhrasePriorRecord {
                prior_id: "sp1".into(),
                normalized_phrase: "valid signal".into(),
                role: InterfaceSignalSemanticRole::HandshakeValidLike,
                protocol_family: ProtocolFamily::AmbaAxi,
                source_kind: SignalSemanticHintSourceKind::ProseStatement,
                support_count: 5,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
                strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
            }],
            semantic_modality_reliability_priors: vec![],
            temporal_phrase_priors: vec![TemporalPhrasePriorRecord {
                prior_id: "tp1".into(),
                normalized_phrase: "after reset".into(),
                protocol_family: ProtocolFamily::AmbaAxi,
                cycle_window: Some(CycleWindowRecord {
                    min_cycles: Some(2),
                    max_cycles: Some(2),
                }),
                actor_grounded: true,
                handshake_completion: false,
                support_count: 3,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
            }],
            table_shape_priors: vec![TableShapePriorRecord {
                prior_id: "ts1".into(),
                normalized_header_signature: "signal | description".into(),
                table_kind: TableKind::SignalDescription,
                protocol_family: ProtocolFamily::AmbaAxi,
                support_count: 5,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
            }],
            visual_motif_priors: vec![VisualMotifPriorRecord {
                prior_id: "vm1".into(),
                normalized_caption_phrase: Some("state machine".into()),
                diagram_kind: DiagramKind::StateMachineDiagram,
                asset_kind: VisualAssetKind::Diagram,
                protocol_family: ProtocolFamily::AmbaAxi,
                support_count: 5,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
            }],
            negative_knowledge_priors: vec![NegativeKnowledgePriorRecord {
                prior_id: "nk1".into(),
                knowledge_kind: NegativeKnowledgeKind::SignalSemanticConflict,
                normalized_pattern: "VALID conflict".into(),
                protocol_family: ProtocolFamily::AmbaAxi,
                support_count: 3,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
            }],
            extraction_profile_priors: vec![ExtractionProfilePriorRecord {
                prior_id: "ep1".into(),
                cluster_signature: vec![
                    "shape:protocol_states:b0".into(),
                    "shape:registers:b2".into(),
                ],
                fired_extractors: vec![ExtractionProfileExtractorSupportRecord {
                    extractor_name: "registers.field_table".into(),
                    member_support: 2,
                }],
                support_count: 2,
                supporting_document_keys: vec!["doc_a".into(), "doc_b".into()],
            }],
        }
    }

    // extraction_profile_priors_for unit tests (CORPUS-PATTERN-REUSE.3b.2)

    #[test]
    fn extraction_profile_priors_for_matches_when_signature_is_subset() {
        let corpus = make_test_corpus();
        // The document exhibits both signature features (plus extras) → the profile matches.
        let fingerprint: BTreeSet<String> = [
            "shape:protocol_states:b0",
            "shape:registers:b2",
            "shape:actors:b1",
            "fired:registers.field_table",
        ]
        .iter()
        .map(|t| t.to_string())
        .collect();
        let matched = corpus.extraction_profile_priors_for(&fingerprint);
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].prior_id, "ep1");
    }

    #[test]
    fn extraction_profile_priors_for_rejects_partial_signature_overlap() {
        let corpus = make_test_corpus();
        // Only one of the two signature features present → signature ⊄ fingerprint → no match.
        let fingerprint: BTreeSet<String> = ["shape:registers:b2", "shape:actors:b1"]
            .iter()
            .map(|t| t.to_string())
            .collect();
        assert!(
            corpus
                .extraction_profile_priors_for(&fingerprint)
                .is_empty()
        );
    }

    #[test]
    fn extraction_profile_priors_for_skips_empty_signature_profiles() {
        // An empty signature would match every document; it must never be returned.
        let corpus = CorpusMemory {
            extraction_profile_priors: vec![ExtractionProfilePriorRecord {
                prior_id: "ep_empty".into(),
                cluster_signature: Vec::new(),
                fired_extractors: Vec::new(),
                support_count: 2,
                supporting_document_keys: vec!["doc_a".into(), "doc_b".into()],
            }],
            ..make_test_corpus()
        };
        let fingerprint: BTreeSet<String> = ["shape:registers:b2"]
            .iter()
            .map(|t| t.to_string())
            .collect();
        assert!(
            !corpus
                .extraction_profile_priors_for(&fingerprint)
                .iter()
                .any(|prior| prior.prior_id == "ep_empty")
        );
    }

    // PRIOR-DECAY: contested-prior detection (revision-on-contradiction)

    #[test]
    fn contested_priors_empty_when_each_key_has_one_value() {
        // make_test_corpus maps each key to a single value -> nothing contested.
        assert!(make_test_corpus().contested_priors().is_empty());
    }

    #[test]
    fn contested_priors_flags_conflicting_actor_taxonomy_value() {
        let mut corpus = make_test_corpus();
        // A second document maps the same term ("dma", AmbaAxi) to a DIFFERENT role.
        corpus.actor_taxonomy_priors.push(ActorTaxonomyPriorRecord {
            prior_id: "at2".into(),
            normalized_actor_term: "dma".into(),
            taxonomy_role: ActorTaxonomyRole::CompleterLike,
            protocol_family: ProtocolFamily::AmbaAxi,
            support_count: 2,
            supporting_document_keys: vec!["docB".into()],
            strongest_automation_confidence: AutomationConfidence::Medium,
            strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
        });
        let contested = corpus.contested_priors();
        assert_eq!(
            contested.len(),
            1,
            "the 'dma' role conflict must be flagged"
        );
        let flagged = &contested[0];
        assert_eq!(flagged.family, ContestedPriorFamily::ActorTaxonomy);
        assert_eq!(flagged.key, "dma");
        assert_eq!(flagged.competing_values.len(), 2);
        // strongest = the higher-support value (RequesterLike, support 5 > 2).
        assert_eq!(flagged.strongest_value, "RequesterLike");
        assert_eq!(flagged.competing_values[0].support_count, 5);
    }

    #[test]
    fn contested_priors_not_flagged_across_protocol_families() {
        let mut corpus = make_test_corpus();
        // Same term + a different role, but in a DIFFERENT protocol family — a
        // legitimately family-specific mapping, not a cross-document contradiction.
        corpus.actor_taxonomy_priors.push(ActorTaxonomyPriorRecord {
            prior_id: "at2".into(),
            normalized_actor_term: "dma".into(),
            taxonomy_role: ActorTaxonomyRole::CompleterLike,
            protocol_family: ProtocolFamily::AmbaApb,
            support_count: 2,
            supporting_document_keys: vec![],
            strongest_automation_confidence: AutomationConfidence::Medium,
            strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
        });
        assert!(
            corpus.contested_priors().is_empty(),
            "different protocol families are different scopes, not a contradiction"
        );
    }

    #[test]
    fn contested_priors_flags_conflicting_semantic_phrase_value() {
        let mut corpus = make_test_corpus();
        // "valid signal" already maps to HandshakeValidLike (support 5); a second
        // document maps it to HandshakeReadyLike with higher support (7).
        corpus
            .semantic_phrase_priors
            .push(SemanticPhrasePriorRecord {
                prior_id: "sp2".into(),
                normalized_phrase: "valid signal".into(),
                role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                protocol_family: ProtocolFamily::AmbaAxi,
                source_kind: SignalSemanticHintSourceKind::ProseStatement,
                support_count: 7,
                supporting_document_keys: vec!["docB".into()],
                strongest_automation_confidence: AutomationConfidence::High,
                strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
            });
        let semantic: Vec<_> = corpus
            .contested_priors()
            .into_iter()
            .filter(|c| c.family == ContestedPriorFamily::SemanticPhrase)
            .collect();
        assert_eq!(semantic.len(), 1);
        // strongest = the higher-support value (HandshakeReadyLike, support 7 > 5).
        assert_eq!(semantic[0].strongest_value, "HandshakeReadyLike");
    }

    // semantic_phrase_priors_for

    #[test]
    fn semantic_phrase_priors_for_matching_filter() {
        // Catches vec![] at line 88 — must return matching records.
        let corpus = make_test_corpus();
        let results = corpus.semantic_phrase_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(SignalSemanticHintSourceKind::ProseStatement),
            Some(InterfaceSignalSemanticRole::HandshakeValidLike),
        );
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn semantic_phrase_priors_for_non_matching_protocol() {
        // Catches ==→!= at line 92 — wrong protocol must exclude record.
        let corpus = make_test_corpus();
        let results = corpus.semantic_phrase_priors_for(
            Some(ProtocolFamily::AmbaApb),
            Some(SignalSemanticHintSourceKind::ProseStatement),
            Some(InterfaceSignalSemanticRole::HandshakeValidLike),
        );
        assert!(results.is_empty());
    }

    #[test]
    fn semantic_phrase_priors_for_no_filters_returns_all() {
        // Catches &&→|| at line 94 — with no filters, all records pass.
        let corpus = make_test_corpus();
        let results = corpus.semantic_phrase_priors_for(None, None, None);
        assert_eq!(results.len(), 1);
    }

    // actor_taxonomy_priors_for

    #[test]
    fn actor_taxonomy_priors_for_matching_filter() {
        // Catches vec![] at line 107.
        let corpus = make_test_corpus();
        let results = corpus.actor_taxonomy_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(ActorTaxonomyRole::RequesterLike),
        );
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn actor_taxonomy_priors_for_non_matching_role() {
        // Catches ==→!= at line 114 — wrong role must exclude record.
        let corpus = make_test_corpus();
        let results = corpus.actor_taxonomy_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(ActorTaxonomyRole::CompleterLike),
        );
        assert!(results.is_empty());
    }

    #[test]
    fn actor_taxonomy_priors_for_no_filters_returns_all() {
        // Catches &&→|| at line 113.
        let corpus = make_test_corpus();
        let results = corpus.actor_taxonomy_priors_for(None, None);
        assert_eq!(results.len(), 1);
    }

    // temporal_phrase_priors_for

    #[test]
    fn temporal_phrase_priors_for_matching_filter() {
        // Catches vec![] at line 179.
        let corpus = make_test_corpus();
        let results =
            corpus.temporal_phrase_priors_for(Some(ProtocolFamily::AmbaAxi), false, Some(true));
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn temporal_phrase_priors_for_requires_cycle_window_matching() {
        // requires_cycle_window=true with record that has a cycle window.
        let corpus = make_test_corpus();
        let results = corpus.temporal_phrase_priors_for(Some(ProtocolFamily::AmbaAxi), true, None);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn temporal_phrase_priors_for_requires_cycle_window_excludes_no_window() {
        // Catches delete ! at line 185 — requires_cycle_window=true must
        // exclude records without a cycle window. Mutant (delete !) would
        // keep them.
        let corpus = CorpusMemory {
            temporal_phrase_priors: vec![TemporalPhrasePriorRecord {
                prior_id: "tp_no_win".into(),
                normalized_phrase: "after reset".into(),
                protocol_family: ProtocolFamily::AmbaAxi,
                cycle_window: None,
                actor_grounded: true,
                handshake_completion: false,
                support_count: 3,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
            }],
            ..make_test_corpus()
        };
        let results = corpus.temporal_phrase_priors_for(Some(ProtocolFamily::AmbaAxi), true, None);
        assert!(results.is_empty());
    }

    #[test]
    fn temporal_phrase_priors_for_non_matching_actor_grounded() {
        // Catches ==→!= at line 187 — actor_grounded mismatch must exclude.
        let corpus = make_test_corpus();
        let results =
            corpus.temporal_phrase_priors_for(Some(ProtocolFamily::AmbaAxi), false, Some(false));
        assert!(results.is_empty());
    }

    // table_shape_priors_for

    #[test]
    fn table_shape_priors_for_matching_filter() {
        // Catches vec![] at line 274.
        let corpus = make_test_corpus();
        let results = corpus.table_shape_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(TableKind::SignalDescription),
        );
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn table_shape_priors_for_non_matching_kind() {
        // Catches ==→!= at line 281 — wrong table kind must exclude.
        let corpus = make_test_corpus();
        let results = corpus
            .table_shape_priors_for(Some(ProtocolFamily::AmbaAxi), Some(TableKind::RegisterMap));
        assert!(results.is_empty());
    }

    // visual_motif_priors_for

    #[test]
    fn visual_motif_priors_for_matching_filter() {
        // Catches vec![] at line 292.
        let corpus = make_test_corpus();
        let results = corpus.visual_motif_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(DiagramKind::StateMachineDiagram),
        );
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn visual_motif_priors_for_non_matching_kind() {
        // Catches ==→!= at line 299 — wrong diagram kind must exclude.
        let corpus = make_test_corpus();
        let results = corpus.visual_motif_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(DiagramKind::TimingDiagram),
        );
        assert!(results.is_empty());
    }

    // negative_knowledge_priors_for

    #[test]
    fn negative_knowledge_priors_for_matching_filter() {
        // Catches vec![] at line 310.
        let corpus = make_test_corpus();
        let results = corpus.negative_knowledge_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(NegativeKnowledgeKind::SignalSemanticConflict),
        );
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn negative_knowledge_priors_for_non_matching_kind() {
        // Catches ==→!= at line 317 — wrong knowledge kind must exclude.
        let corpus = make_test_corpus();
        let results = corpus.negative_knowledge_priors_for(
            Some(ProtocolFamily::AmbaAxi),
            Some(NegativeKnowledgeKind::ResidualDecision),
        );
        assert!(results.is_empty());
    }

    // === Batch 7: CorpusMemory search/resolve methods ===

    // actor_taxonomy_role_for_term

    #[test]
    fn actor_taxonomy_role_for_term_returns_role_for_matching_term() {
        // Catches replace Option with None at line 125 and delete ! at line 126
        // — meaningful term matching a prior must return the role.
        let corpus = make_test_corpus();
        let result = corpus.actor_taxonomy_role_for_term(Some(ProtocolFamily::AmbaAxi), "dma");
        assert_eq!(result, Some(ActorTaxonomyRole::RequesterLike));
    }

    #[test]
    fn actor_taxonomy_role_for_term_returns_none_for_stop_word() {
        // Catches delete ! at line 126 — stop-word terms must return None early.
        let corpus = make_test_corpus();
        let result = corpus.actor_taxonomy_role_for_term(Some(ProtocolFamily::AmbaAxi), "clock");
        assert_eq!(result, None);
    }

    // temporal_cycle_window_in_text

    #[test]
    fn temporal_cycle_window_in_text_matches_prior() {
        // Catches ==→!= at lines 215 and 220 — filter must match record fields.
        let corpus = make_test_corpus();
        let signal_names: BTreeSet<String> = BTreeSet::new();
        let actor_names: BTreeSet<String> = BTreeSet::new();
        let result = corpus.temporal_cycle_window_in_text(
            Some(ProtocolFamily::AmbaAxi),
            "after reset",
            &signal_names,
            &actor_names,
            Some(true),
            Some(false),
        );
        assert!(result.is_some());
    }

    #[test]
    fn temporal_cycle_window_in_text_respects_actor_grounded() {
        // Catches ==→!= at line 215 — actor_grounded mismatch must exclude.
        let corpus = make_test_corpus();
        let signal_names: BTreeSet<String> = BTreeSet::new();
        let actor_names: BTreeSet<String> = BTreeSet::new();
        // Record has actor_grounded=true; filter with Some(false) → excluded.
        let result = corpus.temporal_cycle_window_in_text(
            Some(ProtocolFamily::AmbaAxi),
            "after reset",
            &signal_names,
            &actor_names,
            Some(false),
            Some(false),
        );
        assert!(result.is_none());
    }

    #[test]
    fn temporal_cycle_window_in_text_respects_handshake_completion() {
        // Catches ==→!= at line 220 — handshake_completion mismatch must exclude.
        let corpus = make_test_corpus();
        let signal_names: BTreeSet<String> = BTreeSet::new();
        let actor_names: BTreeSet<String> = BTreeSet::new();
        // Record has handshake_completion=false; filter with Some(true) → excluded.
        let result = corpus.temporal_cycle_window_in_text(
            Some(ProtocolFamily::AmbaAxi),
            "after reset",
            &signal_names,
            &actor_names,
            Some(true),
            Some(true),
        );
        assert!(result.is_none());
    }

    #[test]
    fn temporal_cycle_window_in_text_ambiguous_returns_none() {
        // Exercises >→<, >→==, >→>= at line 238 — multiple different cycle
        // windows matching the same text must return None (ambiguity).
        let mut corpus = make_test_corpus();
        corpus
            .temporal_phrase_priors
            .push(TemporalPhrasePriorRecord {
                prior_id: "tp2".into(),
                normalized_phrase: "after reset".into(),
                protocol_family: ProtocolFamily::AmbaAxi,
                cycle_window: Some(CycleWindowRecord {
                    min_cycles: Some(4),
                    max_cycles: Some(4),
                }),
                actor_grounded: true,
                handshake_completion: false,
                support_count: 3,
                supporting_document_keys: vec![],
                strongest_automation_confidence: AutomationConfidence::High,
            });
        let signal_names: BTreeSet<String> = BTreeSet::new();
        let actor_names: BTreeSet<String> = BTreeSet::new();
        let result = corpus.temporal_cycle_window_in_text(
            Some(ProtocolFamily::AmbaAxi),
            "after reset",
            &signal_names,
            &actor_names,
            Some(true),
            Some(false),
        );
        assert!(result.is_none());
    }

    #[test]
    fn temporal_cycle_window_in_text_ambiguous_first_scope_falls_through() {
        // Catches >→< and >→== at line 238 — when first scope (AmbaAxi) has
        // ambiguity (>1 windows), original returns None immediately. Mutants
        // fail the check and fall through to AmbaGeneric scope which has a
        // single unambiguous match — returning Some is wrong.
        let corpus = CorpusMemory {
            temporal_phrase_priors: vec![
                // AmbaAxi — two records with same phrase but different
                // cycle windows → ambiguity.
                TemporalPhrasePriorRecord {
                    prior_id: "tp_axi_1".into(),
                    normalized_phrase: "after reset".into(),
                    protocol_family: ProtocolFamily::AmbaAxi,
                    cycle_window: Some(CycleWindowRecord {
                        min_cycles: Some(2),
                        max_cycles: Some(2),
                    }),
                    actor_grounded: true,
                    handshake_completion: false,
                    support_count: 3,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
                TemporalPhrasePriorRecord {
                    prior_id: "tp_axi_2".into(),
                    normalized_phrase: "after reset".into(),
                    protocol_family: ProtocolFamily::AmbaAxi,
                    cycle_window: Some(CycleWindowRecord {
                        min_cycles: Some(4),
                        max_cycles: Some(4),
                    }),
                    actor_grounded: true,
                    handshake_completion: false,
                    support_count: 3,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
                // AmbaGeneric — single unambiguous match. Must NOT be
                // returned when first scope is ambiguous.
                TemporalPhrasePriorRecord {
                    prior_id: "tp_gen".into(),
                    normalized_phrase: "after reset".into(),
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    cycle_window: Some(CycleWindowRecord {
                        min_cycles: Some(1),
                        max_cycles: Some(1),
                    }),
                    actor_grounded: true,
                    handshake_completion: false,
                    support_count: 3,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
            ],
            ..make_test_corpus()
        };
        let signal_names: BTreeSet<String> = BTreeSet::new();
        let actor_names: BTreeSet<String> = BTreeSet::new();
        let result = corpus.temporal_cycle_window_in_text(
            Some(ProtocolFamily::AmbaAxi),
            "after reset",
            &signal_names,
            &actor_names,
            Some(true),
            Some(false),
        );
        assert!(result.is_none());
    }

    // semantic_modality_reliability_bonus

    #[test]
    fn semantic_modality_reliability_bonus_skips_zero_in_first_scope() {
        // Catches >→>= at line 261 — zero-bonus in first scope must not
        // short-circuit search of subsequent scopes.
        let corpus = CorpusMemory {
            semantic_modality_reliability_priors: vec![
                SemanticModalityReliabilityPriorRecord {
                    prior_id: "rel_axi".into(),
                    role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    source_kind: SignalSemanticHintSourceKind::ProseStatement,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::SingleSource,
                },
                SemanticModalityReliabilityPriorRecord {
                    prior_id: "rel_gen".into(),
                    role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    source_kind: SignalSemanticHintSourceKind::ProseStatement,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
                },
            ],
            ..make_test_corpus()
        };
        let result = corpus.semantic_modality_reliability_bonus(
            Some(ProtocolFamily::AmbaAxi),
            InterfaceSignalSemanticRole::HandshakeValidLike,
            SignalSemanticHintSourceKind::ProseStatement,
        );
        // SingleSource gives 0, MultiSource gives non-zero. Correct code (> 0)
        // skips Axi and returns Generic bonus. Mutant (>= 0) returns 0.
        assert!(result > 0);
    }

    // diagram_kind_for_visual_caption

    #[test]
    fn diagram_kind_for_visual_caption_ambiguous_returns_none() {
        // Exercising >→== and >→>= at line 374. Multi-scope: first scope
        // (AmbaAxi) ambiguous, second scope (AmbaGeneric) unambiguous.
        // Original and >= retain None; == and < fall through.
        let corpus = CorpusMemory {
            visual_motif_priors: vec![
                VisualMotifPriorRecord {
                    prior_id: "vm_axi_1".into(),
                    normalized_caption_phrase: Some("state machine".into()),
                    diagram_kind: DiagramKind::StateMachineDiagram,
                    asset_kind: VisualAssetKind::Diagram,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
                VisualMotifPriorRecord {
                    prior_id: "vm_axi_2".into(),
                    normalized_caption_phrase: Some("state machine".into()),
                    diagram_kind: DiagramKind::TimingDiagram,
                    asset_kind: VisualAssetKind::Diagram,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
                VisualMotifPriorRecord {
                    prior_id: "vm_gen".into(),
                    normalized_caption_phrase: Some("state machine".into()),
                    diagram_kind: DiagramKind::StateMachineDiagram,
                    asset_kind: VisualAssetKind::Diagram,
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
            ],
            ..make_test_corpus()
        };
        let signal_names: BTreeSet<String> = BTreeSet::new();
        let actor_names: BTreeSet<String> = BTreeSet::new();
        let result = corpus.diagram_kind_for_visual_caption(
            Some(ProtocolFamily::AmbaAxi),
            "state machine",
            &signal_names,
            &actor_names,
        );
        assert!(result.is_none());
    }

    // table_kind_for_structured_table

    #[test]
    fn table_kind_for_structured_table_ambiguous_returns_none() {
        // Multi-scope: first scope (AmbaAxi) ambiguous, second scope
        // (AmbaGeneric) unambiguous. Catches >→< and >→== at line 402.
        let corpus = CorpusMemory {
            table_shape_priors: vec![
                TableShapePriorRecord {
                    prior_id: "ts_axi_1".into(),
                    normalized_header_signature: "signal | description".into(),
                    table_kind: TableKind::SignalDescription,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
                TableShapePriorRecord {
                    prior_id: "ts_axi_2".into(),
                    normalized_header_signature: "signal | description".into(),
                    table_kind: TableKind::RegisterMap,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
                TableShapePriorRecord {
                    prior_id: "ts_gen".into(),
                    normalized_header_signature: "signal | description".into(),
                    table_kind: TableKind::SignalDescription,
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                },
            ],
            ..make_test_corpus()
        };
        let table = StructuredTableRecord {
            table_id: "t1".into(),
            asset_id: "a1".into(),
            page_id: None,
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![vec![
                StructuredTableCellRecord {
                    text: "signal".into(),
                    row_span: 1,
                    col_span: 1,
                    is_header: true,
                },
                StructuredTableCellRecord {
                    text: "description".into(),
                    row_span: 1,
                    col_span: 1,
                    is_header: true,
                },
            ]],
            body_rows: vec![],
            row_count: 1,
            col_count: 2,
        };
        let result = corpus.table_kind_for_structured_table(Some(ProtocolFamily::AmbaAxi), &table);
        assert!(result.is_none());
    }

    // resolve_actor_taxonomy_role (via actor_taxonomy_role_for_term)

    #[test]
    fn resolve_actor_taxonomy_role_ambiguous_returns_none() {
        // Multi-scope: first scope (AmbaAxi) ambiguous, second scope
        // (AmbaGeneric) unambiguous. Catches >→< and >→== at line 430.
        let corpus = CorpusMemory {
            actor_taxonomy_priors: vec![
                ActorTaxonomyPriorRecord {
                    prior_id: "at_axi_1".into(),
                    normalized_actor_term: "dma".into(),
                    taxonomy_role: ActorTaxonomyRole::RequesterLike,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
                },
                ActorTaxonomyPriorRecord {
                    prior_id: "at_axi_2".into(),
                    normalized_actor_term: "dma".into(),
                    taxonomy_role: ActorTaxonomyRole::CompleterLike,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
                },
                ActorTaxonomyPriorRecord {
                    prior_id: "at_gen".into(),
                    normalized_actor_term: "dma".into(),
                    taxonomy_role: ActorTaxonomyRole::RequesterLike,
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
                },
            ],
            ..make_test_corpus()
        };
        let result = corpus.actor_taxonomy_role_for_term(Some(ProtocolFamily::AmbaAxi), "dma");
        assert!(result.is_none());
    }

    // resolve_semantic_phrase_role (via semantic_phrase_role_in_text)

    #[test]
    fn resolve_semantic_phrase_role_ambiguous_returns_none() {
        // Multi-scope: first scope (AmbaAxi) ambiguous, second scope
        // (AmbaGeneric) unambiguous. Catches >→< and >→== at line 462.
        let corpus = CorpusMemory {
            semantic_phrase_priors: vec![
                SemanticPhrasePriorRecord {
                    prior_id: "sp_axi_1".into(),
                    normalized_phrase: "valid signal".into(),
                    role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    source_kind: SignalSemanticHintSourceKind::ProseStatement,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
                },
                SemanticPhrasePriorRecord {
                    prior_id: "sp_axi_2".into(),
                    normalized_phrase: "valid signal".into(),
                    role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                    protocol_family: ProtocolFamily::AmbaAxi,
                    source_kind: SignalSemanticHintSourceKind::ProseStatement,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
                },
                SemanticPhrasePriorRecord {
                    prior_id: "sp_gen".into(),
                    normalized_phrase: "valid signal".into(),
                    role: InterfaceSignalSemanticRole::HandshakeValidLike,
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    source_kind: SignalSemanticHintSourceKind::ProseStatement,
                    support_count: 5,
                    supporting_document_keys: vec![],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::MultiSource,
                },
            ],
            ..make_test_corpus()
        };
        let signal_names: BTreeSet<String> = BTreeSet::new();
        let actor_names: BTreeSet<String> = BTreeSet::new();
        let result = corpus.semantic_phrase_role_in_text(
            Some(ProtocolFamily::AmbaAxi),
            SignalSemanticHintSourceKind::ProseStatement,
            "valid signal",
            &signal_names,
            &actor_names,
        );
        assert!(result.is_none());
    }
}
