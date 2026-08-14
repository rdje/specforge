#!/usr/bin/env python3
"""Validate the frozen behavioral-genericity population and relational oracle."""

from __future__ import annotations

import argparse
import copy
import csv
import hashlib
import json
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
}

EXPECTED_REVIEWED_CHANGE_KINDS = {
    "structure_preserving_paraphrase": {"sentence_paraphrase"},
    "harmless_layout": {
        "heading_layout",
        "table_layout",
        "whitespace_layout",
        "formatting_layout",
    },
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
    if manifest.get("owner") != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.b":
        problems.append("reviewed recipe manifest owner differs from .f.ii.b")
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
        problems.append("reviewed recipe relations differ from the released .f.ii.b set")
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
        if recipe.get("unaffected_complement") != REVIEWED_COMPLEMENT:
            problems.append(f"reviewed recipe {recipe_id} complement is not closed")
        exclusions = recipe.get("unmeasurable_source_surfaces")
        if (
            not isinstance(exclusions, list)
            or set(exclusions) != RICH_CAPTURE_EXCLUSIONS
            or len(exclusions) != len(set(exclusions))
        ):
            problems.append(f"reviewed recipe {recipe_id} rich-capture exclusions differ")


def validate(
    contract: dict[str, Any],
    rows: list[dict[str, str]],
    *,
    verify_artifacts: bool = True,
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
        "reviewed_recipe_manifest": RECIPE_MANIFEST_PATH.as_posix(),
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

    return problems, metrics


def run_self_test(contract: dict[str, Any], rows: list[dict[str, str]]) -> int:
    baseline, _ = validate(contract, rows, verify_artifacts=False)
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
            mutant_contract, mutant_rows, verify_artifacts=False
        )
        if mutant_problems:
            failures += 1
        else:
            print(f"behavioral-genericity-contract self-test missed {label}", file=sys.stderr)

    if failures != len(mutants):
        return 1
    print(f"behavioral-genericity-contract self-test: {failures}/{len(mutants)} pass")
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
