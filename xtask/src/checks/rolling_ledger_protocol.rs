//! Rolling-ledger live-window/archive protocol, native since `RUSTDOC-AND-XTASK.15.w`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py, PyStr};

const ARCHIVE_INDEX_SURFACE_ID: &str = "rolling_ledger_archive_indexes";
const VERIFIER: &str = "scripts/check_rolling_ledger_protocol.pl";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Metrics {
    records: usize,
    lines: usize,
    bytes: usize,
    line_bytes: usize,
}

#[derive(Clone, Debug)]
struct Record {
    ordinal: usize,
    title: PyStr,
    bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
struct Parsed {
    prologue: Vec<u8>,
    records: Vec<Record>,
    trailer: Vec<u8>,
}

#[derive(Clone)]
struct Emission {
    bytes: Vec<u8>,
    source: String,
    state: String,
    capsule: String,
    source_sha256: String,
}

struct RegistryRead {
    meta: Py,
    ledgers: Vec<Py>,
}

struct RolloverPlan {
    control: Py,
    rows: Vec<Py>,
}

struct State {
    root: PathBuf,
    registry_rel: String,
    surfaces_rel: String,
    report: bool,
    errors: Vec<String>,
    notices: Vec<String>,
    surface_by_source: BTreeMap<String, Py>,
    archive_index_surface: Option<Py>,
    planned_rollover_ids: BTreeSet<String>,
    post_apply_validation: bool,
    emissions: BTreeMap<String, Emission>,
}

#[derive(Clone)]
struct PreparedEntry {
    id: String,
    source: String,
    segment_path: String,
    segment_bytes: Vec<u8>,
    segment_metrics: Metrics,
    root_bytes: Vec<u8>,
    root_metrics: Metrics,
    manifest_path: String,
    manifest_bytes: Vec<u8>,
    index_path: String,
    index_bytes: Vec<u8>,
    future_prepends: usize,
}

struct PreparedManifest {
    bytes: Vec<u8>,
    chain: Vec<String>,
    segments: Vec<Py>,
}

struct ArchiveValidation<'a> {
    id: &'a str,
    source: &'a str,
    source_metrics: Metrics,
    parsed: &'a Parsed,
    grammar: &'a Py,
}

struct TransactionContext {
    stage: PathBuf,
    backups: BTreeMap<String, Vec<u8>>,
    installed_segments: Vec<String>,
}

fn integer(value: usize) -> Py {
    Py::Int(value.to_string())
}

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

fn object(pairs: Vec<(&str, Py)>) -> Py {
    Py::Dict(
        pairs
            .into_iter()
            .map(|(key, value)| (pyjson::py_str(key), value))
            .collect(),
    )
}

fn empty_object() -> Py {
    object(Vec::new())
}

fn field<'a>(value: Option<&'a Py>, key: &str) -> Option<&'a Py> {
    value.and_then(|value| pyjson::get(value, key))
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

fn digits(value: Option<&Py>) -> bool {
    defined_scalar(value)
        .is_some_and(|text| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
}

fn positive_digits(value: Option<&Py>) -> bool {
    digits(value) && perl::numeric(value) > 0.0
}

fn numeric_or(value: Option<&Py>, default: f64) -> f64 {
    if value.is_none_or(|value| matches!(value, Py::None)) {
        default
    } else {
        perl::numeric(value)
    }
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

fn canonical(value: &Py) -> Result<String, String> {
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
    .ok_or_else(|| "cannot encode lone surrogate as UTF-8".to_string())
}

fn metrics(bytes: &[u8]) -> Metrics {
    Metrics {
        records: 0,
        lines: bytes.iter().filter(|byte| **byte == b'\n').count()
            + usize::from(!bytes.is_empty() && !bytes.ends_with(b"\n")),
        bytes: bytes.len(),
        line_bytes: bytes
            .split(|byte| *byte == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line).len())
            .max()
            .unwrap_or(0),
    }
}

fn metrics_json(metrics: Metrics) -> Py {
    object(vec![
        ("records", integer(metrics.records)),
        ("lines", integer(metrics.lines)),
        ("bytes", integer(metrics.bytes)),
        ("line_bytes", integer(metrics.line_bytes)),
    ])
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

fn chomp_line(mut line: &[u8]) -> &[u8] {
    if line.ends_with(b"\n") {
        line = &line[..line.len() - 1];
    }
    if line.ends_with(b"\r") {
        line = &line[..line.len() - 1];
    }
    line
}

fn safe_relative(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.split('/').any(|part| matches!(part, "." | ".."))
        && !path.bytes().any(|byte| matches!(byte, 0 | b'\r' | b'\n'))
}

fn valid_identifier(id: &str) -> bool {
    !id.is_empty()
        && id
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && id.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn valid_sha(value: Option<&Py>) -> bool {
    defined_scalar(value).is_some_and(|text| {
        text.len() == 64
            && text
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn valid_date(value: Option<&Py>) -> bool {
    defined_scalar(value).is_some_and(|text| {
        text.len() == 10
            && text.bytes().enumerate().all(|(index, byte)| {
                if matches!(index, 4 | 7) {
                    byte == b'-'
                } else {
                    byte.is_ascii_digit()
                }
            })
    })
}

fn surrogate_at(raw: &[u8], offset: usize) -> Option<u32> {
    let bytes = raw.get(offset..offset + 3)?;
    if bytes[0] == 0xed && (0xa0..=0xbf).contains(&bytes[1]) && (0x80..=0xbf).contains(&bytes[2]) {
        Some(
            (u32::from(bytes[0] & 0x0f) << 12)
                | (u32::from(bytes[1] & 0x3f) << 6)
                | u32::from(bytes[2] & 0x3f),
        )
    } else {
        None
    }
}

fn decode_wtf8(raw: &[u8]) -> Result<PyStr, String> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < raw.len() {
        if let Some(codepoint) = surrogate_at(raw, offset) {
            result.push(codepoint);
            offset += 3;
            continue;
        }
        match std::str::from_utf8(&raw[offset..]) {
            Ok(text) => {
                result.extend(pyjson::py_str(text));
                return Ok(result);
            }
            Err(error) => {
                let valid = error.valid_up_to();
                let prefix =
                    std::str::from_utf8(&raw[offset..offset + valid]).expect("valid prefix");
                result.extend(pyjson::py_str(prefix));
                let start = offset + valid;
                if surrogate_at(raw, start).is_some() {
                    offset = start;
                    continue;
                }
                return Err(format!("invalid UTF-8 at byte {start}"));
            }
        }
    }
    Ok(result)
}

fn codepoints_bytes(text: &[u32]) -> Vec<u8> {
    let mut raw = Vec::new();
    for codepoint in text {
        if (0xd800..0xe000).contains(codepoint) {
            raw.push(0xe0 | (*codepoint >> 12) as u8);
            raw.push(0x80 | ((*codepoint >> 6) & 0x3f) as u8);
            raw.push(0x80 | (*codepoint & 0x3f) as u8);
        } else {
            let character = char::from_u32(*codepoint).expect("a decoded codepoint");
            let mut encoded = [0; 4];
            raw.extend_from_slice(character.encode_utf8(&mut encoded).as_bytes());
        }
    }
    raw
}

fn py_bytes(value: Option<&Py>) -> Vec<u8> {
    match value {
        Some(Py::Str(text)) => codepoints_bytes(text),
        _ => scalar(value).into_bytes(),
    }
}

fn trim_successor_separator(bytes: &mut Vec<u8>) {
    for suffix in [b"\r\n\r\n".as_slice(), b"\r\n\n", b"\n\r\n", b"\n\n"] {
        if bytes.ends_with(suffix) {
            let keep = bytes.len() - suffix.len();
            bytes.truncate(keep);
            bytes.push(b'\n');
            return;
        }
    }
}

impl State {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            registry_rel: "doctrine/live_document_size/rolling_ledgers.jsonl".to_string(),
            surfaces_rel: "doctrine/live_document_size/surfaces.jsonl".to_string(),
            report: false,
            errors: Vec::new(),
            notices: Vec::new(),
            surface_by_source: BTreeMap::new(),
            archive_index_surface: None,
            planned_rollover_ids: BTreeSet::new(),
            post_apply_validation: false,
            emissions: BTreeMap::new(),
        }
    }

    fn absolute(&self, relative: &str) -> PathBuf {
        self.root.join(relative.trim_start_matches('/'))
    }

    fn problem(&mut self, message: impl Into<String>) {
        self.errors.push(message.into());
    }

    fn notice(&mut self, message: impl Into<String>) {
        self.notices.push(message.into());
    }

    fn reject_unknown(&mut self, value: Option<&Py>, label: &str, allowed: &[&str]) {
        let Some(Py::Dict(pairs)) = value else {
            self.problem(format!("{label} must be an object"));
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

    fn scalar_field(&mut self, value: Option<&Py>, key: &str, label: &str) -> Option<String> {
        match defined_scalar(field(value, key)) {
            Some(text) if !text.is_empty() => Some(text),
            _ => {
                self.problem(format!("{label} lacks scalar '{key}'"));
                None
            }
        }
    }

    fn validate_value_bounds(&mut self, value: &Py, label: &str, max_array: f64, max_scalar: f64) {
        match value {
            Py::Dict(pairs) => {
                for (key, item) in pairs {
                    self.validate_value_bounds(
                        item,
                        &format!("{label}.{}", pyjson::to_text(key)),
                        max_array,
                        max_scalar,
                    );
                }
            }
            Py::List(items) => {
                if items.len() as f64 > max_array {
                    self.problem(format!("{label} exceeds max_array_items"));
                }
                for item in items {
                    self.validate_value_bounds(item, &format!("{label}[]"), max_array, max_scalar);
                }
            }
            Py::None => {}
            Py::Str(text) => {
                if text.len() as f64 > max_scalar {
                    self.problem(format!("{label} exceeds max_scalar_bytes"));
                }
            }
            _ => {
                if scalar(Some(value)).len() as f64 > max_scalar {
                    self.problem(format!("{label} exceeds max_scalar_bytes"));
                }
            }
        }
    }

    fn slurp(&mut self, path: &Path, label: &str) -> Option<Vec<u8>> {
        match fs::read(path) {
            Ok(bytes) => Some(bytes),
            Err(error) => {
                self.problem(format!("cannot read {label}: {}", os_error(&error)));
                None
            }
        }
    }

    fn read_registry(&mut self) -> RegistryRead {
        let relative = self.registry_rel.clone();
        let path = self.absolute(&relative);
        if !path.is_file() {
            self.problem(format!("registry is missing: {relative}"));
            return RegistryRead {
                meta: empty_object(),
                ledgers: Vec::new(),
            };
        }
        let file_bytes = fs::metadata(&path)
            .map(|meta| meta.len() as usize)
            .unwrap_or(0);
        if file_bytes > 65_536 {
            self.problem("registry exceeds portable hard byte cap 65536");
        }
        let Some(raw) = self.slurp(&path, "registry") else {
            return RegistryRead {
                meta: empty_object(),
                ledgers: Vec::new(),
            };
        };
        let mut records = Vec::new();
        let mut lengths = Vec::new();
        for (index, physical) in raw_lines(&raw).iter().enumerate() {
            let line_number = index + 1;
            let line = chomp_line(physical);
            if line.is_empty() {
                continue;
            }
            if line.len() > 8_192 {
                self.problem(format!(
                    "registry line {line_number} exceeds portable raw-record cap 8192"
                ));
            }
            match decode_json(line, 151) {
                Ok(record @ Py::Dict(_)) => {
                    records.push(record);
                    lengths.push(line.len());
                }
                Ok(_) => self.problem(format!(
                    "registry line {line_number} is not one JSON object: "
                )),
                Err(error) => self.problem(format!(
                    "registry line {line_number} is not one JSON object: {error}"
                )),
            }
        }
        if records.is_empty() {
            self.problem("registry is empty");
            return RegistryRead {
                meta: empty_object(),
                ledgers: Vec::new(),
            };
        }
        let control = records.remove(0);
        self.reject_unknown(
            Some(&control),
            "registry control",
            &[
                "record_type",
                "schema_version",
                "max_records",
                "max_bytes",
                "max_record_bytes",
                "max_array_items",
                "max_scalar_bytes",
                "retired_paths",
                "milestones",
            ],
        );
        if scalar(field(Some(&control), "record_type")) != "registry" {
            self.problem("first record must have record_type=registry");
        }
        for key in [
            "schema_version",
            "max_records",
            "max_bytes",
            "max_record_bytes",
            "max_array_items",
            "max_scalar_bytes",
        ] {
            if !digits(field(Some(&control), key)) {
                self.problem(format!("registry control lacks numeric '{key}'"));
            }
        }
        if field(Some(&control), "schema_version").is_some_and(|value| !matches!(value, Py::None))
            && perl::numeric(field(Some(&control), "schema_version")) != 2.0
        {
            self.problem("registry schema_version must be 2");
        }
        for (key, hard) in [
            ("max_records", 32.0),
            ("max_bytes", 65_536.0),
            ("max_record_bytes", 8_192.0),
            ("max_array_items", 32.0),
            ("max_scalar_bytes", 1_024.0),
        ] {
            let value = field(Some(&control), key);
            if value.is_none_or(|value| matches!(value, Py::None) || perl::is_reference(value)) {
                continue;
            }
            if perl::numeric(value) == 0.0 {
                self.problem(format!("registry '{key}' must be positive"));
            }
            if perl::numeric(value) > hard {
                self.problem(format!(
                    "registry '{key}' exceeds portable hard cap {}",
                    perl::number(hard)
                ));
            }
        }
        if field(Some(&control), "max_records").is_some_and(|value| !matches!(value, Py::None))
            && records.len() as f64 > perl::numeric(field(Some(&control), "max_records"))
        {
            self.problem("registry has more records than max_records");
        }
        if field(Some(&control), "max_bytes").is_some_and(|value| !matches!(value, Py::None))
            && file_bytes as f64 > perl::numeric(field(Some(&control), "max_bytes"))
        {
            self.problem("registry exceeds max_bytes");
        }
        if field(Some(&control), "max_record_bytes").is_some_and(|value| !matches!(value, Py::None))
            && lengths.iter().any(|length| {
                *length as f64 > perl::numeric(field(Some(&control), "max_record_bytes"))
            })
        {
            self.problem("registry contains a record above max_record_bytes");
        }
        if field(Some(&control), "max_array_items").is_some_and(|value| !matches!(value, Py::None))
            && field(Some(&control), "max_scalar_bytes")
                .is_some_and(|value| !matches!(value, Py::None))
        {
            for record in &records {
                self.validate_value_bounds(
                    record,
                    "registry record",
                    perl::numeric(field(Some(&control), "max_array_items")),
                    perl::numeric(field(Some(&control), "max_scalar_bytes")),
                );
            }
        }
        self.validate_retired_paths(&control);
        RegistryRead {
            meta: control,
            ledgers: records,
        }
    }

    fn validate_retired_paths(&mut self, control: &Py) {
        let Some(Py::List(paths)) = field(Some(control), "retired_paths") else {
            self.problem("registry control retired_paths must be a non-empty array");
            return;
        };
        if paths.is_empty() {
            self.problem("registry control retired_paths must be a non-empty array");
            return;
        }
        if field(Some(control), "max_array_items").is_some_and(|value| !matches!(value, Py::None))
            && paths.len() as f64 > perl::numeric(field(Some(control), "max_array_items"))
        {
            self.problem("registry control retired_paths exceeds max_array_items");
        }
        if field(Some(control), "max_array_items").is_some_and(|value| !matches!(value, Py::None))
            && field(Some(control), "max_scalar_bytes")
                .is_some_and(|value| !matches!(value, Py::None))
        {
            let value = Py::List(paths.clone());
            self.validate_value_bounds(
                &value,
                "registry control.retired_paths",
                perl::numeric(field(Some(control), "max_array_items")),
                perl::numeric(field(Some(control), "max_scalar_bytes")),
            );
        }
        let mut seen = BTreeSet::new();
        for path in paths {
            let path = scalar(Some(path));
            if !safe_relative(&path) {
                self.problem(format!("registry retired path '{path}' is unsafe"));
            }
            if !seen.insert(path.clone()) {
                self.problem(format!("registry retired path '{path}' is duplicated"));
            }
            if safe_relative(&path) && self.absolute(&path).exists() {
                self.problem(format!("registry retired path '{path}' still exists"));
            }
        }
    }

    fn read_surface_authority(&mut self) {
        let relative = self.surfaces_rel.clone();
        let path = self.absolute(&relative);
        if !path.is_file() {
            self.problem(format!("surface registry is missing: {relative}"));
            return;
        }
        let Some(raw) = self.slurp(&path, "surface registry") else {
            return;
        };
        for (index, physical) in raw_lines(&raw).iter().enumerate() {
            let line_number = index + 1;
            let line = chomp_line(physical);
            if line.is_empty() {
                continue;
            }
            let record = match decode_json(line, 224) {
                Ok(record @ Py::Dict(_)) => record,
                Ok(_) => {
                    self.problem(format!(
                        "surface registry line {line_number} is not one JSON object: "
                    ));
                    continue;
                }
                Err(error) => {
                    self.problem(format!(
                        "surface registry line {line_number} is not one JSON object: {error}"
                    ));
                    continue;
                }
            };
            if scalar(field(Some(&record), "record_type")) == "registry" {
                continue;
            }
            if scalar(field(Some(&record), "surface_id")) == ARCHIVE_INDEX_SURFACE_ID {
                if self.archive_index_surface.is_some() {
                    self.problem(format!(
                        "duplicate '{ARCHIVE_INDEX_SURFACE_ID}' surface authority"
                    ));
                }
                self.archive_index_surface = Some(record);
                continue;
            }
            if scalar(field(Some(&record), "lifecycle")) != "rolling_ledger" {
                continue;
            }
            let Some(Py::List(targets)) = field(Some(&record), "targets") else {
                self.problem(format!(
                    "rolling-ledger surface at line {line_number} must declare one scalar target"
                ));
                continue;
            };
            if targets.len() != 1 || perl::is_reference(&targets[0]) {
                self.problem(format!(
                    "rolling-ledger surface at line {line_number} must declare one scalar target"
                ));
                continue;
            }
            let source = scalar(targets.first());
            if !safe_relative(&source) {
                self.problem(format!(
                    "rolling-ledger surface target '{source}' is unsafe"
                ));
            }
            if self.surface_by_source.contains_key(&source) {
                self.problem(format!(
                    "duplicate rolling-ledger surface authority for '{source}'"
                ));
            }
            self.surface_by_source.insert(source, record);
        }
    }

    fn read_rollover_plan(&mut self, relative: &str) -> RolloverPlan {
        if !safe_relative(relative) || !relative.ends_with(".jsonl") {
            self.problem(format!(
                "rollover plan path '{relative}' is unsafe or not JSONL"
            ));
            return RolloverPlan {
                control: empty_object(),
                rows: Vec::new(),
            };
        }
        let path = self.absolute(relative);
        if !path.is_file() {
            self.problem(format!("rollover plan is missing: {relative}"));
            return RolloverPlan {
                control: empty_object(),
                rows: Vec::new(),
            };
        }
        let Some(raw) = self.slurp(&path, &format!("rollover plan '{relative}'")) else {
            return RolloverPlan {
                control: empty_object(),
                rows: Vec::new(),
            };
        };
        let mut objects = Vec::new();
        let mut raw_lengths = Vec::new();
        for (index, physical) in raw_lines(&raw).iter().enumerate() {
            let line_number = index + 1;
            let line = chomp_line(physical);
            if line.is_empty() {
                continue;
            }
            match decode_json(line, 272) {
                Ok(value @ Py::Dict(_)) => {
                    objects.push(value);
                    raw_lengths.push(line.len());
                }
                Ok(_) => self.problem(format!(
                    "rollover plan line {line_number} is not one JSON object: "
                )),
                Err(error) => self.problem(format!(
                    "rollover plan line {line_number} is not one JSON object: {error}"
                )),
            }
        }
        if objects.is_empty() {
            self.problem("rollover plan is empty");
            return RolloverPlan {
                control: empty_object(),
                rows: Vec::new(),
            };
        }
        let control = objects.remove(0);
        self.reject_unknown(
            Some(&control),
            "rollover plan control",
            &[
                "record_type",
                "schema_version",
                "plan_id",
                "owner",
                "boundary_commit",
                "sealed_on",
                "order",
                "terminal_separator",
                "max_records",
                "max_bytes",
                "max_record_bytes",
                "max_scalar_bytes",
            ],
        );
        if scalar(field(Some(&control), "record_type")) != "rolling_ledger_rollover_plan" {
            self.problem("rollover plan control record_type must be rolling_ledger_rollover_plan");
        }
        if perl::numeric(field(Some(&control), "schema_version")) != 1.0 {
            self.problem("rollover plan schema_version must be 1");
        }
        for key in [
            "plan_id",
            "owner",
            "boundary_commit",
            "sealed_on",
            "order",
            "terminal_separator",
        ] {
            self.scalar_field(Some(&control), key, "rollover plan control");
        }
        let boundary = scalar(field(Some(&control), "boundary_commit"));
        if boundary.len() != 40
            || !boundary
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            self.problem("rollover plan boundary_commit must be a full Git object id");
        }
        if !valid_date(field(Some(&control), "sealed_on")) {
            self.problem("rollover plan sealed_on must be an ISO date");
        }
        if scalar(field(Some(&control), "order")) != "registry_order" {
            self.problem("rollover plan order must be registry_order");
        }
        if scalar(field(Some(&control), "terminal_separator")) != "canonical_single_newline" {
            self.problem("rollover plan terminal_separator must be canonical_single_newline");
        }
        for key in [
            "max_records",
            "max_bytes",
            "max_record_bytes",
            "max_scalar_bytes",
        ] {
            if !positive_digits(field(Some(&control), key)) {
                self.problem(format!(
                    "rollover plan control lacks positive numeric '{key}'"
                ));
            }
        }
        for (key, hard) in [
            ("max_records", 16.0),
            ("max_bytes", 32_768.0),
            ("max_record_bytes", 4_096.0),
            ("max_scalar_bytes", 1_024.0),
        ] {
            let value = field(Some(&control), key);
            if value.is_some_and(|value| !matches!(value, Py::None) && !perl::is_reference(value))
                && perl::numeric(value) > hard
            {
                self.problem(format!(
                    "rollover plan '{key}' exceeds portable hard cap {}",
                    perl::number(hard)
                ));
            }
        }
        if field(Some(&control), "max_records").is_some_and(|value| !matches!(value, Py::None))
            && objects.len() as f64 > perl::numeric(field(Some(&control), "max_records"))
        {
            self.problem("rollover plan exceeds max_records");
        }
        if field(Some(&control), "max_bytes").is_some_and(|value| !matches!(value, Py::None))
            && raw.len() as f64 > perl::numeric(field(Some(&control), "max_bytes"))
        {
            self.problem("rollover plan exceeds max_bytes");
        }
        if field(Some(&control), "max_record_bytes").is_some_and(|value| !matches!(value, Py::None))
            && raw_lengths.iter().any(|length| {
                *length as f64 > perl::numeric(field(Some(&control), "max_record_bytes"))
            })
        {
            self.problem("rollover plan contains a record above max_record_bytes");
        }
        let mut seen = BTreeSet::new();
        for row in &objects {
            self.reject_unknown(
                Some(row),
                "rollover plan row",
                &[
                    "ledger_id",
                    "source",
                    "opening_sha256",
                    "opening_records",
                    "post_migration_records",
                    "retained_migration_suffix_records",
                    "keep_opening_prefix_records",
                    "segment_id",
                    "segment_path",
                    "successor",
                    "records",
                    "lines",
                    "bytes",
                    "line_bytes",
                    "sha256",
                    "first_record_sha256",
                    "last_record_sha256",
                    "opening_root_after_cut",
                    "reason",
                ],
            );
            self.reject_unknown(
                field(Some(row), "opening_root_after_cut"),
                "rollover plan opening_root_after_cut",
                &["records", "lines", "bytes", "line_bytes", "sha256"],
            );
            for key in [
                "ledger_id",
                "source",
                "segment_id",
                "segment_path",
                "successor",
                "reason",
            ] {
                self.scalar_field(Some(row), key, "rollover plan row");
            }
            for key in [
                "opening_sha256",
                "sha256",
                "first_record_sha256",
                "last_record_sha256",
            ] {
                if !valid_sha(field(Some(row), key)) {
                    self.problem(format!("rollover plan row '{key}' is not SHA-256"));
                }
            }
            for key in [
                "opening_records",
                "post_migration_records",
                "retained_migration_suffix_records",
                "keep_opening_prefix_records",
                "records",
                "lines",
                "bytes",
                "line_bytes",
            ] {
                if !digits(field(Some(row), key)) {
                    self.problem(format!(
                        "rollover plan row '{key}' must be a nonnegative integer"
                    ));
                }
            }
            let opening = field(Some(row), "opening_root_after_cut");
            if !matches!(opening, Some(Py::Dict(_))) {
                self.problem("rollover plan row opening_root_after_cut must be an object");
            } else {
                for key in ["records", "lines", "bytes", "line_bytes"] {
                    if !digits(field(opening, key)) {
                        self.problem(format!(
                            "rollover plan opening_root_after_cut '{key}' must be a nonnegative integer"
                        ));
                    }
                }
                if !valid_sha(field(opening, "sha256")) {
                    self.problem("rollover plan opening_root_after_cut sha256 is not SHA-256");
                }
            }
            let id = scalar(field(Some(row), "ledger_id"));
            if !seen.insert(id.clone()) {
                self.problem(format!("rollover plan duplicates ledger_id '{id}'"));
            }
            for key in ["source", "segment_path", "successor"] {
                let value = scalar(field(Some(row), key));
                if !safe_relative(&value) {
                    self.problem(format!(
                        "rollover plan row '{key}' path '{value}' is unsafe"
                    ));
                }
            }
            if field(Some(&control), "max_scalar_bytes")
                .is_some_and(|value| !matches!(value, Py::None))
            {
                self.validate_value_bounds(
                    row,
                    "rollover plan row",
                    8.0,
                    perl::numeric(field(Some(&control), "max_scalar_bytes")),
                );
            }
        }
        RolloverPlan {
            control,
            rows: objects,
        }
    }

    fn validate_grammar_contract(&mut self, grammar: &Py, id: &str, kind: &str) {
        for key in ["detached_titles", "trailer_markers"] {
            if !matches!(field(Some(grammar), key), Some(Py::List(_))) {
                self.problem(format!("ledger '{id}' grammar '{key}' must be an array"));
            }
        }
        if !digits(field(Some(grammar), "skipped_h2")) {
            self.problem(format!(
                "ledger '{id}' grammar skipped_h2 must be a nonnegative integer"
            ));
        }
        if kind == "current_snapshot_bullets_v1" {
            self.scalar_field(
                Some(grammar),
                "start_marker",
                &format!("ledger '{id}' grammar"),
            );
            self.scalar_field(
                Some(grammar),
                "end_marker",
                &format!("ledger '{id}' grammar"),
            );
        } else if field(Some(grammar), "start_marker")
            .is_some_and(|value| !matches!(value, Py::None))
            || field(Some(grammar), "end_marker").is_some_and(|value| !matches!(value, Py::None))
        {
            self.problem(format!(
                "ledger '{id}' non-snapshot grammar must not declare range markers"
            ));
        }
        if kind != "changes_mixed_v1"
            && matches!(field(Some(grammar), "detached_titles"), Some(Py::List(items)) if !items.is_empty())
        {
            self.problem(format!(
                "ledger '{id}' detached titles are valid only for changes_mixed_v1"
            ));
        }
    }

    fn parse_ledger(&mut self, bytes: &[u8], grammar: &Py, label: &str) -> Option<Parsed> {
        match scalar(field(Some(grammar), "kind")).as_str() {
            "changes_mixed_v1" => self.parse_changes(bytes, grammar, label),
            "h2_records_v1" => self.parse_h2(bytes, grammar, label),
            "current_snapshot_bullets_v1" => self.parse_snapshot(bytes, grammar, label),
            kind => {
                self.problem(format!(
                    "ledger '{label}' cannot parse unknown grammar '{kind}'"
                ));
                None
            }
        }
    }

    fn decode_title(&mut self, raw: &[u8], label: &str) -> Option<PyStr> {
        match decode_wtf8(raw) {
            Ok(title) => Some(title),
            Err(error) => {
                self.problem(format!("ledger '{label}' heading is not UTF-8: {error}"));
                None
            }
        }
    }

    fn parse_changes(&mut self, bytes: &[u8], grammar: &Py, label: &str) -> Option<Parsed> {
        let detached: BTreeSet<PyStr> = match field(Some(grammar), "detached_titles") {
            Some(Py::List(items)) => items
                .iter()
                .filter_map(|item| match item {
                    Py::Str(title) => Some(title.clone()),
                    _ => None,
                })
                .collect(),
            _ => BTreeSet::new(),
        };
        let mut starts = Vec::new();
        let mut seen_h2 = false;
        let mut offset = 0;
        for line in raw_lines(bytes) {
            let content = line.strip_suffix(b"\n").unwrap_or(line);
            let content = content.strip_suffix(b"\r").unwrap_or(content);
            let heading = if content.starts_with(b"## ")
                && content.get(3) != Some(&b'#')
                && content.len() > 3
            {
                seen_h2 = true;
                Some((0, &content[3..]))
            } else if !seen_h2 && content.starts_with(b"### ") && content.len() > 4 {
                Some((1, &content[4..]))
            } else if seen_h2 && content.starts_with(b"### ") && content.len() > 4 {
                Some((2, &content[4..]))
            } else {
                None
            };
            if let Some((kind, title_raw)) = heading {
                let title = self.decode_title(title_raw, label)?;
                let mut full = if kind == 0 {
                    pyjson::py_str("## ")
                } else {
                    pyjson::py_str("### ")
                };
                full.extend(title);
                if kind != 2 || detached.contains(&full) {
                    starts.push((offset, full));
                }
            }
            offset += line.len();
        }
        if starts.first().is_none_or(|(start, _)| *start != 0) {
            self.problem(format!(
                "ledger '{label}' changes grammar requires a record at byte zero"
            ));
            return None;
        }
        Some(build_parts(
            bytes,
            Vec::new(),
            Vec::new(),
            starts,
            bytes.len(),
        ))
    }

    fn parse_h2(&mut self, bytes: &[u8], grammar: &Py, label: &str) -> Option<Parsed> {
        let mut all = Vec::new();
        let mut offset = 0;
        for line in raw_lines(bytes) {
            let content = line.strip_suffix(b"\n").unwrap_or(line);
            let content = content.strip_suffix(b"\r").unwrap_or(content);
            if content.starts_with(b"## ") && content.get(3) != Some(&b'#') && content.len() > 3 {
                let title = self.decode_title(&content[3..], label)?;
                let mut full = pyjson::py_str("## ");
                full.extend(title);
                all.push((offset, full));
            }
            offset += line.len();
        }
        let skip = perl::numeric(field(Some(grammar), "skipped_h2")) as usize;
        if all.len() <= skip {
            self.problem(format!(
                "ledger '{label}' h2 grammar found no record after {skip} skipped heading(s)"
            ));
            return None;
        }
        let starts = all[skip..].to_vec();
        let prologue = bytes[..starts[0].0].to_vec();
        Some(build_parts(
            bytes,
            prologue,
            Vec::new(),
            starts,
            bytes.len(),
        ))
    }

    fn parse_snapshot(&mut self, bytes: &[u8], grammar: &Py, label: &str) -> Option<Parsed> {
        let start = py_bytes(field(Some(grammar), "start_marker"));
        let end = py_bytes(field(Some(grammar), "end_marker"));
        let start_at = find_bytes(bytes, &start, 0)
            .map(|value| value as isize)
            .unwrap_or(-1);
        let end_at = find_bytes(bytes, &end, 0)
            .map(|value| value as isize)
            .unwrap_or(-1);
        if start_at < 0 || end_at < 0 || end_at <= start_at + start.len() as isize {
            self.problem(format!(
                "ledger '{label}' snapshot markers are missing, duplicated, or out of order"
            ));
            return None;
        }
        let start_at = start_at as usize;
        let end_at = end_at as usize;
        if find_bytes(bytes, &start, start_at + 1).is_some() {
            self.problem(format!(
                "ledger '{label}' snapshot start marker is not unique"
            ));
        }
        if find_bytes(bytes, &end, end_at + 1).is_some() {
            self.problem(format!(
                "ledger '{label}' snapshot end marker is not unique"
            ));
        }
        let records_at = start_at + start.len();
        let region = &bytes[records_at..end_at];
        let mut starts = Vec::new();
        let mut region_offset = 0;
        for line in raw_lines(region) {
            if line.is_empty() {
                break;
            }
            let offset = records_at + region_offset;
            region_offset += line.len();
            if matches!(line, b"\n" | b"\r\n") {
                continue;
            }
            if !line.starts_with(b"- ") {
                self.problem(format!(
                    "ledger '{label}' snapshot record region contains a non-bullet line"
                ));
                return None;
            }
            let content = line.strip_suffix(b"\n").unwrap_or(line);
            let content = content.strip_suffix(b"\r").unwrap_or(content);
            let title = self.decode_title(content, label)?;
            starts.push((offset, title));
        }
        if starts.is_empty() {
            self.problem(format!("ledger '{label}' snapshot has no bullet records"));
            return None;
        }
        let prologue = bytes[..records_at].to_vec();
        let trailer = bytes[end_at..].to_vec();
        if let Some(Py::List(markers)) = field(Some(grammar), "trailer_markers") {
            for marker in markers {
                let marker_text = scalar(Some(marker));
                if find_bytes(&trailer, &py_bytes(Some(marker)), 0).is_none() {
                    self.problem(format!(
                        "ledger '{label}' trailer lacks required marker '{marker_text}'"
                    ));
                }
            }
        }
        Some(build_parts(bytes, prologue, trailer, starts, end_at))
    }

    fn parse_segment(&mut self, bytes: &[u8], grammar: &Py, label: &str) -> Option<Vec<Record>> {
        match scalar(field(Some(grammar), "kind")).as_str() {
            "changes_mixed_v1" => self
                .parse_changes(bytes, grammar, label)
                .map(|parsed| parsed.records),
            "h2_records_v1" => {
                let mut segment_grammar = grammar.clone();
                set_field(&mut segment_grammar, "skipped_h2", integer(0));
                let parsed = self.parse_h2(bytes, &segment_grammar, label)?;
                if !parsed.prologue.is_empty() {
                    self.problem(format!(
                        "ledger '{label}' sealed segment has non-record prologue bytes"
                    ));
                    return None;
                }
                Some(parsed.records)
            }
            "current_snapshot_bullets_v1" => {
                let mut starts = Vec::new();
                let mut offset = 0;
                for line in raw_lines(bytes) {
                    if line.is_empty() {
                        break;
                    }
                    let line_offset = offset;
                    offset += line.len();
                    if matches!(line, b"\n" | b"\r\n") && offset == bytes.len() {
                        continue;
                    }
                    if !line.starts_with(b"- ") {
                        self.problem(format!(
                            "ledger '{label}' sealed status segment contains a non-bullet line"
                        ));
                        return None;
                    }
                    let content = line.strip_suffix(b"\n").unwrap_or(line);
                    let content = content.strip_suffix(b"\r").unwrap_or(content);
                    let title = self.decode_title(content, label)?;
                    starts.push((line_offset, title));
                }
                if starts.is_empty() {
                    self.problem(format!(
                        "ledger '{label}' sealed status segment has no records"
                    ));
                    return None;
                }
                Some(build_parts(bytes, Vec::new(), Vec::new(), starts, bytes.len()).records)
            }
            kind => {
                self.problem(format!(
                    "ledger '{label}' sealed segment has unknown grammar '{kind}'"
                ));
                None
            }
        }
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() {
        return Some(start.min(haystack.len()));
    }
    haystack
        .get(start..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|position| start + position)
}

fn build_parts(
    bytes: &[u8],
    prologue: Vec<u8>,
    trailer: Vec<u8>,
    starts: Vec<(usize, PyStr)>,
    record_end: usize,
) -> Parsed {
    let mut records = Vec::new();
    for (index, (start, title)) in starts.iter().enumerate() {
        let end = starts
            .get(index + 1)
            .map(|(next, _)| *next)
            .unwrap_or(record_end);
        records.push(Record {
            ordinal: index + 1,
            title: title.clone(),
            bytes: bytes[*start..end].to_vec(),
        });
    }
    Parsed {
        prologue,
        records,
        trailer,
    }
}

fn reconstruct(parsed: &Parsed) -> Vec<u8> {
    let mut bytes = parsed.prologue.clone();
    for record in &parsed.records {
        bytes.extend_from_slice(&record.bytes);
    }
    bytes.extend_from_slice(&parsed.trailer);
    bytes
}

fn render_live_view(parsed: &Parsed, selected: &[Record], grammar: &Py) -> Vec<u8> {
    let mut bytes = parsed.prologue.clone();
    for record in selected {
        bytes.extend_from_slice(&record.bytes);
    }
    bytes.extend_from_slice(&parsed.trailer);
    if scalar(field(Some(grammar), "kind")) == "h2_records_v1" && parsed.trailer.is_empty() {
        trim_successor_separator(&mut bytes);
    }
    bytes
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

fn shim_path() -> String {
    std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "scripts/check_rolling_ledger_protocol.pl".to_string())
}

fn decode_json(raw: &[u8], eval_line: usize) -> Result<Py, String> {
    let text = std::str::from_utf8(raw).map_err(|error| error.to_string())?;
    pyjson::loads_strict(text).map_err(|python_error| {
        JsonErrorParser::new(text)
            .run()
            .err()
            .map(|error| {
                format!(
                    "{} at {} line {eval_line}.\n",
                    error.render(text),
                    shim_path()
                )
            })
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
                                && low + 6 <= self.characters.len()
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

impl State {
    fn validate_ledger_schema(&mut self, ledger: &Py) {
        self.reject_unknown(
            Some(ledger),
            "ledger record",
            &[
                "ledger_id",
                "source",
                "owner",
                "order",
                "migration_state",
                "grammar",
                "planned_live",
                "live_limits",
                "archive",
                "consumers",
                "measurement",
            ],
        );
        if matches!(field(Some(ledger), "grammar"), Some(Py::Dict(_))) {
            self.reject_unknown(
                field(Some(ledger), "grammar"),
                "grammar",
                &[
                    "kind",
                    "skipped_h2",
                    "start_marker",
                    "end_marker",
                    "detached_titles",
                    "trailer_markers",
                ],
            );
        }
        if matches!(field(Some(ledger), "planned_live"), Some(Py::Dict(_))) {
            self.reject_unknown(
                field(Some(ledger), "planned_live"),
                "planned_live",
                &[
                    "prefix_records",
                    "promote_titles",
                    "records",
                    "lines",
                    "bytes",
                    "line_bytes",
                    "first_archived_sha256",
                ],
            );
        }
        if matches!(field(Some(ledger), "live_limits"), Some(Py::Dict(_))) {
            self.reject_unknown(
                field(Some(ledger), "live_limits"),
                "live_limits",
                &[
                    "records",
                    "lines",
                    "bytes",
                    "line_bytes",
                    "warning_pct",
                    "rollover_pct",
                ],
            );
        }
        if matches!(field(Some(ledger), "archive"), Some(Py::Dict(_))) {
            self.reject_unknown(
                field(Some(ledger), "archive"),
                "archive",
                &[
                    "landing",
                    "manifest",
                    "index",
                    "directory",
                    "source_capsule",
                    "retrieval",
                    "verifier",
                    "overlap",
                ],
            );
        }
        if matches!(field(Some(ledger), "consumers"), Some(Py::Dict(_))) {
            self.reject_unknown(
                field(Some(ledger), "consumers"),
                "consumers",
                &["readers", "writers", "required_literals"],
            );
        }
        if matches!(field(Some(ledger), "measurement"), Some(Py::Dict(_))) {
            self.reject_unknown(
                field(Some(ledger), "measurement"),
                "measurement",
                &["records", "lines", "bytes", "line_bytes", "sha256"],
            );
        }
        if let Some(Py::List(literals)) =
            field(field(Some(ledger), "consumers"), "required_literals")
        {
            for literal in literals {
                if matches!(literal, Py::Dict(_)) {
                    self.reject_unknown(Some(literal), "required literal", &["path", "literal"]);
                }
            }
        }
    }

    fn validate_surface_binding(&mut self, ledger: &Py, id: &str, source: &str) -> Option<Py> {
        let Some(surface) = self.surface_by_source.get(source).cloned() else {
            self.problem(format!(
                "ledger '{id}' lacks one generic rolling-ledger surface authority for '{source}'"
            ));
            return None;
        };
        if scalar(field(Some(&surface), "locator")) != "file" {
            self.problem(format!(
                "ledger '{id}' generic surface locator must be file"
            ));
        }
        if scalar(field(Some(&surface), "state")) != "normal" {
            self.problem(format!(
                "ledger '{id}' generic surface state must be normal"
            ));
        }
        if scalar(field(Some(&surface), "verifier")) != "builtin:budget" {
            self.problem(format!(
                "ledger '{id}' generic surface verifier must be builtin:budget"
            ));
        }
        let manifest = scalar(field(field(Some(ledger), "archive"), "manifest"));
        if scalar(field(Some(&surface), "archive_manifest")) != manifest {
            self.problem(format!(
                "ledger '{id}' generic surface archive manifest differs from the focused registry"
            ));
        }
        for key in ["health_targets", "enforcement_ceilings", "milestones"] {
            if !matches!(field(Some(&surface), key), Some(Py::Dict(_))) {
                self.problem(format!("ledger '{id}' generic surface lacks '{key}'"));
            }
        }
        let limits = field(Some(ledger), "live_limits");
        if matches!(limits, Some(Py::Dict(_)))
            && matches!(
                field(Some(&surface), "enforcement_ceilings"),
                Some(Py::Dict(_))
            )
        {
            for (key, surface_key) in [
                ("bytes", "bytes_each"),
                ("line_bytes", "line_bytes_each"),
                ("lines", "lines_each"),
            ] {
                if numeric_or(field(limits, key), -1.0)
                    != numeric_or(
                        field(field(Some(&surface), "enforcement_ceilings"), surface_key),
                        -2.0,
                    )
                {
                    self.problem(format!(
                        "ledger '{id}' focused {key} ceiling differs from generic {surface_key}"
                    ));
                }
            }
        }
        if matches!(limits, Some(Py::Dict(_)))
            && matches!(field(Some(&surface), "milestones"), Some(Py::Dict(_)))
        {
            for key in ["warning_pct", "rollover_pct"] {
                if numeric_or(field(limits, key), -1.0)
                    != numeric_or(field(field(Some(&surface), "milestones"), key), -2.0)
                {
                    self.problem(format!(
                        "ledger '{id}' focused {key} differs from generic surface authority"
                    ));
                }
            }
        }
        Some(surface)
    }

    fn select_live_records(
        &self,
        records: &[Record],
        plan: Option<&Py>,
        id: &str,
    ) -> (Vec<Record>, Vec<String>) {
        let mut selection_errors = Vec::new();
        if !matches!(plan, Some(Py::Dict(_))) {
            return (
                Vec::new(),
                vec![format!("ledger '{id}' planned_live must be an object")],
            );
        }
        let prefix_value = field(plan, "prefix_records");
        let prefix = perl::numeric(prefix_value) as usize;
        if !digits(prefix_value) || prefix < 1 || prefix >= records.len() {
            return (
                Vec::new(),
                vec![format!(
                    "ledger '{id}' prefix_records must be positive and leave nonempty archive history"
                )],
            );
        }
        let mut selected = records[..prefix].to_vec();
        let mut selected_ordinals: BTreeSet<usize> =
            selected.iter().map(|record| record.ordinal).collect();
        let Some(Py::List(promote)) = field(plan, "promote_titles") else {
            selection_errors.push(format!("ledger '{id}' promote_titles must be an array"));
            return (selected, selection_errors);
        };
        for title in promote {
            let matches: Vec<&Record> = records
                .iter()
                .filter(|record| match title {
                    Py::Str(title) => record.title == *title,
                    _ => false,
                })
                .collect();
            if matches.len() != 1 {
                selection_errors.push(format!(
                    "ledger '{id}' promoted title '{}' matches {} records, expected one",
                    scalar(Some(title)),
                    matches.len()
                ));
                continue;
            }
            if selected_ordinals.insert(matches[0].ordinal) {
                selected.push(matches[0].clone());
            }
        }
        (selected, selection_errors)
    }

    fn validate_planned(
        &mut self,
        plan: Option<&Py>,
        actual: Metrics,
        records: &[Record],
        selected: &[Record],
        id: &str,
    ) {
        if !matches!(plan, Some(Py::Dict(_))) {
            return;
        }
        for (key, value) in [
            ("records", actual.records),
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            let expected = field(plan, key);
            if !digits(expected) {
                self.problem(format!(
                    "ledger '{id}' planned_live '{key}' must be a nonnegative integer"
                ));
            }
            if digits(expected) && perl::numeric(expected) as usize != value {
                self.problem(format!(
                    "ledger '{id}' planned_live '{key}' drift: actual {value}, expected {}",
                    scalar(expected)
                ));
            }
        }
        let selected: BTreeSet<usize> = selected.iter().map(|record| record.ordinal).collect();
        let first_archived = records
            .iter()
            .find(|record| !selected.contains(&record.ordinal));
        let expected_sha = field(plan, "first_archived_sha256");
        if !valid_sha(expected_sha) {
            self.problem(format!(
                "ledger '{id}' planned_live lacks first_archived_sha256"
            ));
        }
        let actual_sha = first_archived
            .map(|record| sha256_hex(&record.bytes))
            .unwrap_or_default();
        if valid_sha(expected_sha) && actual_sha != scalar(expected_sha) {
            self.problem(format!(
                "ledger '{id}' first archived record identity drift: actual '{actual_sha}', expected '{}'",
                scalar(expected_sha)
            ));
        }
    }

    fn validate_exact_metrics(
        &mut self,
        expected: Option<&Py>,
        actual: Metrics,
        sha: &str,
        label: &str,
    ) {
        if !matches!(expected, Some(Py::Dict(_))) {
            self.problem(format!("{label} must be an object"));
            return;
        }
        for (key, value) in [
            ("records", actual.records),
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            let expected_value = field(expected, key);
            if !digits(expected_value) {
                self.problem(format!("{label} '{key}' must be a nonnegative integer"));
            }
            if digits(expected_value) && perl::numeric(expected_value) as usize != value {
                self.problem(format!(
                    "{label} '{key}' drift: actual {value}, expected {}",
                    scalar(expected_value)
                ));
            }
        }
        if scalar(field(expected, "sha256")) != sha {
            self.problem(format!(
                "{label} sha256 drift: actual {sha}, expected {}",
                defined_scalar(field(expected, "sha256"))
                    .unwrap_or_else(|| "<missing>".to_string())
            ));
        }
    }

    fn validate_limits(
        &mut self,
        limits: Option<&Py>,
        surface: Option<&Py>,
        actual: Metrics,
        id: &str,
        label: &str,
    ) {
        if !matches!(limits, Some(Py::Dict(_))) {
            self.problem(format!("ledger '{id}' live_limits must be an object"));
            return;
        }
        for key in [
            "records",
            "lines",
            "bytes",
            "line_bytes",
            "warning_pct",
            "rollover_pct",
        ] {
            if !positive_digits(field(limits, key)) {
                self.problem(format!(
                    "ledger '{id}' live_limits '{key}' must be a positive integer"
                ));
            }
        }
        if numeric_or(field(limits, "warning_pct"), 100.0)
            >= numeric_or(field(limits, "rollover_pct"), 0.0)
        {
            self.problem(format!(
                "ledger '{id}' warning_pct must be below rollover_pct"
            ));
        }
        if numeric_or(field(limits, "rollover_pct"), 100.0) >= 100.0 {
            self.problem(format!("ledger '{id}' rollover_pct must be below 100"));
        }
        let health = field(surface, "health_targets");
        for (key, value) in [
            ("records", actual.records),
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            let ceiling = field(limits, key);
            if !digits(ceiling) {
                continue;
            }
            if value as f64 > perl::numeric(ceiling) {
                self.problem(format!(
                    "ledger '{id}' {label} exceeds live {key} limit: {value} > {}",
                    scalar(ceiling)
                ));
            }
            let target = if key == "records" {
                ceiling
            } else {
                let health_key = match key {
                    "lines" => "lines_each",
                    "bytes" => "bytes_each",
                    _ => "line_bytes_each",
                };
                field(health, health_key)
            };
            if !positive_digits(target) {
                self.problem(format!(
                    "ledger '{id}' lacks positive generic health target for {key}"
                ));
                continue;
            }
            if label == "planned survivor"
                && value as f64 * 100.0
                    >= perl::numeric(target) * perl::numeric(field(limits, "warning_pct"))
            {
                self.problem(format!(
                    "ledger '{id}' planned survivor starts at or above the {}% warning for {key}",
                    scalar(field(limits, "warning_pct"))
                ));
            } else if label == "live source"
                && value as f64 * 100.0
                    >= perl::numeric(target) * perl::numeric(field(limits, "rollover_pct"))
                && (self.post_apply_validation || !self.planned_rollover_ids.contains(id))
            {
                self.problem(format!(
                    "ledger '{id}' live source reached the {}% rollover threshold for {key}",
                    scalar(field(limits, "rollover_pct"))
                ));
            }
        }
    }

    fn validate_rollover_metrics(
        &mut self,
        expected: Option<&Py>,
        actual: Metrics,
        sha: &str,
        label: &str,
    ) {
        if !matches!(expected, Some(Py::Dict(_))) {
            self.problem(format!("{label} metrics must be an object"));
            return;
        }
        for (key, value) in [
            ("records", actual.records),
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            let expected_value = field(expected, key);
            let matches = expected_value.is_some_and(|value| !matches!(value, Py::None))
                && perl::numeric(expected_value) == value as f64;
            if !matches {
                self.problem(format!(
                    "{label} {key} drift: actual {value}, expected {}",
                    defined_scalar(expected_value).unwrap_or_else(|| "<missing>".to_string())
                ));
            }
        }
        if scalar(field(expected, "sha256")) != sha {
            self.problem(format!(
                "{label} SHA-256 drift: actual {sha}, expected {}",
                defined_scalar(field(expected, "sha256"))
                    .unwrap_or_else(|| "<missing>".to_string())
            ));
        }
    }

    fn validate_warning_safe_survivor(
        &mut self,
        limits: Option<&Py>,
        surface: Option<&Py>,
        actual: Metrics,
        id: &str,
    ) {
        if !matches!(limits, Some(Py::Dict(_))) || !matches!(surface, Some(Py::Dict(_))) {
            return;
        }
        let health = field(surface, "health_targets");
        for (key, value) in [
            ("records", actual.records),
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            let ceiling = field(limits, key);
            if ceiling.is_some_and(|value| !matches!(value, Py::None))
                && value as f64 > perl::numeric(ceiling)
            {
                self.problem(format!(
                    "rollover plan ledger '{id}' survivor exceeds {key} ceiling"
                ));
            }
            let target = if key == "records" {
                ceiling
            } else {
                let health_key = match key {
                    "lines" => "lines_each",
                    "bytes" => "bytes_each",
                    _ => "line_bytes_each",
                };
                field(health, health_key)
            };
            if target.is_none_or(|value| matches!(value, Py::None))
                || field(limits, "warning_pct").is_none_or(|value| matches!(value, Py::None))
            {
                continue;
            }
            if value as f64 * 100.0
                >= perl::numeric(target) * perl::numeric(field(limits, "warning_pct"))
            {
                self.problem(format!(
                    "rollover plan ledger '{id}' survivor is not below the {}% warning for {key}",
                    scalar(field(limits, "warning_pct"))
                ));
            }
        }
    }

    fn measure_record_budget(
        &mut self,
        parsed: &Parsed,
        bytes: &[u8],
        limits: Option<&Py>,
        surface: Option<&Py>,
        id: &str,
    ) -> Option<Py> {
        if !matches!(limits, Some(Py::Dict(_))) || !matches!(surface, Some(Py::Dict(_))) {
            return None;
        }
        let window = field(limits, "records");
        let health = field(field(surface, "health_targets"), "bytes_each");
        if !positive_digits(window) || !positive_digits(health) || parsed.records.is_empty() {
            return None;
        }
        let record_bytes: usize = parsed.records.iter().map(|record| record.bytes.len()).sum();
        let overhead = bytes.len() as isize - record_bytes as isize;
        let health_bytes = perl::numeric(health) as usize;
        if overhead < 0 || overhead as usize >= health_bytes {
            self.problem(format!(
                "ledger '{id}' live-view overhead {overhead} is not below its {health_bytes}-byte health target"
            ));
            return None;
        }
        let overhead = overhead as usize;
        let available = health_bytes - overhead;
        let window_number = perl::numeric(window) as usize;
        let budget = available / window_number;
        if budget < 1 {
            return None;
        }
        let mut widest = 0;
        let mut widest_ordinal = 0;
        let mut oversized = 0;
        for record in &parsed.records {
            let size = record.bytes.len();
            if size > budget {
                oversized += 1;
            }
            if size > widest {
                widest = size;
                widest_ordinal = record.ordinal;
            }
        }
        let mean = record_bytes / parsed.records.len();
        let reachable = available
            .saturating_mul(parsed.records.len())
            .checked_div(record_bytes)
            .unwrap_or(0);
        Some(object(vec![
            ("records", integer(parsed.records.len())),
            ("declared_records", integer(window_number)),
            ("record_bytes", integer(record_bytes)),
            ("overhead_bytes", integer(overhead)),
            ("health_bytes", integer(health_bytes)),
            ("record_budget_bytes", integer(budget)),
            ("record_mean_bytes", integer(mean)),
            ("max_record_bytes", integer(widest)),
            ("max_record_ordinal", integer(widest_ordinal)),
            ("oversized_records", integer(oversized)),
            ("reachable_records", integer(reachable)),
        ]))
    }

    fn validate_record_budget(&mut self, measure: Option<&Py>, id: &str) {
        let Some(measure @ Py::Dict(_)) = measure else {
            return;
        };
        let reachable = perl::numeric(field(Some(measure), "reachable_records")) as usize;
        let declared = perl::numeric(field(Some(measure), "declared_records")) as usize;
        if reachable < declared {
            self.notice(format!(
                "ledger '{id}' cannot reach its declared {declared}-record window: at the measured {}-byte record mean the {}-byte health target holds {reachable} records, so the byte dimension binds first and a rollover only resets the clock",
                scalar(field(Some(measure), "record_mean_bytes")),
                scalar(field(Some(measure), "health_bytes")),
            ));
        }
        let oversized = perl::numeric(field(Some(measure), "oversized_records")) as usize;
        if oversized > 0 {
            self.notice(format!(
                "ledger '{id}' has {oversized} of {} live records above its derived {}-byte record budget (({} - {}) / {}); the widest is record {} at {} bytes",
                scalar(field(Some(measure), "records")),
                scalar(field(Some(measure), "record_budget_bytes")),
                scalar(field(Some(measure), "health_bytes")),
                scalar(field(Some(measure), "overhead_bytes")),
                scalar(field(Some(measure), "declared_records")),
                scalar(field(Some(measure), "max_record_ordinal")),
                scalar(field(Some(measure), "max_record_bytes")),
            ));
        }
    }

    fn validate_retained_suffix(&mut self, live: &[Record], selected: &[Record], id: &str) {
        if live.len() < selected.len() {
            self.problem(format!(
                "ledger '{id}' live source has fewer records than its retained migration window"
            ));
            return;
        }
        let offset = live.len() - selected.len();
        for (index, selected_record) in selected.iter().enumerate() {
            let live_bytes = &live[offset + index].bytes;
            let mut matches = live_bytes == &selected_record.bytes;
            if !matches && index + 1 == selected.len() {
                let mut without_separator = selected_record.bytes.clone();
                trim_successor_separator(&mut without_separator);
                matches = live_bytes == &without_separator;
            }
            if !matches {
                self.problem(format!(
                    "ledger '{id}' retained record identity/order changed at survivor index {}",
                    index + 1
                ));
            }
        }
    }

    fn validate_consumers(&mut self, consumers: Option<&Py>, id: &str) {
        if !matches!(consumers, Some(Py::Dict(_))) {
            self.problem(format!("ledger '{id}' consumers must be an object"));
            return;
        }
        for kind in ["readers", "writers"] {
            let Some(Py::List(paths)) = field(consumers, kind) else {
                self.problem(format!(
                    "ledger '{id}' consumers '{kind}' must be a non-empty array"
                ));
                continue;
            };
            if paths.is_empty() {
                self.problem(format!(
                    "ledger '{id}' consumers '{kind}' must be a non-empty array"
                ));
                continue;
            }
            for path in paths {
                let path = scalar(Some(path));
                if !safe_relative(&path) || !self.absolute(&path).exists() {
                    self.problem(format!(
                        "ledger '{id}' {kind} path '{path}' is missing or unsafe"
                    ));
                }
            }
        }
        let Some(Py::List(literals)) = field(consumers, "required_literals") else {
            self.problem(format!("ledger '{id}' required_literals must be an array"));
            return;
        };
        for entry in literals {
            if !matches!(entry, Py::Dict(_)) {
                self.problem(format!("ledger '{id}' required literal must be an object"));
                continue;
            }
            let path = scalar(field(Some(entry), "path"));
            let literal = scalar(field(Some(entry), "literal"));
            if !safe_relative(&path) || !self.absolute(&path).is_file() {
                self.problem(format!(
                    "ledger '{id}' required-literal path '{path}' is missing or unsafe"
                ));
                continue;
            }
            if let Some(bytes) = self.slurp(
                &self.absolute(&path),
                &format!("ledger '{id}' required-literal path"),
            ) && find_bytes(&bytes, &py_bytes(field(Some(entry), "literal")), 0).is_none()
            {
                self.problem(format!(
                    "ledger '{id}' required literal '{literal}' is absent from '{path}'"
                ));
            }
        }
    }

    fn validate_ledger(&mut self, ledger: &Py, id: &str, output: &mut dyn super::Output) {
        for key in ["source", "owner", "order", "migration_state"] {
            self.scalar_field(Some(ledger), key, &format!("ledger '{id}'"));
        }
        if scalar(field(Some(ledger), "order")) != "newest_first" {
            self.problem(format!("ledger '{id}' order must be newest_first"));
        }
        let state = scalar(field(Some(ledger), "migration_state"));
        if field(Some(ledger), "migration_state").is_some_and(|value| !matches!(value, Py::None))
            && !matches!(state.as_str(), "planned" | "migrated")
        {
            self.problem(format!(
                "ledger '{id}' has unknown migration_state '{state}'"
            ));
        }
        let source = scalar(field(Some(ledger), "source"));
        if !safe_relative(&source) || !source.ends_with(".md") {
            self.problem(format!(
                "ledger '{id}' source '{source}' is not one safe Markdown path"
            ));
        }
        if safe_relative(&source) && !self.absolute(&source).is_file() {
            self.problem(format!("ledger '{id}' source '{source}' is missing"));
        }
        let surface = self.validate_surface_binding(ledger, id, &source);
        let Some(grammar @ Py::Dict(_)) = field(Some(ledger), "grammar") else {
            self.problem(format!("ledger '{id}' grammar must be an object"));
            return;
        };
        let kind = self.scalar_field(Some(grammar), "kind", &format!("ledger '{id}' grammar"));
        if kind.as_ref().is_some_and(|kind| {
            !matches!(
                kind.as_str(),
                "changes_mixed_v1" | "h2_records_v1" | "current_snapshot_bullets_v1"
            )
        }) {
            self.problem(format!(
                "ledger '{id}' has unknown grammar '{}'",
                kind.clone().unwrap_or_default()
            ));
        }
        self.validate_grammar_contract(grammar, id, kind.as_deref().unwrap_or(""));
        let capsule = scalar(field(field(Some(ledger), "archive"), "source_capsule"));
        let measurement_path = if state == "migrated" {
            &capsule
        } else {
            &source
        };
        if state == "migrated" && (!safe_relative(&capsule) || !self.absolute(&capsule).is_file()) {
            self.problem(format!(
                "ledger '{id}' migrated source capsule '{capsule}' is missing or unsafe"
            ));
            return;
        }
        if !safe_relative(measurement_path) || !self.absolute(measurement_path).is_file() {
            return;
        }
        let Some(frozen_bytes) = self.slurp(
            &self.absolute(measurement_path),
            &format!("ledger '{id}' measurement source"),
        ) else {
            return;
        };
        let Some(parsed) = self.parse_ledger(&frozen_bytes, grammar, id) else {
            return;
        };
        if reconstruct(&parsed) != frozen_bytes {
            self.problem(format!(
                "ledger '{id}' parser does not reconstruct its source byte-for-byte"
            ));
        }
        let mut source_metrics = metrics(&frozen_bytes);
        source_metrics.records = parsed.records.len();
        self.validate_exact_metrics(
            field(Some(ledger), "measurement"),
            source_metrics,
            &sha256_hex(&frozen_bytes),
            &format!("ledger '{id}' measurement"),
        );
        let (selected, selection_errors) =
            self.select_live_records(&parsed.records, field(Some(ledger), "planned_live"), id);
        for error in selection_errors {
            self.problem(error);
        }
        let planned_bytes = render_live_view(&parsed, &selected, grammar);
        let mut planned_metrics = metrics(&planned_bytes);
        planned_metrics.records = selected.len();
        self.validate_planned(
            field(Some(ledger), "planned_live"),
            planned_metrics,
            &parsed.records,
            &selected,
            id,
        );
        self.validate_limits(
            field(Some(ledger), "live_limits"),
            surface.as_ref(),
            planned_metrics,
            id,
            "planned survivor",
        );
        self.validate_archive_contract(
            field(Some(ledger), "archive"),
            &state,
            ArchiveValidation {
                id,
                source: &source,
                source_metrics,
                parsed: &parsed,
                grammar,
            },
        );
        self.validate_consumers(field(Some(ledger), "consumers"), id);
        self.emissions.insert(
            id.to_string(),
            Emission {
                bytes: planned_bytes,
                source: source.clone(),
                state: state.clone(),
                capsule,
                source_sha256: scalar(field(field(Some(ledger), "measurement"), "sha256")),
            },
        );
        let mut live_report = None;
        if state == "migrated"
            && let Some(live_bytes) = self.slurp(
                &self.absolute(&source),
                &format!("ledger '{id}' live source"),
            )
            && let Some(live) = self.parse_ledger(&live_bytes, grammar, &format!("{id} live"))
        {
            if reconstruct(&live) != live_bytes {
                self.problem(format!(
                    "ledger '{id}' live parser does not reconstruct byte-for-byte"
                ));
            }
            let mut live_metrics = metrics(&live_bytes);
            live_metrics.records = live.records.len();
            self.validate_limits(
                field(Some(ledger), "live_limits"),
                surface.as_ref(),
                live_metrics,
                id,
                "live source",
            );
            self.validate_retained_suffix(&live.records, &selected, id);
            let budget = self.measure_record_budget(
                &live,
                &live_bytes,
                field(Some(ledger), "live_limits"),
                surface.as_ref(),
                id,
            );
            self.validate_record_budget(budget.as_ref(), id);
            let mut merged = metrics_json(live_metrics);
            if let Some(Py::Dict(budget_fields)) = budget {
                for (key, value) in budget_fields {
                    set_field(&mut merged, &pyjson::to_text(&key), value);
                }
            }
            live_report = Some(merged);
        }
        if self.report {
            let selected_ordinals: BTreeSet<usize> =
                selected.iter().map(|record| record.ordinal).collect();
            let first_archived = parsed
                .records
                .iter()
                .find(|record| !selected_ordinals.contains(&record.ordinal));
            let first_sha = parsed
                .records
                .first()
                .map(|record| sha256_hex(&record.bytes))
                .unwrap_or_default();
            let last_sha = parsed
                .records
                .last()
                .map(|record| sha256_hex(&record.bytes))
                .unwrap_or_default();
            let archived_sha = first_archived
                .map(|record| sha256_hex(&record.bytes))
                .unwrap_or_default();
            let mut source = metrics_json(source_metrics);
            set_field(&mut source, "sha256", text(&sha256_hex(&frozen_bytes)));
            let mut report = object(vec![
                ("ledger_id", text(id)),
                ("migration_state", text(&state)),
                ("source", source),
                ("source_first_record_sha256", text(&first_sha)),
                ("source_last_record_sha256", text(&last_sha)),
                ("planned_live", metrics_json(planned_metrics)),
                (
                    "archived_records",
                    integer(
                        source_metrics
                            .records
                            .saturating_sub(planned_metrics.records),
                    ),
                ),
                ("first_archived_sha256", text(&archived_sha)),
            ]);
            if let Some(live) = live_report {
                set_field(&mut report, "live", live);
            }
            match canonical(&report) {
                Ok(encoded) => output.out(&format!("{encoded}\n")),
                Err(error) => self.problem(error),
            }
        }
    }
}

impl State {
    fn validate_archive_contract(
        &mut self,
        archive: Option<&Py>,
        state: &str,
        validation: ArchiveValidation<'_>,
    ) {
        let ArchiveValidation {
            id,
            source,
            source_metrics,
            parsed,
            grammar,
        } = validation;
        if !matches!(archive, Some(Py::Dict(_))) {
            self.problem(format!("ledger '{id}' archive must be an object"));
            return;
        }
        for key in [
            "landing",
            "manifest",
            "index",
            "directory",
            "source_capsule",
            "verifier",
        ] {
            let path = self.scalar_field(archive, key, &format!("ledger '{id}' archive"));
            if path.as_ref().is_some_and(|path| !safe_relative(path)) {
                self.problem(format!(
                    "ledger '{id}' archive '{key}' path '{}' is unsafe",
                    path.unwrap_or_default()
                ));
            }
        }
        let retrieval = self.scalar_field(archive, "retrieval", &format!("ledger '{id}' archive"));
        if retrieval.as_ref().is_some_and(|retrieval| {
            retrieval != "perl scripts/check_rolling_ledger_protocol.pl --report"
        }) {
            self.problem(format!(
                "ledger '{id}' retrieval must invoke scripts/check_rolling_ledger_protocol.pl --report"
            ));
        }
        if scalar(field(archive, "verifier")) != VERIFIER {
            self.problem(format!(
                "ledger '{id}' archive verifier must be scripts/check_rolling_ledger_protocol.pl"
            ));
        }
        if scalar(field(archive, "overlap")) != "source_capsule_overlaps_live_window" {
            self.problem(format!(
                "ledger '{id}' archive overlap must declare source_capsule_overlaps_live_window"
            ));
        }
        let directory = scalar(field(archive, "directory"));
        for key in ["manifest", "index", "source_capsule"] {
            let path = scalar(field(archive, key));
            if !directory.is_empty() && !path.starts_with(&format!("{directory}/")) {
                self.problem(format!(
                    "ledger '{id}' archive {key} must be below '{directory}/'"
                ));
            }
        }
        if state != "migrated" {
            return;
        }
        for key in ["landing", "manifest", "index", "source_capsule", "verifier"] {
            let path = scalar(field(archive, key));
            if safe_relative(&path) && !self.absolute(&path).is_file() {
                self.problem(format!(
                    "ledger '{id}' migrated archive '{path}' is missing"
                ));
            }
        }
        let manifest = scalar(field(archive, "manifest"));
        let capsule = scalar(field(archive, "source_capsule"));
        if let Some(chain) = self.validate_archive_manifest(
            &manifest,
            &capsule,
            ArchiveValidation {
                id,
                source,
                source_metrics,
                parsed,
                grammar,
            },
        ) {
            let index = scalar(field(archive, "index"));
            self.validate_archive_index(&index, id, &manifest, &chain);
        }
    }

    fn validate_archive_manifest(
        &mut self,
        manifest_rel: &str,
        capsule: &str,
        validation: ArchiveValidation<'_>,
    ) -> Option<Vec<String>> {
        let ArchiveValidation {
            id,
            source,
            source_metrics,
            parsed,
            grammar,
        } = validation;
        if !safe_relative(manifest_rel) || !self.absolute(manifest_rel).is_file() {
            return None;
        }
        let path = self.absolute(manifest_rel);
        let file_bytes = fs::metadata(&path)
            .map(|meta| meta.len() as usize)
            .unwrap_or(0);
        if file_bytes > 65_536 {
            self.problem(format!(
                "ledger '{id}' archive manifest exceeds portable hard byte cap 65536"
            ));
        }
        let raw = match fs::read(&path) {
            Ok(raw) => raw,
            Err(_) => {
                self.problem(format!(
                    "ledger '{id}' cannot read archive manifest '{manifest_rel}'"
                ));
                return None;
            }
        };
        let mut objects = Vec::new();
        let mut raw_lengths = Vec::new();
        for (index, physical) in raw_lines(&raw).iter().enumerate() {
            let line_number = index + 1;
            let line = chomp_line(physical);
            if line.is_empty() {
                continue;
            }
            if line.len() > 4_096 {
                self.problem(format!(
                    "ledger '{id}' archive manifest line {line_number} exceeds portable record cap 4096"
                ));
            }
            match decode_json(line, 1412) {
                Ok(value @ Py::Dict(_)) => {
                    objects.push(value);
                    raw_lengths.push(line.len());
                }
                Ok(_) => self.problem(format!(
                    "ledger '{id}' archive manifest line {line_number} is not one JSON object: "
                )),
                Err(error) => self.problem(format!(
                    "ledger '{id}' archive manifest line {line_number} is not one JSON object: {error}"
                )),
            }
        }
        if objects.is_empty() {
            self.problem(format!("ledger '{id}' archive manifest is empty"));
            return None;
        }
        let control = objects.remove(0);
        self.reject_unknown(
            Some(&control),
            "archive manifest control",
            &[
                "record_type",
                "schema_version",
                "max_records",
                "max_bytes",
                "max_record_bytes",
                "max_scalar_bytes",
            ],
        );
        if scalar(field(Some(&control), "record_type")) != "archive_manifest" {
            self.problem(format!(
                "ledger '{id}' archive manifest control has unknown record_type"
            ));
        }
        for (key, _) in [
            ("schema_version", 1),
            ("max_records", 64),
            ("max_bytes", 65_536),
            ("max_record_bytes", 4_096),
            ("max_scalar_bytes", 1_024),
        ] {
            if !digits(field(Some(&control), key)) {
                self.problem(format!(
                    "ledger '{id}' archive manifest control lacks numeric '{key}'"
                ));
            }
        }
        if field(Some(&control), "schema_version").is_some_and(|value| !matches!(value, Py::None))
            && perl::numeric(field(Some(&control), "schema_version")) != 1.0
        {
            self.problem(format!(
                "ledger '{id}' archive manifest schema_version must be 1"
            ));
        }
        for (key, hard) in [
            ("max_records", 64.0),
            ("max_bytes", 65_536.0),
            ("max_record_bytes", 4_096.0),
            ("max_scalar_bytes", 1_024.0),
        ] {
            let value = field(Some(&control), key);
            if value.is_none_or(|value| matches!(value, Py::None) || perl::is_reference(value)) {
                continue;
            }
            if perl::numeric(value) == 0.0 {
                self.problem(format!(
                    "ledger '{id}' archive manifest '{key}' must be positive"
                ));
            }
            if perl::numeric(value) > hard {
                self.problem(format!(
                    "ledger '{id}' archive manifest '{key}' exceeds portable hard cap {}",
                    perl::number(hard)
                ));
            }
        }
        self.validate_archive_manifest_limits(&control, file_bytes, &objects, &raw_lengths, id);
        if field(Some(&control), "max_scalar_bytes").is_some_and(|value| !matches!(value, Py::None))
        {
            for entry in &objects {
                self.validate_value_bounds(
                    entry,
                    "archive manifest record",
                    1.0,
                    perl::numeric(field(Some(&control), "max_scalar_bytes")),
                );
            }
        }
        let mut path_seen = BTreeSet::new();
        let mut segment_seen = BTreeSet::new();
        for entry in &objects {
            let kind = scalar(field(Some(entry), "record_type"));
            let common = [
                "record_type",
                "ledger_id",
                "path",
                "sealed_on",
                "reason",
                "sha256",
                "records",
                "lines",
                "bytes",
                "line_bytes",
                "first_record_sha256",
                "last_record_sha256",
                "verifier",
            ];
            if kind == "source_capsule" {
                let mut allowed = common.to_vec();
                allowed.push("source");
                self.reject_unknown(Some(entry), "source capsule manifest record", &allowed);
            } else if kind == "sealed_segment" {
                let mut allowed = common.to_vec();
                allowed.extend(["segment_id", "predecessor", "successor"]);
                self.reject_unknown(Some(entry), "sealed segment manifest record", &allowed);
                if let Some(segment_id) =
                    self.scalar_field(Some(entry), "segment_id", "sealed segment manifest record")
                    && !segment_seen.insert(segment_id.clone())
                {
                    self.problem(format!(
                        "archive manifest duplicate segment_id '{segment_id}'"
                    ));
                }
            } else {
                self.problem(format!(
                    "ledger '{id}' archive manifest has unknown record_type '{kind}'"
                ));
                continue;
            }
            for key in [
                "ledger_id",
                "path",
                "sealed_on",
                "reason",
                "sha256",
                "first_record_sha256",
                "last_record_sha256",
                "verifier",
            ] {
                self.scalar_field(Some(entry), key, &format!("{kind} manifest record"));
            }
            self.validate_manifest_ledger_membership(entry, id);
            for key in ["records", "lines", "bytes", "line_bytes"] {
                if !digits(field(Some(entry), key)) {
                    self.problem(format!(
                        "{kind} manifest record '{key}' must be a nonnegative integer"
                    ));
                }
            }
            let entry_path = scalar(field(Some(entry), "path"));
            if !safe_relative(&entry_path) {
                self.problem(format!("archive manifest path '{entry_path}' is unsafe"));
            }
            if !path_seen.insert(entry_path.clone()) {
                self.problem(format!(
                    "archive manifest path '{entry_path}' is duplicated"
                ));
            }
            if safe_relative(&entry_path) && !self.absolute(&entry_path).is_file() {
                self.problem(format!("archive manifest path '{entry_path}' is missing"));
            }
            if !valid_date(field(Some(entry), "sealed_on")) {
                self.problem(format!("{kind} manifest record lacks ISO sealing date"));
            }
            if scalar(field(Some(entry), "verifier")) != VERIFIER {
                self.problem(format!("{kind} manifest record verifier mismatch"));
            }
            for key in ["sha256", "first_record_sha256", "last_record_sha256"] {
                if !valid_sha(field(Some(entry), key)) {
                    self.problem(format!("{kind} manifest record '{key}' is not SHA-256"));
                }
            }
            if !safe_relative(&entry_path) || !self.absolute(&entry_path).is_file() {
                continue;
            }
            let Some(entry_bytes) = self.slurp(
                &self.absolute(&entry_path),
                &format!("{kind} '{entry_path}'"),
            ) else {
                continue;
            };
            let entry_metrics = metrics(&entry_bytes);
            if scalar(field(Some(entry), "sha256")) != sha256_hex(&entry_bytes) {
                self.problem(format!(
                    "{kind} manifest record sha256 mismatch for '{entry_path}'"
                ));
            }
            for (key, actual) in [
                ("lines", entry_metrics.lines),
                ("bytes", entry_metrics.bytes),
                ("line_bytes", entry_metrics.line_bytes),
            ] {
                if !field(Some(entry), key).is_some_and(|value| !matches!(value, Py::None))
                    || perl::numeric(field(Some(entry), key)) != actual as f64
                {
                    self.problem(format!(
                        "{kind} manifest record '{key}' mismatch for '{entry_path}'"
                    ));
                }
            }
        }
        let matches: Vec<&Py> = objects
            .iter()
            .filter(|entry| {
                scalar(field(Some(entry), "record_type")) == "source_capsule"
                    && scalar(field(Some(entry), "ledger_id")) == id
            })
            .collect();
        if matches.len() != 1 {
            self.problem(format!(
                "ledger '{id}' archive manifest must contain exactly one source_capsule record"
            ));
            return None;
        }
        let entry = matches[0];
        for (key, expected) in [
            ("source", source),
            ("path", capsule),
            ("verifier", VERIFIER),
        ] {
            if scalar(field(Some(entry), key)) != expected {
                self.problem(format!("ledger '{id}' archive manifest '{key}' mismatch"));
            }
        }
        for (key, actual) in [
            ("records", source_metrics.records),
            ("lines", source_metrics.lines),
            ("bytes", source_metrics.bytes),
            ("line_bytes", source_metrics.line_bytes),
        ] {
            if !field(Some(entry), key).is_some_and(|value| !matches!(value, Py::None))
                || perl::numeric(field(Some(entry), key)) != actual as f64
            {
                self.problem(format!("ledger '{id}' archive manifest '{key}' mismatch"));
            }
        }
        if let Some(capsule_bytes) = self.slurp(
            &self.absolute(capsule),
            &format!("ledger '{id}' source capsule"),
        ) && scalar(field(Some(entry), "sha256")) != sha256_hex(&capsule_bytes)
        {
            self.problem(format!("ledger '{id}' archive manifest sha256 mismatch"));
        }
        let first_sha = parsed
            .records
            .first()
            .map(|record| sha256_hex(&record.bytes))
            .unwrap_or_default();
        let last_sha = parsed
            .records
            .last()
            .map(|record| sha256_hex(&record.bytes))
            .unwrap_or_default();
        if scalar(field(Some(entry), "first_record_sha256")) != first_sha {
            self.problem(format!(
                "ledger '{id}' archive manifest first record identity mismatch"
            ));
        }
        if scalar(field(Some(entry), "last_record_sha256")) != last_sha {
            self.problem(format!(
                "ledger '{id}' archive manifest last record identity mismatch"
            ));
        }
        let segments: Vec<Py> = objects
            .iter()
            .filter(|entry| {
                scalar(field(Some(entry), "record_type")) == "sealed_segment"
                    && scalar(field(Some(entry), "ledger_id")) == id
            })
            .cloned()
            .collect();
        for segment in &segments {
            let segment_path = scalar(field(Some(segment), "path"));
            if !safe_relative(&segment_path) || !self.absolute(&segment_path).is_file() {
                continue;
            }
            let Some(segment_bytes) = self.slurp(
                &self.absolute(&segment_path),
                &format!("ledger '{id}' sealed segment"),
            ) else {
                continue;
            };
            let Some(segment_parsed) =
                self.parse_segment(&segment_bytes, grammar, &format!("{id} segment"))
            else {
                continue;
            };
            let reconstructed: Vec<u8> = segment_parsed
                .iter()
                .flat_map(|record| record.bytes.clone())
                .collect();
            if reconstructed != segment_bytes {
                self.problem(format!(
                    "ledger '{id}' sealed segment does not reconstruct byte-for-byte"
                ));
            }
            if !field(Some(segment), "records").is_some_and(|value| !matches!(value, Py::None))
                || perl::numeric(field(Some(segment), "records")) != segment_parsed.len() as f64
            {
                self.problem(format!(
                    "ledger '{id}' sealed segment record count mismatch"
                ));
            }
            let first = segment_parsed
                .first()
                .map(|record| sha256_hex(&record.bytes))
                .unwrap_or_default();
            let last = segment_parsed
                .last()
                .map(|record| sha256_hex(&record.bytes))
                .unwrap_or_default();
            if scalar(field(Some(segment), "first_record_sha256")) != first {
                self.problem(format!(
                    "ledger '{id}' sealed segment first record identity mismatch"
                ));
            }
            if scalar(field(Some(segment), "last_record_sha256")) != last {
                self.problem(format!(
                    "ledger '{id}' sealed segment last record identity mismatch"
                ));
            }
        }
        Some(self.validate_archive_chain(id, source, capsule, &segments))
    }

    fn validate_archive_manifest_limits(
        &mut self,
        control: &Py,
        file_bytes: usize,
        objects: &[Py],
        raw_lengths: &[usize],
        id: &str,
    ) {
        if field(Some(control), "max_records").is_some_and(|value| !matches!(value, Py::None))
            && objects.len() as f64 > perl::numeric(field(Some(control), "max_records"))
        {
            self.problem(format!(
                "ledger '{id}' archive manifest exceeds its max_records"
            ));
        }
        if field(Some(control), "max_bytes").is_some_and(|value| !matches!(value, Py::None))
            && file_bytes as f64 > perl::numeric(field(Some(control), "max_bytes"))
        {
            self.problem(format!(
                "ledger '{id}' archive manifest exceeds its max_bytes"
            ));
        }
        if field(Some(control), "max_record_bytes").is_some_and(|value| !matches!(value, Py::None))
            && raw_lengths.iter().any(|length| {
                *length as f64 > perl::numeric(field(Some(control), "max_record_bytes"))
            })
        {
            self.problem(format!(
                "ledger '{id}' archive manifest contains a record above max_record_bytes"
            ));
        }
    }

    fn validate_manifest_ledger_membership(&mut self, entry: &Py, id: &str) {
        let actual = scalar(field(Some(entry), "ledger_id"));
        if actual != id {
            self.problem(format!(
                "ledger '{id}' archive manifest contains foreign ledger_id '{actual}'"
            ));
        }
    }

    fn validate_archive_chain(
        &mut self,
        id: &str,
        source: &str,
        capsule: &str,
        segments: &[Py],
    ) -> Vec<String> {
        let mut by_path = BTreeMap::new();
        let mut by_predecessor = BTreeMap::new();
        for segment in segments {
            let path = scalar(field(Some(segment), "path"));
            let predecessor = self.scalar_field(
                Some(segment),
                "predecessor",
                "sealed segment manifest record",
            );
            let successor =
                self.scalar_field(Some(segment), "successor", "sealed segment manifest record");
            if !path.is_empty() {
                if by_path.contains_key(&path) {
                    self.problem(format!(
                        "ledger '{id}' segment path '{path}' is duplicated in chronology"
                    ));
                } else {
                    by_path.insert(path.clone(), segment.clone());
                }
            }
            for (key, value) in [("predecessor", &predecessor), ("successor", &successor)] {
                if let Some(value) = value
                    && !safe_relative(value)
                {
                    self.problem(format!(
                        "ledger '{id}' segment '{path}' {key} '{value}' is unsafe"
                    ));
                }
            }
            if let Some(predecessor) = predecessor
                && by_predecessor
                    .insert(predecessor.clone(), segment.clone())
                    .is_some()
            {
                self.problem(format!(
                    "ledger '{id}' has duplicate chronology edge after '{predecessor}'"
                ));
            }
        }
        let mut ordered = vec![source.to_string()];
        let mut visited = BTreeSet::new();
        let mut predecessor = source.to_string();
        while let Some(segment) = by_predecessor.get(&predecessor).cloned() {
            let path = scalar(field(Some(&segment), "path"));
            if !visited.insert(path.clone()) {
                self.problem(format!(
                    "ledger '{id}' archive chronology contains a cycle at '{path}'"
                ));
                break;
            }
            ordered.push(path.clone());
            let next = by_predecessor.get(&path);
            let expected_successor = next
                .map(|segment| scalar(field(Some(segment), "path")))
                .unwrap_or_else(|| capsule.to_string());
            if scalar(field(Some(&segment), "successor")) != expected_successor {
                self.problem(format!(
                    "ledger '{id}' segment '{path}' successor mismatch: actual '{}', expected '{expected_successor}'",
                    scalar(field(Some(&segment), "successor"))
                ));
            }
            predecessor = path;
        }
        ordered.push(capsule.to_string());
        if !segments.is_empty() && !by_predecessor.contains_key(source) {
            self.problem(format!(
                "ledger '{id}' archive chronology does not start at live source '{source}'"
            ));
        }
        if visited.len() != segments.len() {
            self.problem(format!(
                "ledger '{id}' archive chronology is disconnected: visited {} of {} segments",
                visited.len(),
                segments.len()
            ));
        }
        let mut reported_cycle = BTreeSet::new();
        for start in by_path.keys() {
            let mut trail = BTreeSet::new();
            let mut cursor = start.clone();
            while let Some(segment) = by_path.get(&cursor) {
                if !trail.insert(cursor.clone()) {
                    if reported_cycle.insert(cursor.clone()) {
                        self.problem(format!(
                            "ledger '{id}' archive chronology contains a cycle at '{cursor}'"
                        ));
                    }
                    break;
                }
                let successor = scalar(field(Some(segment), "successor"));
                if !by_path.contains_key(&successor) {
                    break;
                }
                cursor = successor;
            }
        }
        ordered
    }

    fn validate_archive_index(
        &mut self,
        index_rel: &str,
        id: &str,
        manifest_rel: &str,
        chain: &[String],
    ) {
        if !safe_relative(index_rel) || !self.absolute(index_rel).is_file() {
            return;
        }
        if let Some(bytes) = self.slurp(
            &self.absolute(index_rel),
            &format!("ledger '{id}' archive index"),
        ) {
            self.validate_archive_index_content(&bytes, index_rel, id, manifest_rel, chain);
        }
    }

    fn validate_archive_index_content(
        &mut self,
        bytes: &[u8],
        index_rel: &str,
        id: &str,
        manifest_rel: &str,
        chain: &[String],
    ) {
        let links = markdown_link_paths(bytes, index_rel);
        let mut counts = BTreeMap::new();
        for path in &links {
            *counts.entry(path.clone()).or_insert(0) += 1;
        }
        for path in chain
            .iter()
            .chain(std::iter::once(&manifest_rel.to_string()))
        {
            if counts.get(path).copied().unwrap_or(0) != 1 {
                self.problem(format!(
                    "ledger '{id}' archive index must link '{path}' exactly once"
                ));
            }
        }
        let members: BTreeSet<String> = chain.iter().cloned().collect();
        let actual_order: Vec<String> = links
            .iter()
            .filter(|path| members.contains(*path))
            .cloned()
            .collect();
        if actual_order != chain {
            self.problem(format!(
                "ledger '{id}' archive index member order differs from verified chronology"
            ));
        }
        let allowed: BTreeSet<String> = chain
            .iter()
            .cloned()
            .chain(std::iter::once(manifest_rel.to_string()))
            .collect();
        for path in links {
            if !allowed.contains(&path) {
                self.problem(format!(
                    "ledger '{id}' archive index links undeclared member '{path}'"
                ));
            }
        }
    }

    fn validate_archive_index_cardinality(&mut self, control: &Py, ledgers: &[Py]) {
        let Some(surface) = self.archive_index_surface.clone() else {
            self.problem(format!(
                "surface registry lacks the '{ARCHIVE_INDEX_SURFACE_ID}' authority"
            ));
            return;
        };
        let cap = field(Some(control), "max_records");
        if !positive_digits(cap) {
            self.problem(
                "ledger registry max_records is unusable as the archive-index cardinality",
            );
            return;
        }
        let cap_number = perl::numeric(cap) as usize;
        for band in ["health_targets", "enforcement_ceilings"] {
            let bounds = field(Some(&surface), band);
            if !matches!(bounds, Some(Py::Dict(_))) {
                self.problem(format!("'{ARCHIVE_INDEX_SURFACE_ID}' lacks {band}"));
                continue;
            }
            let files = field(bounds, "files");
            if !positive_digits(files) || perl::numeric(files) as usize != cap_number {
                self.problem(format!(
                    "'{ARCHIVE_INDEX_SURFACE_ID}' {band} files {} must equal the ledger registry max_records {cap_number}",
                    defined_scalar(files).unwrap_or_else(|| "<missing>".to_string())
                ));
                continue;
            }
            for (total_key, each_key) in
                [("lines_total", "lines_each"), ("bytes_total", "bytes_each")]
            {
                let total = field(bounds, total_key);
                let each = field(bounds, each_key);
                if !digits(total) || !digits(each) {
                    continue;
                }
                if perl::numeric(total) != perl::numeric(files) * perl::numeric(each) {
                    self.problem(format!(
                        "'{ARCHIVE_INDEX_SURFACE_ID}' {band} {total_key} {} is not files x {each_key} ({} x {})",
                        scalar(total),
                        scalar(files),
                        scalar(each)
                    ));
                }
            }
        }
        let Some(Py::List(targets)) = field(Some(&surface), "targets") else {
            return;
        };
        let patterns: Vec<String> = targets
            .iter()
            .filter_map(|target| defined_scalar(Some(target)))
            .collect();
        for ledger in ledgers {
            if scalar(field(Some(ledger), "migration_state")) != "migrated" {
                continue;
            }
            let index = scalar(field(field(Some(ledger), "archive"), "index"));
            if index.is_empty() {
                continue;
            }
            if !patterns
                .iter()
                .any(|pattern| archive_index_pattern(pattern, &index))
            {
                self.problem(format!(
                    "ledger '{}' archive index '{index}' is outside the '{ARCHIVE_INDEX_SURFACE_ID}' surface",
                    scalar(field(Some(ledger), "ledger_id"))
                ));
            }
        }
    }

    fn validate_archive_landings(&mut self, control: &Py, ledgers: &[Py]) {
        let migrated: Vec<&Py> = ledgers
            .iter()
            .filter(|ledger| scalar(field(Some(ledger), "migration_state")) == "migrated")
            .collect();
        if migrated.is_empty() {
            return;
        }
        let mut landing_seen = BTreeSet::new();
        let mut index_seen = BTreeSet::new();
        let mut manifest_seen = BTreeSet::new();
        let mut directory_seen = BTreeSet::new();
        let retired: BTreeSet<String> = match field(Some(control), "retired_paths") {
            Some(Py::List(paths)) => paths
                .iter()
                .filter_map(|path| defined_scalar(Some(path)))
                .collect(),
            _ => BTreeSet::new(),
        };
        let mut ids = Vec::new();
        let mut expected_links = Vec::new();
        for ledger in migrated {
            let id = scalar(field(Some(ledger), "ledger_id"));
            let Some(archive @ Py::Dict(_)) = field(Some(ledger), "archive") else {
                continue;
            };
            let landing = scalar(field(Some(archive), "landing"));
            if !landing.is_empty() {
                landing_seen.insert(landing.clone());
            }
            for (key, seen) in [
                ("index", &mut index_seen),
                ("manifest", &mut manifest_seen),
                ("directory", &mut directory_seen),
            ] {
                let path = scalar(field(Some(archive), key));
                if !path.is_empty() && !seen.insert(path.clone()) {
                    self.problem(format!("ledger '{id}' reuses archive {key} '{path}'"));
                }
                if retired.contains(&path) {
                    self.problem(format!(
                        "ledger '{id}' archive {key} still uses retired path '{path}'"
                    ));
                }
            }
            ids.push(id);
            expected_links.push(scalar(field(Some(ledger), "source")));
            expected_links.push(scalar(field(Some(archive), "index")));
            expected_links.push(scalar(field(Some(archive), "manifest")));
        }
        if landing_seen.len() != 1 {
            self.problem("migrated rolling ledgers must share exactly one archive landing");
        }
        let Some(landing_rel) = landing_seen.iter().next().cloned() else {
            return;
        };
        if !safe_relative(&landing_rel) || !self.absolute(&landing_rel).is_file() {
            return;
        }
        if let Some(bytes) = self.slurp(
            &self.absolute(&landing_rel),
            "rolling-ledger archive landing",
        ) {
            self.validate_archive_landing_content(&bytes, &landing_rel, &ids, &expected_links);
        }
    }

    fn validate_archive_landing_content(
        &mut self,
        bytes: &[u8],
        landing_rel: &str,
        ids: &[String],
        expected_links: &[String],
    ) {
        let mut headings = Vec::new();
        for line in raw_lines(bytes) {
            let line = chomp_line(line);
            if line.starts_with(b"## `")
                && let Some(close) = line[4..].iter().position(|byte| *byte == b'`')
            {
                let close = close + 4;
                if line[close + 1..]
                    .iter()
                    .all(|byte| byte.is_ascii_whitespace())
                {
                    headings.push(String::from_utf8_lossy(&line[4..close]).into_owned());
                }
            }
        }
        if headings != ids {
            self.problem(
                "rolling-ledger archive landing ledger order/membership differs from the registry",
            );
        }
        let links = markdown_link_paths(bytes, landing_rel);
        let mut counts = BTreeMap::new();
        for path in &links {
            *counts.entry(path.clone()).or_insert(0) += 1;
        }
        let allowed: BTreeSet<String> = expected_links.iter().cloned().collect();
        for path in expected_links {
            if counts.get(path).copied().unwrap_or(0) != 1 {
                self.problem(format!(
                    "rolling-ledger archive landing must link '{path}' exactly once"
                ));
            }
        }
        for path in links {
            if !allowed.contains(&path) {
                self.problem(format!(
                    "rolling-ledger archive landing links undeclared route '{path}'"
                ));
            }
        }
    }
}

fn archive_index_pattern(pattern: &str, path: &str) -> bool {
    fn visit(pattern: &[u8], path: &[u8], p: usize, s: usize) -> bool {
        if p == pattern.len() {
            return s == path.len();
        }
        match pattern[p] {
            b'*' if pattern.get(p + 1) == Some(&b'*') => {
                visit(pattern, path, p + 2, s) || (s < path.len() && visit(pattern, path, p, s + 1))
            }
            b'*' => {
                visit(pattern, path, p + 1, s)
                    || (s < path.len() && path[s] != b'/' && visit(pattern, path, p, s + 1))
            }
            b'?' => s < path.len() && path[s] != b'/' && visit(pattern, path, p + 1, s + 1),
            literal => s < path.len() && path[s] == literal && visit(pattern, path, p + 1, s + 1),
        }
    }
    visit(pattern.as_bytes(), path.as_bytes(), 0, 0)
}

fn markdown_link_paths(bytes: &[u8], document_rel: &str) -> Vec<String> {
    let base = dirname(document_rel);
    let mut paths = Vec::new();
    let mut position = 0;
    while position + 1 < bytes.len() {
        let Some(open_offset) = bytes[position..].windows(2).position(|pair| pair == b"](") else {
            break;
        };
        let open = position + open_offset + 2;
        let Some(close_offset) = bytes[open..].iter().position(|byte| *byte == b')') else {
            break;
        };
        let close = open + close_offset;
        position = close + 1;
        let mut href = &bytes[open..close];
        if href.starts_with(b"<") {
            href = &href[1..];
        }
        if href.ends_with(b">") {
            href = &href[..href.len() - 1];
        }
        if let Some(fragment) = href.iter().position(|byte| *byte == b'#') {
            href = &href[..fragment];
        }
        if href.is_empty() || href.starts_with(b"/") || external_href(href) {
            continue;
        }
        let href = String::from_utf8_lossy(href).into_owned();
        if let Some(path) = normalize_relative_link(&base, &href) {
            paths.push(path);
        }
    }
    paths
}

fn external_href(href: &[u8]) -> bool {
    let Some(colon) = href.iter().position(|byte| *byte == b':') else {
        return false;
    };
    colon > 0
        && href[0].is_ascii_alphabetic()
        && href[..colon].iter().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_uppercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'+' | b'.' | b'-')
        })
}

fn normalize_relative_link(base: &str, href: &str) -> Option<String> {
    if href
        .bytes()
        .any(|byte| matches!(byte, 0 | b'\r' | b'\n' | b'?'))
    {
        return None;
    }
    let combined = format!("{base}/{href}");
    let mut parts = Vec::new();
    for part in combined.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            parts.pop()?;
        } else {
            parts.push(part);
        }
    }
    Some(parts.join("/"))
}

fn dirname(path: &str) -> String {
    match path.rfind('/') {
        None => ".".to_string(),
        Some(0) => "/".to_string(),
        Some(index) => path[..index].to_string(),
    }
}

fn relative_markdown_path(document_rel: &str, target_rel: &str) -> String {
    let base = dirname(document_rel);
    let base: Vec<&str> = base
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect();
    let target: Vec<&str> = target_rel
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect();
    let mut common = 0;
    while common < base.len() && common < target.len() && base[common] == target[common] {
        common += 1;
    }
    let mut parts = vec![".."; base.len() - common];
    parts.extend_from_slice(&target[common..]);
    if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

impl State {
    fn prepare_rollover_entry(
        &mut self,
        control: &Py,
        row: &Py,
        ledger: &Py,
    ) -> Option<PreparedEntry> {
        let id = scalar(field(Some(row), "ledger_id"));
        let source = scalar(field(Some(ledger), "source"));
        if scalar(field(Some(row), "source")) != source {
            self.problem(format!(
                "rollover plan ledger '{id}' source differs from registry"
            ));
        }
        let Some(archive @ Py::Dict(_)) = field(Some(ledger), "archive") else {
            return None;
        };
        let directory = scalar(field(Some(archive), "directory"));
        let segment_path = scalar(field(Some(row), "segment_path"));
        if directory.is_empty() || !segment_path.starts_with(&format!("{directory}/")) {
            self.problem(format!(
                "rollover plan ledger '{id}' segment path is outside '{directory}/'"
            ));
        }
        let segment_id = scalar(field(Some(row), "segment_id"));
        let sequence = segment_id
            .len()
            .checked_sub(5)
            .and_then(|start| segment_id.get(start..))
            .and_then(|suffix| suffix.strip_prefix('-'))
            .filter(|suffix| suffix.len() == 4 && suffix.bytes().all(|byte| byte.is_ascii_digit()));
        let sequence_ok = sequence.is_some_and(|sequence| {
            let name = format!("segment-{sequence}-");
            segment_path.rsplit('/').next().is_some_and(|filename| {
                filename
                    .strip_prefix(&name)
                    .and_then(|date| date.strip_suffix(".md"))
                    .is_some_and(|date| {
                        date.len() == 10
                            && date.bytes().enumerate().all(|(index, byte)| {
                                if matches!(index, 4 | 7) {
                                    byte == b'-'
                                } else {
                                    byte.is_ascii_digit()
                                }
                            })
                    })
            })
        });
        if !sequence_ok {
            self.problem(format!(
                "rollover plan ledger '{id}' segment id/path sequence is inconsistent"
            ));
        }
        if safe_relative(&segment_path) && self.absolute(&segment_path).exists() {
            self.problem(format!(
                "rollover plan ledger '{id}' segment destination already exists"
            ));
        }
        let live_bytes = self.slurp(
            &self.absolute(&source),
            &format!("ledger '{id}' rollover source"),
        )?;
        let grammar = field(Some(ledger), "grammar")?;
        let parsed = self.parse_ledger(&live_bytes, grammar, &format!("{id} rollover source"))?;
        let records = &parsed.records;
        let opening_records = perl::numeric(field(Some(row), "opening_records")) as usize;
        if opening_records > records.len() {
            self.problem(format!(
                "rollover plan ledger '{id}' has fewer records than its opening boundary"
            ));
            return None;
        }
        let future_prepends = records.len() - opening_records;
        let opening = records[future_prepends..].to_vec();
        let opening_bytes = render_live_view(&parsed, &opening, grammar);
        if sha256_hex(&opening_bytes) != scalar(field(Some(row), "opening_sha256")) {
            self.problem(format!(
                "rollover plan ledger '{id}' opening source identity drift"
            ));
        }
        let boundary = scalar(field(Some(control), "boundary_commit"));
        if boundary.len() == 40
            && boundary
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            && let Some(committed) = self.git_blob_bytes(&boundary, &source, &id)
            && committed != opening_bytes
        {
            self.problem(format!(
                "rollover plan ledger '{id}' committed opening blob differs from reconstructed boundary"
            ));
        }
        if perl::numeric(field(Some(row), "post_migration_records"))
            + perl::numeric(field(Some(row), "retained_migration_suffix_records"))
            != opening_records as f64
        {
            self.problem(format!(
                "rollover plan ledger '{id}' opening record decomposition drift"
            ));
        }
        if perl::numeric(field(Some(row), "keep_opening_prefix_records"))
            + perl::numeric(field(Some(row), "records"))
            != perl::numeric(field(Some(row), "post_migration_records"))
        {
            self.problem(format!(
                "rollover plan ledger '{id}' cut does not consume the oldest opening post-migration records"
            ));
        }
        let keep = perl::numeric(field(Some(row), "keep_opening_prefix_records")) as usize;
        let cut = perl::numeric(field(Some(row), "records")) as usize;
        if cut < 1 || keep.saturating_add(cut) > opening.len() {
            self.problem(format!("rollover plan ledger '{id}' cut range is invalid"));
            return None;
        }
        let segment_records = opening[keep..keep + cut].to_vec();
        let mut segment_bytes: Vec<u8> = segment_records
            .iter()
            .flat_map(|record| record.bytes.clone())
            .collect();
        trim_successor_separator(&mut segment_bytes);
        let mut segment_metrics = metrics(&segment_bytes);
        segment_metrics.records = segment_records.len();
        self.validate_rollover_metrics(
            Some(row),
            segment_metrics,
            &sha256_hex(&segment_bytes),
            &format!("rollover plan ledger '{id}' segment"),
        );
        if let Some(segment_parsed) =
            self.parse_segment(&segment_bytes, grammar, &format!("{id} planned segment"))
        {
            let first = segment_parsed
                .first()
                .map(|record| sha256_hex(&record.bytes))
                .unwrap_or_default();
            let last = segment_parsed
                .last()
                .map(|record| sha256_hex(&record.bytes))
                .unwrap_or_default();
            if first != scalar(field(Some(row), "first_record_sha256")) {
                self.problem(format!(
                    "rollover plan ledger '{id}' segment first-record identity drift: actual {first}, expected {}",
                    defined_scalar(field(Some(row), "first_record_sha256"))
                        .unwrap_or_else(|| "<missing>".to_string())
                ));
            }
            if last != scalar(field(Some(row), "last_record_sha256")) {
                self.problem(format!(
                    "rollover plan ledger '{id}' segment last-record identity drift: actual {last}, expected {}",
                    defined_scalar(field(Some(row), "last_record_sha256"))
                        .unwrap_or_else(|| "<missing>".to_string())
                ));
            }
        }
        let mut opening_survivors = opening[..keep].to_vec();
        opening_survivors.extend_from_slice(&opening[keep + cut..]);
        let opening_root_bytes = render_live_view(&parsed, &opening_survivors, grammar);
        let mut opening_root_metrics = metrics(&opening_root_bytes);
        opening_root_metrics.records = opening_survivors.len();
        self.validate_rollover_metrics(
            field(Some(row), "opening_root_after_cut"),
            opening_root_metrics,
            &sha256_hex(&opening_root_bytes),
            &format!("rollover plan ledger '{id}' opening root after cut"),
        );
        let start = future_prepends + keep;
        let mut current_survivors = records[..start].to_vec();
        current_survivors.extend_from_slice(&records[start + cut..]);
        let root_bytes = render_live_view(&parsed, &current_survivors, grammar);
        let mut root_metrics = metrics(&root_bytes);
        root_metrics.records = current_survivors.len();
        let surface = self.surface_by_source.get(&source).cloned();
        self.validate_warning_safe_survivor(
            field(Some(ledger), "live_limits"),
            surface.as_ref(),
            root_metrics,
            &id,
        );
        let manifest = self.prepare_rollover_manifest(control, row, ledger, segment_metrics)?;
        let index_rel = scalar(field(Some(archive), "index"));
        let manifest_rel = scalar(field(Some(archive), "manifest"));
        let index_bytes = render_archive_index(
            &id,
            &index_rel,
            &manifest_rel,
            &manifest.chain,
            &manifest.segments,
        );
        self.validate_archive_index_content(
            &index_bytes,
            &index_rel,
            &id,
            &manifest_rel,
            &manifest.chain,
        );
        Some(PreparedEntry {
            id,
            source,
            segment_path,
            segment_bytes,
            segment_metrics,
            root_bytes,
            root_metrics,
            manifest_path: manifest_rel,
            manifest_bytes: manifest.bytes,
            index_path: index_rel,
            index_bytes,
            future_prepends,
        })
    }

    fn git_blob_bytes(&mut self, commit: &str, relative: &str, id: &str) -> Option<Vec<u8>> {
        let process = match Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .arg("show")
            .arg(format!("{commit}:{relative}"))
            .output()
        {
            Ok(process) => process,
            Err(_) => {
                self.problem(format!(
                    "rollover plan ledger '{id}' cannot open committed boundary blob"
                ));
                return None;
            }
        };
        if !process.status.success() {
            self.problem(format!(
                "rollover plan ledger '{id}' cannot read '{relative}' at boundary '{commit}'"
            ));
            return None;
        }
        Some(process.stdout)
    }

    fn prepare_rollover_manifest(
        &mut self,
        plan_control: &Py,
        row: &Py,
        ledger: &Py,
        segment_metrics: Metrics,
    ) -> Option<PreparedManifest> {
        let id = scalar(field(Some(row), "ledger_id"));
        let archive = field(Some(ledger), "archive")?;
        let manifest_rel = scalar(field(Some(archive), "manifest"));
        let raw = match fs::read(self.absolute(&manifest_rel)) {
            Ok(raw) => raw,
            Err(_) => {
                self.problem(format!(
                    "rollover plan ledger '{id}' cannot read manifest '{manifest_rel}'"
                ));
                return None;
            }
        };
        let mut raw_records = Vec::new();
        let mut objects = Vec::new();
        for physical in raw_lines(&raw) {
            let line = chomp_line(physical);
            if line.is_empty() {
                continue;
            }
            match decode_json(line, 773) {
                Ok(value @ Py::Dict(_)) => {
                    raw_records.push(line.to_vec());
                    objects.push(value);
                }
                _ => {
                    self.problem(format!(
                        "rollover plan ledger '{id}' manifest contains invalid JSON"
                    ));
                    return None;
                }
            }
        }
        if objects.is_empty() {
            self.problem(format!(
                "rollover plan ledger '{id}' cannot read manifest '{manifest_rel}'"
            ));
            return None;
        }
        let control = objects.remove(0);
        let control_raw = raw_records.remove(0);
        let segments: Vec<Py> = objects
            .iter()
            .filter(|entry| scalar(field(Some(entry), "record_type")) == "sealed_segment")
            .cloned()
            .collect();
        let source = scalar(field(Some(ledger), "source"));
        let capsule = scalar(field(Some(archive), "source_capsule"));
        let prior_chain = self.validate_archive_chain(&id, &source, &capsule, &segments);
        let successor = scalar(field(Some(row), "successor"));
        if prior_chain.len() < 3 || prior_chain.get(1) != Some(&successor) {
            self.problem(format!(
                "rollover plan ledger '{id}' successor is not the current newest segment"
            ));
        }
        let successor_matches: Vec<&Py> = objects
            .iter()
            .filter(|entry| {
                scalar(field(Some(entry), "record_type")) == "sealed_segment"
                    && scalar(field(Some(entry), "path")) == successor
            })
            .collect();
        if successor_matches.len() != 1 {
            self.problem(format!(
                "rollover plan ledger '{id}' successor matches {} manifest rows",
                successor_matches.len()
            ));
        }
        if successor_matches.len() != 1 {
            return None;
        }
        if scalar(field(Some(successor_matches[0]), "predecessor")) != source {
            self.problem(format!(
                "rollover plan ledger '{id}' successor is not attached to the live root"
            ));
        }
        let mut updated_raw = Vec::new();
        let mut updated_objects = Vec::new();
        for (index, original) in objects.iter().enumerate() {
            let mut value = original.clone();
            if scalar(field(Some(&value), "record_type")) == "sealed_segment"
                && scalar(field(Some(&value), "path")) == successor
            {
                set_field(
                    &mut value,
                    "predecessor",
                    text(&scalar(field(Some(row), "segment_path"))),
                );
                match canonical(&value) {
                    Ok(encoded) => updated_raw.push(encoded.into_bytes()),
                    Err(error) => {
                        self.problem(error);
                        return None;
                    }
                }
            } else {
                updated_raw.push(raw_records[index].clone());
            }
            updated_objects.push(value);
        }
        let new_entry = object(vec![
            ("record_type", text("sealed_segment")),
            ("ledger_id", text(&id)),
            (
                "segment_id",
                field(Some(row), "segment_id").cloned().unwrap_or(Py::None),
            ),
            (
                "path",
                field(Some(row), "segment_path")
                    .cloned()
                    .unwrap_or(Py::None),
            ),
            (
                "sealed_on",
                field(Some(plan_control), "sealed_on")
                    .cloned()
                    .unwrap_or(Py::None),
            ),
            (
                "reason",
                field(Some(row), "reason").cloned().unwrap_or(Py::None),
            ),
            (
                "sha256",
                field(Some(row), "sha256").cloned().unwrap_or(Py::None),
            ),
            ("records", integer(segment_metrics.records)),
            ("lines", integer(segment_metrics.lines)),
            ("bytes", integer(segment_metrics.bytes)),
            ("line_bytes", integer(segment_metrics.line_bytes)),
            (
                "first_record_sha256",
                field(Some(row), "first_record_sha256")
                    .cloned()
                    .unwrap_or(Py::None),
            ),
            (
                "last_record_sha256",
                field(Some(row), "last_record_sha256")
                    .cloned()
                    .unwrap_or(Py::None),
            ),
            ("verifier", text(VERIFIER)),
            ("predecessor", text(&source)),
            ("successor", text(&successor)),
        ]);
        match canonical(&new_entry) {
            Ok(encoded) => updated_raw.push(encoded.into_bytes()),
            Err(error) => {
                self.problem(error);
                return None;
            }
        }
        updated_objects.push(new_entry);
        let mut manifest_bytes = control_raw.clone();
        for record in &updated_raw {
            manifest_bytes.push(b'\n');
            manifest_bytes.extend_from_slice(record);
        }
        manifest_bytes.push(b'\n');
        let lengths: Vec<usize> = std::iter::once(control_raw.len())
            .chain(updated_raw.iter().map(Vec::len))
            .collect();
        self.validate_archive_manifest_limits(
            &control,
            manifest_bytes.len(),
            &updated_objects,
            &lengths,
            &id,
        );
        if field(Some(&control), "max_scalar_bytes").is_some_and(|value| !matches!(value, Py::None))
        {
            for entry in &updated_objects {
                self.validate_value_bounds(
                    entry,
                    "archive manifest record",
                    1.0,
                    perl::numeric(field(Some(&control), "max_scalar_bytes")),
                );
            }
        }
        let updated_segments: Vec<Py> = updated_objects
            .iter()
            .filter(|entry| scalar(field(Some(entry), "record_type")) == "sealed_segment")
            .cloned()
            .collect();
        let chain = self.validate_archive_chain(&id, &source, &capsule, &updated_segments);
        Some(PreparedManifest {
            bytes: manifest_bytes,
            chain,
            segments: updated_segments,
        })
    }

    fn run_rollover_transaction(
        &mut self,
        plan: &RolloverPlan,
        ledgers: &[Py],
        meta: &Py,
        apply: bool,
        output: &mut dyn super::Output,
    ) {
        let ledger_by_id: BTreeMap<String, &Py> = ledgers
            .iter()
            .map(|ledger| (scalar(field(Some(ledger), "ledger_id")), ledger))
            .collect();
        let registry_order: Vec<String> = ledgers
            .iter()
            .map(|ledger| scalar(field(Some(ledger), "ledger_id")))
            .collect();
        let plan_order: Vec<String> = plan
            .rows
            .iter()
            .map(|row| scalar(field(Some(row), "ledger_id")))
            .collect();
        let expected_order: Vec<String> = registry_order
            .into_iter()
            .filter(|id| self.planned_rollover_ids.contains(id))
            .collect();
        if plan_order != expected_order {
            self.problem("rollover plan rows differ from registry order");
        }
        let mut prepared = Vec::new();
        for row in &plan.rows {
            let id = scalar(field(Some(row), "ledger_id"));
            let Some(ledger) = ledger_by_id.get(&id).copied() else {
                self.problem(format!("rollover plan names unknown ledger_id '{id}'"));
                continue;
            };
            if scalar(field(Some(ledger), "migration_state")) != "migrated" {
                self.problem(format!("rollover plan ledger '{id}' is not migrated"));
            }
            if let Some(entry) = self.prepare_rollover_entry(&plan.control, row, ledger) {
                prepared.push(entry);
            }
        }
        if !self.errors.is_empty() {
            return;
        }
        if !apply {
            for entry in &prepared {
                let mut segment = metrics_json(entry.segment_metrics);
                set_field(&mut segment, "path", text(&entry.segment_path));
                set_field(
                    &mut segment,
                    "sha256",
                    text(&sha256_hex(&entry.segment_bytes)),
                );
                let mut resulting_live = metrics_json(entry.root_metrics);
                set_field(
                    &mut resulting_live,
                    "sha256",
                    text(&sha256_hex(&entry.root_bytes)),
                );
                let report = object(vec![
                    ("ledger_id", text(&entry.id)),
                    ("segment", segment),
                    ("resulting_live", resulting_live),
                    ("future_prepends", integer(entry.future_prepends)),
                ]);
                match canonical(&report) {
                    Ok(encoded) => output.out(&format!("{encoded}\n")),
                    Err(error) => self.problem(error),
                }
            }
            if self.errors.is_empty() {
                output.out(&format!(
                    "rolling-ledger: rollover plan '{}' dry-run is exact and warning-safe for {} ledger(s).\n",
                    scalar(field(Some(&plan.control), "plan_id")),
                    prepared.len()
                ));
            }
            return;
        }
        let Some(context) = self.install_rollover_transaction(&prepared, None) else {
            return;
        };
        self.post_apply_validation = true;
        self.errors.clear();
        self.emissions.clear();
        for ledger in ledgers {
            let id = scalar(field(Some(ledger), "ledger_id"));
            self.validate_ledger(ledger, &id, output);
        }
        self.validate_archive_landings(meta, ledgers);
        self.validate_archive_index_cardinality(meta, ledgers);
        if !self.errors.is_empty() {
            let post_errors = std::mem::take(&mut self.errors);
            let rollback = self.rollback_rollover_transaction(&context);
            self.errors = post_errors;
            if let Err(error) = rollback {
                self.problem(format!("rollover transaction rollback failed: {error}"));
            }
            return;
        }
        if let Err(error) = cleanup_rollover_stage(&context.stage) {
            self.problem(error);
            return;
        }
        output.out(&format!(
            "rolling-ledger: applied warning-safe root-last rollover plan '{}' for {} ledger(s).\n",
            scalar(field(Some(&plan.control), "plan_id")),
            prepared.len()
        ));
    }

    fn install_rollover_transaction(
        &mut self,
        prepared: &[PreparedEntry],
        inject_failure: Option<&str>,
    ) -> Option<TransactionContext> {
        let stage_rel = format!(
            "generated/.rolling-ledger-transaction.{}",
            std::process::id()
        );
        let stage = self.absolute(&stage_rel);
        if stage.exists() {
            self.problem(format!(
                "rollover transaction stage already exists: {stage_rel}"
            ));
            return None;
        }
        if let Err(error) = fs::create_dir_all(&stage) {
            let _ = error;
            self.problem(format!(
                "cannot create rollover transaction stage '{stage_rel}'"
            ));
            return None;
        }
        let mut context = TransactionContext {
            stage: stage.clone(),
            backups: BTreeMap::new(),
            installed_segments: Vec::new(),
        };
        let mut operation = || -> Result<(), String> {
            let root_device = device(&self.root);
            let stage_device = device(&stage);
            if root_device.is_none() || stage_device.is_none() || root_device != stage_device {
                return Err("staging directory is not on the repository filesystem\n".to_string());
            }
            for entry in prepared {
                for (relative, bytes) in [
                    (&entry.segment_path, &entry.segment_bytes),
                    (&entry.manifest_path, &entry.manifest_bytes),
                    (&entry.index_path, &entry.index_bytes),
                    (&entry.source, &entry.root_bytes),
                ] {
                    let staged = stage.join(relative);
                    if let Some(parent) = staged.parent() {
                        fs::create_dir_all(parent).map_err(|error| {
                            format!(
                                "cannot create staging parent '{}': {}\n",
                                parent.display(),
                                os_error(&error)
                            )
                        })?;
                    }
                    write_new_file(&staged, bytes)?;
                    let staged_bytes = slurp_or_die(&staged)?;
                    if staged_bytes != *bytes {
                        return Err(format!("staged output identity drift for '{relative}'\n"));
                    }
                }
                for relative in [&entry.source, &entry.manifest_path, &entry.index_path] {
                    if !self.absolute(relative).is_file() {
                        return Err(format!(
                            "authority input '{relative}' is missing before transaction\n"
                        ));
                    }
                    let bytes = slurp_or_die(&self.absolute(relative))?;
                    context.backups.insert(relative.clone(), bytes);
                }
            }
            for entry in prepared {
                let destination = self.absolute(&entry.segment_path);
                write_new_file(&destination, &entry.segment_bytes)?;
                context.installed_segments.push(entry.segment_path.clone());
            }
            if inject_failure == Some("after_segments") {
                return Err("injected failure after segment installation\n".to_string());
            }
            for kind in ["manifest", "index", "root"] {
                for entry in prepared {
                    let (relative, bytes) = match kind {
                        "manifest" => (&entry.manifest_path, &entry.manifest_bytes),
                        "index" => (&entry.index_path, &entry.index_bytes),
                        _ => (&entry.source, &entry.root_bytes),
                    };
                    atomic_replace(&self.root, relative, bytes)?;
                }
            }
            Ok(())
        };
        if let Err(failure) = operation() {
            let rollback = self.rollback_rollover_transaction(&context);
            self.problem(format!("rollover transaction failed: {failure}"));
            if let Err(error) = rollback {
                self.problem(format!("rollover transaction rollback failed: {error}"));
            }
            return None;
        }
        Some(context)
    }

    fn rollback_rollover_transaction(
        &mut self,
        context: &TransactionContext,
    ) -> Result<(), String> {
        for (relative, bytes) in &context.backups {
            atomic_replace(&self.root, relative, bytes)?;
        }
        for relative in context.installed_segments.iter().rev() {
            let path = self.absolute(relative);
            if path.exists() {
                fs::remove_file(&path).map_err(|error| {
                    format!(
                        "cannot remove rolled-back segment '{relative}': {}",
                        os_error(&error)
                    )
                })?;
            }
        }
        cleanup_rollover_stage(&context.stage)
    }

    fn emit_planned_view(
        &mut self,
        emit_id: &str,
        output_rel: Option<&str>,
        report: bool,
        self_test: bool,
        output: &mut dyn super::Output,
    ) -> Result<(), String> {
        if report || self_test {
            self.problem("--emit-planned cannot be combined with --report or --self-test");
        }
        if output_rel.is_none() {
            self.problem("--emit-planned requires --output");
        }
        if !self.emissions.contains_key(emit_id) {
            self.problem(format!("emit ledger_id '{emit_id}' is unknown"));
        }
        if !self.errors.is_empty() {
            return Ok(());
        }
        let emission = self
            .emissions
            .get(emit_id)
            .cloned()
            .expect("checked emission");
        if emission.state != "planned" {
            self.problem(format!("ledger '{emit_id}' cannot emit after migration"));
        }
        let output_rel = output_rel.unwrap_or_default();
        if output_rel != emission.source {
            self.problem(format!(
                "ledger '{emit_id}' output must be its declared source '{}'",
                emission.source
            ));
        }
        if !safe_relative(output_rel) {
            self.problem(format!(
                "ledger '{emit_id}' output '{output_rel}' is unsafe"
            ));
        }
        if !safe_relative(&emission.capsule) || !self.absolute(&emission.capsule).is_file() {
            self.problem(format!(
                "ledger '{emit_id}' exact source capsule must exist before live emission"
            ));
        } else if let Some(capsule_bytes) = self.slurp(
            &self.absolute(&emission.capsule),
            &format!("ledger '{emit_id}' source capsule"),
        ) && sha256_hex(&capsule_bytes) != emission.source_sha256
        {
            self.problem(format!(
                "ledger '{emit_id}' source capsule does not match the pinned source identity"
            ));
        }
        if !self.errors.is_empty() {
            return Ok(());
        }
        let destination = self.absolute(output_rel);
        let temporary = PathBuf::from(format!(
            "{}.rolling-ledger-tmp.{}",
            destination.display(),
            std::process::id()
        ));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| {
                format!(
                    "rolling-ledger: cannot create same-directory temporary '{}': {}\n",
                    temporary.display(),
                    os_error(&error)
                )
            })?;
        if let Err(error) = file.write_all(&emission.bytes) {
            return Err(format!(
                "rolling-ledger: cannot write same-directory temporary '{}': {}\n",
                temporary.display(),
                os_error(&error)
            ));
        }
        file.sync_all().map_err(|error| {
            format!(
                "rolling-ledger: cannot close same-directory temporary '{}': {}\n",
                temporary.display(),
                os_error(&error)
            )
        })?;
        drop(file);
        fs::rename(&temporary, &destination).map_err(|error| {
            format!(
                "rolling-ledger: cannot atomically replace '{output_rel}': {}\n",
                os_error(&error)
            )
        })?;
        output.out(&format!(
            "rolling-ledger: wrote planned whole-record view for '{emit_id}' to '{output_rel}'.\n"
        ));
        Ok(())
    }
}

fn render_archive_index(
    id: &str,
    index_rel: &str,
    manifest_rel: &str,
    chain: &[String],
    segments: &[Py],
) -> Vec<u8> {
    let segment_ids: BTreeMap<String, String> = segments
        .iter()
        .map(|segment| {
            (
                scalar(field(Some(segment), "path")),
                scalar(field(Some(segment), "segment_id")),
            )
        })
        .collect();
    let mut lines = vec![
        format!("# `{id}` Rolling-Ledger Archive"),
        String::new(),
        "Complete newest-to-oldest chronology:".to_string(),
        String::new(),
    ];
    for (index, path) in chain.iter().enumerate() {
        let label = if index == 0 {
            "Current root".to_string()
        } else if index + 1 == chain.len() {
            "Immutable source capsule".to_string()
        } else {
            format!(
                "Sealed segment `{}`",
                segment_ids.get(path).cloned().unwrap_or_default()
            )
        };
        lines.push(format!(
            "{}. [{label}]({})",
            index + 1,
            relative_markdown_path(index_rel, path)
        ));
    }
    lines.push(String::new());
    lines.push(format!(
        "Authority: [bounded JSONL manifest]({}).",
        relative_markdown_path(index_rel, manifest_rel)
    ));
    format!("{}\n", lines.join("\n")).into_bytes()
}

#[cfg(unix)]
fn device(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).ok().map(|metadata| metadata.dev())
}

#[cfg(not(unix))]
fn device(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|_| 0)
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            format!(
                "cannot create '{}' exclusively: {}\n",
                path.display(),
                os_error(&error)
            )
        })?;
    if let Err(error) = file.write_all(bytes) {
        let failure = os_error(&error);
        drop(file);
        let _ = fs::remove_file(path);
        return Err(format!("cannot write '{}': {failure}\n", path.display()));
    }
    if let Err(error) = file.sync_all() {
        let failure = os_error(&error);
        drop(file);
        let _ = fs::remove_file(path);
        return Err(format!("cannot close '{}': {failure}\n", path.display()));
    }
    drop(file);
    Ok(())
}

fn slurp_or_die(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path)
        .map_err(|error| format!("cannot read '{}': {}\n", path.display(), os_error(&error)))
}

fn atomic_replace(root: &Path, relative: &str, bytes: &[u8]) -> Result<(), String> {
    let destination = root.join(relative);
    let temporary = PathBuf::from(format!(
        "{}.rolling-ledger-tmp.{}",
        destination.display(),
        std::process::id()
    ));
    write_new_file(&temporary, bytes)?;
    if let Err(error) = fs::rename(&temporary, &destination) {
        let failure = os_error(&error);
        let _ = fs::remove_file(&temporary);
        return Err(format!(
            "cannot atomically replace '{relative}': {failure}\n"
        ));
    }
    Ok(())
}

fn cleanup_rollover_stage(stage: &Path) -> Result<(), String> {
    if stage.exists() {
        fs::remove_dir_all(stage).map_err(|error| {
            format!(
                "cannot remove rollover transaction stage '{}': {}\n",
                stage.display(),
                os_error(&error)
            )
        })?;
    }
    if stage.exists() {
        return Err(format!(
            "rollover transaction stage survived cleanup: '{}'\n",
            stage.display()
        ));
    }
    Ok(())
}

/// Run the rolling-ledger protocol checker with its original arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut requested_root = None;
    let mut registry_rel = "doctrine/live_document_size/rolling_ledgers.jsonl".to_string();
    let mut surfaces_rel = "doctrine/live_document_size/surfaces.jsonl".to_string();
    let mut report = false;
    let mut self_test = false;
    let mut emit_id = None;
    let mut output_rel = None;
    let mut rollover_plan_rel = None;
    let mut apply_rollover = false;
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
            "--surfaces" => match arguments.next() {
                Some(path) => surfaces_rel = path.to_string(),
                None => return usage(output),
            },
            "--report" => report = true,
            "--self-test" => self_test = true,
            "--emit-planned" => match arguments.next() {
                Some(id) => emit_id = Some(id.to_string()),
                None => return usage(output),
            },
            "--output" => match arguments.next() {
                Some(path) => output_rel = Some(path.to_string()),
                None => return usage(output),
            },
            "--rollover-plan" => match arguments.next() {
                Some(path) => rollover_plan_rel = Some(path.to_string()),
                None => return usage(output),
            },
            "--apply-rollover" => apply_rollover = true,
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
                    output.err("rolling-ledger: repository root does not exist\n");
                    return 2;
                }
            }
        }
        None => root.to_path_buf(),
    };
    if apply_rollover && rollover_plan_rel.is_none()
        || rollover_plan_rel.is_some()
            && (report || self_test || emit_id.is_some() || output_rel.is_some())
    {
        return usage(output);
    }
    let mut state = State::new(root);
    state.registry_rel = registry_rel;
    state.surfaces_rel = surfaces_rel;
    state.report = report;
    if self_test {
        return match run_self_test(&mut state) {
            Ok(()) => {
                output.out("rolling-ledger: 45/45 parser/control self-tests pass.\n");
                0
            }
            Err(failures) => {
                for failure in failures {
                    output.err(&format!("rolling-ledger self-test: {failure}\n"));
                }
                255
            }
        };
    }
    let registry = state.read_registry();
    let rollover_plan = rollover_plan_rel
        .as_deref()
        .map(|relative| state.read_rollover_plan(relative));
    if let Some(plan) = &rollover_plan {
        state.planned_rollover_ids = plan
            .rows
            .iter()
            .map(|row| scalar(field(Some(row), "ledger_id")))
            .collect();
    }
    state.read_surface_authority();
    let mut seen = BTreeSet::new();
    for ledger in &registry.ledgers {
        state.validate_ledger_schema(ledger);
        let Some(id) = state.scalar_field(Some(ledger), "ledger_id", "ledger record") else {
            continue;
        };
        if !seen.insert(id.clone()) {
            state.problem(format!("duplicate ledger_id '{id}'"));
        }
        if !valid_identifier(&id) {
            state.problem(format!("ledger_id '{id}' has an invalid identifier shape"));
        }
        state.validate_ledger(ledger, &id, output);
    }
    state.validate_archive_landings(&registry.meta, &registry.ledgers);
    state.validate_archive_index_cardinality(&registry.meta, &registry.ledgers);
    if let Some(id) = emit_id.as_deref()
        && state.errors.is_empty()
        && let Err(error) =
            state.emit_planned_view(id, output_rel.as_deref(), report, self_test, output)
    {
        output.err(&error);
        return 255;
    }
    if let Some(plan) = rollover_plan.as_ref()
        && state.errors.is_empty()
    {
        state.run_rollover_transaction(
            plan,
            &registry.ledgers,
            &registry.meta,
            apply_rollover,
            output,
        );
    }
    for notice in &state.notices {
        output.out(&format!("rolling-ledger: warning: {notice}\n"));
    }
    if !state.errors.is_empty() {
        for error in &state.errors {
            output.err(&format!("rolling-ledger: {error}\n"));
        }
        output.err(&format!(
            "rolling-ledger: FAILED with {} violation(s).\n",
            state.errors.len()
        ));
        return 1;
    }
    if !report {
        output.out(&format!(
            "rolling-ledger: {} ledgers satisfy the lossless live-window/archive protocol.\n",
            registry.ledgers.len()
        ));
    }
    0
}

fn usage(output: &mut dyn super::Output) -> i32 {
    let program = std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "cargo xtask check rolling-ledger-protocol".to_string());
    output.err(&format!(
        "Usage: {program} [--root DIR] [--registry PATH] [--surfaces PATH] [--report] [--self-test] [--emit-planned LEDGER_ID --output SOURCE_PATH] [--rollover-plan PATH [--apply-rollover]]\n"
    ));
    2
}

fn self_check(condition: bool, message: &str, checks: &mut usize, failures: &mut Vec<String>) {
    *checks += 1;
    if !condition {
        failures.push(message.to_string());
    }
}

fn grammar(kind: &str) -> Py {
    let mut value = object(vec![
        ("kind", text(kind)),
        ("skipped_h2", integer(0)),
        ("detached_titles", Py::List(Vec::new())),
        ("trailer_markers", Py::List(Vec::new())),
    ]);
    if kind == "current_snapshot_bullets_v1" {
        set_field(&mut value, "start_marker", text("## Current snapshot\n"));
        set_field(&mut value, "end_marker", text("## Gap\n"));
    }
    value
}

fn record(ordinal: usize, title: &str, bytes: &[u8]) -> Record {
    Record {
        ordinal,
        title: pyjson::py_str(title),
        bytes: bytes.to_vec(),
    }
}

fn limits() -> Py {
    object(vec![
        ("records", integer(20)),
        ("lines", integer(200)),
        ("bytes", integer(2_000)),
        ("line_bytes", integer(200)),
        ("warning_pct", integer(80)),
        ("rollover_pct", integer(90)),
    ])
}

fn health_surface() -> Py {
    object(vec![(
        "health_targets",
        object(vec![
            ("lines_each", integer(100)),
            ("bytes_each", integer(1_000)),
            ("line_bytes_each", integer(100)),
        ]),
    )])
}

fn fixture_segments() -> (String, String, Vec<Py>) {
    let source = "CURRENT.md".to_string();
    let capsule = "docs/archive/rolling-ledgers/fixture/capsule.md".to_string();
    let segments = vec![
        object(vec![
            (
                "path",
                text("docs/archive/rolling-ledgers/fixture/segment-0002.md"),
            ),
            ("predecessor", text(&source)),
            (
                "successor",
                text("docs/archive/rolling-ledgers/fixture/segment-0001.md"),
            ),
        ]),
        object(vec![
            (
                "path",
                text("docs/archive/rolling-ledgers/fixture/segment-0001.md"),
            ),
            (
                "predecessor",
                text("docs/archive/rolling-ledgers/fixture/segment-0002.md"),
            ),
            ("successor", text(&capsule)),
        ]),
    ];
    (source, capsule, segments)
}

fn run_self_test(state: &mut State) -> Result<(), Vec<String>> {
    let mut checks = 0;
    let mut failures = Vec::new();
    let cases = vec![
        (
            "changes",
            b"### new\nA\n## old\nB\n### detached\nC\n".as_slice(),
            {
                let mut grammar = grammar("changes_mixed_v1");
                set_field(
                    &mut grammar,
                    "detached_titles",
                    Py::List(vec![text("### detached")]),
                );
                grammar
            },
            3,
        ),
        (
            "development",
            b"# notes\n## recent\nA\n## old\nB\n".as_slice(),
            grammar("h2_records_v1"),
            2,
        ),
        (
            "analysis",
            b"# analysis\n## Purpose\nP\n## recent\nA\n## old\nB\n".as_slice(),
            {
                let mut grammar = grammar("h2_records_v1");
                set_field(&mut grammar, "skipped_h2", integer(1));
                grammar
            },
            2,
        ),
        (
            "snapshot",
            b"# status\n## Current snapshot\n- new\n- old\n## Gap\n<!-- start -->\n<!-- end -->\n"
                .as_slice(),
            {
                let mut grammar = grammar("current_snapshot_bullets_v1");
                set_field(
                    &mut grammar,
                    "trailer_markers",
                    Py::List(vec![text("<!-- start -->"), text("<!-- end -->")]),
                );
                grammar
            },
            2,
        ),
    ];
    for (name, bytes, grammar, count) in cases {
        let parsed = state.parse_ledger(bytes, &grammar, &format!("self-test {name}"));
        self_check(
            parsed.as_ref().is_some_and(|parsed| {
                parsed.records.len() == count && reconstruct(parsed) == bytes
            }),
            &format!("{name} parser, count, or reconstruction drifted"),
            &mut checks,
            &mut failures,
        );
    }
    state.errors.clear();
    let bad = state.parse_ledger(
        b"# status\n## Current snapshot\nnot a bullet\n## Gap\n",
        &grammar("current_snapshot_bullets_v1"),
        "self-test malformed snapshot",
    );
    self_check(
        bad.is_none() && !state.errors.is_empty(),
        "malformed snapshot was accepted or produced no diagnostic",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let samples = vec![
        record(1, "first", b"## first\n"),
        record(2, "second", b"## second\n"),
    ];
    let plan = object(vec![
        ("prefix_records", integer(2)),
        ("promote_titles", Py::List(Vec::new())),
    ]);
    let (_, selection_errors) =
        state.select_live_records(&samples, Some(&plan), "self-test full prefix");
    self_check(
        !selection_errors.is_empty(),
        "a live selection leaving no archive history was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let warning_limits = object(vec![
        ("records", integer(10)),
        ("lines", integer(10)),
        ("bytes", integer(100)),
        ("line_bytes", integer(10)),
        ("warning_pct", integer(80)),
        ("rollover_pct", integer(90)),
    ]);
    let warning_surface = object(vec![(
        "health_targets",
        object(vec![
            ("lines_each", integer(10)),
            ("bytes_each", integer(100)),
            ("line_bytes_each", integer(10)),
        ]),
    )]);
    state.validate_limits(
        Some(&warning_limits),
        Some(&warning_surface),
        Metrics {
            records: 8,
            lines: 1,
            bytes: 1,
            line_bytes: 1,
        },
        "self-test warning",
        "planned survivor",
    );
    self_check(
        !state.errors.is_empty(),
        "a planned survivor at its warning threshold was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.validate_retained_suffix(
        &[record(1, "changed", b"## changed\n")],
        &[record(1, "first", b"## first\n")],
        "self-test suffix",
    );
    self_check(
        !state.errors.is_empty(),
        "a changed retained record suffix was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let schema = object(vec![("known", integer(1)), ("surprise", integer(1))]);
    state.reject_unknown(Some(&schema), "self-test schema", &["known"]);
    self_check(
        !state.errors.is_empty(),
        "an unknown schema field was accepted",
        &mut checks,
        &mut failures,
    );
    self_check(
        !safe_relative("../archive.md"),
        "an escaping path was accepted",
        &mut checks,
        &mut failures,
    );

    let (source, capsule, segments) = fixture_segments();
    state.errors.clear();
    let chain = state.validate_archive_chain("fixture", &source, &capsule, &segments);
    let expected_chain: Vec<String> = std::iter::once(source.clone())
        .chain(
            segments
                .iter()
                .map(|segment| scalar(field(Some(segment), "path"))),
        )
        .chain(std::iter::once(capsule.clone()))
        .collect();
    self_check(
        state.errors.is_empty() && chain == expected_chain,
        "a valid archive chronology failed or its order drifted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let mut missing_edge = segments.clone();
    if let Py::Dict(pairs) = &mut missing_edge[0] {
        pairs.retain(|(key, _)| pyjson::to_text(key) != "predecessor");
    }
    state.validate_archive_chain("fixture", &source, &capsule, &missing_edge);
    self_check(
        !state.errors.is_empty(),
        "a missing predecessor edge was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let mut duplicate_edge = segments.clone();
    set_field(&mut duplicate_edge[1], "predecessor", text(&source));
    state.validate_archive_chain("fixture", &source, &capsule, &duplicate_edge);
    self_check(
        !state.errors.is_empty(),
        "a duplicate predecessor edge was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let mut broken_successor = segments.clone();
    set_field(&mut broken_successor[0], "successor", text(&capsule));
    state.validate_archive_chain("fixture", &source, &capsule, &broken_successor);
    self_check(
        !state.errors.is_empty(),
        "a broken successor edge was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let mut disconnected_cycle = segments.clone();
    let cycle_path = scalar(field(Some(&disconnected_cycle[1]), "path"));
    set_field(&mut disconnected_cycle[1], "predecessor", text(&cycle_path));
    set_field(&mut disconnected_cycle[1], "successor", text(&cycle_path));
    state.validate_archive_chain("fixture", &source, &capsule, &disconnected_cycle);
    self_check(
        !state.errors.is_empty(),
        "a disconnected chronology cycle was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.validate_manifest_ledger_membership(
        &object(vec![("ledger_id", text("foreign"))]),
        "fixture",
    );
    self_check(
        !state.errors.is_empty(),
        "a foreign-ledger manifest record was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.validate_archive_manifest_limits(
        &object(vec![
            ("max_records", integer(1)),
            ("max_bytes", integer(10)),
            ("max_record_bytes", integer(4)),
        ]),
        11,
        &[empty_object(), empty_object()],
        &[5, 1],
        "fixture",
    );
    self_check(
        state.errors.len() >= 3,
        "manifest record/byte limits were not enforced",
        &mut checks,
        &mut failures,
    );

    let index_rel = "docs/archive/rolling-ledgers/fixture/INDEX.md";
    let manifest_rel = "docs/archive/rolling-ledgers/fixture/manifest.jsonl";
    let valid_index = b"[Current](../../../../CURRENT.md)\n[New segment](segment-0002.md)\n[Old segment](segment-0001.md)\n[Capsule](capsule.md)\n[Manifest](manifest.jsonl)\n";
    state.errors.clear();
    state.validate_archive_index_content(valid_index, index_rel, "fixture", manifest_rel, &chain);
    self_check(
        state.errors.is_empty(),
        "a complete ordered archive index failed",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let missing_index_member = valid_index
        .windows(b"[Old segment](segment-0001.md)\n".len())
        .position(|window| window == b"[Old segment](segment-0001.md)\n")
        .map(|at| {
            let mut bytes = valid_index.to_vec();
            bytes.drain(at..at + b"[Old segment](segment-0001.md)\n".len());
            bytes
        })
        .unwrap_or_else(|| valid_index.to_vec());
    state.validate_archive_index_content(
        &missing_index_member,
        index_rel,
        "fixture",
        manifest_rel,
        &chain,
    );
    self_check(
        !state.errors.is_empty(),
        "an archive index missing a member was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let mut duplicate_index_member = valid_index.to_vec();
    duplicate_index_member.extend_from_slice(b"[Duplicate](segment-0001.md)\n");
    state.validate_archive_index_content(
        &duplicate_index_member,
        index_rel,
        "fixture",
        manifest_rel,
        &chain,
    );
    self_check(
        !state.errors.is_empty(),
        "an archive index with a duplicate member was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let wrong_index_order = b"[Current](../../../../CURRENT.md)\n[Old segment](segment-0001.md)\n[New segment](segment-0002.md)\n[Capsule](capsule.md)\n[Manifest](manifest.jsonl)\n";
    state.validate_archive_index_content(
        wrong_index_order,
        index_rel,
        "fixture",
        manifest_rel,
        &chain,
    );
    self_check(
        !state.errors.is_empty(),
        "an archive index with wrong chronology order was accepted",
        &mut checks,
        &mut failures,
    );

    let landing_rel = "docs/archive/rolling-ledgers/INDEX.md";
    let landing_links = vec![
        source.clone(),
        index_rel.to_string(),
        manifest_rel.to_string(),
    ];
    let valid_landing = b"## `fixture`\n[Current](../../../CURRENT.md)\n[Index](fixture/INDEX.md)\n[Manifest](fixture/manifest.jsonl)\n";
    state.errors.clear();
    state.validate_archive_landing_content(
        valid_landing,
        landing_rel,
        &["fixture".to_string()],
        &landing_links,
    );
    self_check(
        state.errors.is_empty(),
        "a complete archive landing failed",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let missing_landing_route =
        b"## `fixture`\n[Current](../../../CURRENT.md)\n[Index](fixture/INDEX.md)\n";
    state.validate_archive_landing_content(
        missing_landing_route,
        landing_rel,
        &["fixture".to_string()],
        &landing_links,
    );
    self_check(
        !state.errors.is_empty(),
        "an archive landing missing a route was accepted",
        &mut checks,
        &mut failures,
    );

    state.planned_rollover_ids.clear();
    state.post_apply_validation = false;
    let health = health_surface();
    state.errors.clear();
    state.validate_limits(
        Some(&limits()),
        Some(&health),
        Metrics {
            records: 1,
            lines: 90,
            bytes: 1,
            line_bytes: 1,
        },
        "self-test health denominator",
        "live source",
    );
    self_check(
        !state.errors.is_empty(),
        "live rollover still used the enforcement ceiling as its denominator",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.validate_warning_safe_survivor(
        Some(&limits()),
        Some(&health),
        Metrics {
            records: 15,
            lines: 80,
            bytes: 1,
            line_bytes: 1,
        },
        "self-test warning-safe survivor",
    );
    self_check(
        !state.errors.is_empty(),
        "a rollover survivor at a generic health warning was accepted",
        &mut checks,
        &mut failures,
    );

    let saved_surfaces = state.surface_by_source.clone();
    let binding_surface = object(vec![
        ("locator", text("file")),
        ("lifecycle", text("rolling_ledger")),
        ("state", text("normal")),
        ("verifier", text("builtin:budget")),
        ("archive_manifest", text("archive/manifest.jsonl")),
        (
            "health_targets",
            object(vec![
                ("lines_each", integer(100)),
                ("bytes_each", integer(1_000)),
                ("line_bytes_each", integer(100)),
            ]),
        ),
        (
            "enforcement_ceilings",
            object(vec![
                ("lines_each", integer(200)),
                ("bytes_each", integer(2_000)),
                ("line_bytes_each", integer(200)),
            ]),
        ),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
    ]);
    state.surface_by_source = BTreeMap::from([("CURRENT.md".to_string(), binding_surface)]);
    let binding_ledger = object(vec![
        ("live_limits", limits()),
        (
            "archive",
            object(vec![("manifest", text("archive/manifest.jsonl"))]),
        ),
    ]);
    state.errors.clear();
    state.validate_surface_binding(&binding_ledger, "fixture", "CURRENT.md");
    self_check(
        state.errors.is_empty(),
        "matching focused/generic surface authorities failed",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    if let Some(surface) = state.surface_by_source.get_mut("CURRENT.md")
        && let Some(ceilings) = field(Some(surface), "enforcement_ceilings")
    {
        let mut ceilings = ceilings.clone();
        set_field(&mut ceilings, "lines_each", integer(201));
        set_field(surface, "enforcement_ceilings", ceilings);
    }
    state.validate_surface_binding(&binding_ledger, "fixture", "CURRENT.md");
    self_check(
        !state.errors.is_empty(),
        "focused/generic ceiling drift was accepted",
        &mut checks,
        &mut failures,
    );
    state.surface_by_source = saved_surfaces;

    let (writer_checks, writer_failures) = run_rollover_writer_self_test(state);
    checks += writer_checks;
    failures.extend(writer_failures);

    state.errors.clear();
    let residue_rel = format!(
        "generated/rolling-ledger-retired-self-test-{}",
        std::process::id()
    );
    let residue_path = state.absolute(&residue_rel);
    let created = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&residue_path);
    match created {
        Ok(mut file) => {
            if let Err(error) = file.write_all(b"retired\n") {
                failures.push(format!("cannot write retired-path fixture: {error}"));
            }
            drop(file);
            state.validate_retired_paths(&object(vec![
                ("retired_paths", Py::List(vec![text(&residue_rel)])),
                ("max_array_items", integer(1)),
            ]));
            if let Err(error) = fs::remove_file(&residue_path) {
                failures.push(format!("cannot remove retired-path fixture: {error}"));
            }
            self_check(
                !state.errors.is_empty(),
                "a retired manifest residue was accepted",
                &mut checks,
                &mut failures,
            );
        }
        Err(error) => {
            failures.push(format!("cannot create retired-path fixture: {error}"));
            checks += 1;
        }
    }

    let budget_limits = object(vec![
        ("records", integer(10)),
        ("lines", integer(100)),
        ("bytes", integer(4_000)),
        ("line_bytes", integer(500)),
        ("warning_pct", integer(80)),
        ("rollover_pct", integer(90)),
    ]);
    let budget_surface = object(vec![(
        "health_targets",
        object(vec![
            ("lines_each", integer(100)),
            ("bytes_each", integer(2_000)),
            ("line_bytes_each", integer(500)),
        ]),
    )]);
    let budget_parsed = |sizes: &[usize], prologue: &[u8], trailer: &[u8]| {
        let records: Vec<Record> = sizes
            .iter()
            .enumerate()
            .map(|(index, size)| {
                let mut bytes = vec![b'r'; size.saturating_sub(1)];
                bytes.push(b'\n');
                record(index + 1, &format!("record {index}"), &bytes)
            })
            .collect();
        let parsed = Parsed {
            prologue: prologue.to_vec(),
            records,
            trailer: trailer.to_vec(),
        };
        let bytes = reconstruct(&parsed);
        (parsed, bytes)
    };
    let (compliant, compliant_bytes) = budget_parsed(&[100, 100, 100], b"", b"");
    state.errors.clear();
    state.notices.clear();
    let compliant_measure = state.measure_record_budget(
        &compliant,
        &compliant_bytes,
        Some(&budget_limits),
        Some(&budget_surface),
        "self-test budget",
    );
    state.validate_record_budget(compliant_measure.as_ref(), "self-test budget");
    self_check(
        perl::numeric(field(compliant_measure.as_ref(), "record_budget_bytes")) == 200.0
            && state.errors.is_empty()
            && state.notices.is_empty(),
        "a compliant record budget was not derived or produced a warning",
        &mut checks,
        &mut failures,
    );

    let (oversized, oversized_bytes) = budget_parsed(&[100, 100, 400], b"", b"");
    state.errors.clear();
    state.notices.clear();
    let measure = state.measure_record_budget(
        &oversized,
        &oversized_bytes,
        Some(&budget_limits),
        Some(&budget_surface),
        "self-test budget",
    );
    state.validate_record_budget(measure.as_ref(), "self-test budget");
    self_check(
        state.notices.iter().any(|notice| {
            notice.contains("above its derived 200-byte record budget")
                && notice.contains("record 3 at 400 bytes")
        }) && state.errors.is_empty(),
        "an oversized record was accepted or fatal",
        &mut checks,
        &mut failures,
    );

    let (unreachable, unreachable_bytes) = budget_parsed(&[300, 300, 300], b"", b"");
    state.errors.clear();
    state.notices.clear();
    let measure = state.measure_record_budget(
        &unreachable,
        &unreachable_bytes,
        Some(&budget_limits),
        Some(&budget_surface),
        "self-test budget",
    );
    state.validate_record_budget(measure.as_ref(), "self-test budget");
    self_check(
        state.notices.iter().any(|notice| {
            notice.contains("cannot reach its declared 10-record window")
                && notice.contains("holds 6 records")
        }),
        "an unreachable declared record window was accepted",
        &mut checks,
        &mut failures,
    );

    let (charged, charged_bytes) =
        budget_parsed(&[100, 100, 100], &vec![b'p'; 600], &vec![b't'; 600]);
    state.errors.clear();
    state.notices.clear();
    let charged_measure = state.measure_record_budget(
        &charged,
        &charged_bytes,
        Some(&budget_limits),
        Some(&budget_surface),
        "self-test budget",
    );
    state.validate_record_budget(charged_measure.as_ref(), "self-test budget");
    self_check(
        perl::numeric(field(charged_measure.as_ref(), "overhead_bytes")) == 1200.0
            && perl::numeric(field(charged_measure.as_ref(), "record_budget_bytes")) == 80.0
            && state
                .notices
                .iter()
                .any(|notice| notice.contains("above its derived 80-byte record budget")),
        "the live view overhead was not charged to the record budget",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.notices.clear();
    let mut wider_limits = budget_limits.clone();
    set_field(&mut wider_limits, "records", integer(4));
    let wider = state.measure_record_budget(
        &compliant,
        &compliant_bytes,
        Some(&wider_limits),
        Some(&budget_surface),
        "self-test budget",
    );
    let leaner_surface = object(vec![(
        "health_targets",
        object(vec![
            ("lines_each", integer(100)),
            ("bytes_each", integer(1_000)),
            ("line_bytes_each", integer(500)),
        ]),
    )]);
    let leaner = state.measure_record_budget(
        &compliant,
        &compliant_bytes,
        Some(&budget_limits),
        Some(&leaner_surface),
        "self-test budget",
    );
    self_check(
        perl::numeric(field(wider.as_ref(), "record_budget_bytes")) == 500.0
            && perl::numeric(field(leaner.as_ref(), "record_budget_bytes")) == 100.0,
        "the record budget did not follow its declared inputs",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.notices.clear();
    let excessive_surface = object(vec![(
        "health_targets",
        object(vec![
            ("lines_each", integer(100)),
            ("bytes_each", integer(900)),
            ("line_bytes_each", integer(500)),
        ]),
    )]);
    state.measure_record_budget(
        &charged,
        &charged_bytes,
        Some(&budget_limits),
        Some(&excessive_surface),
        "self-test budget",
    );
    self_check(
        !state.errors.is_empty(),
        "a live view whose overhead exceeds its health target was accepted",
        &mut checks,
        &mut failures,
    );

    let saved_index_surface = state.archive_index_surface.clone();
    let index_bounds = |files: usize| {
        object(vec![
            ("surface_id", text(ARCHIVE_INDEX_SURFACE_ID)),
            (
                "targets",
                Py::List(vec![text("docs/archive/rolling-ledgers/*/INDEX.md")]),
            ),
            (
                "health_targets",
                object(vec![
                    ("files", integer(files)),
                    ("lines_each", integer(64)),
                    ("lines_total", integer(files * 64)),
                    ("bytes_each", integer(8_192)),
                    ("bytes_total", integer(files * 8_192)),
                    ("line_bytes_each", integer(512)),
                ]),
            ),
            (
                "enforcement_ceilings",
                object(vec![
                    ("files", integer(files)),
                    ("lines_each", integer(96)),
                    ("lines_total", integer(files * 96)),
                    ("bytes_each", integer(12_288)),
                    ("bytes_total", integer(files * 12_288)),
                    ("line_bytes_each", integer(1_024)),
                ]),
            ),
        ])
    };
    let index_ledgers = vec![object(vec![
        ("ledger_id", text("fixture")),
        ("migration_state", text("migrated")),
        (
            "archive",
            object(vec![(
                "index",
                text("docs/archive/rolling-ledgers/fixture/INDEX.md"),
            )]),
        ),
    ])];
    state.errors.clear();
    state.archive_index_surface = Some(index_bounds(8));
    state.validate_archive_index_cardinality(
        &object(vec![("max_records", integer(8))]),
        &index_ledgers,
    );
    self_check(
        state.errors.is_empty(),
        "the derived archive-index cardinality was rejected",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.archive_index_surface = Some(index_bounds(4));
    state.validate_archive_index_cardinality(
        &object(vec![("max_records", integer(8))]),
        &index_ledgers,
    );
    self_check(
        state
            .errors
            .iter()
            .any(|error| error.contains("must equal the ledger registry max_records 8")),
        "an archive-index bound below the ledger cap was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    let mut aggregate = index_bounds(8);
    let bytes_total = perl::numeric(field(
        field(Some(&aggregate), "enforcement_ceilings"),
        "bytes_total",
    )) as usize;
    if let Some(ceilings) = field(Some(&aggregate), "enforcement_ceilings") {
        let mut ceilings = ceilings.clone();
        set_field(&mut ceilings, "bytes_total", integer(bytes_total - 1));
        set_field(&mut aggregate, "enforcement_ceilings", ceilings);
    }
    state.archive_index_surface = Some(aggregate);
    state.validate_archive_index_cardinality(
        &object(vec![("max_records", integer(8))]),
        &index_ledgers,
    );
    self_check(
        state
            .errors
            .iter()
            .any(|error| error.contains("is not files x bytes_each")),
        "an archive-index aggregate below files x per-file was accepted",
        &mut checks,
        &mut failures,
    );

    state.errors.clear();
    state.archive_index_surface = Some(index_bounds(8));
    let stray = vec![object(vec![
        ("ledger_id", text("stray")),
        ("migration_state", text("migrated")),
        (
            "archive",
            object(vec![("index", text("docs/archive/elsewhere/INDEX.md"))]),
        ),
    ])];
    state.validate_archive_index_cardinality(&object(vec![("max_records", integer(8))]), &stray);
    self_check(
        state
            .errors
            .iter()
            .any(|error| error.contains("is outside the")),
        "an archive index outside the declared surface was accepted",
        &mut checks,
        &mut failures,
    );
    state.archive_index_surface = saved_index_surface;
    state.errors.clear();
    state.notices.clear();
    if checks != 45 {
        failures.push(format!(
            "ran {checks} checks, declaration expects 45; re-derive the declaration beside the suite"
        ));
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

fn run_rollover_writer_self_test(state: &mut State) -> (usize, Vec<String>) {
    let mut checks = 0;
    let mut failures = Vec::new();
    let saved_errors = std::mem::take(&mut state.errors);
    let saved_surfaces = state.surface_by_source.clone();
    let saved_plan_ids = state.planned_rollover_ids.clone();
    let saved_post_apply = state.post_apply_validation;
    let fixture_rel = format!(
        "generated/.rolling-ledger-writer-self-test-{}",
        std::process::id()
    );
    let fixture = state.absolute(&fixture_rel);
    let transaction_stage = state.absolute(&format!(
        "generated/.rolling-ledger-transaction.{}",
        std::process::id()
    ));
    let mut operation = || -> Result<(), String> {
        if fixture.exists() || transaction_stage.exists() {
            return Err("rollover writer self-test fixture already exists\n".to_string());
        }
        let archive_rel = format!("{fixture_rel}/archive");
        fs::create_dir_all(state.absolute(&archive_rel))
            .map_err(|error| format!("cannot create writer fixture: {error}\n"))?;
        let source_rel = format!("{fixture_rel}/CURRENT.md");
        let manifest_rel = format!("{archive_rel}/manifest.jsonl");
        let index_rel = format!("{archive_rel}/INDEX.md");
        let capsule_rel = format!("{archive_rel}/capsule.md");
        let old_segment_rel = format!("{archive_rel}/segment-0001-2026-08-09.md");
        let new_segment_rel = format!("{archive_rel}/segment-0002-2026-08-09.md");
        let grammar = grammar("h2_records_v1");
        let opening = b"# Fixture\n\n## newest\nA\n\n## oldest new\nB\n\n## retained\nC\n".to_vec();
        let current =
            b"# Fixture\n\n## future\nF\n\n## newest\nA\n\n## oldest new\nB\n\n## retained\nC\n"
                .to_vec();
        let opening_parsed = state
            .parse_ledger(&opening, &grammar, "rollover writer opening fixture")
            .ok_or_else(|| "cannot parse writer opening fixture\n".to_string())?;
        let mut segment_bytes = opening_parsed.records[1].bytes.clone();
        trim_successor_separator(&mut segment_bytes);
        let mut segment_metrics = metrics(&segment_bytes);
        segment_metrics.records = 1;
        let segment_parsed = state
            .parse_segment(&segment_bytes, &grammar, "rollover writer segment fixture")
            .ok_or_else(|| "cannot parse writer segment fixture\n".to_string())?;
        let opening_survivors = vec![
            opening_parsed.records[0].clone(),
            opening_parsed.records[2].clone(),
        ];
        let opening_root = render_live_view(&opening_parsed, &opening_survivors, &grammar);
        let mut opening_root_metrics = metrics(&opening_root);
        opening_root_metrics.records = 2;
        let manifest_control = object(vec![
            ("record_type", text("archive_manifest")),
            ("schema_version", integer(1)),
            ("max_records", integer(8)),
            ("max_bytes", integer(16_384)),
            ("max_record_bytes", integer(2_048)),
            ("max_scalar_bytes", integer(768)),
        ]);
        let capsule = object(vec![
            ("record_type", text("source_capsule")),
            ("ledger_id", text("fixture")),
            ("path", text(&capsule_rel)),
        ]);
        let old_segment = object(vec![
            ("record_type", text("sealed_segment")),
            ("ledger_id", text("fixture")),
            ("segment_id", text("fixture-0001")),
            ("path", text(&old_segment_rel)),
            ("predecessor", text(&source_rel)),
            ("successor", text(&capsule_rel)),
        ]);
        let mut manifest_bytes = Vec::new();
        for value in [&manifest_control, &capsule, &old_segment] {
            manifest_bytes.extend_from_slice(
                canonical(value)
                    .map_err(|error| format!("{error}\n"))?
                    .as_bytes(),
            );
            manifest_bytes.push(b'\n');
        }
        let index_bytes = b"old index\n".to_vec();
        for (relative, bytes) in [
            (&source_rel, &current),
            (&manifest_rel, &manifest_bytes),
            (&index_rel, &index_bytes),
            (&capsule_rel, &opening),
            (&old_segment_rel, &b"## archived\nold\n".to_vec()),
        ] {
            write_new_file(&state.absolute(relative), bytes)?;
        }
        let writer_limits = object(vec![
            ("records", integer(16)),
            ("lines", integer(200)),
            ("bytes", integer(4_096)),
            ("line_bytes", integer(200)),
            ("warning_pct", integer(80)),
            ("rollover_pct", integer(90)),
        ]);
        state.surface_by_source = BTreeMap::from([(
            source_rel.clone(),
            object(vec![
                (
                    "health_targets",
                    object(vec![
                        ("lines_each", integer(100)),
                        ("bytes_each", integer(2_000)),
                        ("line_bytes_each", integer(100)),
                    ]),
                ),
                (
                    "enforcement_ceilings",
                    object(vec![
                        ("lines_each", integer(200)),
                        ("bytes_each", integer(4_096)),
                        ("line_bytes_each", integer(200)),
                    ]),
                ),
                (
                    "milestones",
                    object(vec![
                        ("warning_pct", integer(80)),
                        ("rollover_pct", integer(90)),
                    ]),
                ),
            ]),
        )]);
        state.planned_rollover_ids = BTreeSet::from(["fixture".to_string()]);
        state.post_apply_validation = false;
        let ledger = object(vec![
            ("ledger_id", text("fixture")),
            ("source", text(&source_rel)),
            ("grammar", grammar.clone()),
            ("live_limits", writer_limits),
            (
                "archive",
                object(vec![
                    ("directory", text(&archive_rel)),
                    ("manifest", text(&manifest_rel)),
                    ("index", text(&index_rel)),
                    ("source_capsule", text(&capsule_rel)),
                ]),
            ),
        ]);
        let control = object(vec![
            ("sealed_on", text("2026-08-09")),
            ("plan_id", text("fixture-plan")),
        ]);
        let row = object(vec![
            ("ledger_id", text("fixture")),
            ("source", text(&source_rel)),
            ("opening_sha256", text(&sha256_hex(&opening))),
            ("opening_records", integer(3)),
            ("post_migration_records", integer(2)),
            ("retained_migration_suffix_records", integer(1)),
            ("keep_opening_prefix_records", integer(1)),
            ("segment_id", text("fixture-0002")),
            ("segment_path", text(&new_segment_rel)),
            ("successor", text(&old_segment_rel)),
            ("records", integer(segment_metrics.records)),
            ("lines", integer(segment_metrics.lines)),
            ("bytes", integer(segment_metrics.bytes)),
            ("line_bytes", integer(segment_metrics.line_bytes)),
            ("sha256", text(&sha256_hex(&segment_bytes))),
            (
                "first_record_sha256",
                text(&sha256_hex(&segment_parsed[0].bytes)),
            ),
            (
                "last_record_sha256",
                text(&sha256_hex(&segment_parsed[segment_parsed.len() - 1].bytes)),
            ),
            (
                "opening_root_after_cut",
                object(vec![
                    ("records", integer(opening_root_metrics.records)),
                    ("lines", integer(opening_root_metrics.lines)),
                    ("bytes", integer(opening_root_metrics.bytes)),
                    ("line_bytes", integer(opening_root_metrics.line_bytes)),
                    ("sha256", text(&sha256_hex(&opening_root))),
                ]),
            ),
            ("reason", text("Fixture rollover transaction.")),
        ]);

        state.errors.clear();
        let prepared = state.prepare_rollover_entry(&control, &row, &ledger);
        self_check(
            state.errors.is_empty() && prepared.is_some(),
            "a valid rollover plan entry failed preparation",
            &mut checks,
            &mut failures,
        );

        state.errors.clear();
        let mut bad_opening = row.clone();
        set_field(&mut bad_opening, "opening_sha256", text(&"0".repeat(64)));
        state.prepare_rollover_entry(&control, &bad_opening, &ledger);
        self_check(
            !state.errors.is_empty(),
            "opening-boundary identity drift was accepted",
            &mut checks,
            &mut failures,
        );

        state.errors.clear();
        let mut bad_segment = row.clone();
        set_field(&mut bad_segment, "sha256", text(&"0".repeat(64)));
        state.prepare_rollover_entry(&control, &bad_segment, &ledger);
        self_check(
            !state.errors.is_empty(),
            "planned segment identity drift was accepted",
            &mut checks,
            &mut failures,
        );

        state.errors.clear();
        let mut bad_successor = row.clone();
        set_field(&mut bad_successor, "successor", text(&capsule_rel));
        state.prepare_rollover_entry(&control, &bad_successor, &ledger);
        self_check(
            !state.errors.is_empty(),
            "a non-newest rollover successor was accepted",
            &mut checks,
            &mut failures,
        );

        state.errors.clear();
        let failed_context = prepared.as_ref().map(|prepared| {
            state.install_rollover_transaction(
                std::slice::from_ref(prepared),
                Some("after_segments"),
            )
        });
        self_check(
            failed_context.is_some_and(|context| context.is_none()) && !state.errors.is_empty(),
            "an injected post-segment transaction failure was accepted",
            &mut checks,
            &mut failures,
        );
        let restored = slurp_or_die(&state.absolute(&source_rel))? == current
            && slurp_or_die(&state.absolute(&manifest_rel))? == manifest_bytes
            && slurp_or_die(&state.absolute(&index_rel))? == index_bytes
            && !state.absolute(&new_segment_rel).exists()
            && !transaction_stage.exists();
        if !restored {
            failures
                .push("failed transaction did not restore authority or remove residue".to_string());
        }

        state.errors.clear();
        let context = prepared.as_ref().and_then(|prepared| {
            state.install_rollover_transaction(std::slice::from_ref(prepared), None)
        });
        self_check(
            state.errors.is_empty() && context.is_some(),
            "valid root-last rollover installation failed",
            &mut checks,
            &mut failures,
        );
        if let (Some(context_value), Some(prepared_value)) = (&context, &prepared) {
            let _ = context_value;
            if slurp_or_die(&state.absolute(&source_rel))? != prepared_value.root_bytes {
                failures
                    .push("installed rollover root differs from its prepared bytes".to_string());
            }
            if slurp_or_die(&state.absolute(&new_segment_rel))? != prepared_value.segment_bytes {
                failures
                    .push("installed rollover segment differs from its prepared bytes".to_string());
            }
        }

        if let Some(context) = context {
            state.rollback_rollover_transaction(&context)?;
            self_check(
                slurp_or_die(&state.absolute(&source_rel))? == current
                    && slurp_or_die(&state.absolute(&manifest_rel))? == manifest_bytes
                    && slurp_or_die(&state.absolute(&index_rel))? == index_bytes
                    && !state.absolute(&new_segment_rel).exists()
                    && !transaction_stage.exists(),
                "rollback did not restore exact bytes or left residue",
                &mut checks,
                &mut failures,
            );
        } else {
            checks += 1;
            failures.push("rollback could not run without an installed context".to_string());
        }
        Ok(())
    };
    let unexpected = operation().err();
    if transaction_stage.exists() {
        let _ = fs::remove_dir_all(&transaction_stage);
    }
    if fixture.exists() {
        let _ = fs::remove_dir_all(&fixture);
    }
    if let Some(error) = unexpected {
        failures.push(error.trim_end_matches('\n').to_string());
    }
    state.surface_by_source = saved_surfaces;
    state.planned_rollover_ids = saved_plan_ids;
    state.post_apply_validation = saved_post_apply;
    state.errors = saved_errors;
    (checks, failures)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_forty_five_parser_and_control_cases_pass() {
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
            "rolling-ledger: 45/45 parser/control self-tests pass.\n"
        );
    }

    #[test]
    fn lone_surrogate_titles_keep_their_codepoints_and_raw_record_bytes() {
        let root = crate::repository_root();
        let mut state = State::new(root);
        let grammar = grammar("h2_records_v1");
        let bytes = b"## first\nfirst\n## \xed\xa0\x80\nbody\n## old\nold\n";
        let parsed = state.parse_ledger(bytes, &grammar, "surrogate").unwrap();
        assert_eq!(parsed.records[1].title, vec![35, 35, 32, 0xd800]);
        assert_eq!(reconstruct(&parsed), bytes);
        let plan = object(vec![
            ("prefix_records", integer(1)),
            (
                "promote_titles",
                Py::List(vec![Py::Str(vec![35, 35, 32, 0xd800])]),
            ),
        ]);
        let (selected, errors) =
            state.select_live_records(&parsed.records, Some(&plan), "surrogate");
        assert!(errors.is_empty());
        assert_eq!(selected.len(), 2);
        let expected = [
            parsed.records[0].bytes.clone(),
            parsed.records[1].bytes.clone(),
        ]
        .concat();
        assert_eq!(render_live_view(&parsed, &selected, &grammar), expected);
    }
}
