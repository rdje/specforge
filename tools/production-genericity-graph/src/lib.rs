//! Deterministic Rust syntax graph for SpecForge's compiled production surface.
//!
//! Cargo remains the independent type, privacy, and feature-resolution oracle. This crate derives
//! a repository-root-relative module/item/import/call/macro graph for structural genericity checks;
//! it does not make semantic information-flow decisions by itself.

mod analyzer;
mod config;
mod inventory;
mod metadata;
mod model;

use std::path::Path;

pub use model::ProductionGraph;

/// Derive the production graph from Cargo targets and the checked module inventory.
pub fn analyze_repository(root: &Path) -> Result<ProductionGraph, String> {
    analyzer::analyze(root)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use super::analyze_repository;

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
        assert_eq!(first.inventory_files.len(), 77);
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

    #[test]
    fn fixture_graph_fails_closed_on_currentness_and_resolution_breaches() {
        let fixture = Fixture::new();
        let clean = analyze_repository(&fixture.root).expect("clean fixture graph");
        assert_eq!(clean.inventory_files.len(), 7);
        assert_eq!(clean.reachable_files.len(), 6);
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
            "mod absent;\n#[path = \"../../specforge/src/core_shared.rs\"]\npub mod core_shared;\n",
        );
        assert_error(&fixture.root, "module 'absent' has no source");

        fixture.write(
            "crates/specforge-core/src/lib.rs",
            "mod extra;\n#[path = \"../../specforge/src/core_shared.rs\"]\npub mod core_shared;\n",
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

    fn assert_error(root: &Path, expected: &str) {
        let error = analyze_repository(root).expect_err("mutated fixture must fail closed");
        assert!(
            error.contains(expected),
            "expected error containing {expected:?}, got {error:?}"
        );
    }

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new() -> Self {
            let root = repository_root().join("generated").join(format!(
                "production-genericity-graph-tests.{}",
                std::process::id()
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
                "#[path = \"../../specforge/src/core_shared.rs\"]\npub mod core_shared;\npub fn core_entry() { core_shared::helper(); }\n",
            );
            fixture.write("crates/specforge/src/core_shared.rs", BASE_SHARED);
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
                "pub fn named_fixture_only() {}\n",
            );
            fixture.write_inventory(BASE_INVENTORY);
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
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            if self.root.exists() {
                fs::remove_dir_all(&self.root).expect("remove graph fixture");
            }
        }
    }

    const BASE_SHARED: &str = "macro_rules! local_value { () => { 1u8 }; }\n#[cfg(feature = \"conformance-support\")]\npub fn helper() { let _ = local_value!(); }\n";

    const BASE_INVENTORY: &str = "path\tcurrent_plane\ttarget_plane\tprimary_lane\tdisposition\n\
crates/specforge-conformance/src/lib.rs\tconformance\tconformance\te.v.ii\tconformance root\n\
crates/specforge-core/src/lib.rs\tcore_foundation\tcore\te.v.ii\tcore root\n\
crates/specforge/src/conformance_surface.rs\tconformance\tconformance\te.v.ii\tconformance surface\n\
crates/specforge/src/core_shared.rs\tcore\tcore\te.v.ii\tshared core\n\
crates/specforge/src/lib.rs\tapplication\tapplication\te.v.ii\tapplication library\n\
crates/specforge/src/main.rs\tapplication\tapplication\te.v.ii\tapplication binary\n\
crates/specforge/src/test_support.rs\ttest_support\tconformance\te.v.ii\ttest-only support\n";
}
