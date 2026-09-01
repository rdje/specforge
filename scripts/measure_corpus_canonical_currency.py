#!/usr/bin/env python3
"""Read-only census for WIRE-BASED-100.9a — how much of the persisted corpus can the current
binary actually score, and which task-tree owns each document that it cannot?

Two separate gates already publish a coverage number, and neither answers this question:

* `scripts/check_chain_currency.sh` reports `24 replayed / 24 current / 0 stale`. That is a
  currency statement about the REBUILDABLE stratum only; it declares the rest UNMEASURABLE and
  does not count them.
* `scripts/check_corpus_frontier.sh` reports `57 cohort = 52 refreshed + 5 remaining`. That is a
  completion statement about the host-library refresh PROGRAM (`CORPUS-COVERAGE.2`), whose
  cohort rule excludes every `corpus/`-sourced document by contract, and whose `refreshed` set
  was declared under an older SourceIR schema.

So a reader who wants "how much of the persisted corpus is canonically usable" is offered 100%
by one gate and 91% by the other, while the derived answer is neither. This census derives it,
and derives the OWNER of each shortfall, so a re-ingest can be given a leaf instead of a guess.

Definition used throughout. A document is MEASURABLE when its persisted EvidenceIR is at the
current canonical schema — the exact predicate `eval-extraction` applies before it will score a
document (`unmeasurable_disposition`, crates/specforge/src/commands/eval_extraction.rs). Every
other document is LEGACY: refused by design, inspection-only, with a known re-ingest route.

Nothing is hardcoded that the repository already states. The canonical schema version of each
stage is read out of the Rust sources that define it, so this reproducer fails loudly rather
than silently agreeing with itself after a schema bump. Frontier ownership is read out of the
frontier's own contract (`doctrine/corpus_frontier/census.json`), including its cohort rule.
The scored-document set is derived from the tracked eval datasets' own `doc_key` fields, so no
protocol, vendor, or document name appears in this file's logic (ADR 0006).

Reads only persisted `generated/*_ir/<key>/*.json` headers, the four schema constants, the
frontier contract, and the eval datasets. Writes nothing, runs no model, rebuilds no stage, and
is safe to run at any time. Paths resolve from the repository root so the reproducer moves with
the repository.

    python3 scripts/measure_corpus_canonical_currency.py [--json]
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys

# Each stage: the persisted artifact, and the Rust source plus constant that define its current
# canonical schema version. Reading the constant instead of restating it is what makes this
# census stale-detecting rather than self-confirming.
STAGES = (
    ("source", "generated/source_ir", "source_ir.json",
     "crates/specforge/src/ir/source.rs", "SOURCE_IR_SCHEMA_VERSION"),
    ("evidence", "generated/evidence_ir", "evidence_ir.json",
     "crates/specforge/src/ir/evidence.rs", "EVIDENCE_IR_SCHEMA_VERSION"),
    ("semantic", "generated/semantic_ir", "semantic_ir.json",
     "crates/specforge/src/ir/semantic.rs", "SEMANTIC_IR_SCHEMA_VERSION"),
    ("intent", "generated/intent_ir", "intent_ir.json",
     "crates/specforge/src/ir/intent.rs", "INTENT_IR_SCHEMA_VERSION"),
)

# The stage whose persisted schema decides whether `eval-extraction` will score a document.
GATING_STAGE = "evidence"

EVAL_DATASET_DIR = "crates/specforge/test_data/llm_eval"
FRONTIER_CONTRACT = "doctrine/corpus_frontier/census.json"


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def read_schema_constant(root: str, rel_path: str, name: str) -> int:
    """Read `const <name>: u32 = <n>;` out of the Rust source that owns it."""
    path = os.path.join(root, rel_path)
    with open(path, "r", encoding="utf-8") as handle:
        text = handle.read()
    match = re.search(rf"^\s*(?:pub\s+)?const\s+{re.escape(name)}\s*:\s*u32\s*=\s*(\d+)\s*;",
                      text, re.MULTILINE)
    if not match:
        raise SystemExit(f"error: {name} not found in {rel_path}; the census cannot be trusted")
    return int(match.group(1))


def read_json_header(path: str) -> dict | None:
    try:
        with open(path, "r", encoding="utf-8") as handle:
            return json.load(handle)
    except (OSError, ValueError):
        return None


def scored_document_keys(root: str) -> dict[str, list[str]]:
    """Derive which documents carry an eval gold, from the datasets themselves."""
    scored: dict[str, list[str]] = {}
    dataset_dir = os.path.join(root, EVAL_DATASET_DIR)
    if not os.path.isdir(dataset_dir):
        return scored
    for name in sorted(os.listdir(dataset_dir)):
        if not name.endswith(".json"):
            continue
        items = read_json_header(os.path.join(dataset_dir, name))
        if not isinstance(items, list):
            continue
        for item in items:
            key = item.get("doc_key") if isinstance(item, dict) else None
            if not key:
                continue
            datasets = scored.setdefault(key, [])
            if name not in datasets:
                datasets.append(name)
    return scored


def frontier_ownership(root: str) -> dict:
    """Read the refresh frontier's own contract: its cohort rule and its declared lifecycle."""
    contract = read_json_header(os.path.join(root, FRONTIER_CONTRACT))
    if not isinstance(contract, dict):
        return {"available": False}
    rule = contract.get("cohort_rule") or {}
    return {
        "available": True,
        "excluded_source_prefixes": list(rule.get("excluded_source_prefixes") or []),
        "refreshed": set(contract.get("refreshed") or []),
        "remaining": set(contract.get("remaining") or []),
    }


def census(root: str) -> dict:
    canonical = {
        stage: read_schema_constant(root, rust_path, const_name)
        for stage, _, _, rust_path, const_name in STAGES
    }
    frontier = frontier_ownership(root)
    scored = scored_document_keys(root)

    source_root = os.path.join(root, "generated/source_ir")
    if not os.path.isdir(source_root):
        raise SystemExit(
            "skip: generated/source_ir is absent, so there is no persisted corpus to census "
            "(a fresh clone and a hosted CI runner have none)")

    documents: dict[str, dict] = {}
    for key in sorted(os.listdir(source_root)):
        header = read_json_header(os.path.join(source_root, key, "source_ir.json"))
        if header is None:
            continue
        requested_path = (header.get("source") or {}).get("requested_path", "")
        record = {
            "requested_path": requested_path,
            "schema": {"source": header.get("schema_version")},
        }
        for stage, stage_root, artifact, _, _ in STAGES:
            if stage == "source":
                continue
            stage_header = read_json_header(
                os.path.join(root, stage_root, key, artifact))
            record["schema"][stage] = (
                stage_header.get("schema_version") if stage_header else None)
        documents[key] = record

    excluded = tuple(frontier.get("excluded_source_prefixes") or ())
    for key, record in documents.items():
        gating = record["schema"].get(GATING_STAGE)
        record["measurable"] = gating is not None and gating >= canonical[GATING_STAGE]
        in_cohort = bool(excluded) and not record["requested_path"].startswith(excluded)
        if not frontier.get("available"):
            record["owner"] = "unknown (frontier contract unreadable)"
        elif not in_cohort:
            record["owner"] = "outside the refresh frontier (excluded source prefix)"
        elif key in frontier["remaining"]:
            record["owner"] = "refresh frontier: declared remaining"
        elif key in frontier["refreshed"]:
            record["owner"] = "refresh frontier: declared refreshed"
        else:
            record["owner"] = "refresh frontier: cohort member, undeclared"
        record["scored_by"] = scored.get(key, [])

    legacy = {k: v for k, v in documents.items() if not v["measurable"]}
    owner_counts: dict[str, int] = {}
    for record in legacy.values():
        owner_counts[record["owner"]] = owner_counts.get(record["owner"], 0) + 1

    stage_histogram = {
        stage: {}
        for stage, _, _, _, _ in STAGES
    }
    for record in documents.values():
        for stage in stage_histogram:
            version = record["schema"].get(stage)
            label = "absent" if version is None else str(version)
            stage_histogram[stage][label] = stage_histogram[stage].get(label, 0) + 1

    scored_documents = {k: documents[k] for k in sorted(scored) if k in documents}
    scored_unmeasurable = {
        k: v for k, v in scored_documents.items() if not v["measurable"]}

    return {
        "canonical_schema": canonical,
        "gating_stage": GATING_STAGE,
        "documents_persisted": len(documents),
        "measurable": sum(1 for r in documents.values() if r["measurable"]),
        "legacy": len(legacy),
        "stage_schema_histogram": stage_histogram,
        "legacy_by_owner": dict(sorted(owner_counts.items())),
        "legacy_keys_by_owner": {
            owner: sorted(k for k, v in legacy.items() if v["owner"] == owner)
            for owner in sorted(owner_counts)
        },
        "scored_documents": len(scored_documents),
        "scored_measurable": len(scored_documents) - len(scored_unmeasurable),
        "scored_unmeasurable": {
            k: {"datasets": v["scored_by"], "owner": v["owner"],
                "requested_path": v["requested_path"]}
            for k, v in scored_unmeasurable.items()
        },
        "frontier_declared_refreshed": len(frontier.get("refreshed") or ()),
        "frontier_refreshed_still_legacy": sorted(
            k for k in (frontier.get("refreshed") or ())
            if k in legacy),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--json", action="store_true", help="emit the raw census as JSON")
    args = parser.parse_args()

    result = census(repo_root())
    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
        return 0

    canonical = result["canonical_schema"]
    print("current canonical schema (read from the Rust constants):")
    for stage, _, _, rust_path, const_name in STAGES:
        print(f"  {stage:9s} {canonical[stage]}   {const_name} ({rust_path})")
    print(f"  gating stage for `eval-extraction`: {result['gating_stage']}")
    print()

    total = result["documents_persisted"]
    measurable = result["measurable"]
    share = (measurable / total * 100.0) if total else 0.0
    print(f"persisted documents:                 {total}")
    print(f"  MEASURABLE (canonically current):  {measurable}  ({share:.1f}%)")
    print(f"  LEGACY (refused, inspection-only): {result['legacy']}")
    print()

    print("persisted schema version by stage:")
    for stage, _, _, _, _ in STAGES:
        counts = ", ".join(f"{v}x schema {k}" for k, v in
                           sorted(result["stage_schema_histogram"][stage].items()))
        print(f"  {stage:9s} {counts}")
    print()

    print("who owns each LEGACY document today:")
    for owner, count in result["legacy_by_owner"].items():
        print(f"  {count:4d}  {owner}")
    print()

    refreshed = result["frontier_declared_refreshed"]
    still_legacy = result["frontier_refreshed_still_legacy"]
    print(f"refresh frontier declares {refreshed} document(s) refreshed; "
          f"{len(still_legacy)} of them are still LEGACY.")
    print("  -> `refreshed` records completion of the host-library re-ingest PROGRAM, not")
    print("     canonical currency: the sweep finished under an older SourceIR schema.")
    print()

    print(f"documents carrying an eval gold:     {result['scored_documents']}")
    print(f"  measurable:                        {result['scored_measurable']}")
    print(f"  UNMEASURABLE (no score possible):  {len(result['scored_unmeasurable'])}")
    for key, detail in result["scored_unmeasurable"].items():
        print(f"     {key}")
        print(f"       datasets: {', '.join(detail['datasets'])}")
        print(f"       owner:    {detail['owner']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
