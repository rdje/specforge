//! Deterministic Rust syntax graph for SpecForge's compiled production surface.
//!
//! Cargo remains the independent type, privacy, and feature-resolution oracle. This crate derives
//! a repository-root-relative module/item/import/call/macro graph for structural genericity checks;
//! it does not make semantic information-flow decisions by itself.

mod analyzer;
mod config;
mod flow;
mod inventory;
mod metadata;
mod model;

use std::path::Path;

pub use model::{InformationFlowReport, ProductionGraph};

/// Derive the production graph from Cargo targets and the checked module inventory.
pub fn analyze_repository(root: &Path) -> Result<ProductionGraph, String> {
    analyzer::analyze(root)
}

/// Enforce the closed raw/identity-flow and proof-only promotion boundary.
pub fn analyze_information_flow(root: &Path) -> Result<InformationFlowReport, String> {
    flow::analyze(root)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{analyze_information_flow, analyze_repository};

    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(std::path::Path::parent)
            .expect("tool is two levels below the repository root")
            .to_path_buf()
    }

    #[test]
    fn current_repository_graph_is_complete_and_deterministic() {
        let root = repository_root();
        let first = analyze_repository(&root).expect("current production graph");
        let second = analyze_repository(&root).expect("repeat current production graph");
        assert_eq!(first.inventory_files.len(), 79);
        assert_eq!(first.targets.len(), 4);
        assert!(!first.modules.is_empty());
        assert!(!first.items.is_empty());
        assert!(!first.calls.is_empty());
        assert_eq!(
            serde_json::to_vec(&first).expect("serialize first graph"),
            serde_json::to_vec(&second).expect("serialize repeated graph")
        );
        let json = serde_json::to_string(&first).expect("serialize graph for locality check");
        assert!(
            !json.contains(root.to_str().expect("repository root is UTF-8")),
            "graph output must contain repository-relative paths only"
        );
    }

    /// PRODUCTION-GRAPH-CENSUS-PIN.0 — these four numbers are a repository-wide census, and they
    /// are pinned HERE and nowhere else. `scripts/check_production_genericity_flow.sh`, the
    /// gate-tier doctrine that runs on every commit, *prints* them; only this test compares them.
    /// Since ordinary commits run the doctrine driver rather than `cargo test` (the repository
    /// runs the full suite before a push, not per commit), the pins drifted by +18 functions,
    /// +179 helper edges, +226 decision sites and +3 semantic macros before anything noticed.
    /// Re-pinned at that measured truth; the gap between the printing gate and the comparing test
    /// is the tree's own frontier. Moved again by `ACTOR-NOUN-RELATION-DECLARATION.1` (+1 function,
    /// +4 helper edges, +5 decision sites for one predicate at three call sites) — the second slice
    /// in a row to add a single predicate and have to edit this contract to land, which is the
    /// evidence `.1` of that tree needs before choosing between an exact pin and a declared band.
    /// Third consecutive slice to move it by one predicate (`INVARIANT-SHAPE-ADMISSION.1`): three for
    /// three, which is the answer to `.1`'s open question about how often these numbers legitimately
    /// move. An exact pin makes every predicate-adding commit a contract-editing commit.
    /// FOURTH consecutive slice (`INVARIANT-SHAPE-ADMISSION.3`, +6 functions / +32 helper edges /
    /// +29 decision sites / +2 semantic macros for one table reader). Four for four: the question
    /// `PRODUCTION-GRAPH-CENSUS-PIN.1` opened is no longer open — an exact pin on a count that moves
    /// with every ordinary production slice measures the commit rate, not the boundary.
    #[test]
    fn current_repository_flow_is_complete_local_and_deterministic() {
        let root = repository_root();
        let first = analyze_information_flow(&root).expect("current information-flow boundary");
        let second =
            analyze_information_flow(&root).expect("repeat current information-flow boundary");

        assert_eq!(first.boundary_rows, 141);
        assert_eq!(first.source_types, 22);
        assert_eq!(first.source_fields, 13);
        assert_eq!(first.source_parameters, 3);
        assert_eq!(first.source_returns, 2);
        assert_eq!(first.rule_roots, 120);
        assert_eq!(first.grammar_declassifiers, 64);
        assert_eq!(first.canonical_seams, 12);
        assert_eq!(first.proof_gates, 25);
        assert_eq!(first.trusted_regions, 11);
        assert_eq!(first.non_authoritative_regions, 6);
        assert_eq!(first.protected_types, 15);
        assert_eq!(first.analyzed_functions, 2_407);
        assert_eq!(first.helper_edges, 14_983);
        assert_eq!(first.decision_sites, 12_980);
        assert_eq!(first.protected_constructions, 19);
        assert_eq!(first.protected_calls, 28);
        assert_eq!(first.semantic_macros, 1_471);
        assert_eq!(
            serde_json::to_vec(&first).expect("serialize first flow report"),
            serde_json::to_vec(&second).expect("serialize repeated flow report")
        );
        let json = serde_json::to_string(&first).expect("serialize flow report for locality check");
        assert!(
            !json.contains(root.to_str().expect("repository root is UTF-8")),
            "flow output must not contain the absolute repository root"
        );
    }

    #[test]
    fn fixture_graph_fails_closed_on_currentness_and_resolution_breaches() {
        let fixture = Fixture::new();
        let clean = analyze_repository(&fixture.root).expect("clean fixture graph");
        assert_eq!(clean.inventory_files.len(), 8);
        assert_eq!(clean.reachable_files.len(), 7);
        assert_eq!(clean.excluded_test_support_files.len(), 1);
        assert_eq!(
            clean
                .macros
                .iter()
                .filter(|node| node.kind == "definition")
                .count(),
            1
        );
        assert!(clean.macros.iter().any(|node| node.resolution == "local"));

        fixture.write_inventory(BASE_INVENTORY.replace(
            "crates/specforge/src/core_shared.rs\tcore\tcore\te.v.ii\tshared core\n",
            "",
        ));
        assert_error(&fixture.root, "uninventoried source");

        fixture.write_inventory(format!(
            "{BASE_INVENTORY}crates/specforge/src/core_shared.rs\tcore\tcore\te.v.ii\tduplicate\n"
        ));
        assert_error(&fixture.root, "duplicate path");

        fixture.write_inventory(BASE_INVENTORY);
        fixture.write("crates/specforge/src/core_shared.rs", "pub fn broken( {\n");
        assert_error(&fixture.root, "cannot parse inventoried module");

        fixture.write(
            "crates/specforge/src/core_shared.rs",
            "#[cfg(feature = \"missing\")]\npub fn hidden() {}\n",
        );
        assert_error(&fixture.root, "undeclared Cargo feature 'missing'");

        fixture.write(
            "crates/specforge/src/core_shared.rs",
            "use crate::core_shared as duplicate;\nuse crate as duplicate;\npub fn helper() {}\n",
        );
        assert_error(&fixture.root, "ambiguous import alias 'duplicate'");

        fixture.write(
            "crates/specforge/src/core_shared.rs",
            "pub fn helper() {}\npub fn helper() {}\n",
        );
        assert_error(&fixture.root, "duplicate item identity");

        fixture.write("crates/specforge/src/core_shared.rs", BASE_SHARED);
        fixture.write(
            "crates/specforge-core/src/lib.rs",
            "mod absent;\n#[path = \"../../specforge/src/core_shared.rs\"]\npub mod core_shared;\n#[path = \"../../specforge/src/derivation.rs\"]\npub mod derivation;\n",
        );
        assert_error(&fixture.root, "module 'absent' has no source");

        fixture.write(
            "crates/specforge-core/src/lib.rs",
            "mod extra;\n#[path = \"../../specforge/src/core_shared.rs\"]\npub mod core_shared;\n#[path = \"../../specforge/src/derivation.rs\"]\npub mod derivation;\n",
        );
        fixture.write("crates/specforge-core/src/extra.rs", "pub fn first() {}\n");
        fixture.write(
            "crates/specforge-core/src/extra/mod.rs",
            "pub fn second() {}\n",
        );
        fixture.write_inventory(format!(
            "{BASE_INVENTORY}crates/specforge-core/src/extra.rs\tcore\tcore\te.v.ii\tfirst ambiguous module\ncrates/specforge-core/src/extra/mod.rs\tcore\tcore\te.v.ii\tsecond ambiguous module\n"
        ));
        assert_error(&fixture.root, "module 'extra' is ambiguous");
    }

    #[test]
    fn information_flow_fixture_fails_closed_on_boundary_breaches() {
        let fixture = Fixture::new();
        let clean =
            analyze_information_flow(&fixture.root).expect("clean information-flow fixture");
        assert_eq!(clean.source_types, 2);
        assert_eq!(clean.rule_roots, 1);
        assert_eq!(clean.canonical_seams, 1);
        assert_eq!(clean.proof_gates, 1);
        assert_eq!(clean.non_authoritative_regions, 0);
        assert_eq!(clean.protected_types, 2);

        fixture.mutate_shared(
            r#"
pub fn identity_selector(identity: &OpaqueIdentity) -> bool {
    identity.key == "named-document"
}
"#,
        );
        assert_flow_error(&fixture.root, "document_identity reaches semantic control");

        fixture.mutate_shared(
            r#"
pub fn raw_literal_selector(raw: &RawEvidence) -> bool {
    raw.text == "named-token"
}
"#,
        );
        assert_flow_error(&fixture.root, "raw_evidence reaches semantic control");

        fixture.mutate_shared(
            r#"
pub fn raw_substring_selector(raw: &RawEvidence) -> bool {
    raw.text.contains("named-token")
}
"#,
        );
        assert_flow_error(&fixture.root, "raw_evidence reaches semantic control");
        fixture.write(
            "doctrine/production_genericity/information_flow_boundary.tsv",
            format!(
                "{FLOW_BOUNDARY}declassify.wrong_class\tdeclassifier\tsymbol_identity\tcrates/specforge/src/core_shared.rs#raw_substring_selector\texact_identity_only\n"
            ),
        );
        assert_flow_error(&fixture.root, "raw_evidence reaches semantic control");
        fixture.write(
            "doctrine/production_genericity/information_flow_boundary.tsv",
            FLOW_BOUNDARY,
        );

        fixture.mutate_shared(
            r#"
pub struct FixtureRegex;
impl FixtureRegex {
    pub fn is_match(&self, _value: &str) -> bool { false }
}
pub fn raw_regex_selector(raw: &RawEvidence) -> bool {
    FixtureRegex.is_match(&raw.text)
}
"#,
        );
        assert_flow_error(&fixture.root, "raw_evidence reaches semantic control");

        fixture.mutate_shared(
            r#"
pub fn unregistered_inference(artifact: &mut CanonicalArtifact) {
    artifact.claims.push("proofless".to_owned());
}
"#,
        );
        assert_flow_error(&fixture.root, "canonical field 'claims' is mutated");

        fixture.mutate_shared(
            r#"
pub fn aliased_unregistered_inference(artifact: &mut CanonicalArtifact) {
    let claims = &mut artifact.claims;
    claims.push("proofless".to_owned());
}
"#,
        );
        assert_flow_error(&fixture.root, "canonical field 'claims' is mutated");

        fixture.mutate_shared(
            r#"
pub fn forge_capability() -> crate::derivation::SealedCapability {
    crate::derivation::SealedCapability { token: 1 }
}
"#,
        );
        assert_flow_error(
            &fixture.root,
            "protected type 'SealedCapability' is constructed",
        );

        fixture.mutate_shared(
            r#"
fn decide(value: &str) -> bool { value.contains("x") }
pub fn launder(raw: &RawEvidence) -> bool {
    decide(&format!("{}", raw.text))
}
"#,
        );
        assert_flow_error(&fixture.root, "raw_evidence reaches semantic control");

        fixture.mutate_shared(
            r#"
macro_rules! semantic_box { ($value:expr) => { $value }; }
pub fn macro_launder(raw: &RawEvidence) -> String {
    semantic_box!(raw.text.clone())
}
"#,
        );
        assert_flow_error(&fixture.root, "unresolved semantic macro 'semantic_box!'");

        fixture.mutate_shared("");
        let proofless = fs::read_to_string(
            fixture.root.join("crates/specforge/src/core_shared.rs"),
        )
        .expect("read flow fixture source")
        .replace(
            "pub fn write_canonical(value: &CanonicalArtifact) { let _ = verified_canonical_proof(value); }",
            "pub fn write_canonical(_value: &CanonicalArtifact) {}",
        );
        fixture.write("crates/specforge/src/core_shared.rs", proofless);
        assert_flow_error(
            &fixture.root,
            "has no graph path to a stage-matched proof gate",
        );

        fixture.mutate_shared("");
        analyze_information_flow(&fixture.root).expect("restored information-flow fixture");

        fixture.write(
            "doctrine/production_genericity/information_flow_boundary.tsv",
            FLOW_BOUNDARY.replace(
                "verified_authority:source_ir",
                "verified_authority:evidence_ir",
            ),
        );
        assert_flow_error(&fixture.root, "stage-matched proof gate");

        fixture.write(
            "doctrine/production_genericity/information_flow_boundary.tsv",
            format!("{FLOW_BOUNDARY}{}", FLOW_BOUNDARY.lines().nth(1).unwrap()),
        );
        assert_flow_error(&fixture.root, "duplicate boundary id 'source.raw'");
    }

    fn assert_error(root: &Path, expected: &str) {
        let error = analyze_repository(root).expect_err("mutated fixture must fail closed");
        assert!(
            error.contains(expected),
            "expected error containing {expected:?}, got {error:?}"
        );
    }

    fn assert_flow_error(root: &Path, expected: &str) {
        let error =
            analyze_information_flow(root).expect_err("mutated flow fixture must fail closed");
        assert!(
            error.contains(expected),
            "expected flow error containing {expected:?}, got {error:?}"
        );
    }

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new() -> Self {
            let root = repository_root().join("generated").join(format!(
                "production-genericity-graph-tests.{}.{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
            if root.exists() {
                fs::remove_dir_all(&root).expect("remove stale graph fixture");
            }
            fs::create_dir_all(&root).expect("create graph fixture root");
            let fixture = Self { root };
            fixture.write(
                "Cargo.toml",
                "[workspace]\nmembers = [\"crates/specforge\", \"crates/specforge-core\", \"crates/specforge-conformance\"]\nresolver = \"2\"\n\n[workspace.package]\nedition = \"2024\"\nrust-version = \"1.95\"\n",
            );
            fixture.write(
                "crates/specforge-core/Cargo.toml",
                "[package]\nname = \"specforge-core\"\nversion = \"0.0.0\"\nedition.workspace = true\nrust-version.workspace = true\n\n[features]\nconformance-support = []\ntest-support = []\n",
            );
            fixture.write(
                "crates/specforge-conformance/Cargo.toml",
                "[package]\nname = \"specforge-conformance\"\nversion = \"0.0.0\"\nedition.workspace = true\nrust-version.workspace = true\n\n[dependencies]\nspecforge-core = { path = \"../specforge-core\", features = [\"conformance-support\"] }\n",
            );
            fixture.write(
                "crates/specforge/Cargo.toml",
                "[package]\nname = \"specforge\"\nversion = \"0.0.0\"\nedition.workspace = true\nrust-version.workspace = true\n\n[dependencies]\nspecforge-core = { path = \"../specforge-core\" }\nspecforge-conformance = { path = \"../specforge-conformance\" }\n",
            );
            fixture.write(
                "crates/specforge-core/src/lib.rs",
                "#[path = \"../../specforge/src/core_shared.rs\"]\npub mod core_shared;\n#[path = \"../../specforge/src/derivation.rs\"]\npub mod derivation;\npub fn core_entry() { core_shared::helper(); }\n",
            );
            fixture.write("crates/specforge/src/core_shared.rs", BASE_SHARED);
            fixture.write(
                "crates/specforge/src/derivation.rs",
                "pub struct SealedCapability { pub token: u8 }\npub fn mint() -> SealedCapability { SealedCapability { token: 0 } }\n",
            );
            fixture.write(
                "crates/specforge-conformance/src/lib.rs",
                "pub use specforge_core::core_shared;\n#[path = \"../../specforge/src/conformance_surface.rs\"]\npub mod conformance_surface;\n",
            );
            fixture.write(
                "crates/specforge/src/conformance_surface.rs",
                "pub fn observe() { crate::core_shared::helper(); }\n",
            );
            fixture.write(
                "crates/specforge/src/lib.rs",
                "pub use specforge_core::core_shared;\npub fn app() { core_shared::helper(); }\n",
            );
            fixture.write(
                "crates/specforge/src/main.rs",
                "fn main() { specforge::app(); }\n",
            );
            fixture.write(
                "crates/specforge/src/test_support.rs",
                "pub fn named_fixture_only(value: &str) -> bool { value.contains(\"fixture-name\") }\n",
            );
            fixture.write_inventory(BASE_INVENTORY);
            fixture.write(
                "doctrine/production_genericity/claim_family_inventory.tsv",
                FLOW_CLAIM_INVENTORY,
            );
            fixture.write(
                "doctrine/production_genericity/rule_family_inventory.tsv",
                FLOW_RULE_INVENTORY,
            );
            fixture.write(
                "doctrine/production_genericity/conformance_bypass_inventory.tsv",
                FLOW_BYPASS_INVENTORY,
            );
            fixture.write(
                "doctrine/production_genericity/information_flow_boundary.tsv",
                FLOW_BOUNDARY,
            );
            let status = Command::new("cargo")
                .args(["generate-lockfile", "--offline"])
                .current_dir(&fixture.root)
                .status()
                .expect("generate fixture lockfile");
            assert!(status.success(), "fixture lockfile generation failed");
            fixture
        }

        fn write(&self, relative: &str, contents: impl AsRef<[u8]>) {
            let path = self.root.join(relative);
            fs::create_dir_all(path.parent().expect("fixture path parent"))
                .expect("create fixture path parent");
            fs::write(path, contents).expect("write graph fixture file");
        }

        fn write_inventory(&self, contents: impl AsRef<[u8]>) {
            self.write(
                "doctrine/production_genericity/module_inventory.tsv",
                contents,
            );
        }

        fn mutate_shared(&self, suffix: &str) {
            self.write(
                "crates/specforge/src/core_shared.rs",
                format!("{BASE_SHARED}{suffix}"),
            );
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            if self.root.exists() {
                fs::remove_dir_all(&self.root).expect("remove graph fixture");
            }
        }
    }

    const BASE_SHARED: &str = r#"macro_rules! local_value { () => { 1u8 }; }
#[cfg(feature = "conformance-support")]
pub fn helper() { let _ = local_value!(); }
pub struct RawEvidence { pub text: String }
pub struct OpaqueIdentity { pub key: String }
pub struct CanonicalArtifact { pub claims: Vec<String> }
pub fn build(raw: &RawEvidence) -> CanonicalArtifact {
    let claims = if raw.text.contains("shall") { vec![raw.text.clone()] } else { Vec::new() };
    CanonicalArtifact { claims }
}
pub fn verified_canonical_proof(_value: &CanonicalArtifact) -> bool { true }
pub fn write_canonical(value: &CanonicalArtifact) { let _ = verified_canonical_proof(value); }
pub fn display(raw: &RawEvidence) -> String { format!("{}", raw.text) }
pub fn record_provenance(raw: &RawEvidence) -> String { raw.text.clone() }
"#;

    const BASE_INVENTORY: &str = "path\tcurrent_plane\ttarget_plane\tprimary_lane\tdisposition\n\
crates/specforge-conformance/src/lib.rs\tconformance\tconformance\te.v.ii\tconformance root\n\
crates/specforge-core/src/lib.rs\tcore_foundation\tcore\te.v.ii\tcore root\n\
crates/specforge/src/conformance_surface.rs\tconformance\tconformance\te.v.ii\tconformance surface\n\
crates/specforge/src/core_shared.rs\tcore\tcore\te.v.ii\tshared core\n\
crates/specforge/src/derivation.rs\tcore_registry\tcore\te.v.ii\ttrusted derivation fixture\n\
crates/specforge/src/lib.rs\tapplication\tapplication\te.v.ii\tapplication library\n\
crates/specforge/src/main.rs\tapplication\tapplication\te.v.ii\tapplication binary\n\
crates/specforge/src/test_support.rs\ttest_support\tconformance\te.v.ii\ttest-only support\n";

    const FLOW_CLAIM_INVENTORY: &str = "family_id\tstage\ttop_level_fields\tcurrent_authority\ttarget_proof_class\tprimary_lane\nfixture.claim\tsource_ir\tclaims\tfixture grammar\tregistered_grammar\te.v.iii\n";
    const FLOW_RULE_INVENTORY: &str = "family_id\tstage\trule_stem\tcurrent_producer_entrypoints\tcurrent_mutator_entrypoints\tpremise_kinds\tsymbol_capability\talpha_obligation\tcompatibility\ttarget_module\tschema_owner\tcanonical_seams\tmigration_leaf\nfixture.claim\tsource_ir\tfixture.claim\tcrates/specforge/src/core_shared.rs#build\t-\tsource_span\tgrammar_introduces\tintroduced_symbols_preserve_origins\tcurrent_only\tcrate::core_shared\tcrates/specforge/src/core_shared.rs\tcrates/specforge/src/core_shared.rs#write_canonical\te.v.iii\n";
    const FLOW_BYPASS_INVENTORY: &str =
        "entrypoint\tstage\tcurrent_mutation\tcurrent_sink\ttarget_disposition\tmigration_leaf\n";
    const FLOW_BOUNDARY: &str = "boundary_id\trole\tdata_class\tpath\tcontract\nsource.raw\tsource_type\traw_evidence\tcrates/specforge/src/core_shared.rs#RawEvidence\taggregate_input\nsource.identity\tsource_type\tdocument_identity\tcrates/specforge/src/core_shared.rs#OpaqueIdentity\taggregate_input\njoin.rules\tderived_join\tsemantic_control\tdoctrine/production_genericity/rule_family_inventory.tsv#entrypoints\tregistered_rule_entrypoints\njoin.grammar\tderived_join\traw_evidence\tdoctrine/production_genericity/rule_family_inventory.tsv#grammar_introduces\tregistered_grammar_declassifiers\njoin.seams\tderived_join\tcanonical_authority\tdoctrine/production_genericity/rule_family_inventory.tsv#canonical_seams\tproof_only_sinks\nauthority.fixture\tproof_gate\tproof_authority\tcrates/specforge/src/core_shared.rs#verified_canonical_proof\tverified_authority:source_ir\ntrusted.fixture\ttrusted_region\tproof_authority\tcrates/specforge/src/derivation.rs#module\tkernel_implementation\nprotected.capability\tprotected_type\tproof_authority\tcrates/specforge/src/derivation.rs#SealedCapability\tkernel_only_construct\nprotected.artifact\tprotected_type\tproof_authority\tcrates/specforge/src/core_shared.rs#CanonicalArtifact\tregistered_rule_construct\n";
}
