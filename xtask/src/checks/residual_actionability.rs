//! The `RESIDUAL-ACTIONABILITY` doctrine (`SPEC-TO-INTENT-ALIGNMENT.8a`), native since `RUSTDOC-AND-XTASK.13.f`: the
//! frozen required-residual contract (`doctrine/spec_to_intent/residual_actionability_contract.json`) still holds.
//! The required-residual rule is executed over its closed case matrix, the frozen decomposition is joined to the
//! tracked current result, the repaired reproduction command is bound to its published authorities, the retained
//! chain set is pinned by exact digest, and every reviewed residual gold names a record some carrier can emit.
//!
//! The registered script, `scripts/validate_residual_actionability_contract.py`, keeps its `argparse` front end and
//! passes this check `--check` or `--self-test`. Values are compared and printed as Python's `json` and `==`, `str`
//! and `repr` treat them (`xtask/src/pyjson.rs`). Where the script would stop with a traceback (a malformed
//! authority it indexes without checking), the port reports a refusal instead; and where it iterates a hash-ordered
//! `set` of field names, the port reports in sorted order.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::pyjson::{self, Py};

const LABEL: &str = "residual-actionability-contract";
const CONTRACT: &str = "doctrine/spec_to_intent/residual_actionability_contract.json";

const TOP_LEVEL: [&str; 12] = [
    "schema_version",
    "contract_id",
    "owner",
    "decision",
    "authorities",
    "requirement_rule",
    "record_grammar",
    "witness",
    "cases",
    "reproduction",
    "reconciliation",
    "residual_gold_law",
];
const AUTHORITIES: [&str; 6] = [
    "reviewed_dataset",
    "current_result",
    "retained_chains",
    "population_replay",
    "controller_input",
    "controller_report",
];
const RULE_FIELDS: [&str; 8] = [
    "observation_unit",
    "promoted_stages",
    "residual_disposition_rule",
    "canonical_disposition_rule",
    "fail_closed_rule",
    "duplicate_rule",
    "met_rule",
    "stage_independence",
];
const GRAMMAR_FIELDS: [&str; 8] = [
    "collection_pointer",
    "identity_fields",
    "provenance_field",
    "required_actionability_fields",
    "first_failing_stage_values",
    "typed_causes",
    "authority_rule",
    "no_fabrication_rule",
];
const CASE_FIELDS: [&str; 10] = [
    "case_id",
    "control_classes",
    "expected_disposition",
    "stage",
    "canonical_reviewed_keys",
    "canonical_promoted_keys",
    "residual_expected_keys",
    "residual_records",
    "expected_required_observations",
    "expected_met_observations",
];
const RECORD_FIELDS: [&str; 5] = [
    "fact_key",
    "source_ids",
    "reason",
    "first_failing_stage",
    "replay",
];
/// Which typed causes a production carrier exists for, in the script's declaration order; `None` is declared but
/// not built.
const TYPED_CAUSE_CARRIERS: [(&str, Option<&str>); 4] = [
    (
        "outside_executable_digital_domain",
        Some("TimingIntentDisposition::NonApplicable"),
    ),
    (
        "no_canonical_carrier_for_captured_region",
        Some("CapturedRegionResidualRecord"),
    ),
    ("non_contract_region", None),
    ("unresolved_grounding", None),
];
/// Each claimed carrier's production file and the declaration that must be in it.
const CARRIER_DECLARATIONS: [(&str, &str, &str); 2] = [
    (
        "TimingIntentDisposition::NonApplicable",
        "crates/specforge/src/ir/source.rs",
        "pub enum TimingIntentDisposition",
    ),
    (
        "CapturedRegionResidualRecord",
        "crates/specforge/src/ir/source.rs",
        "pub struct CapturedRegionResidualRecord",
    ),
];
const SELECTED_FAMILY_FIELDS: [&str; 9] = [
    "semantic_family",
    "modality",
    "category",
    "cells",
    "observations",
    "typed_cause",
    "selection_reason",
    "owner",
    "state",
];
const PROMOTED_STAGES: [&str; 2] = ["semantic_ir", "intent_ir"];
const ACTIONABILITY_FIELDS: [&str; 3] = ["/reason", "/first_failing_stage", "/replay"];
const BOUNDARIES: [&str; 3] = [
    "source_to_evidence_ir",
    "evidence_to_semantic_ir",
    "semantic_to_intent_ir",
];
const TYPED_CAUSES: [&str; 4] = [
    "no_canonical_carrier_for_captured_region",
    "non_contract_region",
    "outside_executable_digital_domain",
    "unresolved_grounding",
];
const REQUIRED_CONTROLS: [&str; 19] = [
    "required:residual_cell",
    "required:non_applicable_cell",
    "required:canonical_missing_key",
    "not_required:canonical_stage_exact",
    "met:canonical_missing_with_actionable_residual",
    "unmet:absent_residual",
    "unmet:canonical_missing_without_residual",
    "unmet:missing_reason",
    "unmet:missing_first_failing_stage",
    "unmet:missing_replay",
    "unmet:unknown_first_failing_stage",
    "unmet:missing_provenance",
    "unmet:partial_residual_keys",
    "unmet:extra_residual_key",
    "refuse:duplicate_of_promoted_canonical",
    "stage:semantic_ir",
    "stage:intent_ir",
    "witness:not_required_canonical_cell",
    "witness:required_and_absent_cell",
];
const CASE_TOTALS: (usize, usize, usize) = (18, 17, 5);
const AFFECTED_STAGES: [&str; 3] = ["semantic_ir", "intent_ir", "isf_adapter"];
const REPLAY_STAGES: [&str; 4] = ["source_ir", "evidence_ir", "semantic_ir", "intent_ir"];
const REPRODUCTION_FIELDS: [&str; 7] = [
    "gap_id",
    "defective_command",
    "repaired_command",
    "defect",
    "control_producer",
    "control_tests",
    "published_authorities",
];
const GOLD_LAW_FIELDS: [&str; 8] = [
    "law",
    "region_scoped_law",
    "region_scoped_carrier",
    "required_disposition_values",
    "region_scoped_cells",
    "fact_scoped_cells",
    "published_result_agreement",
    "what_this_still_permits",
];
const FACT_SCOPED_FIELDS: [&str; 3] = ["cell_id", "carrier", "keys"];

/// Run the check from `root` with the mode the shim passed.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    match arguments {
        [] => run_check(root),
        [mode] if mode == "--check" => run_check(root),
        [mode] if mode == "--self-test" => run_self_test(root),
        _ => {
            eprintln!("Usage: cargo xtask check residual-actionability [--check|--self-test]");
            2
        }
    }
}

// ---- Python's view of a parsed value --------------------------------------------------------------------------

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

fn int(value: usize) -> Py {
    Py::Int(value.to_string())
}

/// `value.get(key)`: the member, or `None` when absent (a `null` member reads as `None` too).
fn get(value: &Py, key: &str) -> Py {
    pyjson::get(value, key).cloned().unwrap_or(Py::None)
}

/// `value.get(key, default)`: the member when present, even `null`, otherwise the default.
fn get_or(value: &Py, key: &str, default: Py) -> Py {
    pyjson::get(value, key).cloned().unwrap_or(default)
}

fn is_dict(value: &Py) -> bool {
    matches!(value, Py::Dict(_))
}

fn is_str(value: &Py) -> bool {
    matches!(value, Py::Str(_))
}

/// `isinstance(value, str) and value.strip()`.
fn nonempty_statement(value: &Py) -> bool {
    matches!(value, Py::Str(text) if !pyjson::to_text(text).trim().is_empty())
}

/// `set(value) == wanted` for a dict's keys.
fn keys_are(value: &Py, wanted: &[&str]) -> bool {
    let Py::Dict(pairs) = value else {
        return false;
    };
    let keys: BTreeSet<String> = pairs.iter().map(|(key, _)| pyjson::to_text(key)).collect();
    keys == wanted.iter().map(|key| key.to_string()).collect()
}

fn eq(left: &Py, right: &Py) -> bool {
    pyjson::py_eq(left, right)
}

fn eq_str(value: &Py, wanted: &str) -> bool {
    eq(value, &text(wanted))
}

fn str_list(items: &[&str]) -> Py {
    Py::List(items.iter().map(|item| text(item)).collect())
}

/// `value in items` for a list of strings.
fn in_strs(value: &Py, items: &[&str]) -> bool {
    items.iter().any(|item| eq_str(value, item))
}

/// `value in items` for a parsed list, compared with `==`.
fn in_list(value: &Py, items: &Py) -> bool {
    match items {
        Py::List(items) => items.iter().any(|item| eq(item, value)),
        Py::Dict(pairs) => pairs
            .iter()
            .any(|(key, _)| eq(&Py::Str(key.clone()), value)),
        Py::Str(haystack) => match value {
            Py::Str(needle) => pyjson::to_text(haystack).contains(&pyjson::to_text(needle)),
            _ => false,
        },
        _ => false,
    }
}

fn list(value: &Py) -> Vec<Py> {
    match value {
        Py::List(items) => items.clone(),
        _ => Vec::new(),
    }
}

/// A hash key under which Python's `==` and `hash` agree: `1`, `1.0` and `True` share one.
fn key(value: &Py) -> String {
    match value {
        Py::None => "N".to_string(),
        Py::Bool(flag) => format!("n{}", u8::from(*flag)),
        Py::Int(digits) => format!("n{digits}"),
        Py::Float(float) if float.is_finite() && float.fract() == 0.0 => {
            format!(
                "n{}",
                if *float == 0.0 {
                    "0".to_string()
                } else {
                    format!("{float:.0}")
                }
            )
        }
        Py::Float(float) => format!("f{}", float.to_bits()),
        Py::Str(characters) => format!("s{}", pyjson::to_text(characters)),
        other => format!("u{}", pyjson::repr(other)),
    }
}

/// Python's ordering of two values `sorted` compares: strings by code point, numbers by value.
fn order(left: &Py, right: &Py) -> std::cmp::Ordering {
    let number = |value: &Py| match value {
        Py::Bool(flag) => Some(f64::from(u8::from(*flag))),
        Py::Int(digits) => digits.parse::<f64>().ok(),
        Py::Float(float) => Some(*float),
        _ => None,
    };
    match (left, right) {
        (Py::Str(left), Py::Str(right)) => left.cmp(right),
        _ => match (number(left), number(right)) {
            (Some(left), Some(right)) => left
                .partial_cmp(&right)
                .unwrap_or(std::cmp::Ordering::Equal),
            _ => pyjson::repr(left).cmp(&pyjson::repr(right)),
        },
    }
}

fn sorted(mut items: Vec<Py>) -> Vec<Py> {
    items.sort_by(order);
    items
}

fn str_of(value: &Py) -> String {
    pyjson::str_of(value)
}

/// A Python `Counter` of hashable values: each distinct key with its count.
fn counter(items: &[Py]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for item in items {
        *counts.entry(key(item)).or_insert(0) += 1;
    }
    counts
}

fn repr_strings(items: &[String]) -> String {
    pyjson::repr(&Py::List(items.iter().map(|item| text(item)).collect()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

// ---- The contract's validators -------------------------------------------------------------------------------

fn repository_path(root: &Path, value: &Py, label: &str, errors: &mut Vec<String>) {
    let Py::Str(characters) = value else {
        errors.push(format!(
            "{label} must be a non-empty repository-relative path"
        ));
        return;
    };
    let path = pyjson::to_text(characters);
    if path.is_empty() {
        errors.push(format!(
            "{label} must be a non-empty repository-relative path"
        ));
        return;
    }
    if path.starts_with('/') || path.split('/').any(|part| part == "..") {
        errors.push(format!(
            "{label} must be repository-relative without parent traversal: {path}"
        ));
        return;
    }
    if !root.join(&path).is_file() {
        errors.push(format!(
            "{label} does not exist as a tracked authority: {path}"
        ));
    }
}

fn retained_id_digest(ids: &[String]) -> String {
    let mut sorted_ids = ids.to_vec();
    sorted_ids.sort_by_key(|item| pyjson::py_str(item));
    sha256_hex(
        sorted_ids
            .iter()
            .map(|id| format!("{id}\n"))
            .collect::<String>()
            .as_bytes(),
    )
}

/// The executable definition of an actionable residual record.
fn record_is_actionable(record: &Py, grammar: &Py) -> bool {
    match get(record, "source_ids") {
        Py::List(items) if !items.is_empty() => {}
        _ => return false,
    }
    for pointer in list(&get_or(
        grammar,
        "required_actionability_fields",
        Py::List(Vec::new()),
    )) {
        let field = str_of(&pointer);
        if !nonempty_statement(&get(record, field.trim_start_matches('/'))) {
            return false;
        }
    }
    in_list(
        &get(record, "first_failing_stage"),
        &get_or(grammar, "first_failing_stage_values", Py::List(Vec::new())),
    )
}

/// The executable definition of the required-residual rule for one cell at one promoted stage.
fn simulate_case(case: &Py, grammar: &Py) -> (usize, usize) {
    let records: Vec<Py> = list(&get(case, "residual_records"))
        .into_iter()
        .filter(is_dict)
        .collect();
    let promoted = counter(&list(&get(case, "canonical_promoted_keys")));
    let duplicated = records
        .iter()
        .any(|record| promoted.contains_key(&key(&get(record, "fact_key"))));
    if !eq_str(&get(case, "expected_disposition"), "canonical") {
        let keys: Vec<Py> = records
            .iter()
            .map(|record| get(record, "fact_key"))
            .collect();
        let exact = counter(&keys) == counter(&list(&get(case, "residual_expected_keys")));
        let met = exact
            && !records.is_empty()
            && !duplicated
            && records
                .iter()
                .all(|record| record_is_actionable(record, grammar));
        return (1, usize::from(met));
    }
    let reviewed = counter(&list(&get(case, "canonical_reviewed_keys")));
    let missing: BTreeMap<String, usize> = reviewed
        .iter()
        .filter_map(|(name, count)| {
            let left = count.saturating_sub(promoted.get(name).copied().unwrap_or(0));
            (left > 0).then(|| (name.clone(), left))
        })
        .collect();
    let required = missing.values().sum();
    if duplicated {
        return (required, 0);
    }
    let explained = missing
        .iter()
        .map(|(name, count)| {
            let usable = records
                .iter()
                .filter(|record| {
                    key(&get(record, "fact_key")) == *name && record_is_actionable(record, grammar)
                })
                .count();
            (*count).min(usable)
        })
        .sum();
    (required, explained)
}

fn validate_rule_and_grammar(root: &Path, contract: &Py, errors: &mut Vec<String>) {
    let rule = get(contract, "requirement_rule");
    if !keys_are(&rule, &RULE_FIELDS) {
        errors.push("requirement_rule fields differ from the closed schema".to_string());
    } else {
        if !eq(&get(&rule, "promoted_stages"), &str_list(&PROMOTED_STAGES)) {
            errors.push("requirement_rule must count exactly the two promoted stages".to_string());
        }
        let mut fields: Vec<&str> = RULE_FIELDS
            .iter()
            .copied()
            .filter(|field| *field != "promoted_stages")
            .collect();
        fields.sort();
        for field in fields {
            if !nonempty_statement(&get(&rule, field)) {
                errors.push(format!(
                    "requirement_rule.{field} must be a non-empty statement"
                ));
            }
        }
        if !in_list(
            &text("never leave the denominator"),
            &get_or(&rule, "fail_closed_rule", text("")),
        ) {
            errors.push(
                "fail_closed_rule must keep a missing canonical key inside the denominator"
                    .to_string(),
            );
        }
        if !in_list(
            &text("no met observation at that stage"),
            &get_or(&rule, "duplicate_rule", text("")),
        ) {
            errors.push(
                "duplicate_rule must refuse credit for a stage carrying a duplicate residual"
                    .to_string(),
            );
        }
    }
    let grammar = get(contract, "record_grammar");
    if !keys_are(&grammar, &GRAMMAR_FIELDS) {
        errors.push("record_grammar fields differ from the closed schema".to_string());
        return;
    }
    if !eq_str(&get(&grammar, "collection_pointer"), "/residuals") {
        errors.push(
            "record_grammar must keep the reviewed /residuals collection pointer".to_string(),
        );
    }
    if !eq(
        &get(&grammar, "identity_fields"),
        &str_list(&["/region_id", "/family", "/fact_key"]),
    ) {
        errors.push("record_grammar identity must remain region, family, and fact key".to_string());
    }
    if !eq_str(&get(&grammar, "provenance_field"), "/source_ids") {
        errors.push("record_grammar provenance field must remain /source_ids".to_string());
    }
    if !eq(
        &get(&grammar, "required_actionability_fields"),
        &str_list(&ACTIONABILITY_FIELDS),
    ) {
        errors.push(
            "record_grammar must require exactly reason, first failing stage, and replay"
                .to_string(),
        );
    }
    if !eq(
        &get(&grammar, "first_failing_stage_values"),
        &str_list(&BOUNDARIES),
    ) {
        errors.push(
            "record_grammar boundaries must be the three source-to-intent promotions".to_string(),
        );
    }
    let Py::List(causes) = get(&grammar, "typed_causes") else {
        errors.push("record_grammar must declare the four closed typed causes".to_string());
        return;
    };
    if causes.len() != TYPED_CAUSES.len() {
        errors.push("record_grammar must declare the four closed typed causes".to_string());
        return;
    }
    let mut observed: Vec<String> = causes
        .iter()
        .filter(|cause| is_dict(cause))
        .map(|cause| str_of(&get(cause, "cause_id")))
        .collect();
    observed.sort_by_key(|item| pyjson::py_str(item));
    if observed != TYPED_CAUSES.map(String::from) {
        errors.push(format!(
            "typed causes differ from the frozen closed set: {}",
            repr_strings(&observed)
        ));
    }
    for cause in &causes {
        if !keys_are(cause, &["cause_id", "definition", "existing_carrier"]) {
            errors.push(
                "each typed cause must declare id, definition, and existing carrier".to_string(),
            );
            continue;
        }
        if !nonempty_statement(&get(cause, "definition")) {
            errors.push(format!(
                "typed cause {} needs a definition",
                str_of(&get(cause, "cause_id"))
            ));
        }
    }
    let mut carriers: BTreeMap<String, Py> = BTreeMap::new();
    for cause in causes.iter().filter(|cause| is_dict(cause)) {
        carriers.insert(key(&get(cause, "cause_id")), get(cause, "existing_carrier"));
    }
    for (cause_id, expected) in TYPED_CAUSE_CARRIERS {
        let declared = carriers
            .get(&key(&text(cause_id)))
            .cloned()
            .unwrap_or(Py::None);
        let expected_value = expected.map_or(Py::None, text);
        if !eq(&declared, &expected_value) {
            errors.push(format!(
                "typed cause {cause_id} must declare carrier {}, not {}",
                pyjson::repr(&expected_value),
                pyjson::repr(&declared)
            ));
            continue;
        }
        let Some(carrier) = expected else {
            continue;
        };
        let Some((_, path, signature)) = CARRIER_DECLARATIONS
            .iter()
            .find(|(name, _, _)| *name == carrier)
        else {
            errors.push(format!(
                "typed cause {cause_id} names an unknown carrier {}",
                pyjson::repr(&declared)
            ));
            continue;
        };
        let present =
            fs::read_to_string(root.join(path)).is_ok_and(|source| source.contains(signature));
        if !present {
            errors.push(format!(
                "typed cause {cause_id} claims carrier {}, but {} is absent from {path}",
                pyjson::repr(&declared),
                pyjson::string_repr(&pyjson::py_str(signature))
            ));
        }
    }
}

fn validate_cases(contract: &Py, errors: &mut Vec<String>) -> (usize, usize, usize) {
    let grammar = get(contract, "record_grammar");
    let cases = match get(contract, "cases") {
        Py::List(cases) if is_dict(&grammar) && !cases.is_empty() => cases,
        _ => {
            errors.push(
                "cases must be a non-empty array validated against the record grammar".to_string(),
            );
            return (0, 0, 0);
        }
    };
    let mut seen = BTreeSet::new();
    let mut controls = BTreeSet::new();
    let (mut required_total, mut met_total) = (0, 0);
    for (index, case) in cases.iter().enumerate() {
        let label = format!("cases[{index}]");
        if !keys_are(case, &CASE_FIELDS) {
            errors.push(format!("{label} fields differ from the closed case schema"));
            continue;
        }
        let case_id = match get(case, "case_id") {
            Py::Str(characters) if !characters.is_empty() => pyjson::to_text(&characters),
            _ => {
                errors.push(format!("{label}.case_id must be a non-empty string"));
                continue;
            }
        };
        if !seen.insert(case_id.clone()) {
            errors.push(format!("duplicate case_id: {case_id}"));
        }
        match get(case, "control_classes") {
            Py::List(items) if !items.is_empty() => controls.extend(items.iter().map(str_of)),
            _ => errors.push(format!("{case_id} must declare at least one control class")),
        }
        if !in_strs(
            &get(case, "expected_disposition"),
            &["canonical", "residual", "non_applicable"],
        ) {
            errors.push(format!("{case_id} has an unknown reviewed disposition"));
            continue;
        }
        if !in_strs(&get(case, "stage"), &PROMOTED_STAGES) {
            errors.push(format!("{case_id} must be scored at one promoted stage"));
            continue;
        }
        for field in [
            "canonical_reviewed_keys",
            "canonical_promoted_keys",
            "residual_expected_keys",
        ] {
            if !matches!(get(case, field), Py::List(_)) {
                errors.push(format!("{case_id}.{field} must be an array"));
            }
        }
        let Py::List(records) = get(case, "residual_records") else {
            errors.push(format!("{case_id}.residual_records must be an array"));
            continue;
        };
        for record in &records {
            if !keys_are(record, &RECORD_FIELDS) {
                errors.push(format!(
                    "{case_id} residual record differs from the typed record schema"
                ));
            }
        }
        if !eq_str(&get(case, "expected_disposition"), "canonical")
            && pyjson::truthy(&get(case, "canonical_reviewed_keys"))
        {
            errors.push(format!(
                "{case_id} non-canonical cells must not declare canonical gold"
            ));
        }
        let promoted: BTreeSet<String> = list(&get_or(
            case,
            "canonical_promoted_keys",
            Py::List(Vec::new()),
        ))
        .iter()
        .map(key)
        .collect();
        let reviewed: BTreeSet<String> = list(&get_or(
            case,
            "canonical_reviewed_keys",
            Py::List(Vec::new()),
        ))
        .iter()
        .map(key)
        .collect();
        if !promoted.is_subset(&reviewed) {
            errors.push(format!(
                "{case_id} promotes a key the review never declared"
            ));
        }
        let (required, met) = simulate_case(case, &grammar);
        let declared_required = get(case, "expected_required_observations");
        if !eq(&declared_required, &int(required)) {
            errors.push(format!(
                "{case_id} declares {} required observations; the rule derives {required}",
                str_of(&declared_required)
            ));
        }
        let declared_met = get(case, "expected_met_observations");
        if !eq(&declared_met, &int(met)) {
            errors.push(format!(
                "{case_id} declares {} met observations; the rule derives {met}",
                str_of(&declared_met)
            ));
        }
        if met > required {
            errors.push(format!(
                "{case_id} cannot meet more observations than it requires"
            ));
        }
        required_total += required;
        met_total += met;
    }
    let required_controls: BTreeSet<String> = REQUIRED_CONTROLS
        .iter()
        .map(|control| control.to_string())
        .collect();
    let python_sorted = |set: BTreeSet<String>| {
        let mut items: Vec<String> = set.into_iter().collect();
        items.sort_by_key(|item| pyjson::py_str(item));
        items
    };
    let missing = python_sorted(required_controls.difference(&controls).cloned().collect());
    let unknown = python_sorted(controls.difference(&required_controls).cloned().collect());
    if !missing.is_empty() {
        errors.push(format!(
            "case matrix is missing controls: {}",
            repr_strings(&missing)
        ));
    }
    if !unknown.is_empty() {
        errors.push(format!(
            "case matrix has undeclared controls: {}",
            repr_strings(&unknown)
        ));
    }
    let totals = (cases.len(), required_total, met_total);
    if totals != CASE_TOTALS {
        errors.push(format!(
            "case denominator must remain ({}, {}, {}) cases/required/met; got ({}, {}, {})",
            CASE_TOTALS.0, CASE_TOTALS.1, CASE_TOTALS.2, totals.0, totals.1, totals.2
        ));
    }
    totals
}

/// One cell of the current result's declared/required/actionable partition.
fn row(document: &Py, cell: &Py, residual: &Py) -> Vec<(pyjson::PyStr, Py)> {
    let semantic = get_or(residual, "semantic_ir", Py::Dict(Vec::new()));
    vec![
        (
            pyjson::py_str("document_key"),
            get(document, "document_key"),
        ),
        (pyjson::py_str("cell_id"), get(cell, "cell_id")),
        (pyjson::py_str("category"), get(document, "category")),
        (
            pyjson::py_str("semantic_family"),
            get(cell, "semantic_family"),
        ),
        (pyjson::py_str("modality"), get(cell, "modality")),
        (
            pyjson::py_str("expected_disposition"),
            get(cell, "expected_disposition"),
        ),
        (
            pyjson::py_str("reviewed_residual_keys"),
            Py::List(sorted(list(&get_or(
                &semantic,
                "expected_keys",
                Py::List(Vec::new()),
            )))),
        ),
    ]
}

struct Buckets {
    actionable: Vec<Py>,
    not_required: Vec<Py>,
    required_absent: Vec<Py>,
}

/// Re-derive the declared/required/actionable partition from the tracked current result.
fn decompose_current_result(current: &Py) -> Buckets {
    let mut buckets = Buckets {
        actionable: Vec::new(),
        not_required: Vec::new(),
        required_absent: Vec::new(),
    };
    let zero = int(0);
    let stage_exact = |holder: &Py, stage: &str| {
        let stage = get_or(holder, stage, Py::Dict(Vec::new()));
        [
            "false_positives",
            "false_negatives",
            "unprovenanced_records",
        ]
        .iter()
        .all(|field| eq(&get(&stage, field), &zero))
    };
    for document in list(&get_or(current, "documents", Py::List(Vec::new()))) {
        for cell in list(&get_or(&document, "cells", Py::List(Vec::new()))) {
            let residual = get(&cell, "residual");
            if !is_dict(&residual) {
                continue;
            }
            let canonical = get(&cell, "canonical");
            let canonical_exact = is_dict(&canonical)
                && ["evidence_ir", "semantic_ir", "intent_ir"]
                    .iter()
                    .all(|stage| stage_exact(&canonical, stage));
            let met = PROMOTED_STAGES.iter().all(|stage| {
                let prefix = stage.split('_').next().unwrap_or(stage);
                stage_exact(&residual, stage)
                    && matches!(
                        get(&residual, &format!("{prefix}_actionable")),
                        Py::Bool(true)
                    )
            });
            let mut fields = row(&document, &cell, &residual);
            if met {
                buckets.actionable.push(Py::Dict(fields));
            } else if eq_str(&get(&cell, "expected_disposition"), "canonical") && canonical_exact {
                fields.push((
                    pyjson::py_str("canonical_exact_at_every_promoted_stage"),
                    Py::Bool(true),
                ));
                buckets.not_required.push(Py::Dict(fields));
            } else {
                fields.push((
                    pyjson::py_str("hard_failures"),
                    Py::List(sorted(list(&get_or(
                        &cell,
                        "hard_failures",
                        Py::List(Vec::new()),
                    )))),
                ));
                buckets.required_absent.push(Py::Dict(fields));
            }
        }
    }
    for rows in [
        &mut buckets.actionable,
        &mut buckets.not_required,
        &mut buckets.required_absent,
    ] {
        rows.sort_by(|left, right| {
            order(&get(left, "document_key"), &get(right, "document_key"))
                .then_with(|| order(&get(left, "cell_id"), &get(right, "cell_id")))
        });
    }
    buckets
}

fn validate_witness(root: &Path, contract: &Py, current: &Py, errors: &mut Vec<String>) {
    let witness = get(contract, "witness");
    if !is_dict(&witness) {
        errors.push("witness must be an object".to_string());
        return;
    }
    let authorities = get_or(contract, "authorities", Py::Dict(Vec::new()));
    if let Py::Str(path) = get(&authorities, "current_result") {
        let path = root.join(pyjson::to_text(&path));
        if path.is_file() {
            let digest = sha256_hex(&fs::read(&path).unwrap_or_default());
            if !eq_str(&get(&witness, "current_result_sha256"), &digest) {
                errors
                    .push("witness no longer pins the tracked current-result identity".to_string());
            }
        }
    }
    let buckets = decompose_current_result(current);
    let (actionable, not_required, required_absent) = (
        buckets.actionable.len(),
        buckets.not_required.len(),
        buckets.required_absent.len(),
    );
    let declared = actionable + not_required + required_absent;
    let expected = [
        ("declared_cells", declared),
        ("declared_observations", 2 * declared),
        ("actionable_observations", 2 * actionable),
        ("not_required_observations", 2 * not_required),
        ("required_and_absent_observations", 2 * required_absent),
        (
            "corrected_required_observations",
            2 * (actionable + required_absent),
        ),
        ("corrected_met_observations", 2 * actionable),
    ];
    for (field, value) in expected {
        let found = get(&witness, field);
        if !eq(&found, &int(value)) {
            errors.push(format!(
                "witness.{field} is {}; the current result derives {value}",
                str_of(&found)
            ));
        }
    }
    for (field, rows) in [
        ("actionable_cells", &buckets.actionable),
        ("not_required_cells", &buckets.not_required),
        ("required_and_absent_cells", &buckets.required_absent),
    ] {
        if !eq(&get(&witness, field), &Py::List(rows.clone())) {
            errors.push(format!(
                "witness.{field} differs from the current-result derivation"
            ));
        }
    }
    let missing_failure = buckets.required_absent.iter().any(|row| {
        !in_list(
            &text("required_residual_missing_or_inactionable"),
            &get_or(row, "hard_failures", Py::List(Vec::new())),
        )
    });
    if missing_failure {
        errors.push(
            "every required-and-absent cell must carry the residual hard failure".to_string(),
        );
    }
    if buckets
        .not_required
        .iter()
        .any(|row| pyjson::truthy(&get(row, "hard_failures")))
    {
        errors.push("a not-required cell must not carry a hard failure".to_string());
    }
    let family = get(&witness, "selected_family");
    if !keys_are(&family, &SELECTED_FAMILY_FIELDS) {
        errors.push("selected_family must declare the closed selection schema".to_string());
        return;
    }
    let state = get(&family, "state");
    if !in_strs(&state, &["open", "closed"]) {
        errors.push("selected_family state must be open or closed".to_string());
        return;
    }
    let open = eq_str(&state, "open");
    let (bucket_name, bucket) = if open {
        ("required_absent", &buckets.required_absent)
    } else {
        ("actionable", &buckets.actionable)
    };
    let selected: Vec<&Py> = bucket
        .iter()
        .filter(|row| {
            eq(
                &get(row, "semantic_family"),
                &get(&family, "semantic_family"),
            )
        })
        .collect();
    if !eq(&int(selected.len()), &get(&family, "cells"))
        || !eq(&int(2 * selected.len()), &get(&family, "observations"))
    {
        errors.push(format!("selected family cell/observation counts differ from the current result's {bucket_name} cells"));
    }
    if selected
        .iter()
        .any(|row| !eq(&get(row, "modality"), &get(&family, "modality")))
    {
        errors.push("selected family spans more than its declared modality".to_string());
    }
    if selected
        .iter()
        .any(|row| !eq(&get(row, "category"), &get(&family, "category")))
    {
        errors.push("selected family spans more than its declared category".to_string());
    }
    let typed_cause = get(&family, "typed_cause");
    if !in_strs(&typed_cause, &TYPED_CAUSES) {
        errors.push("selected family must name a declared typed cause".to_string());
    }
    if !eq_str(&get(&family, "owner"), "SPEC-TO-INTENT-ALIGNMENT.8c") {
        errors.push("the bounded production family must be owned by .8c".to_string());
    }
    let ships = TYPED_CAUSE_CARRIERS
        .iter()
        .any(|(cause, carrier)| eq_str(&typed_cause, cause) && carrier.is_some());
    if !open && !ships {
        errors.push(
            "a closed selected family must name a typed cause whose carrier ships".to_string(),
        );
    }
    let gaps = buckets
        .required_absent
        .iter()
        .filter(|row| eq(&get(row, "category"), &get(&family, "category")))
        .count();
    if gaps != if open { selected.len() } else { 0 } {
        errors.push(
            "the selected family must account for every residual gap in its category".to_string(),
        );
    }
}

fn validate_reproduction(root: &Path, contract: &Py, errors: &mut Vec<String>) {
    let reproduction = get(contract, "reproduction");
    if !keys_are(&reproduction, &REPRODUCTION_FIELDS) {
        errors.push("reproduction fields differ from the closed schema".to_string());
        return;
    }
    let (Py::Str(defective), Py::Str(repaired)) = (
        get(&reproduction, "defective_command"),
        get(&reproduction, "repaired_command"),
    ) else {
        errors.push(
            "reproduction must contrast a distinct defective and repaired command".to_string(),
        );
        return;
    };
    let (defective, repaired) = (pyjson::to_text(&defective), pyjson::to_text(&repaired));
    if defective == repaired {
        errors.push(
            "reproduction must contrast a distinct defective and repaired command".to_string(),
        );
        return;
    }
    if !format!("{defective} ").contains("-p specforge ")
        || !repaired.contains("-p specforge-conformance")
    {
        errors.push(
            "the repaired command must move the gap onto the owning conformance package"
                .to_string(),
        );
    }
    let producer = get(&reproduction, "control_producer");
    repository_path(root, &producer, "reproduction.control_producer", errors);
    if let Py::Str(path) = &producer {
        let path = root.join(pyjson::to_text(path));
        if path.is_file() {
            let source = fs::read_to_string(&path).unwrap_or_default();
            if !source.contains(&repaired) {
                errors.push("the repaired command is absent from the control producer".to_string());
            }
            match source.find("#[cfg(test)]") {
                None => errors.push("the control producer declares no test plane".to_string()),
                Some(test_plane) => {
                    if source.find(&defective).is_some_and(|at| at < test_plane) {
                        errors.push("the defective command survives in composition code, not only as a RED fixture".to_string());
                    }
                }
            }
            for test in list(&get_or(
                &reproduction,
                "control_tests",
                Py::List(Vec::new()),
            )) {
                let name = str_of(&test);
                let name = name.rsplit("::").next().unwrap_or(&name).to_string();
                if !source.contains(&format!("fn {name}(")) {
                    errors.push(format!(
                        "control test '{name}' is absent from the control producer"
                    ));
                }
            }
        }
    }
    let paths = match get(&reproduction, "published_authorities") {
        Py::List(paths) if !paths.is_empty() => paths,
        _ => {
            errors.push("reproduction must publish its authority set".to_string());
            return;
        }
    };
    for path in &paths {
        repository_path(root, path, "reproduction.published_authorities", errors);
        let Py::Str(relative) = path else {
            continue;
        };
        let relative = pyjson::to_text(relative);
        let file = root.join(&relative);
        if !file.is_file() {
            continue;
        }
        let content = fs::read_to_string(&file).unwrap_or_default();
        if !content.contains(&repaired) {
            errors.push(format!(
                "published authority '{relative}' does not carry the repaired command"
            ));
        }
        if content.contains(&defective) {
            errors.push(format!(
                "published authority '{relative}' still carries the defective command"
            ));
        }
    }
}

fn validate_reconciliation(contract: &Py, retained: &Py, errors: &mut Vec<String>) {
    let reconciliation = get(contract, "reconciliation");
    if !is_dict(&reconciliation) {
        errors.push("reconciliation must be an object".to_string());
        return;
    }
    let authorities = get_or(contract, "authorities", Py::Dict(Vec::new()));
    if !eq(
        &get(&reconciliation, "affected_chain_contract"),
        &get(&authorities, "retained_chains"),
    ) {
        errors.push(
            "affected-chain contract must be the declared retained-chain authority".to_string(),
        );
    }
    let mut ids = get(retained, "retained");
    if !matches!(ids, Py::List(_)) || !matches!(get(retained, "reclamations"), Py::List(_)) {
        errors.push(
            "retained-chain authority must declare list-valued retained and reclamations"
                .to_string(),
        );
        ids = Py::List(Vec::new());
    }
    let ids = list(&ids);
    if !eq(
        &get(&reconciliation, "affected_chain_count"),
        &int(ids.len()),
    ) {
        errors.push(
            "affected-chain denominator must equal the live retained-chain count".to_string(),
        );
    }
    let strings: Vec<String> = ids.iter().map(str_of).collect();
    if !eq_str(
        &get(&reconciliation, "affected_chain_ids_sha256"),
        &retained_id_digest(&strings),
    ) {
        errors.push("affected-chain exact-set digest differs from retained authority".to_string());
    }
    if !eq(
        &get(&reconciliation, "affected_stages"),
        &str_list(&AFFECTED_STAGES),
    ) {
        errors.push("affected stages must cover SemanticIR through the ISF adapter".to_string());
    }
    if !eq(
        &get(&reconciliation, "reviewed_replay_stages"),
        &str_list(&REPLAY_STAGES),
    ) {
        errors.push(
            "reviewed replay must retain the four isolated source-to-intent stages".to_string(),
        );
    }
    let documents = get(&reconciliation, "reviewed_population_documents");
    let attempts = get(&reconciliation, "reviewed_replay_attempts");
    let product = match &documents {
        Py::Int(digits) => digits
            .parse::<i64>()
            .ok()
            .map(|count| int((count * 4).max(0) as usize)),
        _ => None,
    };
    if !eq(&documents, &int(12))
        || !eq(&attempts, &int(48))
        || product.is_none_or(|product| !eq(&attempts, &product))
    {
        errors
            .push("reviewed replay denominator must remain 12 sources x 4 stages = 48".to_string());
    }
    for (field, owner) in [
        ("measurement_owner", "SPEC-TO-INTENT-ALIGNMENT.8b"),
        ("production_owner", "SPEC-TO-INTENT-ALIGNMENT.8c"),
        ("publication_owner", "SPEC-TO-INTENT-ALIGNMENT.8d"),
    ] {
        if !eq_str(&get(&reconciliation, field), owner) {
            errors.push(format!("reconciliation.{field} must remain {owner}"));
        }
    }
    match get(&reconciliation, "publication_invariants") {
        Py::List(items) if items.len() >= 5 => {}
        _ => errors.push("publication invariants must remain complete".to_string()),
    }
}

/// The only key a region-scoped carrier could emit for this query's own predicate pair.
fn region_scoped_gold_key(query: &Py) -> Option<String> {
    if !is_dict(query) {
        return None;
    }
    let mut pinned: Vec<(Py, Py)> = Vec::new();
    for predicate in list(&get_or(query, "predicates", Py::List(Vec::new()))) {
        if !is_dict(&predicate) || !eq_str(&get(&predicate, "operator"), "equals") {
            return None;
        }
        let field = get(&predicate, "field");
        if pinned
            .iter()
            .any(|(existing, _)| key(existing) == key(&field))
        {
            return None;
        }
        pinned.push((field, get(&predicate, "value")));
    }
    let fields: BTreeSet<String> = pinned.iter().map(|(field, _)| key(field)).collect();
    if fields
        != [key(&text("/region_id")), key(&text("/family"))]
            .into_iter()
            .collect()
    {
        return None;
    }
    let value = |name: &str| {
        pinned
            .iter()
            .find(|(field, _)| eq_str(field, name))
            .map(|(_, value)| value.clone())
    };
    match (value("/region_id")?, value("/family")?) {
        (Py::Str(region), Py::Str(family)) => Some(format!(
            "{}|{}",
            pyjson::to_text(&region),
            pyjson::to_text(&family)
        )),
        _ => None,
    }
}

type GoldKey = (String, String, String);

/// Every residual gold key set a dataset or a scored report declares, by cell and stage: the key, its printed
/// cell id, and the sorted keys.
fn reviewed_residual_gold(report: &Py) -> BTreeMap<GoldKey, (Py, String, Vec<Py>)> {
    let mut gold = BTreeMap::new();
    for document in list(&get_or(report, "documents", Py::List(Vec::new()))) {
        for cell in list(&get_or(&document, "cells", Py::List(Vec::new()))) {
            let residual = get(&cell, "residual");
            if !is_dict(&residual) {
                continue;
            }
            for stage in PROMOTED_STAGES {
                let query = get(&residual, stage);
                if is_dict(&query) {
                    let id = (
                        key(&get(&document, "document_key")),
                        key(&get(&cell, "cell_id")),
                        stage.to_string(),
                    );
                    let keys = sorted(list(&get_or(&query, "expected_keys", Py::List(Vec::new()))));
                    gold.insert(id, (get(&cell, "cell_id"), stage.to_string(), keys));
                }
            }
        }
    }
    gold
}

fn validate_published_gold_agreement(dataset: &Py, current: &Py, errors: &mut Vec<String>) {
    let reviewed = reviewed_residual_gold(dataset);
    let published = reviewed_residual_gold(current);
    if reviewed.keys().collect::<BTreeSet<_>>() != published.keys().collect::<BTreeSet<_>>() {
        errors.push("the published current result and the review-locked dataset declare different residual cells".to_string());
        return;
    }
    let mut stale: Vec<&(Py, String, Vec<Py>)> = reviewed
        .iter()
        .filter(|(id, (_, _, keys))| {
            !eq(&Py::List(keys.clone()), &Py::List(published[*id].2.clone()))
        })
        .map(|(_, value)| value)
        .collect();
    stale.sort_by(|left, right| order(&left.0, &right.0).then_with(|| left.1.cmp(&right.1)));
    if !stale.is_empty() {
        let names: Vec<String> = stale
            .iter()
            .map(|(cell, stage, _)| format!("{}@{stage}", str_of(cell)))
            .collect();
        errors.push(format!(
            "the published current result carries a reviewed gold the review-locked dataset no longer declares: {}",
            repr_strings(&names)
        ));
    }
}

fn validate_residual_gold_law(contract: &Py, dataset: &Py, current: &Py, errors: &mut Vec<String>) {
    let law = get(contract, "residual_gold_law");
    if !keys_are(&law, &GOLD_LAW_FIELDS) {
        errors.push("residual_gold_law fields differ from the closed schema".to_string());
        return;
    }
    for field in [
        "law",
        "region_scoped_law",
        "published_result_agreement",
        "what_this_still_permits",
    ] {
        if !nonempty_statement(&get(&law, field)) {
            errors.push(format!(
                "residual_gold_law.{field} must be a non-empty statement"
            ));
        }
    }
    let region_carrier = get(&law, "region_scoped_carrier");
    let declared_carrier = |value: &Py| {
        CARRIER_DECLARATIONS
            .iter()
            .any(|(name, _, _)| eq_str(value, name))
    };
    if !declared_carrier(&region_carrier) {
        errors.push(
            "residual_gold_law.region_scoped_carrier must name a declared carrier".to_string(),
        );
    }
    let dispositions = match get(&law, "required_disposition_values") {
        Py::List(values) if !values.is_empty() && values.iter().all(is_str) => Py::List(values),
        _ => {
            errors.push(
                "residual_gold_law.required_disposition_values must be a non-empty name list"
                    .to_string(),
            );
            return;
        }
    };
    let mut following: Vec<Py> = Vec::new();
    let mut fact_scoped: Vec<(Py, Vec<Py>)> = Vec::new();
    for document in list(&get_or(dataset, "documents", Py::List(Vec::new()))) {
        for cell in list(&get_or(&document, "cells", Py::List(Vec::new()))) {
            let residual = get(&cell, "residual");
            if !is_dict(&residual) || !in_list(&get(&cell, "expected_disposition"), &dispositions) {
                continue;
            }
            let cell_id = get(&cell, "cell_id");
            let derived: BTreeSet<Option<String>> = PROMOTED_STAGES
                .iter()
                .map(|stage| region_scoped_gold_key(&get(&residual, stage)))
                .collect();
            let declared: BTreeSet<Vec<String>> = PROMOTED_STAGES
                .iter()
                .map(|stage| {
                    let query = get_or(&residual, stage, Py::Dict(Vec::new()));
                    sorted(list(&get_or(&query, "expected_keys", Py::List(Vec::new()))))
                        .iter()
                        .map(key)
                        .collect()
                })
                .collect();
            if derived.len() != 1 || declared.len() != 1 {
                errors.push(format!(
                    "residual gold or predicates differ between promoted stages: {}",
                    str_of(&cell_id)
                ));
                continue;
            }
            let expected = derived.into_iter().next().flatten();
            let query = get_or(&residual, PROMOTED_STAGES[0], Py::Dict(Vec::new()));
            let gold = sorted(list(&get_or(&query, "expected_keys", Py::List(Vec::new()))));
            let follows =
                expected.is_some_and(|expected| gold.len() == 1 && eq_str(&gold[0], &expected));
            if follows {
                following.push(cell_id);
            } else {
                match fact_scoped
                    .iter_mut()
                    .find(|(existing, _)| key(existing) == key(&cell_id))
                {
                    Some(entry) => entry.1 = gold,
                    None => fact_scoped.push((cell_id, gold)),
                }
            }
        }
    }
    let following_sorted = sorted(following.clone());
    let declared_region = get(&law, "region_scoped_cells");
    if !matches!(&declared_region, Py::List(items) if eq(&Py::List(sorted(items.clone())), &Py::List(following_sorted.clone())))
    {
        errors.push(format!(
            "residual_gold_law.region_scoped_cells must be exactly the required cells whose gold is their own <region_id>|<family>: {}",
            pyjson::repr(&Py::List(following_sorted))
        ));
    }
    let entries = match get(&law, "fact_scoped_cells") {
        Py::List(entries)
            if entries
                .iter()
                .all(|entry| keys_are(entry, &FACT_SCOPED_FIELDS)) =>
        {
            entries
        }
        _ => {
            errors.push("residual_gold_law.fact_scoped_cells entries must declare cell_id, carrier and keys".to_string());
            return;
        }
    };
    let mut by_id: Vec<(Py, Py)> = Vec::new();
    for entry in &entries {
        let id = get(entry, "cell_id");
        match by_id
            .iter_mut()
            .find(|(existing, _)| key(existing) == key(&id))
        {
            Some(slot) => slot.1 = entry.clone(),
            None => by_id.push((id, entry.clone())),
        }
    }
    let declared_ids: BTreeSet<String> = by_id.iter().map(|(id, _)| key(id)).collect();
    let fact_ids: BTreeSet<String> = fact_scoped.iter().map(|(id, _)| key(id)).collect();
    if by_id.len() != entries.len() || declared_ids != fact_ids {
        let mut difference: Vec<Py> = by_id
            .iter()
            .map(|(id, _)| id.clone())
            .filter(|id| !fact_ids.contains(&key(id)))
            .chain(
                fact_scoped
                    .iter()
                    .map(|(id, _)| id.clone())
                    .filter(|id| !declared_ids.contains(&key(id))),
            )
            .collect();
        difference = sorted(difference);
        errors.push(format!(
            "every required reviewed gold outside the region law must be declared once with the carrier that owns it; undeclared or stale: {}",
            pyjson::repr(&Py::List(difference))
        ));
    }
    for (cell_id, entry) in &by_id {
        if let Some((_, gold)) = fact_scoped.iter().find(|(id, _)| key(id) == key(cell_id))
            && !eq(
                &Py::List(sorted(list(&get(entry, "keys")))),
                &Py::List(gold.clone()),
            )
        {
            errors.push(format!(
                "declared fact-scoped gold no longer matches the dataset: {}",
                str_of(cell_id)
            ));
        }
        let carrier = get(entry, "carrier");
        if !declared_carrier(&carrier) {
            errors.push(format!(
                "fact-scoped carrier is not a declared production carrier: {}",
                str_of(cell_id)
            ));
        } else if eq(&carrier, &region_carrier) {
            errors.push(format!(
                "the region-scoped carrier cannot own a fact-scoped gold: {}",
                str_of(cell_id)
            ));
        }
    }
    validate_published_gold_agreement(dataset, current, errors);
}

/// The whole contract against its three authorities: the errors and the case matrix's totals.
fn validate_contract(
    root: &Path,
    contract: &Py,
    current: &Py,
    retained: &Py,
    dataset: &Py,
) -> (Vec<String>, (usize, usize, usize)) {
    let mut errors = Vec::new();
    if !keys_are(contract, &TOP_LEVEL) {
        return (
            vec!["contract top-level fields differ from the closed schema".to_string()],
            (0, 0, 0),
        );
    }
    if !eq(&get(contract, "schema_version"), &int(1)) {
        errors.push("contract schema_version must be 1".to_string());
    }
    if !eq_str(
        &get(contract, "contract_id"),
        "spec-to-intent-residual-actionability-v1",
    ) {
        errors.push("contract_id changed".to_string());
    }
    if !eq_str(&get(contract, "owner"), "SPEC-TO-INTENT-ALIGNMENT.8a") {
        errors.push("contract owner must remain SPEC-TO-INTENT-ALIGNMENT.8a".to_string());
    }
    if !nonempty_statement(&get(contract, "decision")) {
        errors.push("contract decision must be a non-empty statement".to_string());
    }
    let authorities = get(contract, "authorities");
    if !keys_are(&authorities, &AUTHORITIES) {
        errors.push("authorities must name exactly the six declared inputs".to_string());
    } else if let Py::Dict(pairs) = &authorities {
        for (name, path) in pairs {
            repository_path(
                root,
                path,
                &format!("authorities.{}", pyjson::to_text(name)),
                &mut errors,
            );
        }
    }
    validate_rule_and_grammar(root, contract, &mut errors);
    let totals = validate_cases(contract, &mut errors);
    validate_witness(root, contract, current, &mut errors);
    validate_reproduction(root, contract, &mut errors);
    validate_reconciliation(contract, retained, &mut errors);
    validate_residual_gold_law(contract, dataset, current, &mut errors);
    (errors, totals)
}

fn read_json(root: &Path, path: &str) -> Result<Py, String> {
    let bytes =
        fs::read(root.join(path)).map_err(|error| format!("cannot read {path}: {error}"))?;
    pyjson::loads_bytes(&bytes).map_err(|error| format!("{path} is not JSON: {error}"))
}

struct Inputs {
    contract: Py,
    current: Py,
    retained: Py,
    dataset: Py,
}

fn load(root: &Path) -> Result<Inputs, String> {
    let contract = read_json(root, CONTRACT)?;
    let authorities = get(&contract, "authorities");
    let path = |name: &str| match get(&authorities, name) {
        Py::Str(path) => Ok(pyjson::to_text(&path)),
        _ => Err(format!("the contract's authorities name no {name}")),
    };
    Ok(Inputs {
        current: read_json(root, &path("current_result")?)?,
        retained: read_json(root, &path("retained_chains")?)?,
        dataset: read_json(root, &path("reviewed_dataset")?)?,
        contract,
    })
}

fn run_check(root: &Path) -> i32 {
    let inputs = match load(root) {
        Ok(inputs) => inputs,
        Err(message) => {
            println!("{LABEL}: FAIL: {message}");
            return 1;
        }
    };
    let (errors, (cases, required, met)) = validate_contract(
        root,
        &inputs.contract,
        &inputs.current,
        &inputs.retained,
        &inputs.dataset,
    );
    if !errors.is_empty() {
        for error in &errors {
            println!("{LABEL}: FAIL: {error}");
        }
        return 1;
    }
    let contract = &inputs.contract;
    let witness = get(contract, "witness");
    let field = |name: &str| str_of(&get(&witness, name));
    let count = |holder: &str, name: &str| list(&get(&get(contract, holder), name)).len();
    println!(
        "{LABEL}: {} declaring cells / {} declared = {} actionable + {} not-required + {} required-absent; corrected {}/{}; {cases} rule cases derive {required} required / {met} met; {} typed causes; {} affected chains / {} reviewed stage attempts; {} region-law + {} fact-scoped reviewed golds, published gold in agreement: PASS",
        field("declared_cells"),
        field("declared_observations"),
        field("actionable_observations"),
        field("not_required_observations"),
        field("required_and_absent_observations"),
        field("corrected_met_observations"),
        field("corrected_required_observations"),
        count("record_grammar", "typed_causes"),
        str_of(&get(
            &get(contract, "reconciliation"),
            "affected_chain_count"
        )),
        str_of(&get(
            &get(contract, "reconciliation"),
            "reviewed_replay_attempts"
        )),
        count("residual_gold_law", "region_scoped_cells"),
        count("residual_gold_law", "fact_scoped_cells"),
    );
    0
}

// ---- The self-test: each mutation of the real contract or an authority must be refused ----------------------

/// A path into a parsed value: object members by name, list items by index.
enum Step {
    Field(&'static str),
    Index(usize),
    Case(&'static str),
    Cause(&'static str),
    Cell(&'static str),
}

fn at<'a>(value: &'a mut Py, path: &[Step]) -> &'a mut Py {
    let mut node = value;
    for step in path {
        node = match (step, node) {
            (Step::Field(name), Py::Dict(pairs)) => {
                let name = pyjson::py_str(name);
                &mut pairs
                    .iter_mut()
                    .find(|(key, _)| *key == name)
                    .expect("a fixture field")
                    .1
            }
            (Step::Index(index), Py::List(items)) => &mut items[*index],
            (Step::Case(id), Py::List(items)) => items
                .iter_mut()
                .find(|case| eq_str(&get(case, "case_id"), id))
                .expect("a fixture case"),
            (Step::Cause(id), Py::List(items)) => items
                .iter_mut()
                .find(|cause| eq_str(&get(cause, "cause_id"), id))
                .expect("a fixture cause"),
            (Step::Cell(id), Py::List(documents)) => documents
                .iter_mut()
                .flat_map(|document| match document {
                    Py::Dict(pairs) => pairs
                        .iter_mut()
                        .filter(|(key, _)| *key == pyjson::py_str("cells"))
                        .map(|(_, cells)| cells)
                        .collect::<Vec<_>>(),
                    _ => Vec::new(),
                })
                .flat_map(|cells| match cells {
                    Py::List(items) => items.iter_mut().collect::<Vec<_>>(),
                    _ => Vec::new(),
                })
                .find(|cell| eq_str(&get(cell, "cell_id"), id))
                .expect("a fixture cell"),
            _ => panic!("the fixture path does not fit the value"),
        };
    }
    node
}

fn set_field(value: &mut Py, name: &str, item: Py) {
    if let Py::Dict(pairs) = value {
        let name = pyjson::py_str(name);
        match pairs.iter_mut().find(|(key, _)| *key == name) {
            Some(pair) => pair.1 = item,
            None => pairs.push((name, item)),
        }
    }
}

fn list_mut(value: &mut Py) -> &mut Vec<Py> {
    match value {
        Py::List(items) => items,
        _ => panic!("a fixture list"),
    }
}

type Mutation = (String, Py, Py, Py, Py);

const TOC_CELL: &str =
    "1_0_2025_03_12_risc_v_advanced_interrupt_architecture__01_table_of_contents";
const ANALOG_CELL: &str = "opencapi_25gbps_phy_signaling_spec_1_0__02_analog_channel_loss";

fn set_gold(value: &mut Py, cell_id: &'static str, keys: &[&str]) {
    for stage in PROMOTED_STAGES {
        let cell = at(value, &[Step::Field("documents"), Step::Cell(cell_id)]);
        let query = at(cell, &[Step::Field("residual"), Step::Field(stage)]);
        set_field(query, "expected_keys", str_list(keys));
    }
}

fn mutations(inputs: &Inputs) -> (Vec<Mutation>, Vec<Mutation>) {
    let Inputs {
        contract,
        current,
        retained,
        dataset,
    } = inputs;
    let mut out: Vec<Mutation> = Vec::new();
    let mut contract_case = |name: &str, apply: &dyn Fn(&mut Py)| {
        let mut value = contract.clone();
        apply(&mut value);
        out.push((
            name.to_string(),
            value,
            current.clone(),
            retained.clone(),
            dataset.clone(),
        ));
    };
    let case_field = |id: &'static str, field: &'static str, item: Py| {
        move |value: &mut Py| {
            set_field(
                at(value, &[Step::Field("cases"), Step::Case(id)]),
                field,
                item.clone(),
            )
        }
    };
    contract_case(
        "canonical-miss-leaves-denominator",
        &case_field(
            "canonical_missing_key_without_residual_is_unmet",
            "expected_required_observations",
            int(0),
        ),
    );
    contract_case(
        "canonical-miss-counted-as-success",
        &case_field(
            "canonical_missing_key_without_residual_is_unmet",
            "expected_met_observations",
            int(1),
        ),
    );
    contract_case(
        "exact-canonical-stage-still-required",
        &case_field(
            "canonical_stage_exact_requires_nothing",
            "expected_required_observations",
            int(1),
        ),
    );
    contract_case(
        "duplicate-residual-credited",
        &case_field(
            "residual_duplicating_promoted_canonical_is_refused",
            "expected_met_observations",
            int(1),
        ),
    );
    contract_case(
        "inactionable-residual-credited",
        &case_field(
            "residual_missing_reason_is_unmet",
            "expected_met_observations",
            int(1),
        ),
    );
    contract_case("dropped-actionability-field", &|value: &mut Py| {
        let fields = list_mut(at(
            value,
            &[
                Step::Field("record_grammar"),
                Step::Field("required_actionability_fields"),
            ],
        ));
        if let Some(index) = fields.iter().position(|field| eq_str(field, "/replay")) {
            fields.remove(index);
        }
    });
    contract_case("widened-boundary-vocabulary", &|value: &mut Py| {
        list_mut(at(
            value,
            &[
                Step::Field("record_grammar"),
                Step::Field("first_failing_stage_values"),
            ],
        ))
        .push(text("adapter_lowering"));
    });
    contract_case("invented-typed-cause", &|value: &mut Py| {
        list_mut(at(
            value,
            &[Step::Field("record_grammar"), Step::Field("typed_causes")],
        ))
        .push(
            pyjson::loads(
                "{\"cause_id\": \"unknown\", \"definition\": \"x\", \"existing_carrier\": null}",
            )
            .expect("cause"),
        );
    });
    contract_case("claimed-missing-carrier", &|value: &mut Py| {
        set_field(
            at(
                value,
                &[
                    Step::Field("record_grammar"),
                    Step::Field("typed_causes"),
                    Step::Index(0),
                ],
            ),
            "existing_carrier",
            text("AlreadyShipped"),
        );
    });
    let cause_carrier = |id: &'static str, carrier: Py| {
        move |value: &mut Py| {
            set_field(
                at(
                    value,
                    &[
                        Step::Field("record_grammar"),
                        Step::Field("typed_causes"),
                        Step::Cause(id),
                    ],
                ),
                "existing_carrier",
                carrier.clone(),
            )
        }
    };
    contract_case(
        "claimed-carrier-for-unbuilt-cause",
        &cause_carrier("non_contract_region", text("CapturedRegionResidualRecord")),
    );
    contract_case(
        "renamed-shipped-carrier",
        &cause_carrier(
            "no_canonical_carrier_for_captured_region",
            text("CapturedRegionResidual"),
        ),
    );
    contract_case(
        "disowned-shipped-carrier",
        &cause_carrier("no_canonical_carrier_for_captured_region", Py::None),
    );
    contract_case("missing-control", &|value: &mut Py| {
        list_mut(at(value, &[Step::Field("cases")])).pop();
    });
    let field_at = |path: &'static [&'static str], field: &'static str, item: Py| {
        move |value: &mut Py| {
            let steps: Vec<Step> = path.iter().map(|name| Step::Field(name)).collect();
            set_field(at(value, &steps), field, item.clone())
        }
    };
    contract_case(
        "relaxed-fail-closed-rule",
        &field_at(
            &["requirement_rule"],
            "fail_closed_rule",
            text("best effort"),
        ),
    );
    contract_case(
        "relaxed-duplicate-rule",
        &field_at(
            &["requirement_rule"],
            "duplicate_rule",
            text("duplicates are fine"),
        ),
    );
    contract_case(
        "shrunk-witness",
        &field_at(&["witness"], "required_and_absent_observations", int(10)),
    );
    contract_case(
        "misreported-not-required",
        &field_at(&["witness"], "not_required_observations", int(0)),
    );
    contract_case(
        "stale-witness-identity",
        &field_at(&["witness"], "current_result_sha256", text(&"0".repeat(64))),
    );
    contract_case("widened-selected-family", &|value: &mut Py| {
        let family = at(
            value,
            &[Step::Field("witness"), Step::Field("selected_family")],
        );
        set_field(family, "cells", int(6));
        set_field(family, "observations", int(12));
    });
    contract_case(
        "family-declared-open-after-shipping",
        &field_at(&["witness", "selected_family"], "state", text("open")),
    );
    contract_case(
        "family-state-outside-vocabulary",
        &field_at(&["witness", "selected_family"], "state", text("partial")),
    );
    contract_case(
        "closed-family-names-unshipped-carrier",
        &field_at(
            &["witness", "selected_family"],
            "typed_cause",
            text("non_contract_region"),
        ),
    );
    contract_case("unrepaired-reproduction", &|value: &mut Py| {
        let reproduction = at(value, &[Step::Field("reproduction")]);
        let defective = get(reproduction, "defective_command");
        set_field(reproduction, "repaired_command", defective);
    });
    contract_case("unbound-reproduction-control", &|value: &mut Py| {
        list_mut(at(
            value,
            &[Step::Field("reproduction"), Step::Field("control_tests")],
        ))
        .push(text("tests::absent_control"));
    });
    let mut dataset_case = |name: &str, apply: &dyn Fn(&mut Py)| {
        let mut value = dataset.clone();
        apply(&mut value);
        out.push((
            name.to_string(),
            contract.clone(),
            current.clone(),
            retained.clone(),
            value,
        ));
    };
    dataset_case("reinstated-unproducible-review-label", &|value: &mut Py| {
        set_gold(value, TOC_CELL, &["table_0004|toc_non_contract"])
    });
    dataset_case(
        "region-gold-renamed-to-another-region",
        &|value: &mut Py| set_gold(value, TOC_CELL, &["table_0009|table_of_contents"]),
    );
    dataset_case("region-gold-widened-past-one-record", &|value: &mut Py| {
        set_gold(
            value,
            TOC_CELL,
            &["table_0004|table_of_contents", "table_0004|extra"],
        )
    });
    dataset_case("declared-fact-scoped-gold-drifted", &|value: &mut Py| {
        set_gold(value, ANALOG_CELL, &["IL(f)|max=99|unit=dB"])
    });
    dataset_case("gold-differs-between-promoted-stages", &|value: &mut Py| {
        let cell = at(value, &[Step::Field("documents"), Step::Cell(TOC_CELL)]);
        set_field(
            at(cell, &[Step::Field("residual"), Step::Field("intent_ir")]),
            "expected_keys",
            str_list(&["table_0004|other"]),
        );
    });
    let mut stale_current = current.clone();
    set_gold(
        &mut stale_current,
        TOC_CELL,
        &["table_0004|toc_non_contract"],
    );
    out.push((
        "published-gold-left-behind".to_string(),
        contract.clone(),
        stale_current,
        retained.clone(),
        dataset.clone(),
    ));
    let mut contract_case = |name: &str, apply: &dyn Fn(&mut Py)| {
        let mut value = contract.clone();
        apply(&mut value);
        out.push((
            name.to_string(),
            value,
            current.clone(),
            retained.clone(),
            dataset.clone(),
        ));
    };
    contract_case(
        "region-law-cell-dropped-from-declaration",
        &|value: &mut Py| {
            let cells = list_mut(at(
                value,
                &[
                    Step::Field("residual_gold_law"),
                    Step::Field("region_scoped_cells"),
                ],
            ));
            if let Some(index) = cells.iter().position(|cell| eq_str(cell, TOC_CELL)) {
                cells.remove(index);
            }
        },
    );
    contract_case(
        "fact-scoped-cell-claims-the-region-carrier",
        &|value: &mut Py| {
            let carrier = get(
                at(value, &[Step::Field("residual_gold_law")]),
                "region_scoped_carrier",
            );
            set_field(
                at(
                    value,
                    &[
                        Step::Field("residual_gold_law"),
                        Step::Field("fact_scoped_cells"),
                        Step::Index(0),
                    ],
                ),
                "carrier",
                carrier,
            );
        },
    );
    contract_case(
        "fact-scoped-cell-claims-an-absent-carrier",
        &|value: &mut Py| {
            set_field(
                at(
                    value,
                    &[
                        Step::Field("residual_gold_law"),
                        Step::Field("fact_scoped_cells"),
                        Step::Index(0),
                    ],
                ),
                "carrier",
                text("NeverShippedResidual"),
            );
        },
    );
    contract_case(
        "gold-law-required-dispositions-emptied",
        &field_at(
            &["residual_gold_law"],
            "required_disposition_values",
            Py::List(Vec::new()),
        ),
    );
    contract_case(
        "partial-chain-set",
        &field_at(&["reconciliation"], "affected_chain_count", int(23)),
    );
    contract_case(
        "partial-replay",
        &field_at(&["reconciliation"], "reviewed_replay_attempts", int(47)),
    );
    let mut no_longer_red = current.clone();
    for document in list_mut(at(&mut no_longer_red, &[Step::Field("documents")])) {
        let Py::Dict(fields) = document else { continue };
        for (name, cells) in fields.iter_mut() {
            if *name != pyjson::py_str("cells") {
                continue;
            }
            for cell in list_mut(cells) {
                let residual = get(cell, "residual");
                if !pyjson::truthy(&residual)
                    || eq_str(&get(cell, "expected_disposition"), "canonical")
                {
                    continue;
                }
                let residual = at(cell, &[Step::Field("residual")]);
                set_field(
                    at(residual, &[Step::Field("semantic_ir")]),
                    "false_negatives",
                    int(0),
                );
                set_field(
                    at(residual, &[Step::Field("intent_ir")]),
                    "false_negatives",
                    int(0),
                );
                set_field(residual, "semantic_actionable", Py::Bool(true));
                set_field(residual, "intent_actionable", Py::Bool(true));
            }
        }
    }
    out.push((
        "witness-no-longer-red".to_string(),
        contract.clone(),
        no_longer_red,
        retained.clone(),
        dataset.clone(),
    ));
    let family_name = get(
        &get(&get(contract, "witness"), "selected_family"),
        "semantic_family",
    );
    let mut regressed_current = current.clone();
    let mut regressed = false;
    for document in list_mut(at(&mut regressed_current, &[Step::Field("documents")])) {
        let Py::Dict(fields) = document else { continue };
        for (name, cells) in fields.iter_mut() {
            if *name != pyjson::py_str("cells") {
                continue;
            }
            for cell in list_mut(cells) {
                if regressed
                    || !eq(&get(cell, "semantic_family"), &family_name)
                    || !pyjson::truthy(&get(cell, "residual"))
                {
                    continue;
                }
                for stage in PROMOTED_STAGES {
                    let residual = at(cell, &[Step::Field("residual")]);
                    set_field(
                        at(residual, &[Step::Field(stage)]),
                        "false_negatives",
                        int(1),
                    );
                    let prefix = stage.split('_').next().unwrap_or(stage);
                    set_field(residual, &format!("{prefix}_actionable"), Py::Bool(false));
                }
                let mut failures: Vec<Py> =
                    list(&get_or(cell, "hard_failures", Py::List(Vec::new())));
                if !failures
                    .iter()
                    .any(|failure| eq_str(failure, "required_residual_missing_or_inactionable"))
                {
                    failures.push(text("required_residual_missing_or_inactionable"));
                }
                let mut unique: Vec<Py> = Vec::new();
                for failure in sorted(failures) {
                    if !unique.iter().any(|seen| key(seen) == key(&failure)) {
                        unique.push(failure);
                    }
                }
                set_field(cell, "hard_failures", Py::List(unique));
                regressed = true;
            }
        }
    }
    if regressed {
        out.push((
            "closed-family-leaves-category-gap".to_string(),
            contract.clone(),
            regressed_current,
            retained.clone(),
            dataset.clone(),
        ));
    }
    let mut grown_retained = retained.clone();
    let mut ids: Vec<String> = list(&get(retained, "retained"))
        .iter()
        .map(str_of)
        .collect();
    ids.push("zz_probe_retained_chain".to_string());
    ids.sort_by_key(|item| pyjson::py_str(item));
    set_field(
        &mut grown_retained,
        "retained",
        Py::List(ids.iter().map(|id| text(id)).collect()),
    );
    set_field(
        &mut grown_retained,
        "reclamations",
        pyjson::loads("[{\"document_key\": \"zz_probe_reclaimed_chain\", \"owning_leaf\": \"RETAINED-BUNDLE-POPULATION-FROZEN.1\", \"date\": \"2026-09-19\", \"reason\": \"self-test probe: a recorded reclamation must be admissible\"}]").expect("reclamation"),
    );
    let mut grown_contract = contract.clone();
    set_field(
        at(&mut grown_contract, &[Step::Field("reconciliation")]),
        "affected_chain_count",
        int(ids.len()),
    );
    set_field(
        at(&mut grown_contract, &[Step::Field("reconciliation")]),
        "affected_chain_ids_sha256",
        text(&retained_id_digest(&ids)),
    );
    let admissible = vec![(
        "grown-retained-set-with-reclamation".to_string(),
        grown_contract.clone(),
        current.clone(),
        grown_retained.clone(),
        dataset.clone(),
    )];
    let mut stale_count = grown_contract.clone();
    set_field(
        at(&mut stale_count, &[Step::Field("reconciliation")]),
        "affected_chain_count",
        int(list(&get(retained, "retained")).len()),
    );
    out.push((
        "grown-retained-count-stale".to_string(),
        stale_count,
        current.clone(),
        grown_retained.clone(),
        dataset.clone(),
    ));
    let mut stale_digest = grown_contract;
    let pinned = get(
        &get(contract, "reconciliation"),
        "affected_chain_ids_sha256",
    );
    set_field(
        at(&mut stale_digest, &[Step::Field("reconciliation")]),
        "affected_chain_ids_sha256",
        pinned,
    );
    out.push((
        "grown-retained-digest-stale".to_string(),
        stale_digest,
        current.clone(),
        grown_retained,
        dataset.clone(),
    ));
    if !regressed {
        out.push((
            "could-not-build-the-category-gap-case".to_string(),
            Py::None,
            Py::None,
            Py::None,
            Py::None,
        ));
    }
    (out, admissible)
}

/// The script's self-test, case for case: the real contract passes, each RED mutation is refused, and the
/// mandated retained-set operation is admitted.
fn self_test_outcome(root: &Path) -> Result<(usize, usize), String> {
    let inputs = load(root)?;
    let (baseline, _) = validate_contract(
        root,
        &inputs.contract,
        &inputs.current,
        &inputs.retained,
        &inputs.dataset,
    );
    if !baseline.is_empty() {
        return Err(baseline
            .iter()
            .map(|error| format!("{LABEL}: FAIL baseline: {error}"))
            .collect::<Vec<_>>()
            .join("\n"));
    }
    let (mutations, admissible) = mutations(&inputs);
    if mutations
        .iter()
        .any(|(name, ..)| name == "could-not-build-the-category-gap-case")
    {
        return Err(format!(
            "{LABEL}: FAIL self-test could not build the category-gap case"
        ));
    }
    let rejected: Vec<String> = admissible
        .iter()
        .filter_map(|(name, contract, current, retained, dataset)| {
            let (errors, _) = validate_contract(root, contract, current, retained, dataset);
            errors.first().map(|error| format!("{name}: {error}"))
        })
        .collect();
    if !rejected.is_empty() {
        return Err(format!(
            "{LABEL}: FAIL self-test rejected admissible cases: {}",
            repr_strings(&rejected)
        ));
    }
    let accepted: Vec<String> = mutations
        .iter()
        .filter(|(_, contract, current, retained, dataset)| {
            validate_contract(root, contract, current, retained, dataset)
                .0
                .is_empty()
        })
        .map(|(name, ..)| name.clone())
        .collect();
    if !accepted.is_empty() {
        return Err(format!(
            "{LABEL}: FAIL self-test accepted mutations: {}",
            repr_strings(&accepted)
        ));
    }
    Ok((mutations.len(), admissible.len()))
}

fn run_self_test(root: &Path) -> i32 {
    match self_test_outcome(root) {
        Ok((mutations, admissible)) => {
            println!(
                "{LABEL}: self-test {mutations}/{mutations} denominator, fail-closed, duplicate, actionability, grammar, coverage, witness, family, reproduction, chain, replay, and gold-law RED cases pass, with {admissible}/{admissible} mandated retained-set operations admitted."
            );
            0
        }
        Err(message) => {
            println!("{message}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_test_refuses_every_mutation_and_admits_the_mandated_operation() {
        assert_eq!(self_test_outcome(&crate::repository_root()), Ok((40, 1)));
    }

    #[test]
    fn the_rule_counts_required_and_met_observations() {
        let grammar = pyjson::loads(
            "{\"required_actionability_fields\": [\"/reason\"], \"first_failing_stage_values\": [\"source_to_evidence_ir\"]}",
        )
        .expect("grammar");
        let case = |text: &str| pyjson::loads(text).expect("case");
        let canonical = case(
            "{\"expected_disposition\": \"canonical\", \"canonical_reviewed_keys\": [\"a\", \"b\", \"b\"], \"canonical_promoted_keys\": [\"b\"], \"residual_expected_keys\": [], \"residual_records\": [{\"fact_key\": \"b\", \"source_ids\": [\"s\"], \"reason\": \"r\", \"first_failing_stage\": \"source_to_evidence_ir\"}]}",
        );
        assert_eq!(simulate_case(&canonical, &grammar), (2, 0));
        let residual = case(
            "{\"expected_disposition\": \"residual\", \"canonical_reviewed_keys\": [], \"canonical_promoted_keys\": [], \"residual_expected_keys\": [\"x\"], \"residual_records\": [{\"fact_key\": \"x\", \"source_ids\": [\"s\"], \"reason\": \" \", \"first_failing_stage\": \"source_to_evidence_ir\"}]}",
        );
        assert_eq!(simulate_case(&residual, &grammar), (1, 0));
        // A residual cell whose record duplicates a promoted canonical key earns nothing, though it is otherwise
        // actionable; the contract's case matrix has no such cell, so this pins the branch.
        let duplicate = case(
            "{\"expected_disposition\": \"residual\", \"canonical_reviewed_keys\": [], \"canonical_promoted_keys\": [\"x\"], \"residual_expected_keys\": [\"x\"], \"residual_records\": [{\"fact_key\": \"x\", \"source_ids\": [\"s\"], \"reason\": \"r\", \"first_failing_stage\": \"source_to_evidence_ir\"}]}",
        );
        assert_eq!(simulate_case(&duplicate, &grammar), (1, 0));
        let honest = case(
            "{\"expected_disposition\": \"residual\", \"canonical_reviewed_keys\": [], \"canonical_promoted_keys\": [], \"residual_expected_keys\": [\"x\"], \"residual_records\": [{\"fact_key\": \"x\", \"source_ids\": [\"s\"], \"reason\": \"r\", \"first_failing_stage\": \"source_to_evidence_ir\"}]}",
        );
        assert_eq!(simulate_case(&honest, &grammar), (1, 1));
    }

    #[test]
    fn a_region_query_determines_its_own_gold_key() {
        let query = |text: &str| pyjson::loads(text).expect("query");
        let pinned = query(
            "{\"predicates\": [{\"operator\": \"equals\", \"field\": \"/region_id\", \"value\": \"t1\"}, {\"operator\": \"equals\", \"field\": \"/family\", \"value\": \"toc\"}]}",
        );
        assert_eq!(region_scoped_gold_key(&pinned).as_deref(), Some("t1|toc"));
        let extra = query(
            "{\"predicates\": [{\"operator\": \"equals\", \"field\": \"/region_id\", \"value\": \"t1\"}]}",
        );
        assert_eq!(region_scoped_gold_key(&extra), None);
    }

    #[test]
    fn python_hash_keys_and_ordering_agree_with_python() {
        let parse = |text: &str| pyjson::loads(text).expect("value");
        assert_eq!(key(&parse("1")), key(&parse("1.0")));
        assert_eq!(key(&parse("1")), key(&parse("true")));
        assert_ne!(key(&parse("1")), key(&parse("\"1\"")));
        assert_eq!(
            pyjson::repr(&Py::List(sorted(vec![text("b"), text("B"), text("a")]))),
            "['B', 'a', 'b']"
        );
    }
}
