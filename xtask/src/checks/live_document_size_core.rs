//! Live-document containment core, native since `RUSTDOC-AND-XTASK.15.t`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "live-document-size";
const DIMENSIONS: [&str; 6] = [
    "files",
    "lines_each",
    "bytes_each",
    "lines_total",
    "bytes_total",
    "line_bytes_each",
];
const REGISTRY_BOUNDS: [&str; 2] = ["max_records", "max_bytes"];
const LIFECYCLES: [&str; 7] = [
    "bounded_snapshot",
    "rolling_ledger",
    "partitioned_canonical",
    "generated_projection",
    "archive_terminal",
    "frozen_legacy",
    "maintained_reference",
];
const STATES: [&str; 6] = [
    "normal",
    "warning_debt",
    "rollover_debt",
    "transition_debt",
    "frozen",
    "terminal",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Metrics {
    files: i128,
    lines_each: i128,
    bytes_each: i128,
    lines_total: i128,
    bytes_total: i128,
    line_bytes_each: i128,
}

impl Metrics {
    fn get(&self, dimension: &str) -> i128 {
        match dimension {
            "files" => self.files,
            "lines_each" => self.lines_each,
            "bytes_each" => self.bytes_each,
            "lines_total" => self.lines_total,
            "bytes_total" => self.bytes_total,
            "line_bytes_each" => self.line_bytes_each,
            _ => 0,
        }
    }

    fn json(&self) -> Py {
        Py::Dict(
            DIMENSIONS
                .iter()
                .map(|dimension| {
                    (
                        pyjson::py_str(dimension),
                        Py::Int(self.get(dimension).to_string()),
                    )
                })
                .collect(),
        )
    }
}

struct RegistryRead {
    meta: Py,
    records: Vec<Py>,
}

struct State {
    root: PathBuf,
    registry_rel: String,
    routes_rel: String,
    authorities_rel: String,
    errors: Vec<String>,
    warnings: Vec<String>,
    path_seen: BTreeSet<String>,
    surface_by_id: BTreeMap<String, Py>,
    matches_by_surface: BTreeMap<String, Vec<String>>,
}

fn empty_object() -> Py {
    Py::Dict(Vec::new())
}

fn field<'a>(value: Option<&'a Py>, key: &str) -> Option<&'a Py> {
    value.and_then(|value| pyjson::get(value, key))
}

fn has_field(value: Option<&Py>, key: &str) -> bool {
    match value {
        Some(Py::Dict(pairs)) => pairs
            .iter()
            .any(|(candidate, _)| *candidate == pyjson::py_str(key)),
        _ => false,
    }
}

fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

fn defined_scalar(value: Option<&Py>) -> Option<String> {
    let value = value?;
    if matches!(value, Py::None) || perl::is_reference(value) {
        None
    } else {
        perl::scalar(Some(value))
    }
}

fn numeric(value: Option<&Py>) -> f64 {
    perl::numeric(value)
}

fn digits(value: Option<&Py>) -> bool {
    defined_scalar(value).is_some_and(|text| {
        let digits = text.strip_suffix('\n').unwrap_or(&text);
        !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
    })
}

fn signed_digits(value: Option<&Py>) -> bool {
    defined_scalar(value).is_some_and(|text| {
        let text = text.strip_suffix('\n').unwrap_or(&text);
        let digits = text.strip_prefix('-').unwrap_or(text);
        !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
    })
}

fn canonical_or_empty(value: Option<&Py>) -> String {
    match value {
        None | Some(Py::None) => canonical(&empty_object()),
        Some(value) => canonical(value),
    }
}

fn canonical(value: &Py) -> String {
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
    .unwrap_or_default()
}

fn decode_json(raw: &[u8]) -> Result<Py, String> {
    let text = std::str::from_utf8(raw).map_err(|error| error.to_string())?;
    pyjson::loads_strict(text).map_err(|python_error| {
        JsonErrorParser::new(text)
            .run()
            .err()
            .map(|error| format!("{} at {} line 451.\n", error.render(text), shim_path()))
            .unwrap_or(python_error)
    })
}

#[derive(Debug)]
struct JsonError {
    message: String,
    position: usize,
}

impl JsonError {
    fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }

    fn render(&self, text: &str) -> String {
        let characters: Vec<char> = text.chars().collect();
        let before = if self.position >= characters.len() {
            "(end of string)".to_string()
        } else {
            characters[self.position..]
                .iter()
                .take(25)
                .collect::<String>()
                .replace('\\', "\\\\")
        };
        format!(
            "{}, at character offset {} (before \"{}\")",
            self.message, self.position, before
        )
    }
}

struct JsonErrorParser {
    characters: Vec<char>,
    position: usize,
}

impl JsonErrorParser {
    fn new(text: &str) -> Self {
        Self {
            characters: text.chars().collect(),
            position: 0,
        }
    }

    fn run(mut self) -> Result<(), JsonError> {
        self.value()?;
        self.whitespace();
        if self.position != self.characters.len() {
            self.position += 1;
            return Err(JsonError::new("garbage after JSON object", self.position));
        }
        Ok(())
    }

    fn current(&self) -> Option<char> {
        self.characters.get(self.position).copied()
    }

    fn whitespace(&mut self) {
        while matches!(self.current(), Some(' ' | '\t' | '\n' | '\r')) {
            self.position += 1;
        }
    }

    fn malformed<T>(&self) -> Result<T, JsonError> {
        Err(JsonError::new(
            "malformed JSON string, neither array, object, number, string or atom",
            self.position,
        ))
    }

    fn value(&mut self) -> Result<(), JsonError> {
        self.whitespace();
        match self.current() {
            None => self.malformed(),
            Some('{') => self.object(),
            Some('[') => self.array(),
            Some('"') => self.string(),
            Some('t') => self.literal("true"),
            Some('f') => self.literal("false"),
            Some('n') => self.literal("null"),
            Some('-' | '0'..='9') => self.number(),
            Some(_) => self.malformed(),
        }
    }

    fn literal(&mut self, literal: &str) -> Result<(), JsonError> {
        let start = self.position;
        let matches = literal
            .chars()
            .enumerate()
            .all(|(offset, character)| self.characters.get(start + offset) == Some(&character));
        if !matches {
            return Err(JsonError::new(format!("'{literal}' expected"), start));
        }
        self.position += literal.len();
        Ok(())
    }

    fn object(&mut self) -> Result<(), JsonError> {
        self.position += 1;
        self.whitespace();
        if self.position == self.characters.len() {
            return Err(JsonError::new(
                ", or } expected while parsing object/hash",
                self.position,
            ));
        }
        if self.current() == Some('}') {
            self.position += 1;
            return Ok(());
        }
        loop {
            if self.current() != Some('"') {
                self.position += 1;
                return Err(JsonError::new(
                    "unexpected end of string while parsing JSON string",
                    self.position,
                ));
            }
            self.string()?;
            self.whitespace();
            if self.current() != Some(':') {
                let position = if self.position == self.characters.len() {
                    self.position.saturating_sub(1)
                } else {
                    self.position
                };
                return Err(JsonError::new("':' expected", position));
            }
            self.position += 1;
            self.whitespace();
            if self.position == self.characters.len() {
                return Err(JsonError::new(
                    ", or } expected while parsing object/hash",
                    self.position,
                ));
            }
            self.value()?;
            self.whitespace();
            if self.position == self.characters.len() {
                return Err(JsonError::new(
                    ", or } expected while parsing object/hash",
                    self.position,
                ));
            }
            match self.current() {
                Some('}') => {
                    self.position += 1;
                    return Ok(());
                }
                Some(',') => {
                    self.position += 1;
                    self.whitespace();
                    if self.position == self.characters.len() {
                        return Err(JsonError::new(
                            ", or } expected while parsing object/hash",
                            self.position,
                        ));
                    }
                }
                _ => {
                    return Err(JsonError::new(
                        ", or } expected while parsing object/hash",
                        self.position,
                    ));
                }
            }
        }
    }

    fn array(&mut self) -> Result<(), JsonError> {
        self.position += 1;
        self.whitespace();
        if self.position == self.characters.len() {
            return Err(JsonError::new(
                ", or ] expected while parsing array",
                self.position,
            ));
        }
        if self.current() == Some(']') {
            self.position += 1;
            return Ok(());
        }
        loop {
            self.value()?;
            self.whitespace();
            if self.position == self.characters.len() {
                return Err(JsonError::new(
                    ", or ] expected while parsing array",
                    self.position,
                ));
            }
            match self.current() {
                Some(']') => {
                    self.position += 1;
                    return Ok(());
                }
                Some(',') => {
                    self.position += 1;
                    self.whitespace();
                    if self.position == self.characters.len() {
                        return Err(JsonError::new(
                            ", or ] expected while parsing array",
                            self.position,
                        ));
                    }
                }
                _ => {
                    return Err(JsonError::new(
                        ", or ] expected while parsing array",
                        self.position,
                    ));
                }
            }
        }
    }

    fn string(&mut self) -> Result<(), JsonError> {
        self.position += 1;
        let mut missing_low = false;
        loop {
            let Some(character) = self.current() else {
                return Err(JsonError::new(
                    "unexpected end of string while parsing JSON string",
                    self.position,
                ));
            };
            match character {
                '"' => {
                    self.position += 1;
                    if missing_low {
                        return Err(JsonError::new(
                            "missing low surrogate character in surrogate pair",
                            self.characters.len(),
                        ));
                    }
                    return Ok(());
                }
                '\\' => {
                    let backslash = self.position;
                    self.position += 1;
                    let Some(escape) = self.current() else {
                        return Err(JsonError::new(
                            "unexpected end of string while parsing JSON string",
                            self.position,
                        ));
                    };
                    if escape == 'u' {
                        self.position += 1;
                        let code = self.hex4()?;
                        if (0xd800..0xdc00).contains(&code) {
                            let mut low = self.position;
                            let valid_low = self.characters.get(low) == Some(&'\\')
                                && self.characters.get(low + 1) == Some(&'u')
                                && (low + 6 <= self.characters.len())
                                && self.characters[low + 2..low + 6]
                                    .iter()
                                    .all(|character| character.is_ascii_hexdigit())
                                && (0xdc00..0xe000).contains(
                                    &u32::from_str_radix(
                                        &self.characters[low + 2..low + 6]
                                            .iter()
                                            .collect::<String>(),
                                        16,
                                    )
                                    .unwrap_or(0),
                                );
                            if valid_low {
                                low += 6;
                                self.position = low;
                            } else {
                                missing_low = true;
                            }
                        } else if (0xdc00..0xe000).contains(&code) {
                            return Err(JsonError::new(
                                "missing high surrogate character in surrogate pair",
                                self.position,
                            ));
                        }
                    } else if matches!(escape, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') {
                        self.position += 1;
                    } else {
                        return Err(JsonError::new(
                            "illegal backslash escape sequence in string",
                            backslash,
                        ));
                    }
                }
                character if character <= '\u{1f}' => {
                    return Err(JsonError::new(
                        "invalid character encountered while parsing JSON string",
                        self.position,
                    ));
                }
                _ => self.position += 1,
            }
        }
    }

    fn hex4(&mut self) -> Result<u32, JsonError> {
        let mut code = 0;
        for _ in 0..4 {
            let Some(character) = self.current() else {
                return Err(JsonError::new(
                    "unexpected end of string while parsing JSON string",
                    self.position,
                ));
            };
            self.position += 1;
            let Some(digit) = character.to_digit(16) else {
                return Err(JsonError::new(
                    "unexpected end of string while parsing JSON string",
                    self.position,
                ));
            };
            code = (code << 4) | digit;
        }
        Ok(code)
    }

    fn number(&mut self) -> Result<(), JsonError> {
        if self.current() == Some('-') {
            self.position += 1;
            if !self
                .current()
                .is_some_and(|character| character.is_ascii_digit())
            {
                return Err(JsonError::new(
                    "malformed number (no digits after initial minus)",
                    self.position,
                ));
            }
        }
        if self.current() == Some('0') {
            self.position += 1;
            if self
                .current()
                .is_some_and(|character| character.is_ascii_digit())
            {
                return Err(JsonError::new(
                    "malformed number (leading zero must not be followed by another digit)",
                    self.position,
                ));
            }
        } else {
            while self
                .current()
                .is_some_and(|character| character.is_ascii_digit())
            {
                self.position += 1;
            }
        }
        if self.current() == Some('.') {
            self.position += 1;
            if !self
                .current()
                .is_some_and(|character| character.is_ascii_digit())
            {
                return Err(JsonError::new(
                    "malformed number (no digits after decimal point)",
                    self.position,
                ));
            }
            while self
                .current()
                .is_some_and(|character| character.is_ascii_digit())
            {
                self.position += 1;
            }
        }
        if matches!(self.current(), Some('e' | 'E')) {
            self.position += 1;
            if matches!(self.current(), Some('-' | '+')) {
                self.position += 1;
            }
            if !self
                .current()
                .is_some_and(|character| character.is_ascii_digit())
            {
                return Err(JsonError::new(
                    "malformed number (no digits after exp sign)",
                    self.position,
                ));
            }
            while self
                .current()
                .is_some_and(|character| character.is_ascii_digit())
            {
                self.position += 1;
            }
        }
        Ok(())
    }
}

fn raw_lines(raw: &[u8]) -> Vec<&[u8]> {
    let mut lines = Vec::new();
    let mut start = 0;
    for (index, byte) in raw.iter().enumerate() {
        if *byte == b'\n' {
            lines.push(&raw[start..=index]);
            start = index + 1;
        }
    }
    if start < raw.len() {
        lines.push(&raw[start..]);
    }
    lines
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn dirname(path: &str) -> String {
    match path.rfind('/') {
        None => ".".to_string(),
        Some(0) => "/".to_string(),
        Some(index) => path[..index].to_string(),
    }
}

fn safe_relative_pattern(path: &str) -> bool {
    !path.starts_with('/')
        && !path.contains('\\')
        && !path.split('/').any(|part| part == "..")
        && !path.bytes().any(|byte| byte <= 0x1f)
}

fn glob_matches(pattern: &str, path: &str) -> bool {
    let pattern = pattern.as_bytes();
    let path = path.as_bytes();
    let mut memo = BTreeMap::new();
    fn visit(
        pattern: &[u8],
        path: &[u8],
        p: usize,
        s: usize,
        memo: &mut BTreeMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(found) = memo.get(&(p, s)) {
            return *found;
        }
        let found = if p == pattern.len() {
            s == path.len()
        } else if pattern[p] == b'*' {
            if p + 1 < pattern.len() && pattern[p + 1] == b'*' {
                visit(pattern, path, p + 2, s, memo)
                    || (s < path.len() && path[s] != b'\n' && visit(pattern, path, p, s + 1, memo))
            } else {
                visit(pattern, path, p + 1, s, memo)
                    || (s < path.len() && path[s] != b'/' && visit(pattern, path, p, s + 1, memo))
            }
        } else if pattern[p] == b'?' {
            s < path.len() && path[s] != b'/' && visit(pattern, path, p + 1, s + 1, memo)
        } else {
            s < path.len() && pattern[p] == path[s] && visit(pattern, path, p + 1, s + 1, memo)
        };
        memo.insert((p, s), found);
        found
    }
    visit(pattern, path, 0, 0, &mut memo)
}

impl State {
    fn absolute(&self, relative: &str) -> PathBuf {
        self.root.join(relative.trim_start_matches('/'))
    }

    fn relative_to_root(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn problem(&mut self, message: impl Into<String>) {
        self.errors.push(message.into());
    }

    fn reject_unknown_fields(&mut self, value: Option<&Py>, label: &str, allowed: &[&str]) {
        let Some(Py::Dict(pairs)) = value else {
            return;
        };
        let mut keys: Vec<String> = pairs.iter().map(|(key, _)| pyjson::to_text(key)).collect();
        keys.sort();
        for key in keys {
            if !allowed.contains(&key.as_str()) {
                self.problem(format!("{label} has unknown field '{key}'"));
            }
        }
    }

    fn required_scalar(&mut self, record: &Py, key: &str, label: &str) -> Option<String> {
        match defined_scalar(field(Some(record), key)) {
            Some(value) if !value.is_empty() => Some(value),
            _ => {
                self.problem(format!("{label} lacks non-empty scalar '{key}'"));
                None
            }
        }
    }

    fn registry_pressure(
        &mut self,
        meta: &Py,
        label: &str,
        measured: &[(&str, i128)],
        fields: &[(&str, &str)],
    ) {
        let Some(milestones @ Py::Dict(_)) = field(Some(meta), "milestones") else {
            self.problem(format!("{label} registry record must declare milestones"));
            return;
        };
        self.reject_unknown_fields(
            Some(milestones),
            &format!("{label} registry milestones"),
            &["warning_pct", "rollover_pct"],
        );
        let warning = field(Some(milestones), "warning_pct");
        let rollover = field(Some(milestones), "rollover_pct");
        if !digits(warning)
            || !digits(rollover)
            || numeric(warning) >= numeric(rollover)
            || numeric(rollover) >= 100.0
        {
            self.problem(format!(
                "{label} registry record has invalid warning/rollover milestones"
            ));
            return;
        }
        let mut dimensions: Vec<(&str, i128, &str)> = measured
            .iter()
            .filter_map(|(dimension, actual)| {
                fields
                    .iter()
                    .find(|(candidate, _)| candidate == dimension)
                    .map(|(_, field)| (*dimension, *actual, *field))
            })
            .collect();
        dimensions.sort_by(|left, right| left.0.cmp(right.0));
        for (dimension, actual, field_name) in dimensions {
            let bound = field(Some(meta), field_name);
            if !digits(bound) || numeric(bound) == 0.0 {
                continue;
            }
            let percent = 100.0 * actual as f64 / numeric(bound);
            if percent < numeric(warning) {
                continue;
            }
            let band = if percent >= numeric(rollover) {
                "rollover"
            } else {
                "warning"
            };
            let bound = numeric(bound) as i128;
            self.warnings.push(format!(
                "{label} {dimension} is at or above {band} ({percent:.1}%) — {} below its {bound} {field_name}",
                bound - actual
            ));
        }
    }

    fn read_jsonl_registry(
        &mut self,
        path: &Path,
        relative: &str,
        label: &str,
        hard_records: i128,
        hard_bytes: i128,
        hard_record_bytes: i128,
    ) -> RegistryRead {
        if !path.is_file() {
            self.problem(format!(
                "{label} is missing: {}",
                self.relative_to_root(path)
            ));
            return RegistryRead {
                meta: empty_object(),
                records: Vec::new(),
            };
        }
        let file_bytes = fs::metadata(path)
            .map(|meta| meta.len() as i128)
            .unwrap_or(0);
        if file_bytes > hard_bytes {
            self.problem(format!(
                "{label} exceeds portable hard byte cap {hard_bytes}"
            ));
        }
        let raw = match fs::read(path) {
            Ok(raw) => raw,
            Err(error) => {
                self.problem(format!("cannot read {label}: {}", os_error(&error)));
                return RegistryRead {
                    meta: empty_object(),
                    records: Vec::new(),
                };
            }
        };
        let mut records = Vec::new();
        let mut record_lengths = Vec::new();
        for (index, physical) in raw_lines(&raw).iter().enumerate() {
            let line_number = index + 1;
            let mut line = *physical;
            if line.ends_with(b"\n") {
                line = &line[..line.len() - 1];
            }
            if line.ends_with(b"\r") {
                line = &line[..line.len() - 1];
            }
            if line.is_empty() {
                continue;
            }
            if line.len() as i128 > hard_record_bytes {
                self.problem(format!(
                    "{label} line {line_number} exceeds portable raw-record cap {hard_record_bytes}"
                ));
            }
            match decode_json(line) {
                Ok(record @ Py::Dict(_)) => {
                    records.push(record);
                    record_lengths.push(line.len() as i128);
                }
                Ok(_) => self.problem(format!(
                    "{label} line {line_number} is not one JSON object: "
                )),
                Err(error) => self.problem(format!(
                    "{label} line {line_number} is not one JSON object: {error}"
                )),
            }
        }
        if records.is_empty() {
            self.problem(format!("{label} is empty"));
            return RegistryRead {
                meta: empty_object(),
                records,
            };
        }
        let meta = records.remove(0);
        if scalar(field(Some(&meta), "record_type")) != "registry" {
            self.problem(format!(
                "{label} first record must have record_type=registry"
            ));
        }
        self.reject_unknown_fields(
            Some(&meta),
            &format!("{label} registry record"),
            &[
                "record_type",
                "schema_version",
                "max_records",
                "max_bytes",
                "max_record_bytes",
                "max_array_items",
                "max_scalar_bytes",
                "milestones",
            ],
        );
        for key in [
            "schema_version",
            "max_records",
            "max_bytes",
            "max_record_bytes",
            "max_array_items",
            "max_scalar_bytes",
        ] {
            if !digits(field(Some(&meta), key)) {
                self.problem(format!("{label} registry record lacks numeric '{key}'"));
            }
        }
        if field(Some(&meta), "schema_version").is_some_and(|value| !matches!(value, Py::None))
            && numeric(field(Some(&meta), "schema_version")) != 1.0
        {
            self.problem(format!("{label} schema_version must be 1"));
        }
        for key in [
            "max_records",
            "max_bytes",
            "max_record_bytes",
            "max_array_items",
            "max_scalar_bytes",
        ] {
            if field(Some(&meta), key).is_some_and(|value| !matches!(value, Py::None))
                && numeric(field(Some(&meta), key)) == 0.0
            {
                self.problem(format!("{label} registry '{key}' must be positive"));
            }
        }
        if field(Some(&meta), "max_records").is_some_and(|value| !matches!(value, Py::None)) {
            if numeric(field(Some(&meta), "max_records")) > hard_records as f64 {
                self.problem(format!(
                    "{label} declares max_records above portable hard cap"
                ));
            }
            if records.len() as f64 > numeric(field(Some(&meta), "max_records")) {
                self.problem(format!(
                    "{label} has more records than its declared max_records"
                ));
            }
        }
        if field(Some(&meta), "max_bytes").is_some_and(|value| !matches!(value, Py::None)) {
            if numeric(field(Some(&meta), "max_bytes")) > hard_bytes as f64 {
                self.problem(format!(
                    "{label} declares max_bytes above portable hard cap"
                ));
            }
            if file_bytes as f64 > numeric(field(Some(&meta), "max_bytes")) {
                self.problem(format!("{label} exceeds its declared max_bytes"));
            }
        }
        self.registry_pressure(
            &meta,
            label,
            &[("records", records.len() as i128), ("bytes", file_bytes)],
            &[("records", "max_records"), ("bytes", "max_bytes")],
        );
        if field(Some(&meta), "max_record_bytes").is_some_and(|value| !matches!(value, Py::None)) {
            if numeric(field(Some(&meta), "max_record_bytes")) > hard_record_bytes as f64 {
                self.problem(format!(
                    "{label} declares max_record_bytes above portable hard cap"
                ));
            }
            for length in record_lengths {
                if length as f64 > numeric(field(Some(&meta), "max_record_bytes")) {
                    self.problem(format!(
                        "{label} contains a record above its declared max_record_bytes"
                    ));
                }
            }
        }
        if field(Some(&meta), "max_array_items").is_some_and(|value| !matches!(value, Py::None))
            && numeric(field(Some(&meta), "max_array_items")) > 64.0
        {
            self.problem(format!(
                "{label} declares max_array_items above portable hard cap"
            ));
        }
        if field(Some(&meta), "max_scalar_bytes").is_some_and(|value| !matches!(value, Py::None))
            && numeric(field(Some(&meta), "max_scalar_bytes")) > 1_024.0
        {
            self.problem(format!(
                "{label} declares max_scalar_bytes above portable hard cap"
            ));
        }
        if field(Some(&meta), "max_array_items").is_some_and(|value| !matches!(value, Py::None))
            && field(Some(&meta), "max_scalar_bytes")
                .is_some_and(|value| !matches!(value, Py::None))
        {
            let max_array_items = numeric(field(Some(&meta), "max_array_items"));
            let max_scalar_bytes = numeric(field(Some(&meta), "max_scalar_bytes"));
            for record in &records {
                self.validate_value_bounds(
                    record,
                    &format!("{label} data record"),
                    max_array_items,
                    max_scalar_bytes,
                );
            }
        }
        let _ = relative;
        RegistryRead { meta, records }
    }

    fn validate_value_bounds(
        &mut self,
        value: &Py,
        label: &str,
        max_array_items: f64,
        max_scalar_bytes: f64,
    ) {
        match value {
            Py::Dict(pairs) => {
                let mut entries: Vec<(String, &Py)> = pairs
                    .iter()
                    .map(|(key, value)| (pyjson::to_text(key), value))
                    .collect();
                entries.sort_by(|left, right| left.0.cmp(&right.0));
                for (key, value) in entries {
                    self.validate_value_bounds(
                        value,
                        &format!("{label}.{key}"),
                        max_array_items,
                        max_scalar_bytes,
                    );
                }
            }
            Py::List(items) => {
                if items.len() as f64 > max_array_items {
                    self.problem(format!(
                        "{label} has more than {} array items",
                        perl::number(max_array_items)
                    ));
                }
                for (index, item) in items.iter().enumerate() {
                    self.validate_value_bounds(
                        item,
                        &format!("{label}[{index}]"),
                        max_array_items,
                        max_scalar_bytes,
                    );
                }
            }
            Py::Bool(_) => self.problem(format!("{label} has an unsupported value type")),
            Py::None => {}
            _ => {
                if scalar(Some(value)).len() as f64 > max_scalar_bytes {
                    self.problem(format!(
                        "{label} exceeds the declared scalar byte limit {}",
                        perl::number(max_scalar_bytes)
                    ));
                }
            }
        }
    }

    fn containment_registry_paths(&mut self, directory: &str) -> Vec<String> {
        let absolute = self.absolute(directory);
        let entries = match fs::read_dir(&absolute) {
            Ok(entries) => entries,
            Err(error) => {
                self.problem(format!(
                    "cannot list containment registry directory '{directory}': {}",
                    os_error(&error)
                ));
                return Vec::new();
            }
        };
        let mut names = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if (name.ends_with(".json") || name.ends_with(".jsonl")) && entry.path().is_file() {
                names.push(name);
            }
        }
        names.sort();
        names
            .into_iter()
            .map(|name| {
                if directory == "." {
                    name
                } else {
                    format!("{directory}/{name}")
                }
            })
            .collect()
    }

    fn validate_numeric_evidence_encoding(&mut self, directory: &str) {
        let mut number_first_seen: BTreeMap<String, String> = BTreeMap::new();
        let mut string_fields: Vec<(String, String, String)> = Vec::new();
        for relative in self.containment_registry_paths(directory) {
            let raw = match fs::read(self.absolute(&relative)) {
                Ok(raw) => raw,
                Err(error) => {
                    self.problem(format!(
                        "cannot read containment registry '{relative}': {}",
                        os_error(&error)
                    ));
                    continue;
                }
            };
            let mut documents = Vec::new();
            if relative.ends_with(".jsonl") {
                for (index, line) in raw.split(|byte| *byte == b'\n').enumerate() {
                    if line.iter().any(|byte| !byte.is_ascii_whitespace()) {
                        documents.push((format!("{relative} line {}", index + 1), line));
                    }
                }
            } else {
                documents.push((relative.clone(), raw.as_slice()));
            }
            for (where_, text) in documents {
                match decode_json(text) {
                    Ok(Py::None) | Err(_) => self.problem(format!(
                        "{where_} is not valid JSON, so its numeric evidence cannot be type-checked"
                    )),
                    Ok(decoded) => collect_scalar_types(
                        &decoded,
                        None,
                        &where_,
                        &mut number_first_seen,
                        &mut string_fields,
                    ),
                }
            }
        }
        for (key, where_, value) in string_fields {
            if numeric_evidence_string(&value) {
                self.problem(format!(
                    "{where_}: field '{key}' holds the string \"{value}\"; numeric evidence must be a JSON number"
                ));
            } else if let Some(first) = number_first_seen.get(&key) {
                self.problem(format!(
                    "{where_}: field '{key}' holds the string \"{value}\", but it is a JSON number at {first}; a numeric field must be a number everywhere"
                ));
            }
        }
    }

    fn validate_surface_schema(&mut self, surface: &Py) {
        let id = defined_scalar(field(Some(surface), "surface_id"))
            .unwrap_or_else(|| "<unknown>".to_string());
        self.reject_unknown_fields(
            Some(surface),
            &format!("surface '{id}'"),
            &[
                "surface_id",
                "targets",
                "locator",
                "lifecycle",
                "state",
                "owner",
                "health_targets",
                "enforcement_ceilings",
                "milestones",
                "verifier",
                "baseline",
                "transition",
                "currency",
                "index",
                "index_contract",
                "freshness_verifier",
                "canonical_inputs",
                "sha256",
                "reference_contract",
                "archive_manifest",
                "aggregate_composition",
                "cardinality_exemption",
                "sealed_members",
            ],
        );
        self.validate_aggregate_reachability(surface, &id);
        self.reject_unknown_fields(
            field(Some(surface), "health_targets"),
            &format!("surface '{id}' health_targets"),
            &DIMENSIONS,
        );
        self.reject_unknown_fields(
            field(Some(surface), "enforcement_ceilings"),
            &format!("surface '{id}' enforcement_ceilings"),
            &DIMENSIONS,
        );
        if has_field(Some(surface), "baseline") {
            self.reject_unknown_fields(
                field(Some(surface), "baseline"),
                &format!("surface '{id}' baseline"),
                &DIMENSIONS,
            );
        }
        self.reject_unknown_fields(
            field(Some(surface), "milestones"),
            &format!("surface '{id}' milestones"),
            &["warning_pct", "rollover_pct"],
        );
        if has_field(Some(surface), "transition") {
            let transition = field(Some(surface), "transition");
            self.reject_unknown_fields(
                transition,
                &format!("surface '{id}' transition"),
                &["owner", "max_growth"],
            );
            if matches!(transition, Some(Py::Dict(_))) {
                self.reject_unknown_fields(
                    field(transition, "max_growth"),
                    &format!("surface '{id}' transition.max_growth"),
                    &DIMENSIONS,
                );
            }
        }
        if has_field(Some(surface), "currency") {
            self.reject_unknown_fields(
                field(Some(surface), "currency"),
                &format!("surface '{id}' currency"),
                &["status", "owner", "verifier"],
            );
        }
        if has_field(Some(surface), "index_contract") {
            self.reject_unknown_fields(
                field(Some(surface), "index_contract"),
                &format!("surface '{id}' index_contract"),
                &["kind", "verifier", "route_surface"],
            );
        }
        if has_field(Some(surface), "cardinality_exemption") {
            self.reject_unknown_fields(
                field(Some(surface), "cardinality_exemption"),
                &format!("surface '{id}' cardinality_exemption"),
                &["authority", "work_unit", "route_surface_id", "rationale"],
            );
        }
        if let Some(Py::List(entries)) = field(Some(surface), "sealed_members") {
            for entry in entries {
                if matches!(entry, Py::Dict(_)) {
                    self.reject_unknown_fields(
                        Some(entry),
                        &format!("surface '{id}' sealed member"),
                        &["path", "sha256", "owner"],
                    );
                }
            }
        }
        if has_field(Some(surface), "reference_contract") {
            let contract = field(Some(surface), "reference_contract");
            self.reject_unknown_fields(
                contract,
                &format!("surface '{id}' reference_contract"),
                &["mandatory_read", "max_navigation_depth", "aggregate_change"],
            );
            if matches!(contract, Some(Py::Dict(_))) {
                self.reject_unknown_fields(
                    field(contract, "mandatory_read"),
                    &format!("surface '{id}' mandatory_read"),
                    &["path", "lines_ceiling", "bytes_ceiling"],
                );
                let aggregate = field(contract, "aggregate_change");
                self.reject_unknown_fields(
                    aggregate,
                    &format!("surface '{id}' aggregate_change"),
                    &["authority_id", "owner", "baseline", "delta", "rationale"],
                );
                if matches!(aggregate, Some(Py::Dict(_))) {
                    self.reject_unknown_fields(
                        field(aggregate, "baseline"),
                        &format!("surface '{id}' aggregate baseline"),
                        &["files", "lines_total", "bytes_total"],
                    );
                    self.reject_unknown_fields(
                        field(aggregate, "delta"),
                        &format!("surface '{id}' aggregate delta"),
                        &["files", "lines_total", "bytes_total"],
                    );
                }
            }
        }
    }

    fn validate_aggregate_reachability(&mut self, surface: &Py, id: &str) {
        if scalar(field(Some(surface), "locator")) != "collection" {
            return;
        }
        let composition = field(Some(surface), "aggregate_composition");
        let members = self.validate_aggregate_composition_schema(composition, id);
        for band in ["health_targets", "enforcement_ceilings"] {
            let limits = field(Some(surface), band);
            let Some(Py::Dict(_)) = limits else {
                continue;
            };
            let values = [
                field(limits, "files"),
                field(limits, "lines_each"),
                field(limits, "bytes_each"),
                field(limits, "lines_total"),
                field(limits, "bytes_total"),
            ];
            if values.iter().any(|value| !digits(*value)) {
                continue;
            }
            let files = scalar(values[0]);
            let per_line = scalar(values[1]);
            let per_byte = scalar(values[2]);
            let total_line = scalar(values[3]);
            let total_byte = scalar(values[4]);
            let Some(members) = members else {
                if numeric(values[3]) < numeric(values[0]) * numeric(values[1]) {
                    self.problem(format!(
                        "surface '{id}' {band} lines_total {total_line} is below its own legal maximum {files} x {per_line}; raise the total or declare an aggregate_composition"
                    ));
                }
                if numeric(values[4]) < numeric(values[0]) * numeric(values[2]) {
                    self.problem(format!(
                        "surface '{id}' {band} bytes_total {total_byte} is below its own legal maximum {files} x {per_byte}; raise the total or declare an aggregate_composition"
                    ));
                }
                continue;
            };
            let band_key = if band == "health_targets" {
                "health"
            } else {
                "ceiling"
            };
            let mut count: f64 = 0.0;
            let mut lines: f64 = 0.0;
            let mut bytes: f64 = 0.0;
            let mut max_line: f64 = 0.0;
            let mut max_byte: f64 = 0.0;
            for member in members {
                let bounds = field(Some(member), band_key);
                let Some(Py::Dict(_)) = bounds else {
                    continue;
                };
                let count_value = field(Some(member), "count");
                let member_count = if matches!(count_value, Some(Py::Dict(_))) {
                    field(count_value, band_key)
                } else {
                    count_value
                };
                if member_count.is_none_or(|value| matches!(value, Py::None)) {
                    continue;
                }
                count += numeric(member_count);
                lines += numeric(member_count) * numeric(field(bounds, "lines"));
                bytes += numeric(member_count) * numeric(field(bounds, "bytes"));
                max_line = max_line.max(numeric(field(bounds, "lines")));
                max_byte = max_byte.max(numeric(field(bounds, "bytes")));
            }
            if count != numeric(values[0]) {
                self.problem(format!(
                    "surface '{id}' aggregate_composition counts sum to {}, not the {band} files bound {files}",
                    perl::number(count)
                ));
            }
            if lines != numeric(values[3]) {
                self.problem(format!(
                    "surface '{id}' aggregate_composition {band_key} lines sum to {}, not lines_total {total_line}",
                    perl::number(lines)
                ));
            }
            if bytes != numeric(values[4]) {
                self.problem(format!(
                    "surface '{id}' aggregate_composition {band_key} bytes sum to {}, not bytes_total {total_byte}",
                    perl::number(bytes)
                ));
            }
            if max_line != numeric(values[1]) {
                self.problem(format!(
                    "surface '{id}' aggregate_composition largest {band_key} member is {} lines, not lines_each {per_line}",
                    perl::number(max_line)
                ));
            }
            if max_byte != numeric(values[2]) {
                self.problem(format!(
                    "surface '{id}' aggregate_composition largest {band_key} member is {} bytes, not bytes_each {per_byte}",
                    perl::number(max_byte)
                ));
            }
        }
    }

    fn validate_aggregate_composition_schema<'a>(
        &mut self,
        composition: Option<&'a Py>,
        id: &str,
    ) -> Option<&'a [Py]> {
        let composition = composition?;
        if matches!(composition, Py::None) {
            return None;
        }
        if !matches!(composition, Py::Dict(_)) {
            self.problem(format!(
                "surface '{id}' aggregate_composition must be an object"
            ));
            return None;
        }
        self.reject_unknown_fields(
            Some(composition),
            &format!("surface '{id}' aggregate_composition"),
            &["rationale", "members"],
        );
        let rationale = field(Some(composition), "rationale");
        if defined_scalar(rationale).is_none_or(|value| value.is_empty()) {
            self.problem(format!(
                "surface '{id}' aggregate_composition lacks a nonempty rationale"
            ));
        }
        let members = field(Some(composition), "members");
        let Some(Py::List(members)) = members else {
            self.problem(format!(
                "surface '{id}' aggregate_composition members must list at least two member roles"
            ));
            return None;
        };
        if members.len() < 2 {
            self.problem(format!(
                "surface '{id}' aggregate_composition members must list at least two member roles"
            ));
            return None;
        }
        let mut valid = true;
        let mut seen_role = BTreeSet::new();
        for member in members {
            if !matches!(member, Py::Dict(_)) {
                self.problem(format!(
                    "surface '{id}' aggregate_composition member must be an object"
                ));
                valid = false;
                continue;
            }
            self.reject_unknown_fields(
                Some(member),
                &format!("surface '{id}' aggregate_composition member"),
                &["role", "count", "health", "ceiling"],
            );
            let role = defined_scalar(field(Some(member), "role"));
            match role.as_deref() {
                None | Some("") => {
                    self.problem(format!(
                        "surface '{id}' aggregate_composition member lacks a role"
                    ));
                    valid = false;
                }
                Some(role) => {
                    if !seen_role.insert(role.to_string()) {
                        self.problem(format!(
                            "surface '{id}' aggregate_composition repeats member role '{role}'"
                        ));
                        valid = false;
                    }
                }
            }
            let role = role.unwrap_or_else(|| "<unknown>".to_string());
            let count = field(Some(member), "count");
            if matches!(count, Some(Py::Dict(_))) {
                self.reject_unknown_fields(
                    count,
                    &format!("surface '{id}' aggregate_composition member '{role}' count"),
                    &["health", "ceiling"],
                );
                for band in ["health", "ceiling"] {
                    let value = field(count, band);
                    if !digits(value) || numeric(value) < 1.0 {
                        self.problem(format!(
                            "surface '{id}' aggregate_composition member '{role}' count lacks a positive '{band}'"
                        ));
                        valid = false;
                    }
                }
            } else if !digits(count) || numeric(count) < 1.0 {
                self.problem(format!(
                    "surface '{id}' aggregate_composition member '{role}' lacks a positive count"
                ));
                valid = false;
            }
            for band in ["health", "ceiling"] {
                let bounds = field(Some(member), band);
                if !matches!(bounds, Some(Py::Dict(_))) {
                    self.problem(format!(
                        "surface '{id}' aggregate_composition member '{role}' lacks {band} bounds"
                    ));
                    valid = false;
                    continue;
                }
                self.reject_unknown_fields(
                    bounds,
                    &format!("surface '{id}' aggregate_composition member '{role}' {band}"),
                    &["lines", "bytes"],
                );
                for axis in ["lines", "bytes"] {
                    let value = field(bounds, axis);
                    if !digits(value) || numeric(value) < 1.0 {
                        self.problem(format!(
                            "surface '{id}' aggregate_composition member '{role}' {band} lacks a positive '{axis}'"
                        ));
                        valid = false;
                    }
                }
            }
        }
        valid.then_some(members.as_slice())
    }

    fn numeric_dimensions(
        &mut self,
        object: Option<&Py>,
        label: &str,
        allow_null: bool,
        vocabulary: &[&str],
    ) {
        if !matches!(object, Some(Py::Dict(_))) {
            self.problem(format!("{label} must be an object"));
            return;
        }
        for dimension in vocabulary {
            if !has_field(object, dimension) {
                self.problem(format!("{label} lacks '{dimension}'"));
            } else if matches!(field(object, dimension), Some(Py::None)) {
                if !allow_null {
                    self.problem(format!("{label} may not set '{dimension}' to null"));
                }
            } else if !digits(field(object, dimension)) {
                self.problem(format!(
                    "{label} '{dimension}' must be a non-negative integer or allowed null"
                ));
            }
        }
    }

    fn validate_authority_schema(&mut self, authority: &Py) {
        let (kind, key) = authority_key(authority);
        let id = key.clone().unwrap_or_else(|| "<unknown>".to_string());
        self.reject_unknown_fields(
            Some(authority),
            &format!("ceiling authority '{id}'"),
            &[
                "record_type",
                "surface_id",
                "registry_id",
                "work_unit",
                "owner",
                "rationale",
                "old",
                "new",
            ],
        );
        if scalar(field(Some(authority), "record_type")) != "increase" {
            self.problem(format!(
                "ceiling authority '{id}' must have record_type=increase"
            ));
        }
        if field(Some(authority), "surface_id").is_some_and(|value| !matches!(value, Py::None))
            && field(Some(authority), "registry_id").is_some_and(|value| !matches!(value, Py::None))
        {
            self.problem(format!(
                "ceiling authority '{id}' must name exactly one of surface_id or registry_id"
            ));
        }
        if key.is_none() {
            self.problem("ceiling authority '<unknown>' names no surface_id or registry_id");
        }
        for key in ["work_unit", "owner", "rationale"] {
            self.required_scalar(authority, key, &format!("ceiling authority '{id}'"));
        }
        if kind == "registry_id" {
            let mut governable = BTreeSet::new();
            governable.insert(self.registry_rel.clone());
            governable.extend(self.bounded_registry_paths());
            if !key.as_ref().is_some_and(|key| governable.contains(key)) {
                self.problem(format!(
                    "ceiling authority '{id}' does not name a bounded registry this checker reads"
                ));
            }
            self.reject_unknown_fields(
                field(Some(authority), "old"),
                &format!("ceiling authority '{id}' old"),
                &REGISTRY_BOUNDS,
            );
            self.reject_unknown_fields(
                field(Some(authority), "new"),
                &format!("ceiling authority '{id}' new"),
                &REGISTRY_BOUNDS,
            );
            self.numeric_dimensions(
                field(Some(authority), "old"),
                &format!("ceiling authority '{id}' old"),
                false,
                &REGISTRY_BOUNDS,
            );
            self.numeric_dimensions(
                field(Some(authority), "new"),
                &format!("ceiling authority '{id}' new"),
                false,
                &REGISTRY_BOUNDS,
            );
            return;
        }
        self.reject_unknown_fields(
            field(Some(authority), "old"),
            &format!("ceiling authority '{id}' old"),
            &DIMENSIONS,
        );
        self.reject_unknown_fields(
            field(Some(authority), "new"),
            &format!("ceiling authority '{id}' new"),
            &DIMENSIONS,
        );
        self.numeric_dimensions(
            field(Some(authority), "old"),
            &format!("ceiling authority '{id}' old"),
            false,
            &DIMENSIONS,
        );
        self.numeric_dimensions(
            field(Some(authority), "new"),
            &format!("ceiling authority '{id}' new"),
            true,
            &DIMENSIONS,
        );
    }

    fn git_top(&self) -> String {
        if !self.root.join(".git").exists() {
            return String::new();
        }
        let Ok(process) = Command::new("git")
            .args(["-C"])
            .arg(&self.root)
            .args(["rev-parse", "--show-toplevel"])
            .output()
        else {
            return String::new();
        };
        String::from_utf8_lossy(&process.stdout)
            .lines()
            .next()
            .unwrap_or_default()
            .trim_end_matches('\r')
            .to_string()
    }

    fn markdown_paths(&self, output: &mut dyn super::Output) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        if self.git_top() == self.root.to_string_lossy() {
            let process = Command::new("git")
                .arg("-C")
                .arg(&self.root)
                .args(["ls-files", "-z", "--cached", "--", "*.md"])
                .output()
                .map_err(|_| {
                    "live-document-size: cannot enumerate Git Markdown paths".to_string()
                })?;
            result.extend(
                process
                    .stdout
                    .split(|byte| *byte == 0)
                    .filter(|path| !path.is_empty())
                    .map(|path| String::from_utf8_lossy(path).into_owned()),
            );
        } else {
            fn walk(
                root: &Path,
                directory: &Path,
                result: &mut Vec<String>,
                output: &mut dyn super::Output,
            ) {
                let entries = match fs::read_dir(directory) {
                    Ok(entries) => entries,
                    Err(error) => {
                        output.err(&format!(
                            "Can't stat {}: {}\n at {} line 836.\n",
                            directory.display(),
                            os_error(&error),
                            shim_path()
                        ));
                        return;
                    }
                };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                        walk(root, &path, result, output);
                    } else if !path.is_dir()
                        && path
                            .file_name()
                            .is_some_and(|name| name.to_string_lossy().ends_with(".md"))
                    {
                        let relative = path
                            .strip_prefix(root)
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .replace('\\', "/");
                        if relative != ".git"
                            && !relative.starts_with(".git/")
                            && relative != "generated"
                            && !relative.starts_with("generated/")
                        {
                            result.push(relative);
                        }
                    }
                }
            }
            walk(&self.root, &self.root, &mut result, output);
        }
        result.sort();
        result.dedup();
        Ok(result)
    }

    fn bounded_registry_paths(&self) -> Vec<String> {
        if self.git_top() != self.root.to_string_lossy() {
            return Vec::new();
        }
        let Ok(process) = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .args([
                "ls-files",
                "-z",
                "--cached",
                "--",
                "doctrine/*.jsonl",
                "doctrine/**/*.jsonl",
            ])
            .output()
        else {
            return Vec::new();
        };
        let mut result: Vec<String> = process
            .stdout
            .split(|byte| *byte == 0)
            .filter(|path| !path.is_empty())
            .map(|path| String::from_utf8_lossy(path).into_owned())
            .collect();
        result.sort();
        result.dedup();
        result
    }

    fn registry_text_at_head(&self, relative: &str) -> Option<String> {
        if self.git_top() != self.root.to_string_lossy() {
            return None;
        }
        let tracked = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .args(["ls-tree", "-r", "--name-only", "HEAD", "--", relative])
            .output()
            .ok()?;
        if tracked.stdout.is_empty() {
            return None;
        }
        let text = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .args(["show", &format!("HEAD:{relative}")])
            .output()
            .ok()?;
        if text.stdout.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(&text.stdout).into_owned())
        }
    }

    fn measure_paths(&mut self, paths: &[String]) -> Metrics {
        let mut metrics = Metrics {
            files: paths.len() as i128,
            ..Metrics::default()
        };
        for path in paths {
            let absolute = self.absolute(path);
            if !absolute.is_file() {
                self.problem(format!(
                    "classified Markdown '{path}' is missing from the resulting tree"
                ));
                continue;
            }
            let content = match fs::read(&absolute) {
                Ok(content) => content,
                Err(error) => {
                    self.problem(format!(
                        "cannot read classified Markdown '{path}': {}",
                        os_error(&error)
                    ));
                    continue;
                }
            };
            let bytes = content.len() as i128;
            let lines = content.iter().filter(|byte| **byte == b'\n').count() as i128;
            let max_line = content
                .split(|byte| *byte == b'\n')
                .map(|line| line.strip_suffix(b"\r").unwrap_or(line).len())
                .max()
                .unwrap_or(0) as i128;
            metrics.bytes_total += bytes;
            metrics.lines_total += lines;
            metrics.bytes_each = metrics.bytes_each.max(bytes);
            metrics.lines_each = metrics.lines_each.max(lines);
            metrics.line_bytes_each = metrics.line_bytes_each.max(max_line);
        }
        metrics
    }

    fn validate_cardinality_exemption(&mut self, surface: &Py, id: &str) {
        let exemption = field(Some(surface), "cardinality_exemption");
        let health = field(Some(surface), "health_targets");
        let ceilings = field(Some(surface), "enforcement_ceilings");
        if !matches!(health, Some(Py::Dict(_))) || !matches!(ceilings, Some(Py::Dict(_))) {
            return;
        }
        let health_null =
            has_field(health, "files") && matches!(field(health, "files"), Some(Py::None));
        let ceiling_null =
            has_field(ceilings, "files") && matches!(field(ceilings, "files"), Some(Py::None));
        let lifecycle_allows = scalar(field(Some(surface), "lifecycle")) == "maintained_reference";
        if exemption.is_none_or(|value| matches!(value, Py::None)) {
            if (health_null || ceiling_null) && !lifecycle_allows {
                self.problem(format!(
                    "surface '{id}' nulls files without a declared cardinality exemption"
                ));
            }
            return;
        }
        if !matches!(exemption, Some(Py::Dict(_))) {
            self.problem(format!(
                "surface '{id}' cardinality_exemption must be an object"
            ));
            return;
        }
        if health_null != ceiling_null {
            self.problem(format!(
                "surface '{id}' cardinality exemption must null files in both bands together"
            ));
        }
        if !health_null && !ceiling_null {
            self.problem(format!(
                "surface '{id}' declares a cardinality exemption while files stays bounded"
            ));
        }
        for dimension in DIMENSIONS.iter().filter(|dimension| **dimension != "files") {
            if field(health, dimension).is_none_or(|value| matches!(value, Py::None))
                || field(ceilings, dimension).is_none_or(|value| matches!(value, Py::None))
            {
                self.problem(format!(
                    "surface '{id}' cardinality exemption may not unbound resource dimension '{dimension}'"
                ));
            }
        }
        for key in ["authority", "work_unit", "route_surface_id", "rationale"] {
            if defined_scalar(field(exemption, key)).is_none_or(|value| value.is_empty()) {
                self.problem(format!(
                    "surface '{id}' cardinality exemption lacks '{key}'"
                ));
            }
        }
        let authority = field(exemption, "authority");
        if let Some(authority) = defined_scalar(authority)
            && (!safe_relative_pattern(&authority) || !self.absolute(&authority).is_file())
        {
            self.problem(format!(
                "surface '{id}' cardinality exemption authority '{authority}' is not a tracked repository file"
            ));
        }
        let Some(route_id) = defined_scalar(field(exemption, "route_surface_id")) else {
            return;
        };
        if route_id.is_empty() {
            return;
        }
        if route_id == id {
            self.problem(format!(
                "surface '{id}' cardinality exemption route must be a different registered surface"
            ));
            return;
        }
        let route = self.surface_by_id.get(&route_id).cloned();
        let Some(route) = route else {
            self.problem(format!(
                "surface '{id}' cardinality exemption route surface '{route_id}' is not registered"
            ));
            return;
        };
        let index = field(Some(surface), "index");
        if defined_scalar(index).is_none_or(|value| value.is_empty()) {
            self.problem(format!(
                "surface '{id}' cardinality exemption requires a declared index for its route to cover"
            ));
        } else {
            let index = scalar(index);
            let covered = self
                .matches_by_surface
                .get(&route_id)
                .is_some_and(|paths| paths.contains(&index));
            if !covered {
                self.problem(format!(
                    "surface '{id}' cardinality exemption route '{route_id}' does not cover its declared index"
                ));
            }
        }
        let route_ceilings = field(Some(&route), "enforcement_ceilings");
        if !matches!(route_ceilings, Some(Py::Dict(_))) {
            self.problem(format!(
                "surface '{id}' cardinality exemption route '{route_id}' declares no enforcement ceilings"
            ));
            return;
        }
        for dimension in DIMENSIONS {
            if field(route_ceilings, dimension).is_none_or(|value| matches!(value, Py::None)) {
                self.problem(format!(
                    "surface '{id}' cardinality exemption route '{route_id}' is unbounded in '{dimension}'"
                ));
            }
        }
    }

    fn validate_sealed_members(
        &mut self,
        surface: &Py,
        paths: &[String],
        id: &str,
    ) -> Option<Metrics> {
        if !has_field(Some(surface), "sealed_members") {
            return None;
        }
        let sealed = field(Some(surface), "sealed_members");
        let Some(Py::List(entries)) = sealed else {
            self.problem(format!(
                "surface '{id}' sealed_members must be a non-empty array"
            ));
            return None;
        };
        if entries.is_empty() {
            self.problem(format!(
                "surface '{id}' sealed_members must be a non-empty array"
            ));
            return None;
        }
        if scalar(field(Some(surface), "locator")) != "collection" {
            self.problem(format!(
                "surface '{id}' seals members but is not a collection"
            ));
        }
        let targets = field(Some(surface), "health_targets");
        let rollover = field(field(Some(surface), "milestones"), "rollover_pct");
        let members: BTreeSet<&str> = paths.iter().map(String::as_str).collect();
        let mut seen = BTreeSet::new();
        for entry in entries {
            if !matches!(entry, Py::Dict(_)) {
                self.problem(format!("surface '{id}' sealed member must be an object"));
                continue;
            }
            let Some(path) =
                self.required_scalar(entry, "path", &format!("surface '{id}' sealed member"))
            else {
                continue;
            };
            let expected = self.required_scalar(
                entry,
                "sha256",
                &format!("surface '{id}' sealed member '{path}'"),
            );
            self.required_scalar(
                entry,
                "owner",
                &format!("surface '{id}' sealed member '{path}'"),
            );
            if !seen.insert(path.clone()) {
                self.problem(format!("surface '{id}' seals '{path}' twice"));
            }
            if !members.contains(path.as_str()) {
                self.problem(format!(
                    "surface '{id}' sealed member '{path}' is not one of its files"
                ));
                continue;
            }
            let raw = match fs::read(self.absolute(&path)) {
                Ok(raw) => raw,
                Err(error) => {
                    self.problem(format!(
                        "cannot read sealed member '{path}': {}",
                        os_error(&error)
                    ));
                    continue;
                }
            };
            let actual = sha256_hex(&raw);
            if expected
                .as_ref()
                .is_some_and(|expected| *expected != actual)
            {
                self.problem(format!(
                    "surface '{id}' sealed member '{path}' changed; amend it with a new record"
                ));
            }
            let own = self.measure_paths(std::slice::from_ref(&path));
            let at_rollover = rollover.is_some_and(|value| !matches!(value, Py::None))
                && ["lines_each", "bytes_each", "line_bytes_each"]
                    .iter()
                    .any(|dimension| {
                        let target = field(targets, dimension);
                        target.is_some_and(|value| !matches!(value, Py::None))
                            && numeric(target) > 0.0
                            && 100.0 * own.get(dimension) as f64 / numeric(target)
                                >= numeric(rollover)
                    });
            if !at_rollover {
                self.problem(format!(
                    "surface '{id}' sealed member '{path}' has not reached its per-file rollover"
                ));
            }
        }
        let unsealed: Vec<String> = paths
            .iter()
            .filter(|path| !seen.contains(*path))
            .cloned()
            .collect();
        Some(self.measure_paths(&unsealed))
    }

    fn validate_limits(
        &mut self,
        surface: &Py,
        metrics: Metrics,
        id: &str,
        milestone_metrics: Option<Metrics>,
    ) {
        let milestone_metrics = milestone_metrics.unwrap_or(metrics);
        let allow_null = scalar(field(Some(surface), "lifecycle")) == "maintained_reference"
            || matches!(
                field(Some(surface), "cardinality_exemption"),
                Some(Py::Dict(_))
            );
        self.numeric_dimensions(
            field(Some(surface), "health_targets"),
            &format!("surface '{id}' health_targets"),
            allow_null,
            &DIMENSIONS,
        );
        self.numeric_dimensions(
            field(Some(surface), "enforcement_ceilings"),
            &format!("surface '{id}' enforcement_ceilings"),
            allow_null,
            &DIMENSIONS,
        );
        let ceilings = field(Some(surface), "enforcement_ceilings");
        if !matches!(ceilings, Some(Py::Dict(_))) {
            return;
        }
        for dimension in DIMENSIONS {
            let ceiling = field(ceilings, dimension);
            if ceiling.is_none_or(|value| matches!(value, Py::None)) {
                continue;
            }
            if metrics.get(dimension) as f64 > numeric(ceiling) {
                self.problem(format!(
                    "surface '{id}' exceeds {dimension} ceiling: {} > {}",
                    metrics.get(dimension),
                    scalar(ceiling)
                ));
            }
        }
        let targets = field(Some(surface), "health_targets");
        let milestones = field(Some(surface), "milestones");
        if matches!(targets, Some(Py::Dict(_))) && matches!(milestones, Some(Py::Dict(_))) {
            let warning = field(milestones, "warning_pct");
            let rollover = field(milestones, "rollover_pct");
            if !digits(warning)
                || !digits(rollover)
                || numeric(warning) >= numeric(rollover)
                || numeric(rollover) >= 100.0
            {
                self.problem(format!(
                    "surface '{id}' has invalid warning/rollover milestones"
                ));
            } else {
                for dimension in DIMENSIONS {
                    let target = field(targets, dimension);
                    if target.is_none_or(|value| matches!(value, Py::None))
                        || numeric(target) == 0.0
                    {
                        continue;
                    }
                    let measured = if dimension.ends_with("_each") {
                        milestone_metrics.get(dimension)
                    } else {
                        metrics.get(dimension)
                    };
                    let percent = 100.0 * measured as f64 / numeric(target);
                    if dimension == "files" && metrics.get(dimension) as f64 == numeric(target) {
                        let declared = match field(Some(surface), "targets") {
                            Some(Py::List(targets)) => targets.as_slice(),
                            _ => &[],
                        };
                        let derived = field(Some(surface), "canonical_inputs")
                            .is_some_and(|value| !matches!(value, Py::None))
                            && field(Some(surface), "freshness_verifier")
                                .is_some_and(|value| !matches!(value, Py::None));
                        let enumerated = !declared.is_empty()
                            && declared.iter().all(|target| {
                                let target = scalar(Some(target));
                                !target.contains('*') && !target.contains('?')
                            });
                        if derived || (enumerated && numeric(target) == declared.len() as f64) {
                            continue;
                        }
                    }
                    if metrics.files == 1 && matches!(dimension, "lines_total" | "bytes_total") {
                        continue;
                    }
                    if matches!(
                        scalar(field(Some(surface), "lifecycle")).as_str(),
                        "frozen_legacy" | "archive_terminal"
                    ) || scalar(field(Some(surface), "state")) == "transition_debt"
                    {
                        continue;
                    }
                    let mut headroom = String::new();
                    let ceiling = field(ceilings, dimension);
                    if ceiling.is_some_and(|value| !matches!(value, Py::None))
                        && numeric(ceiling) >= measured as f64
                    {
                        headroom = format!(
                            " — {} below its {} ceiling",
                            numeric(ceiling) as i128 - measured,
                            scalar(ceiling)
                        );
                    }
                    if percent >= numeric(rollover) {
                        self.warnings.push(format!(
                            "surface '{id}' {dimension} is at or above rollover ({percent:.1}%){headroom}"
                        ));
                    } else if percent >= numeric(warning) {
                        self.warnings.push(format!(
                            "surface '{id}' {dimension} is at or above warning ({percent:.1}%){headroom}"
                        ));
                    }
                }
            }
        } else {
            self.problem(format!("surface '{id}' must declare milestones"));
        }
        if scalar(field(Some(surface), "state")) == "transition_debt" {
            self.numeric_dimensions(
                field(Some(surface), "baseline"),
                &format!("surface '{id}' baseline"),
                false,
                &DIMENSIONS,
            );
            let transition = field(Some(surface), "transition");
            if !matches!(transition, Some(Py::Dict(_)))
                || defined_scalar(field(transition, "owner")).is_none_or(|owner| owner.is_empty())
            {
                self.problem(format!("surface '{id}' transition debt lacks an owner"));
            } else {
                self.numeric_dimensions(
                    field(transition, "max_growth"),
                    &format!("surface '{id}' transition.max_growth"),
                    false,
                    &DIMENSIONS,
                );
                let baseline = field(Some(surface), "baseline");
                let growth = field(transition, "max_growth");
                if matches!(baseline, Some(Py::Dict(_))) && matches!(growth, Some(Py::Dict(_))) {
                    for dimension in DIMENSIONS {
                        let before = field(baseline, dimension);
                        let allowance = field(growth, dimension);
                        if before.is_none_or(|value| matches!(value, Py::None))
                            || allowance.is_none_or(|value| matches!(value, Py::None))
                        {
                            continue;
                        }
                        let allowed = numeric(before) + numeric(allowance);
                        if metrics.get(dimension) as f64 > allowed {
                            self.problem(format!(
                                "surface '{id}' exceeds immutable baseline plus transition allowance for {dimension}: {} > {}",
                                metrics.get(dimension),
                                perl::number(allowed)
                            ));
                        }
                    }
                }
            }
        }
    }

    fn validate_lifecycle(
        &mut self,
        surface: &Py,
        paths: &[String],
        metrics: Metrics,
        id: &str,
        executed: &mut BTreeSet<String>,
        output: &mut dyn super::Output,
    ) {
        let lifecycle = scalar(field(Some(surface), "lifecycle"));
        self.required_scalar(surface, "verifier", &format!("surface '{id}'"));
        match lifecycle.as_str() {
            "bounded_snapshot" if paths.len() != 1 => {
                self.problem(format!(
                    "surface '{id}' bounded_snapshot must contain exactly one file"
                ));
            }
            "partitioned_canonical" => self.validate_index(surface, paths, id),
            "generated_projection" => {
                if scalar(field(Some(surface), "locator")) == "file" {
                    if paths.len() != 1 {
                        self.problem(format!(
                            "surface '{id}' generated_projection file must contain exactly one file"
                        ));
                    }
                } else {
                    self.validate_index(surface, paths, id);
                }
                let inputs = field(Some(surface), "canonical_inputs");
                if !matches!(inputs, Some(Py::List(items)) if !items.is_empty()) {
                    self.problem(format!(
                        "surface '{id}' generated_projection must name canonical_inputs"
                    ));
                } else if let Some(Py::List(inputs)) = inputs {
                    for input in inputs {
                        let valid = defined_scalar(Some(input)).is_some_and(|input| {
                            !input.is_empty() && safe_relative_pattern(&input)
                        });
                        if !valid {
                            self.problem(format!("surface '{id}' has an invalid canonical input"));
                        }
                    }
                }
                if let Some(verifier) =
                    self.required_scalar(surface, "freshness_verifier", &format!("surface '{id}'"))
                {
                    if !safe_relative_pattern(&verifier)
                        || verifier.contains('*')
                        || verifier.contains('?')
                    {
                        self.problem(format!(
                            "surface '{id}' freshness verifier must be one repository-relative executable path"
                        ));
                    } else {
                        let script = self.absolute(&verifier);
                        if !executable(&script) {
                            self.problem(format!(
                                "surface '{id}' freshness verifier '{verifier}' is missing or non-executable"
                            ));
                        } else if executed.insert(verifier.clone())
                            && !run_verifier(&script, output)
                        {
                            self.problem(format!(
                                "surface '{id}' freshness verifier '{verifier}' failed"
                            ));
                        }
                    }
                }
            }
            "frozen_legacy" => {
                if paths.len() != 1 {
                    self.problem(format!(
                        "surface '{id}' frozen_legacy must contain exactly one file"
                    ));
                }
                if let Some(expected) =
                    self.required_scalar(surface, "sha256", &format!("surface '{id}'"))
                    && paths.len() == 1
                {
                    match fs::read(self.absolute(&paths[0])) {
                        Ok(raw) => {
                            if sha256_hex(&raw) != expected {
                                self.problem(format!("surface '{id}' frozen hash changed"));
                            }
                        }
                        Err(_) => {
                            self.problem(format!("cannot hash frozen surface '{id}'"));
                            return;
                        }
                    }
                }
            }
            "maintained_reference" => {
                let ceilings = field(Some(surface), "enforcement_ceilings");
                for dimension in ["lines_each", "bytes_each", "line_bytes_each"] {
                    if field(ceilings, dimension).is_none_or(|value| matches!(value, Py::None)) {
                        self.problem(format!(
                            "surface '{id}' maintained_reference requires a numeric {dimension} ceiling"
                        ));
                    }
                }
                for dimension in ["files", "lines_total", "bytes_total"] {
                    if field(ceilings, dimension).is_some_and(|value| !matches!(value, Py::None)) {
                        self.problem(format!(
                            "surface '{id}' maintained_reference must delegate {dimension} to exact aggregate authority"
                        ));
                    }
                }
                self.validate_index(surface, paths, id);
                let contract = field(Some(surface), "reference_contract");
                if !matches!(contract, Some(Py::Dict(_))) {
                    self.problem(format!(
                        "surface '{id}' maintained_reference lacks reference_contract"
                    ));
                    return;
                }
                let mandatory = field(contract, "mandatory_read");
                if !matches!(mandatory, Some(Py::Dict(_))) {
                    self.problem(format!(
                        "surface '{id}' maintained_reference lacks mandatory_read contract"
                    ));
                } else {
                    let path = scalar(field(mandatory, "path"));
                    let file_metrics = self.measure_paths(std::slice::from_ref(&path));
                    if !paths.contains(&path) {
                        self.problem(format!(
                            "surface '{id}' mandatory_read path is outside the surface"
                        ));
                    }
                    if file_metrics.lines_total as f64
                        > field(mandatory, "lines_ceiling")
                            .map_or(-1.0, |value| numeric(Some(value)))
                    {
                        self.problem(format!(
                            "surface '{id}' mandatory_read exceeds line ceiling"
                        ));
                    }
                    if file_metrics.bytes_total as f64
                        > field(mandatory, "bytes_ceiling")
                            .map_or(-1.0, |value| numeric(Some(value)))
                    {
                        self.problem(format!(
                            "surface '{id}' mandatory_read exceeds byte ceiling"
                        ));
                    }
                }
                if numeric(field(contract, "max_navigation_depth")) != 1.0 {
                    self.problem(format!("surface '{id}' max_navigation_depth must be 1"));
                }
                let authority = field(contract, "aggregate_change");
                if !matches!(authority, Some(Py::Dict(_))) {
                    self.problem(format!("surface '{id}' lacks aggregate_change authority"));
                } else {
                    for key in ["authority_id", "owner", "rationale"] {
                        self.required_scalar(
                            authority.unwrap(),
                            key,
                            &format!("surface '{id}' aggregate authority"),
                        );
                    }
                    for dimension in ["files", "lines_total", "bytes_total"] {
                        let baseline = field(field(authority, "baseline"), dimension);
                        let delta = field(field(authority, "delta"), dimension);
                        if !digits(baseline) || !signed_digits(delta) {
                            self.problem(format!(
                                "surface '{id}' aggregate authority has invalid '{dimension}' baseline/delta"
                            ));
                            continue;
                        }
                        let expected = numeric(baseline) + numeric(delta);
                        if metrics.get(dimension) as f64 != expected {
                            self.problem(format!(
                                "surface '{id}' aggregate authority is stale for {dimension}: actual {}, expected {}",
                                metrics.get(dimension),
                                perl::number(expected)
                            ));
                        }
                    }
                }
            }
            "rolling_ledger" => {
                if paths.len() != 1 {
                    self.problem(format!(
                        "surface '{id}' rolling_ledger must contain exactly one file"
                    ));
                }
                if scalar(field(Some(surface), "state")) != "transition_debt"
                    && field(Some(surface), "archive_manifest")
                        .is_none_or(|value| matches!(value, Py::None))
                {
                    self.problem(format!(
                        "surface '{id}' rolling ledger must remain transition_debt until it has a shard/archive manifest"
                    ));
                }
            }
            "archive_terminal" => {
                if scalar(field(Some(surface), "state")) != "terminal" {
                    self.problem(format!(
                        "surface '{id}' archive terminal must have state=terminal"
                    ));
                }
                if let Some(manifest) =
                    self.required_scalar(surface, "archive_manifest", &format!("surface '{id}'"))
                {
                    let safe = safe_relative_pattern(&manifest)
                        && !manifest.contains('*')
                        && !manifest.contains('?');
                    if !safe {
                        self.problem(format!(
                            "surface '{id}' archive manifest must be one repository-relative file"
                        ));
                    }
                    if safe && !self.absolute(&manifest).is_file() {
                        self.problem(format!(
                            "surface '{id}' archive manifest '{manifest}' is missing"
                        ));
                    }
                }
            }
            _ => {}
        }
        if has_field(Some(surface), "currency") {
            self.validate_currency(surface, id, executed, output);
        }
    }

    fn validate_currency(
        &mut self,
        surface: &Py,
        id: &str,
        executed: &mut BTreeSet<String>,
        output: &mut dyn super::Output,
    ) {
        let currency = field(Some(surface), "currency");
        if !matches!(currency, Some(Py::Dict(_))) {
            self.problem(format!(
                "surface '{id}' currency contract must be an object"
            ));
            return;
        }
        let status = scalar(field(currency, "status"));
        let verifier = scalar(field(currency, "verifier"));
        match status.as_str() {
            "transition_debt" => {
                if defined_scalar(field(currency, "owner")).is_none_or(|owner| owner.is_empty()) {
                    self.problem(format!("surface '{id}' currency debt lacks an owner"));
                }
                if !valid_debt_verifier(&verifier) {
                    self.problem(format!(
                        "surface '{id}' currency debt verifier must use debt: identifier"
                    ));
                }
            }
            "enforced" => {
                let script = self.absolute(&verifier);
                if !safe_relative_pattern(&verifier)
                    || verifier.contains('*')
                    || verifier.contains('?')
                    || !executable(&script)
                {
                    self.problem(format!(
                        "surface '{id}' currency verifier '{verifier}' is missing or non-executable"
                    ));
                } else if executed.insert(verifier.clone()) && !run_verifier(&script, output) {
                    self.problem(format!(
                        "surface '{id}' currency verifier '{verifier}' failed"
                    ));
                }
            }
            _ => self.problem(format!(
                "surface '{id}' currency status must be transition_debt or enforced"
            )),
        }
    }

    fn validate_index(&mut self, surface: &Py, paths: &[String], id: &str) {
        let index = self.required_scalar(surface, "index", &format!("surface '{id}'"));
        let contract = field(Some(surface), "index_contract");
        if !matches!(contract, Some(Py::Dict(_))) {
            self.problem(format!("surface '{id}' lacks index_contract"));
            return;
        }
        let kind = scalar(field(contract, "kind"));
        let verifier = self.required_scalar(
            contract.unwrap(),
            "verifier",
            &format!("surface '{id}' index_contract"),
        );
        if has_field(contract, "route_surface") && kind != "routed_membership" {
            self.problem(format!(
                "surface '{id}' index_contract route_surface is only valid for routed_membership"
            ));
        }
        if kind == "query" {
            if index.as_deref().unwrap_or_default() != "git:query" {
                self.problem(format!("surface '{id}' query index must be git:query"));
            }
            if verifier
                .as_ref()
                .is_some_and(|verifier| verifier != "builtin:registry_targets")
            {
                self.problem(format!(
                    "surface '{id}' query index verifier must be builtin:registry_targets"
                ));
            }
            return;
        }
        if !matches!(
            kind.as_str(),
            "membership" | "external_membership" | "routed_membership"
        ) {
            self.problem(format!("surface '{id}' has unknown index kind '{kind}'"));
            return;
        }
        if verifier
            .as_ref()
            .is_some_and(|verifier| verifier != "builtin:markdown_links")
        {
            self.problem(format!(
                "surface '{id}' {kind} index verifier must be builtin:markdown_links"
            ));
        }
        let Some(index) = index else {
            return;
        };
        if !safe_relative_pattern(&index)
            || index.contains('*')
            || index.contains('?')
            || !index.ends_with(".md")
        {
            self.problem(format!(
                "surface '{id}' {kind} index '{index}' must be one safe repository-relative Markdown path"
            ));
            return;
        }
        let inside = paths.contains(&index);
        if kind == "external_membership" {
            if inside {
                self.problem(format!(
                    "surface '{id}' external_membership index '{index}' must be outside the surface"
                ));
            }
            if !self.path_seen.contains(&index) {
                self.problem(format!(
                    "surface '{id}' external_membership index '{index}' is not a classified Markdown surface"
                ));
            }
        } else if kind == "routed_membership" {
            let classified = self
                .matches_by_surface
                .values()
                .any(|members| members.contains(&index));
            if !inside && !classified {
                self.problem(format!(
                    "surface '{id}' routed_membership index '{index}' is outside the surface and is not a classified Markdown surface"
                ));
            }
        } else if !inside {
            self.problem(format!(
                "surface '{id}' {kind} index '{index}' is outside the surface"
            ));
        }
        if !self.absolute(&index).is_file() {
            self.problem(format!(
                "surface '{id}' membership index '{index}' is missing"
            ));
            return;
        }
        let Some(mut linked) = self.markdown_link_targets(&index) else {
            self.problem(format!(
                "surface '{id}' cannot read membership index '{index}'"
            ));
            return;
        };
        if kind == "routed_membership"
            && !self.expand_route_hop(contract, paths, id, &index, &mut linked)
        {
            return;
        }
        for path in paths {
            if path != &index && !linked.contains(path) {
                self.problem(format!(
                    "surface '{id}' index '{index}' does not link member '{path}'"
                ));
            }
        }
    }

    fn expand_route_hop(
        &mut self,
        contract: Option<&Py>,
        _paths: &[String],
        id: &str,
        index: &str,
        linked: &mut BTreeSet<String>,
    ) -> bool {
        let Some(route_id) = defined_scalar(field(contract, "route_surface"))
            .filter(|route_id| !route_id.is_empty())
        else {
            self.problem(format!(
                "surface '{id}' routed_membership lacks a route_surface"
            ));
            return false;
        };
        if route_id == id {
            self.problem(format!(
                "surface '{id}' routed_membership route_surface must name another surface"
            ));
            return false;
        }
        let Some(route) = self.surface_by_id.get(&route_id).cloned() else {
            self.problem(format!(
                "surface '{id}' routed_membership route_surface '{route_id}' is not a registered surface"
            ));
            return false;
        };
        let route_contract = field(Some(&route), "index_contract");
        if matches!(route_contract, Some(Py::Dict(_)))
            && scalar(field(route_contract, "kind")) == "routed_membership"
        {
            self.problem(format!(
                "surface '{id}' routed_membership route_surface '{route_id}' must not itself route"
            ));
            return false;
        }
        let route_paths = self
            .matches_by_surface
            .get(&route_id)
            .cloned()
            .unwrap_or_default();
        if route_paths.is_empty() {
            self.problem(format!(
                "surface '{id}' routed_membership route surface '{route_id}' matches no Markdown path"
            ));
            return false;
        }
        for route_path in route_paths {
            if !linked.contains(&route_path) {
                self.problem(format!(
                    "surface '{id}' index '{index}' does not link route member '{route_path}'"
                ));
                continue;
            }
            let Some(hop) = self.markdown_link_targets(&route_path) else {
                self.problem(format!(
                    "surface '{id}' cannot read route member '{route_path}'"
                ));
                continue;
            };
            linked.extend(hop);
        }
        true
    }

    fn markdown_link_targets(&mut self, relative: &str) -> Option<BTreeSet<String>> {
        let absolute = self.absolute(relative);
        let content = fs::read(&absolute).ok()?;
        let mut linked = BTreeSet::new();
        let mut position = 0;
        while position + 2 < content.len() {
            let Some(open_offset) = content[position..]
                .windows(2)
                .position(|pair| pair == b"](")
            else {
                break;
            };
            let open = position + open_offset + 2;
            let Some(close_offset) = content[open..].iter().position(|byte| *byte == b')') else {
                break;
            };
            let close = open + close_offset;
            position = close + 1;
            if close == open {
                continue;
            }
            let href = String::from_utf8_lossy(&content[open..close]).into_owned();
            let href = href.split('#').next().unwrap_or_default();
            if href.is_empty() || external_href(href) {
                continue;
            }
            let parent = absolute.parent().unwrap_or(&self.root);
            let Ok(candidate) = fs::canonicalize(parent.join(href.trim_start_matches('/'))) else {
                continue;
            };
            if candidate == self.root || !candidate.starts_with(&self.root) {
                continue;
            }
            linked.insert(self.relative_to_root(&candidate));
        }
        Some(linked)
    }

    fn validate_routes(&mut self, path: &Path) {
        if !path.is_file() {
            self.problem("README route registry is missing");
            return;
        }
        let raw = match fs::read(path) {
            Ok(raw) => raw,
            Err(error) => {
                self.problem(format!(
                    "cannot read README route registry: {}",
                    os_error(&error)
                ));
                return;
            }
        };
        let mut lines = raw_lines(&raw).into_iter();
        let header = lines.next().unwrap_or_default();
        let header = header.strip_suffix(b"\n").unwrap_or(header);
        if header != b"target\troute_kind\towner\tlifecycle\tpressure_control\tterminal" {
            self.problem("README route registry schema is unknown");
        }
        for line in lines {
            let line = line.strip_suffix(b"\n").unwrap_or(line);
            let target = line.split(|byte| *byte == b'\t').next().unwrap_or_default();
            let target = String::from_utf8_lossy(target);
            if target.is_empty() || target.starts_with("https://") || target.starts_with("git:") {
                continue;
            }
            if target.ends_with(".md") {
                if !self.path_seen.contains(target.as_ref()) {
                    self.problem(format!(
                        "README route '{target}' is not a classified Markdown surface"
                    ));
                }
            } else if target.ends_with('/')
                && !self
                    .path_seen
                    .iter()
                    .any(|path| path.starts_with(target.as_ref()))
            {
                self.problem(format!(
                    "README route collection '{target}' contains no classified Markdown"
                ));
            }
        }
    }

    fn registry_capacity_coherence(
        &mut self,
        meta: &Py,
        label: &str,
        records: i128,
        data_bytes: i128,
        header_bytes: i128,
    ) {
        if records == 0 || data_bytes < 1 {
            return;
        }
        for key in REGISTRY_BOUNDS {
            let value = field(Some(meta), key);
            if !digits(value) || numeric(value) == 0.0 {
                return;
            }
        }
        let mean = data_bytes / records;
        if mean < 1 {
            return;
        }
        let room = numeric(field(Some(meta), "max_bytes")) as i128 - header_bytes;
        let funded = if room > 0 { room / mean } else { 0 };
        let max_records = numeric(field(Some(meta), "max_records")) as i128;
        if funded >= max_records {
            return;
        }
        self.warnings.push(format!(
            "{label} byte bound funds {funded} records at its real mean of {mean} bytes, below the {max_records} its max_records declares"
        ));
    }

    fn validate_bounded_registry_population(
        &mut self,
        already_reported: &BTreeSet<String>,
        authorities: Option<&[Py]>,
        mut used_authority: Option<&mut BTreeSet<String>>,
    ) {
        for relative in self.bounded_registry_paths() {
            let absolute = self.absolute(&relative);
            let Ok(raw) = fs::read(&absolute) else {
                continue;
            };
            let file_bytes = raw.len() as i128;
            let lines = raw_lines(&raw);
            let first_physical = lines.first().copied().unwrap_or_default();
            let mut first = first_physical.strip_suffix(b"\n").unwrap_or(first_physical);
            first = first.strip_suffix(b"\r").unwrap_or(first);
            if first.is_empty() {
                continue;
            }
            let records = lines.len().saturating_sub(1) as i128;
            let Ok(meta @ Py::Dict(_)) = decode_json(first) else {
                continue;
            };
            if scalar(field(Some(&meta), "record_type")) != "registry" {
                continue;
            }
            if !already_reported.contains(&relative) {
                self.registry_pressure(
                    &meta,
                    &format!("registry '{relative}'"),
                    &[("records", records), ("bytes", file_bytes)],
                    &[("records", "max_records"), ("bytes", "max_bytes")],
                );
            }
            self.registry_capacity_coherence(
                &meta,
                &format!("registry '{relative}'"),
                records,
                file_bytes - first.len() as i128 - 1,
                first.len() as i128 + 1,
            );
            let Some(authorities) = authorities else {
                continue;
            };
            let previous = self.registry_text_at_head(&relative);
            if self.validate_registry_bound_history(
                &meta,
                authorities,
                previous.as_deref(),
                &relative,
            ) && let Some(used) = used_authority.as_deref_mut()
            {
                used.insert(relative);
            }
        }
    }

    fn validate_registry_bound_history(
        &mut self,
        meta: &Py,
        authorities: &[Py],
        previous_text: Option<&str>,
        relative: &str,
    ) -> bool {
        let mut authority_for = BTreeMap::new();
        for authority in authorities {
            if scalar(field(Some(authority), "record_type")) == "increase"
                && field(Some(authority), "registry_id")
                    .is_some_and(|value| !matches!(value, Py::None))
            {
                let (_, key) = authority_key(authority);
                authority_for.insert(key.unwrap_or_default(), authority);
            }
        }
        let authority = authority_for.get(relative).copied();
        let mut previous_meta = None;
        for line in previous_text.unwrap_or_default().split('\n') {
            if line.is_empty() {
                continue;
            }
            if let Ok(record @ Py::Dict(_)) = decode_json(line.as_bytes())
                && scalar(field(Some(&record), "record_type")) == "registry"
            {
                previous_meta = Some(record);
                break;
            }
        }
        let Some(previous_meta) = previous_meta else {
            return false;
        };
        let increased: Vec<&str> = REGISTRY_BOUNDS
            .iter()
            .copied()
            .filter(|dimension| {
                let before = field(Some(&previous_meta), dimension);
                let after = field(Some(meta), dimension);
                before.is_some_and(|value| !matches!(value, Py::None))
                    && after.is_some_and(|value| !matches!(value, Py::None))
                    && numeric(after) > numeric(before)
            })
            .collect();
        if increased.is_empty() {
            return false;
        }
        let old_subset = defined_subset(&previous_meta, &REGISTRY_BOUNDS);
        let new_subset = defined_subset(meta, &REGISTRY_BOUNDS);
        let exact = authority.is_some_and(|authority| {
            canonical_or_empty(field(Some(authority), "old")) == canonical(&old_subset)
                && canonical_or_empty(field(Some(authority), "new")) == canonical(&new_subset)
                && nonempty_defined(field(Some(authority), "work_unit"))
                && nonempty_defined(field(Some(authority), "owner"))
                && nonempty_defined(field(Some(authority), "rationale"))
        });
        if !exact {
            self.problem(format!(
                "registry '{relative}' increased header bounds without exact authority: {}",
                increased.join(", ")
            ));
        }
        true
    }

    fn validate_ceiling_history(
        &mut self,
        current_surfaces: &[Py],
        authorities: &[Py],
        registry_meta: &Py,
    ) {
        if self.git_top() != self.root.to_string_lossy() {
            return;
        }
        let Some(previous_text) = self.registry_text_at_head(&self.registry_rel.clone()) else {
            return;
        };
        let mut previous_records = Vec::new();
        for line in previous_text.split('\n') {
            if line.is_empty() {
                continue;
            }
            if let Ok(record @ Py::Dict(_)) = decode_json(line.as_bytes())
                && scalar(field(Some(&record), "record_type")) != "registry"
            {
                previous_records.push(record);
            }
        }
        let mut previous = BTreeMap::new();
        for record in previous_records {
            previous.insert(scalar(field(Some(&record), "surface_id")), record);
        }
        let mut authority_for = BTreeMap::new();
        for authority in authorities {
            if scalar(field(Some(authority), "record_type")) == "increase"
                && field(Some(authority), "surface_id")
                    .is_some_and(|value| !matches!(value, Py::None))
            {
                authority_for.insert(scalar(field(Some(authority), "surface_id")), authority);
            }
        }
        let mut used_authority = BTreeSet::new();
        for current in current_surfaces {
            let id_value = field(Some(current), "surface_id");
            if id_value.is_none_or(|value| matches!(value, Py::None)) {
                continue;
            }
            let id = scalar(id_value);
            let Some(old) = previous.get(&id).cloned() else {
                continue;
            };
            if scalar(field(Some(&old), "state")) == "transition_debt"
                && scalar(field(Some(current), "state")) == "transition_debt"
                && canonical_or_empty(field(Some(&old), "baseline"))
                    != canonical_or_empty(field(Some(current), "baseline"))
            {
                self.problem(format!(
                    "surface '{id}' moved its immutable transition baseline"
                ));
            }
            if scalar(field(Some(&old), "lifecycle")) == "maintained_reference"
                && scalar(field(Some(current), "lifecycle")) == "maintained_reference"
            {
                let old_contract = field(Some(&old), "reference_contract");
                let new_contract = field(Some(current), "reference_contract");
                let old_change = field(old_contract, "aggregate_change");
                let new_change = field(new_contract, "aggregate_change");
                let mut aggregate_changed = false;
                for dimension in ["files", "lines_total", "bytes_total"] {
                    let old_expected = numeric(field(field(old_change, "baseline"), dimension))
                        + numeric(field(field(old_change, "delta"), dimension));
                    let new_expected = numeric(field(field(new_change, "baseline"), dimension))
                        + numeric(field(field(new_change, "delta"), dimension));
                    if old_expected != new_expected {
                        aggregate_changed = true;
                    }
                }
                if aggregate_changed
                    && scalar(field(old_change, "authority_id"))
                        == scalar(field(new_change, "authority_id"))
                {
                    self.problem(format!(
                        "surface '{id}' reused maintained-reference authority across aggregate change"
                    ));
                }
            }
            let old_ceilings = field(Some(&old), "enforcement_ceilings");
            let new_ceilings = field(Some(current), "enforcement_ceilings");
            let mut increased = Vec::new();
            for dimension in DIMENSIONS {
                let before = field(old_ceilings, dimension);
                let after = field(new_ceilings, dimension);
                if before.is_some_and(|value| !matches!(value, Py::None))
                    && (after.is_none_or(|value| matches!(value, Py::None))
                        || numeric(after) > numeric(before))
                {
                    increased.push(dimension);
                }
            }
            if increased.is_empty() {
                continue;
            }
            let authority = authority_for.get(&id).copied();
            if authority.is_some() {
                used_authority.insert(id.clone());
            }
            let exact = authority.is_some_and(|authority| {
                canonical_or_empty(field(Some(authority), "old"))
                    == canonical_or_empty(old_ceilings)
                    && canonical_or_empty(field(Some(authority), "new"))
                        == canonical_or_empty(new_ceilings)
                    && nonempty_defined(field(Some(authority), "work_unit"))
                    && nonempty_defined(field(Some(authority), "owner"))
                    && nonempty_defined(field(Some(authority), "rationale"))
            });
            if !exact {
                self.problem(format!(
                    "surface '{id}' increased ceiling dimensions without exact authority: {}",
                    increased.join(", ")
                ));
            }
        }
        if self.validate_registry_bound_history(
            registry_meta,
            authorities,
            Some(&previous_text),
            &self.registry_rel.clone(),
        ) {
            used_authority.insert(self.registry_rel.clone());
        }
        let already: BTreeSet<String> = [self.registry_rel.clone(), self.authorities_rel.clone()]
            .into_iter()
            .collect();
        self.validate_bounded_registry_population(
            &already,
            Some(authorities),
            Some(&mut used_authority),
        );
        for authority in authorities {
            let (_, id) = authority_key(authority);
            if let Some(id) = id
                && !used_authority.contains(&id)
            {
                self.problem(format!(
                    "'{id}' has unused or banked ceiling-increase authority"
                ));
            }
        }
    }
}

fn collect_scalar_types(
    value: &Py,
    key: Option<&str>,
    where_: &str,
    number_first_seen: &mut BTreeMap<String, String>,
    string_fields: &mut Vec<(String, String, String)>,
) {
    match value {
        Py::Dict(pairs) => {
            let mut entries: Vec<(String, &Py)> = pairs
                .iter()
                .map(|(key, value)| (pyjson::to_text(key), value))
                .collect();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            for (child_key, value) in entries {
                collect_scalar_types(
                    value,
                    Some(&child_key),
                    where_,
                    number_first_seen,
                    string_fields,
                );
            }
        }
        Py::List(items) => {
            for item in items {
                collect_scalar_types(item, key, where_, number_first_seen, string_fields);
            }
        }
        Py::Str(_) if key.is_some() => string_fields.push((
            key.unwrap().to_string(),
            where_.to_string(),
            scalar(Some(value)),
        )),
        Py::Int(_) | Py::Float(_) if key.is_some() => {
            let key = key.unwrap().to_string();
            number_first_seen
                .entry(key)
                .or_insert_with(|| where_.to_string());
        }
        _ => {}
    }
}

fn numeric_evidence_string(value: &str) -> bool {
    let unsigned = value.strip_prefix('-').unwrap_or(value);
    let (integer, fraction) = unsigned
        .split_once('.')
        .map_or((unsigned, None), |(integer, fraction)| {
            (integer, Some(fraction))
        });
    let integer_valid = integer == "0"
        || (integer.len() <= 15
            && integer.starts_with(|character| matches!(character, '1'..='9'))
            && integer.bytes().all(|byte| byte.is_ascii_digit()));
    integer_valid
        && fraction.is_none_or(|fraction| {
            !fraction.is_empty() && fraction.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn authority_key(authority: &Py) -> (&'static str, Option<String>) {
    for key in ["surface_id", "registry_id"] {
        if let Some(value) = defined_scalar(field(Some(authority), key))
            && !value.is_empty()
        {
            return (key, Some(value));
        }
    }
    ("surface_id", None)
}

fn defined_subset(value: &Py, keys: &[&str]) -> Py {
    Py::Dict(
        keys.iter()
            .filter_map(|key| {
                field(Some(value), key)
                    .filter(|value| !matches!(value, Py::None))
                    .map(|value| (pyjson::py_str(key), value.clone()))
            })
            .collect(),
    )
}

fn nonempty_defined(value: Option<&Py>) -> bool {
    value.is_some_and(|value| {
        !matches!(value, Py::None) && (perl::is_reference(value) || !scalar(Some(value)).is_empty())
    })
}

fn valid_debt_verifier(value: &str) -> bool {
    let value = value.strip_suffix('\n').unwrap_or(value);
    value.strip_prefix("debt:").is_some_and(|identifier| {
        !identifier.is_empty()
            && identifier.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
            })
            && identifier
                .as_bytes()
                .first()
                .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    })
}

fn external_href(href: &str) -> bool {
    let Some((scheme, _)) = href.split_once("://") else {
        return false;
    };
    !scheme.is_empty() && scheme.bytes().all(|byte| byte.is_ascii_alphabetic())
}

fn run_verifier(script: &Path, output: &mut dyn super::Output) -> bool {
    match Command::new(script).stdin(Stdio::inherit()).output() {
        Ok(process) => {
            output.out(&String::from_utf8_lossy(&process.stdout));
            output.err(&String::from_utf8_lossy(&process.stderr));
            process.status.success()
        }
        Err(_) => false,
    }
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

/// Run the live-document core checker with its original arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut requested_root = None;
    let mut registry_rel = "doctrine/live_document_size/surfaces.jsonl".to_string();
    let mut routes_rel = "doctrine/readme_entrypoint/routed_destinations.tsv".to_string();
    let mut authorities_rel =
        "doctrine/live_document_size/ceiling_increase_authorities.jsonl".to_string();
    let mut report = false;
    let mut no_history = false;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(directory) => requested_root = Some(PathBuf::from(directory)),
                None => return usage(output),
            },
            "--registry" => match arguments.next() {
                Some(path) => registry_rel = path.to_string(),
                None => return usage(output),
            },
            "--routes" => match arguments.next() {
                Some(path) => routes_rel = path.to_string(),
                None => return usage(output),
            },
            "--authorities" => match arguments.next() {
                Some(path) => authorities_rel = path.to_string(),
                None => return usage(output),
            },
            "--report" => report = true,
            "--no-history" => no_history = true,
            _ => return usage(output),
        }
    }
    let root = match requested_root {
        Some(directory) => {
            let canonical = fs::canonicalize(&directory).ok().or_else(|| {
                let name = directory.file_name()?;
                let parent = directory
                    .parent()
                    .filter(|parent| !parent.as_os_str().is_empty())
                    .unwrap_or_else(|| Path::new("."));
                Some(fs::canonicalize(parent).ok()?.join(name))
            });
            match canonical {
                Some(root) => root,
                None => {
                    output.err("live-document-size: repository root does not exist\n");
                    return 2;
                }
            }
        }
        None => root.to_path_buf(),
    };
    let mut state = State {
        root,
        registry_rel,
        routes_rel,
        authorities_rel,
        errors: Vec::new(),
        warnings: Vec::new(),
        path_seen: BTreeSet::new(),
        surface_by_id: BTreeMap::new(),
        matches_by_surface: BTreeMap::new(),
    };
    let registry_path = state.absolute(&state.registry_rel.clone());
    let registry = state.read_jsonl_registry(
        &registry_path,
        &state.registry_rel.clone(),
        "surface registry",
        128,
        131_072,
        8_192,
    );
    let authority_path = state.absolute(&state.authorities_rel.clone());
    let authority = state.read_jsonl_registry(
        &authority_path,
        &state.authorities_rel.clone(),
        "ceiling-authority registry",
        128,
        65_536,
        8_192,
    );
    let surfaces = registry.records;
    let authorities = authority.records;
    for surface in &surfaces {
        state.validate_surface_schema(surface);
    }
    for authority_record in &authorities {
        state.validate_authority_schema(authority_record);
    }
    let mut authority_surface_seen = BTreeSet::new();
    for authority_record in &authorities {
        let (_, id) = authority_key(authority_record);
        if let Some(id) = id
            && !authority_surface_seen.insert(id.clone())
        {
            state.problem(format!("duplicate ceiling authority for '{id}'"));
        }
    }
    let paths = match state.markdown_paths(output) {
        Ok(paths) => paths,
        Err(error) => {
            output.err(&format!("{error}\n"));
            return 255;
        }
    };
    state.path_seen = paths.iter().cloned().collect();
    for surface in &surfaces {
        let Some(id) = state.required_scalar(surface, "surface_id", "surface record") else {
            continue;
        };
        if !valid_identifier(&id) {
            state.problem(format!("surface_id '{id}' has an invalid identifier shape"));
        }
        if state.surface_by_id.contains_key(&id) {
            state.problem(format!("duplicate surface_id '{id}'"));
            continue;
        }
        state.surface_by_id.insert(id.clone(), surface.clone());
        let lifecycle = state.required_scalar(surface, "lifecycle", &format!("surface '{id}'"));
        if lifecycle
            .as_ref()
            .is_some_and(|lifecycle| !LIFECYCLES.contains(&lifecycle.as_str()))
        {
            state.problem(format!(
                "surface '{id}' has unknown lifecycle '{}'",
                lifecycle.unwrap()
            ));
        }
        let surface_state = state.required_scalar(surface, "state", &format!("surface '{id}'"));
        if surface_state
            .as_ref()
            .is_some_and(|value| !STATES.contains(&value.as_str()))
        {
            state.problem(format!(
                "surface '{id}' has unknown state '{}'",
                surface_state.unwrap()
            ));
        }
        state.required_scalar(surface, "owner", &format!("surface '{id}'"));
        let locator = state.required_scalar(surface, "locator", &format!("surface '{id}'"));
        if locator
            .as_ref()
            .is_some_and(|locator| !matches!(locator.as_str(), "file" | "collection"))
        {
            state.problem(format!(
                "surface '{id}' has unknown locator '{}'",
                locator.unwrap()
            ));
        }
        let targets = field(Some(surface), "targets");
        let Some(Py::List(targets)) =
            targets.filter(|value| matches!(value, Py::List(items) if !items.is_empty()))
        else {
            state.problem(format!(
                "surface '{id}' must declare a non-empty targets array"
            ));
            continue;
        };
        let mut regexes = Vec::new();
        for target in targets {
            let Some(target) = defined_scalar(Some(target)).filter(|target| !target.is_empty())
            else {
                state.problem(format!("surface '{id}' has an empty or non-scalar target"));
                continue;
            };
            if !safe_relative_pattern(&target) {
                state.problem(format!(
                    "surface '{id}' target '{target}' is absolute, parent-relative, or off-root"
                ));
                continue;
            }
            regexes.push(target);
        }
        let matched: Vec<String> = paths
            .iter()
            .filter(|path| regexes.iter().any(|pattern| glob_matches(pattern, path)))
            .cloned()
            .collect();
        if matched.is_empty() {
            state.problem(format!("surface '{id}' matches no Markdown path"));
        }
        if scalar(field(Some(surface), "locator")) == "file" && matched.len() > 1 {
            state.problem(format!(
                "surface '{id}' declares a file locator but matches {} paths; use the collection locator",
                matched.len()
            ));
        }
        state.matches_by_surface.insert(id, matched);
    }
    for path in &paths {
        let owners: Vec<String> = state
            .matches_by_surface
            .iter()
            .filter(|(_, members)| members.contains(path))
            .map(|(id, _)| id.clone())
            .collect();
        if owners.is_empty() {
            state.problem(format!("tracked Markdown '{path}' is not classified"));
        }
        if owners.len() > 1 {
            state.problem(format!(
                "tracked Markdown '{path}' is classified more than once: {}",
                owners
                    .iter()
                    .map(|id| id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    let ids: Vec<String> = state.surface_by_id.keys().cloned().collect();
    let mut executed_verifier = BTreeSet::new();
    for id in ids {
        let surface = state.surface_by_id.get(&id).unwrap().clone();
        let paths = state
            .matches_by_surface
            .get(&id)
            .cloned()
            .unwrap_or_default();
        let metrics = state.measure_paths(&paths);
        let unsealed = state.validate_sealed_members(&surface, &paths, &id);
        state.validate_limits(&surface, metrics, &id, unsealed);
        state.validate_cardinality_exemption(&surface, &id);
        state.validate_lifecycle(
            &surface,
            &paths,
            metrics,
            &id,
            &mut executed_verifier,
            output,
        );
        if report {
            let encoded = canonical(&Py::Dict(vec![
                (pyjson::py_str("surface_id"), Py::Str(pyjson::py_str(&id))),
                (pyjson::py_str("metrics"), metrics.json()),
            ]));
            output.out(&format!("{encoded}\n"));
        }
    }
    let routes_path = state.absolute(&state.routes_rel.clone());
    state.validate_routes(&routes_path);
    let directory = dirname(&state.registry_rel);
    state.validate_numeric_evidence_encoding(&directory);
    if no_history {
        let already: BTreeSet<String> = [state.registry_rel.clone(), state.authorities_rel.clone()]
            .into_iter()
            .collect();
        state.validate_bounded_registry_population(&already, None, None);
    } else {
        state.validate_ceiling_history(&surfaces, &authorities, &registry.meta);
    }
    if !state.errors.is_empty() {
        for error in &state.errors {
            output.err(&format!("{LABEL}: {error}\n"));
        }
        output.err(&format!(
            "{LABEL}: FAILED with {} violation(s).\n",
            state.errors.len()
        ));
        return 1;
    }
    for warning in &state.warnings {
        output.err(&format!("{LABEL}: warning: {warning}\n"));
    }
    if !report {
        output.out(&format!(
            "{LABEL}: {} Markdown files satisfy {} governed surfaces.\n",
            paths.len(),
            state.surface_by_id.len()
        ));
    }
    0
}

fn shim_path() -> String {
    std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "scripts/check_live_document_size.pl".to_string())
}

fn valid_identifier(id: &str) -> bool {
    !id.is_empty()
        && id.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
        && id
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}

fn usage(output: &mut dyn super::Output) -> i32 {
    let program = std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "cargo xtask check live-document-size-core".to_string());
    output.err(&format!(
        "Usage: {program} [--root DIR] [--registry PATH] [--routes PATH] [--authorities PATH] [--report] [--no-history]\n"
    ));
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_metrics_count_newlines_and_strip_carriage_returns_only_for_line_width() {
        let root = crate::repository_root();
        let mut state = State {
            root: root.clone(),
            registry_rel: String::new(),
            routes_rel: String::new(),
            authorities_rel: String::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            path_seen: BTreeSet::new(),
            surface_by_id: BTreeMap::new(),
            matches_by_surface: BTreeMap::new(),
        };
        let directory = root.join("generated/live-document-size-core-unit");
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("raw.md"), b"ab\r\nlast").unwrap();
        let metrics =
            state.measure_paths(&["generated/live-document-size-core-unit/raw.md".to_string()]);
        fs::remove_dir_all(&directory).unwrap();
        assert_eq!(
            metrics,
            Metrics {
                files: 1,
                lines_each: 1,
                bytes_each: 8,
                lines_total: 1,
                bytes_total: 8,
                line_bytes_each: 4,
            }
        );
        assert!(state.errors.is_empty());
    }

    #[test]
    fn glob_grammar_and_numeric_evidence_lexemes_match_the_checker() {
        assert!(glob_matches("docs/**/*.md", "docs/a/b.md"));
        assert!(glob_matches("docs/*.md", "docs/a.md"));
        assert!(!glob_matches("docs/*.md", "docs/a/b.md"));
        assert!(glob_matches("a?c.md", "abc.md"));
        for value in ["0", "-0", "1", "123456789012345", "1.25", "-12.0"] {
            assert!(numeric_evidence_string(value), "{value}");
        }
        for value in ["00", "01", "1234567890123456", "1.", ".1", "1,000", ""] {
            assert!(!numeric_evidence_string(value), "{value}");
        }
    }

    #[test]
    fn malformed_json_uses_json_pp_diagnostics() {
        for (input, expected) in [
            (
                "{",
                ", or } expected while parsing object/hash, at character offset 1 (before \"(end of string)\")",
            ),
            (
                "{bad",
                "unexpected end of string while parsing JSON string, at character offset 2 (before \"ad\")",
            ),
            (
                "\"\\x\"",
                "illegal backslash escape sequence in string, at character offset 1 (before \"\\\\x\"\")",
            ),
            (
                "\"\\ud800\\x\"",
                "illegal backslash escape sequence in string, at character offset 7 (before \"\\\\x\"\")",
            ),
        ] {
            let error = JsonErrorParser::new(input).run().unwrap_err();
            assert_eq!(error.render(input), expected, "{input:?}");
        }
    }
}
