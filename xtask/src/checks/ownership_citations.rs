//! The `OWNERSHIP-CITATIONS` doctrine (`LIVE-DOCUMENT-PRESSURE-HEADROOM.15`), native since
//! `RUSTDOC-AND-XTASK.13.d`: every work unit cited inside a declared ownership region is classified in
//! `doctrine/ownership_citations/citations.jsonl`, and every citation classified as a current owner names a unit
//! whose own `Status` line is still open. It proves the owner is open, never that the open owner is the right one.
//!
//! The registered script, `scripts/check_ownership_citations.pl`, keeps its argument handling (so its usage line and
//! exit codes are Perl's) and passes this check `--root DIR --registry PATH` and one of `--check`, `--report` or
//! `--self-test`.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use crate::pyjson::{self, Py};

const MIDDLE_DOT: char = '\u{b7}';

/// Run the check with the arguments the shim normalized.
pub fn run(_root: &Path, arguments: &[String]) -> i32 {
    let mut mode = "--check";
    let mut root = None;
    let mut registry = "doctrine/ownership_citations/citations.jsonl".to_string();
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" | "--report" | "--self-test" => mode = argument.as_str(),
            "--root" => root = arguments.next().map(PathBuf::from),
            "--registry" => match arguments.next() {
                Some(path) => registry = path.clone(),
                None => return usage(),
            },
            _ => return usage(),
        }
    }
    let Some(root) = root else {
        return usage();
    };
    let mut gate = Gate::new(root, registry);
    if mode == "--self-test" {
        return gate.self_test();
    }
    gate.check(mode == "--report")
}

fn usage() -> i32 {
    eprintln!(
        "Usage: cargo xtask check ownership-citations --root DIR [--registry PATH] [--check|--report|--self-test]"
    );
    2
}

/// A registry record, as the script read it: its string fields (Perl's stringification, `None` when absent or
/// `null`) and whether each is a reference (an array, an object or a JSON boolean).
#[derive(Debug, Clone, Default)]
struct Record {
    fields: HashMap<String, (Option<String>, bool)>,
}

impl Record {
    fn from(value: &Py) -> Self {
        let Py::Dict(pairs) = value else {
            return Self::default();
        };
        let fields = pairs
            .iter()
            .map(|(key, value)| (pyjson::to_text(key), perl_scalar(value)))
            .collect();
        Self { fields }
    }

    /// The field as Perl's `$record->{name}` stringifies it; `None` for `undef`.
    fn text(&self, name: &str) -> Option<&str> {
        self.fields.get(name).and_then(|(text, _)| text.as_deref())
    }

    fn is_reference(&self, name: &str) -> bool {
        self.fields
            .get(name)
            .is_some_and(|(_, reference)| *reference)
    }
}

/// A decoded JSON value as a Perl scalar: its string form and whether `ref()` is true. `JSON::PP` decodes booleans to
/// blessed objects (true as `1`, false as `0`), arrays and objects to references, `null` to `undef`, numbers to
/// Perl numbers (printed with `%.15g`).
fn perl_scalar(value: &Py) -> (Option<String>, bool) {
    match value {
        Py::None => (None, false),
        Py::Bool(flag) => (Some(if *flag { "1" } else { "0" }.to_string()), true),
        Py::Int(digits) => (Some(digits.clone()), false),
        Py::Float(number) => (Some(perl_number(*number)), false),
        Py::Str(text) => (Some(pyjson::to_text(text)), false),
        Py::List(_) | Py::Dict(_) => (Some(String::new()), true),
    }
}

/// A float as Perl prints it: `%.15g`.
fn perl_number(number: f64) -> String {
    if number.is_infinite() {
        return if number > 0.0 { "Inf" } else { "-Inf" }.to_string();
    }
    if number.is_nan() {
        return "NaN".to_string();
    }
    if number == 0.0 {
        return "0".to_string();
    }
    let exponent = number.abs().log10().floor() as i32;
    if (-4..15).contains(&exponent) {
        let decimals = (14 - exponent).max(0) as usize;
        let fixed = format!("{number:.decimals$}");
        let trimmed = if fixed.contains('.') {
            fixed.trim_end_matches('0').trim_end_matches('.')
        } else {
            &fixed
        };
        trimmed.to_string()
    } else {
        let scientific = format!("{number:.14e}");
        let (mantissa, power) = scientific.split_once('e').expect("exponent");
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        let power: i32 = power.parse().expect("power");
        format!(
            "{mantissa}e{}{:02}",
            if power < 0 { '-' } else { '+' },
            power.abs()
        )
    }
}

struct Gate {
    root: PathBuf,
    registry: String,
    errors: Vec<String>,
    tasks: HashMap<String, Option<String>>,
}

impl Gate {
    fn new(root: PathBuf, registry: String) -> Self {
        Self {
            root,
            registry,
            errors: Vec::new(),
            tasks: HashMap::new(),
        }
    }

    fn problem(&mut self, message: String) {
        self.errors.push(message);
    }

    fn absolute(&self, relative: &str) -> PathBuf {
        relative
            .split('/')
            .fold(self.root.clone(), |path, part| path.join(part))
    }

    fn check(&mut self, report: bool) -> i32 {
        let path = self.absolute(&self.registry.clone());
        let (regions, citations) = self.read_registry(&path);
        let mut classified: BTreeMap<(String, String), Record> = BTreeMap::new();
        for record in &citations {
            let key = (
                record.text("path").unwrap_or("").to_string(),
                record.text("unit").unwrap_or("").to_string(),
            );
            if classified.contains_key(&key) {
                self.problem(format!(
                    "duplicate citation record for '{}' in '{}'",
                    key.1, key.0
                ));
            }
            classified.insert(key, record.clone());
        }
        let mut observed = BTreeSet::new();
        let (mut region_count, mut citation_count) = (0, 0);
        for region in &regions {
            let path = region.text("path").unwrap_or("").to_string();
            let marker = region.text("marker").unwrap_or("").to_string();
            let Some(text) = self.slurp(&path) else {
                continue;
            };
            let units = self.region_units(&text, &path, &marker);
            region_count += 1;
            for unit in units {
                citation_count += 1;
                let key = (path.clone(), unit.clone());
                observed.insert(key.clone());
                let Some(record) = classified.get(&key) else {
                    self.problem(format!(
                        "'{path}' cites '{unit}' inside '{marker}' with no classification record; classify it as current_owner or historical"
                    ));
                    continue;
                };
                if record.text("kind") == Some("historical") {
                    continue;
                }
                match self.unit_status(&unit) {
                    Err(conflict) => self.problem(format!(
                        "'{path}' names '{unit}' as a current owner, but {conflict}"
                    )),
                    Ok(None) => self.problem(format!(
                        "'{path}' names '{unit}' as a current owner, but no such work unit exists"
                    )),
                    Ok(Some(status)) if status == "done" || status == "superseded" => {
                        self.problem(format!(
                            "'{path}' names '{unit}' as a current owner, but its own Status is '{status}'"
                        ))
                    }
                    Ok(Some(_)) => {}
                }
            }
        }
        for (path, unit) in classified.keys() {
            if !observed.contains(&(path.clone(), unit.clone())) {
                self.problem(format!(
                    "citation record for '{unit}' in '{path}' matches no citation in a declared region"
                ));
            }
        }
        if !self.errors.is_empty() {
            for error in &self.errors {
                eprintln!("ownership-citations: {error}");
            }
            eprintln!(
                "ownership-citations: FAILED with {} violation(s).",
                self.errors.len()
            );
            return 1;
        }
        if report {
            let current = citations
                .iter()
                .filter(|record| record.text("kind") == Some("current_owner"))
                .count();
            println!(
                "{{\"citations\":{citation_count},\"current_owner\":{current},\"historical\":{},\"regions\":{region_count}}}",
                citations.len() - current
            );
        } else {
            println!(
                "ownership-citations: {citation_count} cited work units across {region_count} declared ownership regions resolve to open owners or a stated history."
            );
        }
        0
    }

    fn slurp(&mut self, relative: &str) -> Option<String> {
        if !safe_relative(relative) {
            self.problem(format!("declared path '{relative}' is unsafe"));
            return None;
        }
        let absolute = self.absolute(relative);
        if !absolute.is_file() {
            self.problem(format!("declared path '{relative}' is missing"));
            return None;
        }
        match fs::read(&absolute) {
            Ok(bytes) => Some(String::from_utf8_lossy(&bytes).into_owned()),
            Err(error) => {
                self.problem(format!("cannot read '{relative}': {error}"));
                None
            }
        }
    }

    /// The units a region cites: links into `docs/tasks/`, then backticked work-unit ids whose tree file exists,
    /// each once, sorted.
    fn region_units(&mut self, text: &str, path: &str, marker: &str) -> Vec<String> {
        let open = format!("<!-- {marker}:start -->");
        let close = format!("<!-- {marker}:end -->");
        let opens = unquoted(text, &open);
        let closes = unquoted(text, &close);
        if opens.len() != 1 || closes.len() != 1 {
            self.problem(format!(
                "'{path}' must contain exactly one '{marker}' start and end marker; found {} and {}",
                opens.len(),
                closes.len()
            ));
            return Vec::new();
        }
        let start = opens[0] + open.len();
        if closes[0] < start {
            self.problem(format!(
                "'{path}' has its '{marker}' end marker before its start marker"
            ));
            return Vec::new();
        }
        let body = &text[start..closes[0]];
        let mut units: Vec<String> = Vec::new();
        for unit in task_links(body) {
            if !units.contains(&unit) {
                units.push(unit);
            }
        }
        for unit in backticked_ids(body) {
            let tree = unit.split('.').next().unwrap_or("");
            if !self.absolute(&format!("docs/tasks/{tree}.md")).is_file() {
                continue;
            }
            if !units.contains(&unit) {
                units.push(unit);
            }
        }
        units.sort();
        units
    }

    /// The cited unit's own status: `Ok(None)` when it is declared nowhere, `Err` naming the conflict when its
    /// declarations disagree.
    fn unit_status(&mut self, unit: &str) -> Result<Option<String>, String> {
        let (tree, leaf) = match unit.split_once('.') {
            Some((tree, leaf)) => (tree, Some(leaf)),
            None => (unit, None),
        };
        let Some(text) = self.read_task(tree) else {
            return Ok(None);
        };
        if leaf.is_some() {
            let mut texts = vec![text];
            texts.extend(self.tree_part_texts(tree));
            return resolve_leaf_status(unit, &texts);
        }
        Ok(tree_status(&text))
    }

    fn read_task(&mut self, tree: &str) -> Option<String> {
        if let Some(cached) = self.tasks.get(tree) {
            return cached.clone();
        }
        let path = self.absolute(&format!("docs/tasks/{tree}.md"));
        let text = path
            .is_file()
            .then(|| fs::read(&path).ok())
            .flatten()
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned());
        self.tasks.insert(tree.to_string(), text.clone());
        text
    }

    /// A partitioned tree's part files under `docs/tasks/<tree in lower case>/`, in name order.
    fn tree_part_texts(&self, tree: &str) -> Vec<String> {
        let directory = self.absolute(&format!("docs/tasks/{}", tree.to_lowercase()));
        let Ok(entries) = fs::read_dir(&directory) else {
            return Vec::new();
        };
        let mut names: Vec<String> = entries
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".md") && directory.join(name).is_file())
            .collect();
        names.sort();
        names
            .iter()
            .filter_map(|name| fs::read(directory.join(name)).ok())
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .collect()
    }

    /// The registry's region and citation records; every shape error is a problem.
    fn read_registry(&mut self, path: &Path) -> (Vec<Record>, Vec<Record>) {
        let (mut regions, mut citations) = (Vec::new(), Vec::new());
        if !path.is_file() {
            let registry = self.registry.clone();
            self.problem(format!("citation registry is missing: {registry}"));
            return (regions, citations);
        }
        let text = match fs::read(path) {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(error) => {
                self.problem(format!("cannot read citation registry: {error}"));
                return (regions, citations);
            }
        };
        for (index, line) in text.split_inclusive('\n').enumerate() {
            let number = index + 1;
            let line = line.strip_suffix('\n').unwrap_or(line);
            if line.chars().all(char::is_whitespace) {
                continue;
            }
            let record = match pyjson::loads_strict(line) {
                Ok(value @ Py::Dict(_)) => Record::from(&value),
                _ => {
                    self.problem(format!(
                        "citation registry line {number} is not one JSON object"
                    ));
                    continue;
                }
            };
            let kind = record.text("record_type").unwrap_or("").to_string();
            match kind.as_str() {
                "registry" => {
                    if number != 1 {
                        self.problem("the registry header must be the first record".to_string());
                    }
                }
                "region" => {
                    if record.is_reference("path")
                        || !safe_relative(record.text("path").unwrap_or(""))
                    {
                        self.problem(format!("region record at line {number} needs a safe path"));
                    }
                    let marker = record.text("marker");
                    if record.is_reference("marker") || !marker.is_some_and(is_marker) {
                        self.problem(format!("region record at line {number} needs a marker"));
                    }
                    regions.push(record);
                }
                "citation" => {
                    if record.is_reference("path")
                        || !safe_relative(record.text("path").unwrap_or(""))
                    {
                        self.problem(format!(
                            "citation record at line {number} needs a safe path"
                        ));
                    }
                    if record.is_reference("unit") || record.text("unit").unwrap_or("").is_empty() {
                        self.problem(format!("citation record at line {number} needs a unit"));
                    }
                    let classification = record.text("kind").unwrap_or("");
                    if classification != "current_owner" && classification != "historical" {
                        self.problem(format!(
                            "citation record at line {number} must be current_owner or historical"
                        ));
                    }
                    if classification == "historical"
                        && (record.is_reference("reason")
                            || record.text("reason").unwrap_or("").is_empty())
                    {
                        self.problem(format!(
                            "historical citation at line {number} must state a reason"
                        ));
                    }
                    citations.push(record);
                }
                _ => self.problem(format!(
                    "citation registry line {number} has unknown record_type '{kind}'"
                )),
            }
        }
        if regions.is_empty() {
            self.problem("the citation registry declares no ownership region".to_string());
        }
        (regions, citations)
    }

    /// The script's self-test: the region grammar, the registry's refusals, and the status grammars pinned from the
    /// tracked population.
    fn self_test(&mut self) -> i32 {
        match self.self_test_failures() {
            Ok(checks) => {
                println!(
                    "ownership-citations: self-test {checks}/{EXPECTED_SELF_TEST_CHECKS} grammar and registry cases pass."
                );
                0
            }
            Err(failure) => {
                eprintln!("ownership-citations self-test: {failure}");
                1
            }
        }
    }

    fn self_test_failures(&mut self) -> Result<usize, String> {
        let mut checks = 0;
        let mut failures: Vec<String> = Vec::new();
        let sample = "intro\n<!-- current_owners:start -->\n- owned by [`OPEN-TREE`](docs/tasks/OPEN-TREE.md) and `CLOSED-TREE` before it\n<!-- current_owners:end -->\ntail [`OUTSIDE`](docs/tasks/OUTSIDE.md)\n";
        self.errors.clear();
        let units = self.region_units(sample, "fixture.md", "current_owners");
        if units.iter().any(|unit| unit == "OUTSIDE") {
            failures.push(String::from("a link outside the region was counted"));
        }
        if !units.iter().any(|unit| unit == "OPEN-TREE") {
            failures.push(String::from("the linked unit inside the region was missed"));
        }
        checks += 1;
        let refuses = |gate: &mut Self, text: &str, wanted: &str| -> bool {
            gate.errors.clear();
            gate.region_units(text, "fixture.md", "current_owners");
            gate.errors.iter().any(|error| error.contains(wanted))
        };
        if !refuses(self, "no markers here\n", "must contain exactly one") {
            failures.push(String::from("a document with no marker pair was accepted"));
        }
        checks += 1;
        self.errors.clear();
        let quoting = sample.replacen(
            "tail ",
            "tail `<!-- current_owners:start -->` and `<!-- current_owners:end -->` ",
            1,
        );
        let quoted_units = self.region_units(&quoting, "fixture.md", "current_owners");
        if self
            .errors
            .iter()
            .any(|error| error.contains("must contain exactly one"))
        {
            failures.push(String::from(
                "a backtick-quoted marker was counted as a real one",
            ));
        }
        if quoted_units != units {
            failures.push(String::from(
                "quoting the marker changed the extracted citations",
            ));
        }
        checks += 1;
        if !refuses(
            self,
            &format!("{sample}{sample}"),
            "must contain exactly one",
        ) {
            failures.push(String::from("a duplicated marker pair was accepted"));
        }
        checks += 1;
        if !refuses(
            self,
            "<!-- current_owners:end -->\n<!-- current_owners:start -->\n",
            "end marker before its start marker",
        ) {
            failures.push(String::from("an inverted marker pair was accepted"));
        }
        checks += 1;
        let registry_refuses =
            |gate: &mut Self, text: &str, wanted: &str| -> Result<bool, String> {
                gate.errors.clear();
                let generated = gate.root.join("generated");
                let _ = fs::create_dir_all(&generated);
                let fixture = generated.join(format!(
                    ".ownership-citations-self-test.{}",
                    std::process::id()
                ));
                fs::write(&fixture, text)
                    .map_err(|error| format!("cannot write fixture: {error}"))?;
                gate.read_registry(&fixture);
                let _ = fs::remove_file(&fixture);
                Ok(gate.errors.iter().any(|error| error.contains(wanted)))
            };
        let region =
            "{\"record_type\":\"region\",\"path\":\"a.md\",\"marker\":\"current_owners\"}\n";
        if !registry_refuses(
            self,
            &format!(
                "{region}{{\"record_type\":\"citation\",\"path\":\"a.md\",\"unit\":\"X\",\"kind\":\"maybe\"}}\n"
            ),
            "must be current_owner or historical",
        )? {
            failures.push(String::from("an unknown classification was accepted"));
        }
        checks += 1;
        if !registry_refuses(
            self,
            &format!(
                "{region}{{\"record_type\":\"citation\",\"path\":\"a.md\",\"unit\":\"X\",\"kind\":\"historical\"}}\n"
            ),
            "must state a reason",
        )? {
            failures.push(String::from(
                "a historical classification with no reason was accepted",
            ));
        }
        checks += 1;
        if !registry_refuses(
            self,
            "{\"record_type\":\"citation\",\"path\":\"a.md\",\"unit\":\"X\",\"kind\":\"current_owner\"}\n",
            "declares no ownership region",
        )? {
            failures.push(String::from(
                "a registry with no declared region was accepted",
            ));
        }
        checks += 1;
        for (name, shape) in [
            (
                "annotated leaf status",
                "# T\n\n- ID: `T.1`\n  Status: `done` (`2026-08-31`, PROBE/DOC)\n",
            ),
            (
                "emphasised tree status",
                "# T\n\n- Status: **`active`** - UN-PARKED `2026-06-05`\n",
            ),
            ("plain tree status", "# T\n\n- Status: `active`\n"),
        ] {
            let parsed = if name == "annotated leaf status" {
                resolve_leaf_status("T.1", &[shape.to_string()])
                    .ok()
                    .flatten()
            } else {
                tree_status(shape)
            };
            if parsed.is_none() {
                failures.push(format!("the {name} shape did not parse"));
            }
            checks += 1;
        }
        let one_line = format!(
            "- ID: `T.2` {MIDDLE_DOT} Status: `open` (`2026-09-25`; CODE) {MIDDLE_DOT} Goal: x\n"
        );
        let status = |texts: &[String]| resolve_leaf_status("T.2", texts);
        if status(&[format!("# T\n\n{one_line}")]) != Ok(Some("open".to_string())) {
            failures.push(String::from("a one-line leaf in a root did not resolve"));
        }
        checks += 1;
        if status(&[
            "# T\n\n- Status: `active`\n".to_string(),
            format!("# part\n\n{one_line}"),
        ]) != Ok(Some("open".to_string()))
        {
            failures.push(String::from("a one-line leaf in a part did not resolve"));
        }
        checks += 1;
        let closed_line = one_line.replacen("`open`", "`done`", 1);
        if status(&[format!("# part\n\n{closed_line}")]) != Ok(Some("done".to_string())) {
            failures.push(String::from("a closed one-line leaf did not read as done"));
        }
        checks += 1;
        if !matches!(status(&[format!("# T\n\n{one_line}"), format!("# part\n\n{closed_line}")]), Err(conflict) if conflict.contains("disagree"))
        {
            failures.push(String::from(
                "disagreeing declarations were resolved to one status",
            ));
        }
        checks += 1;
        if status(&[format!("# T\n\n- ID: `T.20` {MIDDLE_DOT} Status: `open`\n")]) != Ok(None) {
            failures.push(String::from(
                "a longer id sharing the prefix was taken for the leaf",
            ));
        }
        checks += 1;
        self.errors.clear();
        if let Some(failure) = failures.first() {
            return Err(failure.clone());
        }
        if checks != EXPECTED_SELF_TEST_CHECKS {
            return Err(format!(
                "ran {checks} checks, declaration expects {EXPECTED_SELF_TEST_CHECKS}; re-derive the declaration beside the suite"
            ));
        }
        Ok(checks)
    }
}

/// The script's declared self-test total.
const EXPECTED_SELF_TEST_CHECKS: usize = 16;

/// The positions of each occurrence of `needle` not preceded by a backtick.
fn unquoted(text: &str, needle: &str) -> Vec<usize> {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(offset) = text[from..].find(needle) {
        let at = from + offset;
        if at == 0 || text.as_bytes()[at - 1] != b'`' {
            found.push(at);
        }
        from = at + 1;
    }
    found
}

/// `\]\(([^)]*?)docs/tasks/([A-Za-z0-9\-]+)\.md\)`, each match's unit, left to right.
fn task_links(body: &str) -> Vec<String> {
    const STEM: &str = "docs/tasks/";
    let mut units = Vec::new();
    let mut from = 0;
    'starts: while let Some(offset) = body[from..].find("](") {
        let start = from + offset;
        let span_end = body[start + 2..]
            .find(')')
            .map_or(body.len(), |close| start + 2 + close);
        let mut search = start + 2;
        while let Some(found) = body[search..span_end.min(body.len())].find(STEM) {
            let name_start = search + found + STEM.len();
            let name_len = body[name_start..]
                .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
                .unwrap_or(body.len() - name_start);
            if name_len > 0 && body[name_start + name_len..].starts_with(".md)") {
                units.push(body[name_start..name_start + name_len].to_string());
                from = name_start + name_len + 4;
                continue 'starts;
            }
            search = search + found + 1;
        }
        from = start + 1;
    }
    units
}

/// `` `([A-Z][A-Za-z0-9\-]*(?:\.[0-9a-z][0-9a-z.]*)?)` ``, each match's id, left to right.
fn backticked_ids(body: &str) -> Vec<String> {
    let bytes = body.as_bytes();
    let mut ids = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'`' && index + 1 < bytes.len() && bytes[index + 1].is_ascii_uppercase()
        {
            let mut end = index + 2;
            while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'-') {
                end += 1;
            }
            if end + 1 < bytes.len()
                && bytes[end] == b'.'
                && (bytes[end + 1].is_ascii_digit() || bytes[end + 1].is_ascii_lowercase())
            {
                end += 2;
                while end < bytes.len()
                    && (bytes[end].is_ascii_digit()
                        || bytes[end].is_ascii_lowercase()
                        || bytes[end] == b'.')
                {
                    end += 1;
                }
            }
            if end < bytes.len() && bytes[end] == b'`' {
                ids.push(body[index + 1..end].to_string());
                index = end + 1;
                continue;
            }
        }
        index += 1;
    }
    ids
}

/// `^- Status: [*_]{0,2}`([a-z_]+)``: a tree's own status line, the first one.
fn tree_status(text: &str) -> Option<String> {
    text.split('\n')
        .find_map(|line| status_after(line.strip_prefix("- Status: ")?))
}

/// `[*_]{0,2}`([a-z_]+)`` at the start of `text`.
fn status_after(text: &str) -> Option<String> {
    let marks = text
        .chars()
        .take_while(|character| *character == '*' || *character == '_')
        .count();
    for skip in (0..=marks.min(2)).rev() {
        let rest = &text[skip..];
        if let Some(rest) = rest.strip_prefix('`') {
            let word: String = rest
                .chars()
                .take_while(|character| character.is_ascii_lowercase() || *character == '_')
                .collect();
            if !word.is_empty() && rest[word.len()..].starts_with('`') {
                return Some(word);
            }
        }
    }
    None
}

/// A leaf's status from every declaration of it in `texts`, in either grammar; `Err` names a disagreement.
fn resolve_leaf_status(unit: &str, texts: &[String]) -> Result<Option<String>, String> {
    let prefix = format!("- ID: `{unit}`");
    let mut statuses = Vec::new();
    for text in texts {
        // The two-line form: the id line, whitespace holding a newline, then `Status:`.
        for (offset, _) in text.match_indices(&prefix) {
            if offset != 0 && text.as_bytes()[offset - 1] != b'\n' {
                continue;
            }
            let rest = &text[offset + prefix.len()..];
            let gap = rest.len() - rest.trim_start().len();
            if rest[..gap].contains('\n')
                && let Some(status) = rest[gap..].strip_prefix("Status: ").and_then(status_after)
            {
                statuses.push(status);
            }
        }
        // The one-line form: `- ID: `T.1` · Status: …`.
        let one_line = format!("{prefix} {MIDDLE_DOT} Status: ");
        for (offset, _) in text.match_indices(&one_line) {
            if (offset == 0 || text.as_bytes()[offset - 1] == b'\n')
                && let Some(status) = status_after(&text[offset + one_line.len()..])
            {
                statuses.push(status);
            }
        }
    }
    let Some(first) = statuses.first().cloned() else {
        return Ok(None);
    };
    let distinct: BTreeSet<&String> = statuses.iter().collect();
    if distinct.len() == 1 {
        return Ok(Some(first));
    }
    Err(format!(
        "its declarations disagree ({})",
        distinct.into_iter().cloned().collect::<Vec<_>>().join(", ")
    ))
}

fn is_marker(marker: &str) -> bool {
    let mut characters = marker.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}

/// `safe_relative`: repository-relative, no backslash, control character, `.`/`..` segment or empty segment.
fn safe_relative(path: &str) -> bool {
    if path.is_empty() || path.starts_with('/') || path.contains('\\') || path.contains("//") {
        return false;
    }
    if path
        .chars()
        .any(|character| (character as u32) < 0x20 || character as u32 == 0x7f)
    {
        return false;
    }
    !path
        .split('/')
        .any(|segment| segment == "." || segment == "..")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_test_cases_keep_their_verdicts() {
        let root =
            crate::repository_root().join(".project-data/tmp/xtask-ownership-citations-self-test");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("scratch root");
        let mut gate = Gate::new(root.clone(), String::new());
        let checks = gate.self_test_failures();
        fs::remove_dir_all(&root).expect("cleanup");
        assert_eq!(checks, Ok(EXPECTED_SELF_TEST_CHECKS));
    }

    #[test]
    fn links_and_backticked_ids_are_found_as_the_regexes_find_them() {
        let body = "[a](x/docs/tasks/ONE.md) [b](docs/tasks/TWO.mdx) [c](q docs/tasks/X.mdz docs/tasks/THREE.md) ](docs/tasks/four.md)";
        assert_eq!(task_links(body), ["ONE", "THREE", "four"]);
        let ids = "`T` `T.1` `T.1.a.` `T-2.b3` `t` `T.X` `T.1X` ``T`` `É`";
        assert_eq!(backticked_ids(ids), ["T", "T.1", "T.1.a.", "T-2.b3", "T"]);
    }

    #[test]
    fn statuses_follow_both_leaf_grammars() {
        let two_line = "- ID: `A.1`\n   \n  Status: `open`\n".to_string();
        assert_eq!(
            resolve_leaf_status("A.1", &[two_line]),
            Ok(Some("open".to_string()))
        );
        let same_line = "- ID: `A.1` Status: `open`\n".to_string();
        assert_eq!(resolve_leaf_status("A.1", &[same_line]), Ok(None));
        assert_eq!(
            tree_status("x\n- Status: ***`done`\n- Status: _`active`\n"),
            Some("active".to_string())
        );
    }

    #[test]
    fn perl_scalars_print_as_perl_prints_them() {
        for (number, perl) in [
            (1.0, "1"),
            (0.5, "0.5"),
            (1e20, "1e+20"),
            (1e15, "1e+15"),
            (123456789012345.0, "123456789012345"),
            (0.0001, "0.0001"),
            (0.00001, "1e-05"),
        ] {
            assert_eq!(perl_number(number), perl, "{number}");
        }
        assert_eq!(perl_scalar(&Py::Bool(true)), (Some("1".to_string()), true));
        assert_eq!(perl_scalar(&Py::None), (None, false));
    }

    #[test]
    fn a_closed_current_owner_is_refused_and_a_historical_one_is_not() {
        let root =
            crate::repository_root().join(".project-data/tmp/xtask-ownership-citations-check");
        let verdict = |kind: &str| -> (i32, Vec<String>) {
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(root.join("docs/tasks")).expect("fixture");
            fs::create_dir_all(root.join("doctrine/ownership_citations")).expect("fixture");
            fs::write(root.join("docs/tasks/OPEN.md"), "- Status: `active`\n").expect("tree");
            fs::write(root.join("docs/tasks/CLOSED.md"), "- Status: `done`\n").expect("tree");
            fs::write(
                root.join("doc.md"),
                "<!-- owners:start -->\n[`OPEN`](docs/tasks/OPEN.md) and `CLOSED`\n<!-- owners:end -->\n",
            )
            .expect("doc");
            let registry = format!(
                "{{\"record_type\":\"registry\"}}\n{{\"record_type\":\"region\",\"path\":\"doc.md\",\"marker\":\"owners\"}}\n{{\"record_type\":\"citation\",\"path\":\"doc.md\",\"unit\":\"OPEN\",\"kind\":\"current_owner\"}}\n{{\"record_type\":\"citation\",\"path\":\"doc.md\",\"unit\":\"CLOSED\",\"kind\":\"{kind}\",\"reason\":\"r\"}}\n"
            );
            fs::write(
                root.join("doctrine/ownership_citations/citations.jsonl"),
                registry,
            )
            .expect("registry");
            let mut gate = Gate::new(
                root.clone(),
                "doctrine/ownership_citations/citations.jsonl".to_string(),
            );
            let code = gate.check(false);
            let errors = gate.errors.clone();
            fs::remove_dir_all(&root).expect("cleanup");
            (code, errors)
        };
        let (code, errors) = verdict("current_owner");
        assert_eq!(code, 1);
        assert_eq!(
            errors,
            ["'doc.md' names 'CLOSED' as a current owner, but its own Status is 'done'"]
        );
        assert_eq!(verdict("historical"), (0, Vec::new()));
    }

    #[test]
    fn markers_and_paths_are_checked_as_the_script_checks_them() {
        assert_eq!(
            unquoted("<!-- m:start --> `<!-- m:start -->", "<!-- m:start -->"),
            [0]
        );
        assert!(
            safe_relative("docs/a.md")
                && !safe_relative("../a.md")
                && !safe_relative("a//b")
                && !safe_relative("a/./b")
        );
        assert!(is_marker("current_owners") && !is_marker("Current") && !is_marker("1st"));
    }
}
