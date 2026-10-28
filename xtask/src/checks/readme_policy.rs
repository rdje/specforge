//! The `README-POLICY` doctrine (`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.2`), native since `RUSTDOC-AND-XTASK.12`:
//! the landing `README.md` stays within the line and byte ceilings `README_POLICY.md` records, the policy keeps its
//! fenced local adoption note, and every reader link and author-overflow destination is registered in
//! `doctrine/readme_entrypoint/routed_destinations.tsv` as a controlled terminal inside the repository.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// The reviewed README line ceiling, which `README_POLICY.md`'s adoption note records as `POLICY_CEILINGS`.
pub const LINE_CAP: usize = 150;
/// The reviewed README byte ceiling, recorded beside [`LINE_CAP`].
pub const BYTE_CAP: usize = 5800;
/// The route registry's ceiling on records after its header.
pub const ROUTE_RECORD_CAP: usize = 40;
/// The route registry's ceiling on bytes.
pub const ROUTE_BYTE_CAP: usize = 5000;
/// The route registry's ceiling on bytes in its widest raw record.
pub const ROUTE_RECORD_BYTE_CAP: usize = 192;

const README: &str = "README.md";
const POLICY: &str = "README_POLICY.md";
const ROUTES: &str = "doctrine/readme_entrypoint/routed_destinations.tsv";
const POLICY_FENCE: &str = "README-POLICY-LOCAL-ADOPTION:BEGIN";
/// The adoption note's sentence naming [`LINE_CAP`] and [`BYTE_CAP`].
const POLICY_CEILINGS: &str = "Independent ceilings are 150 lines and 5,800";
const HEADER: &str = "target\troute_kind\towner\tlifecycle\tpressure_control\tterminal";
/// The two routes that leave the tree: the issue tracker and git history.
const EXTERNAL_TARGETS: [&str; 2] = ["https://github.com/rdje/specforge/issues", "git:history"];
const ROUTE_KINDS: [&str; 2] = ["reader_navigation", "author_overflow"];
const LIFECYCLES: [&str; 13] = [
    "hot_live",
    "bounded_collection_index",
    "partitioned_collection",
    "bounded_pointer",
    "generated_projection",
    "maintained_reference",
    "maintained_entrypoint",
    "maintained_standard",
    "executable_reference",
    "executable_check",
    "data_registry",
    "external_service",
    "append_only_history",
];
/// The pressure controls that are a review or a query rather than a script.
const REVIEW_CONTROLS: [&str; 5] = [
    "review_on_behavior_change",
    "review_on_workflow_change",
    "review_on_tooling_change",
    "github_issue_retention_and_query",
    "query_with_git_log",
];
/// The directories a script control may live in.
const SCRIPT_CONTROL_DIRECTORIES: [&str; 2] = ["scripts/", "knowledge-map/scripts/"];
/// Where an author sends detail the README does not carry; each must be an `author_overflow` route.
const AUTHOR_OVERFLOW: [&str; 7] = [
    "docs/book/src/SUMMARY.md",
    "ROADMAP.md",
    "docs/tasks/",
    "docs/decisions/",
    "git:history",
    "docs/book/src/reference/troubleshooting.md",
    "TOOLBOX.md",
];

/// Run the check from `root`, or with `--self-test` prove that each ceiling and refusal fails closed; returns the
/// exit code `scripts/check_readme_policy.sh` gave.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    match arguments {
        [] => {}
        [mode] if mode == "--self-test" => return self_test(root, output),
        _ => {
            output.err("Usage: cargo xtask check readme-policy [--self-test]\n");
            return 2;
        }
    }
    let problems = problems(root);
    if problems.is_empty() {
        output.out(
            "readme-policy: README.md and all registered entrypoint routes satisfy the local contract.\n",
        );
        return 0;
    }
    for problem in &problems {
        output.err(&format!("readme-policy: {problem}\n"));
    }
    output.err(
        "readme-policy: FAILED — repair README.md, README_POLICY.md, or the registered route/control instead of widening a ceiling.\n",
    );
    1
}

/// Every way the tree under `root` breaks the contract, in the order the script reported them.
pub fn problems(root: &Path) -> Vec<String> {
    let mut problems = Vec::new();
    let readme = read_file(&root.join(README));
    match &readme {
        None => {
            problems.push("README.md is missing; restore the project landing page.".to_string())
        }
        Some(bytes) => problems.extend(readme_size_problems(line_count(bytes), bytes.len())),
    }
    match read_file(&root.join(POLICY)) {
        None => problems
            .push("README_POLICY.md is missing; restore the repository-owned policy.".to_string()),
        Some(bytes) => {
            let policy = String::from_utf8_lossy(&bytes);
            if !policy.contains(POLICY_FENCE) {
                problems.push("README_POLICY.md lacks its fenced local adoption note.".to_string());
            }
            if !policy.contains(POLICY_CEILINGS) {
                problems.push(
                    "README_POLICY.md and the enforced line/byte ceilings disagree.".to_string(),
                );
            }
        }
    }
    let Some(routes) = read_file(&root.join(ROUTES)) else {
        problems.push(format!(
            "{ROUTES} is missing; restore the governed route inventory."
        ));
        return problems;
    };
    let text = String::from_utf8_lossy(&routes);
    let records = records(&text);
    if records.first().copied().unwrap_or("") != HEADER {
        problems.push(
            "the README route registry header does not match its six-column schema.".to_string(),
        );
    }
    let widest = records_of(&routes)
        .iter()
        .map(|record| record.strip_suffix(b"\r").unwrap_or(record).len())
        .max()
        .unwrap_or(0);
    problems.extend(registry_size_problems(
        records.len().saturating_sub(1),
        routes.len(),
        widest,
    ));
    // Every row after the header, the last one too when it has no newline: the script's `while read` loop dropped
    // that row unchecked (`RUSTDOC-AND-XTASK.12`).
    let rows = records.get(1..).unwrap_or(&[]);
    let mut seen = HashSet::new();
    for (index, row) in rows.iter().copied().enumerate() {
        problems.extend(row_problems(root, index + 2, row, &mut seen));
    }
    if rows.is_empty() {
        problems.push("the README route registry has no route records.".to_string());
    }
    if let Some(readme) = &readme {
        for target in reader_routes(&String::from_utf8_lossy(readme)) {
            if !registered(&records, target, "reader_navigation") {
                problems.push(format!(
                    "README reader route '{target}' is not registered as reader_navigation."
                ));
            }
        }
    }
    for target in AUTHOR_OVERFLOW {
        if !registered(&records, target, "author_overflow") {
            problems.push(format!(
                "author-overflow route '{target}' is missing from the registry."
            ));
        }
    }
    problems
}

/// One registry row's problems; `number` is its line in the file, and `seen` the kind/target keys before it.
fn row_problems<'a>(
    root: &Path,
    number: usize,
    row: &'a str,
    seen: &mut HashSet<(&'a str, &'a str)>,
) -> Vec<String> {
    let mut problems = Vec::new();
    // Exactly six non-empty columns. The script's `IFS=$'\t' read` merged a run of tabs into one separator, so a
    // row with an empty seventh column, or a leading or trailing tab, read as complete (`RUSTDOC-AND-XTASK.12`).
    let fields: Vec<&str> = row.split('\t').collect();
    let &[target, route_kind, _owner, lifecycle, control, terminal] = fields.as_slice() else {
        problems.push(format!(
            "route registry row {number} is incomplete or has extra columns."
        ));
        return problems;
    };
    if fields.iter().any(|field| field.is_empty()) {
        problems.push(format!(
            "route registry row {number} is incomplete or has extra columns."
        ));
        return problems;
    }
    if !ROUTE_KINDS.contains(&route_kind) {
        problems.push(format!(
            "route registry row {number} has unknown route kind '{route_kind}'."
        ));
    }
    if !lifecycle_is_known(lifecycle) {
        problems.push(format!(
            "route '{target}' has unknown lifecycle '{lifecycle}'."
        ));
    }
    if terminal != "yes" {
        problems.push(format!("route '{target}' is not a controlled terminal; route chains and cycles are not allowed in this registry."));
    }
    if !seen.insert((route_kind, target)) {
        problems.push(format!(
            "route '{target}' is duplicated for '{route_kind}'."
        ));
    }
    if !target_shape_is_allowed(target) {
        problems.push(format!("route '{target}' is not repository-relative."));
    } else if !EXTERNAL_TARGETS.contains(&target)
        && !root
            .join(target.strip_suffix('/').unwrap_or(target))
            .exists()
    {
        problems.push(format!(
            "route '{target}' does not resolve inside the repository."
        ));
    }
    if !control_is_known(control) {
        problems.push(format!(
            "route '{target}' has unknown pressure control '{control}'."
        ));
    } else if is_script_control(control) && !is_executable(&root.join(control)) {
        problems.push(format!(
            "route '{target}' names missing or non-executable control '{control}'."
        ));
    }
    problems
}

/// The README's size problems against [`LINE_CAP`] and [`BYTE_CAP`].
fn readme_size_problems(lines: usize, bytes: usize) -> Vec<String> {
    let mut problems = Vec::new();
    if lines > LINE_CAP {
        problems.push(format!("README.md has {lines} lines; the reviewed ceiling is {LINE_CAP}. Route changing detail through README_POLICY.md."));
    }
    if bytes > BYTE_CAP {
        problems.push(format!("README.md has {bytes} bytes; the reviewed ceiling is {BYTE_CAP}. Route changing detail through README_POLICY.md."));
    }
    problems
}

/// The route registry's size problems against its three ceilings.
fn registry_size_problems(records: usize, bytes: usize, widest: usize) -> Vec<String> {
    let mut problems = Vec::new();
    if records > ROUTE_RECORD_CAP {
        problems.push(format!(
            "the README route registry has {records} records; the reviewed ceiling is {ROUTE_RECORD_CAP}."
        ));
    }
    if bytes > ROUTE_BYTE_CAP {
        problems.push(format!(
            "the README route registry has {bytes} bytes; the reviewed ceiling is {ROUTE_BYTE_CAP}."
        ));
    }
    if widest > ROUTE_RECORD_BYTE_CAP {
        problems.push(format!("the README route registry has a {widest}-byte raw record; the reviewed ceiling is {ROUTE_RECORD_BYTE_CAP}."));
    }
    problems
}

/// Whether a README of `lines` lines and `bytes` bytes is within its ceilings.
pub fn within_limits(lines: usize, bytes: usize) -> bool {
    readme_size_problems(lines, bytes).is_empty()
}

/// Whether a registry of `records` records, `bytes` bytes and a `widest`-byte record is within its ceilings.
pub fn route_registry_within_limits(records: usize, bytes: usize, widest: usize) -> bool {
    registry_size_problems(records, bytes, widest).is_empty()
}

/// Whether a route target names a place inside the repository (or one of the two external routes): not absolute,
/// not climbing out, no backslash.
pub fn target_shape_is_allowed(target: &str) -> bool {
    !(target.starts_with('/')
        || target.starts_with("../")
        || target.contains("/../")
        || target.contains('\\'))
}

/// Whether a pressure control is a reviewed kind: a script in `SCRIPT_CONTROL_DIRECTORIES` or a review.
pub fn control_is_known(control: &str) -> bool {
    is_script_control(control) || REVIEW_CONTROLS.contains(&control)
}

fn is_script_control(control: &str) -> bool {
    SCRIPT_CONTROL_DIRECTORIES
        .iter()
        .any(|directory| control.starts_with(directory))
        && control.ends_with(".sh")
}

/// Whether a route's lifecycle is one of the reviewed kinds.
pub fn lifecycle_is_known(lifecycle: &str) -> bool {
    LIFECYCLES.contains(&lifecycle)
}

/// Whether `target` is registered with `route_kind`: a row after the header whose first two tab-separated columns
/// are exactly those (the script's `awk -F '\t'` lookup).
fn registered(records: &[&str], target: &str, route_kind: &str) -> bool {
    records.iter().skip(1).any(|record| {
        let mut columns = record.split('\t');
        columns.next() == Some(target) && columns.next().unwrap_or("") == route_kind
    })
}

/// The README's link targets, as `grep -oE '\]\([^)]+\)'` found them, cut at the fragment; empty ones are dropped.
fn reader_routes(readme: &str) -> Vec<&str> {
    let mut targets = Vec::new();
    for line in readme.split('\n') {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            match after.find(')') {
                Some(end) if end > 0 => {
                    let target = &after[..end];
                    let target = target.split('#').next().unwrap_or("");
                    if !target.is_empty() {
                        targets.push(target);
                    }
                    rest = &after[end + 1..];
                }
                _ => rest = &rest[start + 1..],
            }
        }
    }
    targets
}

/// A text's records as awk reads them: one per newline, and a last one without its newline.
fn records(text: &str) -> Vec<&str> {
    let mut records: Vec<&str> = text.split('\n').collect();
    if text.is_empty() || text.ends_with('\n') {
        records.pop();
    }
    records
}

/// [`records`] over raw bytes, so a record's width is counted in bytes as `LC_ALL=C awk` counted it.
fn records_of(bytes: &[u8]) -> Vec<&[u8]> {
    let mut records: Vec<&[u8]> = bytes.split(|byte| *byte == b'\n').collect();
    if bytes.is_empty() || bytes.ends_with(b"\n") {
        records.pop();
    }
    records
}

/// The newlines in a file, as `wc -l` counts its lines.
fn line_count(bytes: &[u8]) -> usize {
    bytes.iter().filter(|byte| **byte == b'\n').count()
}

/// A regular file's bytes (following a symlink, as `[ -f ]` does); `None` when there is none to read.
fn read_file(path: &Path) -> Option<Vec<u8>> {
    if path.is_file() {
        fs::read(path).ok()
    } else {
        None
    }
}

/// Whether `path` exists with an execute bit, as `[ -x ]` reports it for the file's owner.
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(path).is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        path.exists()
    }
}

/// The script's self-test: each ceiling passes at its value and fails one above it, and each shape, control,
/// lifecycle and route refusal fails closed.
fn self_test(root: &Path, output: &mut dyn super::Output) -> i32 {
    match self_test_failure(root) {
        Some(failure) => {
            output.err(&format!("readme-policy self-test: {failure}\n"));
            1
        }
        None => {
            output.out(
                "readme-policy self-test: caps, path locality, controls, and route coverage fail closed\n",
            );
            0
        }
    }
}

/// The first self-test case that does not hold, as the script named it.
fn self_test_failure(root: &Path) -> Option<&'static str> {
    if !within_limits(LINE_CAP, BYTE_CAP) {
        return Some("exact ceilings must pass");
    }
    if within_limits(LINE_CAP + 1, BYTE_CAP) {
        return Some("line ceiling did not fail closed");
    }
    if within_limits(LINE_CAP, BYTE_CAP + 1) {
        return Some("byte ceiling did not fail closed");
    }
    if !route_registry_within_limits(ROUTE_RECORD_CAP, ROUTE_BYTE_CAP, ROUTE_RECORD_BYTE_CAP) {
        return Some("exact route-registry ceilings must pass");
    }
    if route_registry_within_limits(ROUTE_RECORD_CAP + 1, ROUTE_BYTE_CAP, ROUTE_RECORD_BYTE_CAP) {
        return Some("route-record ceiling did not fail closed");
    }
    if route_registry_within_limits(ROUTE_RECORD_CAP, ROUTE_BYTE_CAP + 1, ROUTE_RECORD_BYTE_CAP) {
        return Some("route-registry byte ceiling did not fail closed");
    }
    if route_registry_within_limits(ROUTE_RECORD_CAP, ROUTE_BYTE_CAP, ROUTE_RECORD_BYTE_CAP + 1) {
        return Some("route-record byte ceiling did not fail closed");
    }
    if !target_shape_is_allowed("README.md") {
        return Some("repository-relative target was rejected");
    }
    if target_shape_is_allowed("/tmp/off-volume.md") || target_shape_is_allowed("../escape.md") {
        return Some("off-repository target did not fail closed");
    }
    if control_is_known("unreviewed_control") {
        return Some("unknown pressure control did not fail closed");
    }
    if control_is_known("transition_debt:LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5") {
        return Some("retired transition debt did not fail closed");
    }
    if lifecycle_is_known("unclassified_lifecycle") {
        return Some("unknown lifecycle did not fail closed");
    }
    let routes = read_file(&root.join(ROUTES)).unwrap_or_default();
    let text = String::from_utf8_lossy(&routes);
    if registered(
        &records(&text),
        "__self_test_missing_route__",
        "reader_navigation",
    ) {
        return Some("missing reader route did not fail closed");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A minimal tree that satisfies the contract, under the project-data scratch root.
    fn fixture(name: &str) -> PathBuf {
        let root =
            crate::repository_root().join(format!(".project-data/tmp/xtask-readme-policy-{name}"));
        let _ = fs::remove_dir_all(&root);
        for directory in [
            "docs/book/src/reference",
            "docs/tasks",
            "docs/decisions",
            "doctrine/readme_entrypoint",
            "scripts",
        ] {
            fs::create_dir_all(root.join(directory)).expect("directory");
        }
        for file in [
            "docs/book/src/SUMMARY.md",
            "ROADMAP.md",
            "docs/book/src/reference/troubleshooting.md",
            "TOOLBOX.md",
        ] {
            fs::write(root.join(file), "").expect("target");
        }
        fs::write(
            root.join(README),
            "# Fixture\n\nSee the [roadmap](ROADMAP.md#now).\n",
        )
        .expect("readme");
        fs::write(
            root.join(POLICY),
            format!("<!-- {POLICY_FENCE} -->\n{POLICY_CEILINGS} bytes.\n"),
        )
        .expect("policy");
        let control = root.join("scripts/check_fixture.sh");
        fs::write(&control, "#!/bin/sh\n").expect("control");
        set_mode(&control, 0o755);
        let mut routes = format!(
            "{HEADER}\nROADMAP.md\treader_navigation\tmaintainers\thot_live\tscripts/check_fixture.sh\tyes\n"
        );
        for target in AUTHOR_OVERFLOW {
            routes.push_str(&format!(
                "{target}\tauthor_overflow\tmaintainers\tmaintained_reference\treview_on_behavior_change\tyes\n"
            ));
        }
        fs::write(root.join(ROUTES), routes).expect("routes");
        root
    }

    fn set_mode(path: &Path, mode: u32) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("mode");
        }
        #[cfg(not(unix))]
        let _ = (path, mode);
    }

    fn append_route(root: &Path, row: &str) {
        let mut routes = fs::read_to_string(root.join(ROUTES)).expect("routes");
        routes.push_str(row);
        fs::write(root.join(ROUTES), routes).expect("routes");
    }

    /// The fixture's problems after `mutate`, its scratch tree removed.
    fn problems_after(name: &str, mutate: impl FnOnce(&Path)) -> Vec<String> {
        let root = fixture(name);
        mutate(&root);
        let found = problems(&root);
        fs::remove_dir_all(&root).expect("cleanup");
        found
    }

    #[test]
    fn a_conforming_tree_passes() {
        assert_eq!(problems_after("conforming", |_| {}), Vec::<String>::new());
    }

    /// A refusal case: its scratch tree's name, the breakage, and the problem it must report.
    type Case = (&'static str, fn(&Path), &'static str);

    #[test]
    fn each_refusal_names_its_problem() {
        let cases: [Case; 10] = [
            (
                "long-readme",
                |root| fs::write(root.join(README), "line\n".repeat(LINE_CAP + 1)).expect("readme"),
                "README.md has 151 lines; the reviewed ceiling is 150. Route changing detail through README_POLICY.md.",
            ),
            (
                "no-readme",
                |root| fs::remove_file(root.join(README)).expect("readme"),
                "README.md is missing; restore the project landing page.",
            ),
            (
                "unfenced-policy",
                |root| fs::write(root.join(POLICY), POLICY_CEILINGS).expect("policy"),
                "README_POLICY.md lacks its fenced local adoption note.",
            ),
            (
                "short-row",
                |root| append_route(root, "TOOLBOX.md\treader_navigation\tmaintainers\n"),
                "route registry row 10 is incomplete or has extra columns.",
            ),
            (
                "escaping-route",
                |root| {
                    append_route(
                        root,
                        "../escape.md\treader_navigation\tmaintainers\thot_live\treview_on_behavior_change\tyes\n",
                    )
                },
                "route '../escape.md' is not repository-relative.",
            ),
            (
                "duplicate-route",
                |root| {
                    append_route(
                        root,
                        "ROADMAP.md\treader_navigation\tmaintainers\thot_live\treview_on_behavior_change\tyes\n",
                    )
                },
                "route 'ROADMAP.md' is duplicated for 'reader_navigation'.",
            ),
            (
                "non-executable-control",
                |root| set_mode(&root.join("scripts/check_fixture.sh"), 0o644),
                "route 'ROADMAP.md' names missing or non-executable control 'scripts/check_fixture.sh'.",
            ),
            (
                "unregistered-link",
                |root| fs::write(root.join(README), "[tasks](TOOLBOX.md)\n").expect("readme"),
                "README reader route 'TOOLBOX.md' is not registered as reader_navigation.",
            ),
            (
                "trailing-tab",
                |root| {
                    append_route(
                        root,
                        "TOOLBOX.md\treader_navigation\tmaintainers\thot_live\treview_on_behavior_change\tyes\t\n",
                    )
                },
                "route registry row 10 is incomplete or has extra columns.",
            ),
            (
                "unterminated-last-row",
                |root| {
                    let routes = fs::read_to_string(root.join(ROUTES)).expect("routes");
                    let row = "TOOLBOX.md\treader_navigation\tmaintainers\thot_live\treview_on_behavior_change\tno";
                    fs::write(root.join(ROUTES), format!("{routes}{row}")).expect("routes");
                },
                "route 'TOOLBOX.md' is not a controlled terminal; route chains and cycles are not allowed in this registry.",
            ),
        ];
        for (name, mutate, expected) in cases {
            let found = problems_after(name, mutate);
            assert!(
                found.iter().any(|problem| problem == expected),
                "{name}: expected '{expected}', found {found:?}"
            );
        }
    }

    #[test]
    fn the_self_test_cases_keep_their_verdicts() {
        assert!(within_limits(LINE_CAP, BYTE_CAP));
        assert!(!within_limits(LINE_CAP + 1, BYTE_CAP));
        assert!(!within_limits(LINE_CAP, BYTE_CAP + 1));
        assert!(route_registry_within_limits(
            ROUTE_RECORD_CAP,
            ROUTE_BYTE_CAP,
            ROUTE_RECORD_BYTE_CAP
        ));
        assert!(!route_registry_within_limits(
            ROUTE_RECORD_CAP + 1,
            ROUTE_BYTE_CAP,
            ROUTE_RECORD_BYTE_CAP
        ));
        assert!(!route_registry_within_limits(
            ROUTE_RECORD_CAP,
            ROUTE_BYTE_CAP + 1,
            ROUTE_RECORD_BYTE_CAP
        ));
        assert!(!route_registry_within_limits(
            ROUTE_RECORD_CAP,
            ROUTE_BYTE_CAP,
            ROUTE_RECORD_BYTE_CAP + 1
        ));
        assert!(target_shape_is_allowed("README.md"));
        assert!(!target_shape_is_allowed("/tmp/off-volume.md"));
        assert!(!target_shape_is_allowed("../escape.md"));
        assert!(!control_is_known("unreviewed_control"));
        assert!(!control_is_known(
            "transition_debt:LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5"
        ));
        assert!(!lifecycle_is_known("unclassified_lifecycle"));
        assert_eq!(self_test_failure(&crate::repository_root()), None);
    }

    #[test]
    fn reader_routes_are_the_link_targets_grep_found() {
        let readme = "[a](x.md) [b]() [c](y.md#part) [d](#top)\n![e](docs/f.png)";
        assert_eq!(reader_routes(readme), ["x.md", "y.md", "docs/f.png"]);
    }

    #[test]
    fn a_stray_argument_exits_2() {
        let root = crate::repository_root();
        assert_eq!(run(&root, &["--selftest".to_string()]), 2);
        assert_eq!(run(&root, &["--self-test".to_string(), "x".to_string()]), 2);
    }
}
