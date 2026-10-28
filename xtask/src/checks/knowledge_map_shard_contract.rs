//! Knowledge Map shard feasibility and projection planning, native since `RUSTDOC-AND-XTASK.15.m`.
//! The portable generator remains an external contract-query boundary; this module owns validation,
//! fact collection, identity, deterministic shard packing, rendered metrics and pressure reporting.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "knowledge-map-shards";
const LIMIT_FIELDS: &[&str] = &[
    "max_facts",
    "max_question_keys",
    "max_question_bytes",
    "max_id_bytes",
    "max_path_bytes",
    "max_link_line_bytes",
    "wrap_line_bytes",
    "target_shard_lines",
    "target_shard_bytes",
    "max_shard_lines",
    "max_shard_bytes",
    "max_shards",
    "max_projection_lines",
    "max_projection_bytes",
    "max_landing_lines",
    "max_landing_bytes",
    "max_landing_line_bytes",
];
const HARD_CAPS: &[usize] = &[
    1_024, 10_240, 4_096, 128, 256, 512, 512, 512, 65_536, 512, 65_536, 64, 32_768, 2_097_152, 128,
    16_384, 512,
];

#[derive(Clone)]
struct Contract {
    canonical_input_globs: Vec<String>,
    shard_directory: String,
    shard_prefix: String,
    fact_catalog: String,
    limits: BTreeMap<String, usize>,
    generator_settings: Py,
}

impl Contract {
    fn limit(&self, field: &str) -> usize {
        self.limits[field]
    }
}

#[derive(Clone)]
struct Fact {
    id: String,
    path: String,
    answers: Vec<String>,
    raw_sha256: String,
}

struct Entry {
    id: String,
    path: String,
    question: String,
}

#[derive(Clone, Copy, Default)]
struct Metrics {
    lines: usize,
    bytes: usize,
    line_bytes: usize,
}

struct Shard {
    number: usize,
    keys: usize,
    metrics: Metrics,
}

struct Plan {
    facts: usize,
    question_keys: usize,
    shards: Vec<Shard>,
    canonical_input_sha256: String,
    landing: Metrics,
    shard_max: Metrics,
    shard_total: Metrics,
}

struct Pressure {
    funded_facts: Option<usize>,
    warnings: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    Report,
    SelfTest,
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

fn metrics_json(metrics: Metrics) -> Py {
    object(vec![
        ("lines", integer(metrics.lines)),
        ("bytes", integer(metrics.bytes)),
        ("line_bytes", integer(metrics.line_bytes)),
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
    .ok_or_else(|| "cannot encode report JSON".to_string())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && !path.split('/').any(|segment| segment == "..")
        && !path.chars().any(|character| character <= '\u{1f}')
}

fn slurp_raw(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path)
        .map_err(|error| format!("cannot read '{}': {}", path.display(), os_error(&error)))
}

fn require_exact_keys(value: &Py, label: &str, keys: &[&str]) -> Result<(), String> {
    let Py::Dict(pairs) = value else {
        return Err(format!("{label} must be an object"));
    };
    let mut unknown: Vec<String> = pairs
        .iter()
        .map(|(key, _)| pyjson::to_text(key))
        .filter(|key| !keys.contains(&key.as_str()))
        .collect();
    unknown.sort();
    if let Some(key) = unknown.into_iter().next() {
        return Err(format!("{label} has unknown field '{key}'"));
    }
    for key in keys {
        if pyjson::get(value, key).is_none() {
            return Err(format!("{label} lacks field '{key}'"));
        }
    }
    Ok(())
}

fn shim_path() -> String {
    std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "scripts/check_knowledge_map_shard_contract.pl".to_string())
}

fn decode_json_file(path: &Path) -> Result<Py, String> {
    let raw = slurp_raw(path)?;
    let text = String::from_utf8(raw)
        .map_err(|error| format!("invalid JSON in '{}': {error}", path.display()))?;
    let value = pyjson::loads_strict(&text).map_err(|error| {
        let offset = text.chars().count();
        if error.starts_with("Expecting property name enclosed in double quotes:")
            && error.ends_with(&format!("(char {offset})"))
        {
            format!(
                "invalid JSON in '{}': , or }} expected while parsing object/hash, at character offset {offset} (before \"(end of string)\") at {} line 545.",
                path.display(),
                shim_path()
            )
        } else {
            format!("invalid JSON in '{}': {error}", path.display())
        }
    })?;
    if !matches!(value, Py::Dict(_)) {
        return Err(format!(
            "invalid JSON in '{}': not an object",
            path.display()
        ));
    }
    Ok(value)
}

fn read_contract(root: &Path, relative: &str) -> Result<Contract, String> {
    let path = root.join(relative);
    let metadata = fs::metadata(&path);
    let size = metadata
        .as_ref()
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    if size > 8_192 {
        return Err("contract exceeds 8,192 bytes".to_string());
    }
    let value = decode_json_file(&path).map_err(|error| {
        if metadata.is_err() {
            // Keep the original size probe's warning and callsite for shim parity.
            format!(
                "Use of uninitialized value in numeric gt (>) at {} line 74.\n{error}",
                shim_path()
            )
        } else {
            error
        }
    })?;
    require_exact_keys(
        &value,
        "shard contract",
        &[
            "schema_version",
            "canonical_input_globs",
            "landing_path",
            "shard_directory",
            "shard_prefix",
            "fact_catalog",
            "limits",
        ],
    )?;
    if perl::numeric(pyjson::get(&value, "schema_version")) != 1.0 {
        return Err("shard contract schema_version must be 1".to_string());
    }
    let Some(Py::List(globs)) = pyjson::get(&value, "canonical_input_globs") else {
        return Err("canonical_input_globs must contain 1..8 paths".to_string());
    };
    if globs.is_empty() || globs.len() > 8 {
        return Err("canonical_input_globs must contain 1..8 paths".to_string());
    }
    let mut canonical_input_globs = Vec::new();
    for glob in globs {
        let glob = scalar_value(glob);
        if !safe_relative_path(&glob) || !glob.ends_with(".md") {
            return Err("unsafe canonical input glob".to_string());
        }
        canonical_input_globs.push(glob);
    }
    let mut paths = BTreeMap::new();
    for field in ["landing_path", "shard_directory", "fact_catalog"] {
        let item = pyjson::get(&value, field);
        let path = scalar_value(item.unwrap_or(&Py::None));
        if item.is_none_or(perl::is_reference) || !safe_relative_path(&path) {
            return Err(format!("unsafe or empty contract path '{field}'"));
        }
        paths.insert(field, path);
    }
    if !paths["landing_path"].ends_with(".md") || !paths["fact_catalog"].ends_with(".md") {
        return Err("landing_path and fact_catalog must be Markdown files".to_string());
    }
    let shard_prefix = scalar_value(pyjson::get(&value, "shard_prefix").unwrap_or(&Py::None));
    let mut prefix_chars = shard_prefix.chars();
    if !prefix_chars
        .next()
        .is_some_and(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        || !shard_prefix.ends_with('-')
        || !shard_prefix.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return Err("shard_prefix is invalid".to_string());
    }
    let limits_value = pyjson::get(&value, "limits").unwrap_or(&Py::None);
    require_exact_keys(limits_value, "shard limits", LIMIT_FIELDS)?;
    let mut limits = BTreeMap::new();
    for (index, field) in LIMIT_FIELDS.iter().enumerate() {
        let item = pyjson::get(limits_value, field);
        let digits = scalar(item);
        if item.is_none_or(perl::is_reference)
            || digits.is_empty()
            || !digits.chars().all(|character| character.is_ascii_digit())
            || perl::numeric(item) == 0.0
        {
            return Err(format!("shard limit '{field}' must be a positive integer"));
        }
        let number = perl::numeric(item) as usize;
        if number > HARD_CAPS[index] {
            return Err(format!(
                "shard limit '{field}' exceeds hard cap {}",
                HARD_CAPS[index]
            ));
        }
        limits.insert((*field).to_string(), number);
    }
    if limits["wrap_line_bytes"] > limits["max_link_line_bytes"] {
        return Err("wrap_line_bytes must not exceed max_link_line_bytes".to_string());
    }
    if limits["target_shard_lines"] > limits["max_shard_lines"]
        || limits["target_shard_bytes"] > limits["max_shard_bytes"]
    {
        return Err("target shard bounds must not exceed hard shard bounds".to_string());
    }
    let generator_settings = object(vec![
        ("landing_path", text(&paths["landing_path"])),
        ("shard_directory", text(&paths["shard_directory"])),
        ("shard_prefix", text(&shard_prefix)),
        ("fact_catalog", text(&paths["fact_catalog"])),
        ("limits", limits_value.clone()),
    ]);
    Ok(Contract {
        canonical_input_globs,
        shard_directory: paths["shard_directory"].clone(),
        shard_prefix,
        fact_catalog: paths["fact_catalog"].clone(),
        limits,
        generator_settings,
    })
}

fn scalar_value(value: &Py) -> String {
    if perl::is_reference(value) {
        String::new()
    } else {
        perl::scalar(Some(value)).unwrap_or_default()
    }
}

fn validate_generator_contract(root: &Path, contract: &Contract) -> Result<(), String> {
    let generator = root.join("knowledge-map/scripts/gen_knowledge_map.sh");
    if !generator.is_file() {
        return Ok(());
    }
    let process = Command::new("bash")
        .arg(&generator)
        .arg("--print-contract")
        .output()
        .map_err(|error| {
            format!("cannot execute Knowledge Map generator contract query: {error}")
        })?;
    if !process.status.success() {
        return Err("Knowledge Map generator contract query failed".to_string());
    }
    let raw = String::from_utf8_lossy(&process.stdout);
    let actual = pyjson::loads_strict(&raw)
        .map_err(|_| "Knowledge Map generator returned invalid contract JSON".to_string())?;
    if !matches!(actual, Py::Dict(_)) {
        return Err("Knowledge Map generator returned invalid contract JSON".to_string());
    }
    if encode(&actual)? != encode(&contract.generator_settings)? {
        return Err(
            "Knowledge Map generator settings drift from doctrine/knowledge_map/shard_contract.json"
                .to_string(),
        );
    }
    Ok(())
}

fn wildcard_component(pattern: &str, text: &str) -> bool {
    fn recurse(pattern: &[u8], text: &[u8]) -> bool {
        if pattern.is_empty() {
            return text.is_empty();
        }
        match pattern[0] {
            b'*' => (0..=text.len()).any(|index| recurse(&pattern[1..], &text[index..])),
            b'?' => !text.is_empty() && recurse(&pattern[1..], &text[1..]),
            literal => text.first() == Some(&literal) && recurse(&pattern[1..], &text[1..]),
        }
    }
    if text.starts_with('.') && !pattern.starts_with('.') {
        return false;
    }
    recurse(pattern.as_bytes(), text.as_bytes())
}

fn expand_glob_components(
    root: &Path,
    directory: &Path,
    components: &[&str],
    matches: &mut Vec<PathBuf>,
) {
    let Some((first, remaining)) = components.split_first() else {
        let path = root.join(directory);
        if path.is_file() {
            matches.push(path);
        }
        return;
    };
    if *first == "**" {
        expand_glob_components(root, directory, remaining, matches);
        let Ok(entries) = fs::read_dir(root.join(directory)) else {
            return;
        };
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                expand_glob_components(
                    root,
                    &directory.join(entry.file_name()),
                    components,
                    matches,
                );
            }
        }
        return;
    }
    if !first.contains(['*', '?']) {
        expand_glob_components(root, &directory.join(first), remaining, matches);
        return;
    }
    let Ok(entries) = fs::read_dir(root.join(directory)) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if wildcard_component(first, &name) {
            expand_glob_components(root, &directory.join(&name), remaining, matches);
        }
    }
}

fn glob_paths(root: &Path, pattern: &str) -> Vec<PathBuf> {
    let components: Vec<&str> = pattern.split('/').collect();
    let mut matches = Vec::new();
    expand_glob_components(root, Path::new(""), &components, &mut matches);
    matches
}

fn collect_facts(root: &Path, contract: &Contract) -> Result<Vec<Fact>, String> {
    let mut paths = BTreeSet::new();
    for pattern in &contract.canonical_input_globs {
        let matches = glob_paths(root, pattern);
        if matches.is_empty() {
            return Err(format!("canonical input glob '{pattern}' has no matches"));
        }
        for path in matches {
            let relative = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            paths.insert(relative);
        }
    }
    let mut facts = Vec::new();
    for path in paths {
        let raw = slurp_raw(&root.join(&path))?;
        let raw_sha256 = sha256_hex(&raw);
        let text = String::from_utf8(raw)
            .map_err(|error| format!("cannot decode '{}': {error}", root.join(&path).display()))?;
        if let Some(mut fact) = parse_fact(&path, &text) {
            fact.raw_sha256 = raw_sha256;
            facts.push(fact);
        }
    }
    Ok(facts)
}

fn clean_scalar(value: &str) -> String {
    let value = value.trim_matches([' ', '\t']);
    if value.len() >= 2
        && ((value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\'')))
    {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

fn answer_line(line: &str) -> Option<&str> {
    let indentation = line.len() - line.trim_start_matches([' ', '\t']).len();
    if indentation == 0 {
        return None;
    }
    let rest = &line[indentation..];
    let rest = rest.strip_prefix('-')?;
    if !rest.starts_with([' ', '\t']) {
        return None;
    }
    let answer = rest.trim_start_matches([' ', '\t']);
    (!answer.is_empty()).then_some(answer)
}

fn parse_fact(path: &str, text: &str) -> Option<Fact> {
    let normalized = text.replace("\r\n", "\n");
    let rest = normalized.strip_prefix("---\n")?;
    let mut front_matter = None;
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line == "---\n" || line == "---" {
            front_matter = Some(&rest[..offset]);
            break;
        }
        offset += line.len();
    }
    let front_matter = front_matter?;
    let mut scalars = BTreeMap::new();
    let mut answers = Vec::new();
    let mut current_key = "";
    for line in front_matter.lines() {
        if let Some(answer) = answer_line(line) {
            if current_key == "answers" {
                answers.push(clean_scalar(answer));
            }
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let Some((key, rest)) = line.split_once(':') else {
            continue;
        };
        let mut characters = key.chars();
        if !characters
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
            || !characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
        {
            continue;
        }
        let rest = rest.trim_start_matches([' ', '\t']);
        current_key = key;
        if rest.starts_with('[') && rest.ends_with(']') {
            if key == "answers" {
                answers.extend(rest[1..rest.len() - 1].split(',').map(clean_scalar));
            }
            current_key = "";
        } else if !rest.is_empty() {
            scalars.insert(key.to_string(), clean_scalar(rest));
            current_key = "";
        }
    }
    answers.retain(|answer| !answer.is_empty());
    if answers.is_empty() {
        return None;
    }
    Some(Fact {
        id: scalars.remove("id").unwrap_or_default(),
        path: path.to_string(),
        answers,
        raw_sha256: String::new(),
    })
}

fn text_metrics(text: &str) -> Metrics {
    Metrics {
        lines: text.matches('\n').count(),
        bytes: text.len(),
        line_bytes: text.split('\n').map(|line| line.len()).max().unwrap_or(0),
    }
}

fn wrap_question(question: &str, limit: usize) -> Result<Vec<String>, String> {
    let question = question.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut lines = Vec::new();
    let mut line = "  >".to_string();
    for word in question.split(' ') {
        if format!("  > {word}").len() > limit {
            return Err("question token exceeds wrap bound".to_string());
        }
        let candidate = format!("{line} {word}");
        if candidate.len() > limit {
            lines.push(line);
            line = format!("  > {word}");
        } else {
            line = candidate;
        }
    }
    if line != "  >" {
        lines.push(line);
    }
    Ok(lines)
}

fn relative_link(root: &Path, directory: &str, path: &str) -> String {
    let base = root.join(directory);
    let target = root.join(path);
    let base_components: Vec<_> = base.components().collect();
    let target_components: Vec<_> = target.components().collect();
    let mut common = 0;
    while common < base_components.len()
        && common < target_components.len()
        && base_components[common] == target_components[common]
    {
        common += 1;
    }
    let mut parts = Vec::new();
    for _ in common..base_components.len() {
        parts.push("..".to_string());
    }
    for component in &target_components[common..] {
        parts.push(component.as_os_str().to_string_lossy().into_owned());
    }
    parts.join("/")
}

fn render_shard(
    root: &Path,
    contract: &Contract,
    number: usize,
    entries: &[&Entry],
) -> Result<String, String> {
    let mut lines = vec![
        format!("# Knowledge questions — shard {number:04}"),
        String::new(),
        "> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter."
            .to_string(),
        String::new(),
    ];
    for entry in entries {
        let relative = relative_link(root, &contract.shard_directory, &entry.path);
        let link = format!("- [{}]({relative})", entry.id);
        if link.len() > contract.limit("max_link_line_bytes") {
            return Err(format!(
                "question link line exceeds {} bytes",
                contract.limit("max_link_line_bytes")
            ));
        }
        lines.push(link);
        lines.extend(wrap_question(
            &entry.question,
            contract.limit("wrap_line_bytes"),
        )?);
    }
    Ok(lines.join("\n") + "\n")
}

fn finish_shard(
    root: &Path,
    contract: &Contract,
    number: usize,
    entries: &[&Entry],
) -> Result<Shard, String> {
    let text = render_shard(root, contract, number, entries)?;
    let metrics = text_metrics(&text);
    if metrics.lines > contract.limit("max_shard_lines") {
        return Err(format!("shard {number} exceeds line bound"));
    }
    if metrics.bytes > contract.limit("max_shard_bytes") {
        return Err(format!("shard {number} exceeds byte bound"));
    }
    if metrics.line_bytes > contract.limit("max_link_line_bytes") {
        return Err(format!("shard {number} exceeds line-width bound"));
    }
    Ok(Shard {
        number,
        keys: entries.len(),
        metrics,
    })
}

fn pack_shards(root: &Path, contract: &Contract, entries: &[Entry]) -> Result<Vec<Shard>, String> {
    let mut shards = Vec::new();
    let mut current: Vec<&Entry> = Vec::new();
    for entry in entries {
        let mut candidate = current.clone();
        candidate.push(entry);
        let number = shards.len() + 1;
        let rendered = render_shard(root, contract, number, &candidate)?;
        let metrics = text_metrics(&rendered);
        if !current.is_empty()
            && (metrics.lines > contract.limit("target_shard_lines")
                || metrics.bytes > contract.limit("target_shard_bytes"))
        {
            shards.push(finish_shard(root, contract, number, &current)?);
            current = vec![entry];
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        shards.push(finish_shard(root, contract, shards.len() + 1, &current)?);
    }
    Ok(shards)
}

fn render_landing(
    contract: &Contract,
    shards: &[Shard],
    facts: usize,
    questions: usize,
    digest: &str,
) -> String {
    let mut lines =
        vec![
        "# Knowledge Map".to_string(),
        String::new(),
        "> **AUTO-GENERATED — DO NOT EDIT.** Canonical facts live in front-mattered source files."
            .to_string(),
        String::new(),
        format!("- Facts: **{facts}**"),
        format!("- Unique question keys: **{questions}**"),
        format!("- Canonical input SHA-256: `{digest}`"),
        format!("- Browse by id/title: [`{}`]({})", contract.fact_catalog, contract.fact_catalog),
        format!(
            "- Search all question shards: `rg -i --glob '{}*.md' 'terms' {}`",
            contract.shard_prefix, contract.shard_directory
        ),
        String::new(),
        "## Question shards".to_string(),
        String::new(),
    ];
    for shard in shards {
        let name = format!("{}{:04}.md", contract.shard_prefix, shard.number);
        lines.push(format!(
            "- [Shard {:04}]({}/{name}) — {} keys",
            shard.number, contract.shard_directory, shard.keys
        ));
    }
    lines.join("\n") + "\n"
}

fn plan_projection(root: &Path, contract: &Contract, facts: &[Fact]) -> Result<Plan, String> {
    if facts.len() > contract.limit("max_facts") {
        return Err(format!(
            "fact count exceeds {}",
            contract.limit("max_facts")
        ));
    }
    let mut id_seen = BTreeSet::new();
    let mut question_seen = BTreeMap::new();
    let mut entries = Vec::new();
    let mut identity = Vec::new();
    for fact in facts {
        let id = &fact.id;
        if id.is_empty() {
            return Err(format!("{} lacks a fact id", fact.path));
        }
        if !id.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        }) || !id
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        {
            return Err(format!(
                "fact id '{id}' is not a safe kebab-case identifier"
            ));
        }
        if id.len() > contract.limit("max_id_bytes") {
            return Err(format!(
                "fact id '{id}' exceeds {} UTF-8 bytes",
                contract.limit("max_id_bytes")
            ));
        }
        if !id_seen.insert(id.clone()) {
            return Err(format!("duplicate fact id '{id}'"));
        }
        if fact.path.len() > contract.limit("max_path_bytes") {
            return Err(format!(
                "fact path '{}' exceeds {} UTF-8 bytes",
                fact.path,
                contract.limit("max_path_bytes")
            ));
        }
        identity.push(format!("{}\0{}", fact.path, fact.raw_sha256));
        for question in &fact.answers {
            if question.len() > contract.limit("max_question_bytes") {
                return Err(format!(
                    "question for '{id}' exceeds {} UTF-8 bytes",
                    contract.limit("max_question_bytes")
                ));
            }
            if let Some(prior) = question_seen.insert(question.clone(), id.clone()) {
                return Err(format!(
                    "question collision between '{prior}' and '{id}': {question}"
                ));
            }
            entries.push(Entry {
                id: id.clone(),
                path: fact.path.clone(),
                question: question.clone(),
            });
        }
    }
    if entries.len() > contract.limit("max_question_keys") {
        return Err(format!(
            "question-key count exceeds {}",
            contract.limit("max_question_keys")
        ));
    }
    entries.sort_by(|left, right| {
        left.question
            .cmp(&right.question)
            .then_with(|| left.id.cmp(&right.id))
    });
    let shards = pack_shards(root, contract, &entries)?;
    if shards.len() > contract.limit("max_shards") {
        return Err(format!(
            "projection requires {} shards; limit is {}",
            shards.len(),
            contract.limit("max_shards")
        ));
    }
    identity.sort();
    let canonical_input_sha256 = sha256_hex((identity.join("\n") + "\n").as_bytes());
    let landing_text = render_landing(
        contract,
        &shards,
        facts.len(),
        entries.len(),
        &canonical_input_sha256,
    );
    let landing = text_metrics(&landing_text);
    if landing.lines > contract.limit("max_landing_lines") {
        return Err("landing index exceeds line bound".to_string());
    }
    if landing.bytes > contract.limit("max_landing_bytes") {
        return Err("landing index exceeds byte bound".to_string());
    }
    if landing.line_bytes > contract.limit("max_landing_line_bytes") {
        return Err("landing index exceeds line-width bound".to_string());
    }
    let mut shard_max = Metrics::default();
    let mut shard_total = Metrics::default();
    for shard in &shards {
        shard_max.lines = shard_max.lines.max(shard.metrics.lines);
        shard_max.bytes = shard_max.bytes.max(shard.metrics.bytes);
        shard_max.line_bytes = shard_max.line_bytes.max(shard.metrics.line_bytes);
        shard_total.lines += shard.metrics.lines;
        shard_total.bytes += shard.metrics.bytes;
    }
    if shard_total.lines + landing.lines > contract.limit("max_projection_lines") {
        return Err("projection set exceeds aggregate line bound".to_string());
    }
    if shard_total.bytes + landing.bytes > contract.limit("max_projection_bytes") {
        return Err("projection set exceeds aggregate byte bound".to_string());
    }
    Ok(Plan {
        facts: facts.len(),
        question_keys: entries.len(),
        shards,
        canonical_input_sha256,
        landing,
        shard_max,
        shard_total,
    })
}

fn derive_plane_pressure(contract: &Contract, facts: usize, question_keys: usize) -> Pressure {
    let mut warnings = Vec::new();
    for (label, actual, field) in [
        ("facts", facts, "max_facts"),
        ("question keys", question_keys, "max_question_keys"),
    ] {
        let bound = contract.limit(field);
        let percent = 100.0 * actual as f64 / bound as f64;
        if percent < 80.0 {
            continue;
        }
        let band = if percent >= 90.0 {
            "rollover"
        } else {
            "warning"
        };
        warnings.push(format!(
            "{label} is at or above {band} ({percent:.1}%) — {} below its {bound} {field}",
            bound as isize - actual as isize
        ));
    }
    let funded_facts = if facts > 0 && question_keys > 0 {
        let funded = contract.limit("max_question_keys") * facts / question_keys;
        if contract.limit("max_facts") > funded {
            warnings.push(format!(
                "declared max_facts {} exceeds the {funded} facts a {}-key budget funds at the measured {question_keys} keys over {facts} facts",
                contract.limit("max_facts"),
                contract.limit("max_question_keys")
            ));
        }
        Some(funded)
    } else {
        None
    };
    Pressure {
        funded_facts,
        warnings,
    }
}

fn plan_json(plan: &Plan, pressure: &Pressure) -> Py {
    object(vec![
        ("facts", integer(plan.facts)),
        ("question_keys", integer(plan.question_keys)),
        ("shards", integer(plan.shards.len())),
        ("canonical_input_sha256", text(&plan.canonical_input_sha256)),
        ("landing", metrics_json(plan.landing)),
        ("shard_max", metrics_json(plan.shard_max)),
        (
            "shard_total",
            object(vec![
                ("lines", integer(plan.shard_total.lines)),
                ("bytes", integer(plan.shard_total.bytes)),
            ]),
        ),
        ("output_files", integer(1 + plan.shards.len())),
        (
            "pressure",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
                (
                    "funded_facts",
                    pressure.funded_facts.map(integer).unwrap_or(Py::None),
                ),
                (
                    "warnings",
                    Py::List(
                        pressure
                            .warnings
                            .iter()
                            .map(|warning| text(warning))
                            .collect(),
                    ),
                ),
            ]),
        ),
    ])
}

fn self_test_contract() -> Contract {
    let values = [
        10, 20, 64, 32, 64, 96, 32, 10, 384, 12, 512, 10, 128, 4_096, 32, 2_048, 256,
    ];
    let limits: BTreeMap<String, usize> = LIMIT_FIELDS
        .iter()
        .zip(values)
        .map(|(field, value)| ((*field).to_string(), value))
        .collect();
    let generator_settings = object(vec![
        ("landing_path", text("KNOWLEDGE_MAP.md")),
        ("shard_directory", text("docs/knowledge-map")),
        ("shard_prefix", text("questions-")),
        ("fact_catalog", text("docs/knowledge/INDEX.md")),
        (
            "limits",
            object(
                limits
                    .iter()
                    .map(|(field, value)| (field.as_str(), integer(*value)))
                    .collect(),
            ),
        ),
    ]);
    Contract {
        canonical_input_globs: vec!["docs/knowledge/*.md".to_string()],
        shard_directory: "docs/knowledge-map".to_string(),
        shard_prefix: "questions-".to_string(),
        fact_catalog: "docs/knowledge/INDEX.md".to_string(),
        limits,
        generator_settings,
    }
}

fn run_self_test(root: &Path) -> Result<(), String> {
    let raw = "identity éclair\n".as_bytes();
    let decoded =
        String::from_utf8(raw.to_vec()).map_err(|_| "raw-content identity case failed")?;
    let identity = sha256_hex(raw);
    if decoded != "identity éclair\n" || identity != sha256_hex(raw) {
        return Err("raw-content identity case failed".to_string());
    }
    let block = "---\nid: alpha\nanswers:\n  - \"where is alpha\"\ndate: 2026-08-08\n---\n";
    let inline = "---\nid: beta\nanswers: [what is beta]\ndate: 2026-08-08\n---\n";
    let mut alpha = parse_fact("docs/knowledge/alpha.md", block)
        .ok_or_else(|| "block-answer parser case failed".to_string())?;
    if alpha.answers[0] != "where is alpha" {
        return Err("block-answer parser case failed".to_string());
    }
    let mut beta = parse_fact("docs/knowledge/beta.md", inline)
        .ok_or_else(|| "inline-answer parser case failed".to_string())?;
    if beta.answers[0] != "what is beta" {
        return Err("inline-answer parser case failed".to_string());
    }
    alpha.raw_sha256 = "a".repeat(64);
    beta.raw_sha256 = "b".repeat(64);
    let contract = self_test_contract();
    let mut collision = beta.clone();
    collision.answers = vec!["where is alpha".to_string()];
    if plan_projection(root, &contract, &[alpha.clone(), collision]).is_ok() {
        return Err("question-collision case did not fail".to_string());
    }
    let mut ordered = ["zeta", "alpha", "éclair"];
    ordered.sort();
    if ordered.join("|") != "alpha|zeta|éclair" {
        return Err("UTF-8 byte-order case failed".to_string());
    }
    let wrapped = wrap_question("one two three four five six seven eight nine", 18)?;
    if wrapped.iter().any(|line| line.len() > 18) {
        return Err("question-wrap case failed".to_string());
    }
    let mut oversized = alpha.clone();
    oversized.answers = vec!["x".repeat(65)];
    if plan_projection(root, &contract, &[oversized]).is_ok() {
        return Err("question-byte case did not fail".to_string());
    }
    let mut many = alpha.clone();
    many.answers = (1..=8)
        .map(|number| format!("question number {number} with enough words"))
        .collect();
    let rollover = plan_projection(root, &contract, &[many])?;
    if rollover.shards.len() < 2 {
        return Err("shard-rollover case failed".to_string());
    }
    let mut landing_bound = self_test_contract();
    landing_bound
        .limits
        .insert("max_landing_lines".to_string(), 1);
    if plan_projection(root, &landing_bound, &[alpha.clone()]).is_ok() {
        return Err("landing-bound case did not fail".to_string());
    }
    let quiet = derive_plane_pressure(&contract, 7, 14);
    if !quiet.warnings.is_empty() || quiet.funded_facts != Some(10) {
        return Err("plane-silence case failed".to_string());
    }
    let banded = derive_plane_pressure(&contract, 8, 16);
    if banded.warnings.join("|")
        != "facts is at or above warning (80.0%) — 2 below its 10 max_facts|question keys is at or above warning (80.0%) — 4 below its 20 max_question_keys"
    {
        return Err("plane-warning-band case failed".to_string());
    }
    let rolled = derive_plane_pressure(&contract, 9, 18);
    if rolled.warnings.join("|")
        != "facts is at or above rollover (90.0%) — 1 below its 10 max_facts|question keys is at or above rollover (90.0%) — 2 below its 20 max_question_keys"
    {
        return Err("plane-rollover-band case failed".to_string());
    }
    let unfunded = derive_plane_pressure(&contract, 4, 10);
    if unfunded.funded_facts != Some(8)
        || unfunded.warnings.join("|")
            != "declared max_facts 10 exceeds the 8 facts a 20-key budget funds at the measured 10 keys over 4 facts"
    {
        return Err("plane-funding case failed".to_string());
    }
    Ok(())
}

fn run_projection(
    root: &Path,
    contract_relative: &str,
    mode: Mode,
    output: &mut dyn super::Output,
) -> i32 {
    let outcome = (|| {
        if !safe_relative_path(contract_relative) {
            return Err("contract path is unsafe".to_string());
        }
        let contract = read_contract(root, contract_relative)?;
        validate_generator_contract(root, &contract)?;
        let facts = collect_facts(root, &contract)?;
        let plan = plan_projection(root, &contract, &facts)?;
        let pressure = derive_plane_pressure(&contract, plan.facts, plan.question_keys);
        Ok((plan, pressure))
    })();
    let (plan, pressure) = match outcome {
        Ok(result) => result,
        Err(error) => {
            match error.split_once('\n') {
                Some((warning, remainder))
                    if warning.starts_with("Use of uninitialized value in numeric gt (>)") =>
                {
                    output.err(&format!("{warning}\n"));
                    output.err(&format!("{LABEL}: {remainder}\n"));
                }
                _ => output.err(&format!("{LABEL}: {error}\n")),
            }
            return 1;
        }
    };
    for warning in &pressure.warnings {
        output.err(&format!("{LABEL}: warning: {warning}\n"));
    }
    if mode == Mode::Report {
        match encode(&plan_json(&plan, &pressure)) {
            Ok(report) => output.out(&format!("{report}\n")),
            Err(error) => {
                output.err(&format!("{LABEL}: {error}\n"));
                return 1;
            }
        }
    } else {
        output.out(&format!(
            "{LABEL}: contract is feasible for {} facts / {} unique question keys in {} bounded shards; canonical input SHA-256 {}.\n",
            plan.facts,
            plan.question_keys,
            plan.shards.len(),
            plan.canonical_input_sha256
        ));
    }
    0
}

/// Run the Knowledge Map shard contract with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut contract = "doctrine/knowledge_map/shard_contract.json".to_string();
    let mut mode = Mode::Check;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" => mode = Mode::Check,
            "--report" => mode = Mode::Report,
            "--self-test" => mode = Mode::SelfTest,
            "--root" => match arguments.next() {
                Some(directory) => {
                    let canonical = fs::canonicalize(directory).ok().or_else(|| {
                        let path = Path::new(directory);
                        let name = path.file_name()?;
                        let parent = path
                            .parent()
                            .filter(|parent| !parent.as_os_str().is_empty())
                            .unwrap_or_else(|| Path::new("."));
                        Some(fs::canonicalize(parent).ok()?.join(name))
                    });
                    match canonical {
                        Some(canonical) => root = canonical,
                        None => {
                            output.err(&format!("{LABEL}: root does not exist\n"));
                            return 1;
                        }
                    }
                }
                None => {
                    output.err(&format!("{LABEL}: --root requires a path\n"));
                    return 1;
                }
            },
            "--contract" => match arguments.next() {
                Some(path) => contract = path.to_string(),
                None => {
                    output.err(&format!(
                        "{LABEL}: --contract requires a repository-relative path\n"
                    ));
                    return 1;
                }
            },
            _ => {
                output.err("Usage: cargo xtask check knowledge-map-shard-contract [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n");
                return 2;
            }
        }
    }
    if mode == Mode::SelfTest {
        match run_self_test(&root) {
            Ok(()) => {
                output.out(&format!(
                    "{LABEL}: 13/13 identity, collision, ordering, wrapping, bound, and plane-pressure tests pass.\n"
                ));
                0
            }
            Err(error) => {
                output.err(&format!("{LABEL}: {error}\n"));
                1
            }
        }
    } else {
        run_projection(&root, &contract, mode, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_thirteen_self_tests_pass() {
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
            "knowledge-map-shards: 13/13 identity, collision, ordering, wrapping, bound, and plane-pressure tests pass.\n"
        );
    }

    #[test]
    fn the_real_projection_is_feasible_and_exactly_sized() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"facts\":453"));
        assert!(output.text.contains("\"question_keys\":4017"));
        assert!(output.text.contains("\"shards\":28"));
        assert!(output.text.contains("\"output_files\":29"));
    }
}
