//! The 126 live-document lifecycle and control-plane checks, native since `RUSTDOC-AND-XTASK.15.u`.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use sha2::{Digest, Sha256};

use crate::pyjson::{self, Py};

use super::Output;

const EXPECTED_CHECKS: usize = 126;
const DIMENSIONS: [&str; 6] = [
    "files",
    "lines_each",
    "bytes_each",
    "lines_total",
    "bytes_total",
    "line_bytes_each",
];

struct Fixture {
    root: PathBuf,
    registry_meta: Py,
    surfaces: Vec<Py>,
    authority_meta: Py,
    authorities: Vec<Py>,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct VerifierStore {
    root: PathBuf,
    sources: BTreeMap<i32, PathBuf>,
}

impl VerifierStore {
    fn new(root: PathBuf) -> Result<Self, String> {
        if root.exists() {
            fs::remove_dir_all(&root).map_err(|error| {
                format!("remove stale verifier store '{}': {error}", root.display())
            })?;
        }
        fs::create_dir_all(&root)
            .map_err(|error| format!("create verifier store '{}': {error}", root.display()))?;
        Ok(Self {
            root,
            sources: BTreeMap::new(),
        })
    }

    fn source(&mut self, status: i32) -> Result<PathBuf, String> {
        if let Some(path) = self.sources.get(&status) {
            return Ok(path.clone());
        }
        let path = self.root.join(format!("exit{status}.sh"));
        write_text(
            &self.root,
            &format!("exit{status}.sh"),
            &format!("#!/usr/bin/env bash\nexit {status}\n"),
            Some(0o755),
        )?;
        self.sources.insert(status, path.clone());
        Ok(path)
    }
}

impl Drop for VerifierStore {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

fn integer(value: usize) -> Py {
    Py::Int(value.to_string())
}

fn object(pairs: Vec<(&str, Py)>) -> Py {
    Py::Dict(
        pairs
            .into_iter()
            .map(|(key, value)| (pyjson::py_str(key), value))
            .collect(),
    )
}

fn strings(values: &[&str]) -> Py {
    Py::List(values.iter().map(|value| text(value)).collect())
}

fn field<'a>(value: &'a Py, key: &str) -> Option<&'a Py> {
    pyjson::get(value, key)
}

fn field_mut<'a>(value: &'a mut Py, key: &str) -> Option<&'a mut Py> {
    if let Py::Dict(pairs) = value {
        pairs
            .iter_mut()
            .find(|(candidate, _)| *candidate == pyjson::py_str(key))
            .map(|(_, item)| item)
    } else {
        None
    }
}

fn nested_mut<'a>(value: &'a mut Py, path: &[&str]) -> &'a mut Py {
    let mut current = value;
    for key in path {
        current = field_mut(current, key)
            .unwrap_or_else(|| panic!("fixture field '{}' does not exist", key));
    }
    current
}

fn set_field(value: &mut Py, key: &str, item: Py) {
    if let Py::Dict(pairs) = value {
        let key = pyjson::py_str(key);
        if let Some((_, current)) = pairs.iter_mut().find(|(candidate, _)| *candidate == key) {
            *current = item;
        } else {
            pairs.push((key, item));
        }
    }
}

fn remove_field(value: &mut Py, key: &str) {
    if let Py::Dict(pairs) = value {
        let key = pyjson::py_str(key);
        pairs.retain(|(candidate, _)| *candidate != key);
    }
}

fn push_field(value: &mut Py, key: &str, item: Py) {
    match field_mut(value, key) {
        Some(Py::List(items)) => items.push(item),
        _ => panic!("fixture list '{key}' does not exist"),
    }
}

fn remove_existing(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "cannot replace fixture '{}': {error}",
            path.display()
        )),
    }
}

fn set_mode(path: &Path, mode: u32) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
            .map_err(|error| format!("cannot chmod fixture '{}': {error}", path.display()))?;
    }
    #[cfg(not(unix))]
    let _ = (path, mode);
    Ok(())
}

fn write_text(root: &Path, relative: &str, content: &str, mode: Option<u32>) -> Result<(), String> {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    remove_existing(&path)?;
    fs::write(&path, content)
        .map_err(|error| format!("cannot write fixture '{}': {error}", path.display()))?;
    if let Some(mode) = mode {
        set_mode(&path, mode)?;
    }
    Ok(())
}

fn write_verifier(
    store: &mut VerifierStore,
    root: &Path,
    relative: &str,
    status: i32,
) -> Result<(), String> {
    let source = store.source(status)?;
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    remove_existing(&path)?;
    if fs::hard_link(source, &path).is_err() {
        write_text(
            root,
            relative,
            &format!("#!/usr/bin/env bash\nexit {status}\n"),
            Some(0o755),
        )?;
    }
    Ok(())
}

fn read_bytes(root: &Path, relative: &str) -> Result<Vec<u8>, String> {
    let path = root.join(relative);
    fs::read(&path).map_err(|error| format!("cannot read fixture '{}': {error}", path.display()))
}

fn read_text(root: &Path, relative: &str) -> Result<String, String> {
    String::from_utf8(read_bytes(root, relative)?)
        .map_err(|error| format!("fixture '{relative}' is not UTF-8: {error}"))
}

#[derive(Clone, Copy, Debug, Default)]
struct Metrics([usize; 6]);

impl Metrics {
    fn get(&self, dimension: &str) -> usize {
        let index = DIMENSIONS
            .iter()
            .position(|candidate| *candidate == dimension)
            .expect("known fixture dimension");
        self.0[index]
    }

    fn set(&mut self, dimension: &str, value: usize) {
        let index = DIMENSIONS
            .iter()
            .position(|candidate| *candidate == dimension)
            .expect("known fixture dimension");
        self.0[index] = value;
    }

    fn json(&self) -> Py {
        object(
            DIMENSIONS
                .iter()
                .map(|dimension| (*dimension, integer(self.get(dimension))))
                .collect(),
        )
    }
}

fn dimensions_for(root: &Path, paths: &[&str]) -> Result<Metrics, String> {
    let mut metrics = Metrics::default();
    metrics.set("files", paths.len());
    for path in paths {
        let content = read_bytes(root, path)?;
        let bytes = content.len();
        let lines = content.iter().filter(|byte| **byte == b'\n').count();
        let max_line = content
            .split(|byte| *byte == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line).len())
            .max()
            .unwrap_or(0);
        metrics.set("lines_total", metrics.get("lines_total") + lines);
        metrics.set("bytes_total", metrics.get("bytes_total") + bytes);
        metrics.set("lines_each", metrics.get("lines_each").max(lines));
        metrics.set("bytes_each", metrics.get("bytes_each").max(bytes));
        metrics.set(
            "line_bytes_each",
            metrics.get("line_bytes_each").max(max_line),
        );
    }
    Ok(metrics)
}

fn generous_dimensions() -> Py {
    Metrics([16, 100, 4_096, 1_600, 65_536, 256]).json()
}

fn zero_dimensions() -> Py {
    Metrics::default().json()
}

fn base_surface(id: &str, targets: &[&str], locator: &str, lifecycle: &str, state: &str) -> Py {
    object(vec![
        ("surface_id", text(id)),
        ("targets", strings(targets)),
        ("locator", text(locator)),
        ("lifecycle", text(lifecycle)),
        ("state", text(state)),
        ("owner", text("fixture-maintainers")),
        ("health_targets", generous_dimensions()),
        ("enforcement_ceilings", generous_dimensions()),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
        ("verifier", text("builtin:fixture")),
    ])
}

fn encode(value: &Py) -> Result<String, String> {
    pyjson::dumps(
        value,
        &pyjson::Encoder {
            indent: None,
            sort_keys: true,
            ensure_ascii: false,
            item_separator: ",",
            key_separator: ":",
        },
    )
    .ok_or_else(|| "cannot encode fixture JSON".to_string())
}

fn save_jsonl(root: &Path, relative: &str, meta: &Py, records: &[Py]) -> Result<(), String> {
    let mut content = encode(meta)? + "\n";
    for record in records {
        content.push_str(&(encode(record)? + "\n"));
    }
    write_text(root, relative, &content, None)
}

fn save_registry(fixture: &Fixture) -> Result<(), String> {
    save_jsonl(
        &fixture.root,
        "control/surfaces.jsonl",
        &fixture.registry_meta,
        &fixture.surfaces,
    )
}

fn save_authorities(fixture: &Fixture) -> Result<(), String> {
    save_jsonl(
        &fixture.root,
        "control/authorities.jsonl",
        &fixture.authority_meta,
        &fixture.authorities,
    )
}

fn surface_mut<'a>(fixture: &'a mut Fixture, id: &str) -> &'a mut Py {
    fixture
        .surfaces
        .iter_mut()
        .find(|candidate| {
            field(candidate, "surface_id")
                .and_then(|value| match value {
                    Py::Str(text) => Some(pyjson::to_text(text)),
                    _ => None,
                })
                .is_some_and(|candidate| candidate == id)
        })
        .unwrap_or_else(|| panic!("fixture surface '{id}' does not exist"))
}

fn surface<'a>(fixture: &'a Fixture, id: &str) -> &'a Py {
    fixture
        .surfaces
        .iter()
        .find(|candidate| {
            field(candidate, "surface_id")
                .and_then(|value| match value {
                    Py::Str(text) => Some(pyjson::to_text(text)),
                    _ => None,
                })
                .is_some_and(|candidate| candidate == id)
        })
        .unwrap_or_else(|| panic!("fixture surface '{id}' does not exist"))
}

fn registry_meta(max_records: usize, max_bytes: usize) -> Py {
    object(vec![
        ("record_type", text("registry")),
        ("schema_version", integer(1)),
        ("max_records", integer(max_records)),
        ("max_bytes", integer(max_bytes)),
        ("max_record_bytes", integer(4_096)),
        ("max_array_items", integer(16)),
        ("max_scalar_bytes", integer(512)),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
    ])
}

fn sha256_hex(content: &[u8]) -> String {
    format!("{:x}", Sha256::digest(content))
}

fn new_fixture(root: &Path, verifiers: &mut VerifierStore) -> Result<Fixture, String> {
    let fixture = build_fixture(root, verifiers);
    if fixture.is_err() {
        let _ = fs::remove_dir_all(root);
    }
    fixture
}

fn build_fixture(root: &Path, verifiers: &mut VerifierStore) -> Result<Fixture, String> {
    if root.exists() {
        fs::remove_dir_all(root)
            .map_err(|error| format!("remove stale fixture '{}': {error}", root.display()))?;
    }
    fs::create_dir_all(root)
        .map_err(|error| format!("create fixture '{}': {error}", root.display()))?;
    write_text(
        root,
        "snapshot.md",
        "# Snapshot\n[External part](external/part.md)\n",
        None,
    )?;
    write_text(root, "ledger.md", "entry\n", None)?;
    write_text(root, "canonical/INDEX.md", "[Part](part.md)\n", None)?;
    write_text(root, "canonical/part.md", "# Canonical part\n", None)?;
    write_text(root, "query/part.md", "# Query part\n", None)?;
    write_text(root, "external/part.md", "# External-index part\n", None)?;
    write_text(
        root,
        "routed/INDEX.md",
        "# Routed landing\n[Collection README](README.md)\n[Titles 0001](../routed-titles/titles-0001.md)\n",
        None,
    )?;
    write_text(
        root,
        "routed/README.md",
        "# Routed collection guidance\n",
        None,
    )?;
    write_text(root, "routed/part.md", "# Routed member\n", None)?;
    write_text(
        root,
        "routed-titles/titles-0001.md",
        "# Routed titles part 0001\n[Routed member](../routed/part.md)\n",
        None,
    )?;
    write_text(root, "canonical/source.txt", "canonical input\n", None)?;
    write_text(root, "generated.md", "# Generated\n", None)?;
    write_text(root, "archive.md", "# Archive descriptor\n", None)?;
    write_text(
        root,
        "control/archive-manifest.json",
        "{\"fixture\":true}\n",
        None,
    )?;
    write_text(root, "frozen.md", "sealed\n", None)?;
    write_text(root, "book/SUMMARY.md", "[Part](part.md)\n", None)?;
    write_text(root, "book/part.md", "# Maintained part\n", None)?;
    write_verifier(verifiers, root, "scripts/freshness-ok.sh", 0)?;
    write_verifier(verifiers, root, "scripts/currency-ok.sh", 0)?;

    let ledger_metrics = dimensions_for(root, &["ledger.md"])?;
    let book_metrics = dimensions_for(root, &["book/SUMMARY.md", "book/part.md"])?;
    let frozen_sha = sha256_hex(&read_bytes(root, "frozen.md")?);

    let mut snapshot = base_surface(
        "snapshot",
        &["snapshot.md"],
        "file",
        "bounded_snapshot",
        "normal",
    );
    set_field(
        &mut snapshot,
        "currency",
        object(vec![
            ("status", text("enforced")),
            ("owner", text("fixture-maintainers")),
            ("verifier", text("scripts/currency-ok.sh")),
        ]),
    );

    let mut ledger = base_surface(
        "ledger",
        &["ledger.md"],
        "file",
        "rolling_ledger",
        "transition_debt",
    );
    set_field(&mut ledger, "baseline", ledger_metrics.json());
    set_field(
        &mut ledger,
        "transition",
        object(vec![
            ("owner", text("fixture-transition")),
            ("max_growth", zero_dimensions()),
        ]),
    );

    let mut canonical = base_surface(
        "canonical",
        &["canonical/*.md"],
        "collection",
        "partitioned_canonical",
        "normal",
    );
    set_field(&mut canonical, "index", text("canonical/INDEX.md"));
    set_field(
        &mut canonical,
        "index_contract",
        object(vec![
            ("kind", text("membership")),
            ("verifier", text("builtin:markdown_links")),
        ]),
    );

    let mut query = base_surface(
        "query",
        &["query/*.md"],
        "collection",
        "partitioned_canonical",
        "normal",
    );
    set_field(&mut query, "index", text("git:query"));
    set_field(
        &mut query,
        "index_contract",
        object(vec![
            ("kind", text("query")),
            ("verifier", text("builtin:registry_targets")),
        ]),
    );

    let mut external = base_surface(
        "external",
        &["external/*.md"],
        "collection",
        "partitioned_canonical",
        "normal",
    );
    set_field(&mut external, "index", text("snapshot.md"));
    set_field(
        &mut external,
        "index_contract",
        object(vec![
            ("kind", text("external_membership")),
            ("verifier", text("builtin:markdown_links")),
        ]),
    );

    let mut routed = base_surface(
        "routed",
        &["routed/*.md"],
        "collection",
        "partitioned_canonical",
        "normal",
    );
    set_field(&mut routed, "index", text("routed/INDEX.md"));
    set_field(
        &mut routed,
        "index_contract",
        object(vec![
            ("kind", text("routed_membership")),
            ("verifier", text("builtin:markdown_links")),
            ("route_surface", text("routed_titles")),
        ]),
    );

    let mut routed_titles = base_surface(
        "routed_titles",
        &["routed-titles/*.md"],
        "collection",
        "partitioned_canonical",
        "normal",
    );
    set_field(&mut routed_titles, "index", text("routed/INDEX.md"));
    set_field(
        &mut routed_titles,
        "index_contract",
        object(vec![
            ("kind", text("external_membership")),
            ("verifier", text("builtin:markdown_links")),
        ]),
    );

    let mut projection = base_surface(
        "projection",
        &["generated.md"],
        "file",
        "generated_projection",
        "normal",
    );
    set_field(
        &mut projection,
        "canonical_inputs",
        strings(&["canonical/source.txt"]),
    );
    set_field(
        &mut projection,
        "freshness_verifier",
        text("scripts/freshness-ok.sh"),
    );

    let mut archive = base_surface(
        "archive",
        &["archive.md"],
        "file",
        "archive_terminal",
        "terminal",
    );
    set_field(
        &mut archive,
        "archive_manifest",
        text("control/archive-manifest.json"),
    );
    set_field(
        &mut archive,
        "currency",
        object(vec![
            ("status", text("transition_debt")),
            ("owner", text("fixture-transition")),
            ("verifier", text("debt:fixture-archive-currentness")),
        ]),
    );

    let mut frozen = base_surface("frozen", &["frozen.md"], "file", "frozen_legacy", "frozen");
    set_field(&mut frozen, "sha256", text(&frozen_sha));

    let mut maintained = base_surface(
        "maintained",
        &["book/*.md"],
        "collection",
        "maintained_reference",
        "normal",
    );
    for key in ["files", "lines_total", "bytes_total"] {
        set_field(
            field_mut(&mut maintained, "health_targets").expect("health targets"),
            key,
            Py::None,
        );
        set_field(
            field_mut(&mut maintained, "enforcement_ceilings").expect("ceilings"),
            key,
            Py::None,
        );
    }
    set_field(&mut maintained, "index", text("book/SUMMARY.md"));
    set_field(
        &mut maintained,
        "index_contract",
        object(vec![
            ("kind", text("membership")),
            ("verifier", text("builtin:markdown_links")),
        ]),
    );
    set_field(
        &mut maintained,
        "reference_contract",
        object(vec![
            (
                "mandatory_read",
                object(vec![
                    ("path", text("book/SUMMARY.md")),
                    ("lines_ceiling", integer(8)),
                    ("bytes_ceiling", integer(1_024)),
                ]),
            ),
            ("max_navigation_depth", integer(1)),
            (
                "aggregate_change",
                object(vec![
                    ("authority_id", text("fixture-maintained-reference")),
                    ("owner", text("fixture-maintainers")),
                    (
                        "baseline",
                        object(vec![
                            ("files", integer(book_metrics.get("files"))),
                            ("lines_total", integer(book_metrics.get("lines_total"))),
                            ("bytes_total", integer(book_metrics.get("bytes_total"))),
                        ]),
                    ),
                    (
                        "delta",
                        object(vec![
                            ("files", integer(0)),
                            ("lines_total", integer(0)),
                            ("bytes_total", integer(0)),
                        ]),
                    ),
                    (
                        "rationale",
                        text("Exercise exact maintained-reference aggregate authority."),
                    ),
                ]),
            ),
        ]),
    );

    let fixture = Fixture {
        root: root.to_path_buf(),
        registry_meta: registry_meta(16, 32_768),
        surfaces: vec![
            snapshot,
            ledger,
            canonical,
            query,
            external,
            routed,
            routed_titles,
            projection,
            archive,
            frozen,
            maintained,
        ],
        authority_meta: registry_meta(8, 16_384),
        authorities: Vec::new(),
    };
    save_registry(&fixture)?;
    save_authorities(&fixture)?;
    write_text(
        root,
        "control/routes.tsv",
        "target\troute_kind\towner\tlifecycle\tpressure_control\tterminal\n\
snapshot.md\treader_navigation\tfixture\tmaintained_entrypoint\treview_on_behavior_change\tyes\n\
git:history\tauthor_overflow\tfixture\tappend_only_history\tquery_with_git_log\tyes\n",
        None,
    )?;
    Ok(fixture)
}

fn run_checker(
    repository_root: &Path,
    fixture: &Fixture,
    history: bool,
) -> Result<(i32, String), String> {
    let log_path = fixture.root.join("check-output.log");
    let log = File::create(&log_path)
        .map_err(|error| format!("cannot write checker log '{}': {error}", log_path.display()))?;
    let stdout = log
        .try_clone()
        .map_err(|error| format!("cannot clone checker log: {error}"))?;
    let stderr = log
        .try_clone()
        .map_err(|error| format!("cannot clone checker log: {error}"))?;
    let mut command = Command::new("perl");
    command
        .arg(repository_root.join("scripts/check_live_document_size.pl"))
        .arg("--root")
        .arg(&fixture.root)
        .arg("--registry")
        .arg("control/surfaces.jsonl")
        .arg("--routes")
        .arg("control/routes.tsv")
        .arg("--authorities")
        .arg("control/authorities.jsonl")
        .current_dir(repository_root)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    if !history {
        command.arg("--no-history");
    }
    let status = command
        .status()
        .map_err(|error| format!("cannot run live-document-size checker: {error}"))?;
    drop(log);
    let output = read_text(&fixture.root, "check-output.log")?;
    Ok((status.code().unwrap_or(1), output))
}

fn git(root: &Path, arguments: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .status()
        .map_err(|error| format!("fixture Git command failed: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "fixture Git command failed: git {}",
            arguments.join(" ")
        ))
    }
}

fn initialize_git_history(fixture: &Fixture) -> Result<(), String> {
    git(&fixture.root, &["init", "-q"])?;
    git(&fixture.root, &["config", "user.name", "SpecForge Fixture"])?;
    git(
        &fixture.root,
        &["config", "user.email", "fixture@specforge.invalid"],
    )?;
    git(
        &fixture.root,
        &["config", "core.hooksPath", ".git/no-hooks"],
    )?;
    git(&fixture.root, &["add", "."])?;
    git(&fixture.root, &["commit", "-q", "-m", "fixture baseline"])
}

fn stage_fixture_paths(fixture: &Fixture, path: &str) -> Result<(), String> {
    git(&fixture.root, &["add", "--", path])
}

fn commit_fixture(fixture: &Fixture, message: &str) -> Result<(), String> {
    git(&fixture.root, &["commit", "-q", "-m", message])
}

#[derive(Clone, Copy)]
enum RegexAtom {
    Literal(char),
    Any,
    Digit,
}

#[derive(Clone, Copy)]
struct RegexToken {
    atom: RegexAtom,
    minimum: usize,
    maximum: Option<usize>,
}

fn atom_matches(atom: RegexAtom, character: char) -> bool {
    match atom {
        RegexAtom::Literal(expected) => expected == character,
        RegexAtom::Any => character != '\n',
        RegexAtom::Digit => character.is_ascii_digit(),
    }
}

fn regex_tokens(pattern: &str) -> Vec<RegexToken> {
    let characters: Vec<char> = pattern.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < characters.len() {
        let atom = match characters[index] {
            '.' => {
                index += 1;
                RegexAtom::Any
            }
            '\\' if characters.get(index + 1) == Some(&'d') => {
                index += 2;
                RegexAtom::Digit
            }
            '\\' if characters.get(index + 1) == Some(&'Q') => {
                index += 2;
                while index < characters.len()
                    && !(characters[index] == '\\' && characters.get(index + 1) == Some(&'E'))
                {
                    tokens.push(RegexToken {
                        atom: RegexAtom::Literal(characters[index]),
                        minimum: 1,
                        maximum: Some(1),
                    });
                    index += 1;
                }
                index = (index + 2).min(characters.len());
                continue;
            }
            '\\' => {
                let literal = characters.get(index + 1).copied().unwrap_or('\\');
                index += 2;
                RegexAtom::Literal(literal)
            }
            literal => {
                index += 1;
                RegexAtom::Literal(literal)
            }
        };
        let (minimum, maximum) = match characters.get(index) {
            Some('*') => {
                index += 1;
                (0, None)
            }
            Some('+') => {
                index += 1;
                (1, None)
            }
            _ => (1, Some(1)),
        };
        tokens.push(RegexToken {
            atom,
            minimum,
            maximum,
        });
    }
    tokens
}

fn regex_here(tokens: &[RegexToken], text: &[char], token: usize, position: usize) -> bool {
    if token == tokens.len() {
        return true;
    }
    let current = tokens[token];
    let furthest = match current.maximum {
        Some(maximum) => (position + maximum).min(text.len()),
        None => text.len(),
    };
    let mut end = position;
    while end < furthest && atom_matches(current.atom, text[end]) {
        end += 1;
    }
    if end - position < current.minimum {
        return false;
    }
    loop {
        if regex_here(tokens, text, token + 1, end) {
            return true;
        }
        if end == position || end - position == current.minimum {
            return false;
        }
        end -= 1;
    }
}

fn regex_branch_matches(pattern: &str, output: &str) -> bool {
    let tokens = regex_tokens(pattern);
    let characters: Vec<char> = output.chars().collect();
    (0..=characters.len()).any(|start| regex_here(&tokens, &characters, 0, start))
}

fn matches_pattern(output: &str, pattern: &str) -> bool {
    if pattern == r"\A(?:(?!surface 'snapshot' lines_total).)*\z/s" {
        return !output.contains("surface 'snapshot' lines_total");
    }
    pattern
        .split('|')
        .any(|branch| regex_branch_matches(branch, output))
}

#[derive(Clone, Copy)]
enum Mutation {
    None,
    SnapshotMultipleFiles,
    LedgerUnownedGrowth,
    CanonicalStaleIndex,
    QueryWrongAuthority,
    QueryWrongVerifier,
    ExternalStaleIndex,
    ExternalMissingIndex,
    ExternalOffRootIndex,
    ExternalInsideIndex,
    RoutedMissingMember,
    RoutedMissingRouteMember,
    RoutedMissingRouteDeclaration,
    RoutedUnknownRouteSurface,
    RoutedSelfRoute,
    RoutedChainedRoute,
    RoutedExternalIndexProvesMembership,
    DirectMembershipRouteSurface,
    ProjectionFailingFreshness,
    ProjectionMissingInputs,
    ProjectionMissingFreshnessProof,
    ProjectionCollectionAccept,
    ProjectionCollectionStale,
    ArchiveNonterminal,
    ArchiveMissingManifest,
    FrozenDrift,
    MaintainedStaleAggregate,
    CurrencyFailure,
    TransitionAllowanceOverflow,
    RegistryNoMilestones,
    RegistryInvertedMilestones,
    RegistryRecordWarning,
    RegistryRecordRollover,
    RegistryBytePressure,
    RegistryRecordOverflow,
    RegistryByteOverflow,
    RegistryRawRecordOverflow,
    RegistryArrayOverflow,
    RegistryScalarOverflow,
    RegistrySchema,
    RegistryUnknownField,
    RegistryInvalidIdentifier,
    RegistryUnknownLocator,
    RegistryUnknownLifecycle,
    CoverageOrphan,
    CoverageDouble,
    CoverageFileMultiple,
    CanonicalAggregatePressure,
    CanonicalFileBoundPressure,
    ProjectionCardinalityExempt,
    EnumeratedCardinalityExempt,
    SingleFileNoAggregateWarning,
    SealedRollover,
    UnsealedRollover,
    EditedSeal,
    PrematureSeal,
    ForeignSeal,
    SealedCeiling,
    PressureDistance,
    CollectionLineAggregateTooSmall,
    CollectionByteAggregateTooSmall,
    TightHealthBand,
    FileLocatorReachabilityExempt,
    ExactComposition,
    BandComposition,
    CompositionMissingCeilingCount,
    CompositionUnknownCountBand,
    CompositionWrongLineSum,
    CompositionWrongFileCount,
    CompositionWrongLargest,
    CompositionSingleRole,
    CompositionNoRationale,
    CompositionRepeatedRole,
    CompositionUnknownField,
    CompositionMissingHealth,
    OffRootTarget,
    MembershipMissingIndex,
    RouteMissingTarget,
    IndependentCeiling(usize),
    CeilingMissingAuthority,
    CeilingExactAuthority,
    CeilingBankedAuthority,
    DiscoveredNoBand,
    DiscoveredPressure,
    DiscoveredNonRegistry,
    DiscoveredCapacity,
    DiscoveredIncoherentQuiet,
    DiscoveredCoherentQuiet,
    HeaderMissingAuthority,
    HeaderExactAuthority,
    HeaderMismatchedAuthority,
    HeaderBankedAuthority,
    AmbiguousAuthority,
    DiscoveredHeaderMissingAuthority,
    DiscoveredHeaderExactAuthority,
    ForeignRegistryAuthority,
    MaintainedReusedAuthority,
    MaintainedFreshAuthority,
    TransitionMovedBaseline,
    CardinalityDeclared,
    CardinalityMissing,
    CardinalityHalf,
    CardinalityResource,
    CardinalityUnknownRoute,
    CardinalityUnboundedRoute,
    CardinalitySelfRoute,
    CardinalityUntrackedAuthority,
    RoutedFloatingIndex,
    QuotedSurfaceNumber,
    QuotedSiblingNumber,
    NumericFieldString,
    DecimalIdentityAccepted,
}

struct Case {
    name: &'static str,
    success: bool,
    pattern: Option<&'static str>,
    mutation: Mutation,
    history: bool,
    absent: bool,
}

impl Case {
    const fn new(
        name: &'static str,
        success: bool,
        pattern: Option<&'static str>,
        mutation: Mutation,
    ) -> Self {
        Self {
            name,
            success,
            pattern,
            mutation,
            history: false,
            absent: false,
        }
    }

    const fn history(
        name: &'static str,
        success: bool,
        pattern: Option<&'static str>,
        mutation: Mutation,
    ) -> Self {
        Self {
            history: true,
            ..Self::new(name, success, pattern, mutation)
        }
    }

    const fn absent(name: &'static str, pattern: &'static str, mutation: Mutation) -> Self {
        Self {
            absent: true,
            ..Self::new(name, true, Some(pattern), mutation)
        }
    }

    const fn history_absent(name: &'static str, pattern: &'static str, mutation: Mutation) -> Self {
        Self {
            history: true,
            ..Self::absent(name, pattern, mutation)
        }
    }
}

const CASES: &[Case] = &[
    Case::new(
        "positive bounded_snapshot",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive rolling_ledger",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive partitioned_canonical membership and query",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive partitioned_canonical external membership",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive partitioned_canonical routed membership",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive generated_projection",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive archive_terminal",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive frozen_legacy",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "positive maintained_reference",
        true,
        Some("11 governed surfaces"),
        Mutation::None,
    ),
    Case::new(
        "bounded snapshot rejects multiple files",
        false,
        Some("bounded_snapshot must contain exactly one file"),
        Mutation::SnapshotMultipleFiles,
    ),
    Case::new(
        "rolling ledger rejects unowned live growth",
        false,
        Some("must remain transition_debt"),
        Mutation::LedgerUnownedGrowth,
    ),
    Case::new(
        "partition membership index rejects a stale member list",
        false,
        Some(r"does not link member 'canonical\/part\.md'"),
        Mutation::CanonicalStaleIndex,
    ),
    Case::new(
        "partition query index rejects the wrong query authority",
        false,
        Some("query index must be git:query"),
        Mutation::QueryWrongAuthority,
    ),
    Case::new(
        "partition query index rejects an uncontrolled verifier",
        false,
        Some("query index verifier must be builtin:registry_targets"),
        Mutation::QueryWrongVerifier,
    ),
    Case::new(
        "external membership index rejects a stale member list",
        false,
        Some(r"does not link member 'external\/part\.md'"),
        Mutation::ExternalStaleIndex,
    ),
    Case::new(
        "external membership rejects a missing index",
        false,
        Some(
            "external_membership index .* is not a classified Markdown surface|membership index .* is missing",
        ),
        Mutation::ExternalMissingIndex,
    ),
    Case::new(
        "external membership rejects an off-root index",
        false,
        Some("must be one safe repository-relative Markdown path"),
        Mutation::ExternalOffRootIndex,
    ),
    Case::new(
        "external membership rejects an index inside the member surface",
        false,
        Some("must be outside the surface"),
        Mutation::ExternalInsideIndex,
    ),
    Case::new(
        "routed membership rejects a member no declared hop reaches",
        false,
        Some(r"does not link member 'routed\/part\.md'"),
        Mutation::RoutedMissingMember,
    ),
    Case::new(
        "routed membership rejects an unrouted route member",
        false,
        Some(r"does not link route member 'routed-titles\/titles-0001\.md'"),
        Mutation::RoutedMissingRouteMember,
    ),
    Case::new(
        "routed membership rejects a missing route surface declaration",
        false,
        Some("routed_membership lacks a route_surface"),
        Mutation::RoutedMissingRouteDeclaration,
    ),
    Case::new(
        "routed membership rejects an unregistered route surface",
        false,
        Some("route_surface 'absent_titles' is not a registered surface"),
        Mutation::RoutedUnknownRouteSurface,
    ),
    Case::new(
        "routed membership rejects a self-referencing route surface",
        false,
        Some("route_surface must name another surface"),
        Mutation::RoutedSelfRoute,
    ),
    Case::new(
        "routed membership rejects a chained route surface",
        false,
        Some("route_surface 'routed_titles' must not itself route"),
        Mutation::RoutedChainedRoute,
    ),
    Case::new(
        "a routed membership index outside the surface still proves membership",
        false,
        Some(r"index 'snapshot\.md' does not link route member 'routed-titles\/titles-0001\.md'"),
        Mutation::RoutedExternalIndexProvesMembership,
    ),
    Case::new(
        "route_surface is rejected on a direct membership contract",
        false,
        Some("route_surface is only valid for routed_membership"),
        Mutation::DirectMembershipRouteSurface,
    ),
    Case::new(
        "generated projection rejects a failing freshness verifier",
        false,
        Some("freshness verifier .* failed"),
        Mutation::ProjectionFailingFreshness,
    ),
    Case::new(
        "generated projection rejects missing canonical inputs",
        false,
        Some("must name canonical_inputs"),
        Mutation::ProjectionMissingInputs,
    ),
    Case::new(
        "generated projection rejects a missing freshness proof",
        false,
        Some("freshness verifier .* missing or non-executable"),
        Mutation::ProjectionMissingFreshnessProof,
    ),
    Case::new(
        "generated projection collection accepts bounded indexed shards",
        true,
        Some("11 governed surfaces"),
        Mutation::ProjectionCollectionAccept,
    ),
    Case::new(
        "generated projection collection rejects a stale shard index",
        false,
        Some(r"does not link member 'generated-shard\.md'"),
        Mutation::ProjectionCollectionStale,
    ),
    Case::new(
        "archive terminal rejects a nonterminal state",
        false,
        Some("archive terminal must have state=terminal"),
        Mutation::ArchiveNonterminal,
    ),
    Case::new(
        "archive terminal rejects a missing manifest",
        false,
        Some("archive manifest .* is missing"),
        Mutation::ArchiveMissingManifest,
    ),
    Case::new(
        "frozen legacy rejects content drift",
        false,
        Some("frozen hash changed"),
        Mutation::FrozenDrift,
    ),
    Case::new(
        "maintained reference rejects stale aggregate authority",
        false,
        Some("aggregate authority is stale for lines_total"),
        Mutation::MaintainedStaleAggregate,
    ),
    Case::new(
        "currency verifier failure fails closed",
        false,
        Some("currency verifier .* failed"),
        Mutation::CurrencyFailure,
    ),
    Case::new(
        "transition debt rejects baseline plus allowance overflow",
        false,
        Some("exceeds immutable baseline plus transition allowance for lines_each"),
        Mutation::TransitionAllowanceOverflow,
    ),
    Case::new(
        "registry without milestones fails closed",
        false,
        Some("registry record must declare milestones"),
        Mutation::RegistryNoMilestones,
    ),
    Case::new(
        "registry rejects inverted milestones",
        false,
        Some(r"invalid warning\/rollover milestones"),
        Mutation::RegistryInvertedMilestones,
    ),
    Case::new(
        "registry record pressure warns below its hard stop",
        true,
        Some("surface registry records is at or above warning"),
        Mutation::RegistryRecordWarning,
    ),
    Case::new(
        "registry record pressure escalates to rollover",
        true,
        Some(
            r"surface registry records is at or above rollover \(91\.7%\) — 1 below its 12 max_records",
        ),
        Mutation::RegistryRecordRollover,
    ),
    Case::new(
        "registry byte pressure warns on its own dimension",
        true,
        Some("surface registry bytes is at or above rollover"),
        Mutation::RegistryBytePressure,
    ),
    Case::absent(
        "registry below its band reports no pressure",
        "surface registry records is at or above",
        Mutation::None,
    ),
    Case::new(
        "registry rejects record-count overflow",
        false,
        Some("more records than its declared max_records"),
        Mutation::RegistryRecordOverflow,
    ),
    Case::new(
        "registry rejects total-byte overflow",
        false,
        Some("exceeds its declared max_bytes"),
        Mutation::RegistryByteOverflow,
    ),
    Case::new(
        "registry rejects raw-record overflow",
        false,
        Some("record above its declared max_record_bytes"),
        Mutation::RegistryRawRecordOverflow,
    ),
    Case::new(
        "registry rejects array-cardinality overflow",
        false,
        Some("more than 1 array items"),
        Mutation::RegistryArrayOverflow,
    ),
    Case::new(
        "registry rejects scalar-byte overflow",
        false,
        Some("exceeds the declared scalar byte limit 8"),
        Mutation::RegistryScalarOverflow,
    ),
    Case::new(
        "registry rejects an unknown schema version",
        false,
        Some("schema_version must be 1"),
        Mutation::RegistrySchema,
    ),
    Case::new(
        "registry rejects unknown fields",
        false,
        Some("unknown field 'unreviewed_escape'"),
        Mutation::RegistryUnknownField,
    ),
    Case::new(
        "registry rejects invalid identifier shape",
        false,
        Some("invalid identifier shape"),
        Mutation::RegistryInvalidIdentifier,
    ),
    Case::new(
        "registry rejects unknown locator domain",
        false,
        Some("unknown locator 'somewhere'"),
        Mutation::RegistryUnknownLocator,
    ),
    Case::new(
        "registry rejects unknown lifecycle domain",
        false,
        Some("unknown lifecycle 'append_forever'"),
        Mutation::RegistryUnknownLifecycle,
    ),
    Case::new(
        "coverage rejects an unclassified Markdown file",
        false,
        Some(r"tracked Markdown 'orphan\.md' is not classified"),
        Mutation::CoverageOrphan,
    ),
    Case::new(
        "coverage rejects a double-classified Markdown file",
        false,
        Some(r"tracked Markdown 'snapshot\.md' is classified more than once"),
        Mutation::CoverageDouble,
    ),
    Case::new(
        "coverage rejects a file locator over several paths",
        false,
        Some("surface 'snapshot' declares a file locator but matches 2 paths"),
        Mutation::CoverageFileMultiple,
    ),
    Case::new(
        "a multi-file surface reports its aggregate pressure",
        true,
        Some("surface 'canonical' lines_total is at or above rollover"),
        Mutation::CanonicalAggregatePressure,
    ),
    Case::new(
        "a canonical collection at exactly its file bound reports pressure",
        true,
        Some(r"surface 'canonical' files is at or above rollover \(100\.0%\)"),
        Mutation::CanonicalFileBoundPressure,
    ),
    Case::absent(
        "a derived projection at exactly its file bound stays exempt",
        "surface 'projection' files is at or above",
        Mutation::ProjectionCardinalityExempt,
    ),
    Case::absent(
        "an enumerated surface whose bound equals its listed targets stays exempt",
        "surface 'snapshot' files is at or above",
        Mutation::EnumeratedCardinalityExempt,
    ),
    Case::new(
        "a single-file surface stays exempt from duplicate aggregate warnings",
        true,
        Some(r"\A(?:(?!surface 'snapshot' lines_total).)*\z/s"),
        Mutation::SingleFileNoAggregateWarning,
    ),
    Case::absent(
        "a sealed member at its rollover leaves the per-file milestones to the members that can grow",
        "surface 'canonical' lines_each is at or above",
        Mutation::SealedRollover,
    ),
    Case::new(
        "an unsealed member at its rollover still warns beside a sealed one",
        true,
        Some("surface 'canonical' lines_each is at or above warning"),
        Mutation::UnsealedRollover,
    ),
    Case::new(
        "an edited sealed member fails closed",
        false,
        Some(r"sealed member 'canonical/part\.md' changed; amend it with a new record"),
        Mutation::EditedSeal,
    ),
    Case::new(
        "a member below its per-file rollover cannot be sealed",
        false,
        Some(r"sealed member 'canonical/part\.md' has not reached its per-file rollover"),
        Mutation::PrematureSeal,
    ),
    Case::new(
        "a sealed member must be one of the surface's files",
        false,
        Some(r"sealed member 'query/part\.md' is not one of its files"),
        Mutation::ForeignSeal,
    ),
    Case::new(
        "a sealed member still counts toward every ceiling",
        false,
        Some("surface 'canonical' exceeds lines_each ceiling: 120 > 100"),
        Mutation::SealedCeiling,
    ),
    Case::new(
        "pressure names the distance to the hard ceiling, not only the health percentage",
        true,
        Some(r"lines_each is at or above rollover \(\d+\.\d%\) — \d+ below its 100 ceiling"),
        Mutation::PressureDistance,
    ),
    Case::new(
        "a collection aggregate below its own legal maximum is rejected",
        false,
        Some("lines_total 500 is below its own legal maximum 16 x 100"),
        Mutation::CollectionLineAggregateTooSmall,
    ),
    Case::new(
        "a collection byte aggregate below its own legal maximum is rejected",
        false,
        Some("bytes_total 16384 is below its own legal maximum 16 x 4096"),
        Mutation::CollectionByteAggregateTooSmall,
    ),
    Case::new(
        "a tight health band is rejected even when the ceiling is reachable",
        false,
        Some("health_targets lines_total 500 is below its own legal maximum"),
        Mutation::TightHealthBand,
    ),
    Case::new(
        "a file-locator surface is exempt from the collection reachability rule",
        true,
        None,
        Mutation::FileLocatorReachabilityExempt,
    ),
    Case::new(
        "an exact aggregate composition permits a heterogeneous collection",
        true,
        None,
        Mutation::ExactComposition,
    ),
    Case::new(
        "an exact aggregate composition permits different health and ceiling counts",
        true,
        None,
        Mutation::BandComposition,
    ),
    Case::new(
        "a band-specific aggregate composition requires both counts",
        false,
        Some("aggregate_composition member 'continuation' count lacks a positive 'ceiling'"),
        Mutation::CompositionMissingCeilingCount,
    ),
    Case::new(
        "a band-specific aggregate composition rejects an unknown count band",
        false,
        Some("aggregate_composition member 'continuation' count has unknown field 'future'"),
        Mutation::CompositionUnknownCountBand,
    ),
    Case::new(
        "an aggregate composition that does not sum to the declared total is rejected",
        false,
        Some("aggregate_composition ceiling lines sum to 1510, not lines_total 1509"),
        Mutation::CompositionWrongLineSum,
    ),
    Case::new(
        "an aggregate composition whose counts miss the file bound is rejected",
        false,
        Some("aggregate_composition counts sum to 15, not the enforcement_ceilings files bound 16"),
        Mutation::CompositionWrongFileCount,
    ),
    Case::new(
        "an aggregate composition whose largest member misses the per-file bound is rejected",
        false,
        Some("aggregate_composition largest ceiling member is 90 lines, not lines_each 100"),
        Mutation::CompositionWrongLargest,
    ),
    Case::new(
        "a single-role aggregate composition is rejected",
        false,
        Some("aggregate_composition members must list at least two member roles"),
        Mutation::CompositionSingleRole,
    ),
    Case::new(
        "an aggregate composition without a rationale is rejected",
        false,
        Some("aggregate_composition lacks a nonempty rationale"),
        Mutation::CompositionNoRationale,
    ),
    Case::new(
        "an aggregate composition with a repeated role is rejected",
        false,
        Some("aggregate_composition repeats member role 'member'"),
        Mutation::CompositionRepeatedRole,
    ),
    Case::new(
        "an aggregate composition with an unknown field is rejected",
        false,
        Some("aggregate_composition has unknown field 'note'"),
        Mutation::CompositionUnknownField,
    ),
    Case::new(
        "an aggregate composition member missing a band is rejected",
        false,
        Some("aggregate_composition member 'member' lacks health bounds"),
        Mutation::CompositionMissingHealth,
    ),
    Case::new(
        "coverage rejects an off-root target",
        false,
        Some("absolute, parent-relative, or off-root"),
        Mutation::OffRootTarget,
    ),
    Case::new(
        "membership rejects a missing index",
        false,
        Some("membership index .* is missing"),
        Mutation::MembershipMissingIndex,
    ),
    Case::new(
        "route integration rejects an unclassified Markdown target",
        false,
        Some(r"README route 'missing\.md' is not a classified Markdown surface"),
        Mutation::RouteMissingTarget,
    ),
    Case::new(
        "independent files ceiling fails closed",
        false,
        Some(r"exceeds \Qfiles\E ceiling"),
        Mutation::IndependentCeiling(0),
    ),
    Case::new(
        "independent lines_each ceiling fails closed",
        false,
        Some(r"exceeds \Qlines_each\E ceiling"),
        Mutation::IndependentCeiling(1),
    ),
    Case::new(
        "independent bytes_each ceiling fails closed",
        false,
        Some(r"exceeds \Qbytes_each\E ceiling"),
        Mutation::IndependentCeiling(2),
    ),
    Case::new(
        "independent lines_total ceiling fails closed",
        false,
        Some(r"exceeds \Qlines_total\E ceiling"),
        Mutation::IndependentCeiling(3),
    ),
    Case::new(
        "independent bytes_total ceiling fails closed",
        false,
        Some(r"exceeds \Qbytes_total\E ceiling"),
        Mutation::IndependentCeiling(4),
    ),
    Case::new(
        "independent line_bytes_each ceiling fails closed",
        false,
        Some(r"exceeds \Qline_bytes_each\E ceiling"),
        Mutation::IndependentCeiling(5),
    ),
    Case::history(
        "ceiling increase rejects missing exact authority",
        false,
        Some("increased ceiling dimensions without exact authority: bytes_each"),
        Mutation::CeilingMissingAuthority,
    ),
    Case::history(
        "ceiling increase accepts one exact fresh authority",
        true,
        Some("11 governed surfaces"),
        Mutation::CeilingExactAuthority,
    ),
    Case::history(
        "ceiling history rejects unused banked authority",
        false,
        Some("unused or banked ceiling-increase authority"),
        Mutation::CeilingBankedAuthority,
    ),
    Case::history(
        "a discovered bounded registry without a band fails closed",
        false,
        Some(r"registry 'doctrine\/fixture\/extra\.jsonl' registry record must declare milestones"),
        Mutation::DiscoveredNoBand,
    ),
    Case::history(
        "a discovered bounded registry reports its own pressure",
        true,
        Some(r"registry 'doctrine\/fixture\/extra\.jsonl' records is at or above rollover"),
        Mutation::DiscoveredPressure,
    ),
    Case::history(
        "a discovered file that is not a registry is left alone",
        true,
        Some("11 governed surfaces"),
        Mutation::DiscoveredNonRegistry,
    ),
    Case::history(
        "a discovered registry whose byte bound funds fewer records than declared reports it",
        true,
        Some(
            r"registry 'doctrine\/fixture\/extra\.jsonl' byte bound funds 3 records at its real mean of 1000 bytes, below the 8 its max_records declares",
        ),
        Mutation::DiscoveredCapacity,
    ),
    Case::history_absent(
        "incoherent bounds alone do not report while the real records stay small",
        "byte bound funds",
        Mutation::DiscoveredIncoherentQuiet,
    ),
    Case::history_absent(
        "a coherent registry holding the same records is silent",
        "byte bound funds",
        Mutation::DiscoveredCoherentQuiet,
    ),
    Case::history(
        "registry header increase rejects missing exact authority",
        false,
        Some(
            r"registry 'control\/surfaces\.jsonl' increased header bounds without exact authority: max_records",
        ),
        Mutation::HeaderMissingAuthority,
    ),
    Case::history(
        "registry header increase accepts one exact fresh authority",
        true,
        Some("11 governed surfaces"),
        Mutation::HeaderExactAuthority,
    ),
    Case::history(
        "registry header authority must match the exact new bounds",
        false,
        Some("increased header bounds without exact authority"),
        Mutation::HeaderMismatchedAuthority,
    ),
    Case::history(
        "registry header authority is refused as banked when unused",
        false,
        Some("unused or banked ceiling-increase authority"),
        Mutation::HeaderBankedAuthority,
    ),
    Case::new(
        "an authority naming both a surface and a registry is refused",
        false,
        Some("must name exactly one of surface_id or registry_id"),
        Mutation::AmbiguousAuthority,
    ),
    Case::history(
        "a discovered registry header increase rejects missing exact authority",
        false,
        Some(
            r"registry 'doctrine\/fixture\/extra\.jsonl' increased header bounds without exact authority: max_records",
        ),
        Mutation::DiscoveredHeaderMissingAuthority,
    ),
    Case::history(
        "a discovered registry header increase accepts its exact authority",
        true,
        Some("11 governed surfaces"),
        Mutation::DiscoveredHeaderExactAuthority,
    ),
    Case::new(
        "a registry authority naming another file is refused",
        false,
        Some("does not name a bounded registry this checker reads"),
        Mutation::ForeignRegistryAuthority,
    ),
    Case::history(
        "maintained reference rejects reused aggregate authority",
        false,
        Some("reused maintained-reference authority across aggregate change"),
        Mutation::MaintainedReusedAuthority,
    ),
    Case::history(
        "maintained reference accepts fresh exact aggregate authority",
        true,
        Some("11 governed surfaces"),
        Mutation::MaintainedFreshAuthority,
    ),
    Case::history(
        "transition debt rejects a moved immutable baseline",
        false,
        Some("moved its immutable transition baseline"),
        Mutation::TransitionMovedBaseline,
    ),
    Case::new(
        "a declared cardinality exemption with a bounded external route is accepted",
        true,
        None,
        Mutation::CardinalityDeclared,
    ),
    Case::new(
        "a null file count without a declared exemption is rejected",
        false,
        Some("nulls files without a declared cardinality exemption"),
        Mutation::CardinalityMissing,
    ),
    Case::new(
        "a half-declared cardinality exemption is rejected",
        false,
        Some("must null files in both bands together"),
        Mutation::CardinalityHalf,
    ),
    Case::new(
        "a cardinality exemption may not unbound a resource dimension",
        false,
        Some("may not unbound resource dimension 'lines_total'"),
        Mutation::CardinalityResource,
    ),
    Case::new(
        "a cardinality exemption routed to an unregistered surface is rejected",
        false,
        Some("route surface 'absent_route' is not registered"),
        Mutation::CardinalityUnknownRoute,
    ),
    Case::new(
        "a cardinality exemption routed to an unbounded surface is rejected",
        false,
        Some("route 'maintained' is unbounded in 'files'"),
        Mutation::CardinalityUnboundedRoute,
    ),
    Case::new(
        "a cardinality exemption may not route to the exempt surface itself",
        false,
        Some("route must be a different registered surface"),
        Mutation::CardinalitySelfRoute,
    ),
    Case::new(
        "a cardinality exemption with an untracked authority is rejected",
        false,
        Some(r"authority 'routed\/absent\.md' is not a tracked repository file"),
        Mutation::CardinalityUntrackedAuthority,
    ),
    Case::new(
        "a routed membership index outside the surface must itself be classified",
        false,
        Some(
            r"routed_membership index 'unclassified\/INDEX\.md' is outside the surface and is not a classified Markdown surface",
        ),
        Mutation::RoutedFloatingIndex,
    ),
    Case::new(
        "a quoted numeral in the surface registry is refused",
        false,
        Some(
            "field 'bytes_each' holds the string \"1024\"; numeric evidence must be a JSON number",
        ),
        Mutation::QuotedSurfaceNumber,
    ),
    Case::new(
        "a quoted numeral in a sibling registry is refused",
        false,
        Some(r#"control\/sibling\.jsonl line 1: field 'lines' holds the string \"12\""#),
        Mutation::QuotedSiblingNumber,
    ),
    Case::new(
        "a non-numeral string in a field that is numeric elsewhere is refused",
        false,
        Some(
            r#"field 'lines' holds the string \"12 lines\", but it is a JSON number at control\/sibling\.jsonl line 1"#,
        ),
        Mutation::NumericFieldString,
    ),
    Case::new(
        "an all-decimal identity and ordinary text are accepted",
        true,
        Some("11 governed surfaces"),
        Mutation::DecimalIdentityAccepted,
    ),
];

fn set_band_values(surface: &mut Py, band: &str, values: [usize; 5]) {
    let target = field_mut(surface, band).expect("fixture dimension band");
    for (key, value) in [
        "files",
        "lines_each",
        "bytes_each",
        "lines_total",
        "bytes_total",
    ]
    .into_iter()
    .zip(values)
    {
        set_field(target, key, integer(value));
    }
}

fn composition_member(
    role: &str,
    count: Py,
    health: (usize, usize),
    ceiling: (usize, usize),
) -> Py {
    object(vec![
        ("role", text(role)),
        ("count", count),
        (
            "health",
            object(vec![
                ("lines", integer(health.0)),
                ("bytes", integer(health.1)),
            ]),
        ),
        (
            "ceiling",
            object(vec![
                ("lines", integer(ceiling.0)),
                ("bytes", integer(ceiling.1)),
            ]),
        ),
    ])
}

fn standard_composition(rationale: &str, members: Vec<Py>) -> Py {
    object(vec![
        ("rationale", text(rationale)),
        ("members", Py::List(members)),
    ])
}

fn seal_canonical_part(
    fixture: &mut Fixture,
    path: &str,
    sha256: Option<String>,
) -> Result<(), String> {
    let content = read_bytes(&fixture.root, path)?;
    let sha256 = sha256.unwrap_or_else(|| sha256_hex(&content));
    set_field(
        surface_mut(fixture, "canonical"),
        "sealed_members",
        Py::List(vec![object(vec![
            ("path", text(path)),
            ("sha256", text(&sha256)),
            ("owner", text("FIXTURE.1")),
        ])]),
    );
    Ok(())
}

fn padded_registry_record(id: &str, bytes: usize) -> Result<String, String> {
    let prefix = format!("{{\"record_type\":\"member\",\"id\":\"{id}\",\"pad\":\"");
    let suffix = "\"}\n";
    let Some(fill) = bytes.checked_sub(prefix.len() + suffix.len()) else {
        return Err(format!(
            "padded fixture record '{id}' cannot reach {bytes} bytes"
        ));
    };
    Ok(prefix + &"p".repeat(fill) + suffix)
}

fn ceiling_authority(surface_id: &str, work_unit: &str, rationale: &str, old: Py, new: Py) -> Py {
    object(vec![
        ("record_type", text("increase")),
        ("surface_id", text(surface_id)),
        ("work_unit", text(work_unit)),
        ("owner", text("fixture-maintainers")),
        ("rationale", text(rationale)),
        ("old", old),
        ("new", new),
    ])
}

fn registry_authority(registry_id: &str, work_unit: &str, rationale: &str, old: Py, new: Py) -> Py {
    object(vec![
        ("record_type", text("increase")),
        ("registry_id", text(registry_id)),
        ("work_unit", text(work_unit)),
        ("owner", text("fixture-maintainers")),
        ("rationale", text(rationale)),
        ("old", old),
        ("new", new),
    ])
}

fn header_bounds(max_records: usize, max_bytes: usize) -> Py {
    object(vec![
        ("max_records", integer(max_records)),
        ("max_bytes", integer(max_bytes)),
    ])
}

fn declare_exemption(fixture: &mut Fixture, authority: &str, route_surface_id: &str) {
    let routed_titles = surface_mut(fixture, "routed_titles");
    set_field(
        field_mut(routed_titles, "health_targets").expect("health targets"),
        "files",
        Py::None,
    );
    set_field(
        field_mut(routed_titles, "enforcement_ceilings").expect("ceilings"),
        "files",
        Py::None,
    );
    set_field(
        routed_titles,
        "cardinality_exemption",
        object(vec![
            ("authority", text(authority)),
            ("work_unit", text("FIXTURE-EXEMPTION.0")),
            ("route_surface_id", text(route_surface_id)),
            ("rationale", text("fixture exemption")),
        ]),
    );
}

fn py_usize(value: &Py) -> usize {
    match value {
        Py::Int(value) => value.parse().expect("fixture nonnegative integer"),
        _ => panic!("fixture integer expected"),
    }
}

fn maintained_aggregate_change(fixture: &mut Fixture, fresh: bool) -> Result<(), String> {
    write_text(
        &fixture.root,
        "book/part.md",
        "# Maintained part\nextra\n",
        None,
    )?;
    let metrics = dimensions_for(&fixture.root, &["book/SUMMARY.md", "book/part.md"])?;
    let maintained = surface_mut(fixture, "maintained");
    let change = nested_mut(maintained, &["reference_contract", "aggregate_change"]);
    if fresh {
        set_field(
            change,
            "authority_id",
            text("fixture-maintained-reference-next"),
        );
    }
    let baseline_lines = py_usize(nested_mut(change, &["baseline", "lines_total"]));
    let baseline_bytes = py_usize(nested_mut(change, &["baseline", "bytes_total"]));
    set_field(
        nested_mut(change, &["delta"]),
        "lines_total",
        integer(metrics.get("lines_total") - baseline_lines),
    );
    set_field(
        nested_mut(change, &["delta"]),
        "bytes_total",
        integer(metrics.get("bytes_total") - baseline_bytes),
    );
    Ok(())
}

fn mutate(
    fixture: &mut Fixture,
    verifiers: &mut VerifierStore,
    mutation: Mutation,
) -> Result<(), String> {
    match mutation {
        Mutation::None => {}
        Mutation::SnapshotMultipleFiles => {
            write_text(&fixture.root, "snapshot-extra.md", "# Extra\n", None)?;
            push_field(
                surface_mut(fixture, "snapshot"),
                "targets",
                text("snapshot-extra.md"),
            );
        }
        Mutation::LedgerUnownedGrowth => {
            let ledger = surface_mut(fixture, "ledger");
            set_field(ledger, "state", text("normal"));
            remove_field(ledger, "baseline");
            remove_field(ledger, "transition");
        }
        Mutation::CanonicalStaleIndex => {
            write_text(&fixture.root, "canonical/INDEX.md", "# Empty index\n", None)?;
        }
        Mutation::QueryWrongAuthority => {
            set_field(
                surface_mut(fixture, "query"),
                "index",
                text("query:uncontrolled"),
            );
        }
        Mutation::QueryWrongVerifier => {
            set_field(
                nested_mut(surface_mut(fixture, "query"), &["index_contract"]),
                "verifier",
                text("builtin:uncontrolled"),
            );
        }
        Mutation::ExternalStaleIndex => {
            write_text(
                &fixture.root,
                "snapshot.md",
                "# Empty external index\n",
                None,
            )?;
        }
        Mutation::ExternalMissingIndex => {
            set_field(
                surface_mut(fixture, "external"),
                "index",
                text("missing.md"),
            );
        }
        Mutation::ExternalOffRootIndex => {
            set_field(
                surface_mut(fixture, "external"),
                "index",
                text("../escape.md"),
            );
        }
        Mutation::ExternalInsideIndex => {
            set_field(
                surface_mut(fixture, "external"),
                "index",
                text("external/part.md"),
            );
        }
        Mutation::RoutedMissingMember => {
            write_text(
                &fixture.root,
                "routed-titles/titles-0001.md",
                "# Routed titles part 0001\n",
                None,
            )?;
        }
        Mutation::RoutedMissingRouteMember => {
            write_text(
                &fixture.root,
                "routed/INDEX.md",
                "# Routed landing\n[Collection README](README.md)\n",
                None,
            )?;
        }
        Mutation::RoutedMissingRouteDeclaration => {
            remove_field(
                nested_mut(surface_mut(fixture, "routed"), &["index_contract"]),
                "route_surface",
            );
        }
        Mutation::RoutedUnknownRouteSurface => {
            set_field(
                nested_mut(surface_mut(fixture, "routed"), &["index_contract"]),
                "route_surface",
                text("absent_titles"),
            );
        }
        Mutation::RoutedSelfRoute => {
            set_field(
                nested_mut(surface_mut(fixture, "routed"), &["index_contract"]),
                "route_surface",
                text("routed"),
            );
        }
        Mutation::RoutedChainedRoute => {
            set_field(
                surface_mut(fixture, "routed_titles"),
                "index_contract",
                object(vec![
                    ("kind", text("routed_membership")),
                    ("verifier", text("builtin:markdown_links")),
                    ("route_surface", text("routed")),
                ]),
            );
        }
        Mutation::RoutedExternalIndexProvesMembership => {
            set_field(surface_mut(fixture, "routed"), "index", text("snapshot.md"));
        }
        Mutation::DirectMembershipRouteSurface => {
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["index_contract"]),
                "route_surface",
                text("routed_titles"),
            );
        }
        Mutation::ProjectionFailingFreshness => {
            write_verifier(verifiers, &fixture.root, "scripts/freshness-ok.sh", 1)?;
        }
        Mutation::ProjectionMissingInputs => {
            remove_field(surface_mut(fixture, "projection"), "canonical_inputs");
        }
        Mutation::ProjectionMissingFreshnessProof => {
            write_text(
                &fixture.root,
                "scripts/freshness-ok.sh",
                "#!/usr/bin/env bash\nexit 0\n",
                Some(0o644),
            )?;
        }
        Mutation::ProjectionCollectionAccept => {
            write_text(
                &fixture.root,
                "generated.md",
                "# Generated index\n[Shard](generated-shard.md)\n",
                None,
            )?;
            write_text(
                &fixture.root,
                "generated-shard.md",
                "# Generated shard\n",
                None,
            )?;
            let projection = surface_mut(fixture, "projection");
            set_field(projection, "targets", strings(&["generated*.md"]));
            set_field(projection, "locator", text("collection"));
            set_field(projection, "index", text("generated.md"));
            set_field(
                projection,
                "index_contract",
                object(vec![
                    ("kind", text("membership")),
                    ("verifier", text("builtin:markdown_links")),
                ]),
            );
        }
        Mutation::ProjectionCollectionStale => {
            write_text(
                &fixture.root,
                "generated-shard.md",
                "# Generated shard\n",
                None,
            )?;
            let projection = surface_mut(fixture, "projection");
            set_field(projection, "targets", strings(&["generated*.md"]));
            set_field(projection, "locator", text("collection"));
            set_field(projection, "index", text("generated.md"));
            set_field(
                projection,
                "index_contract",
                object(vec![
                    ("kind", text("membership")),
                    ("verifier", text("builtin:markdown_links")),
                ]),
            );
        }
        Mutation::ArchiveNonterminal => {
            set_field(surface_mut(fixture, "archive"), "state", text("normal"));
        }
        Mutation::ArchiveMissingManifest => {
            set_field(
                surface_mut(fixture, "archive"),
                "archive_manifest",
                text("control/missing-manifest.json"),
            );
        }
        Mutation::FrozenDrift => {
            write_text(&fixture.root, "frozen.md", "changed\n", None)?;
        }
        Mutation::MaintainedStaleAggregate => {
            set_field(
                nested_mut(
                    surface_mut(fixture, "maintained"),
                    &["reference_contract", "aggregate_change", "delta"],
                ),
                "lines_total",
                integer(1),
            );
        }
        Mutation::CurrencyFailure => {
            write_verifier(verifiers, &fixture.root, "scripts/currency-ok.sh", 1)?;
        }
        Mutation::TransitionAllowanceOverflow => {
            write_text(&fixture.root, "ledger.md", "entry\nsecond\n", None)?;
        }
        Mutation::RegistryNoMilestones => {
            remove_field(&mut fixture.registry_meta, "milestones");
        }
        Mutation::RegistryInvertedMilestones => {
            set_field(
                &mut fixture.registry_meta,
                "milestones",
                object(vec![
                    ("warning_pct", integer(90)),
                    ("rollover_pct", integer(80)),
                ]),
            );
        }
        Mutation::RegistryRecordWarning => {
            set_field(&mut fixture.registry_meta, "max_records", integer(13));
        }
        Mutation::RegistryRecordRollover => {
            set_field(&mut fixture.registry_meta, "max_records", integer(12));
        }
        Mutation::RegistryBytePressure => {
            save_registry(fixture)?;
            let written = fs::metadata(fixture.root.join("control/surfaces.jsonl"))
                .map_err(|error| format!("cannot size fixture registry: {error}"))?
                .len() as usize;
            set_field(
                &mut fixture.registry_meta,
                "max_bytes",
                integer(written + 64),
            );
        }
        Mutation::RegistryRecordOverflow => {
            set_field(&mut fixture.registry_meta, "max_records", integer(7));
        }
        Mutation::RegistryByteOverflow => {
            set_field(&mut fixture.registry_meta, "max_bytes", integer(512));
        }
        Mutation::RegistryRawRecordOverflow => {
            set_field(&mut fixture.registry_meta, "max_record_bytes", integer(256));
        }
        Mutation::RegistryArrayOverflow => {
            set_field(&mut fixture.registry_meta, "max_array_items", integer(1));
            push_field(
                surface_mut(fixture, "snapshot"),
                "targets",
                text("snapshot.md"),
            );
        }
        Mutation::RegistryScalarOverflow => {
            set_field(&mut fixture.registry_meta, "max_scalar_bytes", integer(8));
        }
        Mutation::RegistrySchema => {
            set_field(&mut fixture.registry_meta, "schema_version", integer(2));
        }
        Mutation::RegistryUnknownField => {
            set_field(
                surface_mut(fixture, "snapshot"),
                "unreviewed_escape",
                integer(1),
            );
        }
        Mutation::RegistryInvalidIdentifier => {
            set_field(
                surface_mut(fixture, "snapshot"),
                "surface_id",
                text("Invalid Surface"),
            );
        }
        Mutation::RegistryUnknownLocator => {
            set_field(
                surface_mut(fixture, "snapshot"),
                "locator",
                text("somewhere"),
            );
        }
        Mutation::RegistryUnknownLifecycle => {
            set_field(
                surface_mut(fixture, "snapshot"),
                "lifecycle",
                text("append_forever"),
            );
        }
        Mutation::CoverageOrphan => {
            write_text(&fixture.root, "orphan.md", "# Orphan\n", None)?;
        }
        Mutation::CoverageDouble => {
            push_field(
                surface_mut(fixture, "ledger"),
                "targets",
                text("snapshot.md"),
            );
        }
        Mutation::CoverageFileMultiple => {
            write_text(
                &fixture.root,
                "snapshot-extra.md",
                "# Extra\n[External part](external/part.md)\n",
                None,
            )?;
            push_field(
                surface_mut(fixture, "snapshot"),
                "targets",
                text("snapshot-extra.md"),
            );
        }
        Mutation::CanonicalAggregatePressure => {
            write_text(
                &fixture.root,
                "canonical/part.md",
                &"# Canonical part\n".repeat(90),
                None,
            )?;
            set_band_values(
                surface_mut(fixture, "canonical"),
                "health_targets",
                [2, 50, 4_096, 100, 8_192],
            );
        }
        Mutation::CanonicalFileBoundPressure => {
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["health_targets"]),
                "files",
                integer(2),
            );
        }
        Mutation::ProjectionCardinalityExempt => {
            let projection = surface_mut(fixture, "projection");
            set_field(projection, "targets", strings(&["generated*.md"]));
            set_field(
                field_mut(projection, "health_targets").expect("health targets"),
                "files",
                integer(1),
            );
        }
        Mutation::EnumeratedCardinalityExempt => {
            set_field(
                nested_mut(surface_mut(fixture, "snapshot"), &["health_targets"]),
                "files",
                integer(1),
            );
        }
        Mutation::SingleFileNoAggregateWarning => {
            set_field(
                nested_mut(surface_mut(fixture, "snapshot"), &["health_targets"]),
                "lines_total",
                integer(2),
            );
        }
        Mutation::SealedRollover => {
            write_text(
                &fixture.root,
                "canonical/part.md",
                &"# Canonical part\n".repeat(95),
                None,
            )?;
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["health_targets"]),
                "lines_each",
                integer(50),
            );
            seal_canonical_part(fixture, "canonical/part.md", None)?;
        }
        Mutation::UnsealedRollover => {
            write_text(
                &fixture.root,
                "canonical/part.md",
                &"# Canonical part\n".repeat(95),
                None,
            )?;
            write_text(
                &fixture.root,
                "canonical/INDEX.md",
                &"[Part](part.md)\n".repeat(42),
                None,
            )?;
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["health_targets"]),
                "lines_each",
                integer(50),
            );
            seal_canonical_part(fixture, "canonical/part.md", None)?;
        }
        Mutation::EditedSeal => {
            write_text(
                &fixture.root,
                "canonical/part.md",
                &"# Canonical part\n".repeat(95),
                None,
            )?;
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["health_targets"]),
                "lines_each",
                integer(50),
            );
            seal_canonical_part(
                fixture,
                "canonical/part.md",
                Some(sha256_hex(b"an earlier content\n")),
            )?;
        }
        Mutation::PrematureSeal => {
            seal_canonical_part(fixture, "canonical/part.md", None)?;
        }
        Mutation::ForeignSeal => {
            seal_canonical_part(fixture, "query/part.md", None)?;
        }
        Mutation::SealedCeiling => {
            write_text(
                &fixture.root,
                "canonical/part.md",
                &"# Canonical part\n".repeat(120),
                None,
            )?;
            seal_canonical_part(fixture, "canonical/part.md", None)?;
        }
        Mutation::PressureDistance => {
            write_text(
                &fixture.root,
                "canonical/part.md",
                &"# Canonical part\n".repeat(95),
                None,
            )?;
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["health_targets"]),
                "lines_each",
                integer(50),
            );
        }
        Mutation::CollectionLineAggregateTooSmall => {
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["enforcement_ceilings"]),
                "lines_total",
                integer(500),
            );
        }
        Mutation::CollectionByteAggregateTooSmall => {
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["enforcement_ceilings"]),
                "bytes_total",
                integer(16_384),
            );
        }
        Mutation::TightHealthBand => {
            set_field(
                nested_mut(surface_mut(fixture, "canonical"), &["health_targets"]),
                "lines_total",
                integer(500),
            );
        }
        Mutation::FileLocatorReachabilityExempt => {
            let snapshot = surface_mut(fixture, "snapshot");
            set_field(
                field_mut(snapshot, "enforcement_ceilings").expect("ceilings"),
                "lines_total",
                integer(5),
            );
            set_field(
                field_mut(snapshot, "health_targets").expect("health targets"),
                "lines_total",
                integer(5),
            );
        }
        Mutation::ExactComposition => {
            let canonical = surface_mut(fixture, "canonical");
            for band in ["health_targets", "enforcement_ceilings"] {
                set_band_values(canonical, band, [16, 100, 4_096, 1_510, 61_952]);
            }
            set_field(
                canonical,
                "aggregate_composition",
                standard_composition(
                    "one small index plus fifteen full members",
                    vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        composition_member("member", integer(15), (100, 4_096), (100, 4_096)),
                    ],
                ),
            );
        }
        Mutation::BandComposition => {
            let canonical = surface_mut(fixture, "canonical");
            set_band_values(
                canonical,
                "health_targets",
                [16, 640, 86_016, 6_400, 786_432],
            );
            set_band_values(
                canonical,
                "enforcement_ceilings",
                [24, 896, 98_304, 9_600, 1_179_648],
            );
            set_field(
                canonical,
                "aggregate_composition",
                standard_composition(
                    "eight initial semantic parts plus a larger ceiling continuation pool",
                    vec![
                        composition_member(
                            "semantic",
                            object(vec![("health", integer(8)), ("ceiling", integer(8))]),
                            (640, 86_016),
                            (896, 98_304),
                        ),
                        composition_member(
                            "continuation",
                            object(vec![("health", integer(8)), ("ceiling", integer(16))]),
                            (160, 12_288),
                            (152, 24_576),
                        ),
                    ],
                ),
            );
        }
        Mutation::CompositionMissingCeilingCount => {
            set_field(
                surface_mut(fixture, "canonical"),
                "aggregate_composition",
                standard_composition(
                    "a partial band count cannot prove either aggregate",
                    vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        composition_member(
                            "continuation",
                            object(vec![("health", integer(15))]),
                            (100, 4_096),
                            (100, 4_096),
                        ),
                    ],
                ),
            );
        }
        Mutation::CompositionUnknownCountBand => {
            set_field(
                surface_mut(fixture, "canonical"),
                "aggregate_composition",
                standard_composition(
                    "count bands are a closed schema",
                    vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        composition_member(
                            "continuation",
                            object(vec![
                                ("health", integer(15)),
                                ("ceiling", integer(15)),
                                ("future", integer(15)),
                            ]),
                            (100, 4_096),
                            (100, 4_096),
                        ),
                    ],
                ),
            );
        }
        Mutation::CompositionWrongLineSum => {
            let canonical = surface_mut(fixture, "canonical");
            for band in ["health_targets", "enforcement_ceilings"] {
                set_band_values(canonical, band, [16, 100, 4_096, 1_509, 61_952]);
            }
            set_field(
                canonical,
                "aggregate_composition",
                standard_composition(
                    "one small index plus fifteen full members",
                    vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        composition_member("member", integer(15), (100, 4_096), (100, 4_096)),
                    ],
                ),
            );
        }
        Mutation::CompositionWrongFileCount => {
            let canonical = surface_mut(fixture, "canonical");
            for band in ["health_targets", "enforcement_ceilings"] {
                set_band_values(canonical, band, [16, 100, 4_096, 1_410, 57_856]);
            }
            set_field(
                canonical,
                "aggregate_composition",
                standard_composition(
                    "fourteen full members and one index, one member short",
                    vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        composition_member("member", integer(14), (100, 4_096), (100, 4_096)),
                    ],
                ),
            );
        }
        Mutation::CompositionWrongLargest => {
            let canonical = surface_mut(fixture, "canonical");
            for band in ["health_targets", "enforcement_ceilings"] {
                set_band_values(canonical, band, [16, 100, 4_096, 1_360, 61_952]);
            }
            set_field(
                canonical,
                "aggregate_composition",
                standard_composition(
                    "every member below the declared per-file bound",
                    vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        composition_member("member", integer(15), (90, 4_096), (90, 4_096)),
                    ],
                ),
            );
        }
        Mutation::CompositionSingleRole => {
            set_field(
                surface_mut(fixture, "canonical"),
                "aggregate_composition",
                standard_composition(
                    "a homogeneous collection needs no exemption",
                    vec![composition_member(
                        "member",
                        integer(16),
                        (100, 4_096),
                        (100, 4_096),
                    )],
                ),
            );
        }
        Mutation::CompositionNoRationale => {
            set_field(
                surface_mut(fixture, "canonical"),
                "aggregate_composition",
                object(vec![(
                    "members",
                    Py::List(vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        composition_member("member", integer(15), (100, 4_096), (100, 4_096)),
                    ]),
                )]),
            );
        }
        Mutation::CompositionRepeatedRole => {
            set_field(
                surface_mut(fixture, "canonical"),
                "aggregate_composition",
                standard_composition(
                    "duplicated roles hide which members were counted",
                    vec![
                        composition_member("member", integer(1), (10, 512), (10, 512)),
                        composition_member("member", integer(15), (100, 4_096), (100, 4_096)),
                    ],
                ),
            );
        }
        Mutation::CompositionUnknownField => {
            set_field(
                surface_mut(fixture, "canonical"),
                "aggregate_composition",
                object(vec![
                    (
                        "rationale",
                        text("unknown control-plane fields fail closed"),
                    ),
                    ("note", text("x")),
                    (
                        "members",
                        Py::List(vec![
                            composition_member("index", integer(1), (10, 512), (10, 512)),
                            composition_member("member", integer(15), (100, 4_096), (100, 4_096)),
                        ]),
                    ),
                ]),
            );
        }
        Mutation::CompositionMissingHealth => {
            set_field(
                surface_mut(fixture, "canonical"),
                "aggregate_composition",
                standard_composition(
                    "both bands must be partitioned, not just the ceiling",
                    vec![
                        composition_member("index", integer(1), (10, 512), (10, 512)),
                        object(vec![
                            ("role", text("member")),
                            ("count", integer(15)),
                            (
                                "ceiling",
                                object(vec![("lines", integer(100)), ("bytes", integer(4_096))]),
                            ),
                        ]),
                    ],
                ),
            );
        }
        Mutation::OffRootTarget => {
            set_field(
                surface_mut(fixture, "snapshot"),
                "targets",
                strings(&["../escape.md"]),
            );
        }
        Mutation::MembershipMissingIndex => {
            set_field(
                surface_mut(fixture, "canonical"),
                "index",
                text("canonical/MISSING.md"),
            );
        }
        Mutation::RouteMissingTarget => {
            write_text(
                &fixture.root,
                "control/routes.tsv",
                "target\troute_kind\towner\tlifecycle\tpressure_control\tterminal\n\
missing.md\treader_navigation\tfixture\tmaintained_entrypoint\treview_on_behavior_change\tyes\n",
                None,
            )?;
        }
        Mutation::IndependentCeiling(index) => {
            set_field(
                nested_mut(surface_mut(fixture, "snapshot"), &["enforcement_ceilings"]),
                DIMENSIONS[index],
                integer(0),
            );
        }
        Mutation::CeilingMissingAuthority => {
            let snapshot = surface_mut(fixture, "snapshot");
            let ceilings = field_mut(snapshot, "enforcement_ceilings").expect("ceilings");
            let old = py_usize(field(ceilings, "bytes_each").expect("bytes_each"));
            set_field(ceilings, "bytes_each", integer(old + 1));
        }
        Mutation::CeilingExactAuthority => {
            let old = field(surface(fixture, "snapshot"), "enforcement_ceilings")
                .expect("ceilings")
                .clone();
            let snapshot = surface_mut(fixture, "snapshot");
            let ceilings = field_mut(snapshot, "enforcement_ceilings").expect("ceilings");
            let old_bytes = py_usize(field(ceilings, "bytes_each").expect("bytes_each"));
            set_field(ceilings, "bytes_each", integer(old_bytes + 1));
            let new = ceilings.clone();
            fixture.authorities = vec![ceiling_authority(
                "snapshot",
                "fixture-ceiling-increase",
                "Exercise exact one-change ceiling authority.",
                old,
                new,
            )];
        }
        Mutation::CeilingBankedAuthority => {
            let ceilings = field(surface(fixture, "snapshot"), "enforcement_ceilings")
                .expect("ceilings")
                .clone();
            fixture.authorities = vec![ceiling_authority(
                "snapshot",
                "fixture-banked-authority",
                "This unchanged authority must be rejected as banked.",
                ceilings.clone(),
                ceilings,
            )];
        }
        Mutation::DiscoveredNoBand => {
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":8,\"max_bytes\":4096}\n\
{\"record_type\":\"member\",\"id\":\"one\"}\n",
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
        }
        Mutation::DiscoveredPressure => {
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":2,\"max_bytes\":4096,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n\
{\"record_type\":\"member\",\"id\":\"one\"}\n\
{\"record_type\":\"member\",\"id\":\"two\"}\n",
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
        }
        Mutation::DiscoveredNonRegistry => {
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"member\",\"id\":\"one\"}\n",
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
        }
        Mutation::DiscoveredCapacity => {
            let content = "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":8,\"max_bytes\":4096,\"max_record_bytes\":2048,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n".to_string()
                + &padded_registry_record("one", 1_000)?
                + &padded_registry_record("two", 1_000)?;
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                &content,
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
        }
        Mutation::DiscoveredIncoherentQuiet => {
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":8,\"max_bytes\":8192,\"max_record_bytes\":4096,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n\
{\"record_type\":\"member\",\"id\":\"one\"}\n\
{\"record_type\":\"member\",\"id\":\"two\"}\n",
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
        }
        Mutation::DiscoveredCoherentQuiet => {
            let content = "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":4,\"max_bytes\":8192,\"max_record_bytes\":2048,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n".to_string()
                + &padded_registry_record("one", 1_000)?
                + &padded_registry_record("two", 1_000)?;
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                &content,
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
        }
        Mutation::HeaderMissingAuthority => {
            set_field(&mut fixture.registry_meta, "max_records", integer(17));
        }
        Mutation::HeaderExactAuthority => {
            set_field(&mut fixture.registry_meta, "max_records", integer(17));
            fixture.authorities = vec![registry_authority(
                "control/surfaces.jsonl",
                "fixture-registry-header-increase",
                "Exercise exact one-change registry header authority.",
                header_bounds(16, 32_768),
                header_bounds(17, 32_768),
            )];
        }
        Mutation::HeaderMismatchedAuthority => {
            set_field(&mut fixture.registry_meta, "max_records", integer(18));
            fixture.authorities = vec![registry_authority(
                "control/surfaces.jsonl",
                "fixture-registry-header-mismatch",
                "A near-miss authority must not authorise a different raise.",
                header_bounds(16, 32_768),
                header_bounds(17, 32_768),
            )];
        }
        Mutation::HeaderBankedAuthority => {
            fixture.authorities = vec![registry_authority(
                "control/surfaces.jsonl",
                "fixture-registry-banked",
                "This unchanged registry authority must be rejected as banked.",
                header_bounds(16, 32_768),
                header_bounds(16, 32_768),
            )];
        }
        Mutation::AmbiguousAuthority => {
            let ceilings = field(surface(fixture, "snapshot"), "enforcement_ceilings")
                .expect("ceilings")
                .clone();
            let mut authority = ceiling_authority(
                "snapshot",
                "fixture-ambiguous-authority",
                "An authority must authorise exactly one thing.",
                ceilings.clone(),
                ceilings,
            );
            set_field(
                &mut authority,
                "registry_id",
                text("control/surfaces.jsonl"),
            );
            fixture.authorities = vec![authority];
        }
        Mutation::DiscoveredHeaderMissingAuthority => {
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":8,\"max_bytes\":4096,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n",
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
            commit_fixture(fixture, "declare a second bounded registry")?;
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":16,\"max_bytes\":4096,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n",
                None,
            )?;
        }
        Mutation::DiscoveredHeaderExactAuthority => {
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":8,\"max_bytes\":4096,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n",
                None,
            )?;
            stage_fixture_paths(fixture, "doctrine/fixture/extra.jsonl")?;
            commit_fixture(fixture, "declare a second bounded registry")?;
            write_text(
                &fixture.root,
                "doctrine/fixture/extra.jsonl",
                "{\"record_type\":\"registry\",\"schema_version\":1,\"max_records\":16,\"max_bytes\":4096,\"milestones\":{\"warning_pct\":80,\"rollover_pct\":90}}\n",
                None,
            )?;
            fixture.authorities = vec![registry_authority(
                "doctrine/fixture/extra.jsonl",
                "fixture-discovered-registry-increase",
                "Exercise the generalized registry header authority.",
                header_bounds(8, 4_096),
                header_bounds(16, 4_096),
            )];
        }
        Mutation::ForeignRegistryAuthority => {
            fixture.authorities = vec![registry_authority(
                "control/authorities.jsonl",
                "fixture-foreign-registry",
                "Only the surface registry header is inside this protocol.",
                header_bounds(16, 32_768),
                header_bounds(17, 32_768),
            )];
        }
        Mutation::MaintainedReusedAuthority => {
            maintained_aggregate_change(fixture, false)?;
        }
        Mutation::MaintainedFreshAuthority => {
            maintained_aggregate_change(fixture, true)?;
        }
        Mutation::TransitionMovedBaseline => {
            let ledger = surface_mut(fixture, "ledger");
            let baseline = field_mut(ledger, "baseline").expect("baseline");
            let old = py_usize(field(baseline, "bytes_each").expect("bytes_each"));
            set_field(baseline, "bytes_each", integer(old + 1));
            set_field(
                nested_mut(ledger, &["transition", "max_growth"]),
                "bytes_each",
                integer(1),
            );
        }
        Mutation::CardinalityDeclared => {
            declare_exemption(fixture, "routed/README.md", "routed");
        }
        Mutation::CardinalityMissing => {
            let routed_titles = surface_mut(fixture, "routed_titles");
            set_field(
                field_mut(routed_titles, "health_targets").expect("health targets"),
                "files",
                Py::None,
            );
            set_field(
                field_mut(routed_titles, "enforcement_ceilings").expect("ceilings"),
                "files",
                Py::None,
            );
        }
        Mutation::CardinalityHalf => {
            declare_exemption(fixture, "routed/README.md", "routed");
            set_field(
                nested_mut(
                    surface_mut(fixture, "routed_titles"),
                    &["enforcement_ceilings"],
                ),
                "files",
                integer(16),
            );
        }
        Mutation::CardinalityResource => {
            declare_exemption(fixture, "routed/README.md", "routed");
            let routed_titles = surface_mut(fixture, "routed_titles");
            set_field(
                field_mut(routed_titles, "health_targets").expect("health targets"),
                "lines_total",
                Py::None,
            );
            set_field(
                field_mut(routed_titles, "enforcement_ceilings").expect("ceilings"),
                "lines_total",
                Py::None,
            );
        }
        Mutation::CardinalityUnknownRoute => {
            declare_exemption(fixture, "routed/README.md", "absent_route");
        }
        Mutation::CardinalityUnboundedRoute => {
            declare_exemption(fixture, "routed/README.md", "maintained");
        }
        Mutation::CardinalitySelfRoute => {
            declare_exemption(fixture, "routed/README.md", "routed_titles");
        }
        Mutation::CardinalityUntrackedAuthority => {
            declare_exemption(fixture, "routed/absent.md", "routed");
        }
        Mutation::RoutedFloatingIndex => {
            write_text(
                &fixture.root,
                "unclassified/INDEX.md",
                "# Floating landing\n",
                None,
            )?;
            set_field(
                surface_mut(fixture, "routed"),
                "index",
                text("unclassified/INDEX.md"),
            );
        }
        Mutation::QuotedSurfaceNumber => {
            set_field(
                nested_mut(surface_mut(fixture, "snapshot"), &["health_targets"]),
                "bytes_each",
                text("1024"),
            );
        }
        Mutation::QuotedSiblingNumber => {
            write_text(
                &fixture.root,
                "control/sibling.jsonl",
                "{\"lines\":\"12\"}\n",
                None,
            )?;
        }
        Mutation::NumericFieldString => {
            write_text(
                &fixture.root,
                "control/sibling.jsonl",
                "{\"lines\":12}\n{\"part\":{\"lines\":\"12 lines\"}}\n",
                None,
            )?;
        }
        Mutation::DecimalIdentityAccepted => {
            write_text(
                &fixture.root,
                "control/sibling.json",
                "{\"artifact_fingerprint\":\"1234567890123456\",\"owner\":\"fixture-maintainers\",\"enabled\":true,\"note\":null}\n",
                None,
            )?;
        }
    }
    Ok(())
}

fn report_result(
    name: &str,
    number: usize,
    passed: bool,
    checker_output: &str,
    quiet: bool,
    output: &mut dyn Output,
) {
    if passed {
        if !quiet {
            output.out(&format!("ok {number} - {name}\n"));
        }
    } else {
        output.out(&format!("not ok {number} - {name}\n"));
        if checker_output.is_empty() {
            output.out("# ");
        }
        let mut remainder = checker_output;
        while !remainder.is_empty() {
            output.out("# ");
            if let Some(newline) = remainder.find('\n') {
                output.out(&remainder[..=newline]);
                remainder = &remainder[newline + 1..];
            } else {
                output.out(remainder);
                remainder = "";
            }
        }
    }
}

fn fixture_root(root: &Path, number: usize) -> PathBuf {
    root.join("generated").join(format!(
        "live-document-size-tests.{}.{number}",
        std::process::id()
    ))
}

/// Run all 126 checks with the original `--quiet` or verbose output contract.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the native `LIVE-DOC-SIZE` orchestrator in `.15.z`.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn Output) -> i32 {
    let quiet = match arguments {
        [] => false,
        [argument] if argument == "--quiet" => true,
        _ => {
            let program = std::env::var("SPECFORGE_SHIM_PATH")
                .unwrap_or_else(|_| "cargo xtask check live-document-size-tests".to_string());
            output.err(&format!("Usage: {program} [--quiet]\n"));
            return 2;
        }
    };
    if let Err(error) = fs::create_dir_all(root.join("generated")) {
        output.err(&format!("cannot create generated fixture root: {error}\n"));
        return 1;
    }
    let mut verifiers = match VerifierStore::new(root.join("generated").join(format!(
        "live-document-size-verifiers.{}",
        std::process::id()
    ))) {
        Ok(store) => store,
        Err(error) => {
            output.err(&format!("{error}\n"));
            return 1;
        }
    };
    let mut number = 0;
    let mut failures = 0;
    for case in CASES {
        number += 1;
        let mut fixture = match new_fixture(&fixture_root(root, number), &mut verifiers) {
            Ok(fixture) => fixture,
            Err(error) => {
                output.err(&format!("{error}\n"));
                return 1;
            }
        };
        if case.history
            && let Err(error) = initialize_git_history(&fixture)
        {
            output.err(&format!("{error}\n"));
            return 1;
        }
        if let Err(error) = mutate(&mut fixture, &mut verifiers, case.mutation) {
            output.err(&format!("{error}\n"));
            return 1;
        }
        if let Err(error) = save_registry(&fixture).and_then(|()| save_authorities(&fixture)) {
            output.err(&format!("{error}\n"));
            return 1;
        }
        let (status, checker_output) = match run_checker(root, &fixture, case.history) {
            Ok(result) => result,
            Err(error) => {
                output.err(&format!("{error}\n"));
                return 1;
            }
        };
        let mut passed = if case.success {
            status == 0
        } else {
            status != 0
        };
        if let Some(pattern) = case.pattern {
            let matched = matches_pattern(&checker_output, pattern);
            passed &= if case.absent { !matched } else { matched };
        }
        if !passed {
            failures += 1;
        }
        report_result(case.name, number, passed, &checker_output, quiet, output);
    }
    if !quiet {
        output.out(&format!("1..{number}\n"));
    }
    if failures > 0 {
        output.err(&format!(
            "live-document-size-tests: {failures} of {number} checks failed\n"
        ));
        return 1;
    }
    if number != EXPECTED_CHECKS {
        output.err(&format!(
            "live-document-size-tests: ran {number} checks, declaration expects {EXPECTED_CHECKS} — re-derive the declaration beside the suite\n"
        ));
        return 1;
    }
    output.err(&format!(
        "live-document-size-tests: {number}/{EXPECTED_CHECKS} lifecycle and control-plane checks pass\n"
    ));
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_126_checks_pass_without_fixture_residue() {
        let root = crate::repository_root();
        let fixture_prefix = format!("live-document-size-tests.{}.", std::process::id());
        let verifier_prefix = format!("live-document-size-verifiers.{}", std::process::id());
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--quiet".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(
            output.text,
            "live-document-size-tests: 126/126 lifecycle and control-plane checks pass\n"
        );
        let residue = fs::read_dir(root.join("generated"))
            .expect("generated fixture root")
            .filter_map(Result::ok)
            .any(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                name.starts_with(&fixture_prefix) || name.starts_with(&verifier_prefix)
            });
        assert!(
            !residue,
            "native live-document-size fixtures were not cleaned"
        );
    }

    #[test]
    fn fixture_construction_failure_removes_the_partial_fixture() {
        let base = crate::repository_root().join("generated").join(format!(
            "live-document-size-construction-failure.{}",
            std::process::id()
        ));
        let fixture = base.join("fixture");
        let mut verifiers = VerifierStore {
            root: fixture.join("snapshot.md"),
            sources: BTreeMap::new(),
        };
        assert!(new_fixture(&fixture, &mut verifiers).is_err());
        assert!(!fixture.exists());
        drop(verifiers);
        fs::remove_dir_all(base).unwrap();
    }
}
