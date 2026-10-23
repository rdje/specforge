use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::derivation::{
    AlphaObligation, ClaimAddress, DerivationError, DerivationResult, PremiseKind, PremiseRef,
    PromotionKernelBuilder, ProofConfidence, ProofLedger, RuleCompatibility, RuleDescriptor,
    RuleId, RuleRegistration, RuleRegistry, RuleVerificationContext, Sha256Digest,
    SymbolCapabilityClass, VerifiedProofLedger, production_semantic_implementation_digest,
};
use crate::ir::evidence::{
    KeywordDefinitionRecord, OptionalFeatureRecord, PermissionRuleRecord, SignalPresenceRecord,
    SoftwareIntentRecord, StatedAbsentValue,
};
use crate::ir::intent::{IntentDocumentIdentity, IntentIr};
use crate::ir::isf_ir::IsfIr;
use crate::ir::semantic::SystemResetPolarity;
use crate::ir::source::{AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket};
use crate::persisted_path::{
    PersistedPathOrigin, normalize_for_storage, resolve_existing, resolve_repository_output,
};

const ADAPTER_ARTIFACT_SCHEMA_VERSION: u32 = 2;
const ADAPTER_PROOF_CONTEXT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct AdapterProofContext {
    schema_version: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    mutations: Vec<AdapterMutationEvent>,
    #[cfg(any(test, feature = "test-support"))]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    test_fixture: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[doc(hidden)]
pub enum AdapterMutationKind {
    ValidationBackannotation,
    #[cfg(any(test, feature = "test-support"))]
    TestFixture,
}

impl AdapterMutationKind {
    fn allowed_fields(self) -> Vec<&'static str> {
        match self {
            Self::ValidationBackannotation => vec!["validation_reports"],
            #[cfg(any(test, feature = "test-support"))]
            Self::TestFixture => ADAPTER_RULE_FIELDS
                .iter()
                .map(|(field, _)| *field)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct AdapterMutationEvent {
    mutation_id: String,
    kind: AdapterMutationKind,
    fields: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug)]
struct AdapterClaimInput {
    address: ClaimAddress,
    rule_id: RuleId,
    conclusion: serde_json::Value,
}

#[derive(Debug)]
struct AdapterProofPremises {
    claim_replays: BTreeMap<ClaimAddress, PremiseRef>,
    mutations: Vec<(AdapterMutationKind, PremiseRef)>,
}

impl AdapterProofPremises {
    fn for_claim(
        &self,
        address: &ClaimAddress,
    ) -> DerivationResult<(Vec<PremiseRef>, ProofConfidence)> {
        let mut premises = vec![self.claim_replays.get(address).cloned().ok_or_else(|| {
            DerivationError::new(format!(
                "adapter claim '{}:{}' lacks its registered replay",
                address.surface(),
                address.stable_record_key()
            ))
        })?];
        premises.extend(
            self.mutations
                .iter()
                .filter(|(kind, _)| kind.allowed_fields().contains(&address.surface()))
                .map(|(_, premise)| premise.clone()),
        );
        Ok((premises, ProofConfidence::Deterministic))
    }
}
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    proof_context: Option<AdapterProofContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    proof_ledger: Option<ProofLedger>,
}

const ADAPTER_RULE_FIELDS: &[(&str, &str)] = &[
    ("stage", "adapter.envelope"),
    ("schema_version", "adapter.envelope"),
    ("target", "adapter.envelope"),
    ("required_input_stage", "adapter.envelope"),
    ("intent_ir_path", "adapter.envelope"),
    ("artifact_layout", "adapter.envelope"),
    ("adapter_identity", "adapter.envelope"),
    ("document_identity", "adapter.envelope"),
    ("lowering_status", "adapter.lowering"),
    ("isf", "adapter.lowering"),
    ("residual_decisions", "adapter.residual"),
    ("validation_reports", "adapter.validation"),
];

fn adapter_derivation_error(error: impl std::fmt::Display) -> AppError {
    AppError::InvalidStageArtifact(format!("adapter proof verification failed: {error}"))
}

fn adapter_rule_registry() -> DerivationResult<RuleRegistry> {
    let implementation_sha256 = production_semantic_implementation_digest(IrStage::IsfAdapter)?;
    let registrations = ADAPTER_RULE_FIELDS
        .iter()
        .map(|(field, family)| {
            let (capability, alpha, compatibility) = match *family {
                "adapter.lowering" => (
                    SymbolCapabilityClass::TargetLowering,
                    AlphaObligation::TargetSafeRenaming,
                    RuleCompatibility::CurrentOnly,
                ),
                "adapter.residual" => (
                    SymbolCapabilityClass::Residual,
                    AlphaObligation::ResidualTopologyInvariant,
                    RuleCompatibility::CurrentOnly,
                ),
                _ => (
                    SymbolCapabilityClass::ExactIdentityOnly,
                    AlphaObligation::IdentityGraphInvariant,
                    RuleCompatibility::CurrentOnly,
                ),
            };
            let rule_id =
                RuleId::try_from(format!("{family}.{field}.v1")).map_err(DerivationError::new)?;
            let descriptor = RuleDescriptor::new(
                rule_id,
                1,
                "crate::ir::adapters",
                implementation_sha256.clone(),
                [
                    PremiseKind::RegisteredDerivation,
                    PremiseKind::UpstreamClaim,
                ],
                IrStage::IsfAdapter,
                *field,
                capability,
                alpha,
                compatibility,
            )?;
            Ok(RuleRegistration::new(
                descriptor,
                verify_adapter_rule_relation,
            ))
        })
        .collect::<DerivationResult<Vec<_>>>()?;
    RuleRegistry::new(registrations, [])
}

#[cfg(test)]
pub(super) fn rule_registry_for_qualification() -> DerivationResult<RuleRegistry> {
    adapter_rule_registry()
}

fn verify_adapter_rule_relation(context: RuleVerificationContext<'_>) -> DerivationResult<()> {
    let expected_bytes = context
        .premise_bytes(0)?
        .ok_or_else(|| DerivationError::new("adapter rule lacks claim replay bytes"))?;
    if expected_bytes == context.conclusion_json() {
        Ok(())
    } else {
        Err(DerivationError::new(format!(
            "adapter field '{}' is not the current registered replay",
            context.proof().address().surface()
        )))
    }
}

fn apply_adapter_mutations(
    artifact: &mut AdapterArtifact,
    mutations: &[AdapterMutationEvent],
) -> DerivationResult<()> {
    let mut seen = BTreeSet::new();
    for mutation in mutations {
        if !seen.insert(mutation.mutation_id.as_str()) {
            return Err(DerivationError::new(format!(
                "duplicate adapter mutation id '{}'",
                mutation.mutation_id
            )));
        }
        let allowed = mutation
            .kind
            .allowed_fields()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let present = mutation
            .fields
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if present != allowed {
            return Err(DerivationError::new(format!(
                "adapter {:?} mutation field set is not exact",
                mutation.kind
            )));
        }
        let mut value = serde_json::to_value(&*artifact).map_err(|error| {
            DerivationError::new(format!("cannot serialize adapter mutation base: {error}"))
        })?;
        let object = value.as_object_mut().ok_or_else(|| {
            DerivationError::new("serialized adapter mutation base is not an object")
        })?;
        for (field, replacement) in &mutation.fields {
            object.insert(field.clone(), replacement.clone());
        }
        object.remove("proof_context");
        object.remove("proof_ledger");
        *artifact = serde_json::from_value(value).map_err(|error| {
            DerivationError::new(format!("invalid typed adapter mutation payload: {error}"))
        })?;
    }
    Ok(())
}

impl AdapterArtifact {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        Self::load_with_verified_proof(path).map(|(artifact, _)| artifact)
    }

    fn load_with_verified_proof(path: &Path) -> Result<(Self, VerifiedProofLedger)> {
        let path = resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
        let value = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(path)?)?;
        let version = value
            .get("schema_version")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(
                    "adapter is missing an integer schema_version".to_string(),
                )
            })?;
        if version != u64::from(ADAPTER_ARTIFACT_SCHEMA_VERSION) {
            let disposition = if version < u64::from(ADAPTER_ARTIFACT_SCHEMA_VERSION) {
                "is legacy/proofless and inspection-only; rebuild it from verified IntentIR"
            } else {
                "is newer than this binary"
            };
            return Err(AppError::InvalidStageArtifact(format!(
                "adapter schema version {version} {disposition}"
            )));
        }
        let artifact = serde_json::from_value::<Self>(value)?;
        if !matches!(artifact.stage, IrStage::IsfAdapter) {
            return Err(AppError::InvalidStageArtifact(
                "artifact must be an ISF adapter document before loading an adapter".to_string(),
            ));
        }
        let runtime = artifact.runtime_clone()?;
        let canonical = runtime.persisted_clone()?;
        let verified = canonical.verified_canonical_proof()?;
        Ok((runtime, verified))
    }

    /// Parse historical adapter output for diagnostics without granting it canonical or emitted
    /// target authority. Only current schema with verified proof may serialize, write, or emit ISF.
    pub fn load_for_inspection(path: &Path) -> Result<Self> {
        let path = resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
        let value = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(path)?)?;
        let version = value
            .get("schema_version")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(
                    "adapter is missing an integer schema_version".to_string(),
                )
            })?;
        if version > u64::from(ADAPTER_ARTIFACT_SCHEMA_VERSION) {
            return Err(AppError::InvalidStageArtifact(format!(
                "adapter schema version {version} is newer than this binary"
            )));
        }
        let artifact = serde_json::from_value::<Self>(value)?;
        if !matches!(artifact.stage, IrStage::IsfAdapter) {
            return Err(AppError::InvalidStageArtifact(
                "artifact must be an ISF adapter document before inspection".to_string(),
            ));
        }
        if version == u64::from(ADAPTER_ARTIFACT_SCHEMA_VERSION) {
            artifact.verify_canonical_proof()?;
        }
        artifact.runtime_clone()
    }

    pub fn build(
        intent_ir_path: &Path,
        target: AdapterTarget,
        artifact_base_root: &Path,
    ) -> Result<Self> {
        let intent_ir_path =
            resolve_existing(intent_ir_path, PersistedPathOrigin::RepositoryOwned)?;
        let (intent_ir, intent_proof) = IntentIr::load_with_verified_proof(&intent_ir_path)?;

        if !matches!(intent_ir.stage, IrStage::IntentIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an IntentIR document before building an adapter artifact",
                intent_ir_path.display()
            )));
        }

        match target {
            AdapterTarget::Isf => {
                let mut artifact =
                    build_isf_adapter_artifact(&intent_ir, &intent_ir_path, artifact_base_root)?;
                artifact.refresh_canonical_proof(&intent_proof)?;
                Ok(artifact)
            }
        }
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        let persisted = self.persisted_clone()?;
        persisted.verify_canonical_proof()?;
        Ok(serde_json::to_string_pretty(&persisted)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        let persisted = self.persisted_clone()?;
        persisted.verify_canonical_proof()?;
        let runtime_layout = persisted.artifact_layout.runtime_layout()?;
        fs::create_dir_all(&runtime_layout.artifact_root)?;
        fs::write(
            &runtime_layout.adapter_artifact_path,
            serde_json::to_string_pretty(&persisted)?,
        )?;

        if let (Some(path), Some(text)) = (
            runtime_layout.emitted_target_path.as_ref(),
            persisted.rendered_target_text(),
        ) {
            fs::write(path, text)?;
        }

        reconcile_emitted_isf_files(&runtime_layout)?;

        Ok(())
    }

    /// Persist a deliberately synthetic adapter fixture. Production builds do not compile this
    /// mutation kind, so arbitrary target text cannot cross the canonical write seam.
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn write_test_fixture_to_disk(&self) -> Result<()> {
        if self.persisted_clone()?.verify_canonical_proof().is_ok() {
            return self.write_to_disk();
        }
        let mut fixture = self.clone();
        fixture.authorize_mutation(AdapterMutationKind::TestFixture)?;
        fixture.write_to_disk()
    }

    fn persisted_clone(&self) -> Result<Self> {
        let mut persisted = self.clone();
        persisted.intent_ir_path = normalize_for_storage(
            &persisted.intent_ir_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        persisted.artifact_layout.normalize_for_storage()?;
        Ok(persisted)
    }

    fn runtime_clone(&self) -> Result<Self> {
        let mut runtime = self.clone();
        runtime.intent_ir_path = resolve_existing(
            &runtime.intent_ir_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        runtime.artifact_layout = runtime.artifact_layout.runtime_layout()?;
        Ok(runtime)
    }

    fn rendered_target_text(&self) -> Option<String> {
        match &self.isf {
            Some(isf) if isf.is_renderable => Some(isf.source_text.clone()),
            _ => None,
        }
    }

    fn public_field_values(&self) -> DerivationResult<BTreeMap<String, serde_json::Value>> {
        let mut fields = BTreeMap::new();
        macro_rules! insert_field {
            ($field:ident) => {
                fields.insert(
                    stringify!($field).to_string(),
                    serde_json::to_value(&self.$field).map_err(|error| {
                        DerivationError::new(format!(
                            "cannot serialize adapter field '{}': {error}",
                            stringify!($field)
                        ))
                    })?,
                );
            };
        }
        insert_field!(stage);
        insert_field!(schema_version);
        insert_field!(target);
        insert_field!(required_input_stage);
        insert_field!(intent_ir_path);
        insert_field!(artifact_layout);
        insert_field!(adapter_identity);
        insert_field!(document_identity);
        insert_field!(lowering_status);
        insert_field!(residual_decisions);
        insert_field!(validation_reports);
        insert_field!(isf);
        if fields.len() != ADAPTER_RULE_FIELDS.len() {
            return Err(DerivationError::new(
                "adapter proof field projection is incomplete",
            ));
        }
        Ok(fields)
    }

    fn claim_inputs(&self) -> DerivationResult<Vec<AdapterClaimInput>> {
        let fields = self.public_field_values()?;
        let mut claims = Vec::new();
        for (field, family) in ADAPTER_RULE_FIELDS {
            let value = fields.get(*field).cloned().ok_or_else(|| {
                DerivationError::new(format!("adapter claim field '{field}' is absent"))
            })?;
            let rule_id =
                RuleId::try_from(format!("{family}.{field}.v1")).map_err(DerivationError::new)?;
            claims.push(AdapterClaimInput {
                address: ClaimAddress::new(IrStage::IsfAdapter, *field, "root", None)?,
                rule_id: rule_id.clone(),
                conclusion: value.clone(),
            });
            if let Some(records) = value.as_array() {
                for (index, record) in records.iter().enumerate() {
                    claims.push(AdapterClaimInput {
                        address: ClaimAddress::new(
                            IrStage::IsfAdapter,
                            *field,
                            format!("record-{index:08}"),
                            Some(format!("[{index}]")),
                        )?,
                        rule_id: rule_id.clone(),
                        conclusion: record.clone(),
                    });
                }
            }
        }
        if let Some(isf) = &self.isf {
            let rule_id = RuleId::try_from("adapter.lowering.isf.v1".to_string())
                .map_err(DerivationError::new)?;
            for (index, line) in isf.source_text.lines().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                claims.push(AdapterClaimInput {
                    address: ClaimAddress::new(
                        IrStage::IsfAdapter,
                        "isf",
                        format!("source-line-{index:08}"),
                        Some(format!("source_text.lines[{index}]")),
                    )?,
                    rule_id: rule_id.clone(),
                    conclusion: serde_json::Value::String(line.to_string()),
                });
            }
            for (index, reason) in isf.blocking_reasons.iter().enumerate() {
                claims.push(AdapterClaimInput {
                    address: ClaimAddress::new(
                        IrStage::IsfAdapter,
                        "isf",
                        format!("blocking-reason-{index:08}"),
                        Some(format!("blocking_reasons[{index}]")),
                    )?,
                    rule_id: rule_id.clone(),
                    conclusion: serde_json::Value::String(reason.clone()),
                });
            }
        }
        Ok(claims)
    }

    fn proof_kernel(
        &self,
        context: &AdapterProofContext,
        intent_proof: &VerifiedProofLedger,
        replay_bytes: &[u8],
        claims: &[AdapterClaimInput],
    ) -> DerivationResult<(crate::ir::derivation::PromotionKernel, AdapterProofPremises)> {
        if context.schema_version != ADAPTER_PROOF_CONTEXT_SCHEMA_VERSION {
            return Err(DerivationError::new(format!(
                "unsupported adapter proof-context schema {}",
                context.schema_version
            )));
        }
        let intent_ledger_bytes = serde_json::to_vec(intent_proof.ledger()).map_err(|error| {
            DerivationError::new(format!(
                "cannot serialize verified IntentIR ledger: {error}"
            ))
        })?;
        let capture_digest = Sha256Digest::of_serializable(&(
            context,
            intent_proof.ruleset_sha256(),
            Sha256Digest::of_bytes(&intent_ledger_bytes),
        ))?;
        let registry = adapter_rule_registry()?;
        let mut builder =
            PromotionKernelBuilder::with_verified_upstream(capture_digest, registry, intent_proof)?;
        let proof_premises = {
            let mut capture = builder.capture();
            let intent_grounding =
                capture.source_span("adapter-verified-intent-ledger", &intent_ledger_bytes)?;
            let mut inputs = intent_proof
                .claims()
                .iter()
                .map(|proof| PremiseRef::UpstreamClaim {
                    address: proof.address().clone(),
                    conclusion_sha256: proof.conclusion_sha256().clone(),
                })
                .collect::<Vec<_>>();
            inputs.push(intent_grounding.clone());
            let mut mutation_premises = Vec::new();
            for mutation in &context.mutations {
                let payload = serde_json::to_vec(mutation).map_err(|error| {
                    DerivationError::new(format!(
                        "cannot serialize registered adapter mutation: {error}"
                    ))
                })?;
                let premise = capture.registered_derivation(
                    mutation.mutation_id.clone(),
                    &payload,
                    vec![intent_grounding.clone()],
                )?;
                inputs.push(premise.clone());
                mutation_premises.push((mutation.kind, premise));
            }
            let registered_replay =
                capture.registered_derivation("adapter.current-replay", replay_bytes, inputs)?;
            let mut claim_replays = BTreeMap::new();
            for claim in claims {
                let exact_conclusion = serde_json::to_vec(&claim.conclusion).map_err(|error| {
                    DerivationError::new(format!("cannot serialize adapter claim replay: {error}"))
                })?;
                let premise = capture.registered_derivation(
                    format!(
                        "adapter.claim.{}.{}",
                        claim.address.surface(),
                        claim.address.stable_record_key()
                    ),
                    &exact_conclusion,
                    vec![registered_replay.clone()],
                )?;
                if claim_replays
                    .insert(claim.address.clone(), premise)
                    .is_some()
                {
                    return Err(DerivationError::new(
                        "duplicate adapter claim replay address",
                    ));
                }
            }
            AdapterProofPremises {
                claim_replays,
                mutations: mutation_premises,
            }
        };
        Ok((builder.seal(), proof_premises))
    }

    fn refresh_canonical_proof(&mut self, intent_proof: &VerifiedProofLedger) -> Result<()> {
        let context = AdapterProofContext {
            schema_version: ADAPTER_PROOF_CONTEXT_SCHEMA_VERSION,
            mutations: Vec::new(),
            #[cfg(any(test, feature = "test-support"))]
            test_fixture: false,
        };
        self.refresh_proof_from_context(context, intent_proof)
    }

    fn refresh_proof_from_context(
        &mut self,
        context: AdapterProofContext,
        intent_proof: &VerifiedProofLedger,
    ) -> Result<()> {
        let mut persisted = self.persisted_clone()?;
        persisted.proof_context = None;
        persisted.proof_ledger = None;
        let replay_bytes = serde_json::to_vec(
            &persisted
                .public_field_values()
                .map_err(adapter_derivation_error)?,
        )?;
        let claims = persisted.claim_inputs().map_err(adapter_derivation_error)?;
        let (mut kernel, proof_premises) = persisted
            .proof_kernel(&context, intent_proof, &replay_bytes, &claims)
            .map_err(adapter_derivation_error)?;
        for claim in claims {
            let (premises, confidence) = proof_premises
                .for_claim(&claim.address)
                .map_err(adapter_derivation_error)?;
            let proposal = kernel.grammar_capability().propose(
                claim.address,
                claim.rule_id,
                premises,
                Vec::new(),
                confidence,
                claim.conclusion,
            );
            kernel.promote(proposal).map_err(adapter_derivation_error)?;
        }
        let local = kernel.finish().map_err(adapter_derivation_error)?;
        let cumulative =
            VerifiedProofLedger::compose(intent_proof, local).map_err(adapter_derivation_error)?;
        self.proof_context = Some(context);
        self.proof_ledger = Some(cumulative.into_ledger());
        Ok(())
    }

    /// Authorize one closed-family post-build mutation and rebuild the cumulative proof.
    #[doc(hidden)]
    pub fn authorize_mutation(&mut self, kind: AdapterMutationKind) -> Result<bool> {
        let mut context = self.proof_context.clone().ok_or_else(|| {
            adapter_derivation_error("proofless adapter cannot authorize a mutation")
        })?;
        #[cfg(any(test, feature = "test-support"))]
        if kind == AdapterMutationKind::TestFixture {
            context.test_fixture = true;
        }
        let intent_runtime_path =
            resolve_existing(&self.intent_ir_path, PersistedPathOrigin::RepositoryOwned)?;
        let (intent_ir, intent_proof) = IntentIr::load_with_verified_proof(&intent_runtime_path)?;
        let runtime_layout = self.artifact_layout.runtime_layout()?;
        let artifact_base_root = runtime_layout
            .artifact_root
            .parent()
            .and_then(Path::parent)
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(
                    "adapter artifact root has no repository-owned base".to_string(),
                )
            })?;
        let mut predecessor =
            build_isf_adapter_artifact(&intent_ir, &intent_runtime_path, artifact_base_root)?;
        apply_adapter_mutations(&mut predecessor, &context.mutations)
            .map_err(adapter_derivation_error)?;
        let predecessor_fields = predecessor
            .persisted_clone()?
            .public_field_values()
            .map_err(adapter_derivation_error)?;
        let current_fields = self
            .persisted_clone()?
            .public_field_values()
            .map_err(adapter_derivation_error)?;
        let allowed = kind
            .allowed_fields()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        for (field, predecessor_value) in &predecessor_fields {
            if !allowed.contains(field.as_str())
                && current_fields.get(field) != Some(predecessor_value)
            {
                return Err(adapter_derivation_error(format!(
                    "{:?} mutation changed unauthorized adapter field '{field}'",
                    kind
                )));
            }
        }
        if kind
            .allowed_fields()
            .iter()
            .all(|field| current_fields.get(*field) == predecessor_fields.get(*field))
        {
            return Ok(false);
        }
        let fields = kind
            .allowed_fields()
            .iter()
            .map(|field| {
                current_fields
                    .get(*field)
                    .cloned()
                    .map(|value| ((*field).to_string(), value))
                    .ok_or_else(|| {
                        adapter_derivation_error(format!(
                            "adapter mutation field '{field}' is absent"
                        ))
                    })
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let kind_name = serde_json::to_value(kind)?
            .as_str()
            .expect("AdapterMutationKind serializes as a string")
            .to_string();
        let event = AdapterMutationEvent {
            mutation_id: format!(
                "adapter-mutation-{index:08}-{kind_name}",
                index = context.mutations.len()
            ),
            kind,
            fields,
        };
        apply_adapter_mutations(&mut predecessor, std::slice::from_ref(&event))
            .map_err(adapter_derivation_error)?;
        if predecessor
            .persisted_clone()?
            .public_field_values()
            .map_err(adapter_derivation_error)?
            != current_fields
        {
            return Err(adapter_derivation_error(
                "typed adapter mutation replay does not reproduce the requested artifact",
            ));
        }
        context.mutations.push(event);
        self.refresh_proof_from_context(context, &intent_proof)?;
        Ok(true)
    }

    fn verify_canonical_proof(&self) -> Result<()> {
        self.verified_canonical_proof().map(|_| ())
    }

    fn verified_canonical_proof(&self) -> Result<VerifiedProofLedger> {
        if self.schema_version != ADAPTER_ARTIFACT_SCHEMA_VERSION {
            return Err(AppError::InvalidStageArtifact(format!(
                "adapter schema {} cannot receive current canonical authority",
                self.schema_version
            )));
        }
        let context = self.proof_context.as_ref().ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "current adapter is proofless; rebuild it from verified IntentIR".to_string(),
            )
        })?;
        let cumulative = self.proof_ledger.clone().ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "current adapter is missing its cumulative proof ledger".to_string(),
            )
        })?;
        let intent_runtime_path =
            resolve_existing(&self.intent_ir_path, PersistedPathOrigin::RepositoryOwned)?;
        let (intent_ir, intent_proof) = IntentIr::load_with_verified_proof(&intent_runtime_path)?;
        let runtime_layout = self.artifact_layout.runtime_layout()?;
        let artifact_base_root = runtime_layout
            .artifact_root
            .parent()
            .and_then(Path::parent)
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(
                    "adapter artifact root has no repository-owned base".to_string(),
                )
            })?;
        let mut expected_runtime =
            build_isf_adapter_artifact(&intent_ir, &intent_runtime_path, artifact_base_root)?;
        apply_adapter_mutations(&mut expected_runtime, &context.mutations)
            .map_err(adapter_derivation_error)?;
        let expected = expected_runtime.persisted_clone()?;
        let replay_fields = expected
            .public_field_values()
            .map_err(adapter_derivation_error)?;
        let replay_bytes = serde_json::to_vec(&replay_fields)?;
        let actual_fields = self
            .public_field_values()
            .map_err(adapter_derivation_error)?;
        let claims = self.claim_inputs().map_err(adapter_derivation_error)?;
        let conclusions = claims
            .iter()
            .map(|claim| {
                serde_json::to_vec(&claim.conclusion)
                    .map(|bytes| (claim.address.clone(), bytes))
                    .map_err(AppError::from)
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let (kernel, _) = self
            .proof_kernel(context, &intent_proof, &replay_bytes, &claims)
            .map_err(adapter_derivation_error)?;
        let local_ruleset = adapter_rule_registry()
            .map_err(adapter_derivation_error)?
            .ruleset_sha256()
            .clone();
        let local_ledger = cumulative
            .local_suffix_after(&intent_proof, &local_ruleset)
            .map_err(adapter_derivation_error)?;
        let verified_local = kernel
            .verify_persisted(local_ledger, &conclusions)
            .map_err(adapter_derivation_error)?;
        let verified = VerifiedProofLedger::compose(&intent_proof, verified_local)
            .map_err(adapter_derivation_error)?;
        if verified.ledger() != &cumulative {
            return Err(adapter_derivation_error(
                "cumulative adapter proof differs from verified IntentIR prefix plus local replay",
            ));
        }
        if actual_fields != replay_fields {
            return Err(adapter_derivation_error(
                "adapter public fields differ from current registered replay",
            ));
        }
        Ok(verified)
    }

    /// Current cumulative proof ledger. This is inspection-only; canonical authority still
    /// requires executing the current loader and lowering relation.
    pub fn proof_ledger(&self) -> Option<&ProofLedger> {
        self.proof_ledger.as_ref()
    }
}

/// Make the generated document directory agree with the adapter manifest after
/// a successful write. Actor selection and renderability can change between
/// runs, so retaining an older sibling `.isf` would expose an output that the
/// current `adapter.json` neither names nor licenses. Only generated ISF leaf
/// files are reconciled; unrelated files and directories remain untouched.
fn reconcile_emitted_isf_files(layout: &AdapterArtifactLayout) -> Result<()> {
    for entry in fs::read_dir(&layout.artifact_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "isf")
            || layout
                .emitted_target_path
                .as_ref()
                .is_some_and(|current| current == &path)
        {
            continue;
        }

        let file_type = entry.file_type()?;
        if file_type.is_file() || file_type.is_symlink() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterArtifactLayout {
    pub artifact_root: PathBuf,
    pub adapter_artifact_path: PathBuf,
    pub emitted_target_path: Option<PathBuf>,
}

impl AdapterArtifactLayout {
    fn normalize_for_storage(&mut self) -> Result<()> {
        self.artifact_root =
            normalize_for_storage(&self.artifact_root, PersistedPathOrigin::RepositoryOwned)?;
        self.adapter_artifact_path = normalize_for_storage(
            &self.adapter_artifact_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        if let Some(path) = &mut self.emitted_target_path {
            *path = normalize_for_storage(path, PersistedPathOrigin::RepositoryOwned)?;
        }
        Ok(())
    }

    fn runtime_layout(&self) -> Result<Self> {
        Ok(Self {
            artifact_root: resolve_repository_output(&self.artifact_root)?,
            adapter_artifact_path: resolve_repository_output(&self.adapter_artifact_path)?,
            emitted_target_path: self
                .emitted_target_path
                .as_deref()
                .map(resolve_repository_output)
                .transpose()?,
        })
    }
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
    // KG-ISF-COMPLETENESS.2a.ii: the single flat `.isf` module is lowered from the protocol's
    // INITIATOR actor's perspective (owner-chosen, `2026-06-17`), so the module is NAMED after that
    // same initiator — keeping the module label and its `(input)`/`(output)` interface coherent (the
    // emitter, `isf_ir::from_intent_ir`, derives the directions from the same initiator). The
    // initiator is recovered structurally from the actor-port graph (`select_initiator_actor`, no
    // chip-name list — ADR 0006); when there is no net-producer initiator (e.g. a register/command
    // doc with no wire actors) we keep the prior `actors.first()` → `document_key` fallback, so those
    // documents' module names are byte-identical.
    // KG-ISF-COMPLETENESS.2a.iii: route every candidate through the shared HDL-identifier sanitizer
    // (`sanitize_isf_name`, an allowlist) so the emitted `(actor <name>)` module label is always a valid
    // FSMGen module name (`[A-Za-z_]\w*`) — a prose-fragment initiator carrying punctuation (e.g. the
    // arrow `→` in GIC-600's `redistributor → distributor`) no longer malforms the whole `.isf`. This is
    // ONLY the module LABEL; `from_intent_ir` re-derives the initiator raw for direction matching, so the
    // sanitized label cannot affect initiator port selection. Byte-identical for clean names
    // (`Manager`→`manager`, `Requester`→`requester`, `debugger`), so the wire golds are unchanged.
    if let Some(initiator) = crate::ir::isf_ir::select_initiator_for(intent_ir)
        && !initiator.is_empty()
    {
        return crate::ir::isf_ir::sanitize_isf_name(&initiator);
    }
    if let Some(actor) = intent_ir.actors.first()
        && let Some(name) = &actor.actor_name
        && !name.is_empty()
    {
        return crate::ir::isf_ir::sanitize_isf_name(name);
    }
    crate::ir::isf_ir::sanitize_isf_name(&intent_ir.document_identity.document_key)
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
// ISF lowering requires source-grounded clock/reset semantics in addition to signals and
// behavior. FSMGen's grammar requires a clock plus reset timing and polarity; inventing those
// values from names or conventional defaults would turn adapter syntax requirements into false
// design intent.
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

    match intent_ir.system_contract.as_ref() {
        None => reasons.push("no source-grounded system clock/reset contract".to_string()),
        Some(contract) if matches!(contract.reset_polarity, SystemResetPolarity::Unknown) => {
            reasons.push("system reset polarity is unresolved".to_string())
        }
        Some(_) => {}
    }

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

/// EXTRACTION-REPRESENTATION-GAPS.3.d.iii.a — one residual for every permission the document states: the records
/// are numbered consecutively, so their range names them all.
fn permission_residual_packet(records: &[PermissionRuleRecord]) -> Option<ResidualDecisionPacket> {
    let first = records.first()?;
    let last = records.last()?;
    let statements = records
        .iter()
        .flat_map(|record| record.supporting_statement_ids.iter())
        .collect::<BTreeSet<_>>()
        .len();
    Some(ResidualDecisionPacket {
        packet_id: "isf_protocol_permission_rules".to_string(),
        question: "How should the permissions the document states be represented downstream of `.isf`?".to_string(),
        why_unresolved: format!(
            "{} permission(s) (`{}`-`{}`, from {} statement(s)) state what a subject may do, under an optional condition; `.isf` has no construct for an optional behavior, and a permission's subject and permitted part are as written, not bound to an actor, signal or transaction. They remain exact in IntentIR and are not rendered.",
            records.len(),
            first.permission_id,
            last.permission_id,
            statements
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "preserve_intent_observation".to_string(),
                description: "Keep the stated permissions in IntentIR and expose this adapter residual without emitting executable syntax.".to_string(),
                downstream_impact: "Independently licensed ISF remains usable; what the document leaves optional stays visible but unmodelled.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "lower_as_choice_after_bindings".to_string(),
                description: "Lower a permission as a choice the subject may take, once its subject is bound to an actor and its permitted part to a typed action.".to_string(),
                downstream_impact: "A future adapter could model optional behavior as nondeterminism after the bindings are source-grounded and FSMGen-strict verified.".to_string(),
            },
        ],
    })
}

/// EXTRACTION-REPRESENTATION-GAPS.3.d.iii.d — one residual for every optional feature the document states, named by
/// the range of its consecutively numbered records.
fn optional_feature_residual_packet(
    records: &[OptionalFeatureRecord],
) -> Option<ResidualDecisionPacket> {
    let first = records.first()?;
    let last = records.last()?;
    Some(ResidualDecisionPacket {
        packet_id: "isf_protocol_optional_features".to_string(),
        question: "How should the features the document states are optional be represented downstream of `.isf`?".to_string(),
        why_unresolved: format!(
            "{} optional feature(s) (`{}`-`{}`) state what may be absent, under an optional condition; `.isf` has no construct for an optional feature or a configuration that omits one, and each feature is a phrase as written, not bound to a port, register or transaction. They remain exact in IntentIR and are not rendered.",
            records.len(),
            first.option_id,
            last.option_id
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "preserve_intent_observation".to_string(),
                description: "Keep the optional features in IntentIR and expose this adapter residual without emitting executable syntax.".to_string(),
                downstream_impact: "Independently licensed ISF remains usable; which features a configuration may omit stays visible but unmodelled.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "lower_as_configuration_after_bindings".to_string(),
                description: "Lower an optional feature as a configuration parameter once the feature is bound to the ports, registers or transactions it adds.".to_string(),
                downstream_impact: "A future adapter could emit one configuration per choice after the bindings are source-grounded and FSMGen-strict verified.".to_string(),
            },
        ],
    })
}

/// EXTRACTION-REPRESENTATION-GAPS.3.d.i — one residual, the explicit non-applicable disposition the category contract
/// requires, for every statement about a software agent, named by the range of its consecutively numbered records.
fn software_intent_residual_packet(
    records: &[SoftwareIntentRecord],
) -> Option<ResidualDecisionPacket> {
    let first = records.first()?;
    let last = records.last()?;
    Some(ResidualDecisionPacket {
        packet_id: "isf_software_intent_not_applicable".to_string(),
        question: "Should the document's statements about software agents stay outside the executable hardware intent?".to_string(),
        why_unresolved: format!(
            "{} statement(s) (`{}`-`{}`) say what an agent the document makes software does, must, may or can do (an English software noun, a glossary row that defines the name as software, or a name the document says is implemented in software). Software intent is non-applicable to executable IntentIR: `.isf` describes hardware, so they remain exact in IntentIR and are not rendered.",
            records.len(),
            first.intent_id,
            last.intent_id
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "non_applicable_software_intent".to_string(),
                description: "Keep the software intent in IntentIR as a non-executable disposition and emit no hardware syntax for it.".to_string(),
                downstream_impact: "Independently licensed ISF remains usable; the document's software procedure stays visible and is never mistaken for hardware behavior.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "model_software_as_environment".to_string(),
                description: "Model a software agent as the environment that drives the hardware's registers once its accesses are bound to the hardware's ports and fields.".to_string(),
                downstream_impact: "A future adapter could emit stimulus or a testbench sequence after the bindings are source-grounded and FSMGen-strict verified.".to_string(),
            },
        ],
    })
}

/// EXTRACTION-REPRESENTATION-GAPS.55.a — one residual for the keywords the document defines, naming those whose
/// definition states an obligation on the fields it marks: no field record carries the keyword yet (`.55.b`), so none
/// is lowered.
fn keyword_definition_residual_packet(
    records: &[KeywordDefinitionRecord],
) -> Option<ResidualDecisionPacket> {
    let first = records.first()?;
    let last = records.last()?;
    let obliging = records
        .iter()
        .filter(|record| !record.obligations.is_empty())
        .map(|record| format!("`{}` ({})", record.keyword, record.obligations.len()))
        .collect::<Vec<_>>();
    Some(ResidualDecisionPacket {
        packet_id: "isf_keyword_obligations_not_bound".to_string(),
        question: "Which fields does each obligation a keyword's definition states bind?".to_string(),
        why_unresolved: format!(
            "{} keyword definition(s) (`{}`-`{}`); {} state obligations on what the keyword marks: {}. No field record carries a keyword, so no obligation reaches a field and none is rendered; the definitions remain exact in IntentIR.",
            records.len(),
            first.keyword_id,
            last.keyword_id,
            obliging.len(),
            if obliging.is_empty() { "none".to_string() } else { obliging.join(", ") }
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "keep_keyword_obligations_unbound".to_string(),
                description: "Keep each keyword definition and its obligations in IntentIR and emit no syntax until a field the document marks with the keyword reaches a record.".to_string(),
                downstream_impact: "Independently licensed ISF remains usable; the document-wide rule stays typed and visible.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "bind_keyword_obligations_to_marked_fields".to_string(),
                description: "Bind each obligation to every field the document's field tables mark with the keyword (a reserved field sent as zero and ignored on receipt).".to_string(),
                downstream_impact: "A future adapter could zero reserved fields on transmit and mask them on receive once the fields are captured and FSMGen-strict verified.".to_string(),
            },
        ],
    })
}

fn protocol_residual_packet(
    surface_id: &str,
    surface_label: &str,
    record_id: &str,
    supporting_statement_ids: &[String],
    supporting_table_ids: &[String],
    missing_bindings: &str,
) -> ResidualDecisionPacket {
    let provenance = match (
        supporting_statement_ids.is_empty(),
        supporting_table_ids.is_empty(),
    ) {
        (true, true) => "no supporting statement ids".to_string(),
        (false, true) => format!(
            "supporting statements: {}",
            supporting_statement_ids.join(", ")
        ),
        (true, false) => format!("supporting tables: {}", supporting_table_ids.join(", ")),
        (false, false) => format!(
            "supporting statements: {}; supporting tables: {}",
            supporting_statement_ids.join(", "),
            supporting_table_ids.join(", ")
        ),
    };
    ResidualDecisionPacket {
        packet_id: format!(
            "isf_protocol_{surface_id}_{}",
            crate::ir::isf_ir::sanitize_isf_name(record_id)
        ),
        question: format!(
            "How should {surface_label} record `{record_id}` be represented downstream of `.isf`?"
        ),
        why_unresolved: format!(
            "The record does not carry {missing_bindings}. It remains exact in IntentIR and is not rendered; no missing binding is inferred. Source provenance: {provenance}."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "preserve_intent_observation".to_string(),
                description: "Keep the grounded protocol observation in IntentIR and expose this adapter residual without emitting executable syntax.".to_string(),
                downstream_impact: "Independently licensed ISF remains usable, while the protocol behavior stays visibly incomplete.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "lower_after_bindings".to_string(),
                description: "Lower the record only after upstream typed data supplies every operand required by a supported ISF construct.".to_string(),
                downstream_impact: "A future adapter may emit the behavior after the missing bindings are source-grounded and FSMGen-strict verified.".to_string(),
            },
        ],
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a — the residual for a signal whose absent value the document
/// states. The record carries everything the sentence states; what is missing is the `.isf` clause that
/// could say it (`(absent-value V)`, requested as `sf-isf-optional-signal` R3 and acknowledged by
/// FSMGen), so the packet says that rather than naming a binding the record lacks. `.3.d.ii.b.iii.a`: a value
/// that holds for one class of requests gets its own packet, numbered in the record's order, and says so.
fn absent_value_residual_packet(
    record: &SignalPresenceRecord,
    absent_value: &StatedAbsentValue,
    class_ordinal: Option<usize>,
) -> ResidualDecisionPacket {
    let presence = crate::ir::isf_ir::sanitize_isf_name(&record.presence_id);
    let packet_id = match class_ordinal {
        None => format!("isf_protocol_signal_absent_value_{}", presence),
        Some(ordinal) => format!(
            "isf_protocol_signal_qualified_absent_value_{}_{}",
            presence, ordinal
        ),
    };
    let class = absent_value
        .applies_to
        .as_ref()
        .map(|class| format!(" for {}", class))
        .unwrap_or_default();
    let class_clause = if absent_value.applies_to.is_some() {
        " The value holds for one class of requests only, which `.isf` has no clause to scope either."
    } else {
        ""
    };
    let encoding = absent_value
        .encoding
        .as_ref()
        .map(|encoding| {
            format!(
                "; encoding table: {} (code {})",
                encoding.table_id,
                encoding.codes.join(" or ")
            )
        })
        .unwrap_or_default();
    ResidualDecisionPacket {
        packet_id,
        question: format!(
            "How should the value `{}` that signal `{}` takes when absent{} be represented downstream of `.isf`?",
            absent_value.value, record.signal_name, class
        ),
        why_unresolved: format!(
            "`.isf` has no port clause for the value an absent input reads (`(absent-value V)`, requested as `sf-isf-optional-signal` R3).{} The value remains exact in IntentIR and is not rendered; no tie-off is inferred. Source provenance: supporting statements: {}; supporting tables: {}{}.",
            class_clause,
            absent_value.supporting_statement_ids.join(", "),
            record.table_id,
            encoding
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "preserve_intent_observation".to_string(),
                description: "Keep the stated absent value on the presence record in IntentIR and expose this adapter residual without emitting executable syntax.".to_string(),
                downstream_impact: "Independently licensed ISF remains usable, while an absent input's value stays visibly unlowered.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "lower_after_bindings".to_string(),
                description: "Emit `(absent-value V)` on the port once FSMGen publishes the optional-port clause and the port's presence condition is lowered beside it.".to_string(),
                downstream_impact: "A future adapter may tie the absent input off exactly as the document states, FSMGen-strict verified.".to_string(),
            },
        ],
    }
}

/// Account for every canonical protocol observation at the adapter boundary (ADR 0016).
///
/// None of the current record types supplies every operand required by a supported executable ISF
/// construct. Keep one stable residual packet per input record, in schema and record order, instead
/// of silently dropping observations or fabricating transitions, values, ports, storage, or timing.
/// `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii` — one residual per signal layout. No `.isf` port takes a field: port
/// options are a closed set (FSMGen ladder F29, anticipated). A sub-field reaching past the elaborated width of a port
/// the layout names (the signal, or each member of the collective it is) is listed with that width, so a field that
/// exists only at a wider parameter value is stated rather than dropped (AHB `HPROT[6:4]` at `HPROT_WIDTH` 4).
fn signal_layout_residuals(intent_ir: &IntentIr, isf_model: &IsfIr) -> Vec<ResidualDecisionPacket> {
    intent_ir
        .signal_layouts
        .iter()
        .map(|layout| {
            let ports: Vec<&str> = intent_ir
                .collective_signal_names
                .iter()
                .find(|collective| collective.collective_name == layout.signal)
                .map(|collective| collective.member_names.iter().map(String::as_str).collect())
                .unwrap_or_else(|| vec![layout.signal.as_str()]);
            let mut beyond = Vec::new();
            let mut absent = Vec::new();
            for port in ports {
                let Some((width, param)) = isf_model.signal_width(port) else {
                    absent.push(format!("`{port}`"));
                    continue;
                };
                let past: Vec<String> = layout
                    .sub_fields
                    .iter()
                    .filter(|field| field.msb >= width)
                    .map(|field| format!("{} {}", field.name, field.bits))
                    .collect();
                if !past.is_empty() {
                    let size = match param {
                        Some(param) => format!("`{param}`'s default {width}"),
                        None => format!("its width {width}"),
                    };
                    beyond.push(format!("`{port}` ({size}) leaves {} past it", past.join(", ")));
                }
            }
            let mut missing = "an `.isf` form for a port's sub-fields: port options are a closed set, so no port \
                               takes a field (FSMGen ladder F29, anticipated)"
                .to_string();
            if !beyond.is_empty() {
                missing.push_str(&format!("; {}", beyond.join("; ")));
            }
            if !absent.is_empty() {
                missing.push_str(&format!(
                    "; no port of this actor is named {}",
                    absent.join(" or ")
                ));
            }
            protocol_residual_packet(
                "signal_layout",
                "signal layout",
                &layout.layout_id,
                &[],
                &layout.table_ids,
                &missing,
            )
        })
        .collect()
}

fn protocol_residual_decisions(intent_ir: &IntentIr) -> Vec<ResidualDecisionPacket> {
    // EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a / .3.d.ii.b.iii.a — every stated absent value, the signal's own and
    // each class-scoped one, in record order.
    let stated_absent_values = intent_ir
        .signal_presence_records
        .iter()
        .flat_map(|record| {
            let own = record
                .absent_value
                .iter()
                .map(move |value| (record, value, None));
            let scoped = record
                .qualified_absent_values
                .iter()
                .enumerate()
                .map(move |(index, value)| (record, value, Some(index + 1)));
            own.chain(scoped)
        })
        .collect::<Vec<_>>();
    let mut residuals = Vec::with_capacity(
        intent_ir.serial_frame_fields.len()
            + intent_ir.protocol_operations.len()
            + intent_ir.protocol_states.len()
            + intent_ir.protocol_transitions.len()
            + intent_ir.value_combinations.len()
            + intent_ir.matrix_tables.len()
            + intent_ir.message_signal_layouts.len()
            + intent_ir.interface_edge_timings.len()
            + stated_absent_values.len()
            + intent_ir.equivalence_rules.len()
            + 3,
    );

    for record in &intent_ir.serial_frame_fields {
        residuals.push(protocol_residual_packet(
            "serial_frame_field",
            "serial-frame field",
            &record.field_id,
            &record.supporting_statement_ids,
            &[],
            "an enclosing transaction/operation binding, the declared serial data wire, and complete literal, activation, and storage semantics",
        ));
    }
    for record in &intent_ir.protocol_operations {
        residuals.push(protocol_residual_packet(
            "operation",
            "protocol operation",
            &record.operation_id,
            &record.supporting_statement_ids,
            &[],
            "a typed activation condition, ordered field/step membership, bound ports, and state-transition effects",
        ));
    }
    for record in &intent_ir.protocol_states {
        // EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.iv — a state a value combination keys has its encoding stated.
        let missing = if record.supporting_combination_ids.is_empty() {
            "typed transitions, guards, initial-state identity, encoding, and bound action operands"
                .to_string()
        } else {
            format!(
                "typed transitions, guards, initial-state identity, and bound action operands; its encoding by signal values is stated by {}",
                record.supporting_combination_ids.join(", ")
            )
        };
        residuals.push(protocol_residual_packet(
            "state",
            "protocol state",
            &record.state_id,
            &record.supporting_statement_ids,
            &record.supporting_table_ids,
            &missing,
        ));
    }
    // EXTRACTION-REPRESENTATION-GAPS.3.b.iii.g — `.isf` has no timer whose expiry is a condition
    // (`FSMGEN-REQUESTS.8`), and a record listing several next states does not say which one is taken.
    for record in &intent_ir.protocol_transitions {
        residuals.push(protocol_residual_packet(
            "transition",
            "protocol transition",
            &record.transition_id,
            &[],
            &record.supporting_table_ids,
            "an executable trigger (a timer whose expiry is a condition), an encoding for its states, and, when several next states are listed, the condition choosing one",
        ));
    }
    // EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.ii / .iv — a row keyed by several signals' or fields' values. It is not
    // lowered itself (every `.isf` assignment is clocked, so a decode needs an invented storage target, ADR 0016); a
    // condition naming one of its members would resolve to its keys' values, and none does yet (`.3.g.iii.b.iii`).
    for record in &intent_ir.value_combinations {
        residuals.push(protocol_residual_packet(
            "value_combination",
            "value combination",
            &record.combination_id,
            &[],
            &record.table_ids,
            "a consumer: no condition names one of its members, and `.isf` has no combinational assignment that states the table without an invented storage target (ADR 0016)",
        ));
    }
    // EXTRACTION-REPRESENTATION-GAPS.3.c.v.ii — a two-way table of marks or modal words. Whether a legality matrix
    // becomes an assertion is decided from a consumer (`.3.c.v.iv`); requirement, version and effect matrices describe
    // configuration and stay typed in IR (ADR 0033 §6).
    for record in &intent_ir.matrix_tables {
        residuals.push(protocol_residual_packet(
            "matrix_table",
            "matrix table",
            &record.matrix_id,
            &[],
            &record.table_ids,
            "a consumer: no rule, guard or drive names its row and column keys; a legality matrix's lowering is decided apart (`EXTRACTION-REPRESENTATION-GAPS.3.c.v.iv`), and a requirement, version or effect matrix states configuration the `.isf` does not carry (ADR 0033 §6)",
        ));
    }
    // EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c.ii — a signal laid out per message kind. No `.isf` port takes a per-message
    // bit mapping (FSMGen ladder F29, anticipated); the record names its signals and message kinds and stays typed.
    for record in &intent_ir.message_signal_layouts {
        let mut signals: Vec<&str> = Vec::new();
        for bit in &record.bits {
            if !signals.contains(&bit.signal.as_str()) {
                signals.push(bit.signal.as_str());
            }
        }
        residuals.push(protocol_residual_packet(
            "message_signal_layout",
            "message signal layout",
            &record.layout_id,
            &[],
            &record.table_ids,
            &format!(
                "an `.isf` form for a signal laid out per message kind ({} over {}): port options are a closed set, so \
                 no port takes a per-message bit mapping (FSMGen ladder F29, anticipated)",
                signals.join(", "),
                record.messages.join("; ")
            ),
        ));
    }
    for record in &intent_ir.interface_edge_timings {
        residuals.push(protocol_residual_packet(
            "interface_edge_timing",
            "interface-edge timing",
            &record.timing_id,
            &record.supporting_statement_ids,
            &[],
            "an activation/guard, sample destination, drive value, and transaction or state association",
        ));
    }
    // EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a — a stated absent value has no `.isf` clause yet.
    // EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.ii — `.isf` has no construct that treats one thing as another.
    for record in &intent_ir.equivalence_rules {
        residuals.push(protocol_residual_packet(
            "equivalence_rule",
            "equivalence rule",
            &record.equivalence_id,
            &record.supporting_statement_ids,
            &[],
            "a construct that treats one thing as another under a condition, and a subject and equivalent `.isf` can name (a state, a transaction class, a value)",
        ));
    }
    // EXTRACTION-REPRESENTATION-GAPS.3.d.iii.a — `.isf` has no construct for what a subject may do; one packet names
    // every stated permission, the way `isf_obligations_not_lowered` names every obligation it does not lower.
    residuals.extend(permission_residual_packet(&intent_ir.permission_rules));
    // EXTRACTION-REPRESENTATION-GAPS.3.d.iii.d — nor for a feature that may be absent; one packet names them all.
    residuals.extend(optional_feature_residual_packet(
        &intent_ir.optional_features,
    ));
    // EXTRACTION-REPRESENTATION-GAPS.3.d.i — nor for what a software agent does, which is non-executable by design.
    residuals.extend(software_intent_residual_packet(&intent_ir.software_intent));
    // EXTRACTION-REPRESENTATION-GAPS.55.a — nor for a keyword's obligation, which no field record carries yet.
    residuals.extend(keyword_definition_residual_packet(
        &intent_ir.keyword_definitions,
    ));
    for (record, absent_value, class_ordinal) in stated_absent_values {
        residuals.push(absent_value_residual_packet(
            record,
            absent_value,
            class_ordinal,
        ));
    }

    residuals
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
    // Count emitted content, not recovered symbols, so the artifact metric
    // matches the rendered `.isf` (same doctrine as transaction/rule counts;
    // `render()`'s safe-value filter may exclude some recovered symbols).
    let constant_count = isf_model.emitted_constant_count();
    let enum_count = isf_model.emitted_enum_count();
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
    // Every projected protocol record receives an
    // explicit adapter disposition. The current directly lowerable subset is empty because the
    // record types do not carry every executable binding; per-record residuals keep that omission
    // visible without blocking independently licensed ISF.
    residual_decisions.extend(protocol_residual_decisions(intent_ir));
    // EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii — a signal's sub-field layout has no `.isf` port form.
    residual_decisions.extend(signal_layout_residuals(intent_ir, &isf_model));
    residual_decisions.extend(isf_model.temporal_residuals().iter().cloned());
    // Register resets that could not be lowered to a storage `(reset V)`
    // (ISF-REGISTER-RESET-EMIT.2) are surfaced the same way, so an un-lowered reset is
    // visible in the artifact rather than silently dropped; no value is fabricated.
    residual_decisions.extend(isf_model.storage_reset_residuals().iter().cloned());
    // Register bit-fields that could not be lowered to the storage `(fields …)` block
    // (DOC-INTENT-TAXONOMY.4a.ii) are surfaced the same way, so the largest measurable
    // intent-loss is visible in the artifact rather than silently dropped — the full field map
    // always remains in IntentIR register_records; no bit position is fabricated.
    residual_decisions.extend(isf_model.storage_field_residuals().iter().cloned());
    // Enums held out of the `.isf` by the member-value-width gate (KG-ISF-COMPLETENESS.2a.iv)
    // — a mega-conflated / over-width enum whose member literal FSMGen would reject — are surfaced
    // the same way, so the dropped enum surface is explicit rather than silently lost; no radix is
    // fabricated and no value is truncated.
    residual_decisions.extend(isf_model.enum_residuals().iter().cloned());
    // Signal widths the document writes symbolically (KG-ISF-COMPLETENESS.2a.vii.b): a placeholder
    // parameter default, and a width no actor parameter can state, are surfaced the same way — the
    // placeholder is never presented as the document's value, and a derived width is never computed.
    residual_decisions.extend(isf_model.signal_width_residuals().iter().cloned());
    // EXTRACTION-REPRESENTATION-GAPS.27.e — interface signals the lowered actor does not declare.
    residual_decisions.extend(isf_model.interface_residuals().iter().cloned());

    let artifact_layout = AdapterArtifactLayout {
        artifact_root,
        adapter_artifact_path,
        emitted_target_path,
    }
    .runtime_layout()?;

    Ok(AdapterArtifact {
        stage: IrStage::IsfAdapter,
        schema_version: ADAPTER_ARTIFACT_SCHEMA_VERSION,
        target: AdapterTarget::Isf,
        required_input_stage: IrStage::IntentIr,
        intent_ir_path: intent_ir_path.to_path_buf(),
        artifact_layout,
        adapter_identity,
        document_identity: intent_ir.document_identity.clone(),
        lowering_status,
        residual_decisions,
        validation_reports: vec![],
        isf: Some(isf),
        proof_context: None,
        proof_ledger: None,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::adapters::{
        AdapterArtifact, AdapterLoweringStatus, AdapterMutationKind, AdapterTarget,
    };
    use crate::ir::evidence::{
        EquivalenceModality, EquivalenceRuleRecord, EvidenceIr, InterfaceClockEdge,
        InterfaceEdgeTimingRecord, ParticipantDriveRecord, ProtocolOperationRecord,
        ProtocolStateRecord, SerialFrameField, SignalPresenceRecord, StatedAbsentValue,
        StatedValueEncoding, VariantPresenceEntry,
    };
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
        source_ir.write_test_fixture_to_disk()?;
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

    #[test]
    fn downstream_artifacts_store_relative_paths_and_rebase_legacy_lineage() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "portable_downstream_paths.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let semantic_ir_path = intent_ir.semantic_ir_path.clone();
        let semantic_ir = SemanticIr::load_from_path(&semantic_ir_path)?;
        let evidence_ir_path = semantic_ir.evidence_ir_path.clone();
        let intent_ir_path = intent_ir.artifact_layout.intent_ir_path.clone();
        let adapter = AdapterArtifact::build(
            &intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("generated/adapters"),
        )?;
        adapter.write_to_disk()?;
        let adapter_path = adapter.artifact_layout.adapter_artifact_path.clone();

        let repository = crate::project_data::repository_root()?;
        for path in [&semantic_ir_path, &intent_ir_path, &adapter_path] {
            let stored = fs::read_to_string(path)?;
            assert!(
                !stored.contains(repository.to_string_lossy().as_ref()),
                "repository-owned downstream paths must serialize without the runtime root: {}",
                path.display()
            );
        }

        fn retire(value: &mut serde_json::Value) {
            let relative = value.as_str().expect("stored path string");
            assert!(Path::new(relative).is_relative());
            *value = serde_json::Value::String(
                Path::new("/retired/specforge")
                    .join(relative)
                    .to_string_lossy()
                    .into_owned(),
            );
        }

        let mut semantic_json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&semantic_ir_path)?)?;
        for pointer in [
            "/evidence_ir_path",
            "/artifact_layout/artifact_root",
            "/artifact_layout/semantic_ir_path",
        ] {
            retire(semantic_json.pointer_mut(pointer).expect("semantic path"));
        }
        fs::write(
            &semantic_ir_path,
            serde_json::to_string_pretty(&semantic_json)?,
        )?;

        let mut intent_json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&intent_ir_path)?)?;
        for pointer in [
            "/semantic_ir_path",
            "/artifact_layout/artifact_root",
            "/artifact_layout/intent_ir_path",
        ] {
            retire(intent_json.pointer_mut(pointer).expect("intent path"));
        }
        fs::write(&intent_ir_path, serde_json::to_string_pretty(&intent_json)?)?;

        let mut adapter_json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&adapter_path)?)?;
        for pointer in [
            "/intent_ir_path",
            "/artifact_layout/artifact_root",
            "/artifact_layout/adapter_artifact_path",
            "/artifact_layout/emitted_target_path",
        ] {
            retire(adapter_json.pointer_mut(pointer).expect("adapter path"));
        }
        fs::write(&adapter_path, serde_json::to_string_pretty(&adapter_json)?)?;

        let reloaded_semantic = SemanticIr::load_from_path(&semantic_ir_path)?;
        assert_eq!(reloaded_semantic.evidence_ir_path, evidence_ir_path);
        assert!(
            reloaded_semantic
                .artifact_layout
                .artifact_root
                .is_absolute()
        );
        assert!(
            !reloaded_semantic
                .to_pretty_json()?
                .contains("/retired/specforge")
        );

        let reloaded_intent = IntentIr::load_from_path(&intent_ir_path)?;
        assert_eq!(reloaded_intent.semantic_ir_path, semantic_ir_path);
        assert!(reloaded_intent.artifact_layout.artifact_root.is_absolute());
        assert!(
            !reloaded_intent
                .to_pretty_json()?
                .contains("/retired/specforge")
        );

        let reloaded_adapter = AdapterArtifact::load_from_path(&adapter_path)?;
        assert_eq!(reloaded_adapter.intent_ir_path, intent_ir_path);
        assert!(reloaded_adapter.artifact_layout.artifact_root.is_absolute());
        assert!(
            reloaded_adapter
                .artifact_layout
                .emitted_target_path
                .as_ref()
                .is_some_and(|path| path.is_absolute())
        );
        assert!(
            !reloaded_adapter
                .to_pretty_json()?
                .contains("/retired/specforge")
        );

        Ok(())
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

    #[test]
    fn isf_adapter_preserves_typed_clock_reset_semantics_under_alpha_renaming() -> Result<()> {
        let tempdir = tempdir()?;
        let conventional_root = tempdir.path().join("conventional");
        let renamed_root = tempdir.path().join("renamed");
        fs::create_dir_all(&conventional_root)?;
        fs::create_dir_all(&renamed_root)?;

        let conventional = build_intent_ir_from_markdown(
            &conventional_root,
            "contract.md",
            concat!(
                "# Contract\n",
                "Clock clk.\n\n",
                "Reset rst_n is asynchronous active low.\n\n",
                "Signal DATA is output width 1.\n\n",
                "Block emit: DATA <- 1.\n",
            ),
        )?;
        let renamed = build_intent_ir_from_markdown(
            &renamed_root,
            "contract.md",
            concat!(
                "# Contract\n",
                "Clock orbit.\n\n",
                "Reset clear is asynchronous active low.\n\n",
                "Signal DATA is output width 1.\n\n",
                "Block emit: DATA <- 1.\n",
            ),
        )?;

        let conventional_artifact = AdapterArtifact::build(
            &conventional.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &conventional_root.join("adapter"),
        )?;
        let renamed_artifact = AdapterArtifact::build(
            &renamed.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &renamed_root.join("adapter"),
        )?;
        let conventional_isf = conventional_artifact.isf.expect("conventional ISF");
        let renamed_isf = renamed_artifact.isf.expect("renamed ISF");

        assert!(conventional_isf.is_renderable);
        assert!(renamed_isf.is_renderable);
        assert_eq!(
            conventional_isf
                .source_text
                .replace("rst_n", "clear")
                .replace("clk", "orbit"),
            renamed_isf.source_text,
            "alpha-renaming may change identities but not reset timing, polarity, or lowering"
        );

        Ok(())
    }

    #[test]
    fn isf_adapter_does_not_infer_clock_or_reset_contracts_from_identifier_spelling() -> Result<()>
    {
        let tempdir = tempdir()?;
        let conventional_root = tempdir.path().join("conventional");
        let renamed_root = tempdir.path().join("renamed");
        fs::create_dir_all(&conventional_root)?;
        fs::create_dir_all(&renamed_root)?;

        let conventional = build_intent_ir_from_markdown(
            &conventional_root,
            "unresolved.md",
            concat!(
                "# Unresolved Contract\n",
                "Signal clk is input width 1.\n\n",
                "Signal rst_n is input width 1.\n\n",
                "Signal DATA is output width 1.\n\n",
                "Block emit: DATA <- 1.\n",
            ),
        )?;
        let renamed = build_intent_ir_from_markdown(
            &renamed_root,
            "unresolved.md",
            concat!(
                "# Unresolved Contract\n",
                "Signal orbit is input width 1.\n\n",
                "Signal clear is input width 1.\n\n",
                "Signal DATA is output width 1.\n\n",
                "Block emit: DATA <- 1.\n",
            ),
        )?;
        assert!(conventional.system_contract.is_none());
        assert!(renamed.system_contract.is_none());

        let conventional_artifact = AdapterArtifact::build(
            &conventional.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &conventional_root.join("adapter"),
        )?;
        let renamed_artifact = AdapterArtifact::build(
            &renamed.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &renamed_root.join("adapter"),
        )?;
        let conventional_isf = conventional_artifact
            .isf
            .expect("conventional ISF diagnostics");
        let renamed_isf = renamed_artifact.isf.expect("renamed ISF diagnostics");

        for isf in [&conventional_isf, &renamed_isf] {
            assert!(!isf.is_renderable);
            assert_eq!(
                isf.blocking_reasons,
                vec!["no source-grounded system clock/reset contract".to_string()]
            );
            assert!(
                isf.source_text
                    .contains("(clock __specforge_unresolved_clock)")
            );
            assert!(
                isf.source_text
                    .contains("(reset (__specforge_unresolved_reset unknown unknown))")
            );
        }
        assert!(
            conventional_artifact
                .artifact_layout
                .emitted_target_path
                .is_none()
        );
        assert!(
            renamed_artifact
                .artifact_layout
                .emitted_target_path
                .is_none()
        );

        Ok(())
    }

    #[test]
    fn a_guard_on_the_contract_reset_survives_the_reset_leaving_the_interface() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.7.b — AXI *SYSCOREQ and SYSCOACK must be deasserted when ARESETn is
        // asserted*, in the shape its IntentIR carries. A contract takes the reset out of the interface; the
        // guard must still name it, at the level its stated polarity gives, and never vanish into an
        // unconditional rule. Nothing may drive the reset.
        use crate::ir::isf_ir::IsfIr;
        use crate::ir::semantic::{
            ClockEdge, TemporalPredicateRecord, TemporalRuleRecord, TickPhase,
        };
        use crate::ir::source::AutomationConfidence;
        let value = |signal: &str, value: &str| TemporalPredicateRecord::SignalValue {
            signal_name: signal.to_string(),
            value: value.to_string(),
            phase: TickPhase::PostTick,
        };
        let rule = |id: &str, antecedent, consequent| TemporalRuleRecord {
            rule_id: id.to_string(),
            clock_signal: Some("clk".to_string()),
            edge: ClockEdge::Rising,
            antecedents: vec![antecedent],
            exceptions: Vec::new(),
            consequents: vec![consequent],
            cycle_window: None,
            source_text: format!("source for {id}"),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        let tempdir = tempdir()?;
        let isf_of = |reset_line: &str| -> Result<String> {
            let root = tempdir.path().join(reset_line.len().to_string());
            fs::create_dir_all(&root)?;
            let mut intent_ir = build_intent_ir_from_markdown(
                &root,
                "reset_guard.md",
                &format!(
                    "# Reset Guard\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
                     Signal REQ is output width 1.\n\nSignal ACK is output width 1.\n\nClock clk.\n\n\
                     {reset_line}\n\nBlock emit: ACK <- 1.\n"
                ),
            )?;
            intent_ir.actor_contracts.clear();
            // As in AXI, no polarity record names the reset: its polarity comes from the contract alone.
            intent_ir.signal_polarities.clear();
            intent_ir.temporal_rules = vec![
                rule(
                    "req_in_reset",
                    value("rst", "ASSERTED"),
                    value("REQ", "LOW"),
                ),
                rule("drives_reset", value("REQ", "HIGH"), value("rst", "LOW")),
            ];
            let model = IsfIr::from_intent_ir(&intent_ir, "reset_guard");
            let refused = model
                .temporal_residuals()
                .iter()
                .any(|packet| packet.packet_id == "isf_temporal_unrepresentable_drives_reset");
            assert!(
                refused,
                "a rule that drives the reset is listed, not emitted"
            );
            Ok(model.render())
        };

        let low = isf_of("Reset rst is asynchronous active low.")?;
        assert!(low.contains("(reset (rst async active_low))"), "{low}");
        assert!(
            low.contains("(rule temporal_req_in_reset (== rst 0)\n    (REQ 0)"),
            "{low}"
        );
        assert!(!low.contains("drives_reset"), "{low}");
        let high = isf_of("Reset rst is asynchronous active high.")?;
        assert!(
            high.contains("(rule temporal_req_in_reset (== rst 1)\n    (REQ 0)"),
            "{high}"
        );
        // An unstated polarity refuses the rule rather than dropping its guard.
        let unknown = isf_of("Reset rst is asynchronous.")?;
        assert!(!unknown.contains("temporal_req_in_reset"), "{unknown}");
        Ok(())
    }

    #[test]
    fn a_temporal_rule_whose_stated_condition_was_not_read_is_listed_not_emitted() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.25.a — AXI *ACTIVATEREQ, ACTIVATEACK and ASKSTOP must be LOW during reset*:
        // its temporal rule has no antecedent (`reset` names no signal). The IR keeps it; lowered with no guard it
        // drove ASKSTOP 0 always. The constraint loop still lowers the constraint guarded on the reset.
        use crate::ir::isf_ir::IsfIr;
        use crate::ir::semantic::{
            ClockEdge, TemporalPredicateRecord, TemporalRuleRecord, TickPhase,
        };
        use crate::ir::source::{
            AutomationConfidence, SignalConstraintKind, SignalConstraintRecord,
        };
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "unread_condition.md",
            "# Unread Condition\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
             Signal ASKSTOP is output width 1.\n\nSignal ACK is output width 1.\n\nClock clk.\n\n\
             Reset rst is asynchronous active low.\n\nBlock emit: ACK <- 1.\n",
        )?;
        intent_ir.actor_contracts.clear();
        intent_ir.signal_polarities.clear();
        intent_ir.temporal_invariants.clear();
        intent_ir.signal_constraints = vec![SignalConstraintRecord {
            constraint_id: "sigcon_reset".to_string(),
            subject_signal: "ASKSTOP".to_string(),
            constraint_kind: SignalConstraintKind::MustBeLow,
            target_value: None,
            condition_text: Some("reset".to_string()),
            negated: false,
            source_text: "ASKSTOP must be LOW during reset.".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }];
        intent_ir.temporal_rules = vec![TemporalRuleRecord {
            rule_id: "temporal_signal_constraint_sigcon_reset".to_string(),
            clock_signal: Some("clk".to_string()),
            edge: ClockEdge::Rising,
            antecedents: vec![],
            exceptions: Vec::new(),
            consequents: vec![TemporalPredicateRecord::SignalValue {
                signal_name: "ASKSTOP".to_string(),
                value: "LOW".to_string(),
                phase: TickPhase::PostTick,
            }],
            cycle_window: None,
            source_text: "ASKSTOP must be LOW during reset.".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }];
        let model = IsfIr::from_intent_ir(&intent_ir, "unread_condition");
        let text = model.render();
        assert!(
            !text.contains("temporal_temporal_signal_constraint_sigcon_reset"),
            "{text}"
        );
        assert!(
            text.contains("(rule constraint_0 (== rst 0)\n    (ASKSTOP 0)"),
            "{text}"
        );
        assert!(model.temporal_residuals().iter().any(|packet| {
            packet.packet_id
                == "isf_temporal_unrepresentable_temporal_signal_constraint_sigcon_reset"
                && packet
                    .why_unresolved
                    .contains("stated condition `reset` was not read")
        }));
        Ok(())
    }

    #[test]
    fn a_rule_keeps_its_exception_as_a_negated_guard_or_is_listed() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.26 — AHB *… when HREADY is LOW, unless HRESP signal is ERROR*, renamed. The
        // exception lowers as the negation of its level; one whose value is no level refuses the rule rather than
        // being dropped; and a conditional rule whose condition was not read is listed, as a constraint's is.
        use crate::ir::isf_ir::IsfIr;
        use crate::ir::semantic::{
            ClockEdge, TemporalPredicateRecord, TemporalRuleRecord, TickPhase,
        };
        use crate::ir::source::{AutomationConfidence, ConditionalRuleRecord};
        let value = |signal: &str, value: &str, phase| TemporalPredicateRecord::SignalValue {
            signal_name: signal.to_string(),
            value: value.to_string(),
            phase,
        };
        let rule = |id: &str, antecedents, exceptions| TemporalRuleRecord {
            rule_id: id.to_string(),
            clock_signal: Some("clk".to_string()),
            edge: ClockEdge::Rising,
            antecedents,
            exceptions,
            consequents: vec![value("HOLD", "LOW", TickPhase::PostTick)],
            cycle_window: None,
            source_text: format!("source for {id}"),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "exception_guard.md",
            "# Exception Guard\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
             Signal READY is input width 1.\n\nSignal RESP is input width 1.\n\n\
             Signal OUT is output width 1.\n\nSignal HOLD is output width 1.\n\nClock clk.\n\nReset rst is asynchronous active low.\n\n\
             Block emit: OUT <- 1.\n",
        )?;
        intent_ir.actor_contracts.clear();
        intent_ir.signal_polarities.clear();
        intent_ir.temporal_invariants.clear();
        intent_ir.conditional_rules = vec![ConditionalRuleRecord {
            rule_id: "condrule_retry".to_string(),
            antecedent_text: "the transfer is retried".to_string(),
            consequent_signal: Some("HOLD".to_string()),
            consequent_action: "must be LOW".to_string(),
            source_text: "HOLD must be LOW when the transfer is retried.".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }];
        let ready_low = || vec![value("READY", "LOW", TickPhase::PreTick)];
        intent_ir.temporal_rules = vec![
            rule(
                "exc_level",
                ready_low(),
                vec![value("RESP", "HIGH", TickPhase::PreTick)],
            ),
            rule(
                "exc_word",
                ready_low(),
                vec![value("RESP", "ERROR", TickPhase::PreTick)],
            ),
            rule("temporal_conditional_rule_condrule_retry", vec![], vec![]),
        ];
        let model = IsfIr::from_intent_ir(&intent_ir, "exception_guard");
        let text = model.render();
        assert!(
            text.contains(
                "(rule temporal_exc_level (& (== READY 0) (! (== RESP 1)))\n    (HOLD 0)"
            ),
            "{text}"
        );
        let listed = |id: &str, reason: &str| {
            model.temporal_residuals().iter().any(|packet| {
                packet.packet_id == format!("isf_temporal_unrepresentable_{id}")
                    && packet.why_unresolved.contains(reason)
            })
        };
        assert!(!text.contains("temporal_exc_word"), "{text}");
        assert!(listed("exc_word", "`RESP ERROR`"), "{text}");
        assert!(!text.contains("condrule_retry"), "{text}");
        assert!(
            listed(
                "temporal_conditional_rule_condrule_retry",
                "stated condition `the transfer is retried` was not read"
            ),
            "{text}"
        );
        Ok(())
    }

    #[test]
    fn the_initiator_is_chosen_without_the_system_clock_and_reset() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.32 — Wishbone's MASTER, renamed: two outputs and one input of its own, plus the
        // system clock and reset every actor reads. Counted with them it ties and no initiator is chosen.
        use crate::ir::semantic::{ActorPortRecord, ActorRelativeDirection};
        use crate::ir::source::AutomationConfidence;
        let tempdir = tempdir()?;
        let mut intent_ir =
            build_intent_ir_from_markdown(tempdir.path(), "isf_spec.md", ISF_ADAPTER_TEST_SPEC)?;
        let port = |actor: &str, signal: &str, direction| ActorPortRecord {
            actor_id: actor.to_string(),
            actor_name: actor.to_string(),
            signal_name: signal.to_string(),
            direction,
            relation_basis: vec![],
            width_hint: None,
            source_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        let contract = intent_ir
            .system_contract
            .clone()
            .expect("the spec states a clock and reset");
        intent_ir.actor_ports = vec![
            port("XINIT", "XADR_O", ActorRelativeDirection::Output),
            port("XINIT", "XSTB_O", ActorRelativeDirection::Output),
            port("XINIT", "XACK_I", ActorRelativeDirection::Input),
            port(
                "XINIT",
                &contract.clock_signal,
                ActorRelativeDirection::Input,
            ),
            port(
                "XINIT",
                &contract.reset_signal,
                ActorRelativeDirection::Input,
            ),
        ];
        assert_eq!(
            crate::ir::isf_ir::select_initiator_actor(&intent_ir.actor_ports),
            None
        );
        assert_eq!(
            crate::ir::isf_ir::select_initiator_for(&intent_ir).as_deref(),
            Some("XINIT")
        );
        Ok(())
    }

    #[test]
    fn an_unstated_reset_kind_is_chosen_to_keep_the_polarity_and_listed() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.7.d — APB-E's *PRESETn is the reset signal and is active-LOW*, renamed: the
        // document states no kind, FSMGen's reset clause needs one, and at the pin only `async` keeps active-low
        // (and `sync` active-high, EXTRACTION-REPRESENTATION-GAPS.21). The choice is emitted and listed as one.
        use crate::ir::isf_ir::IsfIr;
        for (polarity, clause) in [
            ("low", "(reset (rst async active_low))"),
            ("high", "(reset (rst sync active_high))"),
        ] {
            let tempdir = tempdir()?;
            let intent_ir = build_intent_ir_from_markdown(
                tempdir.path(),
                "reset.md",
                &format!(
                    "# Reset\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
                     Signal OUT is output width 1.\n\nClock clk.\n\nReset rst is active {polarity}.\n\n\
                     Block emit: OUT <- 1.\n"
                ),
            )?;
            let model = IsfIr::from_intent_ir(&intent_ir, "reset");
            let text = model.render();
            assert!(text.contains(clause), "{text}");
            assert!(
                model.interface_residuals().iter().any(|packet| {
                    packet.packet_id == "isf_reset_kind_not_stated"
                        && packet.why_unresolved.contains("implementation choice")
                }),
                "{text}"
            );
        }
        Ok(())
    }

    #[test]
    fn a_condition_naming_an_operation_kind_is_the_level_that_states_it() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.10.g.ii — APB's *PWRITE indicates an APB write access when HIGH and an APB read
        // access when LOW* and *For read transfers, … PSTRB LOW*, renamed: the obligation lowers under `XWRITE` LOW.
        use crate::ir::isf_ir::IsfIr;
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "kind.md",
            "# Kind\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
             Signal XWRITE is output width 1.\n\nSignal XSTRB is output width 1.\n\nClock clk.\n\n\
             Reset rst is asynchronous active low.\n\n\
             XWRITE indicates a write access when HIGH and a read access when LOW.\n\n\
             For read transfers, XSTRB must be LOW.\n\nBlock emit: XWRITE <- 1.\n",
        )?;
        assert_eq!(
            intent_ir.operation_kinds.len(),
            2,
            "{:?}",
            intent_ir.operation_kinds
        );
        let text = IsfIr::from_intent_ir(&intent_ir, "kind").render();
        // Every reader reads it alike: the constraint's lowering, its invariant and its temporal rule.
        for rule in [
            "constraint_0",
            "tinv_sc_sigcon_0002",
            "temporal_temporal_signal_constraint_sigcon_0002",
        ] {
            assert!(
                text.contains(&format!("(rule {rule} (== XWRITE 0)\n    (XSTRB 0)")),
                "{rule}: {text}"
            );
        }
        Ok(())
    }

    #[test]
    fn a_family_the_actor_drives_is_declared_once_and_listed() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.24.b — APB's *The Requester generates a PSELx signal for each Completer*,
        // renamed: the driving actor declares one instance and lists the family; a family it only reads is no
        // residual (a reader has its one instance).
        use crate::ir::isf_ir::IsfIr;
        use crate::ir::semantic::SignalFamilyRecord;
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "family.md",
            "# Family\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
             Signal XSEL is output width 1.\n\nSignal XACK is input width 1.\n\nClock clk.\n\n\
             Reset rst is asynchronous active low.\n\nBlock emit: XSEL <- 1.\n",
        )?;
        let family = |signal: &str| SignalFamilyRecord {
            signal_name: signal.to_string(),
            per_actor: "XTARGET".to_string(),
            supporting_statement_ids: vec!["statement_0168".to_string()],
        };
        let listed = |intent_ir: &crate::ir::intent::IntentIr| {
            let model = IsfIr::from_intent_ir(intent_ir, "family");
            let text = model.render();
            assert!(text.contains("(output XSEL (width 1))"), "{text}");
            model
                .interface_residuals()
                .iter()
                .find(|packet| packet.packet_id == "isf_family_driven_once")
                .map(|packet| packet.why_unresolved.clone())
        };
        intent_ir.signal_families = vec![family("XACK")];
        assert_eq!(listed(&intent_ir), None);
        intent_ir.signal_families = vec![family("XSEL"), family("XACK")];
        let why = listed(&intent_ir).expect("the driven family is listed");
        assert!(
            why.contains("`XSEL` (one per XTARGET; statement_0168)"),
            "{why}"
        );
        assert!(!why.contains("XACK"), "{why}");
        Ok(())
    }

    #[test]
    fn a_hold_is_lowered_with_its_release_or_listed() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.25.c — AXI *If ASKSTOP is asserted, it must remain HIGH until ACTIVATEACK is
        // LOW* and *When AWAKEUP is asserted with SYSCOREQ asserted and SYSCOACK deasserted, it must remain asserted
        // until SYSCOACK is asserted*, renamed. A hold persists until its release, so its rule is guarded on the
        // release's negation; a release the condition already excludes adds nothing; a release that cannot be read
        // leaves a hold that would never release, which is listed.
        use crate::ir::isf_ir::IsfIr;
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "hold.md",
            "# Hold\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
             Signal XACK is input width 1.\n\nSignal XSTOP is output width 1.\n\n\
             Signal XWAKE is output width 1.\n\nSignal XHOLD is output width 1.\n\n\
             Signal XVALID is input width 1.\n\nSignal XREADY is input width 1.\n\nClock clk.\n\n\
             Reset rst is asynchronous active low.\n\n\
             If XSTOP is HIGH, it must remain HIGH until XACK is LOW.\n\n\
             When XWAKE is HIGH and XACK is LOW, XWAKE must remain HIGH until XACK is HIGH.\n\n\
             If XHOLD is HIGH, it must remain HIGH until the associated XVALID / XREADY handshake of the ZETA \
             transaction.\n\n\
             Block emit: XHOLD <- 0.\n",
        )?;
        let model = IsfIr::from_intent_ir(&intent_ir, "hold");
        let text = model.render();
        let listed = |needle: &str| {
            model
                .temporal_residuals()
                .iter()
                .any(|packet| packet.why_unresolved.contains(needle))
        };
        assert!(
            text.contains("(& (== XSTOP 1) (! (== XACK 0)))\n    (XSTOP 1)"),
            "{text}"
        );
        assert!(
            text.contains("(& (== XWAKE 1) (== XACK 0))\n    (XWAKE 1)"),
            "{text}"
        );
        assert!(!text.contains("(XHOLD 1)"), "{text}");
        assert!(
            listed(
                "the hold's release `the associated XVALID / XREADY handshake of the ZETA transaction` was not read"
            ),
            "{text}"
        );
        assert!(listed("is a hold (*must remain … until …*)"), "{text}");
        Ok(())
    }

    #[test]
    fn a_temporal_rule_on_the_actors_own_input_is_listed_not_driven() -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.10.a — the temporal path: a rule whose drive target the actor declares as
        // an input is another actor's obligation, listed rather than emitted as this actor's `(rule …)`.
        use crate::ir::isf_ir::IsfIr;
        use crate::ir::semantic::{
            ClockEdge, TemporalPredicateRecord, TemporalRuleRecord, TickPhase,
        };
        use crate::ir::source::AutomationConfidence;
        let value = |signal: &str, value: &str, phase| TemporalPredicateRecord::SignalValue {
            signal_name: signal.to_string(),
            value: value.to_string(),
            phase,
        };
        let rule = |id: &str, target: &str| TemporalRuleRecord {
            rule_id: id.to_string(),
            clock_signal: Some("clk".to_string()),
            edge: ClockEdge::Rising,
            antecedents: vec![value("GO", "HIGH", TickPhase::PreTick)],
            exceptions: Vec::new(),
            consequents: vec![value(target, "LOW", TickPhase::PostTick)],
            cycle_window: None,
            source_text: format!("source for {id}"),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "input_rule.md",
            "# Input Rule\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
             Signal GO is input width 1.\n\nSignal XSTOP is input width 1.\n\n\
             Signal HOLD is output width 1.\n\nSignal OUT is output width 1.\n\nClock clk.\n\n\
             Reset rst is asynchronous active low.\n\nBlock emit: OUT <- 1.\n",
        )?;
        intent_ir.actor_contracts.clear();
        intent_ir.signal_polarities.clear();
        intent_ir.temporal_invariants.clear();
        intent_ir.temporal_rules = vec![rule("on_output", "HOLD"), rule("on_input", "XSTOP")];
        let model = IsfIr::from_intent_ir(&intent_ir, "input_rule");
        let text = model.render();
        assert!(
            text.contains("(rule temporal_on_output (== GO 1)\n    (HOLD 0)"),
            "{text}"
        );
        assert!(!text.contains("temporal_on_input"), "{text}");
        assert!(model.temporal_residuals().iter().any(|packet| {
            packet.packet_id == "isf_temporal_unrepresentable_on_input"
                && packet
                    .why_unresolved
                    .contains("'XSTOP', an input of the actor")
        }));
        Ok(())
    }

    #[test]
    fn a_port_the_document_does_not_ground_on_the_lowered_actor_is_listed_not_declared()
    -> Result<()> {
        // EXTRACTION-REPRESENTATION-GAPS.27.e — AHB's Manager `.isf` declared `HSELx`, which the address decoder
        // drives, as a Manager output with a drive. A port is declared only with a direction grounded on the lowered
        // actor, or, when no actor is related to it at all, the direction its own declaration states.
        use crate::ir::isf_ir::IsfIr;
        use crate::ir::semantic::{ActorPortRecord, ActorRelativeDirection};
        use crate::ir::source::AutomationConfidence;
        let port = |actor: &str, signal: &str, direction| ActorPortRecord {
            actor_id: actor.to_string(),
            actor_name: actor.to_string(),
            signal_name: signal.to_string(),
            direction,
            relation_basis: vec![],
            width_hint: None,
            source_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "grounded_ports.md",
            "# Grounded Ports\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\n\
             Signal XREQ is output width 1.\n\nSignal XDATA is output width 1.\n\n\
             Signal XRESP is input width 1.\n\nSignal XSEL is output width 1.\n\n\
             Signal XTIED is input width 1.\n\nSignal XLOOSE is width 1.\n\nClock clk.\n\n\
             Reset rst is asynchronous active low.\n\nBlock emit: XREQ <- 1.\n",
        )?;
        intent_ir.actor_ports = vec![
            port("Requester", "XREQ", ActorRelativeDirection::Output),
            port("Requester", "XDATA", ActorRelativeDirection::Output),
            port("Requester", "XRESP", ActorRelativeDirection::Input),
            port("decoder", "XSEL", ActorRelativeDirection::Output),
        ];
        let model = IsfIr::from_intent_ir(&intent_ir, "grounded_ports");
        let text = model.render();
        for declared in [
            "(output XREQ ",
            "(output XDATA ",
            "(input XRESP ",
            "(input XTIED ",
        ] {
            assert!(text.contains(declared), "{declared}: {text}");
        }
        assert!(!text.contains("XSEL"), "{text}");
        assert!(!text.contains("XLOOSE"), "{text}");
        let packet = model
            .interface_residuals()
            .iter()
            .find(|packet| packet.packet_id == "isf_ports_not_on_the_lowered_actor")
            .expect("the left-out ports are listed");
        assert!(
            packet
                .why_unresolved
                .contains("the document relates it only to other actors: XSEL"),
            "{}",
            packet.why_unresolved
        );
        assert!(
            packet.why_unresolved.contains(
                "the document relates it to no actor, and its declaration states no direction: XLOOSE"
            ),
            "{}",
            packet.why_unresolved
        );
        Ok(())
    }

    #[test]
    fn adapter_write_reconciles_obsolete_isf_files() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_reconciliation.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;
        artifact.write_to_disk()?;

        let artifact_root = &artifact.artifact_layout.artifact_root;
        let current_isf = artifact
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter has an emitted path");
        let obsolete_isf = artifact_root.join("obsolete_actor.isf");
        let unrelated_file = artifact_root.join("review-notes.txt");
        let unrelated_directory = artifact_root.join("fixture.isf");
        fs::write(&obsolete_isf, "(actor obsolete_actor)")?;
        fs::write(&unrelated_file, "preserve me")?;
        fs::create_dir(&unrelated_directory)?;

        #[cfg(unix)]
        let obsolete_symlink = {
            use std::os::unix::fs::symlink;

            let path = artifact_root.join("obsolete_link.isf");
            symlink(&unrelated_file, &path)?;
            Some(path)
        };
        #[cfg(not(unix))]
        let obsolete_symlink: Option<std::path::PathBuf> = None;

        artifact.write_to_disk()?;

        assert!(
            current_isf.is_file(),
            "the manifest-selected ISF must remain"
        );
        assert!(
            !obsolete_isf.exists(),
            "obsolete regular ISF must be removed"
        );
        if let Some(path) = obsolete_symlink {
            assert!(
                fs::symlink_metadata(path).is_err(),
                "obsolete ISF symlink must be removed without following it"
            );
        }
        assert_eq!(fs::read_to_string(&unrelated_file)?, "preserve me");
        assert!(unrelated_directory.is_dir());

        let mut blocked = artifact.clone();
        blocked.lowering_status = AdapterLoweringStatus::Blocked;
        blocked.artifact_layout.emitted_target_path = None;
        let blocked_isf = blocked.isf.as_mut().expect("ISF metadata remains explicit");
        blocked_isf.is_renderable = false;
        blocked_isf
            .blocking_reasons
            .push("test-only blocked transition".to_string());
        blocked.write_test_fixture_to_disk()?;

        assert!(
            !current_isf.exists(),
            "a blocked manifest must leave no formerly emitted ISF"
        );
        assert_eq!(fs::read_to_string(unrelated_file)?, "preserve me");
        assert!(unrelated_directory.is_dir());

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

    /// `EXTRACTION-REPRESENTATION-GAPS.3.c.v.ii` — a matrix table reaches the adapter as one residual citing its tables,
    /// and the rendered `.isf` is unchanged.
    #[test]
    fn isf_adapter_residualizes_a_matrix_table_citing_its_tables() -> Result<()> {
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_matrix_tables.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let baseline = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("baseline_adapter"),
        )?;
        let baseline_isf = baseline.isf.expect("baseline ISF artifact");
        intent_ir.matrix_tables = vec![crate::ir::evidence::MatrixTableRecord {
            matrix_id: "matrix_table_0001".to_string(),
            table_ids: vec!["table_0174".to_string(), "table_0175".to_string()],
            caption: Some("Table A12.11: Legal tag operations for read transactions".to_string()),
            row_key_titles: vec!["Opcode".to_string()],
            column_keys: vec![vec![
                "MTE_Support = Basic".to_string(),
                "Transfer".to_string(),
            ]],
            rows: vec![crate::ir::evidence::MatrixRow {
                table_id: "table_0174".to_string(),
                keys: vec!["ReadNoSnoop".to_string()],
                cells: vec!["Y".to_string()],
            }],
            cell_class: crate::ir::evidence::MatrixCellClass::Mark,
            legend: Vec::new(),
            notes: Vec::new(),
        }];
        intent_ir.write_to_disk()?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("matrix_adapter"),
        )?;
        let isf = artifact.isf.clone().expect("matrix ISF artifact");
        assert_eq!(isf.source_text, baseline_isf.source_text);
        let packets = artifact
            .residual_decisions
            .iter()
            .filter(|packet| packet.packet_id.starts_with("isf_protocol_matrix_table_"))
            .collect::<Vec<_>>();
        assert_eq!(packets.len(), 1);
        assert_eq!(
            packets[0].packet_id,
            "isf_protocol_matrix_table_matrix_table_0001"
        );
        assert!(packets[0].why_unresolved.contains(".3.c.v.iv"));
        assert!(
            packets[0]
                .why_unresolved
                .contains("supporting tables: table_0174, table_0175")
        );
        Ok(())
    }

    /// `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c.ii` — a per-message layout reaches the adapter as one residual naming its
    /// signals and message kinds, and the rendered `.isf` is unchanged.
    #[test]
    fn isf_adapter_residualizes_a_message_signal_layout_naming_its_messages() -> Result<()> {
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_message_signal_layouts.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let baseline = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("baseline_adapter"),
        )?;
        let baseline_isf = baseline.isf.expect("baseline ISF artifact");
        intent_ir.message_signal_layouts = serde_json::from_value(serde_json::json!([{
            "layout_id": "message_signal_layout_0001",
            "table_ids": ["table_0230", "table_0231"],
            "messages": ["TLBI 1-part", "TLBI 2nd part"],
            "bits": [
                {"table_id": "table_0230", "signal": "ACADDR", "bits": "[47]", "msb": 47, "lsb": 47,
                 "sources": [{"kind": "constant", "code": "0b0"},
                             {"kind": "field", "name": "VA", "bits": "[52]", "msb": 52, "lsb": 52}]},
                {"table_id": "table_0231", "signal": "ACVMIDEXT", "bits": "[0]", "msb": 0, "lsb": 0,
                 "sources": [{"kind": "field", "name": "VMIDV"}, {"kind": "unstated", "text": "-"}]}
            ]
        }]))?;
        intent_ir.write_to_disk()?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("message_layout_adapter"),
        )?;
        let isf = artifact.isf.clone().expect("message layout ISF artifact");
        assert_eq!(isf.source_text, baseline_isf.source_text);
        let packets = artifact
            .residual_decisions
            .iter()
            .filter(|packet| {
                packet
                    .packet_id
                    .starts_with("isf_protocol_message_signal_layout_")
            })
            .collect::<Vec<_>>();
        assert_eq!(packets.len(), 1);
        assert_eq!(
            packets[0].packet_id,
            "isf_protocol_message_signal_layout_message_signal_layout_0001"
        );
        assert!(
            packets[0]
                .why_unresolved
                .contains("ACADDR, ACVMIDEXT over TLBI 1-part; TLBI 2nd part")
        );
        assert!(
            packets[0]
                .why_unresolved
                .contains("supporting tables: table_0230, table_0231")
        );
        Ok(())
    }

    /// `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii` — a signal layout reaches the adapter as one residual citing its
    /// tables and naming each sub-field past its port's width, and the rendered `.isf` is unchanged.
    #[test]
    fn isf_adapter_residualizes_a_signal_layout_naming_fields_past_the_width() -> Result<()> {
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_signal_layouts.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let baseline = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("baseline_adapter"),
        )?;
        let baseline_isf = baseline.isf.expect("baseline ISF artifact");
        let field =
            |bits: &str, msb: u32, lsb: u32, name: &str| crate::ir::evidence::SignalSubField {
                table_id: "table_0001".to_string(),
                bits: bits.to_string(),
                msb,
                lsb,
                name: name.to_string(),
                cells: Vec::new(),
                levels: Vec::new(),
            };
        let layout = |id: &str, signal: &str, sub_fields| crate::ir::evidence::SignalLayoutRecord {
            layout_id: id.to_string(),
            signal: signal.to_string(),
            table_ids: vec!["table_0001".to_string()],
            caption: None,
            name_column: "Name".to_string(),
            other_columns: Vec::new(),
            sub_fields,
        };
        intent_ir.signal_layouts = vec![
            layout(
                "signal_layout_0001",
                "DATA_IN",
                vec![field("[3:0]", 3, 0, "LOW"), field("[9:8]", 9, 8, "HIGH")],
            ),
            layout(
                "signal_layout_0002",
                "NOPE",
                vec![field("[0]", 0, 0, "BIT")],
            ),
            layout(
                "signal_layout_0003",
                "DATA_X",
                vec![field("[8]", 8, 8, "TOP")],
            ),
        ];
        // A layout on a collective names each member's port.
        intent_ir.collective_signal_names = vec![crate::ir::evidence::CollectiveSignalNameRecord {
            collective_id: "collective_0001".to_string(),
            collective_name: "DATA_X".to_string(),
            member_names: vec!["DATA_IN".to_string()],
            supporting_statement_ids: Vec::new(),
        }];
        intent_ir.write_to_disk()?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("layout_adapter"),
        )?;
        let isf = artifact.isf.clone().expect("layout ISF artifact");
        assert_eq!(isf.source_text, baseline_isf.source_text);
        let packets = artifact
            .residual_decisions
            .iter()
            .filter(|packet| packet.packet_id.starts_with("isf_protocol_signal_layout_"))
            .collect::<Vec<_>>();
        assert_eq!(packets.len(), 3);
        assert_eq!(
            packets[0].packet_id,
            "isf_protocol_signal_layout_signal_layout_0001"
        );
        assert!(packets[0].why_unresolved.contains("FSMGen ladder F29"));
        assert!(
            packets[0]
                .why_unresolved
                .contains("`DATA_IN` (its width 8) leaves HIGH [9:8] past it"),
            "{}",
            packets[0].why_unresolved
        );
        assert!(!packets[0].why_unresolved.contains("LOW [3:0]"));
        assert!(
            packets[0]
                .why_unresolved
                .contains("supporting tables: table_0001")
        );
        assert!(
            packets[1]
                .why_unresolved
                .contains("no port of this actor is named `NOPE`")
        );
        assert!(
            packets[2]
                .why_unresolved
                .contains("`DATA_IN` (its width 8) leaves TOP [8] past it"),
            "{}",
            packets[2].why_unresolved
        );
        Ok(())
    }

    /// `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.ii` — a value combination reaches the adapter as one residual citing
    /// its tables, and the rendered `.isf` is unchanged.
    #[test]
    fn isf_adapter_residualizes_a_value_combination_citing_its_tables() -> Result<()> {
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_value_combinations.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let baseline = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("baseline_adapter"),
        )?;
        let baseline_isf = baseline.isf.expect("baseline ISF artifact");
        intent_ir.value_combinations = vec![crate::ir::evidence::ValueCombinationRecord {
            combination_id: "value_combination_0001".to_string(),
            table_ids: vec!["table_0239".to_string(), "table_0240".to_string()],
            keys: ["SYSCOREQ", "SYSCOACK"]
                .iter()
                .map(|name| crate::ir::evidence::CombinationKey {
                    title: name.to_string(),
                    name: name.to_string(),
                    bits: None,
                    note: None,
                })
                .collect(),
            member_column: Some("State".to_string()),
            meaning_columns: Vec::new(),
            enabling_column: None,
            rows: vec![crate::ir::evidence::CombinationRow {
                table_id: "table_0239".to_string(),
                values: vec!["1".to_string(), "0".to_string()],
                member: Some("Connect".to_string()),
                meanings: Vec::new(),
                value_encodings: Vec::new(),
                enabled_by: None,
            }],
        }];
        // EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.iv — the state the combination keys says its encoding is stated.
        intent_ir.protocol_states = vec![ProtocolStateRecord {
            state_id: "combination_state_0001".to_string(),
            machine_name: None,
            state_name: "Connect".to_string(),
            action: None,
            supporting_statement_ids: Vec::new(),
            supporting_table_ids: vec!["table_0239".to_string()],
            supporting_combination_ids: vec!["value_combination_0001".to_string()],
        }];
        intent_ir.write_to_disk()?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("combination_adapter"),
        )?;
        let isf = artifact.isf.clone().expect("combination ISF artifact");
        assert_eq!(isf.source_text, baseline_isf.source_text);
        let packets = artifact
            .residual_decisions
            .iter()
            .filter(|packet| {
                packet
                    .packet_id
                    .starts_with("isf_protocol_value_combination_")
            })
            .collect::<Vec<_>>();
        assert_eq!(packets.len(), 1);
        assert_eq!(
            packets[0].packet_id,
            "isf_protocol_value_combination_value_combination_0001"
        );
        assert!(
            packets[0]
                .why_unresolved
                .contains("no condition names one of its members")
        );
        assert!(
            packets[0]
                .why_unresolved
                .contains("supporting tables: table_0239, table_0240")
        );
        let state = artifact
            .residual_decisions
            .iter()
            .find(|packet| packet.packet_id == "isf_protocol_state_combination_state_0001")
            .expect("the keyed state's residual");
        assert!(
            state
                .why_unresolved
                .contains("its encoding by signal values is stated by value_combination_0001")
        );
        assert!(
            !state
                .why_unresolved
                .contains("initial-state identity, encoding,")
        );
        Ok(())
    }

    #[test]
    fn isf_adapter_residualizes_every_protocol_record_without_changing_rendered_isf() -> Result<()>
    {
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_protocol_residuals.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let baseline = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("baseline_adapter"),
        )?;
        assert!(
            baseline
                .residual_decisions
                .iter()
                .all(|packet| !packet.packet_id.starts_with("isf_protocol_"))
        );
        let baseline_isf = baseline.isf.expect("baseline ISF artifact");

        intent_ir.serial_frame_fields = vec![
            SerialFrameField {
                field_id: "serial_field_0002".to_string(),
                name: "OMEGA".to_string(),
                bit_width: Some(3),
                bit_range: Some((2, 0)),
                phase_name: Some("closing".to_string()),
                participant_drive: Some(ParticipantDriveRecord {
                    source_actor: "recipient".to_string(),
                    destination_actor: Some("initiator".to_string()),
                }),
                order: Some(1),
                response_values: vec!["accepted".to_string(), "rejected".to_string()],
                supporting_statement_ids: vec!["statement_closing".to_string()],
            },
            SerialFrameField {
                field_id: "serial_field_0001".to_string(),
                name: "ALPHA".to_string(),
                bit_width: Some(1),
                bit_range: Some((0, 0)),
                phase_name: Some("opening".to_string()),
                participant_drive: Some(ParticipantDriveRecord {
                    source_actor: "initiator".to_string(),
                    destination_actor: Some("recipient".to_string()),
                }),
                order: Some(0),
                response_values: Vec::new(),
                supporting_statement_ids: vec!["statement_opening".to_string()],
            },
        ];
        intent_ir.protocol_operations = vec![ProtocolOperationRecord {
            operation_id: "protocol_operation_0001".to_string(),
            branch_label: Some("deferred".to_string()),
            operation_name: Some("transfer".to_string()),
            phase_count: 2,
            phase_names: vec!["opening".to_string(), "closing".to_string()],
            supporting_statement_ids: vec!["statement_operation".to_string()],
        }];
        intent_ir.protocol_states = vec![ProtocolStateRecord {
            state_id: "protocol_state_0001".to_string(),
            machine_name: Some("serial machine".to_string()),
            state_name: "Reset".to_string(),
            action: None,
            supporting_statement_ids: vec!["statement_state".to_string()],
            supporting_table_ids: Vec::new(),
            supporting_combination_ids: Vec::new(),
        }];
        intent_ir.interface_edge_timings = vec![InterfaceEdgeTimingRecord {
            timing_id: "interface_edge_timing_0001".to_string(),
            actor_name: "target".to_string(),
            signal_name: "DATA".to_string(),
            clock_signal: "CLK".to_string(),
            edge: InterfaceClockEdge::Rising,
            samples_on_edge: true,
            drive_changes_on_edge: true,
            supporting_statement_ids: vec!["statement_edge".to_string()],
        }];
        // EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.ii — one stated equivalence rule.
        intent_ir.equivalence_rules = vec![EquivalenceRuleRecord {
            equivalence_id: "equivalence_0000".to_string(),
            subject: "Locked requests".to_string(),
            equivalent: "exclusive requests".to_string(),
            condition: Some("When the bridge is idle".to_string()),
            modality: EquivalenceModality::Shall,
            supporting_statement_ids: vec!["statement_equivalence".to_string()],
        }];
        // EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a — one presence row with a stated absent value, one without.
        intent_ir.signal_presence_records = vec![
            SignalPresenceRecord {
                presence_id: "presence_0000".to_string(),
                signal_name: "XCLK".to_string(),
                presence_condition: None,
                variant_presence: vec![VariantPresenceEntry {
                    variant_label: "V1".to_string(),
                    code: "Y".to_string(),
                    legend: Vec::new(),
                }],
                table_id: "table_presence".to_string(),
                legend_matrix_id: None,
                legend_table_id: None,
                absent_value: None,
                qualified_absent_values: Vec::new(),
            },
            SignalPresenceRecord {
                presence_id: "presence_0001".to_string(),
                signal_name: "XRESP".to_string(),
                presence_condition: Some("XRESP_WIDTH > 0".to_string()),
                variant_presence: vec![VariantPresenceEntry {
                    variant_label: "V1".to_string(),
                    code: "O".to_string(),
                    legend: Vec::new(),
                }],
                table_id: "table_presence".to_string(),
                legend_matrix_id: None,
                legend_table_id: None,
                absent_value: Some(StatedAbsentValue {
                    value: "0b000".to_string(),
                    supporting_statement_ids: vec!["statement_absent".to_string()],
                    supporting_table_ids: Vec::new(),
                    collective_name: None,
                    applies_to: None,
                    encoding: None,
                }),
                qualified_absent_values: vec![StatedAbsentValue {
                    value: "Exclusive".to_string(),
                    supporting_statement_ids: vec!["statement_scoped".to_string()],
                    supporting_table_ids: Vec::new(),
                    collective_name: None,
                    applies_to: Some("Locked requests".to_string()),
                    encoding: Some(StatedValueEncoding {
                        table_id: "table_encoding".to_string(),
                        codes: vec!["0b1".to_string()],
                    }),
                }],
            },
        ];
        intent_ir.write_to_disk()?;

        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("protocol_adapter"),
        )?;
        assert_eq!(artifact.lowering_status, AdapterLoweringStatus::Renderable);
        let isf = artifact.isf.clone().expect("protocol ISF artifact");
        assert!(isf.is_renderable);
        assert_eq!(isf.source_text, baseline_isf.source_text);
        assert_eq!(isf.signal_count, baseline_isf.signal_count);
        assert_eq!(isf.transaction_count, baseline_isf.transaction_count);
        assert_eq!(isf.rule_count, baseline_isf.rule_count);

        let protocol_residuals = artifact
            .residual_decisions
            .iter()
            .filter(|packet| packet.packet_id.starts_with("isf_protocol_"))
            .collect::<Vec<_>>();
        assert_eq!(
            protocol_residuals
                .iter()
                .map(|packet| packet.packet_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "isf_protocol_serial_frame_field_serial_field_0002",
                "isf_protocol_serial_frame_field_serial_field_0001",
                "isf_protocol_operation_protocol_operation_0001",
                "isf_protocol_state_protocol_state_0001",
                "isf_protocol_interface_edge_timing_interface_edge_timing_0001",
                "isf_protocol_equivalence_rule_equivalence_0000",
                "isf_protocol_signal_absent_value_presence_0001",
                "isf_protocol_signal_qualified_absent_value_presence_0001_1",
            ]
        );
        let scoped = protocol_residuals
            .last()
            .expect("the class-scoped absent value");
        assert!(scoped.question.contains("`Exclusive`"));
        assert!(scoped.question.contains("for Locked requests"));
        assert!(scoped.why_unresolved.contains("one class of requests only"));
        assert!(
            scoped
                .why_unresolved
                .contains("encoding table: table_encoding (code 0b1)")
        );
        for packet in &protocol_residuals {
            assert_eq!(packet.automation_confidence.as_str(), "low");
            assert_eq!(packet.candidate_interpretations.len(), 2);
            assert!(packet.why_unresolved.contains("remains exact in IntentIR"));
            assert!(packet.why_unresolved.contains("supporting statements:"));
        }
        assert!(
            protocol_residuals[0]
                .question
                .contains("serial-frame field")
        );
        assert!(
            protocol_residuals[0]
                .why_unresolved
                .contains("declared serial data wire")
        );
        assert!(
            protocol_residuals[2]
                .why_unresolved
                .contains("ordered field/step membership")
        );
        assert!(
            protocol_residuals[3]
                .why_unresolved
                .contains("typed transitions")
        );
        assert!(
            protocol_residuals[4]
                .why_unresolved
                .contains("sample destination")
        );
        assert!(protocol_residuals[5].question.contains("equivalence rule"));
        assert!(
            protocol_residuals[5]
                .why_unresolved
                .contains("treats one thing as another")
        );
        assert!(
            protocol_residuals[5]
                .why_unresolved
                .contains("supporting statements: statement_equivalence")
        );
        assert!(protocol_residuals[6].question.contains("`0b000`"));
        assert!(protocol_residuals[6].question.contains("`XRESP`"));
        assert!(protocol_residuals[6].why_unresolved.contains(
            "supporting statements: statement_absent; supporting tables: table_presence"
        ));

        let repeated = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("repeated_protocol_adapter"),
        )?;
        assert_eq!(artifact.residual_decisions, repeated.residual_decisions);

        artifact.write_to_disk()?;
        let reloaded =
            AdapterArtifact::load_from_path(&artifact.artifact_layout.adapter_artifact_path)?;
        assert_eq!(reloaded.residual_decisions, artifact.residual_decisions);

        let isf_path = tempdir.path().join("protocol_residuals.isf");
        fs::write(&isf_path, &isf.source_text)?;
        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|error| {
            panic!("fsmgen non-JSON: {error}\nstdout:{stdout}\nstderr:{stderr}")
        });
        assert_eq!(
            check["diagnostic_summary"]["success"].as_bool(),
            Some(true),
            "protocol residuals must not invalidate independently licensed ISF: {stdout}\n{stderr}"
        );
        assert_eq!(
            check["diagnostic_summary"]["diagnostic_count"].as_i64(),
            Some(0)
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
        // The bounded-eventually now lowers to `(assert (monitor …))` (the
        // `(contract … eventually …)` clause was removed at pin 43b29f5c —
        // FSMGEN-ASSERT-MIGRATE).
        let has_assert_monitor = src.contains("\n    (assert (monitor ");
        let has_temporal_rule = src.contains("(rule temporal_");
        let has_temporal_residual = artifact
            .residual_decisions
            .iter()
            .any(|p| p.packet_id.starts_with("isf_temporal_unrepresentable_"));
        assert!(
            has_assert_monitor || has_temporal_rule || has_temporal_residual,
            "temporal_rules were silently dropped (no assert-monitor, no temporal rule, no residual)\n{src}"
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

    #[test]
    fn adapter_proof_covers_every_field_record_rendered_line_and_exact_intent_prefix() -> Result<()>
    {
        let workspace = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            workspace.path(),
            "adapter_proof.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &workspace.path().join("generated/adapters"),
        )?;
        artifact.write_to_disk()?;
        let (artifact, adapter_proof) = AdapterArtifact::load_with_verified_proof(
            &artifact.artifact_layout.adapter_artifact_path,
        )?;
        let (_, intent_proof) = IntentIr::load_with_verified_proof(&artifact.intent_ir_path)?;

        let fields = artifact
            .persisted_clone()?
            .public_field_values()
            .map_err(super::adapter_derivation_error)?;
        let array_records = fields
            .values()
            .filter_map(serde_json::Value::as_array)
            .map(Vec::len)
            .sum::<usize>();
        let isf = artifact.isf.as_ref().expect("ISF payload");
        let rendered_lines = isf
            .source_text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count();
        let expected_local = super::ADAPTER_RULE_FIELDS.len()
            + array_records
            + rendered_lines
            + isf.blocking_reasons.len();
        let intent_claims = intent_proof.ledger().claims();
        let cumulative_claims = adapter_proof.ledger().claims();
        assert_eq!(super::ADAPTER_RULE_FIELDS.len(), 12);
        assert_eq!(
            super::ADAPTER_RULE_FIELDS
                .iter()
                .map(|(_, family)| *family)
                .collect::<BTreeSet<_>>()
                .len(),
            4
        );
        assert_eq!(
            cumulative_claims.len(),
            intent_claims.len() + expected_local
        );
        assert_eq!(
            &cumulative_claims[..intent_claims.len()],
            intent_claims,
            "adapter must retain the exact verified cumulative IntentIR ledger as its prefix"
        );
        let local = &cumulative_claims[intent_claims.len()..];
        assert!(local.iter().all(|claim| matches!(
            claim.premises().first(),
            Some(crate::ir::derivation::PremiseRef::RegisteredDerivation { .. })
        )));
        assert_eq!(
            local
                .iter()
                .filter(|claim| {
                    claim.address().surface() == "isf"
                        && claim
                            .address()
                            .stable_record_key()
                            .starts_with("source-line-")
                })
                .count(),
            rendered_lines,
            "every rendered nonblank ISF line must carry one lowering claim"
        );
        Ok(())
    }

    #[test]
    fn proofless_and_forged_adapter_cannot_load_serialize_or_write() -> Result<()> {
        let workspace = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            workspace.path(),
            "adapter_forgery.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &workspace.path().join("generated/adapters"),
        )?;
        artifact.write_to_disk()?;
        let path = artifact.artifact_layout.adapter_artifact_path.clone();
        let canonical = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&path)?)?;

        let mut proofless = canonical.clone();
        proofless
            .as_object_mut()
            .expect("adapter object")
            .remove("proof_context");
        proofless
            .as_object_mut()
            .expect("adapter object")
            .remove("proof_ledger");
        fs::write(&path, serde_json::to_string_pretty(&proofless)?)?;
        assert!(AdapterArtifact::load_from_path(&path).is_err());
        let decoded = serde_json::from_value::<AdapterArtifact>(proofless)?;
        assert!(decoded.to_pretty_json().is_err());
        assert!(decoded.write_to_disk().is_err());

        let mut forged = canonical;
        forged["isf"]["source_text"] =
            serde_json::Value::String("(actor forged\n  (interface)\n)\n".to_string());
        fs::write(&path, serde_json::to_string_pretty(&forged)?)?;
        assert!(AdapterArtifact::load_from_path(&path).is_err());
        let forged = serde_json::from_value::<AdapterArtifact>(forged)?;
        assert!(forged.to_pretty_json().is_err());
        assert!(forged.write_to_disk().is_err());
        Ok(())
    }

    #[test]
    fn adapter_validation_is_closed_and_unrelated_mutation_rejects() -> Result<()> {
        let workspace = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            workspace.path(),
            "adapter_mutation.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let mut artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &workspace.path().join("generated/adapters"),
        )?;
        let report = crate::ir::source::ValidationReportRecord {
            report_id: "adapter-validation".to_string(),
            validated_stage: crate::ir::IrStage::IsfAdapter,
            artifact_fingerprint: "fingerprint".to_string(),
            summary: "test report".to_string(),
            overall_score: None,
            grade: None,
            metrics: Vec::new(),
            findings: Vec::new(),
        };
        artifact.validation_reports.push(report);
        assert!(artifact.authorize_mutation(AdapterMutationKind::ValidationBackannotation)?);
        assert!(!artifact.authorize_mutation(AdapterMutationKind::ValidationBackannotation)?);
        artifact.write_to_disk()?;
        assert!(
            AdapterArtifact::load_from_path(&artifact.artifact_layout.adapter_artifact_path)
                .is_ok()
        );

        artifact.adapter_identity.summary.push_str(" forged");
        artifact
            .validation_reports
            .push(crate::ir::source::ValidationReportRecord {
                report_id: "second".to_string(),
                validated_stage: crate::ir::IrStage::IsfAdapter,
                artifact_fingerprint: "second".to_string(),
                summary: "second".to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: Vec::new(),
            });
        assert!(
            artifact
                .authorize_mutation(AdapterMutationKind::ValidationBackannotation)
                .is_err()
        );
        Ok(())
    }

    #[test]
    fn legacy_adapter_is_inspection_only_and_future_schema_rejects() -> Result<()> {
        let workspace = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            workspace.path(),
            "adapter_compatibility.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &workspace.path().join("generated/adapters"),
        )?;
        let path = artifact.artifact_layout.adapter_artifact_path.clone();
        artifact.write_to_disk()?;
        let canonical = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&path)?)?;

        let mut legacy = canonical.clone();
        legacy["schema_version"] = serde_json::json!(1);
        legacy
            .as_object_mut()
            .expect("adapter object")
            .remove("proof_context");
        legacy
            .as_object_mut()
            .expect("adapter object")
            .remove("proof_ledger");
        fs::write(&path, serde_json::to_string_pretty(&legacy)?)?;
        assert!(AdapterArtifact::load_from_path(&path).is_err());
        assert!(AdapterArtifact::load_for_inspection(&path).is_ok());

        let mut future = canonical;
        future["schema_version"] =
            serde_json::json!(u64::from(super::ADAPTER_ARTIFACT_SCHEMA_VERSION) + 1);
        fs::write(&path, serde_json::to_string_pretty(&future)?)?;
        assert!(AdapterArtifact::load_from_path(&path).is_err());
        assert!(AdapterArtifact::load_for_inspection(&path).is_err());
        Ok(())
    }
}

#[cfg(test)]
mod keyword_definition_residual_tests {
    use super::keyword_definition_residual_packet;
    use crate::ir::evidence::{
        KeywordDefinitionRecord, KeywordObligation, KeywordObligationModality,
    };

    #[test]
    fn a_keywords_obligations_are_named_as_unbound() {
        assert!(keyword_definition_residual_packet(&[]).is_none());
        let record = |id: &str, keyword: &str, obligations: usize| KeywordDefinitionRecord {
            keyword_id: id.to_string(),
            keyword: keyword.to_string(),
            definition: format!("{keyword} is a keyword."),
            obligations: (0..obligations)
                .map(|_| KeywordObligation {
                    text: "shall be ignored by the receiver".to_string(),
                    modality: KeywordObligationModality::Obligation,
                    agents: vec!["receiver".to_string()],
                })
                .collect(),
            supporting_statement_ids: vec!["s0".to_string()],
        };
        let packet = keyword_definition_residual_packet(&[
            record("keyword_0000", "May", 0),
            record("keyword_0001", "Reserved", 5),
        ])
        .expect("a residual");
        assert_eq!(packet.packet_id, "isf_keyword_obligations_not_bound");
        assert!(
            packet
                .why_unresolved
                .contains("`keyword_0000`-`keyword_0001`")
        );
        assert!(packet.why_unresolved.contains("1 state obligations"));
        assert!(packet.why_unresolved.contains("`Reserved` (5)"));
        assert!(!packet.why_unresolved.contains("`May`"));
    }
}
