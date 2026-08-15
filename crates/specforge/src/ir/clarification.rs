//! Versioned interchange IR for autonomous-first clarification.
//!
//! The packet and answer envelopes in this module are deliberately not proof witnesses. They make
//! missing information precise, bind every exchange to current repository-local artifacts, and
//! preserve lifecycle and supersession history. Deserialization, a matching digest, or a claimed
//! responder policy never grants canonical authority; the later answer-validation lane must check
//! the current source, policy, schema, conflicts, and proof boundary before resuming derivation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use super::IrStage;
use super::derivation::Sha256Digest;

/// Current clarification-packet schema.
pub const CLARIFICATION_PACKET_SCHEMA_VERSION: u32 = 1;
/// Current answer-envelope schema.
pub const CLARIFICATION_ANSWER_SCHEMA_VERSION: u32 = 1;

const MAX_ID_BYTES: usize = 192;
const MAX_PATH_BYTES: usize = 1_024;
const MAX_SHORT_TEXT_BYTES: usize = 1_024;
const MAX_LONG_TEXT_BYTES: usize = 16_384;
const MAX_QUESTIONS: usize = 4_096;
const MAX_COLLECTION_ITEMS: usize = 4_096;
const MAX_SCHEMA_DEPTH: usize = 16;

pub type ClarificationResult<T> = Result<T, ClarificationError>;

/// A structural/schema failure. It is intentionally separate from proof authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClarificationError {
    message: String,
}

impl ClarificationError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ClarificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ClarificationError {}

/// Exact repository-local artifact identity used for currentness checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRevisionRef {
    pub path: String,
    pub sha256: Sha256Digest,
}

impl ArtifactRevisionRef {
    fn validate(&self, label: &str) -> ClarificationResult<()> {
        validate_repo_relative_path(&format!("{label} path"), &self.path)
    }
}

/// Current source and proof-bearing stage to which a packet is bound.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClarificationArtifactContext {
    pub source: ArtifactRevisionRef,
    pub current_stage: IrStage,
    pub current_artifact: ArtifactRevisionRef,
    pub proof_ruleset_sha256: Sha256Digest,
}

impl ClarificationArtifactContext {
    fn validate(&self) -> ClarificationResult<()> {
        self.source.validate("source artifact")?;
        self.current_artifact.validate("current artifact")
    }
}

/// An optional pixel-space rectangle for a cited visual or table region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PixelBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl PixelBounds {
    fn validate(self) -> ClarificationResult<()> {
        if self.width == 0 || self.height == 0 {
            return Err(ClarificationError::new(
                "visual/table pixel bounds must have positive width and height",
            ));
        }
        self.x
            .checked_add(self.width)
            .ok_or_else(|| ClarificationError::new("pixel bounds overflow on the x axis"))?;
        self.y
            .checked_add(self.height)
            .ok_or_else(|| ClarificationError::new("pixel bounds overflow on the y axis"))?;
        Ok(())
    }
}

/// Exact source, proof, residual, or validation location relevant to a question or answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidenceLink {
    SourceSpan {
        artifact_sha256: Sha256Digest,
        span_id: String,
        content_sha256: Sha256Digest,
    },
    TableRegion {
        artifact_sha256: Sha256Digest,
        table_id: String,
        row_start: u32,
        row_end: u32,
        column_start: u32,
        column_end: u32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bounds: Option<PixelBounds>,
        content_sha256: Sha256Digest,
    },
    VisualRegion {
        artifact_sha256: Sha256Digest,
        region_id: String,
        page_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bounds: Option<PixelBounds>,
        content_sha256: Sha256Digest,
    },
    ProofClaim {
        stage: IrStage,
        surface: String,
        stable_record_key: String,
        conclusion_sha256: Sha256Digest,
    },
    ResidualDecision {
        stage: IrStage,
        artifact_sha256: Sha256Digest,
        packet_id: String,
    },
    ValidationFinding {
        stage: IrStage,
        artifact_sha256: Sha256Digest,
        finding_id: String,
    },
}

impl EvidenceLink {
    fn validate(&self) -> ClarificationResult<()> {
        match self {
            Self::SourceSpan { span_id, .. } => validate_id("source span id", span_id),
            Self::TableRegion {
                table_id,
                row_start,
                row_end,
                column_start,
                column_end,
                bounds,
                ..
            } => {
                validate_id("table id", table_id)?;
                if row_start > row_end || column_start > column_end {
                    return Err(ClarificationError::new(
                        "table region start coordinates must not exceed end coordinates",
                    ));
                }
                if let Some(bounds) = bounds {
                    bounds.validate()?;
                }
                Ok(())
            }
            Self::VisualRegion {
                region_id,
                page_id,
                bounds,
                ..
            } => {
                validate_id("visual region id", region_id)?;
                validate_id("visual page id", page_id)?;
                if let Some(bounds) = bounds {
                    bounds.validate()?;
                }
                Ok(())
            }
            Self::ProofClaim {
                surface,
                stable_record_key,
                ..
            } => {
                validate_id("proof surface", surface)?;
                validate_text(
                    "proof stable record key",
                    stable_record_key,
                    MAX_SHORT_TEXT_BYTES,
                )
            }
            Self::ResidualDecision { packet_id, .. } => {
                validate_id("residual decision packet id", packet_id)
            }
            Self::ValidationFinding { finding_id, .. } => {
                validate_id("validation finding id", finding_id)
            }
        }
    }
}

/// Why governed autonomous work cannot yet decide the proposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingInformationReason {
    SourceAmbiguity,
    SourceContradiction,
    MissingSourceContent,
    ExtractionLimitation,
    ExternalDesignChoice,
}

/// The governed subsystem that produced the unresolved state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClarificationOrigin {
    ResidualDecision,
    Contradiction,
    CompletenessFinding,
    ValidationFinding,
    AdapterBlock,
    ExternalChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClarificationPriority {
    Critical,
    High,
    Normal,
    Low,
}

/// Whether independent pipeline branches remain executable while this question is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutonomousContinuation {
    ContinueUnaffected,
    FullyBlocked,
}

/// One reviewable alternative known before asking the user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClarificationAlternative {
    pub alternative_id: String,
    pub description: String,
    pub downstream_consequence: String,
}

impl ClarificationAlternative {
    fn validate(&self) -> ClarificationResult<()> {
        validate_id("clarification alternative id", &self.alternative_id)?;
        validate_text(
            "clarification alternative description",
            &self.description,
            MAX_LONG_TEXT_BYTES,
        )?;
        validate_text(
            "clarification alternative downstream consequence",
            &self.downstream_consequence,
            MAX_LONG_TEXT_BYTES,
        )
    }
}

/// One precise downstream result affected by an unresolved proposition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DownstreamImpact {
    pub stage: IrStage,
    pub surface: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_record_key: Option<String>,
    pub blocking: bool,
    pub explanation: String,
}

impl DownstreamImpact {
    fn validate(&self) -> ClarificationResult<()> {
        validate_id("downstream surface", &self.surface)?;
        if let Some(key) = &self.stable_record_key {
            validate_text("downstream stable record key", key, MAX_SHORT_TEXT_BYTES)?;
        }
        validate_text(
            "downstream impact explanation",
            &self.explanation,
            MAX_LONG_TEXT_BYTES,
        )
    }
}

/// Semantic authority a value answer claims. Matching this enum is necessary but never sufficient
/// for proof authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerAuthorityKind {
    SourceLocator,
    SourceSupplement,
    ExternalDesignDecision,
}

/// Versioned authority policy a question requires for an answer kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerAuthorityRequirement {
    pub kind: AnswerAuthorityKind,
    pub policy_id: String,
    pub policy_sha256: Sha256Digest,
}

impl AnswerAuthorityRequirement {
    fn validate(&self) -> ClarificationResult<()> {
        validate_id("answer authority policy id", &self.policy_id)
    }
}

/// Atomic and structured value shapes supported by schema version 1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnswerValueSchema {
    Boolean,
    Integer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_inclusive: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_inclusive: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    Decimal {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_inclusive: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_inclusive: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    Enumeration {
        choices: Vec<String>,
        allow_multiple: bool,
    },
    BitVector {
        width: u32,
    },
    Identifier {
        max_bytes: u32,
    },
    Text {
        max_bytes: u32,
    },
    SourceReference {
        allowed_kinds: BTreeSet<EvidenceLinkKind>,
    },
    Record {
        fields: Vec<AnswerFieldSchema>,
    },
    List {
        item: Box<AnswerValueSchema>,
        min_items: u32,
        max_items: u32,
    },
}

impl AnswerValueSchema {
    fn validate(&self, depth: usize) -> ClarificationResult<()> {
        if depth > MAX_SCHEMA_DEPTH {
            return Err(ClarificationError::new(format!(
                "answer schema nesting exceeds {MAX_SCHEMA_DEPTH} levels"
            )));
        }
        match self {
            Self::Boolean => Ok(()),
            Self::Integer {
                min_inclusive,
                max_inclusive,
                unit,
            } => {
                if let (Some(min), Some(max)) = (min_inclusive, max_inclusive)
                    && min > max
                {
                    return Err(ClarificationError::new(
                        "integer answer schema minimum exceeds maximum",
                    ));
                }
                validate_optional_unit(unit)
            }
            Self::Decimal {
                min_inclusive,
                max_inclusive,
                unit,
            } => {
                if let Some(min) = min_inclusive {
                    validate_decimal("decimal minimum", min)?;
                }
                if let Some(max) = max_inclusive {
                    validate_decimal("decimal maximum", max)?;
                }
                validate_optional_unit(unit)
            }
            Self::Enumeration {
                choices,
                allow_multiple: _,
            } => {
                validate_nonempty_collection("enumeration choices", choices)?;
                let mut unique = BTreeSet::new();
                for choice in choices {
                    validate_text("enumeration choice", choice, MAX_SHORT_TEXT_BYTES)?;
                    if !unique.insert(choice) {
                        return Err(ClarificationError::new(
                            "enumeration answer schema contains a duplicate choice",
                        ));
                    }
                }
                Ok(())
            }
            Self::BitVector { width } => {
                if *width == 0 || *width > 1_048_576 {
                    Err(ClarificationError::new(
                        "bit-vector answer width must be within 1..=1048576",
                    ))
                } else {
                    Ok(())
                }
            }
            Self::Identifier { max_bytes } | Self::Text { max_bytes } => {
                if *max_bytes == 0
                    || usize::try_from(*max_bytes).unwrap_or(usize::MAX) > MAX_LONG_TEXT_BYTES
                {
                    Err(ClarificationError::new(format!(
                        "answer text limit must be within 1..={MAX_LONG_TEXT_BYTES} bytes"
                    )))
                } else {
                    Ok(())
                }
            }
            Self::SourceReference { allowed_kinds } => {
                if allowed_kinds.is_empty() {
                    Err(ClarificationError::new(
                        "source-reference answer schema must allow at least one evidence-link kind",
                    ))
                } else {
                    Ok(())
                }
            }
            Self::Record { fields } => {
                validate_nonempty_collection("answer record fields", fields)?;
                let mut unique = BTreeSet::new();
                for field in fields {
                    field.validate(depth + 1)?;
                    if !unique.insert(field.name.as_str()) {
                        return Err(ClarificationError::new(
                            "answer record schema contains a duplicate field name",
                        ));
                    }
                }
                Ok(())
            }
            Self::List {
                item,
                min_items,
                max_items,
            } => {
                if min_items > max_items
                    || usize::try_from(*max_items).unwrap_or(usize::MAX) > MAX_COLLECTION_ITEMS
                {
                    return Err(ClarificationError::new(format!(
                        "answer list bounds must be ordered and max_items must not exceed {MAX_COLLECTION_ITEMS}"
                    )));
                }
                item.validate(depth + 1)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLinkKind {
    SourceSpan,
    TableRegion,
    VisualRegion,
    ProofClaim,
    ResidualDecision,
    ValidationFinding,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerFieldSchema {
    pub name: String,
    pub required: bool,
    pub value: Box<AnswerValueSchema>,
}

impl AnswerFieldSchema {
    fn validate(&self, depth: usize) -> ClarificationResult<()> {
        validate_id("answer field name", &self.name)?;
        self.value.validate(depth)
    }
}

/// Complete response contract shown to a user or machine client.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerSchema {
    pub authorities: Vec<AnswerAuthorityRequirement>,
    pub value: AnswerValueSchema,
    pub validation_rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
}

impl AnswerSchema {
    fn validate(&self) -> ClarificationResult<()> {
        validate_nonempty_collection("answer authority requirements", &self.authorities)?;
        let mut authorities = BTreeSet::new();
        for authority in &self.authorities {
            authority.validate()?;
            if !authorities.insert(authority.kind) {
                return Err(ClarificationError::new(
                    "answer schema contains a duplicate authority kind",
                ));
            }
        }
        self.value.validate(0)?;
        validate_nonempty_collection("answer validation rules", &self.validation_rules)?;
        for rule in &self.validation_rules {
            validate_text("answer validation rule", rule, MAX_LONG_TEXT_BYTES)?;
        }
        validate_bounded_collection("answer examples", &self.examples)?;
        for example in &self.examples {
            validate_text("answer example", example, MAX_LONG_TEXT_BYTES)?;
        }
        Ok(())
    }
}

/// Immutable binding to an issued question definition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionRevisionRef {
    pub question_id: String,
    pub revision: u32,
    pub definition_sha256: Sha256Digest,
}

impl QuestionRevisionRef {
    fn validate(&self) -> ClarificationResult<()> {
        validate_id("question reference id", &self.question_id)?;
        validate_positive_revision("question reference", self.revision)
    }
}

/// Immutable question meaning. Lifecycle transitions never change this digest-bound definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClarificationQuestionDefinition {
    pub question_id: String,
    pub revision: u32,
    pub origin: ClarificationOrigin,
    pub missing_information_reason: MissingInformationReason,
    pub evidence: Vec<EvidenceLink>,
    pub proposition: String,
    pub why_automation_stopped: String,
    pub alternatives: Vec<ClarificationAlternative>,
    pub downstream_impacts: Vec<DownstreamImpact>,
    pub priority: ClarificationPriority,
    /// Deterministic planner score in the closed interval 0..=100.
    pub information_gain_score: u8,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<QuestionRevisionRef>,
    pub autonomous_continuation: AutonomousContinuation,
    pub answer_schema: AnswerSchema,
}

impl ClarificationQuestionDefinition {
    pub fn definition_sha256(&self) -> ClarificationResult<Sha256Digest> {
        Sha256Digest::of_serializable(self).map_err(|error| {
            ClarificationError::new(format!(
                "cannot hash clarification question definition: {error}"
            ))
        })
    }

    pub fn reference(&self) -> ClarificationResult<QuestionRevisionRef> {
        Ok(QuestionRevisionRef {
            question_id: self.question_id.clone(),
            revision: self.revision,
            definition_sha256: self.definition_sha256()?,
        })
    }

    fn validate(&self) -> ClarificationResult<()> {
        validate_id("clarification question id", &self.question_id)?;
        validate_positive_revision("clarification question", self.revision)?;
        validate_nonempty_collection("clarification evidence links", &self.evidence)?;
        validate_unique_serialized("clarification evidence links", &self.evidence)?;
        for link in &self.evidence {
            link.validate()?;
        }
        validate_text(
            "clarification proposition",
            &self.proposition,
            MAX_LONG_TEXT_BYTES,
        )?;
        validate_text(
            "clarification automation-stop explanation",
            &self.why_automation_stopped,
            MAX_LONG_TEXT_BYTES,
        )?;
        validate_nonempty_collection("clarification alternatives", &self.alternatives)?;
        let mut alternative_ids = BTreeSet::new();
        for alternative in &self.alternatives {
            alternative.validate()?;
            if !alternative_ids.insert(alternative.alternative_id.as_str()) {
                return Err(ClarificationError::new(
                    "clarification question contains a duplicate alternative id",
                ));
            }
        }
        validate_nonempty_collection("clarification downstream impacts", &self.downstream_impacts)?;
        validate_unique_serialized("clarification downstream impacts", &self.downstream_impacts)?;
        for impact in &self.downstream_impacts {
            impact.validate()?;
        }
        if self.information_gain_score > 100 {
            return Err(ClarificationError::new(
                "clarification information-gain score must be within 0..=100",
            ));
        }
        validate_bounded_collection("clarification dependencies", &self.dependencies)?;
        let mut dependency_ids = BTreeSet::new();
        for dependency in &self.dependencies {
            dependency.validate()?;
            if dependency.question_id == self.question_id {
                return Err(ClarificationError::new(
                    "clarification question cannot depend on itself",
                ));
            }
            if !dependency_ids.insert((&dependency.question_id, dependency.revision)) {
                return Err(ClarificationError::new(
                    "clarification question contains a duplicate dependency",
                ));
            }
        }
        self.answer_schema.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerRevisionRef {
    pub answer_id: String,
    pub revision: u32,
    pub content_sha256: Sha256Digest,
}

impl AnswerRevisionRef {
    fn validate(&self) -> ClarificationResult<()> {
        validate_id("answer reference id", &self.answer_id)?;
        validate_positive_revision("answer reference", self.revision)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClarificationStatus {
    Open,
    AnsweredPendingValidation,
    Accepted,
    Rejected,
    Deferred,
    ResolvedAutonomously,
    Stale,
    Superseded,
    Cancelled,
}

/// Mutable current-state projection kept separate from the immutable question definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClarificationLifecycle {
    pub status: ClarificationStatus,
    pub state_sequence: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<AnswerRevisionRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_by: Option<QuestionRevisionRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ClarificationLifecycle {
    fn validate(&self) -> ClarificationResult<()> {
        validate_positive_revision("clarification lifecycle sequence", self.state_sequence)?;
        if let Some(answer) = &self.answer {
            answer.validate()?;
        }
        if let Some(question) = &self.superseded_by {
            question.validate()?;
        }
        if let Some(reason) = &self.reason {
            validate_text(
                "clarification lifecycle reason",
                reason,
                MAX_LONG_TEXT_BYTES,
            )?;
        }
        let has_reason = self.reason.is_some();
        let valid = match self.status {
            ClarificationStatus::Open => {
                self.answer.is_none() && self.superseded_by.is_none() && !has_reason
            }
            ClarificationStatus::AnsweredPendingValidation | ClarificationStatus::Accepted => {
                self.answer.is_some() && self.superseded_by.is_none()
            }
            ClarificationStatus::Rejected => {
                self.answer.is_some() && self.superseded_by.is_none() && has_reason
            }
            ClarificationStatus::Deferred => self.superseded_by.is_none() && has_reason,
            ClarificationStatus::ResolvedAutonomously
            | ClarificationStatus::Stale
            | ClarificationStatus::Cancelled => {
                self.answer.is_none() && self.superseded_by.is_none() && has_reason
            }
            ClarificationStatus::Superseded => {
                self.answer.is_none() && self.superseded_by.is_some() && has_reason
            }
        };
        if valid {
            Ok(())
        } else {
            Err(ClarificationError::new(format!(
                "clarification lifecycle fields are inconsistent with status {:?}",
                self.status
            )))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClarificationQuestion {
    pub definition: ClarificationQuestionDefinition,
    pub lifecycle: ClarificationLifecycle,
}

impl ClarificationQuestion {
    pub fn reference(&self) -> ClarificationResult<QuestionRevisionRef> {
        self.definition.reference()
    }

    fn validate(&self) -> ClarificationResult<()> {
        self.definition.validate()?;
        self.lifecycle.validate()?;
        if let Some(superseded_by) = &self.lifecycle.superseded_by
            && superseded_by.question_id == self.definition.question_id
            && superseded_by.revision <= self.definition.revision
        {
            return Err(ClarificationError::new(
                "a superseding question must advance the same question id to a later revision or use a different id",
            ));
        }
        Ok(())
    }
}

/// Dependency-ordered packet bound to one exact source/current-artifact/ruleset context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClarificationPacket {
    schema_version: u32,
    pub packet_id: String,
    pub context: ClarificationArtifactContext,
    pub questions: Vec<ClarificationQuestion>,
}

impl ClarificationPacket {
    pub fn new(
        packet_id: impl Into<String>,
        context: ClarificationArtifactContext,
        questions: Vec<ClarificationQuestion>,
    ) -> ClarificationResult<Self> {
        let packet = Self {
            schema_version: CLARIFICATION_PACKET_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            context,
            questions,
        };
        packet.validate()?;
        Ok(packet)
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn from_json(json: &str) -> ClarificationResult<Self> {
        let packet: Self = serde_json::from_str(json).map_err(|error| {
            ClarificationError::new(format!("cannot parse clarification packet: {error}"))
        })?;
        packet.validate()?;
        Ok(packet)
    }

    pub fn to_pretty_json(&self) -> ClarificationResult<String> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(|error| {
            ClarificationError::new(format!("cannot serialize clarification packet: {error}"))
        })
    }

    pub fn validate(&self) -> ClarificationResult<()> {
        if self.schema_version != CLARIFICATION_PACKET_SCHEMA_VERSION {
            return Err(ClarificationError::new(format!(
                "unsupported clarification packet schema {}; expected {CLARIFICATION_PACKET_SCHEMA_VERSION}",
                self.schema_version
            )));
        }
        validate_id("clarification packet id", &self.packet_id)?;
        self.context.validate()?;
        validate_nonempty_collection("clarification packet questions", &self.questions)?;
        if self.questions.len() > MAX_QUESTIONS {
            return Err(ClarificationError::new(format!(
                "clarification packet has {} questions, above the {MAX_QUESTIONS}-question limit",
                self.questions.len()
            )));
        }

        let mut positions = BTreeMap::new();
        let mut references = BTreeMap::new();
        for (index, question) in self.questions.iter().enumerate() {
            question.validate()?;
            let key = (
                question.definition.question_id.as_str(),
                question.definition.revision,
            );
            if positions.insert(key, index).is_some() {
                return Err(ClarificationError::new(
                    "clarification packet contains a duplicate question id/revision",
                ));
            }
            references.insert(key, question.reference()?);
        }

        for (index, question) in self.questions.iter().enumerate() {
            for dependency in &question.definition.dependencies {
                let key = (dependency.question_id.as_str(), dependency.revision);
                let Some(position) = positions.get(&key) else {
                    return Err(ClarificationError::new(format!(
                        "clarification question '{}' has a dependency absent from its packet",
                        question.definition.question_id
                    )));
                };
                if *position >= index {
                    return Err(ClarificationError::new(format!(
                        "clarification question '{}' is not dependency-ordered",
                        question.definition.question_id
                    )));
                }
                if references.get(&key) != Some(dependency) {
                    return Err(ClarificationError::new(format!(
                        "clarification question '{}' has a stale dependency definition digest",
                        question.definition.question_id
                    )));
                }
            }
        }
        Ok(())
    }

    fn find_question(&self, reference: &QuestionRevisionRef) -> Option<&ClarificationQuestion> {
        self.questions.iter().find(|question| {
            question.definition.question_id == reference.question_id
                && question.definition.revision == reference.revision
        })
    }
}

/// Typed value carried by a value answer. Matching it against the issued schema belongs to answer
/// validation; structural parsing alone is not acceptance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ClarificationValue {
    Boolean {
        value: bool,
    },
    Integer {
        value: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    Decimal {
        value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    Enumeration {
        choices: Vec<String>,
    },
    BitVector {
        value: String,
    },
    Identifier {
        value: String,
    },
    Text {
        value: String,
    },
    SourceReference {
        links: Vec<EvidenceLink>,
    },
    Record {
        fields: BTreeMap<String, ClarificationValue>,
    },
    List {
        items: Vec<ClarificationValue>,
    },
}

impl ClarificationValue {
    fn validate(&self, depth: usize) -> ClarificationResult<()> {
        if depth > MAX_SCHEMA_DEPTH {
            return Err(ClarificationError::new(format!(
                "clarification value nesting exceeds {MAX_SCHEMA_DEPTH} levels"
            )));
        }
        match self {
            Self::Boolean { .. } | Self::Integer { unit: None, .. } => Ok(()),
            Self::Integer {
                unit: Some(unit), ..
            } => validate_text("integer answer unit", unit, MAX_SHORT_TEXT_BYTES),
            Self::Decimal { value, unit } => {
                validate_decimal("decimal answer", value)?;
                validate_optional_unit(unit)
            }
            Self::Enumeration { choices } => {
                validate_nonempty_collection("enumeration answer choices", choices)?;
                validate_bounded_collection("enumeration answer choices", choices)?;
                let mut unique = BTreeSet::new();
                for choice in choices {
                    validate_text("enumeration answer choice", choice, MAX_SHORT_TEXT_BYTES)?;
                    if !unique.insert(choice) {
                        return Err(ClarificationError::new(
                            "enumeration answer contains a duplicate choice",
                        ));
                    }
                }
                Ok(())
            }
            Self::BitVector { value } => validate_bit_vector(value),
            Self::Identifier { value } => validate_id("identifier answer", value),
            Self::Text { value } => validate_text("text answer", value, MAX_LONG_TEXT_BYTES),
            Self::SourceReference { links } => {
                validate_nonempty_collection("source-reference answer links", links)?;
                validate_unique_serialized("source-reference answer links", links)?;
                for link in links {
                    link.validate()?;
                }
                Ok(())
            }
            Self::Record { fields } => {
                if fields.is_empty() {
                    return Err(ClarificationError::new(
                        "record answer fields must contain at least one item",
                    ));
                }
                if fields.len() > MAX_COLLECTION_ITEMS {
                    return Err(ClarificationError::new(format!(
                        "record answer fields contains {} items, above the {MAX_COLLECTION_ITEMS}-item limit",
                        fields.len()
                    )));
                }
                for (name, value) in fields {
                    validate_id("record answer field name", name)?;
                    value.validate(depth + 1)?;
                }
                Ok(())
            }
            Self::List { items } => {
                validate_bounded_collection("list answer items", items)?;
                for item in items {
                    item.validate(depth + 1)?;
                }
                Ok(())
            }
        }
    }
}

/// Explicit value or non-answer. Non-answers remain actionable state rather than fabricated data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnswerDisposition {
    Value { value: ClarificationValue },
    Unknown { explanation: String },
    Unavailable { explanation: String },
    NotApplicable { explanation: String },
    Defer { explanation: String },
}

impl AnswerDisposition {
    fn validate(&self) -> ClarificationResult<()> {
        match self {
            Self::Value { value } => value.validate(0),
            Self::Unknown { explanation }
            | Self::Unavailable { explanation }
            | Self::NotApplicable { explanation }
            | Self::Defer { explanation } => {
                validate_text("non-answer explanation", explanation, MAX_LONG_TEXT_BYTES)
            }
        }
    }

    fn is_value(&self) -> bool {
        matches!(self, Self::Value { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerChannel {
    InteractiveCli,
    ImportedBundle,
    Api,
}

/// Audit provenance for an answer submission. Identity strings do not self-authenticate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerProvenance {
    pub responder_id: String,
    pub channel: AnswerChannel,
    pub submission_sequence: u64,
}

impl AnswerProvenance {
    fn validate(&self) -> ClarificationResult<()> {
        validate_id("answer responder id", &self.responder_id)?;
        if self.submission_sequence == 0 {
            return Err(ClarificationError::new(
                "answer submission sequence must be positive",
            ));
        }
        Ok(())
    }
}

/// Claimed authority and grounding for a value answer. The referenced policy and evidence must be
/// revalidated against the current packet before the claim can be considered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnswerAuthorityClaim {
    SourceLocator {
        policy_id: String,
        policy_sha256: Sha256Digest,
        evidence: Vec<EvidenceLink>,
    },
    SourceSupplement {
        policy_id: String,
        policy_sha256: Sha256Digest,
        artifact: ArtifactRevisionRef,
        evidence: Vec<EvidenceLink>,
    },
    ExternalDesignDecision {
        policy_id: String,
        policy_sha256: Sha256Digest,
        decision_scope: Vec<DownstreamImpact>,
        rationale: String,
    },
}

impl AnswerAuthorityClaim {
    pub fn kind(&self) -> AnswerAuthorityKind {
        match self {
            Self::SourceLocator { .. } => AnswerAuthorityKind::SourceLocator,
            Self::SourceSupplement { .. } => AnswerAuthorityKind::SourceSupplement,
            Self::ExternalDesignDecision { .. } => AnswerAuthorityKind::ExternalDesignDecision,
        }
    }

    fn policy(&self) -> (&str, &Sha256Digest) {
        match self {
            Self::SourceLocator {
                policy_id,
                policy_sha256,
                ..
            }
            | Self::SourceSupplement {
                policy_id,
                policy_sha256,
                ..
            }
            | Self::ExternalDesignDecision {
                policy_id,
                policy_sha256,
                ..
            } => (policy_id, policy_sha256),
        }
    }

    fn validate(&self) -> ClarificationResult<()> {
        let (policy_id, _) = self.policy();
        validate_id("answer authority policy id", policy_id)?;
        match self {
            Self::SourceLocator { evidence, .. } => validate_evidence_set(evidence),
            Self::SourceSupplement {
                artifact, evidence, ..
            } => {
                artifact.validate("supplemental source artifact")?;
                validate_evidence_set(evidence)
            }
            Self::ExternalDesignDecision {
                decision_scope,
                rationale,
                ..
            } => {
                validate_nonempty_collection("external design decision scope", decision_scope)?;
                validate_unique_serialized("external design decision scope", decision_scope)?;
                for impact in decision_scope {
                    impact.validate()?;
                }
                validate_text(
                    "external design decision rationale",
                    rationale,
                    MAX_LONG_TEXT_BYTES,
                )
            }
        }
    }
}

/// Persisted answer exchange. Even a structurally valid current envelope grants no proof authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClarificationAnswerEnvelope {
    schema_version: u32,
    pub answer_id: String,
    pub revision: u32,
    pub question: QuestionRevisionRef,
    pub provenance: AnswerProvenance,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_claim: Option<AnswerAuthorityClaim>,
    pub disposition: AnswerDisposition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<AnswerRevisionRef>,
}

impl ClarificationAnswerEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        answer_id: impl Into<String>,
        revision: u32,
        question: QuestionRevisionRef,
        provenance: AnswerProvenance,
        authority_claim: Option<AnswerAuthorityClaim>,
        disposition: AnswerDisposition,
        supersedes: Option<AnswerRevisionRef>,
    ) -> ClarificationResult<Self> {
        let answer = Self {
            schema_version: CLARIFICATION_ANSWER_SCHEMA_VERSION,
            answer_id: answer_id.into(),
            revision,
            question,
            provenance,
            authority_claim,
            disposition,
            supersedes,
        };
        answer.validate()?;
        Ok(answer)
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn from_json(json: &str) -> ClarificationResult<Self> {
        let answer: Self = serde_json::from_str(json).map_err(|error| {
            ClarificationError::new(format!("cannot parse clarification answer: {error}"))
        })?;
        answer.validate()?;
        Ok(answer)
    }

    pub fn to_pretty_json(&self) -> ClarificationResult<String> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(|error| {
            ClarificationError::new(format!("cannot serialize clarification answer: {error}"))
        })
    }

    pub fn content_sha256(&self) -> ClarificationResult<Sha256Digest> {
        Sha256Digest::of_serializable(self).map_err(|error| {
            ClarificationError::new(format!("cannot hash clarification answer: {error}"))
        })
    }

    pub fn reference(&self) -> ClarificationResult<AnswerRevisionRef> {
        Ok(AnswerRevisionRef {
            answer_id: self.answer_id.clone(),
            revision: self.revision,
            content_sha256: self.content_sha256()?,
        })
    }

    pub fn validate(&self) -> ClarificationResult<()> {
        if self.schema_version != CLARIFICATION_ANSWER_SCHEMA_VERSION {
            return Err(ClarificationError::new(format!(
                "unsupported clarification answer schema {}; expected {CLARIFICATION_ANSWER_SCHEMA_VERSION}",
                self.schema_version
            )));
        }
        validate_id("clarification answer id", &self.answer_id)?;
        validate_positive_revision("clarification answer", self.revision)?;
        self.question.validate()?;
        self.provenance.validate()?;
        self.disposition.validate()?;
        if let Some(claim) = &self.authority_claim {
            claim.validate()?;
        }
        if self.disposition.is_value() != self.authority_claim.is_some() {
            return Err(ClarificationError::new(
                "a value answer requires one authority claim and a non-answer must not claim authority",
            ));
        }
        match (&self.supersedes, self.revision) {
            (None, 1) => {}
            (Some(previous), revision) if revision > 1 => {
                previous.validate()?;
                if previous.answer_id != self.answer_id || previous.revision >= revision {
                    return Err(ClarificationError::new(
                        "a revised answer must supersede an older revision of the same answer id",
                    ));
                }
            }
            (None, _) => {
                return Err(ClarificationError::new(
                    "an answer revision above one must identify the answer it supersedes",
                ));
            }
            (Some(_), 1) => {
                return Err(ClarificationError::new(
                    "answer revision one cannot supersede an earlier answer",
                ));
            }
            (Some(_), _) => unreachable!("positive revision already validated"),
        }
        Ok(())
    }

    /// Check immutable question/current-policy bindings without granting semantic or proof
    /// authority. Value/schema, grounding, responder authorization, source conflicts, and
    /// currentness still belong to the later validator.
    pub fn validate_current_binding(
        &self,
        packet: &ClarificationPacket,
    ) -> ClarificationResult<()> {
        packet.validate()?;
        self.validate()?;
        let question = packet.find_question(&self.question).ok_or_else(|| {
            ClarificationError::new(
                "clarification answer references a question absent from its packet",
            )
        })?;
        if question.reference()? != self.question {
            return Err(ClarificationError::new(
                "clarification answer references a stale question definition digest",
            ));
        }
        if !matches!(
            question.lifecycle.status,
            ClarificationStatus::Open | ClarificationStatus::AnsweredPendingValidation
        ) {
            return Err(ClarificationError::new(
                "clarification answer does not bind to an answerable current question state",
            ));
        }
        if let Some(claim) = &self.authority_claim {
            let (policy_id, policy_sha256) = claim.policy();
            let authorized =
                question
                    .definition
                    .answer_schema
                    .authorities
                    .iter()
                    .any(|requirement| {
                        requirement.kind == claim.kind()
                            && requirement.policy_id == policy_id
                            && requirement.policy_sha256 == *policy_sha256
                    });
            if !authorized {
                return Err(ClarificationError::new(
                    "clarification answer authority claim does not match the issued question policy",
                ));
            }
        }
        Ok(())
    }
}

/// Compatibility classification of an untrusted serialized envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClarificationCompatibility {
    CurrentEnvelopeRequiresValidation,
    UnsupportedLegacySchema { found: u32 },
    UnsupportedFutureSchema { found: u32 },
    InvalidCurrent { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClarificationCompatibilityDisposition {
    RequireValidation,
    InspectOnly,
    Reject,
}

impl ClarificationCompatibility {
    pub fn disposition(&self) -> ClarificationCompatibilityDisposition {
        match self {
            Self::CurrentEnvelopeRequiresValidation => {
                ClarificationCompatibilityDisposition::RequireValidation
            }
            Self::UnsupportedLegacySchema { .. } => {
                ClarificationCompatibilityDisposition::InspectOnly
            }
            Self::UnsupportedFutureSchema { .. } | Self::InvalidCurrent { .. } => {
                ClarificationCompatibilityDisposition::Reject
            }
        }
    }

    pub fn permits_canonical_authority(&self) -> bool {
        false
    }
}

pub fn assess_packet_compatibility(value: &serde_json::Value) -> ClarificationCompatibility {
    assess_compatibility::<ClarificationPacket>(
        value,
        CLARIFICATION_PACKET_SCHEMA_VERSION,
        |packet| packet.validate(),
    )
}

pub fn assess_answer_compatibility(value: &serde_json::Value) -> ClarificationCompatibility {
    assess_compatibility::<ClarificationAnswerEnvelope>(
        value,
        CLARIFICATION_ANSWER_SCHEMA_VERSION,
        |answer| answer.validate(),
    )
}

fn assess_compatibility<T>(
    value: &serde_json::Value,
    expected: u32,
    validate: impl FnOnce(&T) -> ClarificationResult<()>,
) -> ClarificationCompatibility
where
    T: for<'de> Deserialize<'de>,
{
    let Some(version_u64) = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
    else {
        return ClarificationCompatibility::InvalidCurrent {
            reason: "clarification envelope lacks an unsigned schema_version".to_string(),
        };
    };
    let Ok(version) = u32::try_from(version_u64) else {
        return ClarificationCompatibility::UnsupportedFutureSchema { found: u32::MAX };
    };
    if version < expected {
        return ClarificationCompatibility::UnsupportedLegacySchema { found: version };
    }
    if version > expected {
        return ClarificationCompatibility::UnsupportedFutureSchema { found: version };
    }
    let parsed = match serde_json::from_value::<T>(value.clone()) {
        Ok(parsed) => parsed,
        Err(error) => {
            return ClarificationCompatibility::InvalidCurrent {
                reason: error.to_string(),
            };
        }
    };
    match validate(&parsed) {
        Ok(()) => ClarificationCompatibility::CurrentEnvelopeRequiresValidation,
        Err(error) => ClarificationCompatibility::InvalidCurrent {
            reason: error.to_string(),
        },
    }
}

fn validate_evidence_set(evidence: &[EvidenceLink]) -> ClarificationResult<()> {
    validate_nonempty_collection("answer authority evidence", evidence)?;
    validate_unique_serialized("answer authority evidence", evidence)?;
    for link in evidence {
        link.validate()?;
    }
    Ok(())
}

fn validate_id(label: &str, value: &str) -> ClarificationResult<()> {
    validate_text(label, value, MAX_ID_BYTES)?;
    let first = value.bytes().next().unwrap_or_default();
    let last = value.bytes().next_back().unwrap_or_default();
    let valid_edge = |byte: u8| byte.is_ascii_lowercase() || byte.is_ascii_digit();
    if valid_edge(first)
        && valid_edge(last)
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
    {
        Ok(())
    } else {
        Err(ClarificationError::new(format!(
            "{label} must be a lowercase ASCII identifier using '.', '_', or '-' separators"
        )))
    }
}

fn validate_text(label: &str, value: &str, max_bytes: usize) -> ClarificationResult<()> {
    if value.trim().is_empty() {
        return Err(ClarificationError::new(format!(
            "{label} must not be blank"
        )));
    }
    if value.len() > max_bytes {
        return Err(ClarificationError::new(format!(
            "{label} is {} bytes, above its {max_bytes}-byte limit",
            value.len()
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(ClarificationError::new(format!(
            "{label} must not contain control characters"
        )));
    }
    Ok(())
}

fn validate_repo_relative_path(label: &str, value: &str) -> ClarificationResult<()> {
    validate_text(label, value, MAX_PATH_BYTES)?;
    let bytes = value.as_bytes();
    let windows_absolute = bytes.get(1) == Some(&b':');
    if value.starts_with('/')
        || value.starts_with('~')
        || value.contains('\\')
        || windows_absolute
        || value
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(ClarificationError::new(format!(
            "{label} must be a normalized repository-root-relative path"
        )));
    }
    Ok(())
}

fn validate_positive_revision(label: &str, revision: u32) -> ClarificationResult<()> {
    if revision == 0 {
        Err(ClarificationError::new(format!(
            "{label} revision/sequence must be positive"
        )))
    } else {
        Ok(())
    }
}

fn validate_nonempty_collection<T>(label: &str, values: &[T]) -> ClarificationResult<()> {
    validate_bounded_collection(label, values)?;
    if values.is_empty() {
        Err(ClarificationError::new(format!(
            "{label} must contain at least one item"
        )))
    } else {
        Ok(())
    }
}

fn validate_bounded_collection<T>(label: &str, values: &[T]) -> ClarificationResult<()> {
    if values.len() > MAX_COLLECTION_ITEMS {
        Err(ClarificationError::new(format!(
            "{label} contains {} items, above the {MAX_COLLECTION_ITEMS}-item limit",
            values.len()
        )))
    } else {
        Ok(())
    }
}

fn validate_unique_serialized<T: Serialize>(label: &str, values: &[T]) -> ClarificationResult<()> {
    let mut unique = BTreeSet::new();
    for value in values {
        let bytes = serde_json::to_vec(value).map_err(|error| {
            ClarificationError::new(format!("cannot serialize {label} for uniqueness: {error}"))
        })?;
        if !unique.insert(bytes) {
            return Err(ClarificationError::new(format!(
                "{label} contains a duplicate item"
            )));
        }
    }
    Ok(())
}

fn validate_optional_unit(unit: &Option<String>) -> ClarificationResult<()> {
    if let Some(unit) = unit {
        validate_text("answer unit", unit, MAX_SHORT_TEXT_BYTES)?;
    }
    Ok(())
}

fn validate_decimal(label: &str, value: &str) -> ClarificationResult<()> {
    validate_text(label, value, 128)?;
    let unsigned = value.strip_prefix('-').unwrap_or(value);
    let mut parts = unsigned.split('.');
    let integer = parts.next().unwrap_or_default();
    let fraction = parts.next();
    let valid_integer = !integer.is_empty()
        && integer.bytes().all(|byte| byte.is_ascii_digit())
        && (integer == "0" || !integer.starts_with('0'));
    let valid_fraction = fraction
        .is_none_or(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()));
    if valid_integer && valid_fraction && parts.next().is_none() && value != "-0" {
        Ok(())
    } else {
        Err(ClarificationError::new(format!(
            "{label} must use canonical decimal notation"
        )))
    }
}

fn validate_bit_vector(value: &str) -> ClarificationResult<()> {
    validate_text("bit-vector answer", value, MAX_LONG_TEXT_BYTES)?;
    let valid = value.strip_prefix("0b").is_some_and(|digits| {
        !digits.is_empty()
            && digits
                .bytes()
                .all(|byte| matches!(byte, b'0' | b'1' | b'x' | b'z'))
    }) || value.strip_prefix("0x").is_some_and(|digits| {
        !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_hexdigit())
    });
    if valid {
        Ok(())
    } else {
        Err(ClarificationError::new(
            "bit-vector answer must be a non-empty 0b binary/four-state or 0x hexadecimal literal",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(seed: &str) -> Sha256Digest {
        Sha256Digest::of_bytes(seed.as_bytes())
    }

    fn artifact(path: &str, seed: &str) -> ArtifactRevisionRef {
        ArtifactRevisionRef {
            path: path.to_string(),
            sha256: digest(seed),
        }
    }

    fn authority(kind: AnswerAuthorityKind) -> AnswerAuthorityRequirement {
        AnswerAuthorityRequirement {
            kind,
            policy_id: match kind {
                AnswerAuthorityKind::SourceLocator => "authority.source-locator.v1",
                AnswerAuthorityKind::SourceSupplement => "authority.source-supplement.v1",
                AnswerAuthorityKind::ExternalDesignDecision => "authority.external-design.v1",
            }
            .to_string(),
            policy_sha256: digest("authority-policy"),
        }
    }

    fn definition(
        id: &str,
        dependencies: Vec<QuestionRevisionRef>,
    ) -> ClarificationQuestionDefinition {
        ClarificationQuestionDefinition {
            question_id: id.to_string(),
            revision: 1,
            origin: ClarificationOrigin::ResidualDecision,
            missing_information_reason: MissingInformationReason::SourceAmbiguity,
            evidence: vec![EvidenceLink::ResidualDecision {
                stage: IrStage::IntentIr,
                artifact_sha256: digest("intent"),
                packet_id: "residual.clock-polarity".to_string(),
            }],
            proposition: "Which active level applies to the captured reset signal?".to_string(),
            why_automation_stopped: "Two current source regions state incompatible active levels."
                .to_string(),
            alternatives: vec![
                ClarificationAlternative {
                    alternative_id: "active-high".to_string(),
                    description: "Treat the reset as active high.".to_string(),
                    downstream_consequence: "Emit a positive-polarity reset contract.".to_string(),
                },
                ClarificationAlternative {
                    alternative_id: "active-low".to_string(),
                    description: "Treat the reset as active low.".to_string(),
                    downstream_consequence: "Emit a negative-polarity reset contract.".to_string(),
                },
            ],
            downstream_impacts: vec![DownstreamImpact {
                stage: IrStage::IsfAdapter,
                surface: "reset_binding".to_string(),
                stable_record_key: Some("reset.main".to_string()),
                blocking: true,
                explanation: "FSMGen ISF lowering requires an unambiguous reset polarity."
                    .to_string(),
            }],
            priority: ClarificationPriority::High,
            information_gain_score: 90,
            dependencies,
            autonomous_continuation: AutonomousContinuation::ContinueUnaffected,
            answer_schema: AnswerSchema {
                authorities: vec![authority(AnswerAuthorityKind::SourceLocator)],
                value: AnswerValueSchema::Enumeration {
                    choices: vec!["active-high".to_string(), "active-low".to_string()],
                    allow_multiple: false,
                },
                validation_rules: vec![
                    "The cited source region must belong to the current source revision."
                        .to_string(),
                    "The selected polarity must not conflict with a stronger current premise."
                        .to_string(),
                ],
                examples: vec!["active-low, citing the reset timing table".to_string()],
            },
        }
    }

    fn question(definition: ClarificationQuestionDefinition) -> ClarificationQuestion {
        ClarificationQuestion {
            definition,
            lifecycle: ClarificationLifecycle {
                status: ClarificationStatus::Open,
                state_sequence: 1,
                answer: None,
                superseded_by: None,
                reason: None,
            },
        }
    }

    fn packet(questions: Vec<ClarificationQuestion>) -> ClarificationPacket {
        ClarificationPacket::new(
            "clarification.packet-1",
            ClarificationArtifactContext {
                source: artifact("corpus/specification.pdf", "source"),
                current_stage: IrStage::IntentIr,
                current_artifact: artifact(
                    "generated/intent_ir/specification/intent_ir.json",
                    "intent",
                ),
                proof_ruleset_sha256: digest("ruleset"),
            },
            questions,
        )
        .unwrap()
    }

    fn answer(question: QuestionRevisionRef) -> ClarificationAnswerEnvelope {
        ClarificationAnswerEnvelope::new(
            "answer.reset-polarity",
            1,
            question,
            AnswerProvenance {
                responder_id: "operator.design-owner".to_string(),
                channel: AnswerChannel::InteractiveCli,
                submission_sequence: 1,
            },
            Some(AnswerAuthorityClaim::SourceLocator {
                policy_id: "authority.source-locator.v1".to_string(),
                policy_sha256: digest("authority-policy"),
                evidence: vec![EvidenceLink::SourceSpan {
                    artifact_sha256: digest("source"),
                    span_id: "span.reset-table".to_string(),
                    content_sha256: digest("reset is active low"),
                }],
            }),
            AnswerDisposition::Value {
                value: ClarificationValue::Enumeration {
                    choices: vec!["active-low".to_string()],
                },
            },
            None,
        )
        .unwrap()
    }

    #[test]
    fn packet_round_trip_preserves_definition_identity_and_requires_validation() {
        let packet = packet(vec![question(definition(
            "question.reset-polarity",
            vec![],
        ))]);
        let reference = packet.questions[0].reference().unwrap();
        let json = packet.to_pretty_json().unwrap();
        let decoded = ClarificationPacket::from_json(&json).unwrap();
        assert_eq!(decoded, packet);
        assert_eq!(decoded.questions[0].reference().unwrap(), reference);

        let value = serde_json::from_str(&json).unwrap();
        let compatibility = assess_packet_compatibility(&value);
        assert_eq!(
            compatibility,
            ClarificationCompatibility::CurrentEnvelopeRequiresValidation
        );
        assert_eq!(
            compatibility.disposition(),
            ClarificationCompatibilityDisposition::RequireValidation
        );
        assert!(!compatibility.permits_canonical_authority());
    }

    #[test]
    fn packet_requires_dependency_order_and_exact_definition_digest() {
        let first = question(definition("question.first", vec![]));
        let first_ref = first.reference().unwrap();
        let second = question(definition("question.second", vec![first_ref.clone()]));
        packet(vec![first.clone(), second.clone()]);

        let error = ClarificationPacket::new(
            "clarification.out-of-order",
            ClarificationArtifactContext {
                source: artifact("corpus/specification.pdf", "source"),
                current_stage: IrStage::IntentIr,
                current_artifact: artifact(
                    "generated/intent_ir/specification/intent_ir.json",
                    "intent",
                ),
                proof_ruleset_sha256: digest("ruleset"),
            },
            vec![second, first],
        )
        .unwrap_err();
        assert!(error.to_string().contains("not dependency-ordered"));

        let mut stale = first_ref;
        stale.definition_sha256 = digest("stale-definition");
        let error = ClarificationPacket::new(
            "clarification.stale-dependency",
            ClarificationArtifactContext {
                source: artifact("corpus/specification.pdf", "source"),
                current_stage: IrStage::IntentIr,
                current_artifact: artifact(
                    "generated/intent_ir/specification/intent_ir.json",
                    "intent",
                ),
                proof_ruleset_sha256: digest("ruleset"),
            },
            vec![
                question(definition("question.first", vec![])),
                question(definition("question.second", vec![stale])),
            ],
        )
        .unwrap_err();
        assert!(error.to_string().contains("stale dependency"));
    }

    #[test]
    fn artifact_paths_are_repository_relative() {
        let mut packet = packet(vec![question(definition("question.path", vec![]))]);
        packet.context.source.path = "/tmp/specification.pdf".to_string();
        assert!(
            packet
                .validate()
                .unwrap_err()
                .to_string()
                .contains("repository-root-relative")
        );
        packet.context.source.path = "../specification.pdf".to_string();
        assert!(
            packet
                .validate()
                .unwrap_err()
                .to_string()
                .contains("repository-root-relative")
        );
        packet.context.source.path = "C:\\specification.pdf".to_string();
        assert!(
            packet
                .validate()
                .unwrap_err()
                .to_string()
                .contains("repository-root-relative")
        );
    }

    #[test]
    fn lifecycle_shape_fails_closed() {
        let mut question = question(definition("question.lifecycle", vec![]));
        question.lifecycle.status = ClarificationStatus::Accepted;
        assert!(
            question
                .validate()
                .unwrap_err()
                .to_string()
                .contains("inconsistent")
        );

        question.lifecycle.status = ClarificationStatus::Superseded;
        question.lifecycle.reason = Some("Reissued against a new source revision.".to_string());
        question.lifecycle.superseded_by = Some(QuestionRevisionRef {
            question_id: "question.lifecycle".to_string(),
            revision: 2,
            definition_sha256: digest("new-question"),
        });
        question.validate().unwrap();
    }

    #[test]
    fn answer_round_trip_binds_exact_question_and_policy_without_granting_authority() {
        let packet = packet(vec![question(definition(
            "question.reset-polarity",
            vec![],
        ))]);
        let answer = answer(packet.questions[0].reference().unwrap());
        answer.validate_current_binding(&packet).unwrap();
        let json = answer.to_pretty_json().unwrap();
        let decoded = ClarificationAnswerEnvelope::from_json(&json).unwrap();
        assert_eq!(decoded, answer);
        assert_eq!(
            decoded.reference().unwrap().content_sha256,
            answer.content_sha256().unwrap()
        );

        let value = serde_json::from_str(&json).unwrap();
        let compatibility = assess_answer_compatibility(&value);
        assert_eq!(
            compatibility,
            ClarificationCompatibility::CurrentEnvelopeRequiresValidation
        );
        assert!(!compatibility.permits_canonical_authority());
    }

    #[test]
    fn stale_question_and_policy_bindings_reject() {
        let packet = packet(vec![question(definition(
            "question.reset-polarity",
            vec![],
        ))]);
        let mut stale_question = packet.questions[0].reference().unwrap();
        stale_question.definition_sha256 = digest("stale-question");
        let stale_answer = answer(stale_question);
        assert!(
            stale_answer
                .validate_current_binding(&packet)
                .unwrap_err()
                .to_string()
                .contains("stale")
        );

        let mut answer = answer(packet.questions[0].reference().unwrap());
        let Some(AnswerAuthorityClaim::SourceLocator { policy_sha256, .. }) =
            answer.authority_claim.as_mut()
        else {
            panic!("source-locator answer")
        };
        *policy_sha256 = digest("wrong-policy");
        assert!(
            answer
                .validate_current_binding(&packet)
                .unwrap_err()
                .to_string()
                .contains("does not match")
        );
    }

    #[test]
    fn non_answers_are_explicit_and_cannot_claim_authority() {
        let packet = packet(vec![question(definition("question.unknown", vec![]))]);
        let non_answer = ClarificationAnswerEnvelope::new(
            "answer.unknown",
            1,
            packet.questions[0].reference().unwrap(),
            AnswerProvenance {
                responder_id: "operator.design-owner".to_string(),
                channel: AnswerChannel::Api,
                submission_sequence: 2,
            },
            None,
            AnswerDisposition::Unknown {
                explanation: "The specification owner has not decided this behavior.".to_string(),
            },
            None,
        )
        .unwrap();
        non_answer.validate_current_binding(&packet).unwrap();

        let error = ClarificationAnswerEnvelope::new(
            "answer.invalid-unknown",
            1,
            packet.questions[0].reference().unwrap(),
            non_answer.provenance.clone(),
            Some(AnswerAuthorityClaim::SourceLocator {
                policy_id: "authority.source-locator.v1".to_string(),
                policy_sha256: digest("authority-policy"),
                evidence: vec![packet.questions[0].definition.evidence[0].clone()],
            }),
            AnswerDisposition::Defer {
                explanation: "Revisit after the implementation profile is chosen.".to_string(),
            },
            None,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("non-answer must not claim authority")
        );
    }

    #[test]
    fn revised_answers_preserve_exact_supersession() {
        let packet = packet(vec![question(definition(
            "question.reset-polarity",
            vec![],
        ))]);
        let first = answer(packet.questions[0].reference().unwrap());
        let mut second = first.clone();
        second.revision = 2;
        second.supersedes = Some(first.reference().unwrap());
        second.validate().unwrap();

        second.supersedes.as_mut().unwrap().answer_id = "answer.other".to_string();
        assert!(
            second
                .validate()
                .unwrap_err()
                .to_string()
                .contains("same answer id")
        );
    }

    #[test]
    fn legacy_future_malformed_and_unknown_fields_fail_closed() {
        let packet = packet(vec![question(definition("question.compatibility", vec![]))]);
        let mut value = serde_json::to_value(packet).unwrap();

        value["schema_version"] = serde_json::json!(0);
        assert_eq!(
            assess_packet_compatibility(&value),
            ClarificationCompatibility::UnsupportedLegacySchema { found: 0 }
        );
        value["schema_version"] = serde_json::json!(2);
        assert_eq!(
            assess_packet_compatibility(&value),
            ClarificationCompatibility::UnsupportedFutureSchema { found: 2 }
        );
        value["schema_version"] = serde_json::json!(1);
        value["unexpected"] = serde_json::json!(true);
        assert!(matches!(
            assess_packet_compatibility(&value),
            ClarificationCompatibility::InvalidCurrent { .. }
        ));
        value.as_object_mut().unwrap().remove("schema_version");
        let compatibility = assess_packet_compatibility(&value);
        assert!(matches!(
            compatibility,
            ClarificationCompatibility::InvalidCurrent { .. }
        ));
        assert_eq!(
            compatibility.disposition(),
            ClarificationCompatibilityDisposition::Reject
        );
        assert!(!compatibility.permits_canonical_authority());
    }

    #[test]
    fn recursive_schema_and_values_are_bounded() {
        let mut schema = AnswerValueSchema::BitVector { width: 0 };
        assert!(
            schema
                .validate(0)
                .unwrap_err()
                .to_string()
                .contains("width")
        );
        schema = AnswerValueSchema::List {
            item: Box::new(AnswerValueSchema::Boolean),
            min_items: 2,
            max_items: 1,
        };
        assert!(
            schema
                .validate(0)
                .unwrap_err()
                .to_string()
                .contains("bounds")
        );
        assert!(
            ClarificationValue::BitVector {
                value: "0b10xz".to_string()
            }
            .validate(0)
            .is_ok()
        );
        assert!(
            ClarificationValue::BitVector {
                value: "2'b10".to_string()
            }
            .validate(0)
            .is_err()
        );

        let mut packet = packet(vec![question(definition("question.score", vec![]))]);
        packet.questions[0].definition.information_gain_score = 101;
        assert!(
            packet
                .validate()
                .unwrap_err()
                .to_string()
                .contains("information-gain")
        );
    }
}
