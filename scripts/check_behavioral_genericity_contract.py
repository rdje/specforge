#!/usr/bin/env python3
"""Validate the frozen behavioral-genericity population and relational oracle."""

from __future__ import annotations

import argparse
import copy
import csv
import hashlib
import json
import math
from pathlib import Path
import re
import sys
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
CONTRACT_PATH = Path("doctrine/production_genericity/behavioral_qualification.json")
POPULATION_PATH = Path("doctrine/production_genericity/behavioral_population.tsv")
RECIPE_MANIFEST_PATH = Path(
    "doctrine/production_genericity/reviewed_recipe_manifest.json"
)
NEGATIVE_MATRIX_PATH = Path(
    "doctrine/production_genericity/semantic_negative_matrix.json"
)
HELD_OUT_EVIDENCE_PATH = Path(
    "doctrine/production_genericity/behavioral_holdout_evidence.json"
)
BEHAVIORAL_TOOL_PATH = Path(
    "crates/specforge-conformance/src/behavioral_genericity.rs"
)

POPULATION_FIELDS = [
    "document_key",
    "source_origin",
    "source_locator",
    "source_sha256",
    "source_bytes",
    "normalized_markdown_path",
    "normalized_markdown_sha256",
    "pages",
    "visual_assets",
    "structured_tables",
    "content_elements",
    "sections",
    "vendor",
    "family",
    "category",
    "layout",
    "review_role",
    "text_semantic_records",
    "text_intent_records",
]

INTEGER_FIELDS = {
    "source_bytes",
    "pages",
    "visual_assets",
    "structured_tables",
    "content_elements",
    "sections",
    "text_semantic_records",
    "text_intent_records",
}

EXPECTED_RELATIONS = {
    "unchanged_source",
    "adversarial_identity",
    "symbol_alpha",
    "structure_preserving_paraphrase",
    "harmless_layout",
    "semantic_negative",
}

EXPECTED_STAGES = {
    "source_ir",
    "evidence_ir",
    "semantic_ir",
    "intent_ir",
    "isf_adapter",
}

EXPECTED_PLANES = {
    "pdf_full_capture",
    "normalized_text_projection",
    "reviewed_variant",
}

EXPECTED_PLANE_RELATIONS = {
    "pdf_full_capture": {"unchanged_source", "adversarial_identity"},
    "normalized_text_projection": {"symbol_alpha"},
    "reviewed_variant": {
        "structure_preserving_paraphrase",
        "harmless_layout",
        "semantic_negative",
    },
}

EXPECTED_EVIDENCE_IDENTITY = {
    "contract_sha256",
    "production_revision",
    "source_sha256",
    "transform_recipe_sha256",
    "prior_memory_sha256",
    "tool_sha256",
}

EXPECTED_EVIDENCE_COVERAGE = {
    "declared_documents",
    "attempted_documents",
    "completed_documents",
    "unmeasurable_documents",
    "all_stage_fields",
    "all_proof_claims",
    "all_expected_deltas",
    "all_unaffected_complements",
}

EXPECTED_FAILURES = {
    "authority_unavailable": "unmeasurable",
    "provider_unavailable": "unmeasurable",
    "vacuous_baseline": "unmeasurable",
    "eligible_symbol_surface_absent": "unmeasurable",
    "stale_contract_or_population": "invalid",
    "ambiguous_or_nonbijective_transform": "invalid",
    "partial_or_escaped_run": "invalid",
    "missing_expected_delta": "fail",
    "undeclared_semantic_delta": "fail",
    "proof_or_provenance_regression": "fail",
    "validation_or_lowering_regression": "fail",
}

EXPECTED_REVIEWED_RECIPE_RELATIONS = {
    "structure_preserving_paraphrase",
    "harmless_layout",
    "semantic_negative",
}

EXPECTED_REVIEWED_CHANGE_KINDS = {
    "structure_preserving_paraphrase": {"sentence_paraphrase"},
    "harmless_layout": {
        "heading_layout",
        "table_layout",
        "whitespace_layout",
        "formatting_layout",
    },
    "semantic_negative": {"semantic_timing_change"},
}

EXPECTED_NEGATIVE_CONTROL_KINDS = {
    "omission",
    "contradiction",
    "relation_reversal",
    "value_change",
    "timing_change",
    "undeclared_symbol",
    "misleading_name",
    "proof_corruption",
    "disabled_stage",
}

EXPECTED_ATTEMPT_DISPOSITIONS = {
    "authority_unavailable": "unmeasurable",
    "provider_unavailable": "unmeasurable",
    "vacuous_baseline": "unmeasurable",
    "stale_contract_or_population": "invalid",
    "ambiguous_or_nonbijective_transform": "invalid",
    "partial_or_escaped_run": "invalid",
}

SAFE_REVIEWED_PROVENANCE_FIELDS = {
    "conclusion",
    "normalized_markdown",
    "responsibilities",
    "source_text",
    "statement",
    "text",
}

RICH_CAPTURE_EXCLUSIONS = {
    "content_elements",
    "document_sections",
    "page_artifacts",
    "structured_tables",
    "visual_assets",
}

REVIEWED_COMPLEMENT = "all_unlisted_leaf_values_and_all_proof_topology_exact"

ARTIFACT_PATHS = {
    "source_ir": "generated/source_ir/{key}/source_ir.json",
    "evidence_ir": "generated/evidence_ir/{key}/evidence_ir.json",
    "semantic_ir": "generated/semantic_ir/{key}/semantic_ir.json",
    "intent_ir": "generated/intent_ir/{key}/intent_ir.json",
    "isf_adapter": "generated/adapters/isf/{key}/adapter.json",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json(path: Path) -> Any:
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def read_population(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream, delimiter="\t")
        if reader.fieldnames != POPULATION_FIELDS:
            raise ValueError(
                f"population header differs: {reader.fieldnames!r} != {POPULATION_FIELDS!r}"
            )
        return list(reader)


def safe_relative_path(value: object) -> Path | None:
    if not isinstance(value, str) or not value:
        return None
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        return None
    return path


def unique_ids(rows: object, key: str, label: str, problems: list[str]) -> set[str]:
    if not isinstance(rows, list):
        problems.append(f"{label} must be an array")
        return set()
    values: list[str] = []
    for index, row in enumerate(rows):
        if not isinstance(row, dict) or not isinstance(row.get(key), str):
            problems.append(f"{label}[{index}] lacks string {key}")
            continue
        values.append(row[key])
    if len(values) != len(set(values)):
        problems.append(f"{label} contains duplicate {key} values")
    return set(values)


def integer(row: dict[str, str], field: str, problems: list[str]) -> int:
    value = row.get(field, "")
    if not re.fullmatch(r"0|[1-9][0-9]*", value):
        problems.append(f"{row.get('document_key', '<unknown>')}: {field} is not an integer")
        return 0
    return int(value)


def expected_layout(pages: int) -> str:
    if pages <= 32:
        return "compact"
    if pages <= 96:
        return "medium"
    return "long"


def canonical_source_path(source: dict[str, Any]) -> Path:
    path = Path(source["canonical_path"])
    return path if path.is_absolute() else ROOT / path


def check_file_identity(
    declaration: object,
    label: str,
    problems: list[str],
) -> None:
    if not isinstance(declaration, dict):
        problems.append(f"frozen census {label} must be an object")
        return
    relative = safe_relative_path(declaration.get("path"))
    expected_hash = declaration.get("sha256")
    if relative is None:
        problems.append(f"frozen census {label} path is not repository-relative")
        return
    path = ROOT / relative
    if not path.is_file():
        problems.append(f"frozen census {label} is missing: {relative}")
    elif not isinstance(expected_hash, str) or sha256(path) != expected_hash:
        problems.append(f"frozen census {label} SHA-256 differs: {relative}")


def validate_reviewed_recipes(
    rows: list[dict[str, str]], problems: list[str]
) -> None:
    try:
        manifest = read_json(ROOT / RECIPE_MANIFEST_PATH)
    except (OSError, json.JSONDecodeError) as error:
        problems.append(f"reviewed recipe manifest is unavailable: {error}")
        return
    if not isinstance(manifest, dict):
        problems.append("reviewed recipe manifest must be an object")
        return
    if manifest.get("schema_version") != 1:
        problems.append("reviewed recipe manifest schema_version must be 1")
    if manifest.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii":
        problems.append("reviewed recipe manifest owner differs from .f.ii parent")
    recipes = manifest.get("recipes")
    recipe_ids = unique_ids(recipes, "recipe_id", "reviewed recipes", problems)
    if not recipe_ids:
        problems.append("reviewed recipe manifest is empty")
    if not isinstance(recipes, list):
        return
    relations = {
        row.get("relation")
        for row in recipes
        if isinstance(row, dict) and isinstance(row.get("relation"), str)
    }
    if relations != EXPECTED_REVIEWED_RECIPE_RELATIONS:
        problems.append("reviewed recipe relations differ from the released .f.ii set")
    population_by_markdown = {
        row.get("normalized_markdown_path"): row for row in rows
    }
    for index, declaration in enumerate(recipes):
        if not isinstance(declaration, dict):
            problems.append(f"reviewed recipes[{index}] must be an object")
            continue
        recipe_id = declaration.get("recipe_id", f"index-{index}")
        relative = safe_relative_path(declaration.get("path"))
        if relative is None or not relative.is_relative_to(
            Path("doctrine/production_genericity/reviewed_recipes")
        ):
            problems.append(f"reviewed recipe {recipe_id} path is unsafe")
            continue
        recipe_path = ROOT / relative
        if not recipe_path.is_file():
            problems.append(f"reviewed recipe {recipe_id} is missing: {relative}")
            continue
        if declaration.get("sha256") != sha256(recipe_path):
            problems.append(f"reviewed recipe {recipe_id} SHA-256 differs")
        try:
            recipe = read_json(recipe_path)
        except (OSError, json.JSONDecodeError) as error:
            problems.append(f"reviewed recipe {recipe_id} is invalid JSON: {error}")
            continue
        if not isinstance(recipe, dict):
            problems.append(f"reviewed recipe {recipe_id} must be an object")
            continue
        expected_keys = {
            "schema_version",
            "recipe_id",
            "relation",
            "source_authority",
            "source_sha256",
            "review_status",
            "changed_spans",
            "preserved_conclusions",
            "unaffected_complement",
            "unmeasurable_source_surfaces",
        }
        if recipe.get("relation") == "semantic_negative":
            expected_keys |= {
                "semantic_negative_kind",
                "required_deltas",
                "dependent_proof_deltas",
            }
        if set(recipe) != expected_keys:
            problems.append(f"reviewed recipe {recipe_id} has an open or incomplete schema")
        for field in (
            "recipe_id",
            "relation",
            "source_authority",
            "source_sha256",
        ):
            if recipe.get(field) != declaration.get(field):
                problems.append(f"reviewed recipe {recipe_id} differs from manifest {field}")
        if recipe.get("schema_version") != 1 or recipe.get("review_status") != "approved":
            problems.append(f"reviewed recipe {recipe_id} is not approved schema 1")
        if declaration.get("review_role") != "reviewed_calibration":
            problems.append(f"reviewed recipe {recipe_id} is not calibration-owned")
        source_row = population_by_markdown.get(declaration.get("source_authority"))
        if source_row is None or source_row.get("review_role") != "reviewed_calibration":
            problems.append(f"reviewed recipe {recipe_id} source is not frozen calibration")
        elif source_row.get("normalized_markdown_sha256") != declaration.get(
            "source_sha256"
        ):
            problems.append(f"reviewed recipe {recipe_id} source digest differs from population")
        source_relative = safe_relative_path(recipe.get("source_authority"))
        source_path = ROOT / source_relative if source_relative is not None else None
        source_text = None
        if source_path is None or not source_path.is_file():
            problems.append(f"reviewed recipe {recipe_id} source is missing or unsafe")
        elif sha256(source_path) != recipe.get("source_sha256"):
            problems.append(f"reviewed recipe {recipe_id} source SHA-256 differs")
        else:
            source_text = source_path.read_text(encoding="utf-8")
        changes = recipe.get("changed_spans")
        change_ids = unique_ids(changes, "change_id", f"recipe {recipe_id} changes", problems)
        if not change_ids or not isinstance(changes, list):
            problems.append(f"reviewed recipe {recipe_id} has no exhaustive changed spans")
            continue
        kinds: set[str] = set()
        for change_index, change in enumerate(changes):
            if not isinstance(change, dict):
                problems.append(f"recipe {recipe_id} change {change_index} must be an object")
                continue
            if set(change) != {
                "change_id",
                "kind",
                "exact_before",
                "exact_after",
                "expected_occurrences",
                "allowed_provenance_fields",
                "allow_source_bound_identifier_projection",
            }:
                problems.append(f"recipe {recipe_id} change {change_index} schema is open")
            kind = change.get("kind")
            if isinstance(kind, str):
                kinds.add(kind)
            before = change.get("exact_before")
            after = change.get("exact_after")
            if (
                not isinstance(before, str)
                or not before
                or not isinstance(after, str)
                or not after
                or before == after
                or change.get("expected_occurrences") != 1
            ):
                problems.append(f"recipe {recipe_id} change {change_index} is not one exact delta")
            elif source_text is not None and (
                source_text.count(before) != 1 or source_text.count(after) != 0
            ):
                problems.append(f"recipe {recipe_id} change {change_index} is ambiguous or stale")
            fields = change.get("allowed_provenance_fields")
            if (
                not isinstance(fields, list)
                or not fields
                or len(fields) != len(set(fields))
                or not set(fields) <= SAFE_REVIEWED_PROVENANCE_FIELDS
            ):
                problems.append(f"recipe {recipe_id} change {change_index} opens unsafe fields")
            if change.get("allow_source_bound_identifier_projection") is not True:
                problems.append(
                    f"recipe {recipe_id} change {change_index} lacks closed identifier projection"
                )
        if kinds != EXPECTED_REVIEWED_CHANGE_KINDS.get(recipe.get("relation"), set()):
            problems.append(f"reviewed recipe {recipe_id} change-kind coverage differs")
        conclusions = recipe.get("preserved_conclusions")
        conclusion_ids = unique_ids(
            conclusions, "conclusion_id", f"recipe {recipe_id} conclusions", problems
        )
        if not conclusion_ids or not isinstance(conclusions, list):
            problems.append(f"reviewed recipe {recipe_id} has no preserved conclusions")
        else:
            for conclusion_index, conclusion in enumerate(conclusions):
                if not isinstance(conclusion, dict) or set(conclusion) != {
                    "conclusion_id",
                    "stage",
                    "baseline_pointer",
                    "transformed_pointer",
                    "baseline_value",
                    "transformed_value",
                }:
                    problems.append(
                        f"recipe {recipe_id} conclusion {conclusion_index} schema is open"
                    )
                elif conclusion.get("stage") not in EXPECTED_STAGES:
                    problems.append(
                        f"recipe {recipe_id} conclusion {conclusion_index} stage is invalid"
                    )
        if recipe.get("relation") == "semantic_negative":
            validate_semantic_negative_recipe(recipe_id, recipe, problems)
        if recipe.get("unaffected_complement") != REVIEWED_COMPLEMENT:
            problems.append(f"reviewed recipe {recipe_id} complement is not closed")
        exclusions = recipe.get("unmeasurable_source_surfaces")
        if (
            not isinstance(exclusions, list)
            or set(exclusions) != RICH_CAPTURE_EXCLUSIONS
            or len(exclusions) != len(set(exclusions))
        ):
            problems.append(f"reviewed recipe {recipe_id} rich-capture exclusions differ")


def validate_semantic_negative_recipe(
    recipe_id: str, recipe: dict[str, Any], problems: list[str]
) -> None:
    if recipe.get("semantic_negative_kind") != "timing_grammar_admission":
        problems.append(f"semantic-negative recipe {recipe_id} kind differs")
    deltas = recipe.get("required_deltas")
    delta_ids = unique_ids(
        deltas, "delta_id", f"semantic-negative recipe {recipe_id} deltas", problems
    )
    if not delta_ids or not isinstance(deltas, list):
        problems.append(f"semantic-negative recipe {recipe_id} has no required delta")
    else:
        for index, delta in enumerate(deltas):
            if not isinstance(delta, dict) or set(delta) != {
                "delta_id",
                "stage",
                "kind",
                "baseline_pointer",
                "transformed_pointer",
                "baseline_value",
                "transformed_value",
            }:
                problems.append(
                    f"semantic-negative recipe {recipe_id} delta {index} schema is open"
                )
                continue
            baseline_pointer = delta.get("baseline_pointer")
            transformed_pointer = delta.get("transformed_pointer")
            pointers = [
                pointer
                for pointer in (baseline_pointer, transformed_pointer)
                if pointer is not None
            ]
            if delta.get("stage") not in {"semantic_ir", "intent_ir"} or any(
                not isinstance(pointer, str) or not pointer.startswith("/")
                for pointer in pointers
            ):
                problems.append(
                    f"semantic-negative recipe {recipe_id} delta {index} path/stage differs"
                )
            kind = delta.get("kind")
            if kind == "added":
                valid_shape = (
                    baseline_pointer is None
                    and delta.get("baseline_value") is None
                    and isinstance(transformed_pointer, str)
                    and delta.get("transformed_value") is not None
                )
            elif kind == "changed":
                valid_shape = (
                    isinstance(baseline_pointer, str)
                    and delta.get("baseline_value") is not None
                    and isinstance(transformed_pointer, str)
                    and delta.get("transformed_value") is not None
                    and delta.get("baseline_value") != delta.get("transformed_value")
                )
            elif kind == "removed":
                valid_shape = (
                    isinstance(baseline_pointer, str)
                    and delta.get("baseline_value") is not None
                    and transformed_pointer is None
                    and delta.get("transformed_value") is None
                )
            else:
                valid_shape = False
            if not valid_shape:
                problems.append(
                    f"semantic-negative recipe {recipe_id} delta {index} shape differs"
                )

    proof_deltas = recipe.get("dependent_proof_deltas")
    proof_ids = unique_ids(
        proof_deltas,
        "delta_id",
        f"semantic-negative recipe {recipe_id} proof deltas",
        problems,
    )
    if not proof_ids or not isinstance(proof_deltas, list):
        problems.append(f"semantic-negative recipe {recipe_id} has no proof delta")
        return
    expected_propagation = {
        "evidence_ir": {"evidence_ir", "semantic_ir", "intent_ir", "isf_adapter"},
        "semantic_ir": {"semantic_ir", "intent_ir", "isf_adapter"},
        "intent_ir": {"intent_ir", "isf_adapter"},
    }
    for index, proof in enumerate(proof_deltas):
        if not isinstance(proof, dict) or set(proof) != {
            "delta_id",
            "stages",
            "rule_id",
            "address",
        }:
            problems.append(
                f"semantic-negative recipe {recipe_id} proof delta {index} schema is open"
            )
            continue
        address = proof.get("address")
        if not isinstance(address, dict) or set(address) != {
            "stage",
            "surface",
            "stable_record_key",
            "field_path",
        }:
            problems.append(
                f"semantic-negative recipe {recipe_id} proof delta {index} address differs"
            )
            continue
        stages = proof.get("stages")
        expected_stages = expected_propagation.get(address.get("stage"))
        if (
            not isinstance(stages, list)
            or expected_stages is None
            or set(stages) != expected_stages
            or len(stages) != len(set(stages))
            or not isinstance(proof.get("rule_id"), str)
            or not proof["rule_id"]
            or not isinstance(address.get("surface"), str)
            or not address["surface"]
            or not isinstance(address.get("stable_record_key"), str)
            or not address["stable_record_key"]
        ):
            problems.append(
                f"semantic-negative recipe {recipe_id} proof delta {index} is incomplete"
            )


def validate_negative_sensitivity_matrix(problems: list[str]) -> None:
    try:
        matrix = read_json(ROOT / NEGATIVE_MATRIX_PATH)
    except (OSError, json.JSONDecodeError) as error:
        problems.append(f"semantic-negative matrix is unavailable: {error}")
        return
    if not isinstance(matrix, dict) or set(matrix) != {
        "schema_version",
        "owner",
        "unaffected_complement",
        "controls",
        "attempt_dispositions",
    }:
        problems.append("semantic-negative matrix schema is open or incomplete")
        return
    if (
        matrix.get("schema_version") != 1
        or matrix.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.c"
        or matrix.get("unaffected_complement") != REVIEWED_COMPLEMENT
    ):
        problems.append("semantic-negative matrix identity or complement differs")
    controls = matrix.get("controls")
    control_ids = unique_ids(controls, "control_id", "semantic-negative controls", problems)
    if not control_ids or not isinstance(controls, list):
        problems.append("semantic-negative matrix has no controls")
        return
    kinds = {
        control.get("kind")
        for control in controls
        if isinstance(control, dict) and isinstance(control.get("kind"), str)
    }
    if kinds != EXPECTED_NEGATIVE_CONTROL_KINDS:
        problems.append("semantic-negative matrix control-kind coverage differs")
    for index, control in enumerate(controls):
        if not isinstance(control, dict) or set(control) != {
            "control_id",
            "kind",
            "stage",
            "operation",
            "path",
            "baseline_value",
            "transformed_value",
            "expected_state",
            "expected_failure_id",
        }:
            problems.append(f"semantic-negative control {index} schema is open")
            continue
        disabled = control.get("kind") == "disabled_stage"
        if control.get("stage") not in EXPECTED_STAGES:
            problems.append(f"semantic-negative control {index} stage differs")
        if disabled:
            valid = (
                control.get("operation") == "disable_stage"
                and control.get("path") is None
                and control.get("baseline_value") is None
                and control.get("transformed_value") is None
                and control.get("expected_state") == "invalid"
                and control.get("expected_failure_id") == "partial_or_escaped_run"
            )
        else:
            operation = control.get("operation")
            valid = (
                operation in {"remove", "replace"}
                and isinstance(control.get("path"), str)
                and control["path"].startswith("/")
                and control.get("baseline_value") is not None
                and (
                    (operation == "remove" and control.get("transformed_value") is None)
                    or (operation == "replace" and control.get("transformed_value") is not None)
                )
                and control.get("expected_state") == "fail"
                and control.get("expected_failure_id") == "undeclared_semantic_delta"
            )
        if not valid:
            problems.append(f"semantic-negative control {index} disposition differs")
    if matrix.get("attempt_dispositions") != EXPECTED_ATTEMPT_DISPOSITIONS:
        problems.append("semantic-negative attempt dispositions differ")


def wilson_parts_per_million(passes: int, completed: int) -> tuple[int, int, int] | None:
    if completed == 0:
        return None
    proportion = passes / completed
    z_value = 1.959963984540054
    z_squared = z_value * z_value
    denominator = 1.0 + z_squared / completed
    center = (proportion + z_squared / (2.0 * completed)) / denominator
    margin = (
        z_value
        * math.sqrt(
            proportion * (1.0 - proportion) / completed
            + z_squared / (4.0 * completed * completed)
        )
        / denominator
    )
    scaled = lambda value: round(max(0.0, min(1.0, value)) * 1_000_000)
    return scaled(proportion), scaled(center - margin), scaled(center + margin)


def validate_held_out_evidence(
    contract: dict[str, Any],
    rows: list[dict[str, str]],
    problems: list[str],
    report_override: object | None = None,
) -> None:
    if report_override is None:
        try:
            report = read_json(ROOT / HELD_OUT_EVIDENCE_PATH)
        except (OSError, json.JSONDecodeError) as error:
            problems.append(f"held-out evidence is unreadable: {error}")
            return
    else:
        report = report_override
    if not isinstance(report, dict):
        problems.append("held-out evidence must be an object")
        return
    expected_relations = ["unchanged_source", "adversarial_identity", "symbol_alpha"]
    expected_relation_set = set(expected_relations)
    if report.get("schema_version") != 2:
        problems.append("held-out evidence schema_version must be 2")
    if report.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii":
        problems.append("held-out evidence owner differs")
    expected_identity = {
        "contract_path": CONTRACT_PATH.as_posix(),
        "contract_sha256": sha256(ROOT / CONTRACT_PATH),
        "population_path": POPULATION_PATH.as_posix(),
        "population_sha256": sha256(ROOT / POPULATION_PATH),
        "prior_memory_sha256": contract.get("frozen_census", {})
        .get("prior_memory", {})
        .get("sha256"),
        "tool_sha256": sha256(ROOT / BEHAVIORAL_TOOL_PATH),
    }
    for field, expected in expected_identity.items():
        if report.get(field) != expected:
            problems.append(
                f"held-out evidence {field} differs: {report.get(field)!r} != {expected!r}"
            )
    retained_path = report.get("retained_evidence_path")
    retained_digest = report.get("retained_evidence_sha256")
    retained_tool_digest = report.get("retained_tool_sha256")
    if (
        not isinstance(retained_path, str)
        or Path(retained_path).is_absolute()
        or ".." in Path(retained_path).parts
        or not retained_path.startswith(".project-data/tmp/")
        or not retained_path.endswith("/behavioral_holdout_evidence.json")
    ):
        problems.append("held-out retained evidence path is not safe and repository-relative")
    if not isinstance(retained_digest, str) or not re.fullmatch(
        r"[0-9a-f]{64}", retained_digest
    ):
        problems.append("held-out retained evidence digest is invalid")
    if not isinstance(retained_tool_digest, str) or not re.fullmatch(
        r"[0-9a-f]{64}", retained_tool_digest
    ):
        problems.append("held-out retained tool digest is invalid")
    if not isinstance(report.get("production_revision"), str) or not re.fullmatch(
        r"[0-9a-f]{40}", report["production_revision"]
    ):
        problems.append("held-out production revision is not a full Git id")
    if report.get("leakage_boundary") != contract.get("held_out_policy", {}).get(
        "leakage_rule"
    ):
        problems.append("held-out evidence leakage boundary differs")
    if report.get("eligible_relations") != expected_relations:
        problems.append("held-out evidence eligible relations differ")
    if report.get("final_signoff_deferred") is not True:
        problems.append("held-out evidence prematurely claims final signoff")

    calibration = [row for row in rows if row.get("review_role") == "reviewed_calibration"]
    prospective = [row for row in rows if row.get("review_role") == "prospective_holdout"]
    calibration_keys = sorted(row["document_key"] for row in calibration)
    prospective_keys = sorted(row["document_key"] for row in prospective)
    split = report.get("split")
    if not isinstance(split, dict):
        problems.append("held-out evidence split must be an object")
    else:
        split_checks = {
            "selection_boundary_commit": contract.get("selection_boundary_commit"),
            "calibration_document_keys": calibration_keys,
            "prospective_document_keys": prospective_keys,
            "overlapping_document_keys": [],
            "overlapping_source_sha256": [],
            "overlapping_normalized_markdown_sha256": [],
            "identity_disjoint": True,
        }
        for field, expected in split_checks.items():
            if split.get(field) != expected:
                problems.append(f"held-out split {field} differs")

    calibration_vendors = {row["vendor"] for row in calibration}
    calibration_families = {row["family"] for row in calibration}
    prospective_by_key = {row["document_key"]: row for row in prospective}
    documents = report.get("documents")
    document_by_key: dict[str, dict[str, Any]] = {}
    if not isinstance(documents, list):
        problems.append("held-out documents must be an array")
        documents = []
    for document in documents:
        if not isinstance(document, dict) or not isinstance(document.get("document_key"), str):
            problems.append("held-out document row is malformed")
            continue
        key = document["document_key"]
        if key in document_by_key:
            problems.append(f"held-out document is duplicated: {key}")
        document_by_key[key] = document
        population = prospective_by_key.get(key)
        if population is None:
            problems.append(f"held-out evidence includes a non-prospective document: {key}")
            continue
        direct_fields = (
            "source_origin",
            "source_locator",
            "source_sha256",
            "normalized_markdown_path",
            "normalized_markdown_sha256",
            "vendor",
            "family",
            "category",
            "layout",
        )
        for field in direct_fields:
            if document.get(field) != population.get(field):
                problems.append(f"{key}: held-out document {field} differs")
        integer_checks = {
            "text_semantic_records": int(population["text_semantic_records"]),
            "text_intent_records": int(population["text_intent_records"]),
        }
        for field, expected in integer_checks.items():
            if document.get(field) != expected:
                problems.append(f"{key}: held-out document {field} differs")
        if document.get("vendor_novel") != (population["vendor"] not in calibration_vendors):
            problems.append(f"{key}: held-out vendor novelty differs")
        if document.get("family_novel") != (population["family"] not in calibration_families):
            problems.append(f"{key}: held-out family novelty differs")
    if sorted(document_by_key) != prospective_keys:
        problems.append("held-out evidence document set differs from prospective population")

    attempts = report.get("attempts")
    attempt_by_pair: dict[tuple[str, str], dict[str, Any]] = {}
    if not isinstance(attempts, list):
        problems.append("held-out attempts must be an array")
        attempts = []
    for attempt in attempts:
        if not isinstance(attempt, dict):
            problems.append("held-out attempt row is malformed")
            continue
        key = attempt.get("document_key")
        relation = attempt.get("relation")
        pair = (key, relation)
        if key not in prospective_by_key or relation not in expected_relation_set:
            problems.append(f"held-out attempt identity is outside the matrix: {pair!r}")
            continue
        if pair in attempt_by_pair:
            problems.append(f"held-out attempt is duplicated: {pair!r}")
        attempt_by_pair[pair] = attempt
        expected_plane = (
            "normalized_text_projection" if relation == "symbol_alpha" else "pdf_full_capture"
        )
        if attempt.get("input_plane") != expected_plane:
            problems.append(f"held-out attempt input plane differs: {pair!r}")
        state = attempt.get("state")
        if state not in {"pass", "fail", "unmeasurable", "invalid"}:
            problems.append(f"held-out attempt state is invalid: {pair!r}")
        execution_mode = attempt.get("execution_mode")
        expected_mode = (
            "retained_report_revalidated"
            if relation in {"unchanged_source", "adversarial_identity"}
            else "fresh_pipeline"
            if state in {"pass", "fail"}
            else "eligibility_preflight"
        )
        if execution_mode != expected_mode:
            problems.append(f"held-out attempt execution mode differs: {pair!r}")
        identity = attempt.get("identity")
        coverage = attempt.get("coverage")
        report_digest = attempt.get("attempt_report_sha256")
        if state in {"pass", "fail"}:
            if not isinstance(identity, dict) or not isinstance(coverage, dict):
                problems.append(f"completed held-out attempt lacks identity/coverage: {pair!r}")
            else:
                source_field = (
                    "normalized_markdown_sha256"
                    if relation == "symbol_alpha"
                    else "source_sha256"
                )
                identity_checks = {
                    "contract_sha256": report.get("contract_sha256"),
                    "production_revision": report.get("production_revision"),
                    "source_sha256": prospective_by_key[key][source_field],
                    "prior_memory_sha256": report.get("prior_memory_sha256"),
                    "tool_sha256": (
                        report.get("retained_tool_sha256")
                        if execution_mode == "retained_report_revalidated"
                        else report.get("tool_sha256")
                    ),
                }
                for field, expected in identity_checks.items():
                    if identity.get(field) != expected:
                        problems.append(f"held-out attempt {pair!r} identity {field} differs")
            if not isinstance(report_digest, str) or not re.fullmatch(
                r"[0-9a-f]{64}", report_digest
            ):
                problems.append(f"completed held-out attempt lacks report digest: {pair!r}")
            if attempt.get("completed_stages") != 5:
                problems.append(f"completed held-out attempt lacks five stages: {pair!r}")
        else:
            if identity is not None or coverage is not None or report_digest is not None:
                problems.append(f"non-completed held-out attempt carries a report: {pair!r}")
            if attempt.get("completed_stages") != 0:
                problems.append(f"non-completed held-out attempt claims completed stages: {pair!r}")
        if state == "pass" and attempt.get("failure_id") is not None:
            problems.append(f"passing held-out attempt carries a failure id: {pair!r}")
        if state != "pass" and not isinstance(attempt.get("failure_id"), str):
            problems.append(f"non-passing held-out attempt lacks a failure id: {pair!r}")
        if state in {"unmeasurable", "invalid"} and not isinstance(
            attempt.get("detail"), str
        ):
            problems.append(f"non-completed held-out attempt lacks detail: {pair!r}")
        if state in {"pass", "fail"} and attempt.get("detail") is not None:
            problems.append(f"completed held-out attempt carries error detail: {pair!r}")
        if attempt.get("all_completed_stages_passed") != (state == "pass"):
            problems.append(f"held-out all-stage disposition differs: {pair!r}")
    expected_pairs = {
        (key, relation) for key in prospective_keys for relation in expected_relations
    }
    if set(attempt_by_pair) != expected_pairs:
        problems.append("held-out evidence attempt matrix is incomplete")

    coverage = report.get("coverage")
    if not isinstance(coverage, dict):
        problems.append("held-out aggregate coverage must be an object")
    else:
        state_counts = {
            state: sum(attempt.get("state") == state for attempt in attempts)
            for state in ("pass", "fail", "unmeasurable", "invalid")
        }
        coverage_checks = {
            "declared_documents": prospective_keys,
            "attempted_documents": prospective_keys,
            "declared_attempts": len(expected_pairs),
            "pass_attempts": state_counts["pass"],
            "fail_attempts": state_counts["fail"],
            "unmeasurable_attempts": state_counts["unmeasurable"],
            "invalid_attempts": state_counts["invalid"],
        }
        for field, expected in coverage_checks.items():
            if coverage.get(field) != expected:
                problems.append(f"held-out aggregate coverage {field} differs")
        completed_by_key = {
            key: sum(
                attempt_by_pair.get((key, relation), {}).get("state") in {"pass", "fail"}
                for relation in expected_relations
            )
            for key in prospective_keys
        }
        set_checks = {
            "fully_completed_documents": sorted(
                key for key, count in completed_by_key.items() if count == len(expected_relations)
            ),
            "partially_completed_documents": sorted(
                key for key, count in completed_by_key.items() if 0 < count < len(expected_relations)
            ),
            "unmeasurable_documents": sorted(
                key
                for key in prospective_keys
                if any(
                    attempt_by_pair.get((key, relation), {}).get("state") == "unmeasurable"
                    for relation in expected_relations
                )
            ),
            "invalid_documents": sorted(
                key
                for key in prospective_keys
                if any(
                    attempt_by_pair.get((key, relation), {}).get("state") == "invalid"
                    for relation in expected_relations
                )
            ),
        }
        for field, expected in set_checks.items():
            if coverage.get(field) != expected:
                problems.append(f"held-out aggregate coverage {field} differs")
        coverage_fields = (
            "baseline_top_level_fields",
            "transformed_top_level_fields",
            "baseline_proof_claims",
            "transformed_proof_claims",
            "compared_leaf_values",
        )
        for field in coverage_fields:
            expected = sum(
                attempt.get("coverage", {}).get(field, 0)
                for attempt in attempts
                if isinstance(attempt.get("coverage"), dict)
            )
            if coverage.get(field) != expected:
                problems.append(f"held-out aggregate coverage {field} differs")
        expected_deltas = sum(
            sum(
                attempt.get("coverage", {}).get(field, 0)
                for field in (
                    "expected_symbol_deltas",
                    "expected_reviewed_span_deltas",
                    "expected_semantic_deltas",
                )
            )
            for attempt in attempts
            if isinstance(attempt.get("coverage"), dict)
        )
        observed_deltas = sum(
            sum(
                attempt.get("coverage", {}).get(field, 0)
                for field in (
                    "observed_symbol_deltas",
                    "observed_reviewed_span_deltas",
                    "observed_semantic_deltas",
                )
            )
            for attempt in attempts
            if isinstance(attempt.get("coverage"), dict)
        )
        if coverage.get("expected_deltas") != expected_deltas:
            problems.append("held-out aggregate expected-delta coverage differs")
        if coverage.get("observed_deltas") != observed_deltas:
            problems.append("held-out aggregate observed-delta coverage differs")

    strata = report.get("strata")
    expected_values = {
        "overall": {"all"},
        "vendor": {row["vendor"] for row in prospective},
        "vendor_novelty": {"seen_in_calibration", "novel"},
        "family": {row["family"] for row in prospective},
        "family_novelty": {"seen_in_calibration", "novel"},
        "category": {
            "cpu-isa",
            "methodology-guide",
            "physical-link",
            "platform-system-ip",
            "register-ip",
            "wire-protocol",
        },
        "layout": {"compact", "medium", "long"},
    }
    expected_strata = {
        (relation, dimension, value)
        for relation in expected_relations
        for dimension, values in expected_values.items()
        for value in values
    }
    stratum_by_key: dict[tuple[str, str, str], dict[str, Any]] = {}
    if not isinstance(strata, list):
        problems.append("held-out strata must be an array")
        strata = []
    for stratum in strata:
        if not isinstance(stratum, dict):
            problems.append("held-out stratum row is malformed")
            continue
        identity = (stratum.get("relation"), stratum.get("dimension"), stratum.get("value"))
        if identity in stratum_by_key:
            problems.append(f"held-out stratum is duplicated: {identity!r}")
        stratum_by_key[identity] = stratum
        relation, dimension, value = identity
        if identity not in expected_strata:
            problems.append(f"held-out stratum is outside the closed matrix: {identity!r}")
            continue
        selected_keys = []
        for key, document in document_by_key.items():
            included = {
                "overall": value == "all",
                "vendor": document.get("vendor") == value,
                "family": document.get("family") == value,
                "category": document.get("category") == value,
                "layout": document.get("layout") == value,
                "vendor_novelty": document.get("vendor_novel") == (value == "novel"),
                "family_novelty": document.get("family_novel") == (value == "novel"),
            }[dimension]
            if included:
                selected_keys.append(key)
        selected_keys.sort()
        selected_attempts = [
            attempt_by_pair[(key, relation)]
            for key in selected_keys
            if (key, relation) in attempt_by_pair
        ]
        counts = {
            state: sum(attempt.get("state") == state for attempt in selected_attempts)
            for state in ("pass", "fail", "unmeasurable", "invalid")
        }
        expected_state = (
            "invalid"
            if counts["invalid"]
            else "fail"
            if counts["fail"]
            else "unmeasurable"
            if not selected_keys or counts["unmeasurable"]
            else "pass"
        )
        stratum_checks = {
            "document_keys": selected_keys,
            "declared_documents": len(selected_keys),
            "passes": counts["pass"],
            "failures": counts["fail"],
            "unmeasurable": counts["unmeasurable"],
            "invalid": counts["invalid"],
            "state": expected_state,
        }
        for field, expected in stratum_checks.items():
            if stratum.get(field) != expected:
                problems.append(f"held-out stratum {identity!r} {field} differs")
        expected_limitation = (
            "no_prospective_denominator"
            if not selected_keys
            else "invalid_attempts_present"
            if counts["invalid"]
            else "unmeasurable_attempts_excluded_from_interval"
            if counts["unmeasurable"]
            else None
        )
        if stratum.get("limitation") != expected_limitation:
            problems.append(f"held-out stratum {identity!r} limitation differs")
        uncertainty = stratum.get("uncertainty")
        completed = counts["pass"] + counts["fail"]
        interval = wilson_parts_per_million(counts["pass"], completed)
        if not isinstance(uncertainty, dict):
            problems.append(f"held-out stratum {identity!r} uncertainty is absent")
        elif interval is None:
            if (
                uncertainty.get("method") != "unavailable_no_completed_denominator"
                or uncertainty.get("sample_size") != 0
                or uncertainty.get("confidence_basis_points") is not None
                or any(
                    uncertainty.get(field) is not None
                    for field in (
                        "point_estimate_parts_per_million",
                        "lower_parts_per_million",
                        "upper_parts_per_million",
                    )
                )
            ):
                problems.append(f"held-out stratum {identity!r} unavailable interval differs")
        else:
            point, lower, upper = interval
            uncertainty_checks = {
                "method": "wilson_score_95_percent",
                "confidence_basis_points": 9500,
                "sample_size": completed,
                "pass_numerator": counts["pass"],
                "point_estimate_parts_per_million": point,
                "lower_parts_per_million": lower,
                "upper_parts_per_million": upper,
            }
            for field, expected in uncertainty_checks.items():
                if uncertainty.get(field) != expected:
                    problems.append(
                        f"held-out stratum {identity!r} uncertainty {field} differs"
                    )
        if not isinstance(uncertainty, dict) or "frozen population" not in str(
            uncertainty.get("scope_limit", "")
        ):
            problems.append(f"held-out stratum {identity!r} lacks scope limitation")
    if set(stratum_by_key) != expected_strata:
        problems.append("held-out evidence stratum matrix is incomplete")

    def find_absolute(value: object, path: str = "") -> list[str]:
        found: list[str] = []
        if isinstance(value, dict):
            for field, child in value.items():
                found.extend(find_absolute(child, f"{path}/{field}"))
        elif isinstance(value, list):
            for index, child in enumerate(value):
                found.extend(find_absolute(child, f"{path}/{index}"))
        elif isinstance(value, str) and (value.startswith("/") or "/Volumes/" in value):
            found.append(path)
        return found

    absolute_paths = find_absolute(report)
    if absolute_paths:
        problems.append(
            f"held-out evidence persists absolute paths: {absolute_paths[:5]}"
        )


def validate(
    contract: dict[str, Any],
    rows: list[dict[str, str]],
    *,
    verify_artifacts: bool = True,
    verify_held_out_evidence: bool = True,
) -> tuple[list[str], dict[str, int]]:
    problems: list[str] = []
    metrics: dict[str, int] = {}

    if contract.get("schema_version") != 1:
        problems.append("contract schema_version must be 1")
    if contract.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f":
        problems.append("contract owner differs from the behavioral task-tree parent")
    boundary = contract.get("selection_boundary_commit")
    if not isinstance(boundary, str) or not re.fullmatch(r"[0-9a-f]{40}", boundary):
        problems.append("selection_boundary_commit must be a full Git object id")

    declarations = contract.get("declarations")
    if not isinstance(declarations, dict):
        problems.append("declarations must be an object")
        return problems, metrics
    expected_declarations = {
        "current_population": POPULATION_PATH.as_posix(),
        "retained_population": "doctrine/chain_currency/retained_bundles.json",
        "reviewed_population": (
            "crates/specforge/test_data/source_to_intent_vertical/reviewed_dataset.json"
        ),
        "held_out_evidence": HELD_OUT_EVIDENCE_PATH.as_posix(),
        "reviewed_recipe_manifest": RECIPE_MANIFEST_PATH.as_posix(),
        "semantic_negative_matrix": NEGATIVE_MATRIX_PATH.as_posix(),
        "claim_family_inventory": (
            "doctrine/production_genericity/claim_family_inventory.tsv"
        ),
        "rule_family_inventory": (
            "doctrine/production_genericity/rule_family_inventory.tsv"
        ),
    }
    if declarations != expected_declarations:
        problems.append("contract declarations differ from the closed authority set")

    for label, relative_text in expected_declarations.items():
        relative = safe_relative_path(relative_text)
        if relative is None or not (ROOT / relative).is_file():
            problems.append(f"declared {label} is missing or unsafe: {relative_text}")

    validate_reviewed_recipes(rows, problems)
    validate_negative_sensitivity_matrix(problems)

    relation_ids = unique_ids(contract.get("relations"), "relation_id", "relations", problems)
    if relation_ids != EXPECTED_RELATIONS:
        problems.append(
            f"relation ids differ: missing={sorted(EXPECTED_RELATIONS - relation_ids)}, "
            f"extra={sorted(relation_ids - EXPECTED_RELATIONS)}"
        )
    for relation in contract.get("relations", []):
        if not isinstance(relation, dict):
            continue
        relation_id = relation.get("relation_id", "<unknown>")
        expected_plane = next(
            (
                plane_id
                for plane_id, plane_relations in EXPECTED_PLANE_RELATIONS.items()
                if relation_id in plane_relations
            ),
            None,
        )
        if relation.get("input_plane") != expected_plane:
            problems.append(
                f"relation {relation_id} input plane differs: "
                f"{relation.get('input_plane')!r} != {expected_plane!r}"
            )
        for field in ("eligibility", "required_relation"):
            if not isinstance(relation.get(field), str) or not relation[field].strip():
                problems.append(f"relation {relation_id} lacks nonempty {field}")
        for field in ("allowed_deltas", "required_deltas", "forbidden_deltas"):
            deltas = relation.get(field)
            if not isinstance(deltas, list) or not all(
                isinstance(delta, str) and delta for delta in deltas
            ):
                problems.append(f"relation {relation_id} has invalid {field}")
            elif len(deltas) != len(set(deltas)):
                problems.append(f"relation {relation_id} has duplicate {field}")
        if not relation.get("forbidden_deltas"):
            problems.append(f"relation {relation_id} has no forbidden-delta complement")

    stage_ids = unique_ids(
        contract.get("stage_comparison"), "stage_id", "stage comparison", problems
    )
    if stage_ids != EXPECTED_STAGES:
        problems.append(
            f"comparison stages differ: missing={sorted(EXPECTED_STAGES - stage_ids)}, "
            f"extra={sorted(stage_ids - EXPECTED_STAGES)}"
        )
    for stage in contract.get("stage_comparison", []):
        if not isinstance(stage, dict):
            continue
        if stage.get("proof_required") is not True:
            problems.append(f"stage {stage.get('stage_id')} does not require proof")
        if "every serialized top-level field" not in str(stage.get("coverage", "")):
            problems.append(f"stage {stage.get('stage_id')} has partial field coverage")
        if "every proof claim" not in str(stage.get("coverage", "")):
            problems.append(f"stage {stage.get('stage_id')} has partial proof coverage")
        if not isinstance(stage.get("semantic_surfaces"), str) or not stage[
            "semantic_surfaces"
        ].strip():
            problems.append(f"stage {stage.get('stage_id')} lacks semantic surfaces")

    plane_ids = unique_ids(contract.get("input_planes"), "plane_id", "input planes", problems)
    if plane_ids != EXPECTED_PLANES:
        problems.append(
            f"input planes differ: missing={sorted(EXPECTED_PLANES - plane_ids)}, "
            f"extra={sorted(plane_ids - EXPECTED_PLANES)}"
        )
    for plane in contract.get("input_planes", []):
        if not isinstance(plane, dict):
            continue
        plane_id = plane.get("plane_id", "<unknown>")
        plane_relations = plane.get("relations")
        if not isinstance(plane_relations, list) or set(plane_relations) != (
            EXPECTED_PLANE_RELATIONS.get(plane_id, set())
        ):
            problems.append(f"input plane {plane_id} has an incomplete relation partition")
        elif len(plane_relations) != len(set(plane_relations)):
            problems.append(f"input plane {plane_id} repeats a relation")
        for field in ("authority", "restriction"):
            if not isinstance(plane.get(field), str) or not plane[field].strip():
                problems.append(f"input plane {plane_id} lacks nonempty {field}")
        if plane_id == "reviewed_variant":
            if plane.get("base_plane") != "normalized_text_projection":
                problems.append("reviewed variants must derive from normalized text")
        elif "base_plane" in plane:
            problems.append(f"input plane {plane_id} has an unexpected base plane")

    failures: dict[str, str] = {}
    failure_rows = contract.get("failure_taxonomy")
    unique_ids(failure_rows, "failure_id", "failure taxonomy", problems)
    if isinstance(failure_rows, list):
        failures = {
            row["failure_id"]: row.get("disposition", "")
            for row in failure_rows
            if isinstance(row, dict) and isinstance(row.get("failure_id"), str)
        }
    if failures != EXPECTED_FAILURES:
        problems.append("failure taxonomy differs from the closed fail/invalid/unmeasurable map")

    evidence = contract.get("evidence_contract")
    if not isinstance(evidence, dict):
        problems.append("evidence_contract must be an object")
    else:
        evidence_sets = {
            "required_run_states": {"pass", "fail", "unmeasurable", "invalid"},
            "required_identity": EXPECTED_EVIDENCE_IDENTITY,
            "required_coverage": EXPECTED_EVIDENCE_COVERAGE,
        }
        for field, expected in evidence_sets.items():
            values = evidence.get(field)
            if (
                not isinstance(values, list)
                or set(values) != expected
                or len(values) != len(set(values))
            ):
                problems.append(f"evidence {field} is incomplete or duplicated")
        expected_dispositions = {
            "partial_runs": "invalid",
            "missing_expected_delta": "fail",
            "undeclared_delta": "fail",
            "missing_authority_or_provider": "unmeasurable",
            "ambiguous_or_escaped_transform": "invalid",
        }
        for field, expected in expected_dispositions.items():
            if evidence.get(field) != expected:
                problems.append(
                    f"evidence disposition {field} differs: "
                    f"{evidence.get(field)!r} != {expected!r}"
                )

    retained_path = ROOT / expected_declarations["retained_population"]
    reviewed_path = ROOT / expected_declarations["reviewed_population"]
    retained = read_json(retained_path)
    reviewed = read_json(reviewed_path)
    retained_keys = retained.get("retained") if isinstance(retained, dict) else None
    reviewed_documents = reviewed.get("documents") if isinstance(reviewed, dict) else None
    if not isinstance(retained_keys, list) or not all(
        isinstance(key, str) for key in retained_keys
    ):
        problems.append("retained population has no string retained array")
        retained_keys = []
    if not isinstance(reviewed_documents, list):
        problems.append("reviewed population has no documents array")
        reviewed_documents = []
    reviewed_by_key = {
        document.get("document_key"): document
        for document in reviewed_documents
        if isinstance(document, dict) and isinstance(document.get("document_key"), str)
    }

    row_keys = [row.get("document_key", "") for row in rows]
    if row_keys != sorted(row_keys):
        problems.append("behavioral population rows are not sorted by document_key")
    if len(row_keys) != len(set(row_keys)):
        problems.append("behavioral population contains duplicate document keys")
    if set(row_keys) != set(retained_keys):
        problems.append(
            f"behavioral population differs from retained current population: "
            f"missing={sorted(set(retained_keys) - set(row_keys))}, "
            f"extra={sorted(set(row_keys) - set(retained_keys))}"
        )

    calibration_rows: list[dict[str, str]] = []
    prospective_rows: list[dict[str, str]] = []
    source_verified = 0
    external_unavailable = 0
    markdown_bytes = 0
    semantic_total = 0
    intent_total = 0

    for row in rows:
        key = row.get("document_key", "<unknown>")
        for field in POPULATION_FIELDS:
            if field not in row or row[field] == "":
                problems.append(f"{key}: population field {field} is empty")
        values = {field: integer(row, field, problems) for field in INTEGER_FIELDS}
        if not re.fullmatch(r"[0-9a-f]{64}", row.get("source_sha256", "")):
            problems.append(f"{key}: source_sha256 is not a lowercase SHA-256")
        if not re.fullmatch(r"[0-9a-f]{64}", row.get("normalized_markdown_sha256", "")):
            problems.append(f"{key}: normalized_markdown_sha256 is not a lowercase SHA-256")
        if row.get("layout") != expected_layout(values["pages"]):
            problems.append(f"{key}: layout differs from the frozen page bands")

        reviewed_document = reviewed_by_key.get(key)
        expected_role = "reviewed_calibration" if reviewed_document else "prospective_holdout"
        if row.get("review_role") != expected_role:
            problems.append(f"{key}: review_role must be {expected_role}")
        if reviewed_document:
            calibration_rows.append(row)
            if row.get("category") != reviewed_document.get("category"):
                problems.append(f"{key}: category differs from reviewed authority")
            source = reviewed_document.get("source", {})
            if row.get("source_sha256") != source.get("sha256"):
                problems.append(f"{key}: source digest differs from reviewed authority")
            expected_locator = source.get("relative_path") or source.get("portable_id")
            if row.get("source_locator") != expected_locator:
                problems.append(f"{key}: source locator differs from reviewed authority")
        else:
            prospective_rows.append(row)

        if values["text_semantic_records"] == 0 and values["text_intent_records"] != 0:
            problems.append(f"{key}: text projection has IntentIR records without SemanticIR records")
        if values["text_semantic_records"] != 0 and values["text_intent_records"] == 0:
            problems.append(f"{key}: text projection silently loses every semantic record")
        semantic_total += values["text_semantic_records"]
        intent_total += values["text_intent_records"]

        source_ir_path = ROOT / ARTIFACT_PATHS["source_ir"].format(key=key)
        if not source_ir_path.is_file():
            problems.append(f"{key}: current SourceIR is missing")
            continue
        source_ir = read_json(source_ir_path)
        source = source_ir.get("source", {})
        plan = source_ir.get("normalization_plan", {})
        if source_ir.get("document_identity", {}).get("document_key") != key:
            problems.append(f"{key}: SourceIR document identity differs")
        if source.get("path_origin") != row.get("source_origin"):
            problems.append(f"{key}: source origin differs from SourceIR")
        if source.get("source_kind") != "pdf" or plan.get("status") != "ready":
            problems.append(f"{key}: SourceIR is not a ready PDF capture")
        if source.get("size_bytes") != values["source_bytes"]:
            problems.append(f"{key}: source byte count differs from SourceIR")
        expected_counts = {
            "pages": len(source_ir.get("page_artifacts", [])),
            "visual_assets": len(source_ir.get("visual_assets", [])),
            "structured_tables": len(source_ir.get("structured_tables", [])),
            "content_elements": len(source_ir.get("content_elements", [])),
            "sections": len(source_ir.get("document_sections", [])),
        }
        for field, expected in expected_counts.items():
            if values[field] != expected:
                problems.append(f"{key}: {field} differs from SourceIR ({values[field]} != {expected})")

        markdown_relative = safe_relative_path(row.get("normalized_markdown_path"))
        if markdown_relative is None:
            problems.append(f"{key}: normalized Markdown path is not repository-relative")
        else:
            if plan.get("promoted_markdown_path") != markdown_relative.as_posix():
                problems.append(f"{key}: normalized Markdown path differs from SourceIR")
            markdown_path = ROOT / markdown_relative
            if not markdown_path.is_file():
                problems.append(f"{key}: normalized Markdown is missing")
            else:
                markdown_bytes += markdown_path.stat().st_size
                if sha256(markdown_path) != row.get("normalized_markdown_sha256"):
                    problems.append(f"{key}: normalized Markdown digest differs")

        canonical_text = source.get("canonical_path")
        if not isinstance(canonical_text, str):
            problems.append(f"{key}: SourceIR canonical source path is missing")
        else:
            canonical = Path(canonical_text)
            if row.get("source_origin") == "repository_owned":
                if canonical.is_absolute() or row.get("source_locator") != canonical.as_posix():
                    problems.append(f"{key}: repository source locator is not canonical and relative")
            elif row.get("source_origin") == "external_input":
                if Path(row.get("source_locator", "")).name != row.get("source_locator"):
                    problems.append(f"{key}: external source locator must be a portable basename")
                if canonical.name != row.get("source_locator"):
                    problems.append(f"{key}: external portable locator differs from SourceIR basename")
            else:
                problems.append(f"{key}: unsupported source_origin")

            source_path = canonical_source_path(source)
            if source_path.is_file():
                source_verified += 1
                if source_path.stat().st_dev != ROOT.stat().st_dev:
                    problems.append(f"{key}: source authority is off the repository volume")
                if source_path.stat().st_size != values["source_bytes"]:
                    problems.append(f"{key}: live source byte count differs")
                if sha256(source_path) != row.get("source_sha256"):
                    problems.append(f"{key}: live source digest differs")
            elif row.get("source_origin") == "repository_owned":
                problems.append(f"{key}: repository-owned source authority is missing")
            else:
                external_unavailable += 1

        if verify_artifacts:
            for stage_id, path_template in ARTIFACT_PATHS.items():
                artifact_path = ROOT / path_template.format(key=key)
                if not artifact_path.is_file():
                    problems.append(f"{key}: current {stage_id} artifact is missing")
                    continue
                artifact = read_json(artifact_path)
                if artifact.get("stage") != stage_id:
                    problems.append(f"{key}: current {stage_id} artifact names another stage")
                if artifact.get("document_identity", {}).get("document_key") != key:
                    problems.append(f"{key}: current {stage_id} artifact names another document")
                claims = artifact.get("proof_ledger", {}).get("claims")
                if not isinstance(claims, list) or not claims:
                    problems.append(f"{key}: current {stage_id} artifact has no proof claims")

    metrics.update(
        {
            "current_documents": len(rows),
            "repository_owned_sources": sum(
                row.get("source_origin") == "repository_owned" for row in rows
            ),
            "external_input_sources": sum(
                row.get("source_origin") == "external_input" for row in rows
            ),
            "reviewed_documents": len(reviewed_documents),
            "reviewed_current_overlap": len(calibration_rows),
            "prospective_holdout_documents": len(prospective_rows),
            "text_projection_measurable_documents": sum(
                integer(row, "text_intent_records", []) > 0 for row in rows
            ),
            "text_projection_unmeasurable_documents": sum(
                integer(row, "text_intent_records", []) == 0 for row in rows
            ),
            "vendor_strata": len({row.get("vendor") for row in rows}),
            "family_strata": len({row.get("family") for row in rows}),
            "layout_strata": len({row.get("layout") for row in rows}),
            "prospective_category_strata": len(
                {row.get("category") for row in prospective_rows}
            ),
            "reviewed_category_strata": len(
                {
                    document.get("category")
                    for document in reviewed_documents
                    if isinstance(document, dict)
                }
            ),
            "source_authorities_verified": source_verified,
            "external_authorities_unavailable": external_unavailable,
        }
    )
    calibration_vendors = {row.get("vendor") for row in calibration_rows}
    calibration_families = {row.get("family") for row in calibration_rows}
    metrics["prospective_vendor_novel_documents"] = sum(
        row.get("vendor") not in calibration_vendors for row in prospective_rows
    )
    metrics["prospective_family_novel_documents"] = sum(
        row.get("family") not in calibration_families for row in prospective_rows
    )

    assertions = contract.get("population_assertions")
    if not isinstance(assertions, dict):
        problems.append("population_assertions must be an object")
    else:
        for field in (
            "current_documents",
            "repository_owned_sources",
            "external_input_sources",
            "reviewed_documents",
            "reviewed_current_overlap",
            "prospective_holdout_documents",
            "text_projection_measurable_documents",
            "text_projection_unmeasurable_documents",
            "vendor_strata",
            "prospective_vendor_novel_documents",
            "family_strata",
            "prospective_family_novel_documents",
            "layout_strata",
            "prospective_category_strata",
            "reviewed_category_strata",
        ):
            if assertions.get(field) != metrics.get(field):
                problems.append(
                    f"population assertion {field} differs: "
                    f"{assertions.get(field)!r} != {metrics.get(field)!r}"
                )

    census = contract.get("frozen_census")
    if not isinstance(census, dict):
        problems.append("frozen_census must be an object")
    else:
        check_file_identity(census.get("prior_memory"), "prior_memory", problems)
        check_file_identity(census.get("replay_library"), "replay_library", problems)
        check_file_identity(census.get("replay_entrypoint"), "replay_entrypoint", problems)
        if census.get("provider_calls") != "forbidden":
            problems.append("frozen text census must forbid provider calls")
        if census.get("raw_output_disposition") != "inspected_then_exactly_cleaned":
            problems.append("frozen text census does not attest exact scratch cleanup")
        aggregate = census.get("aggregate")
        if not isinstance(aggregate, dict):
            problems.append("frozen census aggregate must be an object")
        else:
            aggregate_checks = {
                "documents": len(rows),
                "normalized_markdown_bytes": markdown_bytes,
                "semantic_records": semantic_total,
                "intent_records": intent_total,
            }
            for field, expected in aggregate_checks.items():
                if aggregate.get(field) != expected:
                    problems.append(
                        f"frozen census aggregate {field} differs: "
                        f"{aggregate.get(field)!r} != {expected}"
                    )
            if aggregate.get("source_ir_rich_capture_records") != 0:
                problems.append("text census must not claim rich SourceIR capture")
            for field in (
                "source_ir_proof_claims",
                "evidence_statements",
                "evidence_proof_claims",
                "semantic_proof_claims",
                "intent_proof_claims",
            ):
                if not isinstance(aggregate.get(field), int) or aggregate[field] <= 0:
                    problems.append(f"frozen census aggregate {field} must be positive")

    policy = contract.get("held_out_policy")
    if not isinstance(policy, dict):
        problems.append("held_out_policy must be an object")
    else:
        if "not claimed historically unseen" not in policy.get(
            "historical_reviewed_status", ""
        ):
            problems.append("reviewed population historical exposure is overstated")
        if "four categories" not in policy.get("category_limit", ""):
            problems.append("prospective category limitation is not explicit")
        if "Production core cannot read" not in policy.get("leakage_rule", ""):
            problems.append("held-out calibration authority is not isolated from core")

    execution = contract.get("held_out_execution")
    expected_execution = {
        "owner": "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii",
        "eligible_relations": [
            "unchanged_source",
            "adversarial_identity",
            "symbol_alpha",
        ],
        "declared_attempts": 51,
        "stratum_dimensions": [
            "overall",
            "vendor",
            "vendor_novelty",
            "family",
            "family_novelty",
            "category",
            "layout",
        ],
        "uncertainty_method": "wilson_score_95_percent_over_completed_documents",
        "uncertainty_scope": "Descriptive for the frozen non-random prospective population only; unavailable or invalid attempts are excluded from the interval but remain explicit in its denominator record.",
        "zero_denominator_disposition": "unmeasurable",
        "final_signoff": "deferred_to_SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v",
    }
    if execution != expected_execution:
        problems.append("held-out execution contract differs from the closed matrix")

    if verify_held_out_evidence:
        validate_held_out_evidence(contract, rows, problems)

    return problems, metrics


def run_self_test(contract: dict[str, Any], rows: list[dict[str, str]]) -> int:
    baseline, _ = validate(
        contract,
        rows,
        verify_artifacts=False,
        verify_held_out_evidence=False,
    )
    if baseline:
        print("behavioral-genericity-contract self-test baseline failed:", file=sys.stderr)
        for problem in baseline:
            print(f"  {problem}", file=sys.stderr)
        return 1

    mutants: list[tuple[str, dict[str, Any], list[dict[str, str]]]] = []

    missing_row = copy.deepcopy(rows)
    missing_row.pop()
    mutants.append(("population omission", copy.deepcopy(contract), missing_row))

    absolute_locator = copy.deepcopy(rows)
    absolute_locator[0]["source_locator"] = "/off-volume/source.pdf"
    mutants.append(("absolute locator", copy.deepcopy(contract), absolute_locator))

    bad_markdown_hash = copy.deepcopy(rows)
    bad_markdown_hash[0]["normalized_markdown_sha256"] = "0" * 64
    mutants.append(("Markdown hash drift", copy.deepcopy(contract), bad_markdown_hash))

    leaked_holdout = copy.deepcopy(rows)
    calibration_index = next(
        index
        for index, row in enumerate(leaked_holdout)
        if row["review_role"] == "reviewed_calibration"
    )
    leaked_holdout[calibration_index]["review_role"] = "prospective_holdout"
    mutants.append(("review leakage", copy.deepcopy(contract), leaked_holdout))

    vacuity_laundering = copy.deepcopy(rows)
    zero_index = next(
        index
        for index, row in enumerate(vacuity_laundering)
        if row["text_intent_records"] == "0"
    )
    vacuity_laundering[zero_index]["text_intent_records"] = "1"
    mutants.append(("vacuity laundering", copy.deepcopy(contract), vacuity_laundering))

    missing_relation = copy.deepcopy(contract)
    missing_relation["relations"].pop()
    mutants.append(("relation omission", missing_relation, copy.deepcopy(rows)))

    partial_stage = copy.deepcopy(contract)
    partial_stage["stage_comparison"].pop()
    mutants.append(("stage omission", partial_stage, copy.deepcopy(rows)))

    false_count = copy.deepcopy(contract)
    false_count["population_assertions"]["prospective_holdout_documents"] += 1
    mutants.append(("assertion drift", false_count, copy.deepcopy(rows)))

    failures = 0
    for label, mutant_contract, mutant_rows in mutants:
        mutant_problems, _ = validate(
            mutant_contract,
            mutant_rows,
            verify_artifacts=False,
            verify_held_out_evidence=False,
        )
        if mutant_problems:
            failures += 1
        else:
            print(f"behavioral-genericity-contract self-test missed {label}", file=sys.stderr)

    if failures != len(mutants):
        return 1
    report = read_json(ROOT / HELD_OUT_EVIDENCE_PATH)
    evidence_mutants: list[tuple[str, dict[str, Any]]] = []

    missing_attempt = copy.deepcopy(report)
    missing_attempt["attempts"].pop()
    evidence_mutants.append(("held-out attempt omission", missing_attempt))

    overlapping_split = copy.deepcopy(report)
    overlapping_split["split"]["overlapping_document_keys"] = [
        overlapping_split["split"]["prospective_document_keys"][0]
    ]
    overlapping_split["split"]["identity_disjoint"] = False
    evidence_mutants.append(("held-out identity overlap", overlapping_split))

    laundered_state = copy.deepcopy(report)
    nonpass = next(
        attempt for attempt in laundered_state["attempts"] if attempt["state"] != "pass"
    )
    nonpass["state"] = "pass"
    nonpass["failure_id"] = None
    evidence_mutants.append(("held-out state laundering", laundered_state))

    bad_interval = copy.deepcopy(report)
    measured = next(
        stratum
        for stratum in bad_interval["strata"]
        if stratum["uncertainty"]["sample_size"] > 0
    )
    measured["uncertainty"]["lower_parts_per_million"] = 1_000_000
    evidence_mutants.append(("held-out uncertainty drift", bad_interval))

    absolute_path = copy.deepcopy(report)
    absolute_path["documents"][0]["source_locator"] = "/off-volume/source.pdf"
    evidence_mutants.append(("held-out absolute path", absolute_path))

    bad_tool = copy.deepcopy(report)
    bad_tool["tool_sha256"] = "0" * 64
    evidence_mutants.append(("held-out tool drift", bad_tool))

    bad_execution_mode = copy.deepcopy(report)
    full_capture = next(
        attempt
        for attempt in bad_execution_mode["attempts"]
        if attempt["relation"] == "unchanged_source"
    )
    full_capture["execution_mode"] = "fresh_pipeline"
    evidence_mutants.append(("held-out execution provenance drift", bad_execution_mode))

    missing_detail = copy.deepcopy(report)
    noncompleted = next(
        attempt
        for attempt in missing_detail["attempts"]
        if attempt["state"] in {"unmeasurable", "invalid"}
    )
    noncompleted.pop("detail")
    evidence_mutants.append(("held-out disposition detail omission", missing_detail))

    bad_retained_digest = copy.deepcopy(report)
    bad_retained_digest["retained_evidence_sha256"] = "0" * 63
    evidence_mutants.append(("held-out retained evidence identity malformed", bad_retained_digest))

    evidence_failures = 0
    for label, mutant in evidence_mutants:
        mutant_problems: list[str] = []
        validate_held_out_evidence(contract, rows, mutant_problems, mutant)
        if mutant_problems:
            evidence_failures += 1
        else:
            print(
                f"behavioral-genericity-contract self-test missed {label}",
                file=sys.stderr,
            )
    if evidence_failures != len(evidence_mutants):
        return 1
    total = failures + evidence_failures
    expected_total = len(mutants) + len(evidence_mutants)
    print(f"behavioral-genericity-contract self-test: {total}/{expected_total} pass")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    try:
        contract = read_json(ROOT / CONTRACT_PATH)
        rows = read_population(ROOT / POPULATION_PATH)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"behavioral-genericity-contract: {error}", file=sys.stderr)
        return 1

    if not isinstance(contract, dict):
        print("behavioral-genericity-contract: contract must be an object", file=sys.stderr)
        return 1
    if args.self_test:
        return run_self_test(contract, rows)

    problems, metrics = validate(contract, rows)
    if problems:
        for problem in problems:
            print(f"behavioral-genericity-contract: {problem}", file=sys.stderr)
        return 1

    print(
        "behavioral-genericity-contract: "
        f"{metrics['current_documents']} current documents; "
        f"{metrics['text_projection_measurable_documents']} text-measurable / "
        f"{metrics['text_projection_unmeasurable_documents']} text-unmeasurable; "
        f"{metrics['reviewed_current_overlap']} reviewed calibration / "
        f"{metrics['prospective_holdout_documents']} prospective holdout; "
        f"{metrics['prospective_vendor_novel_documents']} vendor-novel / "
        f"{metrics['prospective_family_novel_documents']} family-novel; "
        f"{metrics['source_authorities_verified']} live source authorities verified / "
        f"{metrics['external_authorities_unavailable']} external unavailable"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
