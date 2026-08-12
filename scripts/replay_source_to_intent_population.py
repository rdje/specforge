#!/usr/bin/env python3
"""Replay the review-locked source-to-IntentIR population in repository-local scratch.

External source authority is supplied through an untracked runtime map. Every external PDF must
reside on the repository volume; it is copied, hash-verified, and used only below the requested
`.project-data/tmp` root. The tracked reviewed dataset and canonical `generated/` tree are never
written. Cleanup is deliberately a separate, auditable exact-root operation after the manifest and
current evaluator result have been inspected and promoted.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
DATASET = Path("crates/specforge/test_data/source_to_intent_vertical/reviewed_dataset.json")
FIXTURE_BUILDER = Path("crates/specforge/test_data/source_to_intent_vertical/build_fixture.py")
PRIOR_MEMORY = Path("generated/prior_memory/corpus_memory.json")
PROJECT_TMP = Path(".project-data/tmp")
REPLAY_ENVIRONMENT_KEYS = (
    "DOCLING_DEVICE",
    "SPECFORGE_INGEST_BATCH_THRESHOLD",
    "SPECFORGE_INGEST_BATCH_PAGES",
)


class CommandFailure(RuntimeError):
    """A replay child failed, with its otherwise-hidden diagnostics retained."""

    def __init__(self, command: list[str], error: subprocess.CalledProcessError):
        self.returncode = error.returncode
        rendered = shlex.join(command)
        super().__init__(
            f"command failed with return code {error.returncode}: {rendered}\n"
            f"stdout:\n{error.stdout}\nstderr:\n{error.stderr}"
        )


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def pretty_bytes(value: object) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


def safe_repository_path(path: Path, label: str, *, below_project_tmp: bool = False) -> Path:
    if not path.parts or path.is_absolute() or ".." in path.parts:
        raise ValueError(f"{label} must be a safe repository-relative path: {path}")
    if below_project_tmp and (
        path == PROJECT_TMP or path.parts[:2] != PROJECT_TMP.parts
    ):
        raise ValueError(f"{label} must be a child of {PROJECT_TMP}: {path}")
    absolute = (ROOT / path).resolve()
    absolute.relative_to(ROOT)
    return absolute


def read_json(path: Path) -> object:
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def run_json(command: list[str]) -> dict:
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
    except subprocess.CalledProcessError as error:
        raise CommandFailure(command, error) from error
    try:
        value = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"command did not emit JSON: {shlex.join(command)}\n{completed.stdout}"
        ) from error
    if not isinstance(value, dict):
        raise RuntimeError(f"command emitted a non-object JSON value: {shlex.join(command)}")
    return value


def load_external_map(path: Path) -> dict[str, Path]:
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
        result[portable_id] = Path(source_path).resolve(strict=True)
    return result


def replay_environment_prefix() -> list[str]:
    assignments = [
        f"{key}={os.environ[key]}" for key in REPLAY_ENVIRONMENT_KEYS if key in os.environ
    ]
    return ["env", *assignments] if assignments else []


def artifact_identity(path_text: str) -> dict:
    path = Path(path_text)
    absolute = safe_repository_path(path, "replay artifact")
    if not absolute.is_file():
        raise ValueError(f"replay artifact is missing: {path}")
    return {
        "path": path.as_posix(),
        "sha256": sha256(absolute),
        "byte_count": absolute.stat().st_size,
    }


def source_path_for_document(
    document: dict,
    external_sources: dict[str, Path],
    output_root: Path,
) -> tuple[Path, dict]:
    source = document["source"]
    expected_sha = source["sha256"]
    if source["location"] == "repository":
        relative = Path(source["relative_path"])
        absolute = safe_repository_path(relative, "reviewed repository source")
        persisted = relative
        copied = False
    elif source["location"] == "external_read_only":
        portable_id = source["portable_id"]
        if portable_id not in external_sources:
            raise ValueError(f"external source map is missing {portable_id}")
        authority = external_sources[portable_id]
        if not authority.is_file():
            raise ValueError(f"external authority is not a file: {authority}")
        if authority.name != portable_id:
            raise ValueError(
                f"external authority basename differs from portable_id: {authority.name} != {portable_id}"
            )
        if authority.stat().st_dev != ROOT.stat().st_dev:
            raise ValueError(f"external authority is not on the repository volume: {portable_id}")
        persisted = output_root / "sources" / portable_id
        absolute = ROOT / persisted
        absolute.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(authority, absolute)
        copied = True
    else:
        raise ValueError(f"unknown source location for {document['document_key']}")

    actual_sha = sha256(absolute)
    if actual_sha != expected_sha:
        raise ValueError(
            f"source SHA-256 mismatch for {document['document_key']}: {actual_sha} != {expected_sha}"
        )
    portable_id = source.get("portable_id")
    if portable_id is None:
        portable_id = Path(source["relative_path"]).name
    return persisted, {
        "location": source["location"],
        "portable_id": portable_id,
        "reviewed_sha256": expected_sha,
        "replay_sha256": actual_sha,
        "byte_count": absolute.stat().st_size,
        "repository_path": persisted.as_posix(),
        "copied_from_external_authority": copied,
        "verified_equal_to_reviewed_authority": True,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-root", type=Path, required=True)
    parser.add_argument("--external-source-map", type=Path, required=True)
    args = parser.parse_args()

    output_absolute = safe_repository_path(
        args.output_root, "population replay output root", below_project_tmp=True
    )
    source_map_absolute = safe_repository_path(
        args.external_source_map, "external source map", below_project_tmp=True
    )
    if output_absolute.exists():
        raise ValueError(f"population replay output root already exists: {args.output_root}")
    if not source_map_absolute.is_file():
        raise ValueError(f"external source map is missing: {args.external_source_map}")

    dataset = read_json(ROOT / DATASET)
    if not isinstance(dataset, dict) or dataset.get("schema_version") != 1:
        raise ValueError("reviewed dataset must be a schema-version-1 object")
    documents = dataset.get("documents")
    if not isinstance(documents, list) or len(documents) != 12:
        raise ValueError("reviewed population must contain exactly 12 documents")
    external_sources = load_external_map(source_map_absolute)
    required_external = {
        document["source"]["portable_id"]
        for document in documents
        if document["source"]["location"] == "external_read_only"
    }
    if set(external_sources) != required_external:
        missing = sorted(required_external - set(external_sources))
        extra = sorted(set(external_sources) - required_external)
        raise ValueError(f"external source map coverage mismatch: missing={missing}, extra={extra}")

    rust_diff = subprocess.run(
        ["git", "diff", "--quiet", "--", "crates/specforge/src"], cwd=ROOT
    )
    if rust_diff.returncode != 0:
        raise ValueError("production Rust sources must be unchanged while qualification evidence is replayed")
    production_revision = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()

    output_absolute.mkdir(parents=True)
    replay_records = []
    for index, document in enumerate(documents, 1):
        key = document["document_key"]
        source_path, source_identity = source_path_for_document(
            document, external_sources, args.output_root
        )
        replay_root = args.output_root / "replays" / key
        command = replay_environment_prefix() + [
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
            PRIOR_MEMORY.as_posix(),
            "-",
        ]
        print(f"[{index:02d}/12] replaying {key}", file=sys.stderr, flush=True)
        report = run_json(command)
        if report.get("document_key") != key or report.get("output_root") != replay_root.as_posix():
            raise ValueError(f"replay report identity mismatch for {key}")
        replay_records.append(
            {
                "document_key": key,
                "category": document["category"],
                "source": source_identity,
                "command": shlex.join(command),
                "stages": {
                    "source_ir": artifact_identity(report["source_ir_path"]),
                    "evidence_ir": artifact_identity(report["evidence_ir"]["path"]),
                    "semantic_ir": artifact_identity(report["semantic_ir"]["path"]),
                    "intent_ir": artifact_identity(report["intent_ir"]["path"]),
                },
            }
        )

    current_dataset = args.output_root / "current_dataset.json"
    subprocess.run(
        [
            sys.executable,
            "-B",
            FIXTURE_BUILDER.as_posix(),
            "--write-replay",
            "--replay-root",
            args.output_root.as_posix(),
            "--output",
            current_dataset.as_posix(),
        ],
        cwd=ROOT,
        check=True,
    )
    current_result = run_json(
        [
            "cargo",
            "run",
            "--quiet",
            "-p",
            "specforge",
            "--example",
            "source_to_intent_eval",
            "--",
            current_dataset.as_posix(),
        ]
    )
    result_path = args.output_root / "current_result.json"
    result_absolute = ROOT / result_path
    result_absolute.write_bytes(pretty_bytes(current_result))

    manifest = {
        "schema_version": 1,
        "replay_id": "spec-to-intent-6bii-access-carrier-population",
        "owner": "SPEC-TO-INTENT-ALIGNMENT.6b.ii.b",
        "production_revision": production_revision,
        "reviewed_dataset": {
            "path": DATASET.as_posix(),
            "sha256": sha256(ROOT / DATASET),
            "document_count": len(documents),
            "cell_count": sum(len(document["cells"]) for document in documents),
        },
        "prior_memory": {
            "path": PRIOR_MEMORY.as_posix(),
            "sha256": sha256(ROOT / PRIOR_MEMORY),
            "byte_count": (ROOT / PRIOR_MEMORY).stat().st_size,
        },
        "tools": {
            "orchestrator": {
                "path": Path(__file__).resolve().relative_to(ROOT).as_posix(),
                "sha256": sha256(Path(__file__)),
            },
            "projection": {
                "path": FIXTURE_BUILDER.as_posix(),
                "sha256": sha256(ROOT / FIXTURE_BUILDER),
            },
        },
        "population": replay_records,
        "current_dataset": artifact_identity(current_dataset.as_posix()),
        "current_result": artifact_identity(result_path.as_posix()),
        "cleanup": {
            "external_source_map": args.external_source_map.as_posix(),
            "population_root": args.output_root.as_posix(),
            "status": "pending_exact_cleanup",
        },
    }
    manifest_path = args.output_root / "replay_manifest.json"
    (ROOT / manifest_path).write_bytes(pretty_bytes(manifest))
    print(f"wrote {manifest_path}", file=sys.stderr)
    print(json.dumps({"manifest": manifest_path.as_posix(), "result": result_path.as_posix()}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
