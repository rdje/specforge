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
    SymbolCapabilityClass, VerifiedProofLedger,
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
    let implementation_sha256 = Sha256Digest::of_serializable(&[
        Sha256Digest::of_bytes(include_bytes!("adapters.rs")),
        Sha256Digest::of_bytes(include_bytes!("isf_ir.rs")),
    ])?;
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
    if let Some(initiator) = crate::ir::isf_ir::select_initiator_actor(&intent_ir.actor_ports)
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

fn protocol_residual_packet(
    surface_id: &str,
    surface_label: &str,
    record_id: &str,
    supporting_statement_ids: &[String],
    missing_bindings: &str,
) -> ResidualDecisionPacket {
    let provenance = if supporting_statement_ids.is_empty() {
        "no supporting statement ids".to_string()
    } else {
        format!(
            "supporting statements: {}",
            supporting_statement_ids.join(", ")
        )
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

/// Account for every canonical protocol observation at the adapter boundary (ADR 0016).
///
/// None of the current record types supplies every operand required by a supported executable ISF
/// construct. Keep one stable residual packet per input record, in schema and record order, instead
/// of silently dropping observations or fabricating transitions, values, ports, storage, or timing.
fn protocol_residual_decisions(intent_ir: &IntentIr) -> Vec<ResidualDecisionPacket> {
    let mut residuals = Vec::with_capacity(
        intent_ir.serial_frame_fields.len()
            + intent_ir.protocol_operations.len()
            + intent_ir.protocol_states.len()
            + intent_ir.interface_edge_timings.len(),
    );

    for record in &intent_ir.serial_frame_fields {
        residuals.push(protocol_residual_packet(
            "serial_frame_field",
            "serial-frame field",
            &record.field_id,
            &record.supporting_statement_ids,
            "an enclosing transaction/operation binding, the declared serial data wire, and complete literal, activation, and storage semantics",
        ));
    }
    for record in &intent_ir.protocol_operations {
        residuals.push(protocol_residual_packet(
            "operation",
            "protocol operation",
            &record.operation_id,
            &record.supporting_statement_ids,
            "a typed activation condition, ordered field/step membership, bound ports, and state-transition effects",
        ));
    }
    for record in &intent_ir.protocol_states {
        residuals.push(protocol_residual_packet(
            "state",
            "protocol state",
            &record.state_id,
            &record.supporting_statement_ids,
            "typed transitions, guards, initial-state identity, encoding, and bound action operands",
        ));
    }
    for record in &intent_ir.interface_edge_timings {
        residuals.push(protocol_residual_packet(
            "interface_edge_timing",
            "interface-edge timing",
            &record.timing_id,
            &record.supporting_statement_ids,
            "an activation/guard, sample destination, drive value, and transaction or state association",
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
        EvidenceIr, InterfaceClockEdge, InterfaceEdgeTimingRecord, ParticipantDriveRecord,
        ProtocolOperationRecord, ProtocolStateRecord, SerialFrameField,
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
            ]
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
