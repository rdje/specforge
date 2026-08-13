pub mod adapters;
pub mod ambiguity;
pub mod condition_extract;
pub mod constraint_extract_llm;
pub mod contract;
pub mod corpus_cluster;
pub mod cve;
pub mod derivation;
pub mod entity_typing;
pub mod evidence;
pub mod extraction_filters;
pub mod extractor;
pub mod fidelity;
pub mod figure_region;
pub mod fusion;
pub mod intent;
pub mod isf_ir;
pub mod nli_verify;
pub mod nlp_relation_extract;
pub mod normative_vocab;
pub mod prior_memory;
pub mod protocol_graph;
pub mod register_bits;
pub mod semantic;
pub mod source;
pub mod temporal_ltl;
pub mod waveform;

use serde::{Deserialize, Serialize};

/// Serializes tests that shell out to the external `subs/fsmgen/bin/fsmgen`
/// Perl binary. That binary resolves paths relative to its own location and
/// performs an internal `cd`; under cargo's parallel test execution the
/// concurrent invocations race and intermittently fail with
/// `Can't cd to : No such file or directory` (a CWD/contention flake, not an
/// `.isf` correctness issue). Every fsmgen-invoking test takes this lock so
/// at most one runs the binary at a time.
#[cfg(test)]
pub(crate) static FSMGEN_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Run `subs/fsmgen/bin/fsmgen --strict --check --json <isf_path>` for tests,
/// serialized via [`FSMGEN_TEST_LOCK`] and pinned to the fsmgen repo root as
/// CWD (mirrors the downstream issue-bundle protocol's "run from the FSMGen
/// repository root" requirement, so `FindBin`/internal `cd` resolve
/// deterministically). `isf_path` must be absolute. A poisoned lock is
/// recovered so one failing locked test does not cascade-fail the rest.
#[cfg(test)]
pub(crate) fn run_fsmgen_strict_check(isf_path: &std::path::Path) -> std::process::Output {
    let _guard = FSMGEN_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let fsmgen_root = crate::project_data::repository_root()
        .expect("current SpecForge repository root")
        .join("subs/fsmgen");
    let temporary = crate::project_data::tempdir().expect("project-local fsmgen workspace");
    let mut command = std::process::Command::new(fsmgen_root.join("bin/fsmgen"));
    crate::project_data::configure_command(&mut command).expect("project-local fsmgen environment");
    command
        .env("TMPDIR", temporary.path())
        .env("TMP", temporary.path())
        .env("TEMP", temporary.path())
        .args(["--strict", "--check", "--json"])
        .arg(isf_path)
        .current_dir(&fsmgen_root)
        .output()
        .expect("run fsmgen")
}

/// Run `subs/fsmgen/bin/fsmgen --emit-schedule-json <isf_path>` for tests, serialized via
/// [`FSMGEN_TEST_LOCK`] and pinned to the fsmgen repo root as CWD (same protocol as
/// [`run_fsmgen_strict_check`]). The scheduler report JSON carries `inferred_storage[].fields[]`
/// (`name, msb, lsb, width, access, reset, enum`), the introspection surface used to assert that
/// emitted register field maps round-trip through FSMGen (DOC-INTENT-TAXONOMY.4a.ii). `isf_path`
/// must be absolute.
#[cfg(test)]
pub(crate) fn run_fsmgen_schedule_json(isf_path: &std::path::Path) -> std::process::Output {
    let _guard = FSMGEN_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let fsmgen_root = crate::project_data::repository_root()
        .expect("current SpecForge repository root")
        .join("subs/fsmgen");
    let temporary = crate::project_data::tempdir().expect("project-local fsmgen workspace");
    let mut command = std::process::Command::new(fsmgen_root.join("bin/fsmgen"));
    crate::project_data::configure_command(&mut command).expect("project-local fsmgen environment");
    command
        .env("TMPDIR", temporary.path())
        .env("TMP", temporary.path())
        .env("TEMP", temporary.path())
        .args(["--emit-schedule-json"])
        .arg(isf_path)
        .current_dir(&fsmgen_root)
        .output()
        .expect("run fsmgen")
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IrStage {
    SourceIr,
    EvidenceIr,
    SemanticIr,
    IntentIr,
    IsfAdapter,
}

impl IrStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceIr => "source_ir",
            Self::EvidenceIr => "evidence_ir",
            Self::SemanticIr => "semantic_ir",
            Self::IntentIr => "intent_ir",
            Self::IsfAdapter => "isf_adapter",
        }
    }
}

#[cfg(test)]
mod production_genericity_qualification_tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs;

    use super::derivation::{
        AlphaObligation, PremiseKind, RuleCompatibility, RuleDescriptor, SymbolCapabilityClass,
    };
    use super::{adapters, evidence, intent, semantic, source};

    const CLAIM_HEADER: &str =
        "family_id\tstage\ttop_level_fields\tcurrent_authority\ttarget_proof_class\tprimary_lane";
    const RULE_HEADER: &str = "family_id\tstage\trule_stem\tcurrent_producer_entrypoints\tcurrent_mutator_entrypoints\tpremise_kinds\tsymbol_capability\talpha_obligation\tcompatibility\ttarget_module\tschema_owner\tcanonical_seams\tmigration_leaf";

    #[derive(Debug)]
    struct ExpectedRule {
        stage: String,
        surface: String,
        premises: BTreeSet<String>,
        capability: String,
        alpha: String,
        compatibility: String,
        runtime_module: String,
    }

    #[test]
    fn every_registered_rule_satisfies_its_inventory_bound_structural_alpha_obligation() {
        let root = crate::project_data::repository_root().expect("current repository root");
        let doctrine = root.join("doctrine/production_genericity");
        let claims = fs::read_to_string(doctrine.join("claim_family_inventory.tsv"))
            .expect("read claim-family inventory");
        let rules = fs::read_to_string(doctrine.join("rule_family_inventory.tsv"))
            .expect("read rule-family inventory");

        let mut claim_lines = claims.lines();
        assert_eq!(claim_lines.next(), Some(CLAIM_HEADER));
        let mut fields_by_family = BTreeMap::new();
        for line in claim_lines {
            let columns: Vec<_> = line.split('\t').collect();
            assert_eq!(columns.len(), 6, "malformed claim-family row: {line}");
            let fields = columns[2]
                .split(',')
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>();
            assert!(
                fields_by_family
                    .insert(columns[0].to_owned(), fields)
                    .is_none(),
                "duplicate claim family '{}'",
                columns[0]
            );
        }

        let mut rule_lines = rules.lines();
        assert_eq!(rule_lines.next(), Some(RULE_HEADER));
        let mut expected = BTreeMap::new();
        for line in rule_lines {
            let columns: Vec<_> = line.split('\t').collect();
            assert_eq!(columns.len(), 13, "malformed rule-family row: {line}");
            let family = columns[0];
            let family_fields = fields_by_family
                .get(family)
                .unwrap_or_else(|| panic!("rule family '{family}' has no claim-family row"));
            for field in family_fields {
                let id = format!("{}.{}.v1", columns[2], field);
                let contract = ExpectedRule {
                    stage: columns[1].to_owned(),
                    surface: field.clone(),
                    premises: columns[5].split(',').map(ToOwned::to_owned).collect(),
                    capability: columns[6].to_owned(),
                    alpha: columns[7].to_owned(),
                    compatibility: columns[8].to_owned(),
                    runtime_module: match columns[1] {
                        "source_ir" => "crate::ir::source",
                        "evidence_ir" => "crate::ir::evidence",
                        "semantic_ir" => "crate::ir::semantic",
                        "intent_ir" => "crate::ir::intent",
                        "isf_adapter" => "crate::ir::adapters",
                        stage => panic!("unknown inventory stage '{stage}'"),
                    }
                    .to_owned(),
                };
                assert!(
                    expected.insert(id.clone(), contract).is_none(),
                    "duplicate inventory-expanded rule '{id}'"
                );
            }
        }
        assert_eq!(fields_by_family.len(), 41);
        assert_eq!(expected.len(), 168);

        let registries = [
            source::rule_registry_for_qualification().expect("SourceIR rule registry"),
            evidence::rule_registry_for_qualification().expect("EvidenceIR rule registry"),
            semantic::rule_registry_for_qualification().expect("SemanticIR rule registry"),
            intent::rule_registry_for_qualification().expect("IntentIR rule registry"),
            adapters::rule_registry_for_qualification().expect("adapter rule registry"),
        ];
        let mut actual = BTreeMap::<String, RuleDescriptor>::new();
        for registry in &registries {
            for descriptor in registry.descriptors() {
                descriptor
                    .validate_structural_alpha_obligation()
                    .unwrap_or_else(|error| {
                        panic!(
                            "registered rule '{}' failed its structural alpha obligation: {error}",
                            descriptor.rule_id().as_str()
                        )
                    });
                let id = descriptor.rule_id().as_str().to_owned();
                assert!(
                    actual.insert(id.clone(), descriptor.clone()).is_none(),
                    "duplicate runtime rule '{id}'"
                );
            }
        }
        assert_eq!(actual.len(), 168);
        assert_eq!(
            actual.keys().collect::<BTreeSet<_>>(),
            expected.keys().collect::<BTreeSet<_>>(),
            "runtime and inventory-expanded rule sets differ"
        );

        for (id, contract) in expected {
            let descriptor = &actual[&id];
            assert_eq!(descriptor.version(), 1, "rule '{id}' version");
            assert_eq!(
                descriptor.conclusion_stage().as_str(),
                contract.stage,
                "rule '{id}' stage"
            );
            assert_eq!(
                descriptor.conclusion_surface(),
                contract.surface,
                "rule '{id}' conclusion surface"
            );
            assert_eq!(
                descriptor
                    .premise_kinds()
                    .iter()
                    .copied()
                    .map(premise_name)
                    .map(ToOwned::to_owned)
                    .collect::<BTreeSet<_>>(),
                contract.premises,
                "rule '{id}' premise kinds"
            );
            assert_eq!(
                capability_name(descriptor.symbol_capability()),
                contract.capability,
                "rule '{id}' symbol capability"
            );
            assert_eq!(
                alpha_name(descriptor.alpha_obligation()),
                contract.alpha,
                "rule '{id}' alpha obligation"
            );
            assert_eq!(
                compatibility_name(descriptor.compatibility()),
                contract.compatibility,
                "rule '{id}' compatibility"
            );
            assert_eq!(
                descriptor.implementation_module(),
                contract.runtime_module,
                "rule '{id}' implementation module"
            );
        }
    }

    fn premise_name(kind: PremiseKind) -> &'static str {
        match kind {
            PremiseKind::SourceSpan => "source_span",
            PremiseKind::TableCell => "table_cell",
            PremiseKind::VisualRegion => "visual_region",
            PremiseKind::UpstreamClaim => "upstream_claim",
            PremiseKind::RegisteredDerivation => "registered_derivation",
            PremiseKind::GroundedModelProposal => "grounded_model_proposal",
            PremiseKind::ValidatedPrior => "validated_prior",
            PremiseKind::UniversalAxiom => "universal_axiom",
        }
    }

    fn capability_name(capability: SymbolCapabilityClass) -> &'static str {
        match capability {
            SymbolCapabilityClass::SymbolBlind => "symbol_blind",
            SymbolCapabilityClass::ExactIdentityOnly => "exact_identity_only",
            SymbolCapabilityClass::GrammarIntroduces => "grammar_introduces",
            SymbolCapabilityClass::LosslessCarry => "lossless_carry",
            SymbolCapabilityClass::MergeOrConflict => "merge_or_conflict",
            SymbolCapabilityClass::Residual => "residual",
            SymbolCapabilityClass::TargetLowering => "target_lowering",
        }
    }

    fn alpha_name(obligation: AlphaObligation) -> &'static str {
        match obligation {
            AlphaObligation::ByteIdenticalNonSymbolOutput => "byte_identical_non_symbol_output",
            AlphaObligation::IdentityGraphInvariant => "identity_graph_invariant",
            AlphaObligation::IntroducedSymbolsPreserveOrigins => {
                "introduced_symbols_preserve_origins"
            }
            AlphaObligation::LosslessTopologyInvariant => "lossless_topology_invariant",
            AlphaObligation::MergeConflictTopologyInvariant => "merge_conflict_topology_invariant",
            AlphaObligation::ResidualTopologyInvariant => "residual_topology_invariant",
            AlphaObligation::TargetSafeRenaming => "target_safe_renaming",
        }
    }

    fn compatibility_name(compatibility: RuleCompatibility) -> &'static str {
        match compatibility {
            RuleCompatibility::CurrentOnly => "current_only",
            RuleCompatibility::LosslessCarry => "lossless_carry",
        }
    }
}
