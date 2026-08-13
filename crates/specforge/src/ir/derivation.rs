//! Trusted derivation kernel for specification-instance-neutral production authority.
//!
//! This module separates three things that ordinary `String` provenance cannot separate:
//!
//! - document-owned spellings are opaque identity atoms, not semantic decision inputs;
//! - registered rules produce grounded proposals, never canonical authority directly; and
//! - the small promotion kernel checks typed current-chain premises and emits a proof ledger.
//!
//! Rule-family migration is deliberately separate. This module supplies the sealed substrate;
//! production grammars and canonical artifact builders move onto it in the next migration lane.
//!
//! The opacity boundary is compiler-visible to downstream crates. In particular, an opaque symbol
//! is not displayable, serializable, orderable by spelling, or convertible to a string:
//!
//! ```compile_fail
//! use specforge_core::ir::derivation::OpaqueSymbol;
//!
//! fn leak(symbol: &OpaqueSymbol) -> String {
//!     symbol.to_string()
//! }
//! ```
//!
//! ```compile_fail
//! use specforge_core::ir::derivation::OpaqueSymbol;
//!
//! fn serialize(symbol: &OpaqueSymbol) -> String {
//!     serde_json::to_string(symbol).unwrap()
//! }
//! ```
//!
//! ```compile_fail
//! use specforge_core::ir::derivation::OpaqueSymbol;
//!
//! fn spelling_order(left: &OpaqueSymbol, right: &OpaqueSymbol) -> std::cmp::Ordering {
//!     left.cmp(right)
//! }
//! ```
//!
//! Capability tokens and grounded proposals also cannot be minted by application or conformance
//! code:
//!
//! ```compile_fail
//! use specforge_core::ir::derivation::GrammarCapability;
//!
//! let _forged = GrammarCapability::new();
//! ```
//!
//! ```compile_fail
//! use specforge_core::ir::derivation::GroundedProposal;
//!
//! let _laundered: GroundedProposal<u32> = serde_json::from_str("{}").unwrap();
//! ```

use crate::ir::IrStage;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Current persisted proof-ledger schema.
pub const PROOF_LEDGER_SCHEMA_VERSION: u32 = 1;

/// Current registered-rule descriptor schema.
pub const RULE_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

include!(concat!(env!("OUT_DIR"), "/production_semantic_digests.rs"));

/// Digest the compiler-token closure of the registered production verifier for one IR stage.
///
/// The build-time derivation roots at the canonical production registry constructor, recursively
/// follows its stage-local production items (including the verifier it binds), and incorporates
/// the complete production trusted-kernel token graph. Rust comments, doc attributes, formatting,
/// `cfg(test)` items, and conformance sources are not inputs. The returned value is therefore
/// implementation authority, not a whole-file freshness proxy.
pub(crate) fn production_semantic_implementation_digest(
    stage: IrStage,
) -> DerivationResult<Sha256Digest> {
    debug_assert_eq!(PRODUCTION_SEMANTIC_DIGEST_SCHEMA_VERSION, 1);
    debug_assert_eq!(PRODUCTION_KERNEL_SEMANTIC_SHA256.len(), 64);
    let digest = match stage {
        IrStage::SourceIr => SOURCE_PRODUCTION_SEMANTIC_SHA256,
        IrStage::EvidenceIr => EVIDENCE_PRODUCTION_SEMANTIC_SHA256,
        IrStage::SemanticIr => SEMANTIC_PRODUCTION_SEMANTIC_SHA256,
        IrStage::IntentIr => INTENT_PRODUCTION_SEMANTIC_SHA256,
        IrStage::IsfAdapter => ADAPTER_PRODUCTION_SEMANTIC_SHA256,
    };
    Sha256Digest::try_from(digest.to_string()).map_err(DerivationError::new)
}

/// A validated lowercase SHA-256 digest.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Sha256Digest(String);

impl Sha256Digest {
    /// Hash exact bytes.
    pub fn of_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        Self(format!("{digest:x}"))
    }

    /// Hash the deterministic JSON representation supplied by a canonical record producer.
    pub fn of_serializable<T: Serialize>(value: &T) -> DerivationResult<Self> {
        let bytes = serde_json::to_vec(value).map_err(|error| {
            DerivationError::new(format!("cannot serialize conclusion: {error}"))
        })?;
        Ok(Self::of_bytes(&bytes))
    }

    /// The validated hexadecimal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Sha256Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Sha256Digest")
            .field(&self.0)
            .finish()
    }
}

impl TryFrom<String> for Sha256Digest {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            Ok(Self(value))
        } else {
            Err("SHA-256 digest must be exactly 64 lowercase hexadecimal characters".to_string())
        }
    }
}

impl From<Sha256Digest> for String {
    fn from(value: Sha256Digest) -> Self {
        value.0
    }
}

/// Stable namespace for atoms captured from one current document chain.
///
/// The digest is an identity namespace only. No API interprets it as a filename, title, vendor,
/// protocol family, or semantic feature.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentScope(Sha256Digest);

impl DocumentScope {
    pub fn digest(&self) -> &Sha256Digest {
        &self.0
    }

    fn from_capture_digest(digest: Sha256Digest) -> Self {
        Self(digest)
    }
}

/// Public, spelling-free identity of a document-owned symbol.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolIdentity {
    scope: DocumentScope,
    source_ordinal: u32,
}

impl SymbolIdentity {
    pub fn scope(&self) -> &DocumentScope {
        &self.scope
    }

    pub fn source_ordinal(&self) -> u32 {
        self.source_ordinal
    }
}

/// A document-owned symbol whose spelling is unavailable to ordinary semantic code.
///
/// Equality and hashing use only the stable scope/ordinal identity. `Debug` is intentionally
/// redacted, and this type deliberately implements neither `Display`, `AsRef<str>`, `Deref`,
/// `Ord`, nor Serde traits.
#[derive(Clone)]
pub struct OpaqueSymbol {
    identity: SymbolIdentity,
    // The spelling becomes live when the later persistence/presentation/lowering stage leaves
    // `.e.iv.ii`; SourceIR deliberately cannot read it through ordinary semantic code.
    #[allow(dead_code)]
    spelling: Box<str>,
}

impl OpaqueSymbol {
    pub fn identity(&self) -> &SymbolIdentity {
        &self.identity
    }

    pub fn same_identity(&self, other: &Self) -> bool {
        self.identity == other.identity
    }
}

impl fmt::Debug for OpaqueSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueSymbol")
            .field("identity", &self.identity)
            .field("spelling", &"<opaque>")
            .finish()
    }
}

impl PartialEq for OpaqueSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.same_identity(other)
    }
}

impl Eq for OpaqueSymbol {}

impl Hash for OpaqueSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identity.hash(state);
    }
}

/// Spelling-free identity for a source document.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentIdentityAtom {
    scope: DocumentScope,
}

impl DocumentIdentityAtom {
    pub fn scope(&self) -> &DocumentScope {
        &self.scope
    }
}

/// A document identity whose key/title/path spelling is presentation-only.
#[derive(Clone)]
pub struct OpaqueDocumentIdentity {
    identity: DocumentIdentityAtom,
    // Kept sealed until the later presentation/persistence migration consumes this capability.
    #[allow(dead_code)]
    private_label: Box<str>,
}

impl OpaqueDocumentIdentity {
    pub fn identity(&self) -> &DocumentIdentityAtom {
        &self.identity
    }

    pub fn same_identity(&self, other: &Self) -> bool {
        self.identity == other.identity
    }
}

impl fmt::Debug for OpaqueDocumentIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueDocumentIdentity")
            .field("identity", &self.identity)
            .field("label", &"<opaque>")
            .finish()
    }
}

impl PartialEq for OpaqueDocumentIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.same_identity(other)
    }
}

impl Eq for OpaqueDocumentIdentity {}

impl Hash for OpaqueDocumentIdentity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identity.hash(state);
    }
}

#[derive(Default)]
struct CapabilitySeal;

/// Sealed token for registered grammar access to current-document text and symbol spelling.
pub struct GrammarCapability<'kernel> {
    _seal: &'kernel CapabilitySeal,
}

/// Sealed token for transparent persisted spelling transport.
pub struct PersistenceCapability<'kernel> {
    _seal: &'kernel CapabilitySeal,
}

/// Sealed token for diagnostics and UI rendering.
pub struct PresentationCapability<'kernel> {
    _seal: &'kernel CapabilitySeal,
}

/// Sealed token for target-safe encoding after semantics are proved.
pub struct LoweringCapability<'kernel> {
    _seal: &'kernel CapabilitySeal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub(crate) struct PersistedOpaqueSymbol {
    identity: SymbolIdentity,
    spelling: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub(crate) struct PersistedOpaqueDocumentIdentity {
    identity: DocumentIdentityAtom,
    private_label: String,
}

// These sealed operations become reachable as the later IR-stage children migrate. Keeping the
// suppression on the capability implementation (not the module) lets new unrelated dead code fail.
#[allow(dead_code)]
impl PersistenceCapability<'_> {
    pub(crate) fn persist_symbol(&self, symbol: &OpaqueSymbol) -> PersistedOpaqueSymbol {
        PersistedOpaqueSymbol {
            identity: symbol.identity.clone(),
            spelling: symbol.spelling.to_string(),
        }
    }

    pub(crate) fn restore_symbol(&self, persisted: PersistedOpaqueSymbol) -> OpaqueSymbol {
        OpaqueSymbol {
            identity: persisted.identity,
            spelling: persisted.spelling.into_boxed_str(),
        }
    }

    pub(crate) fn persist_document(
        &self,
        document: &OpaqueDocumentIdentity,
    ) -> PersistedOpaqueDocumentIdentity {
        PersistedOpaqueDocumentIdentity {
            identity: document.identity.clone(),
            private_label: document.private_label.to_string(),
        }
    }

    pub(crate) fn restore_document(
        &self,
        persisted: PersistedOpaqueDocumentIdentity,
    ) -> OpaqueDocumentIdentity {
        OpaqueDocumentIdentity {
            identity: persisted.identity,
            private_label: persisted.private_label.into_boxed_str(),
        }
    }
}

#[allow(dead_code)]
impl PresentationCapability<'_> {
    pub(crate) fn symbol<'a>(&self, symbol: &'a OpaqueSymbol) -> &'a str {
        &symbol.spelling
    }

    pub(crate) fn document<'a>(&self, document: &'a OpaqueDocumentIdentity) -> &'a str {
        &document.private_label
    }
}

#[allow(dead_code)]
impl LoweringCapability<'_> {
    pub(crate) fn encode_symbol<T>(
        &self,
        symbol: &OpaqueSymbol,
        encoder: impl FnOnce(&str) -> T,
    ) -> T {
        encoder(&symbol.spelling)
    }
}

/// Source-order interner. Duplicate spelling denotes the same current-document atom; the first
/// occurrence fixes its ordinal. No spelling-based ordering escapes this type.
struct SymbolInterner {
    #[allow(dead_code)]
    scope: DocumentScope,
    #[allow(dead_code)]
    by_spelling: BTreeMap<String, OpaqueSymbol>,
    #[allow(dead_code)]
    next_ordinal: u32,
}

impl SymbolInterner {
    fn new(scope: DocumentScope) -> Self {
        Self {
            scope,
            by_spelling: BTreeMap::new(),
            next_ordinal: 0,
        }
    }

    #[allow(dead_code)]
    fn intern(&mut self, spelling: &str) -> DerivationResult<OpaqueSymbol> {
        if spelling.is_empty() {
            return Err(DerivationError::new(
                "opaque symbol spelling cannot be empty",
            ));
        }
        if let Some(existing) = self.by_spelling.get(spelling) {
            return Ok(existing.clone());
        }
        let ordinal = self.next_ordinal;
        self.next_ordinal = self
            .next_ordinal
            .checked_add(1)
            .ok_or_else(|| DerivationError::new("opaque symbol ordinal overflow"))?;
        let symbol = OpaqueSymbol {
            identity: SymbolIdentity {
                scope: self.scope.clone(),
                source_ordinal: ordinal,
            },
            spelling: spelling.into(),
        };
        self.by_spelling
            .insert(spelling.to_string(), symbol.clone());
        Ok(symbol)
    }
}

/// Stable registered-rule id.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct RuleId(String);

impl RuleId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for RuleId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = (3..=128).contains(&value.len())
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_' | b'-')
            })
            && value.bytes().any(|byte| byte == b'.');
        if valid {
            Ok(Self(value))
        } else {
            Err("rule id must be a 3..128-byte lowercase dotted identifier".to_string())
        }
    }
}

impl From<RuleId> for String {
    fn from(value: RuleId) -> Self {
        value.0
    }
}

/// Typed premise kinds understood by the trusted kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PremiseKind {
    SourceSpan,
    TableCell,
    VisualRegion,
    UpstreamClaim,
    RegisteredDerivation,
    GroundedModelProposal,
    ValidatedPrior,
    UniversalAxiom,
}

/// Scope contract attached to a learned prior before it may become a premise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatedPriorScope {
    CurrentDocument,
    GlobalIdentityIndependent,
}

/// A typed, digest-bound reference to current-chain evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PremiseRef {
    SourceSpan {
        scope: DocumentScope,
        span_id: String,
        content_sha256: Sha256Digest,
    },
    TableCell {
        scope: DocumentScope,
        table_id: String,
        row: u32,
        column: u32,
        content_sha256: Sha256Digest,
    },
    VisualRegion {
        scope: DocumentScope,
        region_id: String,
        content_sha256: Sha256Digest,
    },
    UpstreamClaim {
        address: ClaimAddress,
        conclusion_sha256: Sha256Digest,
    },
    RegisteredDerivation {
        scope: DocumentScope,
        derivation_id: String,
        output_sha256: Sha256Digest,
        inputs_sha256: Sha256Digest,
    },
    GroundedModelProposal {
        scope: DocumentScope,
        proposal_id: String,
        proposal_sha256: Sha256Digest,
    },
    ValidatedPrior {
        prior_id: String,
        prior_sha256: Sha256Digest,
        validation_sha256: Sha256Digest,
        scope_contract: ValidatedPriorScope,
    },
    UniversalAxiom {
        axiom_id: String,
        version: u32,
    },
}

impl PremiseRef {
    pub fn kind(&self) -> PremiseKind {
        match self {
            Self::SourceSpan { .. } => PremiseKind::SourceSpan,
            Self::TableCell { .. } => PremiseKind::TableCell,
            Self::VisualRegion { .. } => PremiseKind::VisualRegion,
            Self::UpstreamClaim { .. } => PremiseKind::UpstreamClaim,
            Self::RegisteredDerivation { .. } => PremiseKind::RegisteredDerivation,
            Self::GroundedModelProposal { .. } => PremiseKind::GroundedModelProposal,
            Self::ValidatedPrior { .. } => PremiseKind::ValidatedPrior,
            Self::UniversalAxiom { .. } => PremiseKind::UniversalAxiom,
        }
    }
}

/// Exact address of one canonical conclusion in an artifact proof ledger.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimAddress {
    stage: IrStage,
    surface: String,
    stable_record_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    field_path: Option<String>,
}

impl ClaimAddress {
    pub fn new(
        stage: IrStage,
        surface: impl Into<String>,
        stable_record_key: impl Into<String>,
        field_path: Option<String>,
    ) -> DerivationResult<Self> {
        let address = Self {
            stage,
            surface: surface.into(),
            stable_record_key: stable_record_key.into(),
            field_path,
        };
        address.validate()?;
        Ok(address)
    }

    pub fn stage(&self) -> IrStage {
        self.stage
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn stable_record_key(&self) -> &str {
        &self.stable_record_key
    }

    pub fn field_path(&self) -> Option<&str> {
        self.field_path.as_deref()
    }

    fn validate(&self) -> DerivationResult<()> {
        validate_identifier("claim surface", &self.surface, 128)?;
        validate_nonempty_bounded("stable record key", &self.stable_record_key, 512)?;
        if let Some(path) = &self.field_path {
            validate_identifier("claim field path", path, 256)?;
        }
        Ok(())
    }
}

/// The only symbol operations a proof may declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolUseKind {
    Copied,
    ExactIdentityCompared,
    IntroducedFromPremise,
    TargetEncoded,
}

/// Spelling-free record of how one source-owned symbol contributed to a conclusion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolUse {
    identity: SymbolIdentity,
    use_kind: SymbolUseKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    introduced_from_premise: Option<u32>,
}

impl SymbolUse {
    pub fn copied(symbol: &OpaqueSymbol) -> Self {
        Self {
            identity: symbol.identity.clone(),
            use_kind: SymbolUseKind::Copied,
            introduced_from_premise: None,
        }
    }

    pub fn exact_compared(symbol: &OpaqueSymbol) -> Self {
        Self {
            identity: symbol.identity.clone(),
            use_kind: SymbolUseKind::ExactIdentityCompared,
            introduced_from_premise: None,
        }
    }

    pub fn introduced(symbol: &OpaqueSymbol, premise_index: u32) -> Self {
        Self {
            identity: symbol.identity.clone(),
            use_kind: SymbolUseKind::IntroducedFromPremise,
            introduced_from_premise: Some(premise_index),
        }
    }

    pub fn target_encoded(symbol: &OpaqueSymbol) -> Self {
        Self {
            identity: symbol.identity.clone(),
            use_kind: SymbolUseKind::TargetEncoded,
            introduced_from_premise: None,
        }
    }

    pub fn identity(&self) -> &SymbolIdentity {
        &self.identity
    }

    pub fn use_kind(&self) -> SymbolUseKind {
        self.use_kind
    }
}

/// Information-flow class declared by a registered rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolCapabilityClass {
    SymbolBlind,
    ExactIdentityOnly,
    GrammarIntroduces,
    LosslessCarry,
    MergeOrConflict,
    Residual,
    TargetLowering,
}

/// Executable alpha-equivalence obligation category. There is deliberately no `None` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlphaObligation {
    ByteIdenticalNonSymbolOutput,
    IdentityGraphInvariant,
    IntroducedSymbolsPreserveOrigins,
    LosslessTopologyInvariant,
    MergeConflictTopologyInvariant,
    ResidualTopologyInvariant,
    TargetSafeRenaming,
}

/// Compatibility contract of a rule implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleCompatibility {
    CurrentOnly,
    LosslessCarry,
}

/// Closed metadata for one production grammar, inference, carry, residual, or lowering rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleDescriptor {
    schema_version: u32,
    rule_id: RuleId,
    version: u32,
    implementation_module: String,
    implementation_sha256: Sha256Digest,
    premise_kinds: BTreeSet<PremiseKind>,
    conclusion_stage: IrStage,
    conclusion_surface: String,
    symbol_capability: SymbolCapabilityClass,
    alpha_obligation: AlphaObligation,
    compatibility: RuleCompatibility,
}

impl RuleDescriptor {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        rule_id: RuleId,
        version: u32,
        implementation_module: impl Into<String>,
        implementation_sha256: Sha256Digest,
        premise_kinds: impl IntoIterator<Item = PremiseKind>,
        conclusion_stage: IrStage,
        conclusion_surface: impl Into<String>,
        symbol_capability: SymbolCapabilityClass,
        alpha_obligation: AlphaObligation,
        compatibility: RuleCompatibility,
    ) -> DerivationResult<Self> {
        let descriptor = Self {
            schema_version: RULE_DESCRIPTOR_SCHEMA_VERSION,
            rule_id,
            version,
            implementation_module: implementation_module.into(),
            implementation_sha256,
            premise_kinds: premise_kinds.into_iter().collect(),
            conclusion_stage,
            conclusion_surface: conclusion_surface.into(),
            symbol_capability,
            alpha_obligation,
            compatibility,
        };
        descriptor.validate()?;
        Ok(descriptor)
    }

    pub fn rule_id(&self) -> &RuleId {
        &self.rule_id
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn implementation_module(&self) -> &str {
        &self.implementation_module
    }

    pub fn implementation_sha256(&self) -> &Sha256Digest {
        &self.implementation_sha256
    }

    pub fn premise_kinds(&self) -> &BTreeSet<PremiseKind> {
        &self.premise_kinds
    }

    pub fn conclusion_stage(&self) -> IrStage {
        self.conclusion_stage
    }

    pub fn conclusion_surface(&self) -> &str {
        &self.conclusion_surface
    }

    pub fn symbol_capability(&self) -> SymbolCapabilityClass {
        self.symbol_capability
    }

    pub fn alpha_obligation(&self) -> AlphaObligation {
        self.alpha_obligation
    }

    #[cfg(test)]
    pub(crate) fn compatibility(&self) -> RuleCompatibility {
        self.compatibility
    }

    fn validate(&self) -> DerivationResult<()> {
        if self.schema_version != RULE_DESCRIPTOR_SCHEMA_VERSION {
            return Err(DerivationError::new(format!(
                "rule '{}' uses unsupported descriptor schema {}",
                self.rule_id.as_str(),
                self.schema_version
            )));
        }
        if self.version == 0 {
            return Err(DerivationError::new(format!(
                "rule '{}' version must be positive",
                self.rule_id.as_str()
            )));
        }
        if !self.implementation_module.starts_with("crate::ir::") {
            return Err(DerivationError::new(format!(
                "rule '{}' implementation must be a core crate::ir module",
                self.rule_id.as_str()
            )));
        }
        if self.premise_kinds.is_empty() {
            return Err(DerivationError::new(format!(
                "rule '{}' must declare at least one premise kind",
                self.rule_id.as_str()
            )));
        }
        validate_identifier("rule conclusion surface", &self.conclusion_surface, 128)?;
        validate_alpha_pair(self.symbol_capability, self.alpha_obligation)
    }

    /// Qualify the premise and compatibility shape required by this rule's alpha contract.
    ///
    /// This CI-only structural oracle deliberately does not participate in production-semantic proof
    /// identity. Population-level metamorphic behavior remains a separate qualification boundary.
    #[cfg(test)]
    pub(crate) fn validate_structural_alpha_obligation(&self) -> DerivationResult<()> {
        validate_alpha_pair(self.symbol_capability, self.alpha_obligation)?;

        let has = |kind| self.premise_kinds.contains(&kind);
        let direct_grounding = has(PremiseKind::SourceSpan)
            || has(PremiseKind::TableCell)
            || has(PremiseKind::VisualRegion)
            || has(PremiseKind::GroundedModelProposal);
        let valid = match self.alpha_obligation {
            AlphaObligation::ByteIdenticalNonSymbolOutput => {
                self.compatibility == RuleCompatibility::CurrentOnly
            }
            AlphaObligation::IdentityGraphInvariant => {
                self.compatibility == RuleCompatibility::CurrentOnly
            }
            AlphaObligation::IntroducedSymbolsPreserveOrigins => {
                self.compatibility == RuleCompatibility::CurrentOnly && direct_grounding
            }
            AlphaObligation::LosslessTopologyInvariant => {
                self.compatibility == RuleCompatibility::LosslessCarry
                    && has(PremiseKind::UpstreamClaim)
            }
            AlphaObligation::MergeConflictTopologyInvariant => {
                self.compatibility == RuleCompatibility::CurrentOnly
                    && has(PremiseKind::RegisteredDerivation)
                    && (has(PremiseKind::UpstreamClaim) || direct_grounding)
            }
            AlphaObligation::ResidualTopologyInvariant => {
                self.compatibility == RuleCompatibility::CurrentOnly
                    && ((has(PremiseKind::SourceSpan) && has(PremiseKind::UniversalAxiom))
                        || (has(PremiseKind::RegisteredDerivation)
                            && has(PremiseKind::UpstreamClaim)))
            }
            AlphaObligation::TargetSafeRenaming => {
                self.compatibility == RuleCompatibility::CurrentOnly
                    && has(PremiseKind::RegisteredDerivation)
                    && has(PremiseKind::UpstreamClaim)
            }
        };
        if valid {
            Ok(())
        } else {
            Err(DerivationError::new(format!(
                "rule '{}' does not satisfy the premise/compatibility shape required by its structural alpha obligation",
                self.rule_id.as_str()
            )))
        }
    }
}

/// Exact inputs supplied to one executable registered-rule verifier.
///
/// A persisted rule id and matching hashes are not sufficient authority: an editor could
/// recompute both. The verifier bound into the current binary must accept the exact conclusion
/// relation against the captured and already-verified premise bytes.
pub(crate) struct RuleVerificationContext<'a> {
    proof: &'a ClaimProof,
    conclusion_json: &'a [u8],
    evidence: &'a EvidenceCatalog,
    verified_prefix: &'a BTreeMap<ClaimAddress, Vec<u8>>,
    verified_local: &'a BTreeMap<ClaimAddress, Vec<u8>>,
}

impl RuleVerificationContext<'_> {
    pub(crate) fn proof(&self) -> &ClaimProof {
        self.proof
    }

    pub(crate) fn conclusion_json(&self) -> &[u8] {
        self.conclusion_json
    }

    pub(crate) fn premise_bytes(&self, index: usize) -> DerivationResult<Option<&[u8]>> {
        let premise =
            self.proof.premises.get(index).ok_or_else(|| {
                DerivationError::new("rule verifier premise index is out of bounds")
            })?;
        self.evidence
            .premise_bytes(premise, self.verified_prefix, self.verified_local)
    }
}

pub(crate) type RuleVerifier = fn(RuleVerificationContext<'_>) -> DerivationResult<()>;

/// A descriptor paired with the executable verifier that gives the rule semantic authority.
#[derive(Clone)]
pub(crate) struct RuleRegistration {
    descriptor: RuleDescriptor,
    verifier: RuleVerifier,
}

impl RuleRegistration {
    pub(crate) fn new(descriptor: RuleDescriptor, verifier: RuleVerifier) -> Self {
        Self {
            descriptor,
            verifier,
        }
    }
}

/// Registered universal literal/grammar axiom. Its name is domain-generic and versioned; registry
/// membership, not absence from a vocabulary denylist, grants authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UniversalAxiomDescriptor {
    axiom_id: String,
    version: u32,
    implementation_module: String,
}

impl UniversalAxiomDescriptor {
    #[allow(dead_code)]
    pub(crate) fn new(
        axiom_id: impl Into<String>,
        version: u32,
        implementation_module: impl Into<String>,
    ) -> DerivationResult<Self> {
        let descriptor = Self {
            axiom_id: axiom_id.into(),
            version,
            implementation_module: implementation_module.into(),
        };
        validate_identifier("axiom id", &descriptor.axiom_id, 128)?;
        if descriptor.version == 0 {
            return Err(DerivationError::new("axiom version must be positive"));
        }
        if !descriptor.implementation_module.starts_with("crate::ir::") {
            return Err(DerivationError::new(
                "axiom implementation must be a core crate::ir module",
            ));
        }
        Ok(descriptor)
    }
}

/// Closed, deterministically hashed rule set consumed by the kernel.
#[derive(Debug, Clone)]
pub struct RuleRegistry {
    descriptors: BTreeMap<RuleId, RuleDescriptor>,
    verifiers: BTreeMap<RuleId, RuleVerifier>,
    axioms: BTreeMap<(String, u32), UniversalAxiomDescriptor>,
    ruleset_sha256: Sha256Digest,
}

#[derive(Serialize)]
struct RulesetHashInput<'a> {
    descriptor_schema_version: u32,
    descriptors: Vec<&'a RuleDescriptor>,
    axioms: Vec<&'a UniversalAxiomDescriptor>,
}

#[derive(Serialize)]
struct CumulativeRulesetHashInput<'a> {
    upstream_ruleset_sha256: &'a Sha256Digest,
    local_ruleset_sha256: &'a Sha256Digest,
}

fn cumulative_ruleset_sha256(
    upstream: &Sha256Digest,
    local: &Sha256Digest,
) -> DerivationResult<Sha256Digest> {
    Sha256Digest::of_serializable(&CumulativeRulesetHashInput {
        upstream_ruleset_sha256: upstream,
        local_ruleset_sha256: local,
    })
}

impl RuleRegistry {
    pub(crate) fn new(
        registrations: impl IntoIterator<Item = RuleRegistration>,
        axioms: impl IntoIterator<Item = UniversalAxiomDescriptor>,
    ) -> DerivationResult<Self> {
        let mut by_id = BTreeMap::new();
        let mut verifiers = BTreeMap::new();
        for registration in registrations {
            let descriptor = registration.descriptor;
            descriptor.validate()?;
            let id = descriptor.rule_id.clone();
            if by_id.insert(id.clone(), descriptor).is_some() {
                return Err(DerivationError::new(format!(
                    "duplicate rule id '{}'",
                    id.as_str()
                )));
            }
            verifiers.insert(id, registration.verifier);
        }
        if by_id.is_empty() {
            return Err(DerivationError::new("rule registry cannot be empty"));
        }

        let mut by_axiom = BTreeMap::new();
        for axiom in axioms {
            let key = (axiom.axiom_id.clone(), axiom.version);
            if by_axiom.insert(key.clone(), axiom).is_some() {
                return Err(DerivationError::new(format!(
                    "duplicate universal axiom '{}@{}'",
                    key.0, key.1
                )));
            }
        }

        let ruleset_sha256 = Sha256Digest::of_serializable(&RulesetHashInput {
            descriptor_schema_version: RULE_DESCRIPTOR_SCHEMA_VERSION,
            descriptors: by_id.values().collect(),
            axioms: by_axiom.values().collect(),
        })?;
        Ok(Self {
            descriptors: by_id,
            verifiers,
            axioms: by_axiom,
            ruleset_sha256,
        })
    }

    pub fn ruleset_sha256(&self) -> &Sha256Digest {
        &self.ruleset_sha256
    }

    pub fn descriptors(&self) -> impl ExactSizeIterator<Item = &RuleDescriptor> {
        self.descriptors.values()
    }

    fn descriptor(&self, id: &RuleId) -> Option<&RuleDescriptor> {
        self.descriptors.get(id)
    }

    fn verifier(&self, id: &RuleId) -> Option<RuleVerifier> {
        self.verifiers.get(id).copied()
    }

    fn contains_axiom(&self, id: &str, version: u32) -> bool {
        self.axioms.contains_key(&(id.to_string(), version))
    }
}

#[derive(Debug, Clone)]
struct GroundingAttestation {
    payload_sha256: Sha256Digest,
    exact_payload: Vec<u8>,
    validation_sha256: Option<Sha256Digest>,
    // Validation bytes are consumed by the later validated-prior rule migration.
    #[allow(dead_code)]
    exact_validation: Option<Vec<u8>>,
    direct_grounding: Vec<PremiseRef>,
    prior_scope: Option<ValidatedPriorScope>,
}

#[derive(Debug, Clone)]
struct CapturedEvidence {
    digest: Sha256Digest,
    exact_content: Vec<u8>,
}

#[derive(Debug, Clone)]
struct RegisteredDerivationAttestation {
    output_sha256: Sha256Digest,
    exact_output: Vec<u8>,
    inputs_sha256: Sha256Digest,
    inputs: Vec<PremiseRef>,
}

#[derive(Debug, Clone)]
struct EvidenceCatalog {
    scope: DocumentScope,
    source_spans: BTreeMap<String, CapturedEvidence>,
    table_cells: BTreeMap<(String, u32, u32), CapturedEvidence>,
    visual_regions: BTreeMap<String, CapturedEvidence>,
    registered_derivations: BTreeMap<String, RegisteredDerivationAttestation>,
    model_proposals: BTreeMap<String, GroundingAttestation>,
    validated_priors: BTreeMap<String, GroundingAttestation>,
}

impl EvidenceCatalog {
    fn new(scope: DocumentScope) -> Self {
        Self {
            scope,
            source_spans: BTreeMap::new(),
            table_cells: BTreeMap::new(),
            visual_regions: BTreeMap::new(),
            registered_derivations: BTreeMap::new(),
            model_proposals: BTreeMap::new(),
            validated_priors: BTreeMap::new(),
        }
    }

    fn validate_direct_grounding(&self, premise: &PremiseRef) -> DerivationResult<()> {
        match premise {
            PremiseRef::SourceSpan { .. }
            | PremiseRef::TableCell { .. }
            | PremiseRef::VisualRegion { .. } => self.validate_capture(premise),
            _ => Err(DerivationError::new(
                "proposal/prior grounding must resolve directly to captured current-document evidence",
            )),
        }
    }

    fn validate_capture(&self, premise: &PremiseRef) -> DerivationResult<()> {
        match premise {
            PremiseRef::SourceSpan {
                scope,
                span_id,
                content_sha256,
            } => {
                self.require_current_scope(scope)?;
                require_captured_digest(&self.source_spans, span_id, content_sha256, "source span")
            }
            PremiseRef::TableCell {
                scope,
                table_id,
                row,
                column,
                content_sha256,
            } => {
                self.require_current_scope(scope)?;
                require_captured_digest(
                    &self.table_cells,
                    &(table_id.clone(), *row, *column),
                    content_sha256,
                    "table cell",
                )
            }
            PremiseRef::VisualRegion {
                scope,
                region_id,
                content_sha256,
            } => {
                self.require_current_scope(scope)?;
                require_captured_digest(
                    &self.visual_regions,
                    region_id,
                    content_sha256,
                    "visual region",
                )
            }
            _ => Err(DerivationError::new("premise is not captured evidence")),
        }
    }

    fn premise_bytes<'a>(
        &'a self,
        premise: &PremiseRef,
        verified_prefix: &'a BTreeMap<ClaimAddress, Vec<u8>>,
        verified_local: &'a BTreeMap<ClaimAddress, Vec<u8>>,
    ) -> DerivationResult<Option<&'a [u8]>> {
        let bytes = match premise {
            PremiseRef::SourceSpan { span_id, .. } => self
                .source_spans
                .get(span_id)
                .map(|capture| capture.exact_content.as_slice()),
            PremiseRef::TableCell {
                table_id,
                row,
                column,
                ..
            } => self
                .table_cells
                .get(&(table_id.clone(), *row, *column))
                .map(|capture| capture.exact_content.as_slice()),
            PremiseRef::VisualRegion { region_id, .. } => self
                .visual_regions
                .get(region_id)
                .map(|capture| capture.exact_content.as_slice()),
            PremiseRef::UpstreamClaim { address, .. } => verified_local
                .get(address)
                .or_else(|| verified_prefix.get(address))
                .map(Vec::as_slice),
            PremiseRef::RegisteredDerivation { derivation_id, .. } => self
                .registered_derivations
                .get(derivation_id)
                .map(|attestation| attestation.exact_output.as_slice()),
            PremiseRef::GroundedModelProposal { proposal_id, .. } => self
                .model_proposals
                .get(proposal_id)
                .map(|attestation| attestation.exact_payload.as_slice()),
            PremiseRef::ValidatedPrior { prior_id, .. } => self
                .validated_priors
                .get(prior_id)
                .map(|attestation| attestation.exact_payload.as_slice()),
            PremiseRef::UniversalAxiom { .. } => return Ok(None),
        };
        bytes
            .map(Some)
            .ok_or_else(|| DerivationError::new("rule verifier premise bytes are unavailable"))
    }

    fn require_current_scope(&self, scope: &DocumentScope) -> DerivationResult<()> {
        if scope == &self.scope {
            Ok(())
        } else {
            Err(DerivationError::new(
                "premise belongs to a different document scope",
            ))
        }
    }
}

/// Sealed builder used by trusted capture code to bind the current evidence catalog before any
/// canonical promotion is possible.
pub(crate) struct PromotionKernelBuilder {
    seal: CapabilitySeal,
    registry: RuleRegistry,
    evidence: EvidenceCatalog,
    symbols: SymbolInterner,
    upstream_conclusions: BTreeMap<ClaimAddress, Vec<u8>>,
}

impl PromotionKernelBuilder {
    pub(crate) fn new(
        capture_digest: Sha256Digest,
        registry: RuleRegistry,
    ) -> DerivationResult<Self> {
        let scope = DocumentScope::from_capture_digest(capture_digest);
        if registry.descriptors.is_empty() {
            return Err(DerivationError::new("rule registry cannot be empty"));
        }
        Ok(Self {
            seal: CapabilitySeal,
            registry,
            evidence: EvidenceCatalog::new(scope.clone()),
            symbols: SymbolInterner::new(scope),
            upstream_conclusions: BTreeMap::new(),
        })
    }

    /// Start a downstream stage from an already verified cumulative prefix. The witness is
    /// intentionally non-deserializable: persisted bytes cannot seed upstream authority without
    /// first passing their owning stage verifier.
    pub(crate) fn with_verified_upstream(
        capture_digest: Sha256Digest,
        registry: RuleRegistry,
        upstream: &VerifiedProofLedger,
    ) -> DerivationResult<Self> {
        let mut builder = Self::new(capture_digest, registry)?;
        builder.upstream_conclusions = upstream.conclusions.clone();
        Ok(builder)
    }

    pub(crate) fn capture(&mut self) -> CaptureCapability<'_> {
        CaptureCapability {
            _seal: &self.seal,
            evidence: &mut self.evidence,
            symbols: &mut self.symbols,
        }
    }

    pub(crate) fn seal(self) -> PromotionKernel {
        PromotionKernel {
            seal: self.seal,
            registry: self.registry,
            evidence: self.evidence,
            claims: Vec::new(),
            conclusions: BTreeMap::new(),
            upstream_conclusions: self.upstream_conclusions,
        }
    }
}

/// Sealed capture token. It computes premise digests itself and rejects duplicate identities with
/// conflicting bytes, so callers cannot register a locator independently of its captured content.
pub(crate) struct CaptureCapability<'kernel> {
    _seal: &'kernel CapabilitySeal,
    evidence: &'kernel mut EvidenceCatalog,
    #[allow(dead_code)]
    symbols: &'kernel mut SymbolInterner,
}

impl CaptureCapability<'_> {
    #[allow(dead_code)]
    pub(crate) fn document_identity(
        &self,
        private_label: impl Into<Box<str>>,
    ) -> OpaqueDocumentIdentity {
        OpaqueDocumentIdentity {
            identity: DocumentIdentityAtom {
                scope: self.evidence.scope.clone(),
            },
            private_label: private_label.into(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn intern_symbol(&mut self, spelling: &str) -> DerivationResult<OpaqueSymbol> {
        self.symbols.intern(spelling)
    }

    pub(crate) fn source_span(
        &mut self,
        span_id: impl Into<String>,
        exact_content: &[u8],
    ) -> DerivationResult<PremiseRef> {
        let span_id = span_id.into();
        validate_nonempty_bounded("source span id", &span_id, 256)?;
        let digest = Sha256Digest::of_bytes(exact_content);
        insert_exact(
            &mut self.evidence.source_spans,
            span_id.clone(),
            CapturedEvidence {
                digest: digest.clone(),
                exact_content: exact_content.to_vec(),
            },
            "source span",
        )?;
        Ok(PremiseRef::SourceSpan {
            scope: self.evidence.scope.clone(),
            span_id,
            content_sha256: digest,
        })
    }

    pub(crate) fn table_cell(
        &mut self,
        table_id: impl Into<String>,
        row: u32,
        column: u32,
        exact_content: &[u8],
    ) -> DerivationResult<PremiseRef> {
        let table_id = table_id.into();
        validate_nonempty_bounded("table id", &table_id, 256)?;
        let digest = Sha256Digest::of_bytes(exact_content);
        insert_exact(
            &mut self.evidence.table_cells,
            (table_id.clone(), row, column),
            CapturedEvidence {
                digest: digest.clone(),
                exact_content: exact_content.to_vec(),
            },
            "table cell",
        )?;
        Ok(PremiseRef::TableCell {
            scope: self.evidence.scope.clone(),
            table_id,
            row,
            column,
            content_sha256: digest,
        })
    }

    pub(crate) fn visual_region(
        &mut self,
        region_id: impl Into<String>,
        exact_content: &[u8],
    ) -> DerivationResult<PremiseRef> {
        let region_id = region_id.into();
        validate_nonempty_bounded("visual region id", &region_id, 256)?;
        let digest = Sha256Digest::of_bytes(exact_content);
        insert_exact(
            &mut self.evidence.visual_regions,
            region_id.clone(),
            CapturedEvidence {
                digest: digest.clone(),
                exact_content: exact_content.to_vec(),
            },
            "visual region",
        )?;
        Ok(PremiseRef::VisualRegion {
            scope: self.evidence.scope.clone(),
            region_id,
            content_sha256: digest,
        })
    }

    /// Bind the exact output of a deterministic registered computation to its complete typed
    /// input topology. The output is recomputed by trusted stage code before this capability is
    /// called; it is never deserialized from the artifact under verification.
    pub(crate) fn registered_derivation(
        &mut self,
        derivation_id: impl Into<String>,
        exact_output: &[u8],
        inputs: Vec<PremiseRef>,
    ) -> DerivationResult<PremiseRef> {
        let derivation_id = derivation_id.into();
        validate_identifier("registered derivation id", &derivation_id, 256)?;
        if inputs.is_empty() {
            return Err(DerivationError::new(
                "registered derivation requires at least one typed input",
            ));
        }
        if inputs
            .iter()
            .any(|premise| matches!(premise, PremiseRef::UniversalAxiom { .. }))
        {
            return Err(DerivationError::new(
                "registered derivation inputs cannot be universal axioms",
            ));
        }
        for input in &inputs {
            if let PremiseRef::RegisteredDerivation {
                derivation_id,
                output_sha256,
                inputs_sha256,
                ..
            } = input
            {
                let dependency = self
                    .evidence
                    .registered_derivations
                    .get(derivation_id)
                    .ok_or_else(|| {
                        DerivationError::new(format!(
                            "registered derivation dependency '{derivation_id}' must already exist"
                        ))
                    })?;
                if &dependency.output_sha256 != output_sha256
                    || &dependency.inputs_sha256 != inputs_sha256
                {
                    return Err(DerivationError::new(format!(
                        "registered derivation dependency '{derivation_id}' is stale"
                    )));
                }
            }
        }
        let output_sha256 = Sha256Digest::of_bytes(exact_output);
        let inputs_sha256 = Sha256Digest::of_serializable(&inputs)?;
        let attestation = RegisteredDerivationAttestation {
            output_sha256: output_sha256.clone(),
            exact_output: exact_output.to_vec(),
            inputs_sha256: inputs_sha256.clone(),
            inputs,
        };
        if let Some(existing) = self.evidence.registered_derivations.get(&derivation_id) {
            if existing.output_sha256 == attestation.output_sha256
                && existing.exact_output == attestation.exact_output
                && existing.inputs_sha256 == attestation.inputs_sha256
                && existing.inputs == attestation.inputs
            {
                return Ok(PremiseRef::RegisteredDerivation {
                    scope: self.evidence.scope.clone(),
                    derivation_id,
                    output_sha256,
                    inputs_sha256,
                });
            }
            return Err(DerivationError::new(
                "duplicate registered derivation has conflicting output or inputs",
            ));
        }
        self.evidence
            .registered_derivations
            .insert(derivation_id.clone(), attestation);
        Ok(PremiseRef::RegisteredDerivation {
            scope: self.evidence.scope.clone(),
            derivation_id,
            output_sha256,
            inputs_sha256,
        })
    }

    pub(crate) fn grounded_model_proposal(
        &mut self,
        proposal_id: impl Into<String>,
        exact_payload: &[u8],
        direct_grounding: Vec<PremiseRef>,
    ) -> DerivationResult<PremiseRef> {
        let proposal_id = proposal_id.into();
        validate_nonempty_bounded("model proposal id", &proposal_id, 256)?;
        require_nonempty_direct_grounding(self.evidence, &direct_grounding)?;
        let digest = Sha256Digest::of_bytes(exact_payload);
        insert_attestation(
            &mut self.evidence.model_proposals,
            proposal_id.clone(),
            GroundingAttestation {
                payload_sha256: digest.clone(),
                exact_payload: exact_payload.to_vec(),
                validation_sha256: None,
                exact_validation: None,
                direct_grounding,
                prior_scope: None,
            },
            "grounded model proposal",
        )?;
        Ok(PremiseRef::GroundedModelProposal {
            scope: self.evidence.scope.clone(),
            proposal_id,
            proposal_sha256: digest,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn validated_prior(
        &mut self,
        prior_id: impl Into<String>,
        exact_payload: &[u8],
        exact_validation: &[u8],
        scope_contract: ValidatedPriorScope,
        direct_grounding: Vec<PremiseRef>,
    ) -> DerivationResult<PremiseRef> {
        let prior_id = prior_id.into();
        validate_nonempty_bounded("validated prior id", &prior_id, 256)?;
        require_nonempty_direct_grounding(self.evidence, &direct_grounding)?;
        let payload_sha256 = Sha256Digest::of_bytes(exact_payload);
        let validation_sha256 = Sha256Digest::of_bytes(exact_validation);
        insert_attestation(
            &mut self.evidence.validated_priors,
            prior_id.clone(),
            GroundingAttestation {
                payload_sha256: payload_sha256.clone(),
                exact_payload: exact_payload.to_vec(),
                validation_sha256: Some(validation_sha256.clone()),
                exact_validation: Some(exact_validation.to_vec()),
                direct_grounding,
                prior_scope: Some(scope_contract),
            },
            "validated prior",
        )?;
        Ok(PremiseRef::ValidatedPrior {
            prior_id,
            prior_sha256: payload_sha256,
            validation_sha256,
            scope_contract,
        })
    }
}

/// Confidence/provenance class attached to a checked claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofConfidence {
    Captured,
    Deterministic,
    GroundedModel,
    ValidatedPrior,
    Residual,
}

/// Proposal emitted by a sealed registered-grammar capability. Fields are private so an external
/// crate cannot synthesize a proposal around an arbitrary canonical record.
pub struct GroundedProposal<T> {
    address: ClaimAddress,
    rule_id: RuleId,
    premises: Vec<PremiseRef>,
    symbol_uses: Vec<SymbolUse>,
    confidence: ProofConfidence,
    conclusion: T,
}

impl GrammarCapability<'_> {
    pub(crate) fn propose<T>(
        &self,
        address: ClaimAddress,
        rule_id: RuleId,
        premises: Vec<PremiseRef>,
        symbol_uses: Vec<SymbolUse>,
        confidence: ProofConfidence,
        conclusion: T,
    ) -> GroundedProposal<T> {
        GroundedProposal {
            address,
            rule_id,
            premises,
            symbol_uses,
            confidence,
            conclusion,
        }
    }
}

/// One checked canonical conclusion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimProof {
    address: ClaimAddress,
    conclusion_sha256: Sha256Digest,
    rule_id: RuleId,
    premises: Vec<PremiseRef>,
    symbol_uses: Vec<SymbolUse>,
    confidence: ProofConfidence,
}

impl ClaimProof {
    pub fn address(&self) -> &ClaimAddress {
        &self.address
    }

    pub fn conclusion_sha256(&self) -> &Sha256Digest {
        &self.conclusion_sha256
    }

    pub fn rule_id(&self) -> &RuleId {
        &self.rule_id
    }

    pub fn premises(&self) -> &[PremiseRef] {
        &self.premises
    }

    pub fn symbol_uses(&self) -> &[SymbolUse] {
        &self.symbol_uses
    }
}

/// Artifact-level proof ledger. Deserialization alone grants no authority; the ledger must be
/// validated against the current closed registry, evidence catalog, and exact conclusion digests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofLedger {
    schema_version: u32,
    ruleset_sha256: Sha256Digest,
    claims: Vec<ClaimProof>,
}

impl ProofLedger {
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn ruleset_sha256(&self) -> &Sha256Digest {
        &self.ruleset_sha256
    }

    pub fn claims(&self) -> &[ClaimProof] {
        &self.claims
    }

    pub fn to_pretty_json(&self) -> DerivationResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            DerivationError::new(format!("cannot serialize proof ledger: {error}"))
        })
    }

    /// Validate an exact verified upstream prefix and recover the current stage's local ledger for
    /// executable verification. A cumulative hash or matching claim count alone is insufficient.
    pub(crate) fn local_suffix_after(
        &self,
        upstream: &VerifiedProofLedger,
        local_ruleset_sha256: &Sha256Digest,
    ) -> DerivationResult<ProofLedger> {
        if self.schema_version != PROOF_LEDGER_SCHEMA_VERSION {
            return Err(DerivationError::new(format!(
                "unsupported cumulative proof ledger schema {}; expected {PROOF_LEDGER_SCHEMA_VERSION}",
                self.schema_version
            )));
        }
        let expected_ruleset =
            cumulative_ruleset_sha256(&upstream.ledger.ruleset_sha256, local_ruleset_sha256)?;
        if self.ruleset_sha256 != expected_ruleset {
            return Err(DerivationError::new(
                "cumulative proof ledger ruleset hash is stale",
            ));
        }
        let prefix_len = upstream.ledger.claims.len();
        if self.claims.len() < prefix_len || self.claims[..prefix_len] != upstream.ledger.claims[..]
        {
            return Err(DerivationError::new(
                "cumulative proof ledger does not retain the exact verified upstream prefix",
            ));
        }
        Ok(ProofLedger {
            schema_version: PROOF_LEDGER_SCHEMA_VERSION,
            ruleset_sha256: local_ruleset_sha256.clone(),
            claims: self.claims[prefix_len..].to_vec(),
        })
    }
}

/// Canonical value paired with its checked proof. The value cannot be extracted outside core.
#[derive(Debug)]
pub struct Proved<T> {
    #[allow(dead_code)]
    value: T,
    proof: ClaimProof,
}

impl<T> Proved<T> {
    pub fn proof(&self) -> &ClaimProof {
        &self.proof
    }

    #[allow(dead_code)]
    pub(crate) fn value(&self) -> &T {
        &self.value
    }

    #[allow(dead_code)]
    pub(crate) fn into_value(self) -> T {
        self.value
    }
}

/// The only component that can turn a grounded proposal into a proved canonical value.
pub(crate) struct PromotionKernel {
    seal: CapabilitySeal,
    registry: RuleRegistry,
    evidence: EvidenceCatalog,
    claims: Vec<ClaimProof>,
    conclusions: BTreeMap<ClaimAddress, Vec<u8>>,
    upstream_conclusions: BTreeMap<ClaimAddress, Vec<u8>>,
}

impl PromotionKernel {
    pub(crate) fn grammar_capability(&self) -> GrammarCapability<'_> {
        GrammarCapability { _seal: &self.seal }
    }

    #[allow(dead_code)]
    pub(crate) fn persistence_capability(&self) -> PersistenceCapability<'_> {
        PersistenceCapability { _seal: &self.seal }
    }

    #[allow(dead_code)]
    pub(crate) fn presentation_capability(&self) -> PresentationCapability<'_> {
        PresentationCapability { _seal: &self.seal }
    }

    #[allow(dead_code)]
    pub(crate) fn lowering_capability(&self) -> LoweringCapability<'_> {
        LoweringCapability { _seal: &self.seal }
    }

    pub(crate) fn promote<T: Serialize>(
        &mut self,
        proposal: GroundedProposal<T>,
    ) -> DerivationResult<Proved<T>> {
        if self.conclusions.contains_key(&proposal.address)
            || self.upstream_conclusions.contains_key(&proposal.address)
        {
            return Err(DerivationError::new(format!(
                "claim address '{}:{}' already has a proof",
                proposal.address.surface, proposal.address.stable_record_key
            )));
        }
        let conclusion_json = serde_json::to_vec(&proposal.conclusion).map_err(|error| {
            DerivationError::new(format!("cannot serialize conclusion: {error}"))
        })?;
        let conclusion_sha256 = Sha256Digest::of_bytes(&conclusion_json);
        let proof = ClaimProof {
            address: proposal.address,
            conclusion_sha256: conclusion_sha256.clone(),
            rule_id: proposal.rule_id,
            premises: proposal.premises,
            symbol_uses: proposal.symbol_uses,
            confidence: proposal.confidence,
        };
        validate_claim(
            &proof,
            &conclusion_json,
            &self.registry,
            &self.evidence,
            &self.upstream_conclusions,
            &self.conclusions,
        )?;
        self.conclusions
            .insert(proof.address.clone(), conclusion_json);
        self.claims.push(proof.clone());
        Ok(Proved {
            value: proposal.conclusion,
            proof,
        })
    }

    pub(crate) fn verify_persisted(
        &self,
        ledger: ProofLedger,
        conclusions: &BTreeMap<ClaimAddress, Vec<u8>>,
    ) -> DerivationResult<VerifiedProofLedger> {
        verify_ledger(
            &ledger,
            &self.registry,
            &self.evidence,
            &self.upstream_conclusions,
            conclusions,
        )?;
        Ok(VerifiedProofLedger {
            ledger,
            conclusions: conclusions.clone(),
        })
    }

    pub(crate) fn finish(self) -> DerivationResult<VerifiedProofLedger> {
        let ledger = ProofLedger {
            schema_version: PROOF_LEDGER_SCHEMA_VERSION,
            ruleset_sha256: self.registry.ruleset_sha256.clone(),
            claims: self.claims,
        };
        verify_ledger(
            &ledger,
            &self.registry,
            &self.evidence,
            &self.upstream_conclusions,
            &self.conclusions,
        )?;
        Ok(VerifiedProofLedger {
            ledger,
            conclusions: self.conclusions,
        })
    }
}

/// Unforgeable in-memory witness that the ledger, current registry, captured premises, upstream
/// topology, and exact canonical conclusion digests were checked together.
#[derive(Debug)]
pub struct VerifiedProofLedger {
    ledger: ProofLedger,
    conclusions: BTreeMap<ClaimAddress, Vec<u8>>,
}

impl VerifiedProofLedger {
    pub fn ledger(&self) -> &ProofLedger {
        &self.ledger
    }

    pub fn permits_canonical_authority(&self) -> bool {
        true
    }

    pub(crate) fn into_ledger(self) -> ProofLedger {
        self.ledger
    }

    /// Join a verified upstream chain with a verified current-stage suffix. The persisted claim
    /// order remains an exact prefix relation, while the cumulative ruleset digest commits to the
    /// ordered stage composition.
    pub(crate) fn compose(upstream: &Self, local: Self) -> DerivationResult<VerifiedProofLedger> {
        let mut claims = upstream.ledger.claims.clone();
        claims.extend(local.ledger.claims.iter().cloned());
        let mut conclusions = upstream.conclusions.clone();
        for (address, bytes) in local.conclusions {
            if conclusions.insert(address, bytes).is_some() {
                return Err(DerivationError::new(
                    "cumulative proof chain has a duplicate claim address",
                ));
            }
        }
        if claims.len() != conclusions.len() {
            return Err(DerivationError::new(
                "cumulative proof chain does not cover every conclusion exactly once",
            ));
        }
        Ok(VerifiedProofLedger {
            ledger: ProofLedger {
                schema_version: PROOF_LEDGER_SCHEMA_VERSION,
                ruleset_sha256: cumulative_ruleset_sha256(
                    &upstream.ledger.ruleset_sha256,
                    &local.ledger.ruleset_sha256,
                )?,
                claims,
            },
            conclusions,
        })
    }
}

impl std::ops::Deref for VerifiedProofLedger {
    type Target = ProofLedger;

    fn deref(&self) -> &Self::Target {
        &self.ledger
    }
}

impl Serialize for VerifiedProofLedger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.ledger.serialize(serializer)
    }
}

/// Compatibility status of a persisted proof ledger before it may feed a canonical build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofCompatibility {
    CurrentEnvelopeRequiresVerification,
    LegacyProofless,
    UnsupportedLegacySchema { found: u32 },
    UnsupportedFutureSchema { found: u32 },
    StaleRuleset,
    InvalidCurrent { reason: String },
}

/// Fail-closed downstream disposition for a compatibility status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofCompatibilityDisposition {
    RequireKernelVerification,
    InspectAndRebuildOrResidualize,
    Reject,
}

impl ProofCompatibility {
    pub fn disposition(&self) -> ProofCompatibilityDisposition {
        match self {
            Self::CurrentEnvelopeRequiresVerification => {
                ProofCompatibilityDisposition::RequireKernelVerification
            }
            Self::LegacyProofless | Self::UnsupportedLegacySchema { .. } | Self::StaleRuleset => {
                ProofCompatibilityDisposition::InspectAndRebuildOrResidualize
            }
            Self::UnsupportedFutureSchema { .. } | Self::InvalidCurrent { .. } => {
                ProofCompatibilityDisposition::Reject
            }
        }
    }

    pub fn permits_canonical_authority(&self) -> bool {
        false
    }
}

/// Classify a serialized ledger without granting it authority. A missing ledger is explicitly
/// legacy/proofless, old or stale ledgers require rebuild/residualization, and future/malformed
/// ledgers reject.
pub fn assess_proof_compatibility(
    serialized_ledger: Option<&serde_json::Value>,
    expected_ruleset_sha256: &Sha256Digest,
) -> ProofCompatibility {
    let Some(value) = serialized_ledger else {
        return ProofCompatibility::LegacyProofless;
    };
    let Some(version_u64) = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
    else {
        return ProofCompatibility::InvalidCurrent {
            reason: "proof ledger lacks an unsigned schema_version".to_string(),
        };
    };
    let Ok(version) = u32::try_from(version_u64) else {
        return ProofCompatibility::UnsupportedFutureSchema { found: u32::MAX };
    };
    if version < PROOF_LEDGER_SCHEMA_VERSION {
        return ProofCompatibility::UnsupportedLegacySchema { found: version };
    }
    if version > PROOF_LEDGER_SCHEMA_VERSION {
        return ProofCompatibility::UnsupportedFutureSchema { found: version };
    }
    match serde_json::from_value::<ProofLedger>(value.clone()) {
        Ok(ledger) if ledger.ruleset_sha256 != *expected_ruleset_sha256 => {
            ProofCompatibility::StaleRuleset
        }
        Ok(_) => ProofCompatibility::CurrentEnvelopeRequiresVerification,
        Err(error) => ProofCompatibility::InvalidCurrent {
            reason: error.to_string(),
        },
    }
}

fn verify_ledger(
    ledger: &ProofLedger,
    registry: &RuleRegistry,
    evidence: &EvidenceCatalog,
    verified_prefix: &BTreeMap<ClaimAddress, Vec<u8>>,
    conclusions: &BTreeMap<ClaimAddress, Vec<u8>>,
) -> DerivationResult<()> {
    if ledger.schema_version != PROOF_LEDGER_SCHEMA_VERSION {
        return Err(DerivationError::new(format!(
            "unsupported proof ledger schema {}; expected {}",
            ledger.schema_version, PROOF_LEDGER_SCHEMA_VERSION
        )));
    }
    if ledger.ruleset_sha256 != registry.ruleset_sha256 {
        return Err(DerivationError::new("proof ledger ruleset hash is stale"));
    }
    validate_registered_derivation_catalog(evidence, registry, verified_prefix)?;
    let mut verified = BTreeMap::new();
    for proof in &ledger.claims {
        if verified.contains_key(&proof.address) {
            return Err(DerivationError::new(
                "proof ledger has a duplicate claim address",
            ));
        }
        let conclusion_json = conclusions.get(&proof.address).ok_or_else(|| {
            DerivationError::new(format!(
                "proof has no canonical conclusion at '{}:{}'",
                proof.address.surface, proof.address.stable_record_key
            ))
        })?;
        if Sha256Digest::of_bytes(conclusion_json) != proof.conclusion_sha256 {
            return Err(DerivationError::new(format!(
                "conclusion digest mismatch at '{}:{}'",
                proof.address.surface, proof.address.stable_record_key
            )));
        }
        validate_claim(
            proof,
            conclusion_json,
            registry,
            evidence,
            verified_prefix,
            &verified,
        )?;
        verified.insert(proof.address.clone(), conclusion_json.clone());
    }
    if verified.len() != conclusions.len() {
        return Err(DerivationError::new(
            "one or more canonical conclusions have no current proof",
        ));
    }
    Ok(())
}

/// Validate each registered deterministic replay once per ledger verification. A single replay is
/// deliberately shared by every field/record proof in a stage; recursively revalidating thousands
/// of identical upstream inputs for every conclusion would turn proof checking into a quadratic
/// corpus-size operation without adding authority.
fn validate_registered_derivation_catalog(
    evidence: &EvidenceCatalog,
    registry: &RuleRegistry,
    verified_prefix: &BTreeMap<ClaimAddress, Vec<u8>>,
) -> DerivationResult<()> {
    let no_local_claims = BTreeMap::new();
    for (derivation_id, attestation) in &evidence.registered_derivations {
        if Sha256Digest::of_bytes(&attestation.exact_output) != attestation.output_sha256
            || Sha256Digest::of_serializable(&attestation.inputs)? != attestation.inputs_sha256
        {
            return Err(DerivationError::new(format!(
                "registered derivation '{derivation_id}' attestation is internally inconsistent"
            )));
        }
        for input in &attestation.inputs {
            validate_premise(input, registry, evidence, verified_prefix, &no_local_claims)?;
        }
    }
    Ok(())
}

fn validate_claim(
    proof: &ClaimProof,
    conclusion_json: &[u8],
    registry: &RuleRegistry,
    evidence: &EvidenceCatalog,
    verified_prefix: &BTreeMap<ClaimAddress, Vec<u8>>,
    verified_local: &BTreeMap<ClaimAddress, Vec<u8>>,
) -> DerivationResult<()> {
    proof.address.validate()?;
    let descriptor = registry.descriptor(&proof.rule_id).ok_or_else(|| {
        DerivationError::new(format!(
            "unregistered rule '{}' cannot promote a claim",
            proof.rule_id.as_str()
        ))
    })?;
    if descriptor.conclusion_stage != proof.address.stage
        || descriptor.conclusion_surface != proof.address.surface
    {
        return Err(DerivationError::new(format!(
            "rule '{}' cannot conclude '{}:{}'",
            descriptor.rule_id.as_str(),
            proof.address.stage.as_str(),
            proof.address.surface
        )));
    }
    if proof.premises.is_empty() {
        return Err(DerivationError::new("promoted claim has no premises"));
    }
    for premise in &proof.premises {
        if !descriptor.premise_kinds.contains(&premise.kind()) {
            return Err(DerivationError::new(format!(
                "rule '{}' does not admit {:?} premises",
                descriptor.rule_id.as_str(),
                premise.kind()
            )));
        }
        validate_premise(premise, registry, evidence, verified_prefix, verified_local)?;
    }
    validate_confidence(proof)?;
    validate_symbol_uses(proof, descriptor, evidence)?;
    let verifier = registry.verifier(&proof.rule_id).ok_or_else(|| {
        DerivationError::new(format!(
            "registered rule '{}' has no executable verifier",
            proof.rule_id.as_str()
        ))
    })?;
    verifier(RuleVerificationContext {
        proof,
        conclusion_json,
        evidence,
        verified_prefix,
        verified_local,
    })
}

fn validate_premise(
    premise: &PremiseRef,
    registry: &RuleRegistry,
    evidence: &EvidenceCatalog,
    verified_prefix: &BTreeMap<ClaimAddress, Vec<u8>>,
    verified_local: &BTreeMap<ClaimAddress, Vec<u8>>,
) -> DerivationResult<()> {
    match premise {
        PremiseRef::SourceSpan { .. }
        | PremiseRef::TableCell { .. }
        | PremiseRef::VisualRegion { .. } => evidence.validate_capture(premise),
        PremiseRef::UpstreamClaim {
            address,
            conclusion_sha256,
        } => {
            let exact = verified_local
                .get(address)
                .or_else(|| verified_prefix.get(address))
                .ok_or_else(|| {
                    DerivationError::new(format!("upstream claim {:?} is absent", address))
                })?;
            if Sha256Digest::of_bytes(exact) == *conclusion_sha256 {
                Ok(())
            } else {
                Err(DerivationError::new(format!(
                    "upstream claim {:?} digest is stale",
                    address
                )))
            }
        }
        PremiseRef::RegisteredDerivation {
            scope,
            derivation_id,
            output_sha256,
            inputs_sha256,
        } => {
            evidence.require_current_scope(scope)?;
            let attestation = evidence
                .registered_derivations
                .get(derivation_id)
                .ok_or_else(|| {
                    DerivationError::new(format!(
                        "registered derivation '{derivation_id}' is not available from the current binary replay"
                    ))
                })?;
            if &attestation.output_sha256 != output_sha256
                || &attestation.inputs_sha256 != inputs_sha256
            {
                return Err(DerivationError::new(format!(
                    "registered derivation '{derivation_id}' output or input topology is stale"
                )));
            }
            Ok(())
        }
        PremiseRef::GroundedModelProposal {
            scope,
            proposal_id,
            proposal_sha256,
        } => {
            evidence.require_current_scope(scope)?;
            let attestation = evidence.model_proposals.get(proposal_id).ok_or_else(|| {
                DerivationError::new(format!(
                    "grounded model proposal '{proposal_id}' is not registered"
                ))
            })?;
            if &attestation.payload_sha256 != proposal_sha256 {
                return Err(DerivationError::new(format!(
                    "grounded model proposal '{proposal_id}' digest is stale"
                )));
            }
            for grounding in &attestation.direct_grounding {
                evidence.validate_direct_grounding(grounding)?;
            }
            Ok(())
        }
        PremiseRef::ValidatedPrior {
            prior_id,
            prior_sha256,
            validation_sha256,
            scope_contract,
        } => {
            let attestation = evidence.validated_priors.get(prior_id).ok_or_else(|| {
                DerivationError::new(format!("validated prior '{prior_id}' is not registered"))
            })?;
            if &attestation.payload_sha256 != prior_sha256
                || attestation.validation_sha256.as_ref() != Some(validation_sha256)
                || attestation.prior_scope != Some(*scope_contract)
            {
                return Err(DerivationError::new(format!(
                    "validated prior '{prior_id}' attestation is stale or scope-mismatched"
                )));
            }
            for grounding in &attestation.direct_grounding {
                evidence.validate_direct_grounding(grounding)?;
            }
            Ok(())
        }
        PremiseRef::UniversalAxiom { axiom_id, version } => {
            if registry.contains_axiom(axiom_id, *version) {
                Ok(())
            } else {
                Err(DerivationError::new(format!(
                    "unregistered universal axiom '{axiom_id}@{version}'"
                )))
            }
        }
    }
}

fn validate_confidence(proof: &ClaimProof) -> DerivationResult<()> {
    let has_model = proof
        .premises
        .iter()
        .any(|premise| premise.kind() == PremiseKind::GroundedModelProposal);
    let has_prior = proof
        .premises
        .iter()
        .any(|premise| premise.kind() == PremiseKind::ValidatedPrior);
    match proof.confidence {
        ProofConfidence::GroundedModel if !has_model => Err(DerivationError::new(
            "grounded-model confidence requires a grounded model premise",
        )),
        ProofConfidence::ValidatedPrior if !has_prior => Err(DerivationError::new(
            "validated-prior confidence requires a validated prior premise",
        )),
        _ => Ok(()),
    }
}

fn validate_symbol_uses(
    proof: &ClaimProof,
    descriptor: &RuleDescriptor,
    evidence: &EvidenceCatalog,
) -> DerivationResult<()> {
    for symbol_use in &proof.symbol_uses {
        evidence.require_current_scope(symbol_use.identity.scope())?;
        let allowed = match descriptor.symbol_capability {
            SymbolCapabilityClass::SymbolBlind => false,
            SymbolCapabilityClass::ExactIdentityOnly => matches!(
                symbol_use.use_kind,
                SymbolUseKind::Copied | SymbolUseKind::ExactIdentityCompared
            ),
            SymbolCapabilityClass::GrammarIntroduces => {
                !matches!(symbol_use.use_kind, SymbolUseKind::TargetEncoded)
            }
            SymbolCapabilityClass::LosslessCarry
            | SymbolCapabilityClass::MergeOrConflict
            | SymbolCapabilityClass::Residual => matches!(
                symbol_use.use_kind,
                SymbolUseKind::Copied | SymbolUseKind::ExactIdentityCompared
            ),
            SymbolCapabilityClass::TargetLowering => matches!(
                symbol_use.use_kind,
                SymbolUseKind::Copied | SymbolUseKind::TargetEncoded
            ),
        };
        if !allowed {
            return Err(DerivationError::new(format!(
                "rule '{}' does not permit {:?} symbol use",
                descriptor.rule_id.as_str(),
                symbol_use.use_kind
            )));
        }
        match symbol_use.use_kind {
            SymbolUseKind::IntroducedFromPremise => {
                let Some(index) = symbol_use.introduced_from_premise else {
                    return Err(DerivationError::new(
                        "introduced symbol lacks an origin premise",
                    ));
                };
                let Some(origin) = proof.premises.get(index as usize) else {
                    return Err(DerivationError::new(
                        "introduced symbol origin premise is out of bounds",
                    ));
                };
                if !matches!(
                    origin.kind(),
                    PremiseKind::SourceSpan
                        | PremiseKind::TableCell
                        | PremiseKind::VisualRegion
                        | PremiseKind::GroundedModelProposal
                ) {
                    return Err(DerivationError::new(
                        "introduced symbol origin must be current-document grounded",
                    ));
                }
            }
            _ if symbol_use.introduced_from_premise.is_some() => {
                return Err(DerivationError::new(
                    "only introduced symbols may name an origin premise",
                ));
            }
            _ => {}
        }
    }
    if descriptor.symbol_capability == SymbolCapabilityClass::SymbolBlind
        && !proof.symbol_uses.is_empty()
    {
        return Err(DerivationError::new(format!(
            "symbol-blind rule '{}' recorded symbol use",
            descriptor.rule_id.as_str()
        )));
    }
    Ok(())
}

fn validate_alpha_pair(
    capability: SymbolCapabilityClass,
    obligation: AlphaObligation,
) -> DerivationResult<()> {
    let valid = matches!(
        (capability, obligation),
        (
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput
        ) | (
            SymbolCapabilityClass::ExactIdentityOnly,
            AlphaObligation::IdentityGraphInvariant
        ) | (
            SymbolCapabilityClass::GrammarIntroduces,
            AlphaObligation::IntroducedSymbolsPreserveOrigins
        ) | (
            SymbolCapabilityClass::LosslessCarry,
            AlphaObligation::LosslessTopologyInvariant
        ) | (
            SymbolCapabilityClass::MergeOrConflict,
            AlphaObligation::MergeConflictTopologyInvariant
        ) | (
            SymbolCapabilityClass::Residual,
            AlphaObligation::ResidualTopologyInvariant
        ) | (
            SymbolCapabilityClass::TargetLowering,
            AlphaObligation::TargetSafeRenaming
        )
    );
    if valid {
        Ok(())
    } else {
        Err(DerivationError::new(
            "rule symbol capability and alpha obligation are incompatible",
        ))
    }
}

fn require_nonempty_direct_grounding(
    evidence: &EvidenceCatalog,
    grounding: &[PremiseRef],
) -> DerivationResult<()> {
    if grounding.is_empty() {
        return Err(DerivationError::new(
            "proposal/prior attestation requires direct grounding",
        ));
    }
    for premise in grounding {
        evidence.validate_direct_grounding(premise)?;
    }
    Ok(())
}

fn require_captured_digest<K: Ord + fmt::Debug>(
    catalog: &BTreeMap<K, CapturedEvidence>,
    key: &K,
    digest: &Sha256Digest,
    label: &str,
) -> DerivationResult<()> {
    match catalog.get(key) {
        Some(expected) if &expected.digest == digest => Ok(()),
        Some(_) => Err(DerivationError::new(format!(
            "{label} {key:?} digest is stale"
        ))),
        None => Err(DerivationError::new(format!(
            "{label} {key:?} is not registered in the current chain"
        ))),
    }
}

fn insert_exact<K: Ord + Clone + fmt::Debug>(
    catalog: &mut BTreeMap<K, CapturedEvidence>,
    key: K,
    captured: CapturedEvidence,
    label: &str,
) -> DerivationResult<()> {
    if let Some(existing) = catalog.get(&key) {
        if existing.digest == captured.digest && existing.exact_content == captured.exact_content {
            return Ok(());
        }
        return Err(DerivationError::new(format!(
            "duplicate {label} {key:?} has conflicting content"
        )));
    }
    catalog.insert(key, captured);
    Ok(())
}

fn insert_attestation(
    catalog: &mut BTreeMap<String, GroundingAttestation>,
    key: String,
    attestation: GroundingAttestation,
    label: &str,
) -> DerivationResult<()> {
    if catalog.contains_key(&key) {
        return Err(DerivationError::new(format!("duplicate {label} '{key}'")));
    }
    catalog.insert(key, attestation);
    Ok(())
}

fn validate_identifier(label: &str, value: &str, maximum: usize) -> DerivationResult<()> {
    validate_nonempty_bounded(label, value, maximum)?;
    if value.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'.' | b'_' | b'-' | b'[' | b']')
    }) {
        Ok(())
    } else {
        Err(DerivationError::new(format!(
            "{label} must be a lowercase structural identifier"
        )))
    }
}

fn validate_nonempty_bounded(label: &str, value: &str, maximum: usize) -> DerivationResult<()> {
    if value.is_empty() || value.len() > maximum || value.contains(['\n', '\r', '\0']) {
        Err(DerivationError::new(format!(
            "{label} must be non-empty, bounded, and single-line"
        )))
    } else {
        Ok(())
    }
}

/// Typed failure returned by the trusted kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationError {
    message: String,
}

impl DerivationError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for DerivationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DerivationError {}

pub type DerivationResult<T> = Result<T, DerivationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_semantic_digests_are_compiler_derived_stage_closures() {
        let stages = [
            (
                IrStage::SourceIr,
                SOURCE_PRODUCTION_SEMANTIC_SHA256,
                SOURCE_PRODUCTION_SEMANTIC_ITEMS,
                [
                    "const:SOURCE_RULE_FIELDS",
                    "fn:source_rule_registry",
                    "fn:verify_source_rule_relation",
                ],
            ),
            (
                IrStage::EvidenceIr,
                EVIDENCE_PRODUCTION_SEMANTIC_SHA256,
                EVIDENCE_PRODUCTION_SEMANTIC_ITEMS,
                [
                    "const:EVIDENCE_RULE_FIELDS",
                    "fn:evidence_rule_registry",
                    "fn:verify_evidence_rule_relation",
                ],
            ),
            (
                IrStage::SemanticIr,
                SEMANTIC_PRODUCTION_SEMANTIC_SHA256,
                SEMANTIC_PRODUCTION_SEMANTIC_ITEMS,
                [
                    "const:SEMANTIC_RULE_FIELDS",
                    "fn:semantic_rule_registry",
                    "fn:verify_semantic_rule_relation",
                ],
            ),
            (
                IrStage::IntentIr,
                INTENT_PRODUCTION_SEMANTIC_SHA256,
                INTENT_PRODUCTION_SEMANTIC_ITEMS,
                [
                    "const:INTENT_RULE_FIELDS",
                    "fn:intent_rule_registry",
                    "fn:verify_intent_rule_relation",
                ],
            ),
            (
                IrStage::IsfAdapter,
                ADAPTER_PRODUCTION_SEMANTIC_SHA256,
                ADAPTER_PRODUCTION_SEMANTIC_ITEMS,
                [
                    "const:ADAPTER_RULE_FIELDS",
                    "fn:adapter_rule_registry",
                    "fn:verify_adapter_rule_relation",
                ],
            ),
        ];
        let mut unique = BTreeSet::new();
        for (stage, expected, items, required_items) in stages {
            assert_eq!(
                production_semantic_implementation_digest(stage)
                    .unwrap()
                    .as_str(),
                expected
            );
            assert!(!items.is_empty(), "{stage:?} semantic closure is empty");
            assert!(
                items.iter().all(|item| {
                    !item.contains("test")
                        && !item.contains("fixture")
                        && !item.contains("conformance")
                }),
                "{stage:?} semantic closure contains non-production authority: {items:?}"
            );
            for required in required_items {
                assert!(
                    items.contains(&required),
                    "{stage:?} semantic closure omits {required}"
                );
            }
            assert!(
                items.iter().any(|item| item.starts_with("use:")),
                "{stage:?} semantic closure omits referenced import bindings"
            );
            assert!(unique.insert(expected), "duplicate stage semantic digest");
        }
        for required in [
            "fn:classify_source_captures",
            "fn:apply_source_grounded_proposal",
            "fn:source_validation_report_from_fields",
        ] {
            assert!(
                SOURCE_PRODUCTION_SEMANTIC_ITEMS.contains(&required),
                "SourceIR semantic closure omits {required}"
            );
        }
        assert_eq!(PRODUCTION_SEMANTIC_DIGEST_SCHEMA_VERSION, 1);
        assert_eq!(PRODUCTION_KERNEL_SEMANTIC_SHA256.len(), 64);
        assert_eq!(PRODUCTION_SEMANTIC_INPUTS.len(), 6);
        assert!(PRODUCTION_SEMANTIC_INPUTS.iter().all(|path| {
            path.ends_with(".rs")
                && !path.contains("conformance")
                && !path.contains("test")
                && !path.contains("fixture")
                && !path.contains("docs/")
        }));
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct SyntheticConclusion {
        subject_ordinal: u32,
        property: String,
    }

    fn verify_synthetic_relation(context: RuleVerificationContext<'_>) -> DerivationResult<()> {
        if context.conclusion_json().is_empty() {
            return Err(DerivationError::new("synthetic conclusion is empty"));
        }
        for index in 0..context.proof().premises().len() {
            let _ = context.premise_bytes(index)?;
        }
        Ok(())
    }

    fn verify_exact_copy(context: RuleVerificationContext<'_>) -> DerivationResult<()> {
        let premise = context
            .premise_bytes(0)?
            .ok_or_else(|| DerivationError::new("exact-copy rule requires captured bytes"))?;
        if premise == context.conclusion_json() {
            Ok(())
        } else {
            Err(DerivationError::new(
                "executable exact-copy relation rejected the conclusion",
            ))
        }
    }

    fn registration(descriptor: RuleDescriptor) -> RuleRegistration {
        RuleRegistration::new(descriptor, verify_synthetic_relation)
    }

    fn rule(
        id: &str,
        premises: impl IntoIterator<Item = PremiseKind>,
        capability: SymbolCapabilityClass,
        obligation: AlphaObligation,
    ) -> RuleDescriptor {
        let compatibility = if capability == SymbolCapabilityClass::LosslessCarry {
            RuleCompatibility::LosslessCarry
        } else {
            RuleCompatibility::CurrentOnly
        };
        RuleDescriptor::new(
            RuleId::try_from(id.to_string()).unwrap(),
            1,
            "crate::ir::derivation::tests",
            Sha256Digest::of_bytes(b"derivation test verifier v1"),
            premises,
            IrStage::EvidenceIr,
            "synthetic_claims",
            capability,
            obligation,
            compatibility,
        )
        .unwrap()
    }

    fn registry(descriptors: Vec<RuleDescriptor>) -> RuleRegistry {
        RuleRegistry::new(
            descriptors.into_iter().map(registration),
            [
                UniversalAxiomDescriptor::new(
                    "binary.logic.level",
                    1,
                    "crate::ir::normative_vocab",
                )
                .unwrap(),
            ],
        )
        .unwrap()
    }

    fn address(key: &str) -> ClaimAddress {
        ClaimAddress::new(IrStage::EvidenceIr, "synthetic_claims", key, None).unwrap()
    }

    fn source_kernel() -> (PromotionKernel, PremiseRef, OpaqueSymbol) {
        let registry = registry(vec![rule(
            "synthetic.source_copy",
            [PremiseKind::SourceSpan],
            SymbolCapabilityClass::GrammarIntroduces,
            AlphaObligation::IntroducedSymbolsPreserveOrigins,
        )]);
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry).unwrap();
        let (premise, symbol) = {
            let mut capture = builder.capture();
            let premise = capture
                .source_span("span-0", b"bound atom is asserted")
                .unwrap();
            let symbol = capture.intern_symbol("BOUND_ATOM").unwrap();
            (premise, symbol)
        };
        (builder.seal(), premise, symbol)
    }

    #[test]
    fn opaque_symbol_debug_redacts_spelling_and_identity_is_alpha_invariant() {
        let registry = registry(vec![rule(
            "synthetic.identity",
            [PremiseKind::SourceSpan],
            SymbolCapabilityClass::ExactIdentityOnly,
            AlphaObligation::IdentityGraphInvariant,
        )]);
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"scope"), registry).unwrap();
        let mut capture = builder.capture();
        let first = capture.intern_symbol("FIRST_SPELLING").unwrap();
        let repeated = capture.intern_symbol("FIRST_SPELLING").unwrap();
        let second = capture.intern_symbol("ADVERSARIAL_RENAME").unwrap();
        assert!(first.same_identity(&repeated));
        assert!(!first.same_identity(&second));
        assert_eq!(first.identity().source_ordinal(), 0);
        assert_eq!(second.identity().source_ordinal(), 1);
        let debug = format!("{first:?}");
        assert!(debug.contains("<opaque>"));
        assert!(!debug.contains("FIRST_SPELLING"));

        let kernel = builder.seal();
        let persistence = kernel.persistence_capability();
        let mut renamed = persistence.persist_symbol(&first);
        renamed.spelling = "FRESH_ALPHA_NAME".to_string();
        let renamed = persistence.restore_symbol(renamed);
        assert!(first.same_identity(&renamed));
        assert_eq!(
            kernel.presentation_capability().symbol(&renamed),
            "FRESH_ALPHA_NAME"
        );
    }

    #[test]
    fn sealed_persistence_round_trip_is_transparent_but_semantics_remain_spelling_free() {
        let (kernel, _, symbol) = source_kernel();
        let persistence = kernel.persistence_capability();
        let persisted = persistence.persist_symbol(&symbol);
        let restored = persistence.restore_symbol(persisted);
        assert!(symbol.same_identity(&restored));
        let presentation = kernel.presentation_capability();
        assert_eq!(presentation.symbol(&restored), "BOUND_ATOM");
        let encoded = kernel
            .lowering_capability()
            .encode_symbol(&restored, |value| value.to_ascii_lowercase());
        assert_eq!(encoded, "bound_atom");
    }

    #[test]
    fn opaque_document_identity_is_exact_only_and_presentation_is_capability_gated() {
        let registry = registry(vec![rule(
            "synthetic.identity",
            [PremiseKind::SourceSpan],
            SymbolCapabilityClass::ExactIdentityOnly,
            AlphaObligation::IdentityGraphInvariant,
        )]);
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"scope"), registry).unwrap();
        let document = {
            let capture = builder.capture();
            capture.document_identity("private input label")
        };
        let kernel = builder.seal();
        assert!(document.same_identity(&document.clone()));
        assert_eq!(
            kernel.presentation_capability().document(&document),
            "private input label"
        );
        let persisted = kernel.persistence_capability().persist_document(&document);
        let restored = kernel.persistence_capability().restore_document(persisted);
        assert!(document.same_identity(&restored));
        assert!(!format!("{document:?}").contains("private input label"));
    }

    #[test]
    fn registry_hash_is_deterministic_and_order_independent() {
        let first = rule(
            "synthetic.first",
            [PremiseKind::SourceSpan],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let second = rule(
            "synthetic.second",
            [PremiseKind::TableCell],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let left = registry(vec![first.clone(), second.clone()]);
        let right = registry(vec![second, first]);
        assert_eq!(left.ruleset_sha256(), right.ruleset_sha256());
    }

    #[test]
    fn executable_verifier_rejects_hash_consistent_but_false_conclusion() {
        let descriptor = rule(
            "synthetic.exact_copy",
            [PremiseKind::SourceSpan],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let registry =
            RuleRegistry::new([RuleRegistration::new(descriptor, verify_exact_copy)], []).unwrap();
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry).unwrap();
        let premise = builder
            .capture()
            .source_span("exact", &serde_json::to_vec("captured").unwrap())
            .unwrap();
        let mut kernel = builder.seal();
        let proposal = kernel.grammar_capability().propose(
            address("false"),
            RuleId::try_from("synthetic.exact_copy".to_string()).unwrap(),
            vec![premise],
            Vec::new(),
            ProofConfidence::Deterministic,
            "forged",
        );
        let error = kernel.promote(proposal).unwrap_err();
        assert!(error.to_string().contains("exact-copy relation rejected"));
    }

    #[test]
    fn implementation_digest_changes_ruleset_and_stales_a_persisted_ledger() {
        let descriptor = |implementation: &[u8]| {
            RuleDescriptor::new(
                RuleId::try_from("synthetic.implementation_bound".to_string()).unwrap(),
                1,
                "crate::ir::derivation::tests",
                Sha256Digest::of_bytes(implementation),
                [PremiseKind::SourceSpan],
                IrStage::EvidenceIr,
                "synthetic_claims",
                SymbolCapabilityClass::SymbolBlind,
                AlphaObligation::ByteIdenticalNonSymbolOutput,
                RuleCompatibility::CurrentOnly,
            )
            .unwrap()
        };
        let original = RuleRegistry::new(
            [RuleRegistration::new(
                descriptor(b"implementation-v1"),
                verify_exact_copy,
            )],
            [],
        )
        .unwrap();
        let changed = RuleRegistry::new(
            [RuleRegistration::new(
                descriptor(b"implementation-v2"),
                verify_exact_copy,
            )],
            [],
        )
        .unwrap();
        assert_ne!(original.ruleset_sha256(), changed.ruleset_sha256());

        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), original).unwrap();
        let exact = serde_json::to_vec("captured").unwrap();
        let premise = builder.capture().source_span("exact", &exact).unwrap();
        let mut kernel = builder.seal();
        let proposal = kernel.grammar_capability().propose(
            address("bound"),
            RuleId::try_from("synthetic.implementation_bound".to_string()).unwrap(),
            vec![premise],
            Vec::new(),
            ProofConfidence::Deterministic,
            "captured",
        );
        kernel.promote(proposal).unwrap();
        let ledger = kernel.finish().unwrap().into_ledger();
        let serialized = serde_json::to_value(ledger).unwrap();
        assert_eq!(
            assess_proof_compatibility(Some(&serialized), changed.ruleset_sha256()),
            ProofCompatibility::StaleRuleset
        );
    }

    #[test]
    fn registry_rejects_duplicate_rule_and_incompatible_alpha_contract() {
        let duplicate = rule(
            "synthetic.duplicate",
            [PremiseKind::SourceSpan],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let error = RuleRegistry::new(
            vec![registration(duplicate.clone()), registration(duplicate)],
            [],
        )
        .unwrap_err();
        assert!(error.to_string().contains("duplicate rule id"));

        let error = RuleDescriptor::new(
            RuleId::try_from("synthetic.bad_alpha".to_string()).unwrap(),
            1,
            "crate::ir::derivation::tests",
            Sha256Digest::of_bytes(b"derivation test verifier v1"),
            [PremiseKind::SourceSpan],
            IrStage::EvidenceIr,
            "synthetic_claims",
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::TargetSafeRenaming,
            RuleCompatibility::CurrentOnly,
        )
        .unwrap_err();
        assert!(error.to_string().contains("incompatible"));

        let descriptor = RuleDescriptor::new(
            RuleId::try_from("synthetic.missing_origin".to_string()).unwrap(),
            1,
            "crate::ir::derivation::tests",
            Sha256Digest::of_bytes(b"derivation test verifier v1"),
            [PremiseKind::UniversalAxiom],
            IrStage::EvidenceIr,
            "synthetic_claims",
            SymbolCapabilityClass::GrammarIntroduces,
            AlphaObligation::IntroducedSymbolsPreserveOrigins,
            RuleCompatibility::CurrentOnly,
        )
        .unwrap();
        let error = descriptor
            .validate_structural_alpha_obligation()
            .unwrap_err();
        assert!(error.to_string().contains("structural alpha obligation"));

        let descriptor = RuleDescriptor::new(
            RuleId::try_from("synthetic.nonlossless_carry".to_string()).unwrap(),
            1,
            "crate::ir::derivation::tests",
            Sha256Digest::of_bytes(b"derivation test verifier v1"),
            [
                PremiseKind::RegisteredDerivation,
                PremiseKind::UpstreamClaim,
            ],
            IrStage::EvidenceIr,
            "synthetic_claims",
            SymbolCapabilityClass::LosslessCarry,
            AlphaObligation::LosslessTopologyInvariant,
            RuleCompatibility::CurrentOnly,
        )
        .unwrap();
        let error = descriptor
            .validate_structural_alpha_obligation()
            .unwrap_err();
        assert!(error.to_string().contains("structural alpha obligation"));
    }

    #[test]
    fn source_grounded_promotion_emits_one_current_proof() {
        let (mut kernel, premise, symbol) = source_kernel();
        let proposal = kernel.grammar_capability().propose(
            address("claim-0"),
            RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
            vec![premise],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            SyntheticConclusion {
                subject_ordinal: symbol.identity().source_ordinal(),
                property: "asserted".to_string(),
            },
        );
        let proved = kernel.promote(proposal).unwrap();
        assert_eq!(proved.value().property, "asserted");
        assert_eq!(proved.proof().premises().len(), 1);
        let ledger = kernel.finish().unwrap();
        assert_eq!(ledger.schema_version(), PROOF_LEDGER_SCHEMA_VERSION);
        assert_eq!(ledger.claims().len(), 1);
    }

    #[test]
    fn promotion_rejects_unregistered_rule_wrong_surface_and_duplicate_address() {
        let (mut kernel, premise, symbol) = source_kernel();
        let unknown = kernel.grammar_capability().propose(
            address("unknown"),
            RuleId::try_from("synthetic.unknown".to_string()).unwrap(),
            vec![premise.clone()],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(unknown)
                .unwrap_err()
                .to_string()
                .contains("unregistered")
        );

        let wrong = kernel.grammar_capability().propose(
            ClaimAddress::new(IrStage::EvidenceIr, "other_surface", "wrong", None).unwrap(),
            RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
            vec![premise.clone()],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(wrong)
                .unwrap_err()
                .to_string()
                .contains("cannot conclude")
        );

        for attempt in 0..2 {
            let proposal = kernel.grammar_capability().propose(
                address("duplicate"),
                RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
                vec![premise.clone()],
                vec![SymbolUse::introduced(&symbol, 0)],
                ProofConfidence::Deterministic,
                attempt,
            );
            if attempt == 0 {
                kernel.promote(proposal).unwrap();
            } else {
                assert!(
                    kernel
                        .promote(proposal)
                        .unwrap_err()
                        .to_string()
                        .contains("already has a proof")
                );
            }
        }
    }

    #[test]
    fn current_document_scope_and_exact_capture_digest_fail_closed() {
        let (mut kernel, premise, symbol) = source_kernel();
        let mut stale = premise.clone();
        if let PremiseRef::SourceSpan { content_sha256, .. } = &mut stale {
            *content_sha256 = Sha256Digest::of_bytes(b"tampered");
        }
        let proposal = kernel.grammar_capability().propose(
            address("stale"),
            RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
            vec![stale],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(proposal)
                .unwrap_err()
                .to_string()
                .contains("stale")
        );

        let mut foreign = premise;
        if let PremiseRef::SourceSpan { scope, .. } = &mut foreign {
            *scope = DocumentScope::from_capture_digest(Sha256Digest::of_bytes(b"foreign"));
        }
        let proposal = kernel.grammar_capability().propose(
            address("foreign"),
            RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
            vec![foreign],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(proposal)
                .unwrap_err()
                .to_string()
                .contains("different document scope")
        );
    }

    #[test]
    fn model_and_prior_premises_require_independent_grounding_and_matching_attestation() {
        let descriptors = vec![
            rule(
                "synthetic.model",
                [PremiseKind::GroundedModelProposal],
                SymbolCapabilityClass::SymbolBlind,
                AlphaObligation::ByteIdenticalNonSymbolOutput,
            ),
            rule(
                "synthetic.prior",
                [PremiseKind::ValidatedPrior],
                SymbolCapabilityClass::SymbolBlind,
                AlphaObligation::ByteIdenticalNonSymbolOutput,
            ),
        ];
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry(descriptors))
                .unwrap();
        let (model, prior) = {
            let mut capture = builder.capture();
            let span = capture.source_span("span", b"grounding").unwrap();
            let model = capture
                .grounded_model_proposal("model-0", b"proposal", vec![span.clone()])
                .unwrap();
            let prior = capture
                .validated_prior(
                    "prior-0",
                    b"prior",
                    b"validation",
                    ValidatedPriorScope::GlobalIdentityIndependent,
                    vec![span],
                )
                .unwrap();
            (model, prior)
        };
        let mut kernel = builder.seal();
        let model_proposal = kernel.grammar_capability().propose(
            address("model"),
            RuleId::try_from("synthetic.model".to_string()).unwrap(),
            vec![model],
            vec![],
            ProofConfidence::GroundedModel,
            1_u32,
        );
        kernel.promote(model_proposal).unwrap();
        let prior_proposal = kernel.grammar_capability().propose(
            address("prior"),
            RuleId::try_from("synthetic.prior".to_string()).unwrap(),
            vec![prior],
            vec![],
            ProofConfidence::ValidatedPrior,
            2_u32,
        );
        kernel.promote(prior_proposal).unwrap();
        assert_eq!(kernel.finish().unwrap().claims().len(), 2);
    }

    #[test]
    fn model_and_prior_registration_refuse_empty_or_indirect_grounding() {
        let registry = registry(vec![rule(
            "synthetic.model",
            [PremiseKind::GroundedModelProposal],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        )]);
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry).unwrap();
        let mut capture = builder.capture();
        assert!(
            capture
                .grounded_model_proposal("empty", b"proposal", vec![])
                .unwrap_err()
                .to_string()
                .contains("direct grounding")
        );
        let axiom = PremiseRef::UniversalAxiom {
            axiom_id: "binary.logic.level".to_string(),
            version: 1,
        };
        assert!(
            capture
                .grounded_model_proposal("indirect", b"proposal", vec![axiom])
                .unwrap_err()
                .to_string()
                .contains("directly")
        );
    }

    #[test]
    fn table_visual_and_universal_axiom_premises_are_typed_and_registry_checked() {
        let descriptor = rule(
            "synthetic.multimodal",
            [
                PremiseKind::TableCell,
                PremiseKind::VisualRegion,
                PremiseKind::UniversalAxiom,
            ],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let mut builder = PromotionKernelBuilder::new(
            Sha256Digest::of_bytes(b"capture"),
            registry(vec![descriptor]),
        )
        .unwrap();
        let (cell, region) = {
            let mut capture = builder.capture();
            (
                capture
                    .table_cell("table-0", 2, 1, b"bounded value")
                    .unwrap(),
                capture
                    .visual_region("region-0", b"two connected nodes")
                    .unwrap(),
            )
        };
        let mut kernel = builder.seal();
        let proposal = kernel.grammar_capability().propose(
            address("multimodal"),
            RuleId::try_from("synthetic.multimodal".to_string()).unwrap(),
            vec![
                cell,
                region,
                PremiseRef::UniversalAxiom {
                    axiom_id: "binary.logic.level".to_string(),
                    version: 1,
                },
            ],
            vec![],
            ProofConfidence::Deterministic,
            3_u32,
        );
        kernel.promote(proposal).unwrap();
        assert_eq!(kernel.finish().unwrap().claims().len(), 1);
    }

    #[test]
    fn disallowed_premise_kind_and_unregistered_axiom_fail_closed() {
        let descriptor = rule(
            "synthetic.source_only",
            [PremiseKind::SourceSpan],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let mut builder = PromotionKernelBuilder::new(
            Sha256Digest::of_bytes(b"capture"),
            registry(vec![descriptor]),
        )
        .unwrap();
        let cell = builder
            .capture()
            .table_cell("table-0", 0, 0, b"cell")
            .unwrap();
        let mut kernel = builder.seal();
        let proposal = kernel.grammar_capability().propose(
            address("wrong-kind"),
            RuleId::try_from("synthetic.source_only".to_string()).unwrap(),
            vec![cell],
            vec![],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(proposal)
                .unwrap_err()
                .to_string()
                .contains("does not admit")
        );

        let descriptor = rule(
            "synthetic.axiom",
            [PremiseKind::UniversalAxiom],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let registry = RuleRegistry::new([registration(descriptor)], []).unwrap();
        let builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry).unwrap();
        let mut kernel = builder.seal();
        let proposal = kernel.grammar_capability().propose(
            address("unknown-axiom"),
            RuleId::try_from("synthetic.axiom".to_string()).unwrap(),
            vec![PremiseRef::UniversalAxiom {
                axiom_id: "binary.logic.level".to_string(),
                version: 1,
            }],
            vec![],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(proposal)
                .unwrap_err()
                .to_string()
                .contains("unregistered universal axiom")
        );
    }

    #[test]
    fn symbol_blind_and_exact_identity_rules_reject_excess_capability() {
        let descriptors = vec![
            rule(
                "synthetic.blind",
                [PremiseKind::SourceSpan],
                SymbolCapabilityClass::SymbolBlind,
                AlphaObligation::ByteIdenticalNonSymbolOutput,
            ),
            rule(
                "synthetic.identity",
                [PremiseKind::SourceSpan],
                SymbolCapabilityClass::ExactIdentityOnly,
                AlphaObligation::IdentityGraphInvariant,
            ),
        ];
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry(descriptors))
                .unwrap();
        let (premise, symbol) = {
            let mut capture = builder.capture();
            let premise = capture.source_span("span", b"content").unwrap();
            let symbol = capture.intern_symbol("ATOM").unwrap();
            (premise, symbol)
        };
        let mut kernel = builder.seal();
        let blind = kernel.grammar_capability().propose(
            address("blind"),
            RuleId::try_from("synthetic.blind".to_string()).unwrap(),
            vec![premise.clone()],
            vec![SymbolUse::copied(&symbol)],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(blind)
                .unwrap_err()
                .to_string()
                .contains("does not permit")
        );
        let identity = kernel.grammar_capability().propose(
            address("identity"),
            RuleId::try_from("synthetic.identity".to_string()).unwrap(),
            vec![premise],
            vec![SymbolUse::target_encoded(&symbol)],
            ProofConfidence::Deterministic,
            1_u32,
        );
        assert!(
            kernel
                .promote(identity)
                .unwrap_err()
                .to_string()
                .contains("does not permit")
        );
    }

    #[test]
    fn compatibility_envelope_still_requires_kernel_verification() {
        let (mut kernel, premise, symbol) = source_kernel();
        let ruleset = kernel.registry.ruleset_sha256().clone();
        let proposal = kernel.grammar_capability().propose(
            address("claim"),
            RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
            vec![premise],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            1_u32,
        );
        kernel.promote(proposal).unwrap();
        let ledger = kernel.finish().unwrap();
        let value = serde_json::to_value(&ledger).unwrap();
        assert_eq!(
            assess_proof_compatibility(Some(&value), &ruleset),
            ProofCompatibility::CurrentEnvelopeRequiresVerification
        );
        assert!(!assess_proof_compatibility(Some(&value), &ruleset).permits_canonical_authority());
        assert_eq!(
            assess_proof_compatibility(Some(&value), &ruleset).disposition(),
            ProofCompatibilityDisposition::RequireKernelVerification
        );
        assert!(ledger.permits_canonical_authority());
        assert_eq!(
            assess_proof_compatibility(None, &ruleset).disposition(),
            ProofCompatibilityDisposition::InspectAndRebuildOrResidualize
        );
        let legacy = serde_json::json!({"schema_version": 0});
        assert_eq!(
            assess_proof_compatibility(Some(&legacy), &ruleset),
            ProofCompatibility::UnsupportedLegacySchema { found: 0 }
        );
    }

    #[test]
    fn compatibility_rejects_future_and_malformed_and_residualizes_stale() {
        let expected = Sha256Digest::of_bytes(b"ruleset");
        let future = serde_json::json!({
            "schema_version": PROOF_LEDGER_SCHEMA_VERSION + 1,
            "ruleset_sha256": expected,
            "claims": []
        });
        assert_eq!(
            assess_proof_compatibility(Some(&future), &Sha256Digest::of_bytes(b"ruleset")),
            ProofCompatibility::UnsupportedFutureSchema { found: 2 }
        );
        assert_eq!(
            assess_proof_compatibility(Some(&future), &Sha256Digest::of_bytes(b"ruleset"))
                .disposition(),
            ProofCompatibilityDisposition::Reject
        );

        let malformed = serde_json::json!({
            "schema_version": PROOF_LEDGER_SCHEMA_VERSION,
            "ruleset_sha256": Sha256Digest::of_bytes(b"ruleset"),
            "claims": [],
            "unexpected": true
        });
        assert!(matches!(
            assess_proof_compatibility(Some(&malformed), &Sha256Digest::of_bytes(b"ruleset")),
            ProofCompatibility::InvalidCurrent { .. }
        ));

        let stale = serde_json::json!({
            "schema_version": PROOF_LEDGER_SCHEMA_VERSION,
            "ruleset_sha256": Sha256Digest::of_bytes(b"old-ruleset"),
            "claims": []
        });
        assert_eq!(
            assess_proof_compatibility(Some(&stale), &Sha256Digest::of_bytes(b"new-ruleset")),
            ProofCompatibility::StaleRuleset
        );
    }

    #[test]
    fn cumulative_chain_preserves_exact_verified_prefix_and_replays_local_suffix() {
        let (mut source_kernel, premise, symbol) = source_kernel();
        let source = source_kernel.grammar_capability().propose(
            address("source"),
            RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
            vec![premise],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            7_u32,
        );
        source_kernel.promote(source).unwrap();
        let upstream = source_kernel.finish().unwrap();
        let source_proof = upstream.claims().first().unwrap();
        let upstream_premise = PremiseRef::UpstreamClaim {
            address: source_proof.address().clone(),
            conclusion_sha256: source_proof.conclusion_sha256().clone(),
        };

        let downstream_registry = registry(vec![rule(
            "synthetic.carry",
            [PremiseKind::UpstreamClaim],
            SymbolCapabilityClass::LosslessCarry,
            AlphaObligation::LosslessTopologyInvariant,
        )]);
        let local_ruleset = downstream_registry.ruleset_sha256().clone();
        let mut downstream = PromotionKernelBuilder::with_verified_upstream(
            Sha256Digest::of_bytes(b"capture"),
            downstream_registry.clone(),
            &upstream,
        )
        .unwrap()
        .seal();
        let carried = downstream.grammar_capability().propose(
            address("evidence"),
            RuleId::try_from("synthetic.carry".to_string()).unwrap(),
            vec![upstream_premise],
            vec![],
            ProofConfidence::Deterministic,
            7_u32,
        );
        downstream.promote(carried).unwrap();
        let local = downstream.finish().unwrap();
        let local_conclusions = local.conclusions.clone();
        let cumulative = VerifiedProofLedger::compose(&upstream, local).unwrap();

        assert_eq!(
            &cumulative.claims()[..upstream.claims().len()],
            upstream.claims()
        );
        let local_suffix = cumulative
            .ledger()
            .local_suffix_after(&upstream, &local_ruleset)
            .unwrap();
        let verifier = PromotionKernelBuilder::with_verified_upstream(
            Sha256Digest::of_bytes(b"capture"),
            downstream_registry,
            &upstream,
        )
        .unwrap()
        .seal();
        verifier
            .verify_persisted(local_suffix, &local_conclusions)
            .unwrap();
    }

    #[test]
    fn cumulative_chain_rejects_prefix_or_stage_composition_tampering() {
        let (mut source_kernel, premise, symbol) = source_kernel();
        let source = source_kernel.grammar_capability().propose(
            address("source"),
            RuleId::try_from("synthetic.source_copy".to_string()).unwrap(),
            vec![premise],
            vec![SymbolUse::introduced(&symbol, 0)],
            ProofConfidence::Deterministic,
            7_u32,
        );
        source_kernel.promote(source).unwrap();
        let upstream = source_kernel.finish().unwrap();
        let downstream_registry = registry(vec![rule(
            "synthetic.carry",
            [PremiseKind::UpstreamClaim],
            SymbolCapabilityClass::LosslessCarry,
            AlphaObligation::LosslessTopologyInvariant,
        )]);
        let local_ruleset = downstream_registry.ruleset_sha256().clone();
        let source_proof = upstream.claims().first().unwrap();
        let mut downstream = PromotionKernelBuilder::with_verified_upstream(
            Sha256Digest::of_bytes(b"capture"),
            downstream_registry,
            &upstream,
        )
        .unwrap()
        .seal();
        let carried = downstream.grammar_capability().propose(
            address("evidence"),
            RuleId::try_from("synthetic.carry".to_string()).unwrap(),
            vec![PremiseRef::UpstreamClaim {
                address: source_proof.address().clone(),
                conclusion_sha256: source_proof.conclusion_sha256().clone(),
            }],
            vec![],
            ProofConfidence::Deterministic,
            7_u32,
        );
        downstream.promote(carried).unwrap();
        let mut cumulative = VerifiedProofLedger::compose(&upstream, downstream.finish().unwrap())
            .unwrap()
            .into_ledger();

        cumulative.claims[0].conclusion_sha256 = Sha256Digest::of_bytes(b"forged-prefix");
        assert!(
            cumulative
                .local_suffix_after(&upstream, &local_ruleset)
                .unwrap_err()
                .to_string()
                .contains("exact verified upstream prefix")
        );

        cumulative.claims[0] = upstream.claims()[0].clone();
        cumulative.ruleset_sha256 = Sha256Digest::of_bytes(b"reordered-stage-composition");
        assert!(
            cumulative
                .local_suffix_after(&upstream, &local_ruleset)
                .unwrap_err()
                .to_string()
                .contains("ruleset hash is stale")
        );
    }

    #[test]
    fn registered_derivation_binds_recomputed_output_and_input_topology() {
        let descriptor = rule(
            "synthetic.registered_replay",
            [PremiseKind::RegisteredDerivation],
            SymbolCapabilityClass::SymbolBlind,
            AlphaObligation::ByteIdenticalNonSymbolOutput,
        );
        let registry =
            RuleRegistry::new([RuleRegistration::new(descriptor, verify_exact_copy)], []).unwrap();
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry).unwrap();
        let replay = serde_json::to_vec(&7_u32).unwrap();
        let registered = {
            let mut capture = builder.capture();
            let input = capture
                .source_span("source", b"exact current input")
                .unwrap();
            capture
                .registered_derivation("synthetic.replay", &replay, vec![input])
                .unwrap()
        };
        let mut kernel = builder.seal();
        let proposal = kernel.grammar_capability().propose(
            address("derived"),
            RuleId::try_from("synthetic.registered_replay".to_string()).unwrap(),
            vec![registered],
            vec![],
            ProofConfidence::Deterministic,
            7_u32,
        );
        kernel.promote(proposal).unwrap();
        let registry = kernel.registry.clone();
        let evidence = kernel.evidence.clone();
        let verified = kernel.finish().unwrap();

        let mut ledger = verified.ledger.clone();
        let forged = serde_json::to_vec(&8_u32).unwrap();
        ledger.claims[0].conclusion_sha256 = Sha256Digest::of_bytes(&forged);
        let conclusions = BTreeMap::from([(address("derived"), forged)]);
        let verifier = PromotionKernel {
            seal: CapabilitySeal,
            registry,
            evidence,
            claims: Vec::new(),
            conclusions: BTreeMap::new(),
            upstream_conclusions: BTreeMap::new(),
        };
        assert!(
            verifier
                .verify_persisted(ledger, &conclusions)
                .unwrap_err()
                .to_string()
                .contains("executable exact-copy relation rejected")
        );
    }

    #[test]
    fn conclusion_digest_and_upstream_topology_detect_tampering() {
        let registry = registry(vec![
            rule(
                "synthetic.capture",
                [PremiseKind::SourceSpan],
                SymbolCapabilityClass::SymbolBlind,
                AlphaObligation::ByteIdenticalNonSymbolOutput,
            ),
            rule(
                "synthetic.carry",
                [PremiseKind::UpstreamClaim],
                SymbolCapabilityClass::LosslessCarry,
                AlphaObligation::LosslessTopologyInvariant,
            ),
        ]);
        let mut builder =
            PromotionKernelBuilder::new(Sha256Digest::of_bytes(b"capture"), registry).unwrap();
        let premise = builder.capture().source_span("span", b"content").unwrap();
        let mut kernel = builder.seal();
        let first = kernel.grammar_capability().propose(
            address("first"),
            RuleId::try_from("synthetic.capture".to_string()).unwrap(),
            vec![premise],
            vec![],
            ProofConfidence::Captured,
            1_u32,
        );
        let first = kernel.promote(first).unwrap();
        let upstream = PremiseRef::UpstreamClaim {
            address: first.proof().address().clone(),
            conclusion_sha256: first.proof().conclusion_sha256().clone(),
        };
        let second = kernel.grammar_capability().propose(
            address("second"),
            RuleId::try_from("synthetic.carry".to_string()).unwrap(),
            vec![upstream],
            vec![],
            ProofConfidence::Deterministic,
            first.into_value(),
        );
        kernel.promote(second).unwrap();
        let registry = kernel.registry.clone();
        let evidence = kernel.evidence.clone();
        let conclusions = kernel.conclusions.clone();
        let ledger = kernel.finish().unwrap();
        assert_eq!(ledger.claims().len(), 2);

        let mut value = serde_json::to_value(ledger).unwrap();
        value["claims"][0]["conclusion_sha256"] =
            serde_json::json!(Sha256Digest::of_bytes(b"tampered"));
        let reparsed: ProofLedger = serde_json::from_value(value).unwrap();
        let verifier = PromotionKernel {
            seal: CapabilitySeal,
            registry,
            evidence,
            claims: vec![],
            conclusions: BTreeMap::new(),
            upstream_conclusions: BTreeMap::new(),
        };
        let error = verifier
            .verify_persisted(reparsed, &conclusions)
            .unwrap_err();
        assert!(error.to_string().contains("conclusion digest mismatch"));
    }
}
