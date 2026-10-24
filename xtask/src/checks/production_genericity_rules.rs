//! `PRODUCTION-GENERICITY`'s `RULES` component, native since `RUSTDOC-AND-XTASK.15.c`: every claim family has one
//! rule-migration row in `doctrine/production_genericity/rule_family_inventory.tsv`, held to its schema (its stage,
//! stem, migration leaf, premises, symbol capability and the alpha obligation that capability requires, its
//! compatibility, target module and schema owner), every producer, mutator and canonical-seam entrypoint it names
//! resolves to a function in the source, each family's fields expand to valid, unique rule ids, the six canonical
//! write seams are inventoried, and every conformance-only bypass in `conformance_bypass_inventory.tsv` carries a
//! fail-closed disposition.
//!
//! `scripts/check_production_genericity_rules.pl` is a shim that keeps its `Getopt::Long` front end and the root
//! resolution, then passes `--root DIR` and `--self-test`.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

const LABEL: &str = "production-genericity-rules";
const CLAIM_HEADER: &str =
    "family_id\tstage\ttop_level_fields\tcurrent_authority\ttarget_proof_class\tprimary_lane";
const RULE_HEADER: &str = "family_id\tstage\trule_stem\tcurrent_producer_entrypoints\tcurrent_mutator_entrypoints\t\
                           premise_kinds\tsymbol_capability\talpha_obligation\tcompatibility\ttarget_module\t\
                           schema_owner\tcanonical_seams\tmigration_leaf";
const BYPASS_HEADER: &str =
    "entrypoint\tstage\tcurrent_mutation\tcurrent_sink\ttarget_disposition\tmigration_leaf";
const PREMISES: [&str; 8] = [
    "source_span",
    "table_cell",
    "visual_region",
    "upstream_claim",
    "registered_derivation",
    "grounded_model_proposal",
    "validated_prior",
    "universal_axiom",
];
/// Each symbol capability and the alpha obligation it requires.
const CAPABILITY_ALPHA: [(&str, &str); 7] = [
    ("symbol_blind", "byte_identical_non_symbol_output"),
    ("exact_identity_only", "identity_graph_invariant"),
    ("grammar_introduces", "introduced_symbols_preserve_origins"),
    ("lossless_carry", "lossless_topology_invariant"),
    ("merge_or_conflict", "merge_conflict_topology_invariant"),
    ("residual", "residual_topology_invariant"),
    ("target_lowering", "target_safe_renaming"),
];
const MIGRATION_LEAF: [(&str, &str); 5] = [
    ("source_ir", "6d.ii.e.iv.ii"),
    ("evidence_ir", "6d.ii.e.iv.iii"),
    ("semantic_ir", "6d.ii.e.iv.iv"),
    ("intent_ir", "6d.ii.e.iv.v"),
    ("isf_adapter", "6d.ii.e.iv.vi"),
];
const CANONICAL_WRITES: [&str; 6] = [
    "crates/specforge/src/ir/source.rs#SourceIr::write_to_disk",
    "crates/specforge/src/ir/evidence.rs#EvidenceIr::write_to_disk",
    "crates/specforge/src/ir/semantic.rs#SemanticIr::write_to_disk",
    "crates/specforge/src/ir/intent.rs#IntentIr::write_to_disk",
    "crates/specforge/src/ir/adapters.rs#AdapterArtifact::write_to_disk",
    "crates/specforge/src/commands/validate.rs#write_backannotated_artifact",
];
const KNOWN_BYPASSES: [&str; 4] = [
    "crates/specforge/src/commands/kg_bench.rs#run_fixture\tsource_ir",
    "crates/specforge/src/commands/kg_bench.rs#run_fixture\tevidence_ir",
    "crates/specforge/src/commands/kg_bench.rs#run_fixture\tsemantic_ir",
    "crates/specforge/src/commands/eval_extraction.rs#extract_on_copy\tevidence_ir",
];
const FAIL_CLOSED_DISPOSITIONS: [&str; 3] = [
    "noncanonical typed overlay cannot call canonical write or lowering",
    "noncanonical typed overlay cannot call canonical write or downstream production build",
    "verified persistence rebase preserves proof bytes and grants no semantic mutation",
];

fn migration_leaf(stage: &str) -> Option<&'static str> {
    MIGRATION_LEAF
        .iter()
        .find(|(name, _)| *name == stage)
        .map(|(_, leaf)| *leaf)
}

fn alpha_for(capability: &str) -> Option<&'static str> {
    CAPABILITY_ALPHA
        .iter()
        .find(|(name, _)| *name == capability)
        .map(|(_, alpha)| *alpha)
}

/// What Perl's `die` printed and the status it exited with.
struct Death {
    message: String,
    code: i32,
}

/// Files read once and kept: the script re-read a source file for every entrypoint naming it.
struct Sources {
    cache: RefCell<HashMap<PathBuf, String>>,
}

impl Sources {
    fn new() -> Self {
        Sources {
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn slurp(&self, path: &Path) -> Result<String, Death> {
        if let Some(text) = self.cache.borrow().get(path) {
            return Ok(text.clone());
        }
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
        self.cache
            .borrow_mut()
            .insert(path.to_path_buf(), text.clone());
        Ok(text)
    }
}

/// `split /\n/, $text, -1` with one trailing empty element dropped.
fn lines_of(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = text.split('\n').map(str::to_string).collect();
    if lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }
    lines
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
            println!("{LABEL} self-test: 7/7 pass");
            0
        })
    } else {
        inspect_rules(&root).map(|census| {
            if census.problems.is_empty() {
                println!(
                    "{LABEL}: {} families expand to {} field rules; {} producer/mutator entrypoints, {} canonical \
                     seams, and {} conformance-only bypass obligations resolve",
                    census.families, census.rules, census.entrypoints, census.seams, census.bypasses
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
    eprintln!("Usage: cargo xtask check production-genericity-rules [--root DIR] [--self-test]");
    2
}

fn is_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{c}')
}

/// `^[a-z][a-z0-9_]*\.[a-z][a-z0-9_]*$`.
fn is_rule_stem(text: &str) -> bool {
    let identifier = |part: &str| {
        let mut bytes = part.bytes();
        bytes.next().is_some_and(|first| first.is_ascii_lowercase())
            && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    };
    text.split_once('.')
        .is_some_and(|(left, right)| identifier(left) && identifier(right))
}

/// A repository-relative path, or `None` for one with a `..` component or an absolute one.
fn repository_path(root: &Path, relative: &str) -> Option<PathBuf> {
    if relative.split('/').any(|component| component == "..") || relative.starts_with('/') {
        return None;
    }
    Some(root.join(relative))
}

/// The function a `Type::name` symbol names: Perl's `split /::/` drops trailing empty fields.
fn function_name(symbol: &str) -> &str {
    let mut parts: Vec<&str> = symbol.split("::").collect();
    while parts.len() > 1 && parts.last() == Some(&"") {
        parts.pop();
    }
    parts.last().copied().unwrap_or("")
}

/// `^(crates/specforge/src/[A-Za-z0-9_./-]+\.rs)#([A-Za-z][A-Za-z0-9_:]*)$`.
fn split_entry(entry: &str) -> Option<(&str, &str)> {
    let (path, symbol) = entry.split_once('#')?;
    let stem = path
        .strip_prefix("crates/specforge/src/")?
        .strip_suffix(".rs")?;
    let path_ok = !stem.is_empty()
        && stem
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'/' | b'-'));
    let symbol_ok = symbol.starts_with(|character: char| character.is_ascii_alphabetic())
        && symbol
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b':');
    (path_ok && symbol_ok).then_some((path, symbol))
}

fn is_word(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

/// `/\bfn\s+NAME\s*(?:<[^;{}()]*>)?\s*\(/s` anywhere in the source.
fn declares_function(source: &str, name: &str) -> bool {
    source.match_indices("fn").any(|(at, _)| {
        if source[..at].chars().next_back().is_some_and(is_word) {
            return false;
        }
        let after_fn = &source[at + 2..];
        let spaced = after_fn.trim_start_matches(is_space);
        if spaced.len() == after_fn.len() {
            return false;
        }
        let Some(rest) = spaced.strip_prefix(name) else {
            return false;
        };
        let rest = rest.trim_start_matches(is_space);
        if rest.starts_with('(') {
            return true;
        }
        let Some(generics) = rest.strip_prefix('<') else {
            return false;
        };
        // `<[^;{}()]*>` then `\s*\(`: the run stops at the first of `;{}()`, which must be the `(`, and the greedy
        // run backtracks to its last `>` with only white space between it and that `(`.
        let run_end = generics
            .find([';', '{', '}', '(', ')'])
            .unwrap_or(generics.len());
        if !generics[run_end..].starts_with('(') {
            return false;
        }
        let run = &generics[..run_end];
        run.rfind('>')
            .is_some_and(|close| run[close + 1..].chars().all(is_space))
    })
}

/// A rule-inventory cell of `path#symbol` entries, `;`-separated, or `-`: each entry well formed, unique, its file
/// present and its function declared. The counter counts the entries whose file was found.
fn inspect_entrypoints(
    root: &Path,
    cell: &str,
    label: &str,
    number: usize,
    problems: &mut Vec<String>,
    counter: &mut usize,
    sources: &Sources,
) -> Result<(), Death> {
    if cell == "-" {
        return Ok(());
    }
    let mut seen = BTreeSet::new();
    for entry in cell.split(';') {
        let Some((relative, symbol)) = split_entry(entry) else {
            problems.push(format!(
                "rule inventory line {number} has malformed {label} entry '{entry}'"
            ));
            continue;
        };
        if !seen.insert(entry) {
            problems.push(format!(
                "rule inventory line {number} duplicates {label} entry '{entry}'"
            ));
        }
        let path = repository_path(root, relative).filter(|path| path.is_file());
        let Some(path) = path else {
            problems.push(format!(
                "rule inventory line {number} {label} path is missing: {relative}"
            ));
            continue;
        };
        let source = sources.slurp(&path)?;
        if !declares_function(&source, function_name(symbol)) {
            problems.push(format!(
                "rule inventory line {number} {label} symbol '{symbol}' is absent from {relative}"
            ));
        }
        *counter += 1;
    }
    Ok(())
}

/// `^crate::ir::([a-z][a-z0-9_]*)$`, resolved to its `<name>.rs` or `<name>/mod.rs`.
fn target_module_path(root: &Path, module: &str) -> Option<PathBuf> {
    let name = module.strip_prefix("crate::ir::")?;
    let mut bytes = name.bytes();
    let valid = bytes.next().is_some_and(|first| first.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_');
    if !valid {
        return None;
    }
    let directory = root.join("crates/specforge/src/ir");
    let file = directory.join(format!("{name}.rs"));
    if file.is_file() {
        return Some(file);
    }
    let module_file = directory.join(name).join("mod.rs");
    module_file.is_file().then_some(module_file)
}

struct Census {
    problems: Vec<String>,
    families: usize,
    rules: usize,
    entrypoints: usize,
    seams: usize,
    bypasses: usize,
}

struct Claim {
    stage: String,
    fields: Vec<String>,
}

fn inspect_rules(root: &Path) -> Result<Census, Death> {
    let sources = Sources::new();
    let mut problems = Vec::new();
    let claim_lines = lines_of(
        &sources.slurp(&root.join("doctrine/production_genericity/claim_family_inventory.tsv"))?,
    );
    let (claim_header, claim_rows) = claim_lines
        .split_first()
        .map_or(("", &[][..]), |(header, rows)| (header.as_str(), rows));
    if claim_header != CLAIM_HEADER {
        problems.push("claim-family inventory header is not the schema-1 contract".to_string());
    }
    let mut claims: BTreeMap<String, Claim> = BTreeMap::new();
    for (index, line) in claim_rows.iter().enumerate() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 6 || fields.iter().any(|field| field.is_empty()) {
            problems.push(format!(
                "claim-family inventory line {} is malformed",
                index + 2
            ));
            continue;
        }
        claims.insert(
            fields[0].to_string(),
            Claim {
                stage: fields[1].to_string(),
                fields: fields[2].split(',').map(str::to_string).collect(),
            },
        );
    }
    let rule_lines = lines_of(
        &sources.slurp(&root.join("doctrine/production_genericity/rule_family_inventory.tsv"))?,
    );
    let (rule_header, rule_rows) = rule_lines
        .split_first()
        .map_or(("", &[][..]), |(header, rows)| (header.as_str(), rows));
    if rule_header != RULE_HEADER {
        problems.push("rule-family inventory header is not the schema-1 contract".to_string());
    }
    let mut declared_families: BTreeSet<String> = BTreeSet::new();
    let mut expanded_rules: BTreeSet<String> = BTreeSet::new();
    let (mut entrypoints, mut seams) = (0usize, 0usize);
    for (index, line) in rule_rows.iter().enumerate() {
        let number = index + 2;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 13 || fields.iter().any(|field| field.is_empty()) {
            problems.push(format!(
                "rule inventory line {number} must contain 13 non-empty TSV fields"
            ));
            continue;
        }
        let [
            family,
            stage,
            stem,
            producers,
            mutators,
            premises,
            capability,
            alpha,
            compatibility,
            target,
            owner,
            seam_cell,
            leaf,
        ] = fields.as_slice()
        else {
            unreachable!("thirteen fields")
        };
        if !declared_families.insert((*family).to_string()) {
            problems.push(format!("rule inventory duplicates family '{family}'"));
        }
        let Some(claim) = claims.get(*family) else {
            problems.push(format!("rule inventory has unknown family '{family}'"));
            continue;
        };
        if *stage != claim.stage {
            problems.push(format!(
                "rule inventory family '{family}' stage '{stage}' disagrees with claim inventory"
            ));
        }
        if stem != family {
            problems.push(format!(
                "rule inventory family '{family}' must use rule stem '{family}'"
            ));
        }
        if !is_rule_stem(stem) {
            problems.push(format!(
                "rule inventory family '{family}' has invalid rule stem '{stem}'"
            ));
        }
        if migration_leaf(stage) != Some(*leaf) {
            problems.push(format!(
                "rule inventory family '{family}' has wrong migration leaf '{leaf}'"
            ));
        }
        let mut premise_seen = BTreeSet::new();
        for premise in premises.split(',') {
            if !PREMISES.contains(&premise) {
                problems.push(format!(
                    "rule inventory family '{family}' has unknown premise '{premise}'"
                ));
            }
            if !premise_seen.insert(premise) {
                problems.push(format!(
                    "rule inventory family '{family}' duplicates premise '{premise}'"
                ));
            }
        }
        match alpha_for(capability) {
            None => problems.push(format!(
                "rule inventory family '{family}' has unknown capability '{capability}'"
            )),
            Some(required) if required != *alpha => problems.push(format!(
                "rule inventory family '{family}' capability '{capability}' requires alpha obligation '{required}'"
            )),
            Some(_) => {}
        }
        if *compatibility != "current_only" && *compatibility != "lossless_carry" {
            problems.push(format!(
                "rule inventory family '{family}' has invalid compatibility '{compatibility}'"
            ));
        }
        if (*compatibility == "lossless_carry") != (*capability == "lossless_carry") {
            problems.push(format!(
                "rule inventory family '{family}' may use lossless compatibility only with lossless capability"
            ));
        }
        if target_module_path(root, target).is_none() {
            problems.push(format!(
                "rule inventory family '{family}' target module is absent: {target}"
            ));
        }
        if !repository_path(root, owner).is_some_and(|path| path.is_file()) {
            problems.push(format!(
                "rule inventory family '{family}' schema owner is absent: {owner}"
            ));
        }
        inspect_entrypoints(
            root,
            producers,
            "producer",
            number,
            &mut problems,
            &mut entrypoints,
            &sources,
        )?;
        inspect_entrypoints(
            root,
            mutators,
            "mutator",
            number,
            &mut problems,
            &mut entrypoints,
            &sources,
        )?;
        inspect_entrypoints(
            root,
            seam_cell,
            "canonical seam",
            number,
            &mut problems,
            &mut seams,
            &sources,
        )?;
        for field in &claim.fields {
            let rule_id = format!("{stem}.{field}.v1");
            let valid = rule_id.len() <= 128
                && rule_id.contains('.')
                && rule_id.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || matches!(byte, b'_' | b'.' | b'-')
                });
            if !valid {
                problems.push(format!(
                    "expanded rule id is invalid or overlong: {rule_id}"
                ));
            }
            if !expanded_rules.insert(rule_id.clone()) {
                problems.push(format!("expanded rule id is duplicated: {rule_id}"));
            }
        }
    }
    for family in claims.keys() {
        if !declared_families.contains(family) {
            problems.push(format!("claim family lacks a rule migration row: {family}"));
        }
    }
    let seam_cells: Vec<&str> = rule_rows
        .iter()
        .filter_map(|line| line.split('\t').nth(11))
        .collect();
    for required in CANONICAL_WRITES {
        if !seam_cells
            .iter()
            .any(|cell| cell.split(';').any(|entry| entry == required))
        {
            problems.push(format!(
                "canonical write seam is not inventoried: {required}"
            ));
        }
    }
    let bypass_lines = lines_of(
        &sources
            .slurp(&root.join("doctrine/production_genericity/conformance_bypass_inventory.tsv"))?,
    );
    let (bypass_header, bypass_rows) = bypass_lines
        .split_first()
        .map_or(("", &[][..]), |(header, rows)| (header.as_str(), rows));
    if bypass_header != BYPASS_HEADER {
        problems
            .push("conformance-bypass inventory header is not the schema-1 contract".to_string());
    }
    let mut declared_bypasses: BTreeSet<String> = BTreeSet::new();
    let mut bypass_entrypoints = 0usize;
    for (index, line) in bypass_rows.iter().enumerate() {
        let number = index + 2;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 6 || fields.iter().any(|field| field.is_empty()) {
            problems.push(format!(
                "conformance-bypass line {number} must contain six non-empty TSV fields"
            ));
            continue;
        }
        let (entrypoint, stage, disposition, leaf) = (fields[0], fields[1], fields[4], fields[5]);
        let key = format!("{entrypoint}\t{stage}");
        if !declared_bypasses.insert(key.clone()) {
            problems.push(format!("conformance-bypass inventory duplicates '{key}'"));
        }
        match migration_leaf(stage) {
            None => problems.push(format!(
                "conformance-bypass line {number} has unknown stage '{stage}'"
            )),
            Some(expected) if expected != leaf => problems.push(format!(
                "conformance-bypass line {number} has wrong migration leaf '{leaf}'"
            )),
            Some(_) => {}
        }
        if !FAIL_CLOSED_DISPOSITIONS.contains(&disposition) {
            problems.push(format!(
                "conformance-bypass line {number} lacks a fail-closed target disposition"
            ));
        }
        inspect_entrypoints(
            root,
            entrypoint,
            "conformance bypass",
            number,
            &mut problems,
            &mut bypass_entrypoints,
            &sources,
        )?;
    }
    if root
        .join("crates/specforge/src/commands/kg_bench.rs")
        .is_file()
    {
        for required in KNOWN_BYPASSES {
            if !declared_bypasses.contains(required) {
                problems.push(format!(
                    "known conformance bypass is unclassified: {required}"
                ));
            }
        }
    }
    Ok(Census {
        problems,
        families: declared_families.len(),
        rules: expanded_rules.len(),
        entrypoints,
        seams,
        bypasses: declared_bypasses.len(),
    })
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
    let stages = [
        ("source_ir", "source", "SourceIr"),
        ("evidence_ir", "evidence", "EvidenceIr"),
        ("semantic_ir", "semantic", "SemanticIr"),
        ("intent_ir", "intent", "IntentIr"),
        ("isf_adapter", "adapter", "AdapterArtifact"),
    ];
    let mut claim_rows = Vec::new();
    let mut rule_rows = Vec::new();
    for (stage, prefix, kind) in stages {
        let file = if stage == "isf_adapter" {
            "adapters.rs".to_string()
        } else {
            format!("{prefix}.rs")
        };
        let relative = format!("crates/specforge/src/ir/{file}");
        write_text(
            &fixture.join(&relative),
            &format!(
                "pub struct {kind} {{\n    pub alpha: String,\n}}\nimpl {kind} {{\n    pub fn build() {{}}\n    pub fn write_to_disk() {{}}\n}}\n"
            ),
        )?;
        let family = format!("{prefix}.family");
        claim_rows.push(format!(
            "{family}\t{stage}\talpha\tfixture\tintegrity_attestation\te.iv"
        ));
        let module = if stage == "isf_adapter" {
            "adapters"
        } else {
            prefix
        };
        rule_rows.push(
            [
                family.clone(),
                stage.to_string(),
                family,
                format!("{relative}#{kind}::build"),
                "-".to_string(),
                "universal_axiom".to_string(),
                "exact_identity_only".to_string(),
                "identity_graph_invariant".to_string(),
                "current_only".to_string(),
                format!("crate::ir::{module}"),
                relative.clone(),
                format!("{relative}#{kind}::write_to_disk"),
                migration_leaf(stage).expect("a stage").to_string(),
            ]
            .join("\t"),
        );
    }
    let validate = "crates/specforge/src/commands/validate.rs";
    write_text(
        &fixture.join(validate),
        "fn write_backannotated_artifact() {}\n",
    )?;
    rule_rows[0] = rule_rows[0].replacen(
        "\t6d.ii.e.iv.ii",
        &format!(";{validate}#write_backannotated_artifact\t6d.ii.e.iv.ii"),
        1,
    );
    write_text(
        &fixture.join("doctrine/production_genericity/claim_family_inventory.tsv"),
        &format!("{CLAIM_HEADER}\n{}\n", claim_rows.join("\n")),
    )?;
    write_text(
        &fixture.join("doctrine/production_genericity/rule_family_inventory.tsv"),
        &format!("{RULE_HEADER}\n{}\n", rule_rows.join("\n")),
    )?;
    write_text(
        &fixture.join("doctrine/production_genericity/conformance_bypass_inventory.tsv"),
        &format!("{BYPASS_HEADER}\n"),
    )
}

/// The script's seven cases on a fixture under `generated/`, removed whatever the verdict.
fn run_self_tests(root: &Path) -> Result<(), Death> {
    let fixture = root.join(format!(
        "generated/production-genericity-rule-tests.{}",
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
    let refused = |fixture: &Path, fragment: &str| -> Result<bool, Death> {
        Ok(inspect_rules(fixture)?
            .problems
            .iter()
            .any(|problem| problem.contains(fragment)))
    };
    self_test_fixture(fixture)?;
    let clean = inspect_rules(fixture)?;
    if !clean.problems.is_empty() {
        return Err(Death {
            message: format!("clean fixture failed: {}", clean.problems.join(" ")),
            code: 255,
        });
    }
    let rules = fixture.join("doctrine/production_genericity/rule_family_inventory.tsv");
    let original = fs::read_to_string(&rules).map_err(|error| die(&error.to_string()))?;
    let mut lines: Vec<&str> = original.split('\n').collect();
    if lines.last() == Some(&"") {
        lines.pop();
    }
    lines.remove(2);
    write_text(&rules, &format!("{}\n", lines.join("\n")))?;
    if !refused(fixture, "lacks a rule migration row")? {
        return Err(die("missing-family mutation was admitted"));
    }
    write_text(
        &rules,
        &original.replacen("#SourceIr::build", "#SourceIr::absent", 1),
    )?;
    if !refused(fixture, "symbol 'SourceIr::absent' is absent")? {
        return Err(die("missing-entrypoint mutation was admitted"));
    }
    write_text(
        &rules,
        &original.replacen("identity_graph_invariant", "target_safe_renaming", 1),
    )?;
    if !refused(fixture, "requires alpha obligation")? {
        return Err(die("capability-alpha mutation was admitted"));
    }
    write_text(
        &rules,
        &original.replacen("\tidentity_graph_invariant\t", "\t\t", 1),
    )?;
    if !refused(fixture, "13 non-empty TSV fields")? {
        return Err(die("missing-alpha-declaration mutation was admitted"));
    }
    write_text(
        &rules,
        &original.replacen(
            "source.family\tsource_ir\tsource.family",
            "source.family\tsource_ir\tevidence.family",
            1,
        ),
    )?;
    if !refused(fixture, "must use rule stem")? {
        return Err(die("rule-stem mutation was admitted"));
    }
    write_text(&rules, &original)?;
    write_text(
        &fixture.join("doctrine/production_genericity/conformance_bypass_inventory.tsv"),
        &format!(
            "{BYPASS_HEADER}\ncrates/specforge/src/ir/source.rs#SourceIr::build\tsource_ir\tfixture mutation\tfixture \
             sink\tallows canonical write\t6d.ii.e.iv.ii\n"
        ),
    )?;
    if !refused(fixture, "lacks a fail-closed target disposition")? {
        return Err(die("unsafe conformance-bypass disposition was admitted"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_rules_resolve_and_the_self_test_refuses_every_mutation() {
        let root = crate::repository_root();
        let census = inspect_rules(&root).ok().expect("a census");
        assert!(census.problems.is_empty(), "{:?}", census.problems);
        assert!(census.rules > 0 && census.entrypoints > 0 && census.seams > 0);
        assert!(run_self_tests(&root).is_ok());
    }

    #[test]
    fn an_uninventoried_canonical_write_seam_and_a_duplicate_entry_are_refused() {
        let fixture =
            crate::repository_root().join(".project-data/tmp/xtask-production-genericity-rules");
        let _ = fs::remove_dir_all(&fixture);
        self_test_fixture(&fixture).ok().expect("a fixture");
        let rules = fixture.join("doctrine/production_genericity/rule_family_inventory.tsv");
        let original = fs::read_to_string(&rules).expect("the rule inventory");
        let mutated = original
            .replacen(";crates/specforge/src/commands/validate.rs#write_backannotated_artifact", "", 1)
            .replacen(
                "crates/specforge/src/ir/source.rs#SourceIr::build\t",
                "crates/specforge/src/ir/source.rs#SourceIr::build;crates/specforge/src/ir/source.rs#SourceIr::build\t",
                1,
            );
        fs::write(&rules, mutated).expect("a mutated inventory");
        let census = inspect_rules(&fixture).ok().expect("a census");
        fs::remove_dir_all(&fixture).expect("cleanup");
        for expected in [
            "canonical write seam is not inventoried: crates/specforge/src/commands/validate.rs#write_backannotated_artifact",
            "rule inventory line 2 duplicates producer entry 'crates/specforge/src/ir/source.rs#SourceIr::build'",
        ] {
            assert!(
                census.problems.iter().any(|problem| problem == expected),
                "{expected}: {:?}",
                census.problems
            );
        }
    }

    #[test]
    fn entries_and_signatures_read_as_the_scripts_patterns_read_them() {
        assert_eq!(function_name("SourceIr::build"), "build");
        assert_eq!(function_name("SourceIr::"), "SourceIr");
        assert_eq!(function_name("a::::b"), "b");
        assert_eq!(
            split_entry("crates/specforge/src/ir/a.rs#A::b"),
            Some(("crates/specforge/src/ir/a.rs", "A::b"))
        );
        assert_eq!(split_entry("crates/specforge/src/ir/a.rs#1b"), None);
        assert_eq!(split_entry("crates/other/src/a.rs#b"), None);
        assert!(declares_function("pub fn build(", "build"));
        assert!(declares_function("fn build <T: Into<U>> (x: T)", "build"));
        assert!(declares_function("fn\nbuild\n(", "build"));
        assert!(!declares_function("fn build_all(", "build"));
        assert!(!declares_function("refn build(", "build"));
        assert!(!declares_function("fn build<T> where T: X (", "build"));
        assert!(!declares_function("fn build<T>;", "build"));
        assert!(
            is_rule_stem("source.envelope") && !is_rule_stem("source") && !is_rule_stem("Source.x")
        );
    }
}
