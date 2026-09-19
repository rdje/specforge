#!/usr/bin/env python3
"""Do a bounded registry's three capacity bounds describe the same resource? (read-only)

`LIVE-DOCUMENT-PRESSURE-HEADROOM.36a` — `claims.jsonl` reached 94.7% of its byte ceiling while using
12 of 64 records, and the leaf's first deliverable is to measure the CLASS before proposing anything.
The class turns out to have a shared defect, and it explains the stop better than "needs more room"
does.

Every banded JSONL registry declares three numbers in its header record:

  `max_records`       how many records it may hold
  `max_record_bytes`  how large one record may be
  `max_bytes`         how large the whole file may be

Those are not independent quantities. A registry that admits `max_records` records of
`max_record_bytes` each needs `max_records x max_record_bytes` bytes, so a `max_bytes` below that
product means **the declared record capacity cannot actually be filled** — the file stops first, at a
record count nothing declared. ADR 0029 already states the rule this checks
(*capacity is the part quantum times the part count*), and
`[[FACT-CARD-CAPACITY-HEADROOM]]` applied it: `max_cards = cards_per_part x max_parts`.

## What the incoherence costs, and why it is usually invisible

Incoherence alone is harmless while a registry's REAL mean record is far below its permitted maximum:
a registry allowed 16KB records but writing 370-byte ones reaches its record bound long before its
byte bound, so the byte bound never binds and nobody notices. It bites only when the real record size
approaches the permitted one — and then the file stops at a record count that appears nowhere in the
contract, which is what happened to `claims.jsonl`.

So this producer reports both: whether the bounds are coherent, and how close the registry's real
records are to the size that makes the incoherence bite.

Read-only and deterministic: no network, no clock, no randomness, no write.

Usage:
    python3 scripts/measure_registry_capacity_coherence.py
    python3 scripts/measure_registry_capacity_coherence.py --json
    python3 scripts/measure_registry_capacity_coherence.py --self-test
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from build_bounded_decision_baseline import repo_root  # noqa: E402


def registries(root: str) -> list[dict]:
    found: list[dict] = []
    for path in sorted(glob.glob(os.path.join(root, "doctrine/**/*.jsonl"), recursive=True)):
        with open(path, "r", encoding="utf-8") as handle:
            first = handle.readline()
        try:
            header = json.loads(first)
        except json.JSONDecodeError:
            continue
        if header.get("record_type") != "registry":
            continue
        with open(path, "r", encoding="utf-8") as handle:
            records = sum(1 for line in handle if line.strip()) - 1
        relative = os.path.relpath(path, root)
        size = os.path.getsize(path)
        max_bytes = header.get("max_bytes")
        max_records = header.get("max_records")
        max_record_bytes = header.get("max_record_bytes")
        entry = {
            "path": relative,
            "bytes": size,
            "records": records,
            "max_bytes": max_bytes,
            "max_records": max_records,
            "max_record_bytes": max_record_bytes,
            "milestones": header.get("milestones"),
            "mean_record_bytes": size // records if records > 0 else None,
            "bytes_pct": round(100 * size / max_bytes, 1) if max_bytes else None,
            "records_pct": round(100 * records / max_records, 1) if max_records else None,
        }
        if max_bytes and max_records and max_record_bytes:
            implied = max_records * max_record_bytes
            entry["implied_bytes_for_declared_records"] = implied
            entry["coherent"] = implied <= max_bytes
            # The record count the BYTE bound actually funds, at the permitted record size and at
            # the size this registry really writes. The gap between them is the warning nobody gets.
            entry["records_funded_at_permitted_size"] = max_bytes // max_record_bytes
            entry["records_funded_at_real_size"] = (
                max_bytes // entry["mean_record_bytes"] if entry["mean_record_bytes"] else None
            )
        else:
            entry["coherent"] = None
        found.append(entry)
    return found


def summarise(found: list[dict]) -> dict:
    scored = [r for r in found if r["coherent"] is not None]
    incoherent = [r for r in scored if not r["coherent"]]
    # A registry BITES when the byte bound funds fewer records than the record bound declares.
    biting = [
        r for r in incoherent
        if r["records_funded_at_real_size"] is not None
        and r["records_funded_at_real_size"] < r["max_records"]
    ]
    return {
        "registries": len(found),
        "with_all_three_bounds": len(scored),
        "incoherent": len(incoherent),
        "coherent": len(scored) - len(incoherent),
        "byte_bound_funds_fewer_than_declared": sorted(r["path"] for r in biting),
        "at_or_above_rollover": sorted(
            r["path"] for r in found
            if (r["bytes_pct"] or 0) >= 90 or (r["records_pct"] or 0) >= 90
        ),
    }


def render(found: list[dict]) -> None:
    summary = summarise(found)
    print("=== LIVE-DOCUMENT-PRESSURE-HEADROOM.36a — registry capacity coherence (read-only) ===")
    print(f"  banded registries {summary['registries']}"
          f"   declaring all three bounds {summary['with_all_three_bounds']}"
          f"   COHERENT {summary['coherent']}   incoherent {summary['incoherent']}")
    print()
    header = (f"  {'registry':46s} {'bytes%':>7s} {'recs%':>6s} {'real/rec':>8s}"
              f" {'permitted':>9s} {'funds@real':>10s} {'declared':>8s} {'coherent':>8s}")
    print(header)
    for entry in sorted(found, key=lambda r: -(r["bytes_pct"] or 0)):
        if entry["coherent"] is None:
            continue
        print(f"  {entry['path'][-46:]:46s} {entry['bytes_pct']:7.1f} {entry['records_pct']:6.1f}"
              f" {entry['mean_record_bytes'] or 0:8d} {entry['max_record_bytes']:9d}"
              f" {entry['records_funded_at_real_size'] or 0:10d} {entry['max_records']:8d}"
              f" {'yes' if entry['coherent'] else 'NO':>8s}")
    print()
    print("  `funds@real` is how many records the BYTE bound actually pays for at the size this")
    print("  registry really writes; `declared` is what `max_records` promises. Where the first is")
    print("  smaller, the file stops at a count that appears nowhere in its contract.")
    print()
    print("  byte bound funds fewer records than declared:")
    for path in summary["byte_bound_funds_fewer_than_declared"]:
        print(f"    {path}")
    print("  at or above a 90% rollover milestone:")
    for path in summary["at_or_above_rollover"] or ["    (none)"]:
        print(f"    {path}")


# The class as `.36a` measured it. These pin the SHAPE of the finding, not a moment's fill: a commit
# that makes a registry coherent, or pushes a new one past rollover, is a real event.
PINNED = {"with_all_three_bounds": 10, "incoherent": 9, "coherent": 1}


def self_test(root: str) -> int:
    failures: list[str] = []
    passed = 0

    def check(case: str, condition: bool) -> None:
        nonlocal passed
        if condition:
            passed += 1
        else:
            failures.append(case)

    found = registries(root)
    summary = summarise(found)
    by_path = {r["path"]: r for r in found}

    # RED 1 — the class finding: nearly every banded registry declares a record capacity its byte
    # bound cannot fund. A producer that silently scored them coherent would report no defect.
    for key, expected in PINNED.items():
        check(f"class-pin-{key}", summary[key] == expected)

    # RED 2 — coherence must be the product test, not a fill test. `canonical_catalogs` is the one
    # coherent registry precisely because 8 x 1024 == 8192, and it is not the emptiest.
    coherent = [r["path"] for r in found if r["coherent"]]
    check("coherence-is-the-product-test",
          coherent == ["doctrine/live_document_size/canonical_catalogs.jsonl"])

    # RED 3 — the harm is where the REAL record size approaches the permitted one, and `claims.jsonl`
    # writes the largest records in the class, which is why it stopped first.
    claims = by_path.get("doctrine/claim_verification/claims.jsonl")
    others = [r["mean_record_bytes"] for path, r in by_path.items()
              if r["mean_record_bytes"] and path != "doctrine/claim_verification/claims.jsonl"]
    check("claims-writes-the-largest-records",
          claims is not None and claims["mean_record_bytes"] > max(others))

    # RED 4 — and it is the ONLY registry whose byte bound funds fewer records than `max_records`
    # declares. That is the finding: the incoherence is class-wide in the DECLARATION and binding in
    # exactly one place, so the remedy is a coherence rule rather than more room for everyone.
    check("claims-is-the-only-binding-case",
          summary["byte_bound_funds_fewer_than_declared"]
          == ["doctrine/claim_verification/claims.jsonl"])
    check("claims-byte-bound-funds-far-fewer",
          claims is not None
          and claims["records_funded_at_real_size"] < claims["max_records"] // 4)

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"registry capacity coherence: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    args = parser.parse_args()
    root = repo_root()
    if args.self_test:
        return self_test(root)
    found = registries(root)
    if args.json:
        json.dump({"registries": found, "summary": summarise(found)},
                  sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(found)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
