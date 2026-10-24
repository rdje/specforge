//! `PRODUCTION-GENERICITY`'s `INVENTORY` component, native since `RUSTDOC-AND-XTASK.15.b`: every Rust module of the
//! three product packages is classified exactly once in `doctrine/production_genericity/module_inventory.tsv`, and
//! every top-level field of the five stage artifacts exactly once in `claim_family_inventory.tsv`, so a module or an
//! artifact field nobody classified cannot enter the product unseen. Each row is held to its schema: its planes,
//! lane, stage, family name and field names.
//!
//! `scripts/check_production_genericity_inventory.pl` is a shim that keeps its `Getopt::Long` front end and the root
//! resolution, then passes `--root DIR` and `--self-test`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const LABEL: &str = "production-genericity-inventory";
const MODULE_HEADER: &str = "path\tcurrent_plane\ttarget_plane\tprimary_lane\tdisposition";
const CLAIM_HEADER: &str =
    "family_id\tstage\ttop_level_fields\tcurrent_authority\ttarget_proof_class\tprimary_lane";
const CURRENT_PLANES: [&str; 15] = [
    "application",
    "conformance",
    "core_provider",
    "mixed",
    "core_foundation",
    "core",
    "core_grammar",
    "core_schema",
    "core_learning",
    "core_verifier",
    "core_registry",
    "core_inference",
    "core_lowering",
    "core_capture",
    "test_support",
];
const TARGET_PLANES: [&str; 7] = [
    "application",
    "conformance",
    "core_provider",
    "application+conformance",
    "core+conformance",
    "core",
    "core_rule",
];
const STAGES: [&str; 5] = [
    "source_ir",
    "evidence_ir",
    "semantic_ir",
    "intent_ir",
    "isf_adapter",
];
/// Each stage's artifact struct and the file it is declared in, in the script's sorted stage order.
const STRUCTS: [(&str, &str, &str); 5] = [
    (
        "evidence_ir",
        "crates/specforge/src/ir/evidence.rs",
        "EvidenceIr",
    ),
    ("intent_ir", "crates/specforge/src/ir/intent.rs", "IntentIr"),
    (
        "isf_adapter",
        "crates/specforge/src/ir/adapters.rs",
        "AdapterArtifact",
    ),
    (
        "semantic_ir",
        "crates/specforge/src/ir/semantic.rs",
        "SemanticIr",
    ),
    ("source_ir", "crates/specforge/src/ir/source.rs", "SourceIr"),
];

/// What Perl's `die` printed and the status it exited with.
struct Death {
    message: String,
    code: i32,
}

/// The file's lines with each trailing newline removed, as `<$handle>` and `chomp` leave them.
fn slurp_lines(path: &Path) -> Result<Vec<String>, Death> {
    let bytes = fs::read(path).map_err(|error| {
        let text = error.to_string();
        Death {
            message: format!(
                "cannot read {}: {}",
                path.display(),
                text.split(" (os error").next().unwrap_or(&text)
            ),
            code: error.raw_os_error().unwrap_or(255),
        }
    })?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let mut lines: Vec<String> = text.split('\n').map(str::to_string).collect();
    if text.ends_with('\n') || text.is_empty() {
        lines.pop();
    }
    Ok(lines)
}

/// `--root DIR` (resolved by the shim) and `--self-test`.
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    let mut root = repository.to_path_buf();
    let mut self_test = false;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(directory) => root = PathBuf::from(directory),
                None => return usage(),
            },
            "--self-test" => self_test = true,
            _ => return usage(),
        }
    }
    let outcome = if self_test {
        run_self_tests(&root).map(|()| {
            println!("{LABEL} self-test: 3/3 pass");
            0
        })
    } else {
        inspect_inventory(&root).map(|census| {
            if census.problems.is_empty() {
                println!(
                    "{LABEL}: {} modules, {} claim families, {} top-level artifact fields are classified exactly once",
                    census.modules, census.families, census.fields
                );
                return 0;
            }
            for problem in &census.problems {
                eprintln!("{LABEL}: {problem}");
            }
            1
        })
    };
    match outcome {
        Ok(code) => code,
        Err(death) => {
            eprintln!("{}", death.message);
            death.code
        }
    }
}

fn usage() -> i32 {
    eprintln!(
        "Usage: cargo xtask check production-genericity-inventory [--root DIR] [--self-test]"
    );
    2
}

struct Census {
    problems: Vec<String>,
    modules: usize,
    families: usize,
    fields: usize,
}

/// `^crates/specforge(?:-core|-conformance)?/src/(?:[A-Za-z0-9_./-]+)\.rs$`, and no `..` component.
fn is_repository_rust_path(path: &str) -> bool {
    let rest = [
        "crates/specforge/src/",
        "crates/specforge-core/src/",
        "crates/specforge-conformance/src/",
    ]
    .iter()
    .find_map(|prefix| path.strip_prefix(prefix));
    let Some(stem) = rest.and_then(|rest| rest.strip_suffix(".rs")) else {
        return false;
    };
    !stem.is_empty()
        && stem
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'/' | b'-'))
        && !path.split('/').any(|component| component == "..")
}

/// `^e\.(?:ii|iii|iv)$`.
fn is_lane(lane: &str) -> bool {
    matches!(lane, "e.ii" | "e.iii" | "e.iv")
}

/// `^[a-z][a-z0-9_]*$`.
fn is_identifier(text: &str) -> bool {
    let mut bytes = text.bytes();
    bytes.next().is_some_and(|first| first.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

/// `^[a-z][a-z0-9_]*\.[a-z][a-z0-9_]*$`.
fn is_family_id(text: &str) -> bool {
    text.split_once('.')
        .is_some_and(|(stage, name)| is_identifier(stage) && is_identifier(name))
}

/// Every regular `.rs` file under `directory` (symbolic links to files counted, links to directories not followed),
/// as repository-relative paths.
fn rust_files(root: &Path, directory: &Path, out: &mut BTreeSet<String>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        match entry.file_type() {
            Ok(kind) if kind.is_dir() => rust_files(root, &path, out),
            _ => {
                let name = entry.file_name().to_string_lossy().into_owned();
                if path.is_file() && name.ends_with(".rs") {
                    let relative = path
                        .strip_prefix(root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .into_owned();
                    out.insert(relative);
                }
            }
        }
    }
}

fn inspect_inventory(root: &Path) -> Result<Census, Death> {
    let mut problems = Vec::new();
    let module_lines =
        slurp_lines(&root.join("doctrine/production_genericity/module_inventory.tsv"))?;
    let (module_header, module_rows) = module_lines
        .split_first()
        .map_or(("", &[][..]), |(header, rows)| (header.as_str(), rows));
    if module_header != MODULE_HEADER {
        problems.push("module inventory header is not the schema-1 contract".to_string());
    }
    let mut declared_modules = BTreeSet::new();
    for (index, line) in module_rows.iter().enumerate() {
        let number = index + 2;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 5 || fields.iter().any(|field| field.is_empty()) {
            problems.push(format!(
                "module inventory line {number} must contain five non-empty TSV fields"
            ));
            continue;
        }
        let (path, current, target, lane) = (fields[0], fields[1], fields[2], fields[3]);
        if !is_repository_rust_path(path) {
            problems.push(format!(
                "module inventory line {number} has a non-repository Rust path '{path}'"
            ));
        }
        if !is_lane(lane) {
            problems.push(format!(
                "module inventory line {number} has invalid primary lane '{lane}'"
            ));
        }
        if !CURRENT_PLANES.contains(&current) {
            problems.push(format!(
                "module inventory line {number} has unknown current plane '{current}'"
            ));
        }
        if !TARGET_PLANES.contains(&target) {
            problems.push(format!(
                "module inventory line {number} has unknown target plane '{target}'"
            ));
        }
        if !declared_modules.insert(path.to_string()) {
            problems.push(format!("module inventory duplicates '{path}'"));
        }
    }
    let mut actual_modules = BTreeSet::new();
    for package in ["specforge", "specforge-core", "specforge-conformance"] {
        let source = root.join("crates").join(package).join("src");
        if source.is_dir() {
            rust_files(root, &source, &mut actual_modules);
        }
    }
    for missing in actual_modules.difference(&declared_modules) {
        problems.push(format!("Rust module is unclassified: {missing}"));
    }
    for stale in declared_modules.difference(&actual_modules) {
        problems.push(format!("module inventory path is stale: {stale}"));
    }

    let claim_lines =
        slurp_lines(&root.join("doctrine/production_genericity/claim_family_inventory.tsv"))?;
    let (claim_header, claim_rows) = claim_lines
        .split_first()
        .map_or(("", &[][..]), |(header, rows)| (header.as_str(), rows));
    if claim_header != CLAIM_HEADER {
        problems.push("claim-family inventory header is not the schema-1 contract".to_string());
    }
    let mut declared_families: BTreeSet<String> = BTreeSet::new();
    let mut declared_fields: BTreeMap<String, String> = BTreeMap::new();
    for (index, line) in claim_rows.iter().enumerate() {
        let number = index + 2;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 6 || fields.iter().any(|field| field.is_empty()) {
            problems.push(format!(
                "claim-family inventory line {number} must contain six non-empty TSV fields"
            ));
            continue;
        }
        let (family, stage, field_list, lane) = (fields[0], fields[1], fields[2], fields[5]);
        if !is_family_id(family) {
            problems.push(format!(
                "claim-family inventory line {number} has invalid family id '{family}'"
            ));
        }
        if !declared_families.insert(family.to_string()) {
            problems.push(format!("claim-family inventory duplicates '{family}'"));
        }
        if !STAGES.contains(&stage) {
            problems.push(format!(
                "claim-family inventory line {number} has unknown stage '{stage}'"
            ));
        }
        let prefix = if stage == "isf_adapter" {
            "adapter"
        } else {
            stage.strip_suffix("_ir").unwrap_or(stage)
        };
        if !family.starts_with(&format!("{prefix}.")) {
            problems.push(format!(
                "claim-family inventory line {number} family '{family}' does not match stage '{stage}'"
            ));
        }
        if !is_lane(lane) {
            problems.push(format!(
                "claim-family inventory line {number} has invalid primary lane '{lane}'"
            ));
        }
        for field in field_list.split(',') {
            if !is_identifier(field) {
                problems.push(format!(
                    "claim-family inventory line {number} has invalid field '{field}'"
                ));
                continue;
            }
            let qualified = format!("{stage}.{field}");
            match declared_fields.get(&qualified) {
                Some(first) => problems.push(format!(
                    "claim field '{qualified}' appears in both '{first}' and '{family}'"
                )),
                None => {
                    declared_fields.insert(qualified, family.to_string());
                }
            }
        }
    }
    let mut actual_fields = BTreeSet::new();
    for (stage, relative, name) in STRUCTS {
        let lines = slurp_lines(&root.join(relative))?;
        let opening = format!("pub struct {name} {{");
        let mut inside = false;
        let mut found = false;
        for line in &lines {
            if !inside {
                if line
                    .strip_prefix(&opening)
                    .is_some_and(|rest| rest.chars().all(is_space))
                {
                    inside = true;
                    found = true;
                }
                continue;
            }
            if line
                .strip_prefix('}')
                .is_some_and(|rest| rest.chars().all(is_space))
            {
                break;
            }
            if let Some(field) = field_name(line) {
                actual_fields.insert(format!("{stage}.{field}"));
            }
        }
        if !found {
            problems.push(format!(
                "could not locate top-level struct {name} in {relative}"
            ));
        }
    }
    let declared_set: BTreeSet<String> = declared_fields.keys().cloned().collect();
    for missing in actual_fields.difference(&declared_set) {
        problems.push(format!(
            "top-level artifact field is unclassified: {missing}"
        ));
    }
    for stale in declared_set.difference(&actual_fields) {
        problems.push(format!("claim-family inventory field is stale: {stale}"));
    }
    Ok(Census {
        problems,
        modules: actual_modules.len(),
        families: declared_families.len(),
        fields: actual_fields.len(),
    })
}

fn is_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{c}')
}

/// `^\s+pub ([A-Za-z][A-Za-z0-9_]*):`.
fn field_name(line: &str) -> Option<&str> {
    let rest = line.trim_start_matches(is_space);
    if rest.len() == line.len() {
        return None;
    }
    let rest = rest.strip_prefix("pub ")?;
    let length = rest
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(rest.len());
    let name = &rest[..length];
    (name.starts_with(|character: char| character.is_ascii_alphabetic())
        && rest[length..].starts_with(':'))
    .then_some(name)
}

// The self-test.

fn write_text(path: &Path, text: &str) -> Result<(), Death> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, text).map_err(|error| Death {
        message: format!("cannot write {}: {error}", path.display()),
        code: error.raw_os_error().unwrap_or(255),
    })
}

fn self_test_fixture(fixture: &Path) -> Result<(), Death> {
    let structs = [
        ("source_ir", "source.rs", "SourceIr", "alpha"),
        ("evidence_ir", "evidence.rs", "EvidenceIr", "beta"),
        ("semantic_ir", "semantic.rs", "SemanticIr", "gamma"),
        ("intent_ir", "intent.rs", "IntentIr", "delta"),
        ("isf_adapter", "adapters.rs", "AdapterArtifact", "epsilon"),
    ];
    let mut module_rows = Vec::new();
    let mut claim_rows = Vec::new();
    for (stage, file, name, field) in structs {
        let prefix = if stage == "isf_adapter" {
            "adapter"
        } else {
            stage.strip_suffix("_ir").unwrap_or(stage)
        };
        let relative = format!("crates/specforge/src/ir/{file}");
        write_text(
            &fixture.join(&relative),
            &format!("pub struct {name} {{\n    pub {field}: String,\n}}\n"),
        )?;
        module_rows.push(format!("{relative}\tcore\tcore\te.iii\tfixture"));
        claim_rows.push(format!(
            "{prefix}.family\t{stage}\t{field}\tfixture\tintegrity_attestation\te.iii"
        ));
    }
    write_text(
        &fixture.join("doctrine/production_genericity/module_inventory.tsv"),
        &format!("{MODULE_HEADER}\n{}\n", module_rows.join("\n")),
    )?;
    write_text(
        &fixture.join("doctrine/production_genericity/claim_family_inventory.tsv"),
        &format!("{CLAIM_HEADER}\n{}\n", claim_rows.join("\n")),
    )
}

/// The script's three cases on a fixture under `generated/`, removed whatever the verdict.
fn run_self_tests(root: &Path) -> Result<(), Death> {
    let fixture = root.join(format!(
        "generated/production-genericity-inventory-tests.{}",
        std::process::id()
    ));
    if fixture.exists() {
        let _ = fs::remove_dir_all(&fixture);
    }
    let failure = self_test_cases(&fixture).err();
    if fixture.exists() {
        let _ = fs::remove_dir_all(&fixture);
    }
    if fixture.exists() {
        return Err(Death {
            message: format!("self-test residue remains at {}", fixture.display()),
            code: 255,
        });
    }
    failure.map_or(Ok(()), Err)
}

fn self_test_cases(fixture: &Path) -> Result<(), Death> {
    let die = |message: &str| Death {
        message: message.to_string(),
        code: 255,
    };
    self_test_fixture(fixture)?;
    let clean = inspect_inventory(fixture)?;
    if !clean.problems.is_empty() {
        return Err(Death {
            message: format!(
                "self-test clean fixture failed: {}",
                clean.problems.join(" ")
            ),
            code: 255,
        });
    }
    let unclassified = fixture.join("crates/specforge/src/ir/unclassified.rs");
    write_text(&unclassified, "pub struct Unclassified;\n")?;
    let census = inspect_inventory(fixture)?;
    if !census
        .problems
        .iter()
        .any(|problem| problem.contains("Rust module is unclassified"))
    {
        return Err(die("self-test did not reject an unclassified module"));
    }
    let _ = fs::remove_file(&unclassified);
    write_text(
        &fixture.join("crates/specforge/src/ir/source.rs"),
        "pub struct SourceIr {\n    pub alpha: String,\n    pub new_field: String,\n}\n",
    )?;
    let census = inspect_inventory(fixture)?;
    if !census
        .problems
        .iter()
        .any(|problem| problem.contains("top-level artifact field is unclassified"))
    {
        return Err(die(
            "self-test did not reject an unclassified artifact field",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_inventories_are_exact_and_the_self_test_refuses_drift() {
        let root = crate::repository_root();
        let census = inspect_inventory(&root).ok().expect("a census");
        assert!(census.problems.is_empty(), "{:?}", census.problems);
        assert!(census.modules > 0 && census.families > 0 && census.fields > 0);
        assert!(run_self_tests(&root).is_ok());
    }

    #[test]
    fn duplicates_stale_paths_planes_and_double_classified_fields_are_refused() {
        let fixture = crate::repository_root()
            .join(".project-data/tmp/xtask-production-genericity-inventory");
        let _ = fs::remove_dir_all(&fixture);
        self_test_fixture(&fixture).ok().expect("a fixture");
        let modules = fixture.join("doctrine/production_genericity/module_inventory.tsv");
        let claims = fixture.join("doctrine/production_genericity/claim_family_inventory.tsv");
        let module_text = fs::read_to_string(&modules).expect("the module inventory");
        let claim_text = fs::read_to_string(&claims).expect("the claim inventory");
        fs::write(
            &modules,
            format!(
                "{module_text}crates/specforge/src/ir/source.rs\tcore\tcore\te.ii\tx\ncrates/specforge/src/gone.rs\tcore\tnowhere\te.ii\tx\n"
            ),
        )
        .expect("a mutated module inventory");
        fs::write(
            &claims,
            format!("{claim_text}source.again\tsource_ir\talpha\tx\ty\te.ii\n"),
        )
        .expect("a mutated claim inventory");
        let census = inspect_inventory(&fixture).ok().expect("a census");
        fs::remove_dir_all(&fixture).expect("cleanup");
        for expected in [
            "module inventory duplicates 'crates/specforge/src/ir/source.rs'",
            "module inventory line 8 has unknown target plane 'nowhere'",
            "module inventory path is stale: crates/specforge/src/gone.rs",
            "claim field 'source_ir.alpha' appears in both 'source.family' and 'source.again'",
        ] {
            assert!(
                census.problems.iter().any(|problem| problem == expected),
                "{expected}: {:?}",
                census.problems
            );
        }
    }

    #[test]
    fn rows_are_held_to_the_scripts_patterns() {
        assert!(is_repository_rust_path(
            "crates/specforge-core/src/ir/a.b-c.rs"
        ));
        assert!(!is_repository_rust_path("crates/specforge/src/../x.rs"));
        assert!(!is_repository_rust_path("crates/other/src/x.rs"));
        assert!(!is_repository_rust_path("crates/specforge/src/.rs"));
        assert!(
            is_family_id("source.family_1") && !is_family_id("Source.x") && !is_family_id("source")
        );
        assert_eq!(field_name("    pub kind: Kind,"), Some("kind"));
        assert_eq!(field_name("pub kind: Kind,"), None);
        assert_eq!(field_name("    pub kind : Kind,"), None);
        assert_eq!(field_name("    pub(crate) kind: Kind,"), None);
    }
}
