//! FSMGen request bundles meet `docs/upstream/fsmgen/PROCESS.md`, native since
//! `RUSTDOC-AND-XTASK.15.g`: layout, summary fields, request schema, signoff and verification, ownership,
//! lifecycle/ledger agreement, and portability. The Perl script remains the argument-compatible shim.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "fsmgen-request-bundles";
const LEGACY: [&str; 3] = [
    "sf-isf-contract-eventually-flat",
    "sf-isf-stage-ready-valid",
    "sf-isf-explicit-fsm-declaration",
];
const REQUEST_FIELDS: [&str; 14] = [
    "schema_version",
    "id",
    "kind",
    "title",
    "owner",
    "feedback_record",
    "failure_class",
    "verified_on",
    "specforge_side",
    "status",
    "signed_off",
    "forwarded",
    "upstream_reference",
    "resolution",
];
const SUMMARY_FIELDS: [&str; 12] = [
    "Title",
    "FSMGen command or public API entrypoint",
    "FSMGen-facing primary artifact path",
    "Failure class",
    "FSMGen commit or capability-manifest producer commit",
    "SPECFORGE version/commit",
    "Expected behavior",
    "Observed behavior",
    "First failing command or API call",
    "Exit status or exception",
    "Is the attached artifact bundle minimized",
    "Does the minimized/redacted bundle still reproduce",
];

fn problem(errors: &mut Vec<String>, message: impl AsRef<str>) {
    errors.push(format!("{LABEL}: {}", message.as_ref()));
}

fn slurp(path: &Path) -> Option<Vec<u8>> {
    fs::read(path).ok()
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn directories(path: &Path) -> std::io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.starts_with('.') {
                names.push(name);
            }
        }
    }
    names.sort();
    Ok(names)
}

fn markdown_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    let mut entries: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.is_file() && path.extension().is_some_and(|extension| extension == "md") {
            out.push(path);
        }
    }
}

fn declared_leaves(root: &Path) -> BTreeSet<String> {
    let mut files = Vec::new();
    markdown_files(&root.join("docs/tasks"), &mut files);
    let mut leaves = BTreeSet::new();
    for path in files {
        let Some(raw) = slurp(&path) else {
            continue;
        };
        let text = String::from_utf8_lossy(&raw);
        let mut rest = text.as_ref();
        while let Some(start) = rest.find("ID: `") {
            rest = &rest[start + 5..];
            let mut split = rest.splitn(2, |character: char| {
                !(character.is_ascii_alphanumeric() || character == '-' || character == '.')
            });
            let candidate = split.next().unwrap_or("");
            if valid_task_id(candidate) {
                leaves.insert(candidate.to_string());
            }
            rest = split.next().unwrap_or("");
        }
    }
    leaves
}

fn valid_task_id(id: &str) -> bool {
    let mut characters = id.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    if !first.is_ascii_uppercase() && !first.is_ascii_digit() {
        return false;
    }
    let mut parts = id.split('.');
    let Some(stem) = parts.next() else {
        return false;
    };
    if stem.is_empty()
        || !stem.chars().all(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return false;
    }
    parts.all(|part| {
        !part.is_empty()
            && part
                .chars()
                .all(|character| character.is_ascii_alphanumeric())
    })
}

fn valid_packet_id(id: &str) -> bool {
    let Some(suffix) = id.strip_prefix("sf-") else {
        return false;
    };
    !suffix.is_empty()
        && suffix.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        })
}

fn heading_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("### ")?;
    let end = rest
        .char_indices()
        .find(|(_, character)| {
            !(character.is_ascii_lowercase() || character.is_ascii_digit() || *character == '-')
        })
        .map(|(index, _)| index)
        .unwrap_or(rest.len());
    let name = &rest[..end];
    if name.is_empty()
        || name.starts_with('-')
        || name.ends_with('-')
        || name.split('-').any(|part| part.is_empty())
        || rest[end..]
            .chars()
            .next()
            .is_some_and(|character| character.is_alphanumeric() || character == '_')
    {
        return None;
    }
    Some(name)
}

fn field_values(line: &str, fields: &mut BTreeMap<String, String>) {
    let mut offset = 0;
    while offset < line.len() {
        let start = if offset == 0 && line.starts_with("- ") {
            Some(2)
        } else {
            line[offset..]
                .find(" · ")
                .map(|index| offset + index + " · ".len())
        };
        let Some(start) = start else {
            break;
        };
        let rest = &line[start..];
        let Some(separator) = rest.find(": `") else {
            break;
        };
        let key = &rest[..separator];
        if key.is_empty()
            || !key
                .chars()
                .all(|character| character.is_alphanumeric() || character == '_')
        {
            offset = start + separator;
            continue;
        }
        let value_start = start + separator + 3;
        let Some(value_end) = line[value_start..].find('`') else {
            break;
        };
        let value_end = value_start + value_end;
        fields.insert(key.to_string(), line[value_start..value_end].to_string());
        offset = value_end + 1;
    }
}

fn feedback_records(
    feedback: &str,
) -> (BTreeSet<String>, BTreeMap<String, BTreeMap<String, String>>) {
    let region = feedback
        .split_once("<!-- fsmgen_feedback_open:start -->")
        .and_then(|(_, rest)| rest.split_once("<!-- fsmgen_feedback_open:end -->"))
        .map(|(region, _)| region)
        .unwrap_or("");
    let mut open = BTreeSet::new();
    let mut records = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in region.lines() {
        if let Some(name) = heading_name(line) {
            open.insert(name.to_string());
            records
                .entry(name.to_string())
                .or_insert_with(BTreeMap::new);
            current = Some(name.to_string());
        } else if let Some(name) = &current
            && let Some(fields) = records.get_mut(name)
        {
            field_values(line, fields);
        }
    }
    (open, records)
}

fn field<'a>(request: &'a Py, key: &str) -> Option<&'a Py> {
    pyjson::get(request, key)
}

fn scalar(request: &Py, key: &str) -> String {
    perl::scalar(field(request, key)).unwrap_or_default()
}

fn lifecycle_rank(status: &str) -> Option<usize> {
    [
        "drafted",
        "signed_off",
        "forwarded",
        "acknowledged",
        "resolved_upstream",
        "integrated",
        "rejected",
        "withdrawn",
    ]
    .iter()
    .position(|candidate| *candidate == status)
}

fn closed(status: &str) -> bool {
    matches!(status, "integrated" | "rejected" | "withdrawn")
}

fn record_status(status: &str) -> Option<&'static [&'static str]> {
    match status {
        "drafted" => Some(&["open"]),
        "signed_off" => Some(&["open"]),
        "forwarded" => Some(&["awaiting_response"]),
        "acknowledged" => Some(&["accepted"]),
        "resolved_upstream" => Some(&["shipped", "resolved", "answered"]),
        _ => None,
    }
}

fn check_root(root: &Path) -> (Vec<String>, usize) {
    let mut errors = Vec::new();
    let upstream = root.join("docs/upstream");
    let projects = match directories(&upstream) {
        Ok(projects) => projects,
        Err(error) => {
            problem(
                &mut errors,
                format!("cannot read {}: {}", upstream.display(), os_error(&error)),
            );
            return (errors, 0);
        }
    };
    let mut checked = 0;
    for project in projects {
        if project != "fsmgen" {
            problem(
                &mut errors,
                format!(
                    "docs/upstream/{project}: not a registered upstream project (add its ledger, ADR 0054)"
                ),
            );
            continue;
        }
        checked += check_project(root, &project, "docs/FSMGEN_FEEDBACK.md", &mut errors);
    }
    (errors, checked)
}

fn check_project(root: &Path, project: &str, ledger: &str, errors: &mut Vec<String>) -> usize {
    let issues = root.join("docs/upstream").join(project);
    let bundles = match directories(&issues) {
        Ok(bundles) => bundles,
        Err(error) => {
            problem(
                errors,
                format!("cannot read {}: {}", issues.display(), os_error(&error)),
            );
            return 0;
        }
    };
    let feedback = slurp(&root.join(ledger))
        .map(|raw| String::from_utf8_lossy(&raw).into_owned())
        .unwrap_or_default();
    let (open_records, record_fields) = feedback_records(&feedback);
    let declared = declared_leaves(root);
    let mut packets_for_record: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut checked = 0;

    for id in bundles {
        if LEGACY.contains(&id.as_str()) {
            continue;
        }
        checked += 1;
        let directory = issues.join(&id);
        if !valid_packet_id(&id) {
            problem(errors, format!("{id}: a request id starts with 'sf-'"));
        }
        let Some(request_raw) = slurp(&directory.join("request.json")) else {
            problem(errors, format!("{id}: missing request.json"));
            continue;
        };
        let request = pyjson::loads_strict(&String::from_utf8_lossy(&request_raw));
        let Ok(request) = request else {
            problem(errors, format!("{id}: request.json is not a JSON object"));
            continue;
        };
        if !matches!(request, Py::Dict(_)) {
            problem(errors, format!("{id}: request.json is not a JSON object"));
            continue;
        }
        for name in REQUEST_FIELDS {
            if field(&request, name).is_none() {
                problem(errors, format!("{id}: request.json lacks '{name}'"));
            }
        }
        if let Py::Dict(pairs) = &request {
            let mut unknown: Vec<String> = pairs
                .iter()
                .map(|(key, _)| pyjson::to_text(key))
                .filter(|key| !REQUEST_FIELDS.contains(&key.as_str()))
                .collect();
            unknown.sort();
            for name in unknown {
                problem(
                    errors,
                    format!("{id}: request.json has unknown field '{name}'"),
                );
            }
        }
        let request_id = scalar(&request, "id");
        if request_id != id {
            problem(
                errors,
                format!("{id}: request.json id '{request_id}' differs from the directory"),
            );
        }
        let kind = scalar(&request, "kind");
        if !matches!(kind.as_str(), "bug_report" | "feature_request" | "question") {
            problem(
                errors,
                format!("{id}: request.json kind must be bug_report, feature_request or question"),
            );
        }
        let status = scalar(&request, "status");
        let Some(rank) = lifecycle_rank(&status) else {
            problem(
                errors,
                format!("{id}: request.json status '{status}' is not a lifecycle state"),
            );
            continue;
        };
        let past_signoff = (rank >= 1 && !closed(&status)) || status == "integrated";
        let owner = scalar(&request, "owner");
        if !declared.contains(&owner) {
            problem(
                errors,
                format!("{id}: owner '{owner}' is not a leaf any docs/tasks tree declares"),
            );
        }
        let record = scalar(&request, "feedback_record");
        packets_for_record
            .entry(record.clone())
            .or_default()
            .push(id.clone());
        if !closed(&status) && !open_records.contains(&record) {
            problem(
                errors,
                format!("{id}: feedback_record '{record}' is not an open record of {ledger}"),
            );
        }
        if let Some(fields) = record_fields.get(&record) {
            let record_kind = fields.get("Kind").map(String::as_str).unwrap_or("");
            if record_kind != kind && !(kind == "question" && record_kind == "clarity_request") {
                problem(
                    errors,
                    format!("{id}: kind '{kind}' but record '{record}' says '{record_kind}'"),
                );
            }
            if let Some(wanted) = record_status(&status) {
                let actual = fields.get("Status").map(String::as_str).unwrap_or("");
                if !wanted.contains(&actual) {
                    problem(
                        errors,
                        format!(
                            "{id}: status '{status}' needs record status {}, but record '{record}' says '{actual}'",
                            wanted.join(" or ")
                        ),
                    );
                }
            }
            let evidence = fields.get("Evidence").map(String::as_str).unwrap_or("");
            let expected = format!("docs/upstream/{project}/{id}/README.md");
            if evidence != expected {
                problem(
                    errors,
                    format!("{id}: record '{record}' cites '{evidence}', not this packet's README"),
                );
            }
        }

        for name in [
            "README.md",
            "commands.sh",
            "env.txt",
            "SIGNOFF.md",
            "expected/expected-notes.md",
        ] {
            if !directory.join(name).is_file() {
                problem(errors, format!("{id}: missing {name}"));
            }
        }
        for name in ["sources/fsmgen-input", "observed"] {
            match fs::read_dir(directory.join(name)) {
                Ok(entries) => {
                    let count = entries.filter_map(|entry| entry.ok()).count();
                    if count == 0 {
                        problem(errors, format!("{id}: {name}/ is empty"));
                    }
                }
                Err(_) => problem(errors, format!("{id}: missing {name}/")),
            }
        }
        let commands = directory.join("commands.sh");
        if commands.is_file() && !executable(&commands) {
            problem(errors, format!("{id}: commands.sh is not executable"));
        }
        let readme = slurp(&directory.join("README.md"))
            .map(|raw| String::from_utf8_lossy(&raw).into_owned())
            .unwrap_or_default();
        for name in SUMMARY_FIELDS {
            let prefix = format!("{name}: ");
            if !readme.lines().any(|line| {
                line.strip_prefix(&prefix).is_some_and(|value| {
                    value
                        .chars()
                        .next()
                        .is_some_and(|character| !character.is_whitespace())
                })
            }) {
                problem(
                    errors,
                    format!("{id}: README.md lacks the summary field '{name}:'"),
                );
            }
        }
        portability_errors(&directory, &id, errors);

        if !past_signoff {
            continue;
        }
        let signoff = slurp(&directory.join("SIGNOFF.md"))
            .map(|raw| String::from_utf8_lossy(&raw).into_owned())
            .unwrap_or_default();
        let open_boxes = signoff
            .lines()
            .filter(|line| line.starts_with("- [ ]"))
            .count();
        let ticked = signoff
            .lines()
            .filter(|line| line.starts_with("- [x]"))
            .count();
        if open_boxes > 0 {
            problem(
                errors,
                format!("{id}: SIGNOFF.md has {open_boxes} open box(es) at status '{status}'"),
            );
        }
        if ticked < 9 {
            problem(
                errors,
                format!("{id}: SIGNOFF.md ticks {ticked} of the 9 required boxes"),
            );
        }
        let mut roles = BTreeSet::new();
        if let Some(Py::List(entries)) = field(&request, "verified_on") {
            for entry in entries {
                if !matches!(entry, Py::Dict(_)) {
                    continue;
                }
                let revision = scalar(entry, "fsmgen_revision");
                if revision.len() != 40
                    || !revision
                        .chars()
                        .all(|character| character.is_ascii_hexdigit())
                {
                    problem(
                        errors,
                        format!("{id}: verified_on revision '{revision}' is not a 40-hex commit"),
                    );
                }
                if !scalar(entry, "result").is_empty() {
                    roles.insert(scalar(entry, "role"));
                }
            }
        }
        for role in ["pinned", "upstream"] {
            if !roles.contains(role) {
                problem(
                    errors,
                    format!("{id}: verified_on lacks a '{role}' revision with a result"),
                );
            }
        }
        let signed_off = scalar(&request, "signed_off");
        let valid_date = signed_off.len() == 10
            && signed_off.chars().enumerate().all(|(index, character)| {
                if matches!(index, 4 | 7) {
                    character == '-'
                } else {
                    character.is_ascii_digit()
                }
            });
        if !valid_date {
            problem(errors, format!("{id}: signed_off date missing"));
        }
        if rank >= 2
            && !closed(&status)
            && !matches!(field(&request, "forwarded"), Some(Py::Dict(_)))
        {
            problem(
                errors,
                format!("{id}: status '{status}' requires 'forwarded' {{date, channel}}"),
            );
        }
        if status == "integrated" && !directory.join("RESOLUTION.md").is_file() {
            problem(
                errors,
                format!("{id}: an integrated request needs RESOLUTION.md"),
            );
        }
    }

    for (name, fields) in &record_fields {
        let outgoing = fields.get("Direction").map(String::as_str) == Some("SpecForge → FSMGen")
            && matches!(
                fields.get("Kind").map(String::as_str),
                Some("bug_report" | "feature_request" | "question" | "clarity_request")
            );
        if !outgoing {
            continue;
        }
        let count = packets_for_record.get(name).map(Vec::len).unwrap_or(0);
        if count != 1 {
            problem(
                errors,
                format!(
                    "record '{name}': an outgoing record needs exactly one packet (ADR 0054); {count} name it"
                ),
            );
        }
    }
    checked
}

#[cfg(unix)]
fn executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn executable(path: &Path) -> bool {
    path.is_file()
}

fn portability_errors(directory: &Path, id: &str, errors: &mut Vec<String>) {
    let mut files = Vec::new();
    all_files(directory, &mut files);
    for path in files {
        let Some(raw) = slurp(&path) else {
            continue;
        };
        if raw.contains(&0) {
            continue;
        }
        let text = String::from_utf8_lossy(&raw);
        if has_machine_local_path(&text) {
            let relative = path.strip_prefix(directory).unwrap_or(&path);
            problem(
                errors,
                format!(
                    "{id}: {} contains a machine-local absolute path",
                    relative.display()
                ),
            );
        }
    }
}

fn all_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    let mut entries: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            all_files(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}

fn has_machine_local_path(text: &str) -> bool {
    for prefix in ["/Users/", "/Volumes/", "/home/", "/private/", "/tmp/"] {
        for (index, _) in text.match_indices(prefix) {
            let previous = text[..index].chars().next_back();
            if previous.is_none_or(|character| {
                !(character.is_alphanumeric() || matches!(character, '_' | '.' | '>'))
            }) {
                return true;
            }
        }
    }
    false
}

/// Run the checker with the arguments its Perl shim normalized.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut self_test_mode = false;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" => {}
            "--self-test" => self_test_mode = true,
            "--root" => match arguments.next() {
                Some(directory) => root = PathBuf::from(directory),
                None => return usage(output),
            },
            _ => return usage(output),
        }
    }
    if self_test_mode {
        return self_test(&root, output);
    }
    let (errors, checked) = check_root(&root);
    if !errors.is_empty() {
        for error in &errors {
            output.err(&format!("{error}\n"));
        }
        output.err(&format!(
            "{LABEL}: FAILED with {} violation(s).\n",
            errors.len()
        ));
        return 1;
    }
    output.out(&format!(
        "{LABEL}: {checked} request packet(s) under docs/upstream/<project>/ meet their PROCESS.md; {} legacy bundle(s) exempt.\n",
        LEGACY.len()
    ));
    0
}

fn usage(output: &mut dyn super::Output) -> i32 {
    output.err("Usage: cargo xtask check fsmgen-request-bundles [--check|--self-test] [--root PROJECT_ROOT]\n");
    2
}

#[derive(Clone, Copy)]
enum Mutation {
    Baseline,
    OpenSignoff,
    NoUpstream,
    AbsolutePath,
    MissingSummary,
    UnknownStatus,
    CommandsNotExecutable,
    UnknownOwner,
    NoOpenRecord,
    ForwardedWithoutDetails,
    MissingExpectedNotes,
    UnknownKind,
    StatusDisagrees,
    KindDisagrees,
    RecordCitesAnotherPacket,
    UnregisteredProject,
    OutgoingWithoutPacket,
}

enum Expected {
    Accept,
    Plain(&'static str),
    StatusDisagrees,
    KindDisagrees,
}

struct Case {
    name: &'static str,
    expected: Expected,
    mutation: Mutation,
}

const CASES: &[Case] = &[
    Case {
        name: "baseline",
        expected: Expected::Accept,
        mutation: Mutation::Baseline,
    },
    Case {
        name: "open signoff box",
        expected: Expected::Plain("open box"),
        mutation: Mutation::OpenSignoff,
    },
    Case {
        name: "no upstream verification",
        expected: Expected::Plain("lacks a 'upstream' revision"),
        mutation: Mutation::NoUpstream,
    },
    Case {
        name: "absolute path",
        expected: Expected::Plain("machine-local absolute path"),
        mutation: Mutation::AbsolutePath,
    },
    Case {
        name: "missing summary field",
        expected: Expected::Plain("summary field 'Failure class:'"),
        mutation: Mutation::MissingSummary,
    },
    Case {
        name: "unknown status",
        expected: Expected::Plain("not a lifecycle state"),
        mutation: Mutation::UnknownStatus,
    },
    Case {
        name: "commands.sh not executable",
        expected: Expected::Plain("not executable"),
        mutation: Mutation::CommandsNotExecutable,
    },
    Case {
        name: "unknown owner",
        expected: Expected::Plain("not a leaf any docs/tasks tree declares"),
        mutation: Mutation::UnknownOwner,
    },
    Case {
        name: "no open feedback record",
        expected: Expected::Plain("not an open record"),
        mutation: Mutation::NoOpenRecord,
    },
    Case {
        name: "forwarded without details",
        expected: Expected::Plain("requires 'forwarded'"),
        mutation: Mutation::ForwardedWithoutDetails,
    },
    Case {
        name: "missing expected notes",
        expected: Expected::Plain("missing expected/expected-notes.md"),
        mutation: Mutation::MissingExpectedNotes,
    },
    Case {
        name: "unknown kind",
        expected: Expected::Plain("kind must be bug_report, feature_request or question"),
        mutation: Mutation::UnknownKind,
    },
    Case {
        name: "status disagrees with its record",
        expected: Expected::StatusDisagrees,
        mutation: Mutation::StatusDisagrees,
    },
    Case {
        name: "kind disagrees with its record",
        expected: Expected::KindDisagrees,
        mutation: Mutation::KindDisagrees,
    },
    Case {
        name: "record cites another packet",
        expected: Expected::Plain("not this packet's README"),
        mutation: Mutation::RecordCitesAnotherPacket,
    },
    Case {
        name: "unregistered upstream project",
        expected: Expected::Plain("docs/upstream/other: not a registered upstream project"),
        mutation: Mutation::UnregisteredProject,
    },
    Case {
        name: "outgoing record without a packet",
        expected: Expected::Plain(
            "record 'no-such-packet': an outgoing record needs exactly one packet",
        ),
        mutation: Mutation::OutgoingWithoutPacket,
    },
];

struct Scratch {
    path: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    if from.is_dir() {
        fs::create_dir_all(to).map_err(|error| format!("create {}: {error}", to.display()))?;
        let mut entries: Vec<_> = fs::read_dir(from)
            .map_err(|error| format!("read {}: {error}", from.display()))?
            .collect::<Result<_, _>>()
            .map_err(|error| format!("read {}: {error}", from.display()))?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            copy_tree(&entry.path(), &to.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create {}: {error}", parent.display()))?;
        }
        fs::copy(from, to).map_err(|error| format!("copy {}: {error}", from.display()))?;
        let permissions = fs::metadata(from)
            .map_err(|error| format!("stat {}: {error}", from.display()))?
            .permissions();
        fs::set_permissions(to, permissions)
            .map_err(|error| format!("chmod {}: {error}", to.display()))?;
    }
    Ok(())
}

fn request_value(directory: &Path) -> Result<Py, String> {
    let path = directory.join("request.json");
    let raw = fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    pyjson::loads_strict(&String::from_utf8_lossy(&raw)).map_err(|error| error.to_string())
}

fn set_field(request: &mut Py, key: &str, value: Py) {
    if let Py::Dict(pairs) = request {
        let key = pyjson::py_str(key);
        if let Some((_, current)) = pairs.iter_mut().find(|(candidate, _)| *candidate == key) {
            *current = value;
        } else {
            pairs.push((key, value));
        }
    }
}

fn write_request(directory: &Path, request: &Py) -> Result<(), String> {
    let encoder = pyjson::Encoder {
        indent: Some(3),
        sort_keys: true,
        ensure_ascii: false,
        item_separator: ",",
        key_separator: " : ",
    };
    let encoded =
        pyjson::dumps(request, &encoder).ok_or_else(|| "encode request.json".to_string())?;
    let path = directory.join("request.json");
    fs::write(&path, encoded).map_err(|error| format!("write {}: {error}", path.display()))
}

fn edit_text(path: &Path, change: impl FnOnce(&mut String)) -> Result<(), String> {
    let mut text =
        fs::read_to_string(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    change(&mut text);
    fs::write(path, text).map_err(|error| format!("write {}: {error}", path.display()))
}

fn heading_at(text: &str, record: &str) -> Option<usize> {
    let heading = format!("### {record}");
    text.match_indices(&heading)
        .find(|(index, _)| {
            (*index == 0 || text[..*index].ends_with('\n'))
                && text[*index + heading.len()..]
                    .chars()
                    .next()
                    .is_none_or(|character| !(character.is_alphanumeric() || character == '_'))
        })
        .map(|(index, _)| index)
}

fn mutate(root: &Path, directory: &Path, record: &str, mutation: Mutation) -> Result<(), String> {
    match mutation {
        Mutation::Baseline => {}
        Mutation::OpenSignoff => edit_text(&directory.join("SIGNOFF.md"), |text| {
            *text = text.replacen("- [x] **MINIMIZED", "- [ ] **MINIMIZED", 1);
        })?,
        Mutation::AbsolutePath => edit_text(&directory.join("env.txt"), |text| {
            text.push_str("see /Users/someone/x\n");
        })?,
        Mutation::MissingSummary => edit_text(&directory.join("README.md"), |text| {
            if let Some(start) = text.find("Failure class:")
                && let Some(end) = text[start..].find('\n')
            {
                text.replace_range(start..=start + end, "");
            }
        })?,
        Mutation::CommandsNotExecutable => {
            let path = directory.join("commands.sh");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&path, fs::Permissions::from_mode(0o644))
                    .map_err(|error| format!("chmod {}: {error}", path.display()))?;
            }
            #[cfg(not(unix))]
            {
                let _ = path;
            }
        }
        Mutation::NoOpenRecord => edit_text(&root.join("docs/FSMGEN_FEEDBACK.md"), |text| {
            if let Some(start) = heading_at(text, record) {
                text.replace_range(start..start + 3 + record.len(), "### renamed-record");
            }
        })?,
        Mutation::MissingExpectedNotes => {
            let path = directory.join("expected/expected-notes.md");
            fs::remove_file(&path)
                .map_err(|error| format!("remove {}: {error}", path.display()))?;
        }
        Mutation::UnregisteredProject => {
            let path = root.join("docs/upstream/other/sf-x");
            fs::create_dir_all(&path)
                .map_err(|error| format!("create {}: {error}", path.display()))?;
        }
        Mutation::RecordCitesAnotherPacket => {
            edit_text(&root.join("docs/FSMGEN_FEEDBACK.md"), |text| {
                if let Some(start) = heading_at(text, record)
                    && let Some(relative) = text[start..].find("Evidence: `")
                {
                    let value = start + relative + "Evidence: `".len();
                    if let Some(end) = text[value..].find('`') {
                        text.replace_range(value..value + end, "docs/elsewhere.md");
                    }
                }
            })?
        }
        Mutation::OutgoingWithoutPacket => {
            edit_text(&root.join("docs/FSMGEN_FEEDBACK.md"), |text| {
                if let Some(start) = heading_at(text, record) {
                    let next = text[start + 3..]
                        .find("\n### ")
                        .map(|index| start + 3 + index)
                        .unwrap_or(text.len());
                    text.insert_str(
                    next,
                    "\n\n### no-such-packet — a question with no packet\n\n- Direction: `SpecForge → FSMGen`\n- Kind: `question`\n- Status: `open`\n- Owner: `none`\n- Evidence: `none`\n",
                );
                }
            })?
        }
        Mutation::NoUpstream
        | Mutation::UnknownStatus
        | Mutation::UnknownOwner
        | Mutation::ForwardedWithoutDetails
        | Mutation::UnknownKind
        | Mutation::StatusDisagrees
        | Mutation::KindDisagrees => {
            let mut request = request_value(directory)?;
            match mutation {
                Mutation::NoUpstream => {
                    let first = match field(&request, "verified_on") {
                        Some(Py::List(entries)) => entries.first().cloned().unwrap_or(Py::None),
                        _ => Py::None,
                    };
                    set_field(&mut request, "verified_on", Py::List(vec![first]));
                }
                Mutation::UnknownStatus => {
                    set_field(&mut request, "status", Py::Str(pyjson::py_str("ready")))
                }
                Mutation::UnknownOwner => set_field(
                    &mut request,
                    "owner",
                    Py::Str(pyjson::py_str("FSMGEN-REQUESTS.99")),
                ),
                Mutation::ForwardedWithoutDetails => {
                    set_field(&mut request, "status", Py::Str(pyjson::py_str("forwarded")));
                    set_field(&mut request, "forwarded", Py::None);
                }
                Mutation::UnknownKind => set_field(
                    &mut request,
                    "kind",
                    Py::Str(pyjson::py_str("clarity_request")),
                ),
                Mutation::StatusDisagrees => {
                    let status = scalar(&request, "status");
                    let changed = if status == "acknowledged" {
                        "forwarded"
                    } else {
                        "acknowledged"
                    };
                    set_field(&mut request, "status", Py::Str(pyjson::py_str(changed)));
                    if !matches!(field(&request, "forwarded"), Some(Py::Dict(_))) {
                        set_field(
                            &mut request,
                            "forwarded",
                            Py::Dict(vec![
                                (
                                    pyjson::py_str("date"),
                                    Py::Str(pyjson::py_str("2026-09-25")),
                                ),
                                (pyjson::py_str("channel"), Py::Str(pyjson::py_str("test"))),
                            ]),
                        );
                    }
                }
                Mutation::KindDisagrees => {
                    set_field(&mut request, "kind", Py::Str(pyjson::py_str("question")))
                }
                _ => unreachable!(),
            }
            write_request(directory, &request)?;
        }
    }
    Ok(())
}

fn expected_match(expected: &Expected, error: &str, id: &str) -> bool {
    let addressed = error.contains(&format!("{id}: "))
        || error.contains(": record '")
        || error.contains(": docs/upstream/");
    if !addressed {
        return false;
    }
    match expected {
        Expected::Accept => false,
        Expected::Plain(wanted) => error.contains(wanted),
        Expected::StatusDisagrees => {
            error.contains("needs record status accepted")
                || error.contains("needs record status awaiting_response")
        }
        Expected::KindDisagrees => error.contains("record '") && error.contains(" says"),
    }
}

fn self_test(root: &Path, output: &mut dyn super::Output) -> i32 {
    let bundles = match directories(&root.join("docs/upstream/fsmgen")) {
        Ok(bundles) => bundles,
        Err(error) => {
            output.err(&format!("{LABEL} self-test: {error}\n"));
            return 1;
        }
    };
    let Some(id) = bundles.into_iter().find(|id| {
        !LEGACY.contains(&id.as_str())
            && root
                .join(format!("docs/upstream/fsmgen/{id}/request.json"))
                .is_file()
    }) else {
        output.err(&format!("{LABEL} self-test: no request bundle to mutate\n"));
        return 1;
    };
    let directory = root.join(format!("docs/upstream/fsmgen/{id}"));
    let request = match request_value(&directory) {
        Ok(request) => request,
        Err(error) => {
            output.err(&format!("{LABEL} self-test: {error}\n"));
            return 1;
        }
    };
    let record = scalar(&request, "feedback_record");
    let mut failures = 0;
    for (index, case) in CASES.iter().enumerate() {
        let scratch = Scratch {
            path: root.join(format!(
                "generated/fsmgen-request-bundles-self-test.{}.{}",
                std::process::id(),
                index
            )),
        };
        let result = (|| {
            copy_tree(
                &root.join("docs/upstream/fsmgen"),
                &scratch.path.join("docs/upstream/fsmgen"),
            )?;
            copy_tree(&root.join("docs/tasks"), &scratch.path.join("docs/tasks"))?;
            copy_tree(
                &root.join("docs/FSMGEN_FEEDBACK.md"),
                &scratch.path.join("docs/FSMGEN_FEEDBACK.md"),
            )?;
            mutate(
                &scratch.path,
                &scratch.path.join(format!("docs/upstream/fsmgen/{id}")),
                &record,
                case.mutation,
            )
        })();
        if let Err(error) = result {
            output.err(&format!("{LABEL} self-test: {error}\n"));
            return 1;
        }
        let (errors, _) = check_root(&scratch.path);
        let accepted = match case.expected {
            Expected::Accept => errors.is_empty(),
            _ => errors
                .iter()
                .any(|error| expected_match(&case.expected, error, &id)),
        };
        if !accepted {
            failures += 1;
            let verdict = match case.expected {
                Expected::Accept => "accepted",
                _ => "refused as expected",
            };
            output.err(&format!(
                "{LABEL} self-test: '{}' was not {}: {}\n",
                case.name,
                verdict,
                errors.join(" ")
            ));
        }
    }
    if failures > 0 {
        return 1;
    }
    output.out(&format!(
        "{LABEL} self-test: {} cases (1 accepted, {} refused) behave as specified.\n",
        CASES.len(),
        CASES.len() - 1
    ));
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_tree_and_all_self_test_cases_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--self-test".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(
            output.text,
            "fsmgen-request-bundles self-test: 17 cases (1 accepted, 16 refused) behave as specified.\n"
        );
    }

    #[test]
    fn ledger_fields_ids_and_absolute_paths_follow_the_script_grammar() {
        assert!(valid_task_id("FSMGEN-REQUESTS.6"));
        assert!(!valid_task_id("fsmgen-requests.6"));
        assert!(valid_packet_id("sf-axi4-lite"));
        assert!(!valid_packet_id("axi4-lite"));
        assert_eq!(heading_name("### record-name — title"), Some("record-name"));
        assert_eq!(heading_name("### not.valid"), Some("not"));
        let mut fields = BTreeMap::new();
        field_values(
            "- Direction: `SpecForge → FSMGen` · Kind: `question`",
            &mut fields,
        );
        assert_eq!(
            fields.get("Direction").map(String::as_str),
            Some("SpecForge → FSMGen")
        );
        assert_eq!(fields.get("Kind").map(String::as_str), Some("question"));
        assert!(has_machine_local_path("see /Users/someone/x"));
        assert!(has_machine_local_path("/tmp/x"));
        assert!(!has_machine_local_path("<FSMGEN_ROOT>/Users/x"));
        assert!(!has_machine_local_path(".artifacts/tmp/x"));
    }
}
