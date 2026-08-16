#!/usr/bin/env python3
"""Validate the frozen SPEC-TO-INTENT-ALIGNMENT.7a recovery contract.

This checker owns design currency, not extraction behavior. It joins the exact reviewed/current
witness, the closed grammar-control matrix, the retained-chain denominator, and the complete
reviewed replay obligation. SPEC-TO-INTENT-ALIGNMENT.7b must execute the same cases against the
production extractor; SPEC-TO-INTENT-ALIGNMENT.7c must execute the declared population replay.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
CONTRACT_PATH = Path("doctrine/spec_to_intent/canonical_recovery_contract.json")

EXPECTED_TOP_LEVEL = {
    "schema_version",
    "contract_id",
    "owner",
    "decision",
    "authorities",
    "witness",
    "grammar",
    "polarity_matrix",
    "cases",
    "reconciliation",
}
EXPECTED_MARKERS = {
    "which means that",
    "which means",
    "which implies that",
    "which implies",
    "meaning that",
}
EXPECTED_POLARITY = {
    ("must_be_asserted", "active_high"): "must_be_high",
    ("must_be_asserted", "active_low"): "must_be_low",
    ("must_be_deasserted", "active_high"): "must_be_low",
    ("must_be_deasserted", "active_low"): "must_be_high",
    ("must_be_asserted", "unknown"): "must_be_asserted",
    ("must_be_deasserted", "unknown"): "must_be_deasserted",
}
REQUIRED_CONTROLS = {
    "exact_witness",
    "marker:which_means_that",
    "marker:which_means",
    "marker:which_implies_that",
    "marker:which_implies",
    "marker:meaning_that",
    "state:asserted",
    "state:deasserted",
    "polarity:active_high",
    "polarity:active_low",
    "polarity:unknown",
    "descriptive_label",
    "casefold_unique",
    "local_appositive_declaration",
    "dedup",
    "consequence_borrowing",
    "undeclared_subject",
    "multiple_subjects",
    "contradictory_states",
    "no_marker",
    "condition_only",
    "non_copular",
    "negated_state",
    "trailing_prefix_qualifier",
    "missing_state",
    "multiple_markers",
    "casefold_collision",
    "opaque_identifier_no_suffix_alias",
    "wrong_statement_class",
}
EXPECTED_CASE_FIELDS = {
    "case_id",
    "disposition",
    "control_classes",
    "statement_class",
    "text",
    "declared_signals",
    "polarity",
    "expected_antecedent",
    "expected_consequence",
    "forbidden_keys",
}
EXPECTED_STAGES = ["source_ir", "evidence_ir", "semantic_ir", "intent_ir"]
EXPECTED_AFFECTED_STAGES = [
    "evidence_ir",
    "semantic_ir",
    "intent_ir",
    "isf_adapter",
]


def read_json(path: Path) -> Any:
    with (ROOT / path).open(encoding="utf-8") as stream:
        return json.load(stream)


def repository_path(value: Any, label: str, errors: list[str]) -> Path | None:
    if not isinstance(value, str) or not value:
        errors.append(f"{label} must be a non-empty repository-relative path")
        return None
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        errors.append(f"{label} must be repository-relative without parent traversal: {value}")
        return None
    absolute = (ROOT / path).resolve()
    try:
        absolute.relative_to(ROOT)
    except ValueError:
        errors.append(f"{label} escapes the repository: {value}")
        return None
    if not absolute.is_file():
        errors.append(f"{label} does not exist as a tracked authority candidate: {value}")
    return path


def one_by_key(rows: Any, key: str, value: str, label: str, errors: list[str]) -> dict[str, Any]:
    if not isinstance(rows, list):
        errors.append(f"{label} must be an array")
        return {}
    matches = [row for row in rows if isinstance(row, dict) and row.get(key) == value]
    if len(matches) != 1:
        errors.append(f"{label} must contain exactly one {key}={value!r}; found {len(matches)}")
        return {}
    return matches[0]


def sorted_strings(value: Any, label: str, errors: list[str]) -> list[str]:
    if not isinstance(value, list) or any(not isinstance(item, str) for item in value):
        errors.append(f"{label} must be an array of strings")
        return []
    if len(value) != len(set(value)):
        errors.append(f"{label} contains duplicate values")
    return sorted(value)


def retained_id_digest(ids: list[str]) -> str:
    payload = "".join(f"{identifier}\n" for identifier in sorted(ids)).encode()
    return hashlib.sha256(payload).hexdigest()


def validate_case_matrix(contract: dict[str, Any], errors: list[str]) -> tuple[int, int]:
    grammar = contract.get("grammar")
    if not isinstance(grammar, dict):
        errors.append("grammar must be an object")
        return (0, 0)
    if grammar.get("producer_boundary") != "sibling_evidence_normative_signal_constraint":
        errors.append("grammar producer boundary must remain the sibling normative producer")
    if grammar.get("input_statement_class") != "signal_value_constraint":
        errors.append("grammar must remain gated by signal_value_constraint classification")
    if grammar.get("sentence_selector") != "constraint_bearing_sentence":
        errors.append("grammar must retain the bounded constraint-bearing sentence selector")
    if set(grammar.get("inference_markers", [])) != EXPECTED_MARKERS:
        errors.append("grammar inference-marker set changed from the five frozen alternatives")
    if grammar.get("required_marker_count") != 1:
        errors.append("grammar must require exactly one supported inference marker")
    if set(grammar.get("prefix_state_forms", [])) != {"is asserted", "is deasserted"}:
        errors.append("grammar prefix state forms must be exactly asserted/deasserted copular forms")
    if grammar.get("consequence_rule") != (
        "the existing consequent_after_inference_marker path remains authoritative and unchanged"
    ):
        errors.append("grammar may not widen or replace consequence subject authority")
    record_contract = grammar.get("record_contract")
    if not isinstance(record_contract, dict) or record_contract.get("condition_text", "missing") is not None:
        errors.append("antecedent records must be explicitly unconditional (condition_text null)")
    if not isinstance(record_contract, dict) or record_contract.get("negated") is not False:
        errors.append("antecedent records must carry negated=false")

    matrix = contract.get("polarity_matrix")
    observed_matrix: dict[tuple[str, str], str] = {}
    if not isinstance(matrix, list):
        errors.append("polarity_matrix must be an array")
    else:
        for index, row in enumerate(matrix):
            if not isinstance(row, dict) or set(row) != {
                "symbolic_kind",
                "polarity",
                "refined_kind",
            }:
                errors.append(f"polarity_matrix[{index}] has an invalid shape")
                continue
            key = (row["symbolic_kind"], row["polarity"])
            if key in observed_matrix:
                errors.append(f"polarity_matrix repeats {key}")
            observed_matrix[key] = row["refined_kind"]
    if observed_matrix != EXPECTED_POLARITY:
        errors.append("polarity_matrix differs from the frozen shared-post-pass truth table")

    cases = contract.get("cases")
    if not isinstance(cases, list):
        errors.append("cases must be an array")
        return (0, 0)
    ids: set[str] = set()
    controls: set[str] = set()
    positives = 0
    negatives = 0
    for index, case in enumerate(cases):
        label = f"cases[{index}]"
        if not isinstance(case, dict):
            errors.append(f"{label} must be an object")
            continue
        if set(case) != EXPECTED_CASE_FIELDS:
            errors.append(f"{label} fields differ from the closed case schema")
            continue
        case_id = case.get("case_id")
        if not isinstance(case_id, str) or not case_id:
            errors.append(f"{label}.case_id must be a non-empty string")
        elif case_id in ids:
            errors.append(f"duplicate case_id: {case_id}")
        else:
            ids.add(case_id)
        case_controls = sorted_strings(case.get("control_classes"), f"{label}.control_classes", errors)
        controls.update(case_controls)
        declared = sorted_strings(case.get("declared_signals"), f"{label}.declared_signals", errors)
        if not isinstance(case.get("text"), str) or not case["text"].strip():
            errors.append(f"{label}.text must be non-empty")
        if not isinstance(case.get("polarity"), dict):
            errors.append(f"{label}.polarity must be an object")
        if not isinstance(case.get("expected_consequence"), list):
            errors.append(f"{label}.expected_consequence must be an array")
        if not isinstance(case.get("forbidden_keys"), list):
            errors.append(f"{label}.forbidden_keys must be an array")

        disposition = case.get("disposition")
        expected = case.get("expected_antecedent")
        if disposition == "positive":
            positives += 1
            if case.get("statement_class") != "signal_value_constraint":
                errors.append(f"positive case {case_id} must use signal_value_constraint")
            if not isinstance(expected, dict) or set(expected) != {
                "subject_signal",
                "symbolic_kind",
                "refined_kind",
            }:
                errors.append(f"positive case {case_id} has no exact antecedent expectation")
                continue
            subject = expected["subject_signal"]
            if subject not in declared:
                errors.append(f"positive case {case_id} antecedent subject is not declared")
            polarity = case["polarity"].get(subject, "unknown")
            key = (expected["symbolic_kind"], polarity)
            if EXPECTED_POLARITY.get(key) != expected["refined_kind"]:
                errors.append(f"positive case {case_id} contradicts the polarity matrix")
        elif disposition == "negative":
            negatives += 1
            if expected is not None:
                errors.append(f"negative case {case_id} must expect no antecedent record")
        else:
            errors.append(f"{label}.disposition must be positive or negative")

    if positives != 7 or negatives != 13:
        errors.append(f"case denominator must remain 7 positive / 13 negative; got {positives}/{negatives}")
    missing_controls = sorted(REQUIRED_CONTROLS - controls)
    unknown_controls = sorted(controls - REQUIRED_CONTROLS)
    if missing_controls:
        errors.append(f"case matrix is missing controls: {missing_controls}")
    if unknown_controls:
        errors.append(f"case matrix has undeclared controls: {unknown_controls}")

    exact = one_by_key(cases, "case_id", "positive_exact_witness", "cases", errors)
    witness = contract.get("witness", {})
    if exact:
        if exact.get("text") != witness.get("source_excerpt"):
            errors.append("exact positive case does not preserve the reviewed source excerpt")
        if sorted(exact.get("expected_consequence", [])) != sorted(witness.get("current_canonical_keys", [])):
            errors.append("exact positive case consequence differs from the three current canonical facts")
        expected = exact.get("expected_antecedent") or {}
        if (
            expected.get("subject_signal") != "PSEL"
            or expected.get("symbolic_kind") != "must_be_asserted"
            or expected.get("refined_kind") != "must_be_asserted"
        ):
            errors.append("exact positive case no longer describes the polarity-neutral PSEL asserted fact")
        if exact.get("forbidden_keys") != [
            "PSEL|must_be_value|HIGH",
            "PSEL|must_be_value|LOW",
            "PSEL|must_be_value|VALID",
        ]:
            errors.append("exact positive case must forbid guessed levels and borrowed VALID on PSEL")
        if "PSELX" not in exact.get("declared_signals", []):
            errors.append("exact positive case must retain the distinct preexisting PSELX declaration")
    suffix_alias = one_by_key(cases, "case_id", "negative_indexed_spelling_alias", "cases", errors)
    if suffix_alias:
        if suffix_alias.get("declared_signals") != ["PSELX", "PAYLOAD_S"]:
            errors.append("suffix-alias refusal must expose only PSELX and PAYLOAD_S declarations")
        if suffix_alias.get("forbidden_keys") != ["PSELX|must_be_value|HIGH"]:
            errors.append("suffix-alias refusal must forbid shape-derived PSELX state")
    return positives, negatives


def validate_witness(
    contract: dict[str, Any],
    reviewed: dict[str, Any],
    current: dict[str, Any],
    errors: list[str],
) -> None:
    witness = contract.get("witness")
    if not isinstance(witness, dict):
        errors.append("witness must be an object")
        return
    document_key = witness.get("document_key")
    cell_id = witness.get("cell_id")
    region_id = witness.get("region_id")
    statement_id = witness.get("statement_id")
    expected = sorted_strings(witness.get("expected_canonical_keys"), "witness expected keys", errors)
    actual = sorted_strings(witness.get("current_canonical_keys"), "witness current keys", errors)
    missing_key = witness.get("missing_key")
    if sorted(set(expected) - set(actual)) != [missing_key]:
        errors.append("witness must contain exactly its declared missing key")
    expected_identity_grounding = {
        "preexisting_declarations": ["PADDR", "PSELX", "PWDATA", "PWRITE"],
        "local_declaration_form": "descriptive signal appositive",
        "local_declared_identifier": "PSEL",
        "forbidden_spelling_alias": "PSELX",
        "decision": (
            "PSEL is independently grounded by the same-clause appositive 'select signal, PSEL'; "
            "PSELX remains a distinct opaque declaration and suffix spelling supplies no alias authority."
        ),
    }
    if witness.get("identity_grounding") != expected_identity_grounding:
        errors.append("witness identity grounding must preserve local PSEL and opaque PSELX separately")
    expected_polarity_grounding = {
        "resolved_polarity": "unknown",
        "expected_symbolic_kind": "must_be_asserted",
        "forbidden_level_keys": [
            "PSEL|must_be_value|HIGH",
            "PSEL|must_be_value|LOW",
        ],
        "decision": (
            "The span says asserted but supplies no active-high or active-low authority, so the "
            "canonical fact remains polarity-neutral."
        ),
    }
    if witness.get("polarity_grounding") != expected_polarity_grounding:
        errors.append("witness polarity grounding must remain unknown and polarity-neutral")

    if reviewed.get("schema_version") != 1 or len(reviewed.get("documents", [])) != 12:
        errors.append("reviewed authority must remain schema 1 with exactly 12 documents")
    reviewed_document = one_by_key(
        reviewed.get("documents"), "document_key", document_key, "reviewed documents", errors
    )
    reviewed_cell = one_by_key(reviewed_document.get("cells"), "cell_id", cell_id, "reviewed cells", errors)
    if reviewed_cell and reviewed_cell.get("expected_disposition") != "canonical":
        errors.append("reviewed witness cell must remain canonical")
    source_snapshot = reviewed_document.get("stages", {}).get("source_ir", {}).get("snapshot", {})
    source_region = one_by_key(
        source_snapshot.get("regions"), "region_id", region_id, "reviewed source regions", errors
    )
    if source_region:
        if source_region.get("source_excerpt") != witness.get("source_excerpt"):
            errors.append("reviewed source excerpt differs from the frozen witness")
        if sorted(source_region.get("reviewed_source_facts", [])) != expected:
            errors.append("reviewed source facts differ from the frozen four-key oracle")
    for stage in ["evidence_ir", "semantic_ir", "intent_ir"]:
        snapshot = reviewed_document.get("stages", {}).get(stage, {}).get("snapshot", {})
        stage_records = [
            record
            for record in snapshot.get("canonical", [])
            if isinstance(record, dict) and record.get("region_id") == region_id
        ]
        stage_keys = sorted(record.get("fact_key") for record in stage_records)
        if stage_keys != actual:
            errors.append(f"reviewed {stage} current keys differ from the frozen three-key baseline")
        if any(record.get("source_ids") != [statement_id] for record in stage_records):
            errors.append(f"reviewed {stage} witness provenance must be exactly {statement_id}")

    documents = current.get("documents", [])
    if current.get("schema_version") != 1 or len(documents) != 12:
        errors.append("current result must remain schema 1 with exactly 12 documents")
    current_document = one_by_key(documents, "document_key", document_key, "current documents", errors)
    current_cell = one_by_key(current_document.get("cells"), "cell_id", cell_id, "current cells", errors)
    if current_cell:
        for stage in ["evidence_ir", "semantic_ir", "intent_ir"]:
            score = current_cell.get("canonical", {}).get(stage, {})
            if sorted(score.get("actual_keys", [])) != actual:
                errors.append(f"current {stage} actual keys differ from the frozen witness")
            if sorted(score.get("expected_keys", [])) != expected:
                errors.append(f"current {stage} expected keys differ from the frozen witness")
            if (
                score.get("true_positives") != 3
                or score.get("false_positives") != 0
                or score.get("false_negatives") != 1
            ):
                errors.append(f"current {stage} witness score must remain 3/0/1")
        boundaries = {
            row.get("boundary"): row for row in current_cell.get("boundaries", []) if isinstance(row, dict)
        }
        if boundaries.get("source_to_evidence_ir") != {
            "boundary": "source_to_evidence_ir",
            "expected": 4,
            "conserved_or_residualized": 3,
            "unexplained_drops": 1,
        }:
            errors.append("witness first boundary must remain the exact 4/3/1 loss")
        for stage_boundary in ["evidence_to_semantic_ir", "semantic_to_intent_ir"]:
            row = boundaries.get(stage_boundary, {})
            if row.get("expected") != 3 or row.get("conserved_or_residualized") != 3 or row.get("unexplained_drops") != 0:
                errors.append(f"witness downstream boundary {stage_boundary} must remain 3/3/0")

    all_cells = [
        cell
        for document in documents
        for cell in document.get("cells", [])
        if isinstance(cell, dict)
    ]
    intent_scores = [cell.get("canonical", {}).get("intent_ir", {}) for cell in all_cells]
    population = witness.get("current_population", {})
    observed_population = {
        "documents": len(documents),
        "cells": len(all_cells),
        "intent_true_positives": sum(score.get("true_positives", 0) for score in intent_scores),
        "intent_false_positives": sum(score.get("false_positives", 0) for score in intent_scores),
        "intent_false_negatives": sum(score.get("false_negatives", 0) for score in intent_scores),
        "canonical_provenance_met": current.get("global", {})
        .get("canonical_provenance_closure", {})
        .get("met"),
        "canonical_provenance_total": current.get("global", {})
        .get("canonical_provenance_closure", {})
        .get("total"),
        "conservation_met": current.get("global", {})
        .get("stage_conservation_or_residual", {})
        .get("met"),
        "conservation_total": current.get("global", {})
        .get("stage_conservation_or_residual", {})
        .get("total"),
        "fabricated_canonical_facts": current.get("global", {}).get("fabricated_canonical_facts"),
        "unexplained_stage_drops": current.get("global", {}).get("unexplained_stage_drops"),
    }
    if population != observed_population:
        errors.append(f"frozen current-population summary drifted: {observed_population}")


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
    retained_ids = sorted_strings(retained.get("retained"), "retained chain ids", errors)
    if retained.get("schema_version") != 1 or retained.get("reclamations") != []:
        errors.append("retained-chain authority must remain schema 1 with no reclamations")
    if reconciliation.get("affected_chain_count") != len(retained_ids) or len(retained_ids) != 24:
        errors.append("affected-chain denominator must be the exact 24 retained chains")
    observed_digest = retained_id_digest(retained_ids)
    if reconciliation.get("affected_chain_ids_sha256") != observed_digest:
        errors.append("affected-chain exact-set digest differs from retained authority")
    if reconciliation.get("affected_stages") != EXPECTED_AFFECTED_STAGES:
        errors.append("affected stages must cover EvidenceIR through the ISF adapter")
    if reconciliation.get("reviewed_population_documents") != 12:
        errors.append("reviewed replay denominator must remain 12 sources")
    if reconciliation.get("reviewed_replay_stages") != EXPECTED_STAGES:
        errors.append("reviewed replay must retain the four isolated source-to-intent stages")
    attempts = reconciliation.get("reviewed_replay_attempts")
    expected_attempts = reconciliation.get("reviewed_population_documents", 0) * len(EXPECTED_STAGES)
    if attempts != expected_attempts or attempts != 48:
        errors.append("reviewed replay denominator must remain 12 sources × 4 stages = 48")


def validate_contract(
    contract: dict[str, Any],
    reviewed: dict[str, Any],
    current: dict[str, Any],
    retained: dict[str, Any],
) -> tuple[list[str], tuple[int, int]]:
    errors: list[str] = []
    if not isinstance(contract, dict) or set(contract) != EXPECTED_TOP_LEVEL:
        return (["contract top-level fields differ from the closed schema"], (0, 0))
    if contract.get("schema_version") != 1:
        errors.append("contract schema_version must be 1")
    if contract.get("contract_id") != "spec-to-intent-canonical-recovery-v1":
        errors.append("contract_id changed")
    if contract.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.7a":
        errors.append("contract owner must remain SPEC-TO-INTENT-ALIGNMENT.7a")

    authorities = contract.get("authorities")
    expected_authorities = {
        "reviewed_dataset",
        "current_result",
        "retained_chains",
        "population_replay",
    }
    if not isinstance(authorities, dict) or set(authorities) != expected_authorities:
        errors.append("authorities must name exactly reviewed/current/retained/replay inputs")
    else:
        for key, path in authorities.items():
            repository_path(path, f"authorities.{key}", errors)

    counts = validate_case_matrix(contract, errors)
    validate_witness(contract, reviewed, current, errors)
    validate_reconciliation(contract, retained, errors)
    return errors, counts


def load_authorities(contract: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    authorities = contract["authorities"]
    return (
        read_json(Path(authorities["reviewed_dataset"])),
        read_json(Path(authorities["current_result"])),
        read_json(Path(authorities["retained_chains"])),
    )


def run_check() -> int:
    contract = read_json(CONTRACT_PATH)
    reviewed, current, retained = load_authorities(contract)
    errors, (positives, negatives) = validate_contract(contract, reviewed, current, retained)
    if errors:
        for error in errors:
            print(f"canonical-recovery-contract: FAIL: {error}")
        return 1
    population = contract["witness"]["current_population"]
    print(
        "canonical-recovery-contract: "
        f"witness {population['documents']} docs / {population['cells']} cells / "
        f"{population['intent_true_positives']}/{population['intent_false_positives']}/"
        f"{population['intent_false_negatives']} TP/FP/FN; "
        f"{positives} positive + {negatives} negative controls; "
        f"{len(EXPECTED_MARKERS)} markers / {len(EXPECTED_POLARITY)} polarity rows; "
        f"{contract['reconciliation']['affected_chain_count']} affected chains / "
        f"{contract['reconciliation']['reviewed_replay_attempts']} reviewed stage attempts: PASS"
    )
    return 0


def run_self_test() -> int:
    contract = read_json(CONTRACT_PATH)
    reviewed, current, retained = load_authorities(contract)
    baseline_errors, _ = validate_contract(contract, reviewed, current, retained)
    if baseline_errors:
        for error in baseline_errors:
            print(f"canonical-recovery-contract: FAIL baseline: {error}")
        return 1

    mutations: list[tuple[str, dict[str, Any], dict[str, Any], dict[str, Any], dict[str, Any]]] = []

    value = copy.deepcopy(contract)
    value["grammar"]["inference_markers"].pop()
    mutations.append(("missing-marker", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["polarity_matrix"][0]["refined_kind"] = "must_be_low"
    mutations.append(("wrong-polarity", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["cases"][1]["case_id"] = value["cases"][0]["case_id"]
    mutations.append(("duplicate-case", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["cases"][0]["declared_signals"].remove("PSEL")
    mutations.append(("undeclared-positive", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["witness"]["identity_grounding"]["local_declared_identifier"] = "PSELX"
    mutations.append(("spelling-alias-identity", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["witness"]["polarity_grounding"]["resolved_polarity"] = "active_high"
    mutations.append(("invented-witness-polarity", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    negative = next(case for case in value["cases"] if case["disposition"] == "negative")
    negative["expected_antecedent"] = {
        "subject_signal": "INVENTED",
        "symbolic_kind": "must_be_asserted",
        "refined_kind": "must_be_asserted",
    }
    mutations.append(("negative-emission", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["reconciliation"]["affected_chain_count"] = 23
    mutations.append(("partial-chain-set", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["reconciliation"]["reviewed_replay_attempts"] = 47
    mutations.append(("partial-replay", value, reviewed, current, retained))

    value = copy.deepcopy(contract)
    value["cases"] = [
        case
        for case in value["cases"]
        if "condition_only" not in case["control_classes"]
    ]
    mutations.append(("missing-control", value, reviewed, current, retained))

    mutated_current = copy.deepcopy(current)
    witness_document = next(
        document
        for document in mutated_current["documents"]
        if document["document_key"] == contract["witness"]["document_key"]
    )
    witness_cell = next(
        cell
        for cell in witness_document["cells"]
        if cell["cell_id"] == contract["witness"]["cell_id"]
    )
    witness_cell["canonical"]["evidence_ir"]["actual_keys"].append(
        contract["witness"]["missing_key"]
    )
    mutations.append(("witness-no-longer-red", contract, reviewed, mutated_current, retained))

    failures: list[str] = []
    for name, mutated_contract, mutated_reviewed, mutated_result, mutated_retained in mutations:
        mutation_errors, _ = validate_contract(
            mutated_contract, mutated_reviewed, mutated_result, mutated_retained
        )
        if not mutation_errors:
            failures.append(name)
    if failures:
        print(f"canonical-recovery-contract: FAIL self-test accepted mutations: {failures}")
        return 1
    print(
        "canonical-recovery-contract: self-test "
        f"{len(mutations)}/{len(mutations)} marker, polarity, case, declaration, negative, "
        "chain, replay, coverage, and witness RED cases pass."
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
