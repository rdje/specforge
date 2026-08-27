#!/usr/bin/env python3
"""Validate the frozen SPEC-TO-INTENT-ALIGNMENT.8a required-residual contract.

This checker owns the measurement and record design, not extraction behavior. It executes the
required-residual rule over a closed case matrix, joins the frozen decomposition to the tracked
current result, and binds the repaired reproduction command to its published authorities.
SPEC-TO-INTENT-ALIGNMENT.8b must make the vertical evaluator agree with the same rule;
SPEC-TO-INTENT-ALIGNMENT.8c must emit the typed record; SPEC-TO-INTENT-ALIGNMENT.8d must replay
the declared population.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
CONTRACT_PATH = Path("doctrine/spec_to_intent/residual_actionability_contract.json")

EXPECTED_TOP_LEVEL = {
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
}
EXPECTED_AUTHORITIES = {
    "reviewed_dataset",
    "current_result",
    "retained_chains",
    "population_replay",
    "controller_input",
    "controller_report",
}
EXPECTED_RULE_FIELDS = {
    "observation_unit",
    "promoted_stages",
    "residual_disposition_rule",
    "canonical_disposition_rule",
    "fail_closed_rule",
    "duplicate_rule",
    "met_rule",
    "stage_independence",
}
EXPECTED_GRAMMAR_FIELDS = {
    "collection_pointer",
    "identity_fields",
    "provenance_field",
    "required_actionability_fields",
    "first_failing_stage_values",
    "typed_causes",
    "authority_rule",
    "no_fabrication_rule",
}
EXPECTED_CASE_FIELDS = {
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
}
EXPECTED_RECORD_FIELDS = {"fact_key", "source_ids", "reason", "first_failing_stage", "replay"}
# Which typed causes a production carrier already exists for, and what that carrier is called.
# `None` means the cause is declared but not yet built. SPEC-TO-INTENT-ALIGNMENT.8c shipped the
# captured-region carrier, so its entry moved from `None` to the type it ships.
EXPECTED_TYPED_CAUSE_CARRIERS = {
    "outside_executable_digital_domain": "TimingIntentDisposition::NonApplicable",
    "no_canonical_carrier_for_captured_region": "CapturedRegionResidualRecord",
    "non_contract_region": None,
    "unresolved_grounding": None,
}
# A claimed carrier has to be findable in production source, so "a carrier exists" is a checked
# fact rather than a sentence in a JSON file — in both directions. A slice cannot claim coverage it
# never built, and a later slice cannot delete a shipped carrier while the contract still cites it.
CARRIER_DECLARATIONS = {
    "TimingIntentDisposition::NonApplicable": (
        "crates/specforge/src/ir/source.rs",
        "pub enum TimingIntentDisposition",
    ),
    "CapturedRegionResidualRecord": (
        "crates/specforge/src/ir/source.rs",
        "pub struct CapturedRegionResidualRecord",
    ),
}
EXPECTED_PROMOTED_STAGES = ["semantic_ir", "intent_ir"]
EXPECTED_ACTIONABILITY_FIELDS = ["/reason", "/first_failing_stage", "/replay"]
EXPECTED_BOUNDARIES = [
    "source_to_evidence_ir",
    "evidence_to_semantic_ir",
    "semantic_to_intent_ir",
]
EXPECTED_TYPED_CAUSES = [
    "no_canonical_carrier_for_captured_region",
    "non_contract_region",
    "outside_executable_digital_domain",
    "unresolved_grounding",
]
REQUIRED_CONTROLS = {
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
}
EXPECTED_CASE_TOTALS = (18, 17, 5)
EXPECTED_AFFECTED_STAGES = ["semantic_ir", "intent_ir", "isf_adapter"]
EXPECTED_REPLAY_STAGES = ["source_ir", "evidence_ir", "semantic_ir", "intent_ir"]
EXPECTED_REPRODUCTION_FIELDS = {
    "gap_id",
    "defective_command",
    "repaired_command",
    "defect",
    "control_producer",
    "control_tests",
    "published_authorities",
}


def read_json(path: Path) -> Any:
    with (ROOT / path).open(encoding="utf-8") as stream:
        return json.load(stream)


def read_text(path: Path) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def repository_path(value: Any, label: str, errors: list[str]) -> None:
    if not isinstance(value, str) or not value:
        errors.append(f"{label} must be a non-empty repository-relative path")
        return
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        errors.append(f"{label} must be repository-relative without parent traversal: {value}")
        return
    if not (ROOT / path).is_file():
        errors.append(f"{label} does not exist as a tracked authority: {value}")


def retained_id_digest(ids: list[str]) -> str:
    payload = "".join(f"{identifier}\n" for identifier in sorted(ids)).encode()
    return hashlib.sha256(payload).hexdigest()


def record_is_actionable(record: dict[str, Any], grammar: dict[str, Any]) -> bool:
    """Executable definition of an actionable residual record."""
    source_ids = record.get("source_ids")
    if not isinstance(source_ids, list) or not source_ids:
        return False
    for pointer in grammar.get("required_actionability_fields", []):
        value = record.get(str(pointer).lstrip("/"))
        if not isinstance(value, str) or not value.strip():
            return False
    return record.get("first_failing_stage") in grammar.get("first_failing_stage_values", [])


def simulate_case(case: dict[str, Any], grammar: dict[str, Any]) -> tuple[int, int]:
    """Executable definition of the required-residual rule for one cell at one promoted stage."""
    records = [record for record in case["residual_records"] if isinstance(record, dict)]
    promoted = Counter(case["canonical_promoted_keys"])
    duplicated = any(record.get("fact_key") in promoted for record in records)

    if case["expected_disposition"] != "canonical":
        exact = Counter(record.get("fact_key") for record in records) == Counter(
            case["residual_expected_keys"]
        )
        met = int(
            exact
            and bool(records)
            and not duplicated
            and all(record_is_actionable(record, grammar) for record in records)
        )
        return 1, met

    missing = Counter(case["canonical_reviewed_keys"]) - promoted
    required = sum(missing.values())
    if duplicated:
        return required, 0
    explained = 0
    for key, count in missing.items():
        usable = [
            record
            for record in records
            if record.get("fact_key") == key and record_is_actionable(record, grammar)
        ]
        explained += min(count, len(usable))
    return required, explained


def validate_rule_and_grammar(contract: dict[str, Any], errors: list[str]) -> None:
    rule = contract.get("requirement_rule")
    if not isinstance(rule, dict) or set(rule) != EXPECTED_RULE_FIELDS:
        errors.append("requirement_rule fields differ from the closed schema")
    else:
        if rule.get("promoted_stages") != EXPECTED_PROMOTED_STAGES:
            errors.append("requirement_rule must count exactly the two promoted stages")
        for field in EXPECTED_RULE_FIELDS - {"promoted_stages"}:
            if not isinstance(rule.get(field), str) or not rule[field].strip():
                errors.append(f"requirement_rule.{field} must be a non-empty statement")
        if "never leave the denominator" not in rule.get("fail_closed_rule", ""):
            errors.append("fail_closed_rule must keep a missing canonical key inside the denominator")
        if "no met observation at that stage" not in rule.get("duplicate_rule", ""):
            errors.append("duplicate_rule must refuse credit for a stage carrying a duplicate residual")

    grammar = contract.get("record_grammar")
    if not isinstance(grammar, dict) or set(grammar) != EXPECTED_GRAMMAR_FIELDS:
        errors.append("record_grammar fields differ from the closed schema")
        return
    if grammar.get("collection_pointer") != "/residuals":
        errors.append("record_grammar must keep the reviewed /residuals collection pointer")
    if grammar.get("identity_fields") != ["/region_id", "/family", "/fact_key"]:
        errors.append("record_grammar identity must remain region, family, and fact key")
    if grammar.get("provenance_field") != "/source_ids":
        errors.append("record_grammar provenance field must remain /source_ids")
    if grammar.get("required_actionability_fields") != EXPECTED_ACTIONABILITY_FIELDS:
        errors.append("record_grammar must require exactly reason, first failing stage, and replay")
    if grammar.get("first_failing_stage_values") != EXPECTED_BOUNDARIES:
        errors.append("record_grammar boundaries must be the three source-to-intent promotions")
    causes = grammar.get("typed_causes")
    if not isinstance(causes, list) or len(causes) != len(EXPECTED_TYPED_CAUSES):
        errors.append("record_grammar must declare the four closed typed causes")
        return
    observed = sorted(str(cause.get("cause_id")) for cause in causes if isinstance(cause, dict))
    if observed != EXPECTED_TYPED_CAUSES:
        errors.append(f"typed causes differ from the frozen closed set: {observed}")
    for cause in causes:
        if not isinstance(cause, dict) or set(cause) != {"cause_id", "definition", "existing_carrier"}:
            errors.append("each typed cause must declare id, definition, and existing carrier")
            continue
        if not isinstance(cause.get("definition"), str) or not cause["definition"].strip():
            errors.append(f"typed cause {cause.get('cause_id')} needs a definition")
    carriers = {
        cause.get("cause_id"): cause.get("existing_carrier")
        for cause in causes
        if isinstance(cause, dict)
    }
    for cause_id, expected_carrier in EXPECTED_TYPED_CAUSE_CARRIERS.items():
        declared = carriers.get(cause_id)
        if declared != expected_carrier:
            errors.append(
                f"typed cause {cause_id} must declare carrier {expected_carrier!r}, not {declared!r}"
            )
            continue
        if declared is None:
            continue
        declaration = CARRIER_DECLARATIONS.get(declared)
        if declaration is None:
            errors.append(f"typed cause {cause_id} names an unknown carrier {declared!r}")
            continue
        path, signature = declaration
        source = ROOT / path
        if not source.is_file() or signature not in source.read_text(encoding="utf-8"):
            errors.append(
                f"typed cause {cause_id} claims carrier {declared!r}, "
                f"but {signature!r} is absent from {path}"
            )


def validate_cases(contract: dict[str, Any], errors: list[str]) -> tuple[int, int, int]:
    grammar = contract.get("record_grammar")
    cases = contract.get("cases")
    if not isinstance(grammar, dict) or not isinstance(cases, list) or not cases:
        errors.append("cases must be a non-empty array validated against the record grammar")
        return (0, 0, 0)
    seen: set[str] = set()
    controls: set[str] = set()
    required_total = 0
    met_total = 0
    for index, case in enumerate(cases):
        label = f"cases[{index}]"
        if not isinstance(case, dict) or set(case) != EXPECTED_CASE_FIELDS:
            errors.append(f"{label} fields differ from the closed case schema")
            continue
        case_id = case.get("case_id")
        if not isinstance(case_id, str) or not case_id:
            errors.append(f"{label}.case_id must be a non-empty string")
            continue
        if case_id in seen:
            errors.append(f"duplicate case_id: {case_id}")
        seen.add(case_id)
        case_controls = case.get("control_classes")
        if not isinstance(case_controls, list) or not case_controls:
            errors.append(f"{case_id} must declare at least one control class")
        else:
            controls.update(str(control) for control in case_controls)
        if case.get("expected_disposition") not in {"canonical", "residual", "non_applicable"}:
            errors.append(f"{case_id} has an unknown reviewed disposition")
            continue
        if case.get("stage") not in EXPECTED_PROMOTED_STAGES:
            errors.append(f"{case_id} must be scored at one promoted stage")
            continue
        for field in [
            "canonical_reviewed_keys",
            "canonical_promoted_keys",
            "residual_expected_keys",
        ]:
            if not isinstance(case.get(field), list):
                errors.append(f"{case_id}.{field} must be an array")
        records = case.get("residual_records")
        if not isinstance(records, list):
            errors.append(f"{case_id}.residual_records must be an array")
            continue
        for record in records:
            if not isinstance(record, dict) or set(record) != EXPECTED_RECORD_FIELDS:
                errors.append(f"{case_id} residual record differs from the typed record schema")
        if case.get("expected_disposition") != "canonical" and case.get("canonical_reviewed_keys"):
            errors.append(f"{case_id} non-canonical cells must not declare canonical gold")
        promoted = set(case.get("canonical_promoted_keys", []))
        if not promoted <= set(case.get("canonical_reviewed_keys", [])):
            errors.append(f"{case_id} promotes a key the review never declared")

        required, met = simulate_case(case, grammar)
        if case.get("expected_required_observations") != required:
            errors.append(
                f"{case_id} declares {case.get('expected_required_observations')} required "
                f"observations; the rule derives {required}"
            )
        if case.get("expected_met_observations") != met:
            errors.append(
                f"{case_id} declares {case.get('expected_met_observations')} met observations; "
                f"the rule derives {met}"
            )
        if met > required:
            errors.append(f"{case_id} cannot meet more observations than it requires")
        required_total += required
        met_total += met

    missing_controls = sorted(REQUIRED_CONTROLS - controls)
    unknown_controls = sorted(controls - REQUIRED_CONTROLS)
    if missing_controls:
        errors.append(f"case matrix is missing controls: {missing_controls}")
    if unknown_controls:
        errors.append(f"case matrix has undeclared controls: {unknown_controls}")
    totals = (len(cases), required_total, met_total)
    if totals != EXPECTED_CASE_TOTALS:
        errors.append(
            f"case denominator must remain {EXPECTED_CASE_TOTALS} cases/required/met; got {totals}"
        )
    return totals


def decompose_current_result(current: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    """Re-derive the declared/required/actionable partition from the tracked current result."""
    buckets: dict[str, list[dict[str, Any]]] = {
        "actionable": [],
        "not_required": [],
        "required_absent": [],
    }
    for document in current.get("documents", []):
        for cell in document.get("cells", []):
            residual = cell.get("residual")
            if not isinstance(residual, dict):
                continue
            canonical = cell.get("canonical")
            canonical_exact = isinstance(canonical, dict) and all(
                canonical.get(stage, {}).get("false_positives") == 0
                and canonical.get(stage, {}).get("false_negatives") == 0
                and canonical.get(stage, {}).get("unprovenanced_records") == 0
                for stage in ["evidence_ir", "semantic_ir", "intent_ir"]
            )
            met = all(
                residual.get(stage, {}).get("false_positives") == 0
                and residual.get(stage, {}).get("false_negatives") == 0
                and residual.get(stage, {}).get("unprovenanced_records") == 0
                and residual.get(f"{stage.split('_')[0]}_actionable") is True
                for stage in EXPECTED_PROMOTED_STAGES
            )
            row = {
                "document_key": document.get("document_key"),
                "cell_id": cell.get("cell_id"),
                "category": document.get("category"),
                "semantic_family": cell.get("semantic_family"),
                "modality": cell.get("modality"),
                "expected_disposition": cell.get("expected_disposition"),
                "reviewed_residual_keys": sorted(
                    residual.get("semantic_ir", {}).get("expected_keys", [])
                ),
            }
            if met:
                buckets["actionable"].append(row)
            elif cell.get("expected_disposition") == "canonical" and canonical_exact:
                row["canonical_exact_at_every_promoted_stage"] = True
                buckets["not_required"].append(row)
            else:
                row["hard_failures"] = sorted(cell.get("hard_failures", []))
                buckets["required_absent"].append(row)
    for rows in buckets.values():
        rows.sort(key=lambda row: (row["document_key"], row["cell_id"]))
    return buckets


def validate_witness(contract: dict[str, Any], current: dict[str, Any], errors: list[str]) -> None:
    witness = contract.get("witness")
    if not isinstance(witness, dict):
        errors.append("witness must be an object")
        return
    authorities = contract.get("authorities", {})
    result_path = authorities.get("current_result")
    if isinstance(result_path, str) and (ROOT / result_path).is_file():
        digest = hashlib.sha256((ROOT / result_path).read_bytes()).hexdigest()
        if witness.get("current_result_sha256") != digest:
            errors.append("witness no longer pins the tracked current-result identity")
    buckets = decompose_current_result(current)
    counts = {name: len(rows) for name, rows in buckets.items()}
    declared_cells = sum(counts.values())
    expected = {
        "declared_cells": declared_cells,
        "declared_observations": 2 * declared_cells,
        "actionable_observations": 2 * counts["actionable"],
        "not_required_observations": 2 * counts["not_required"],
        "required_and_absent_observations": 2 * counts["required_absent"],
        "corrected_required_observations": 2 * (counts["actionable"] + counts["required_absent"]),
        "corrected_met_observations": 2 * counts["actionable"],
    }
    for field, value in expected.items():
        if witness.get(field) != value:
            errors.append(f"witness.{field} is {witness.get(field)}; the current result derives {value}")
    for field, rows in [
        ("actionable_cells", buckets["actionable"]),
        ("not_required_cells", buckets["not_required"]),
        ("required_and_absent_cells", buckets["required_absent"]),
    ]:
        if witness.get(field) != rows:
            errors.append(f"witness.{field} differs from the current-result derivation")
    if expected["declared_observations"] != (
        expected["actionable_observations"]
        + expected["not_required_observations"]
        + expected["required_and_absent_observations"]
    ):
        errors.append("the declared decomposition does not partition the declared observations")
    if any("required_residual_missing_or_inactionable" not in row.get("hard_failures", [])
           for row in buckets["required_absent"]):
        errors.append("every required-and-absent cell must carry the residual hard failure")
    if any(row.get("hard_failures") for row in buckets["not_required"]):
        errors.append("a not-required cell must not carry a hard failure")

    family = witness.get("selected_family")
    if not isinstance(family, dict) or set(family) != {
        "semantic_family",
        "modality",
        "category",
        "cells",
        "observations",
        "typed_cause",
        "selection_reason",
        "owner",
    }:
        errors.append("selected_family must declare the closed selection schema")
        return
    selected = [
        row
        for row in buckets["required_absent"]
        if row["semantic_family"] == family.get("semantic_family")
    ]
    if len(selected) != family.get("cells") or 2 * len(selected) != family.get("observations"):
        errors.append("selected family cell/observation counts differ from the current result")
    if any(row["modality"] != family.get("modality") for row in selected):
        errors.append("selected family spans more than its declared modality")
    if any(row["category"] != family.get("category") for row in selected):
        errors.append("selected family spans more than its declared category")
    if family.get("typed_cause") not in EXPECTED_TYPED_CAUSES:
        errors.append("selected family must name a declared typed cause")
    if family.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.8c":
        errors.append("the bounded production family must be owned by .8c")
    category_cells = [
        row for row in buckets["required_absent"] if row["category"] == family.get("category")
    ]
    if len(category_cells) != len(selected):
        errors.append("the selected family must be the only residual gap left in its category")


def validate_reproduction(contract: dict[str, Any], errors: list[str]) -> None:
    reproduction = contract.get("reproduction")
    if not isinstance(reproduction, dict) or set(reproduction) != EXPECTED_REPRODUCTION_FIELDS:
        errors.append("reproduction fields differ from the closed schema")
        return
    defective = reproduction.get("defective_command")
    repaired = reproduction.get("repaired_command")
    if not isinstance(defective, str) or not isinstance(repaired, str) or defective == repaired:
        errors.append("reproduction must contrast a distinct defective and repaired command")
        return
    if "-p specforge " not in f"{defective} " or "-p specforge-conformance" not in repaired:
        errors.append("the repaired command must move the gap onto the owning conformance package")
    producer = reproduction.get("control_producer")
    repository_path(producer, "reproduction.control_producer", errors)
    if isinstance(producer, str) and (ROOT / producer).is_file():
        source = read_text(Path(producer))
        if repaired not in source:
            errors.append("the repaired command is absent from the control producer")
        test_plane = source.find("#[cfg(test)]")
        if test_plane < 0:
            errors.append("the control producer declares no test plane")
        elif source.find(defective) != -1 and source.find(defective) < test_plane:
            errors.append("the defective command survives in composition code, not only as a RED fixture")
        for test in reproduction.get("control_tests", []):
            name = str(test).rsplit("::", 1)[-1]
            if f"fn {name}(" not in source:
                errors.append(f"control test '{name}' is absent from the control producer")
    authority_paths = reproduction.get("published_authorities")
    if not isinstance(authority_paths, list) or not authority_paths:
        errors.append("reproduction must publish its authority set")
        return
    for path in authority_paths:
        repository_path(path, "reproduction.published_authorities", errors)
        if not isinstance(path, str) or not (ROOT / path).is_file():
            continue
        text = read_text(Path(path))
        if repaired not in text:
            errors.append(f"published authority '{path}' does not carry the repaired command")
        if defective in text:
            errors.append(f"published authority '{path}' still carries the defective command")


def validate_reconciliation(
    contract: dict[str, Any], retained: dict[str, Any], errors: list[str]
) -> None:
    reconciliation = contract.get("reconciliation")
    if not isinstance(reconciliation, dict):
        errors.append("reconciliation must be an object")
        return
    authorities = contract.get("authorities", {})
    if reconciliation.get("affected_chain_contract") != authorities.get("retained_chains"):
        errors.append("affected-chain contract must be the declared retained-chain authority")
    ids = retained.get("retained")
    if not isinstance(ids, list) or retained.get("reclamations") != []:
        errors.append("retained-chain authority must remain a clean retained set")
        ids = []
    if reconciliation.get("affected_chain_count") != len(ids) or len(ids) != 24:
        errors.append("affected-chain denominator must be the exact 24 retained chains")
    if reconciliation.get("affected_chain_ids_sha256") != retained_id_digest([str(i) for i in ids]):
        errors.append("affected-chain exact-set digest differs from retained authority")
    if reconciliation.get("affected_stages") != EXPECTED_AFFECTED_STAGES:
        errors.append("affected stages must cover SemanticIR through the ISF adapter")
    if reconciliation.get("reviewed_replay_stages") != EXPECTED_REPLAY_STAGES:
        errors.append("reviewed replay must retain the four isolated source-to-intent stages")
    documents = reconciliation.get("reviewed_population_documents")
    attempts = reconciliation.get("reviewed_replay_attempts")
    if documents != 12 or attempts != 48 or attempts != documents * len(EXPECTED_REPLAY_STAGES):
        errors.append("reviewed replay denominator must remain 12 sources x 4 stages = 48")
    for field, owner in [
        ("measurement_owner", "SPEC-TO-INTENT-ALIGNMENT.8b"),
        ("production_owner", "SPEC-TO-INTENT-ALIGNMENT.8c"),
        ("publication_owner", "SPEC-TO-INTENT-ALIGNMENT.8d"),
    ]:
        if reconciliation.get(field) != owner:
            errors.append(f"reconciliation.{field} must remain {owner}")
    invariants = reconciliation.get("publication_invariants")
    if not isinstance(invariants, list) or len(invariants) < 5:
        errors.append("publication invariants must remain complete")


def validate_contract(
    contract: dict[str, Any], current: dict[str, Any], retained: dict[str, Any]
) -> tuple[list[str], tuple[int, int, int]]:
    errors: list[str] = []
    if not isinstance(contract, dict) or set(contract) != EXPECTED_TOP_LEVEL:
        return (["contract top-level fields differ from the closed schema"], (0, 0, 0))
    if contract.get("schema_version") != 1:
        errors.append("contract schema_version must be 1")
    if contract.get("contract_id") != "spec-to-intent-residual-actionability-v1":
        errors.append("contract_id changed")
    if contract.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.8a":
        errors.append("contract owner must remain SPEC-TO-INTENT-ALIGNMENT.8a")
    if not isinstance(contract.get("decision"), str) or not contract["decision"].strip():
        errors.append("contract decision must be a non-empty statement")
    authorities = contract.get("authorities")
    if not isinstance(authorities, dict) or set(authorities) != EXPECTED_AUTHORITIES:
        errors.append("authorities must name exactly the six declared inputs")
    else:
        for key, path in authorities.items():
            repository_path(path, f"authorities.{key}", errors)

    validate_rule_and_grammar(contract, errors)
    totals = validate_cases(contract, errors)
    validate_witness(contract, current, errors)
    validate_reproduction(contract, errors)
    validate_reconciliation(contract, retained, errors)
    return errors, totals


def load_authorities(contract: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any]]:
    authorities = contract["authorities"]
    return (
        read_json(Path(authorities["current_result"])),
        read_json(Path(authorities["retained_chains"])),
    )


def run_check() -> int:
    contract = read_json(CONTRACT_PATH)
    current, retained = load_authorities(contract)
    errors, (cases, required, met) = validate_contract(contract, current, retained)
    if errors:
        for error in errors:
            print(f"residual-actionability-contract: FAIL: {error}")
        return 1
    witness = contract["witness"]
    print(
        "residual-actionability-contract: "
        f"{witness['declared_cells']} declaring cells / {witness['declared_observations']} declared "
        f"= {witness['actionable_observations']} actionable + {witness['not_required_observations']} "
        f"not-required + {witness['required_and_absent_observations']} required-absent; corrected "
        f"{witness['corrected_met_observations']}/{witness['corrected_required_observations']}; "
        f"{cases} rule cases derive {required} required / {met} met; "
        f"{len(contract['record_grammar']['typed_causes'])} typed causes; "
        f"{contract['reconciliation']['affected_chain_count']} affected chains / "
        f"{contract['reconciliation']['reviewed_replay_attempts']} reviewed stage attempts: PASS"
    )
    return 0


def run_self_test() -> int:
    contract = read_json(CONTRACT_PATH)
    current, retained = load_authorities(contract)
    baseline_errors, _ = validate_contract(contract, current, retained)
    if baseline_errors:
        for error in baseline_errors:
            print(f"residual-actionability-contract: FAIL baseline: {error}")
        return 1

    mutations: list[tuple[str, dict[str, Any], dict[str, Any], dict[str, Any]]] = []

    def mutate(name: str, apply) -> None:
        value = copy.deepcopy(contract)
        apply(value)
        mutations.append((name, value, current, retained))

    def case_by_id(value: dict[str, Any], case_id: str) -> dict[str, Any]:
        return next(case for case in value["cases"] if case["case_id"] == case_id)

    mutate(
        "canonical-miss-leaves-denominator",
        lambda value: case_by_id(value, "canonical_missing_key_without_residual_is_unmet").update(
            {"expected_required_observations": 0}
        ),
    )
    mutate(
        "canonical-miss-counted-as-success",
        lambda value: case_by_id(value, "canonical_missing_key_without_residual_is_unmet").update(
            {"expected_met_observations": 1}
        ),
    )
    mutate(
        "exact-canonical-stage-still-required",
        lambda value: case_by_id(value, "canonical_stage_exact_requires_nothing").update(
            {"expected_required_observations": 1}
        ),
    )
    mutate(
        "duplicate-residual-credited",
        lambda value: case_by_id(value, "residual_duplicating_promoted_canonical_is_refused").update(
            {"expected_met_observations": 1}
        ),
    )
    mutate(
        "inactionable-residual-credited",
        lambda value: case_by_id(value, "residual_missing_reason_is_unmet").update(
            {"expected_met_observations": 1}
        ),
    )
    mutate(
        "dropped-actionability-field",
        lambda value: value["record_grammar"]["required_actionability_fields"].remove("/replay"),
    )
    mutate(
        "widened-boundary-vocabulary",
        lambda value: value["record_grammar"]["first_failing_stage_values"].append("adapter_lowering"),
    )
    mutate(
        "invented-typed-cause",
        lambda value: value["record_grammar"]["typed_causes"].append(
            {"cause_id": "unknown", "definition": "x", "existing_carrier": None}
        ),
    )
    mutate(
        "claimed-missing-carrier",
        lambda value: value["record_grammar"]["typed_causes"][0].update(
            {"existing_carrier": "AlreadyShipped"}
        ),
    )

    def carrier_by_id(value: dict[str, Any], cause_id: str) -> dict[str, Any]:
        return next(
            cause
            for cause in value["record_grammar"]["typed_causes"]
            if cause["cause_id"] == cause_id
        )

    mutate(
        "claimed-carrier-for-unbuilt-cause",
        lambda value: carrier_by_id(value, "non_contract_region").update(
            {"existing_carrier": "CapturedRegionResidualRecord"}
        ),
    )
    mutate(
        "renamed-shipped-carrier",
        lambda value: carrier_by_id(value, "no_canonical_carrier_for_captured_region").update(
            {"existing_carrier": "CapturedRegionResidual"}
        ),
    )
    mutate(
        "disowned-shipped-carrier",
        lambda value: carrier_by_id(value, "no_canonical_carrier_for_captured_region").update(
            {"existing_carrier": None}
        ),
    )
    mutate("missing-control", lambda value: value["cases"].pop())
    mutate(
        "relaxed-fail-closed-rule",
        lambda value: value["requirement_rule"].update({"fail_closed_rule": "best effort"}),
    )
    mutate(
        "relaxed-duplicate-rule",
        lambda value: value["requirement_rule"].update({"duplicate_rule": "duplicates are fine"}),
    )
    mutate("shrunk-witness", lambda value: value["witness"].update({"required_and_absent_observations": 10}))
    mutate("misreported-not-required", lambda value: value["witness"].update({"not_required_observations": 0}))
    mutate("stale-witness-identity", lambda value: value["witness"].update({"current_result_sha256": "0" * 64}))
    mutate(
        "widened-selected-family",
        lambda value: value["witness"]["selected_family"].update({"cells": 6, "observations": 12}),
    )
    mutate(
        "unrepaired-reproduction",
        lambda value: value["reproduction"].update(
            {"repaired_command": value["reproduction"]["defective_command"]}
        ),
    )
    mutate(
        "unbound-reproduction-control",
        lambda value: value["reproduction"]["control_tests"].append("tests::absent_control"),
    )
    mutate("partial-chain-set", lambda value: value["reconciliation"].update({"affected_chain_count": 23}))
    mutate("partial-replay", lambda value: value["reconciliation"].update({"reviewed_replay_attempts": 47}))

    mutated_current = copy.deepcopy(current)
    for document in mutated_current["documents"]:
        for cell in document["cells"]:
            residual = cell.get("residual")
            if residual and cell.get("expected_disposition") != "canonical":
                residual["semantic_ir"]["false_negatives"] = 0
                residual["intent_ir"]["false_negatives"] = 0
                residual["semantic_actionable"] = True
                residual["intent_actionable"] = True
    mutations.append(("witness-no-longer-red", contract, mutated_current, retained))

    failures = [
        name
        for name, mutated_contract, mutated_current_result, mutated_retained in mutations
        if not validate_contract(mutated_contract, mutated_current_result, mutated_retained)[0]
    ]
    if failures:
        print(f"residual-actionability-contract: FAIL self-test accepted mutations: {failures}")
        return 1
    print(
        "residual-actionability-contract: self-test "
        f"{len(mutations)}/{len(mutations)} denominator, fail-closed, duplicate, actionability, "
        "grammar, coverage, witness, family, reproduction, chain, and replay RED cases pass."
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    return run_self_test() if args.self_test else run_check()


if __name__ == "__main__":
    raise SystemExit(main())
