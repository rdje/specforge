#!/usr/bin/env python3
"""Census the standing SourceIR reproducibility of the persisted corpus.

`SOURCE-IR-REPRODUCIBILITY.1`. `scripts/check_chain_currency.sh` replays every stage at its fixed
persisted input, so it starts from `source_ir.json` and never re-runs ingest: a green chain says
nothing about the artifact all of its stages descend from. This census is the missing measurement.

It is READ-ONLY with respect to the tracked tree and `generated/`: it reads persisted artifacts,
re-ingests a declared sample through the isolated `source_to_intent_replay` example below a fresh
`.project-data/tmp` root, and compares. Nothing under `generated/` is written or removed.

Measurability is STATED, never implied. Every persisted document lands in exactly one stratum:

  live_measured        schema-3 artifact whose exact recorded source is on the repository volume
  live_unmeasurable    schema-3 artifact whose source is not resolvable, or whose bytes moved
  legacy_unmeasurable  pre-schema-3 artifact; the current toolchain cannot emit that artifact at
                       all, so "does it reproduce" has no answer rather than a negative one

`--plan-only` prints the partition without ingesting, so the declared sample can be reviewed before
any compute is spent.

External source authority never enters the tracked tree: it is supplied through the same untracked
runtime map shape `scripts/replay_source_to_intent_population.py` uses. Without a map, external
documents are simply reported unmeasurable.
"""

from __future__ import annotations

import argparse
import difflib
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
PERSISTED_SOURCE_ROOT = Path("generated/source_ir")
PROJECT_TMP = Path(".project-data/tmp")
CURRENT_SOURCE_IR_SCHEMA = 3

# Passed through to every replay child so a census inherits the caller's declared ingest regime.
REPLAY_ENVIRONMENT_KEYS = (
    "DOCLING_DEVICE",
    "SPECFORGE_INGEST_BATCH_THRESHOLD",
    "SPECFORGE_INGEST_BATCH_PAGES",
)

# Back-annotated by `specforge validate` AFTER the stage ran. `check_chain_currency.sh`'s
# `compare_stage_artifact` deletes exactly these three, and the product's own `*_ir_fingerprint`
# helpers clear `validation_reports` before hashing. `proof_context`/`proof_ledger` attest that same
# post-build mutation — the SourceIR capture digest is the hash of the whole premise map, so one
# extra `validation_reports` premise re-scopes every claim. Excluding them loses no content
# authority: each premise mirrors an artifact field that is compared directly, and the structural
# residue of the proof surface is re-checked field-by-field in `compare_proof_surface`.
EXCLUDED_SECTIONS = ("validation_reports", "proof_context", "proof_ledger")

# The input's LOCATION, not its content. Equal by construction for a repository source and
# necessarily different for an external source staged into the repository. Input identity is
# carried instead by `source.size_bytes` (compared) and the measured SHA-256 (reported).
EXCLUDED_SOURCE_FIELDS = ("requested_path", "canonical_path", "path_origin")

# Emitted by `neutralize_legacy_source_classifications` when a pre-schema-3 artifact is loaded for
# migration (crates/specforge/src/ir/source.rs). It records how the PERSISTED artifact came to be,
# and a fresh ingest has no reason to carry it. Removal is per-document and reported, never silent.
LEGACY_MIGRATION_NOTE_PREFIX = "loaded legacy SourceIR schema "

COUNTED_COLLECTIONS = (
    "content_elements",
    "structured_tables",
    "page_artifacts",
    "visual_assets",
    "document_sections",
)


class CommandFailure(RuntimeError):
    """A replay child failed, with its otherwise-hidden diagnostics retained."""

    def __init__(self, command: list[str], error: subprocess.CalledProcessError):
        self.returncode = error.returncode
        super().__init__(
            f"command failed with return code {error.returncode}: {shlex.join(command)}\n"
            f"stdout:\n{error.stdout}\nstderr:\n{error.stderr}"
        )


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def census_identifier(value: str, label: str) -> str:
    if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]{0,63}", value):
        raise ValueError(f"{label} must be 1-64 portable identifier characters: {value!r}")
    return value


def safe_repository_path(path: Path, label: str, *, below_project_tmp: bool = False) -> Path:
    if path.is_absolute():
        raise ValueError(f"{label} must be repository-root-relative: {path}")
    if any(part == ".." for part in path.parts):
        raise ValueError(f"{label} must not traverse upward: {path}")
    if below_project_tmp and path.parts[: len(PROJECT_TMP.parts)] != PROJECT_TMP.parts:
        raise ValueError(f"{label} must live below {PROJECT_TMP}: {path}")
    return ROOT / path


def read_json(path: Path) -> object:
    with path.open("r", encoding="utf-8") as stream:
        return json.load(stream)


def run_json(command: list[str]) -> dict:
    completed = subprocess.run(
        command, cwd=ROOT, check=True, capture_output=True, text=True
    )
    value = json.loads(completed.stdout)
    if not isinstance(value, dict):
        raise RuntimeError(f"command emitted a non-object JSON value: {shlex.join(command)}")
    return value


def load_external_map(path: Path) -> dict[str, Path]:
    """Load the untracked runtime map of external source authorities.

    Same schema and same predicates as the population replay orchestrator: repository-relative map
    below `.project-data/tmp`, absolute runtime paths, basename equal to the portable id, and the
    authority on the repository's own filesystem.
    """
    value = read_json(path)
    if not isinstance(value, dict) or value.get("schema_version") != 1:
        raise ValueError("external source map must be a schema-version-1 object")
    rows = value.get("sources")
    if not isinstance(rows, list):
        raise ValueError("external source map must contain a sources array")
    result: dict[str, Path] = {}
    for row in rows:
        if not isinstance(row, dict) or set(row) != {"portable_id", "path"}:
            raise ValueError("each external source row must contain only portable_id and path")
        portable_id = row["portable_id"]
        source_path = row["path"]
        if not isinstance(portable_id, str) or not portable_id:
            raise ValueError("external portable_id must be a non-empty string")
        if not isinstance(source_path, str) or not Path(source_path).is_absolute():
            raise ValueError(f"external source path must be absolute at runtime: {portable_id}")
        if portable_id in result:
            raise ValueError(f"duplicate external portable_id: {portable_id}")
        authority = Path(source_path).resolve(strict=True)
        if authority.name != portable_id:
            raise ValueError(
                f"external authority basename differs from portable_id: {authority.name}"
            )
        if authority.stat().st_dev != ROOT.stat().st_dev:
            raise ValueError(f"external authority is not on the repository volume: {portable_id}")
        result[portable_id] = authority
    return result


def replay_command(source_path: Path, replay_root: Path) -> list[str]:
    assignments = [
        f"{key}={os.environ[key]}" for key in REPLAY_ENVIRONMENT_KEYS if key in os.environ
    ]
    prefix = ["env", *assignments] if assignments else []
    return prefix + [
        "cargo",
        "run",
        "--quiet",
        "-p",
        "specforge",
        "--example",
        "source_to_intent_replay",
        "--",
        source_path.as_posix(),
        replay_root.as_posix(),
        "-",
        "-",
    ]


# --------------------------------------------------------------------------------------------
# Frame construction
# --------------------------------------------------------------------------------------------


def persisted_documents() -> list[tuple[str, Path]]:
    root = ROOT / PERSISTED_SOURCE_ROOT
    if not root.is_dir():
        return []
    found = []
    for directory in sorted(root.iterdir()):
        artifact = directory / "source_ir.json"
        if artifact.is_file():
            found.append((directory.name, artifact))
    return found


def classify_document(
    key: str,
    artifact: Path,
    external_sources: dict[str, Path],
    output_root: Path,
) -> dict:
    """Place one persisted document in exactly one stratum and record why."""
    persisted = read_json(artifact)
    schema_version = persisted["schema_version"]
    source = persisted["source"]
    recorded_path = source["canonical_path"]
    recorded_bytes = source["size_bytes"]
    record = {
        "document_key": key,
        "persisted_artifact": (PERSISTED_SOURCE_ROOT / key / "source_ir.json").as_posix(),
        "persisted_artifact_sha256": sha256(artifact),
        "schema_version": schema_version,
        "recorded_source_bytes": recorded_bytes,
        "content_element_count": len(persisted["content_elements"]),
    }

    if schema_version != CURRENT_SOURCE_IR_SCHEMA:
        record["stratum"] = "legacy_unmeasurable"
        record["reason"] = (
            f"persisted artifact is SourceIR schema {schema_version}; the current toolchain emits "
            f"schema {CURRENT_SOURCE_IR_SCHEMA} with a verified proof ledger, so no ingest can "
            "reproduce this artifact"
        )
        return record

    candidate = Path(recorded_path)
    if not candidate.is_absolute() and (ROOT / candidate).is_file():
        record["source_location"] = "repository"
        record["source_path"] = candidate.as_posix()
        record["stage_source"] = None
    else:
        portable_id = candidate.name
        record["source_portable_id"] = portable_id
        if portable_id not in external_sources:
            record["stratum"] = "live_unmeasurable"
            record["source_location"] = "absent"
            record["reason"] = (
                "the recorded source is not a repository file and no external source map entry "
                f"supplies {portable_id} on the repository volume"
            )
            return record
        record["source_location"] = "external_read_only"
        record["source_path"] = (output_root / "sources" / portable_id).as_posix()
        record["stage_source"] = external_sources[portable_id].as_posix()

    record["stratum"] = "live_measured"
    return record


def stage_source(record: dict, *, copy: bool) -> Path:
    """Resolve the census's own copy of one document's source, staging externals on demand."""
    destination = ROOT / Path(record["source_path"])
    authority = record.get("stage_source")
    if copy and authority is not None:
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(Path(authority), destination)
    if not destination.is_file():
        raise ValueError(f"census source is missing: {record['source_path']}")
    return destination


# --------------------------------------------------------------------------------------------
# Comparison
# --------------------------------------------------------------------------------------------


def canonical(value: object) -> str:
    return json.dumps(value, sort_keys=True, ensure_ascii=False)


def normalized_replay_artifact(replay_artifact: Path, replay_base_root: str) -> dict:
    """Load a replayed artifact with its own output root rewritten to the persisted one.

    `SourceIR` embeds its own output paths in `artifact_layout`, `normalization_plan`, and every
    page/visual record, so two replays into differently named roots differ everywhere by
    construction. That is not drift; it is the root. Rewrite it, then compare exactly.
    """
    if not replay_base_root.startswith(f"{PROJECT_TMP.as_posix()}/"):
        raise ValueError(f"replay base root must live below {PROJECT_TMP}: {replay_base_root}")
    text = replay_artifact.read_text(encoding="utf-8")
    return json.loads(text.replace(replay_base_root, PERSISTED_SOURCE_ROOT.as_posix()))


def strip_legacy_migration_note(artifact: dict) -> bool:
    notes = artifact.get("normalization_plan", {}).get("notes")
    if not isinstance(notes, list):
        return False
    kept = [note for note in notes if not str(note).startswith(LEGACY_MIGRATION_NOTE_PREFIX)]
    if len(kept) == len(notes):
        return False
    artifact["normalization_plan"]["notes"] = kept
    return True


def comparable(artifact: dict) -> dict:
    reduced = {key: value for key, value in artifact.items() if key not in EXCLUDED_SECTIONS}
    source = dict(reduced.get("source", {}))
    for field in EXCLUDED_SOURCE_FIELDS:
        source.pop(field, None)
    reduced["source"] = source
    return reduced


def validation_premise_key(key: str) -> bool:
    return key == "validation_reports" or key.startswith("validation_reports[")


def compare_proof_surface(persisted: dict, replayed: dict) -> dict:
    """Re-check the proof surface excluded from the byte comparison, without reconstructing it.

    The claim `scope` is the digest of the whole premise map, so it necessarily moves when
    post-build validation adds a premise. What must still hold is that both artifacts were proved
    by the same ruleset over the same addresses.
    """
    persisted_ledger = persisted.get("proof_ledger", {})
    replayed_ledger = replayed.get("proof_ledger", {})
    persisted_keys = {
        key
        for key in persisted.get("proof_context", {}).get("field_premises", {})
        if not validation_premise_key(key)
    }
    replayed_keys = {
        key
        for key in replayed.get("proof_context", {}).get("field_premises", {})
        if not validation_premise_key(key)
    }

    def addresses(ledger: dict) -> set[str]:
        return {
            canonical(claim["address"])
            for claim in ledger.get("claims", [])
            if not validation_premise_key(claim["address"].get("surface", ""))
        }

    persisted_addresses = addresses(persisted_ledger)
    replayed_addresses = addresses(replayed_ledger)
    return {
        "ruleset_sha256_equal": persisted_ledger.get("ruleset_sha256")
        == replayed_ledger.get("ruleset_sha256"),
        "persisted_ruleset_sha256": persisted_ledger.get("ruleset_sha256"),
        "replayed_ruleset_sha256": replayed_ledger.get("ruleset_sha256"),
        "premise_keys_equal": persisted_keys == replayed_keys,
        "premise_key_count": len(persisted_keys),
        "premise_keys_only_persisted": sorted(persisted_keys - replayed_keys)[:8],
        "premise_keys_only_replayed": sorted(replayed_keys - persisted_keys)[:8],
        "claim_addresses_equal": persisted_addresses == replayed_addresses,
        "claim_address_count": len(persisted_addresses),
    }


def content_drift_shape(persisted: dict, replayed: dict) -> dict:
    """Classify HOW the element stream moved, not merely that it did.

    The competing readings of a drifted document are opposite in consequence: current ingest
    LOSING persisted content, versus current ingest GAINING content the earlier run missed. A
    text-level opcode diff separates them, and the `kind` histogram of what was added says which
    part of the document the change came from.
    """
    left = [str(element.get("text", "")) for element in persisted.get("content_elements", [])]
    right = [str(element.get("text", "")) for element in replayed.get("content_elements", [])]
    added: list[int] = []
    removed: list[int] = []
    matcher = difflib.SequenceMatcher(a=left, b=right, autojunk=False)
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag in ("insert", "replace"):
            added.extend(range(j1, j2))
        if tag in ("delete", "replace"):
            removed.extend(range(i1, i2))
    added_kinds: dict[str, int] = {}
    for index in added:
        kind = str(replayed["content_elements"][index].get("kind", "unknown"))
        added_kinds[kind] = added_kinds.get(kind, 0) + 1
    # A removed element is not the same thing as removed content. Docling may split or merge the
    # same prose differently, which drops an element while keeping every word. Only text that no
    # longer appears anywhere in the replayed stream is content the current toolchain actually
    # loses, and that is the reading with real consequences.
    replayed_text = " \n".join(right)
    absent = [left[index] for index in removed if left[index] not in replayed_text]
    return {
        "added_elements": len(added),
        "removed_elements": len(removed),
        "purely_additive": bool(added) and not removed,
        "removed_but_text_retained": len(removed) - len(absent),
        "removed_text_absent_from_replay": len(absent),
        "added_element_kinds": dict(sorted(added_kinds.items())),
        "added_text_samples": [right[index][:80] for index in added[:8]],
        "absent_text_samples": [text[:110] for text in absent[:4]],
    }


def first_content_divergence(persisted: dict, replayed: dict) -> dict | None:
    left = persisted.get("content_elements", [])
    right = replayed.get("content_elements", [])
    for index, (a, b) in enumerate(zip(left, right)):
        if canonical(a) != canonical(b):
            return {
                "ordinal": index,
                "persisted_element_id": a.get("element_id"),
                "replayed_element_id": b.get("element_id"),
                "persisted_text_prefix": str(a.get("text", ""))[:120],
                "replayed_text_prefix": str(b.get("text", ""))[:120],
            }
    if len(left) != len(right):
        return {
            "ordinal": min(len(left), len(right)),
            "persisted_element_id": None,
            "replayed_element_id": None,
            "persisted_text_prefix": "",
            "replayed_text_prefix": "",
        }
    return None


def compare_artifacts(persisted: dict, replayed: dict) -> dict:
    proof = compare_proof_surface(persisted, replayed)
    persisted_reduced = comparable(persisted)
    replayed_reduced = comparable(replayed)
    legacy_note_removed = strip_legacy_migration_note(persisted_reduced)
    strip_legacy_migration_note(replayed_reduced)

    sections = sorted(set(persisted_reduced) | set(replayed_reduced))
    differing = [
        section
        for section in sections
        if canonical(persisted_reduced.get(section)) != canonical(replayed_reduced.get(section))
    ]
    counts = {
        collection: {
            "persisted": len(persisted.get(collection, [])),
            "replayed": len(replayed.get(collection, [])),
        }
        for collection in COUNTED_COLLECTIONS
    }
    result = {
        "verdict": "reproduced" if not differing else "drifted",
        "differing_sections": differing,
        "legacy_migration_note_removed": legacy_note_removed,
        "collection_counts": counts,
        "proof_surface": proof,
    }
    if differing:
        result["first_content_divergence"] = first_content_divergence(persisted, replayed)
        result["content_drift_shape"] = content_drift_shape(persisted, replayed)
    return result


# --------------------------------------------------------------------------------------------
# Self-test — the controlled negatives that keep "reproduced" from being the default answer
# --------------------------------------------------------------------------------------------


def _fixture() -> dict:
    """A minimal artifact with one of every surface the comparison reasons about."""
    return {
        "schema_version": CURRENT_SOURCE_IR_SCHEMA,
        "stage": "source_ir",
        "source": {
            "requested_path": "corpus/example.pdf",
            "canonical_path": "corpus/example.pdf",
            "path_origin": "repository_owned",
            "size_bytes": 1024,
        },
        "artifact_layout": {"artifact_root": "generated/source_ir/example"},
        "normalization_plan": {"status": "ready", "notes": ["docling backend version: 0.0.0"]},
        "content_elements": [
            {"element_id": "elem_00001", "text": "first"},
            {"element_id": "elem_00002", "text": "second"},
        ],
        "structured_tables": [{"table_id": "table_0001", "table_kind": "timing_parameter"}],
        "page_artifacts": [{"page_number": 1}],
        "visual_assets": [{"asset_id": "picture_0001", "diagram_kind": "unknown"}],
        "document_sections": [{"section_id": "section_0001", "section_kind": "normative"}],
        "document_profile": {"page_count": 1},
        "validation_reports": [],
        "proof_context": {
            "schema_version": 1,
            "field_premises": {"schema_version": 3, "content_elements": []},
        },
        "proof_ledger": {
            "schema_version": 1,
            "ruleset_sha256": "a" * 64,
            "claims": [
                {"address": {"stage": "source_ir", "surface": "schema_version"}},
                {"address": {"stage": "source_ir", "surface": "content_elements"}},
            ],
        },
    }


def _case(name: str, mutate) -> tuple[str, dict]:
    persisted, replayed = _fixture(), _fixture()
    mutate(persisted, replayed)
    return name, compare_artifacts(persisted, replayed)


def run_self_test() -> int:
    """Prove each declared exclusion is exactly as wide as it claims, and no wider."""

    def note(persisted, replayed):
        persisted["normalization_plan"]["notes"].append(
            f"{LEGACY_MIGRATION_NOTE_PREFIX}1 for inspection only"
        )

    cases = [
        _case("identical", lambda p, r: None),
        _case(
            "content-element-text-changed",
            lambda p, r: r["content_elements"][1].update({"text": "SECOND"}),
        ),
        _case(
            "content-element-inserted",
            lambda p, r: r["content_elements"].insert(
                1, {"element_id": "elem_00099", "text": "figure interior"}
            ),
        ),
        _case(
            "content-element-removed",
            lambda p, r: r["content_elements"].pop(),
        ),
        _case(
            "content-element-resegmented",
            lambda p, r: r["content_elements"].__setitem__(
                slice(0, 2), [{"element_id": "elem_00001", "text": "first second"}]
            ),
        ),
        _case(
            "table-kind-changed",
            lambda p, r: r["structured_tables"][0].update({"table_kind": "unknown"}),
        ),
        _case("legacy-migration-note-only-on-persisted", note),
        _case(
            "unrelated-note-only-on-persisted",
            lambda p, r: p["normalization_plan"]["notes"].append("an unrelated note"),
        ),
        _case(
            "validation-report-only-on-persisted",
            lambda p, r: p["validation_reports"].append({"artifact_fingerprint": "f" * 64}),
        ),
        _case(
            "source-bytes-changed",
            lambda p, r: r["source"].update({"size_bytes": 2048}),
        ),
        _case(
            "source-path-changed",
            lambda p, r: r["source"].update(
                {
                    "requested_path": ".project-data/tmp/c/sources/example.pdf",
                    "canonical_path": ".project-data/tmp/c/sources/example.pdf",
                    "path_origin": "external_read_only",
                }
            ),
        ),
        _case(
            "ruleset-changed",
            lambda p, r: r["proof_ledger"].update({"ruleset_sha256": "b" * 64}),
        ),
        _case(
            "claim-address-removed",
            lambda p, r: r["proof_ledger"]["claims"].pop(),
        ),
    ]
    results = dict(cases)

    def expect(condition: bool, message: str) -> bool:
        if not condition:
            print(f"[source-ir-census] FAIL: {message}", file=sys.stderr)
        return condition

    checks = [
        expect(results["identical"]["verdict"] == "reproduced", "identical artifacts must reproduce"),
        expect(
            results["content-element-text-changed"]["verdict"] == "drifted"
            and results["content-element-text-changed"]["differing_sections"] == ["content_elements"]
            and results["content-element-text-changed"]["first_content_divergence"]["ordinal"] == 1,
            "a changed element text must be reported at its exact ordinal",
        ),
        expect(
            results["content-element-inserted"]["verdict"] == "drifted"
            and results["content-element-inserted"]["collection_counts"]["content_elements"]
            == {"persisted": 2, "replayed": 3}
            and results["content-element-inserted"]["first_content_divergence"]["ordinal"] == 1,
            "an inserted element must move the count and the first divergence",
        ),
        expect(
            results["content-element-inserted"]["content_drift_shape"]["purely_additive"]
            and results["content-element-inserted"]["content_drift_shape"]["added_elements"] == 1
            and results["content-element-inserted"]["content_drift_shape"]["removed_elements"] == 0,
            "a gained element must be classified as purely additive",
        ),
        expect(
            not results["content-element-removed"]["content_drift_shape"]["purely_additive"]
            and results["content-element-removed"]["content_drift_shape"]["removed_elements"] == 1
            and results["content-element-removed"]["content_drift_shape"][
                "removed_text_absent_from_replay"
            ]
            == 1,
            "a lost element must never be classified as purely additive, and its text must read as absent",
        ),
        expect(
            results["content-element-resegmented"]["content_drift_shape"][
                "removed_text_absent_from_replay"
            ]
            == 0
            and results["content-element-resegmented"]["content_drift_shape"][
                "removed_but_text_retained"
            ]
            == 2,
            "merged prose must read as re-segmented, never as lost content",
        ),
        expect(
            results["table-kind-changed"]["differing_sections"] == ["structured_tables"],
            "a re-classified table must drift without contaminating another section",
        ),
        expect(
            results["legacy-migration-note-only-on-persisted"]["verdict"] == "reproduced"
            and results["legacy-migration-note-only-on-persisted"]["legacy_migration_note_removed"],
            "the migration note must be removed, and its removal reported",
        ),
        expect(
            results["unrelated-note-only-on-persisted"]["verdict"] == "drifted"
            and not results["unrelated-note-only-on-persisted"]["legacy_migration_note_removed"],
            "note handling must exempt the migration note only, never notes as a class",
        ),
        expect(
            results["validation-report-only-on-persisted"]["verdict"] == "reproduced",
            "post-build validation back-annotation must not read as drift",
        ),
        expect(
            results["source-bytes-changed"]["verdict"] == "drifted"
            and results["source-bytes-changed"]["differing_sections"] == ["source"],
            "input identity must stay inside the comparison",
        ),
        expect(
            results["source-path-changed"]["verdict"] == "reproduced",
            "an externally staged source must not read as drift on its path alone",
        ),
        expect(
            results["ruleset-changed"]["verdict"] == "reproduced"
            and not results["ruleset-changed"]["proof_surface"]["ruleset_sha256_equal"],
            "an excluded proof ledger must still be checked structurally",
        ),
        expect(
            not results["claim-address-removed"]["proof_surface"]["claim_addresses_equal"],
            "a lost claim address must be visible on the proof surface",
        ),
    ]
    passed = sum(1 for check in checks if check)
    print(f"[source-ir-census] self-test {passed}/{len(checks)} comparison cases pass.")
    return 0 if passed == len(checks) else 1


# --------------------------------------------------------------------------------------------
# Driver
# --------------------------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-root", type=Path)
    parser.add_argument("--census-id")
    parser.add_argument("--owner")
    parser.add_argument("--external-source-map", type=Path)
    parser.add_argument("--plan-only", action="store_true")
    parser.add_argument(
        "--compare-only",
        action="store_true",
        help="re-derive the comparison from an existing output root's retained replays",
    )
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        if args.output_root or args.census_id or args.owner or args.external_source_map:
            raise ValueError("--self-test takes no other argument")
        return run_self_test()
    if args.plan_only and args.compare_only:
        raise ValueError("--plan-only and --compare-only are mutually exclusive")
    for required in ("output_root", "census_id", "owner"):
        if getattr(args, required) is None:
            raise ValueError(f"--{required.replace('_', '-')} is required")

    census_id = census_identifier(args.census_id, "census id")
    owner = census_identifier(args.owner, "owner")
    output_absolute = safe_repository_path(
        args.output_root, "census output root", below_project_tmp=True
    )

    external_sources: dict[str, Path] = {}
    external_map_path = None
    if args.external_source_map is not None:
        map_absolute = safe_repository_path(
            args.external_source_map, "external source map", below_project_tmp=True
        )
        if not map_absolute.is_file():
            raise ValueError(f"external source map is missing: {args.external_source_map}")
        external_sources = load_external_map(map_absolute)
        external_map_path = args.external_source_map.as_posix()

    documents = persisted_documents()
    if not documents:
        raise ValueError(f"no persisted SourceIR artifacts under {PERSISTED_SOURCE_ROOT}")

    records = [
        classify_document(key, artifact, external_sources, args.output_root)
        for key, artifact in documents
    ]
    sample = [record for record in records if record["stratum"] == "live_measured"]

    strata: dict[str, int] = {}
    for record in records:
        strata[record["stratum"]] = strata.get(record["stratum"], 0) + 1

    # "the current toolchain" must name a revision. A dirty production tree would make the
    # replayed binary un-attributable, so the census refuses rather than publishing an
    # unreproducible comparison.
    if not args.compare_only and (
        subprocess.run(["git", "diff", "--quiet", "--", "crates"], cwd=ROOT).returncode != 0
    ):
        raise ValueError("production Rust sources must be unmodified while the census re-ingests")
    revision = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, check=True, capture_output=True, text=True
    ).stdout.strip()
    tree_clean = (
        subprocess.run(["git", "status", "--porcelain"], cwd=ROOT, capture_output=True, text=True)
        .stdout.strip()
        == ""
    )

    census = {
        "schema_version": 1,
        "census_id": census_id,
        "mode": "compare_only" if args.compare_only else "re_ingest",
        "owner": owner,
        "production_revision": revision,
        "tree_clean_at_start": tree_clean,
        "persisted_root": PERSISTED_SOURCE_ROOT.as_posix(),
        "output_root": args.output_root.as_posix(),
        "external_source_map": external_map_path,
        "selection_rule": (
            "every persisted generated/source_ir/<key>/source_ir.json is placed in exactly one "
            "stratum: live_measured when the artifact is SourceIR schema 3 and its exact recorded "
            "source resolves on the repository volume, live_unmeasurable when a schema-3 artifact's "
            "source does not, and legacy_unmeasurable when the artifact predates schema 3. The "
            "sample is the whole live_measured stratum; there is no discretionary selection inside "
            "it."
        ),
        "comparison_rule": {
            "root_normalization": (
                "the replay artifact base root is rewritten to the persisted root before parsing, "
                "because SourceIR embeds its own output paths"
            ),
            "excluded_sections": list(EXCLUDED_SECTIONS),
            "excluded_sections_reason": (
                "post-build `specforge validate` back-annotation; the same three sections that "
                "scripts/check_chain_currency.sh excludes, re-checked structurally under "
                "proof_surface"
            ),
            "excluded_source_fields": list(EXCLUDED_SOURCE_FIELDS),
            "excluded_source_fields_reason": (
                "input location rather than content; input identity is carried by "
                "source.size_bytes, which is compared, and by the reported source SHA-256"
            ),
            "legacy_migration_note_prefix": LEGACY_MIGRATION_NOTE_PREFIX,
        },
        "frame_size": len(records),
        "strata": strata,
        "documents": records,
    }

    if args.plan_only:
        plan = dict(census)
        # `stage_source` is the runtime authority path: machine-specific, and never part of any
        # record a reader may keep. The portable id already identifies the input.
        plan["documents"] = [
            {key: value for key, value in record.items() if key != "stage_source"}
            for record in records
        ]
        json.dump(plan, sys.stdout, indent=2, sort_keys=False)
        sys.stdout.write("\n")
        return 0

    if args.compare_only:
        if not output_absolute.is_dir():
            raise ValueError(f"census output root does not exist: {args.output_root}")
    elif output_absolute.exists():
        raise ValueError(f"census output root already exists: {args.output_root}")
    else:
        output_absolute.mkdir(parents=True)

    for index, record in enumerate(sample, 1):
        key = record["document_key"]
        source_path = Path(record["source_path"])
        staged = stage_source(record, copy=not args.compare_only)
        # The external authority's runtime path is machine-specific and never durable evidence;
        # the portable id plus the measured digest carry the input's identity instead.
        record.pop("stage_source", None)
        record["source_sha256"] = sha256(staged)
        record["staged_source_bytes"] = staged.stat().st_size
        if record["staged_source_bytes"] != record["recorded_source_bytes"]:
            record["stratum"] = "live_unmeasurable"
            record["reason"] = (
                f"resolved source is {record['staged_source_bytes']} bytes but the persisted "
                f"artifact records {record['recorded_source_bytes']}; this is a different input"
            )
            continue

        replay_root = args.output_root / "replays" / key
        replay_base_root = (replay_root / "source_ir").as_posix()
        replayed_artifact = Path(replay_base_root) / key / "source_ir.json"
        if args.compare_only:
            if not (ROOT / replayed_artifact).is_file():
                record["stratum"] = "live_unmeasurable"
                record["reason"] = (
                    f"--compare-only found no retained replay artifact at {replayed_artifact}"
                )
                continue
            print(f"[{index:02d}/{len(sample)}] comparing {key}", file=sys.stderr, flush=True)
        else:
            command = replay_command(source_path, replay_root)
            print(f"[{index:02d}/{len(sample)}] re-ingesting {key}", file=sys.stderr, flush=True)
            try:
                report = run_json(command)
            except subprocess.CalledProcessError as error:
                # One document that will not ingest is an unmeasurable document, not a reason to
                # discard the census. Record why, keep going, and let the census state it.
                record["stratum"] = "live_unmeasurable"
                record["reason"] = (
                    f"re-ingest failed with return code {error.returncode}: "
                    + " ".join((error.stderr or "").split())[-400:]
                )
                print(f"  ! {key} did not re-ingest; recorded unmeasurable", file=sys.stderr)
                continue
            if report.get("document_key") != key:
                raise ValueError(f"replay report identity mismatch for {key}")
            if Path(report["source_ir_path"]) != replayed_artifact:
                raise ValueError(f"replay wrote an unexpected artifact path for {key}")
            record["replay_command"] = shlex.join(command)

        replayed = normalized_replay_artifact(ROOT / replayed_artifact, replay_base_root)
        persisted = read_json(ROOT / Path(record["persisted_artifact"]))
        record["replayed_artifact"] = replayed_artifact.as_posix()
        record["comparison"] = compare_artifacts(persisted, replayed)

    measured = [record for record in records if record["stratum"] == "live_measured"]
    census["strata"] = {}
    for record in records:
        census["strata"][record["stratum"]] = census["strata"].get(record["stratum"], 0) + 1
    census["result"] = {
        "measured": len(measured),
        "reproduced": sum(
            1 for record in measured if record["comparison"]["verdict"] == "reproduced"
        ),
        "drifted": sum(1 for record in measured if record["comparison"]["verdict"] == "drifted"),
    }

    report_path = output_absolute / "source_ir_reproducibility_census.json"
    report_path.write_text(json.dumps(census, indent=2) + "\n", encoding="utf-8")
    json.dump(census["result"] | {"strata": census["strata"]}, sys.stdout, indent=2)
    sys.stdout.write(f"\nreport: {(args.output_root / report_path.name).as_posix()}\n")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, CommandFailure, RuntimeError) as error:
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
