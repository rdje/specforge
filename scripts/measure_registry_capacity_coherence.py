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
import re
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


# LIVE-DOCUMENT-PRESSURE-HEADROOM.36b.1 — how much room a further RE-DERIVATION could still buy.
# `.36b` published "the derivation buys nine and then spends the class portable envelope", and an
# audit refused it: the envelope is not spent. Coherence needs `max_records x max_record_bytes <=
# max_bytes <= <portable cap>`, and the per-record ceiling only has to ADMIT the largest record that
# exists — so trading ceiling headroom for slots reaches a higher record count than the one shipped.
# The number is derived here rather than asserted anywhere, and the portable cap is read out of the
# CHECKER that compiles it rather than from a description of it (`CLAIM_VERIFICATION.md` §3).
CLAIM_REGISTRY = "doctrine/claim_verification/claims.jsonl"
CLAIM_CHECKER = "scripts/check_claim_verification.pl"


def portable_max_bytes(root: str) -> int | None:
    """The `max_bytes` portable hard cap `check_claim_verification.pl` compiles, read from its source."""
    path = os.path.join(root, CLAIM_CHECKER)
    if not os.path.exists(path):
        return None
    with open(path, "r", encoding="utf-8") as handle:
        source = handle.read()
    match = re.search(r"%hard\s*=\s*\((?:[^)]*?)max_bytes\s*=>\s*([0-9_]+)", source, re.S)
    return int(match.group(1).replace("_", "")) if match else None


def reachable_records(root: str, entry: dict) -> dict | None:
    """The largest coherent `max_records` still reachable inside the portable envelope."""
    cap = portable_max_bytes(root)
    path = os.path.join(root, CLAIM_REGISTRY)
    if cap is None or not os.path.exists(path) or not entry.get("max_records"):
        return None
    with open(path, "rb") as handle:
        lines = [line for line in handle.read().split(b"\n") if line.strip()]
    largest = max((len(line) + 1 for line in lines[1:]), default=0)
    if largest <= 0:
        return None
    best_ceiling = largest
    best_records = cap // largest
    return {
        "portable_max_bytes": cap,
        "largest_record_bytes": largest,
        "declared_records": entry["max_records"],
        "reachable_records": best_records,
        "reachable_at_max_record_bytes": best_ceiling,
        "residual_records": best_records - entry["max_records"],
    }


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
    claims = next((r for r in found if r["path"] == CLAIM_REGISTRY), None)
    reach = reachable_records(repo_root(), claims) if claims else None
    if reach:
        print()
        print(f"  claim registry residual headroom, DERIVED (not asserted): declares"
              f" {reach['declared_records']} records; the portable envelope of"
              f" {reach['portable_max_bytes']} still reaches {reach['reachable_records']}"
              f" at a per-record ceiling of {reach['reachable_at_max_record_bytes']}"
              f" — {reach['residual_records']} more.")
        print("  That ceiling sits flush against today's largest record, so the extra slots are")
        print("  bought by giving up the headroom the ceiling exists to provide. The envelope is")
        print("  NEARLY spent, not spent; LIVE-DOCUMENT-PRESSURE-HEADROOM.36d owns the lifecycle.")


# The class as `.36a` measured it and `.36b` left it. These pin the SHAPE of the finding, not a
# moment's fill: a commit that makes a registry coherent, or pushes a new one past rollover, is a
# real event. `.36b` repaired exactly one of them, so `incoherent` went 9 -> 8 and `coherent` 1 -> 2.
PINNED = {"with_all_three_bounds": 10, "incoherent": 8, "coherent": 2}


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

    # RED 2 — coherence must be the product test, not a fill test. `canonical_catalogs` is coherent
    # precisely because 8 x 1024 == 8192, and it is not the emptiest; `claims.jsonl` joined it when
    # `.36b` derived its triple rather than raising one number.
    coherent = sorted(r["path"] for r in found if r["coherent"])
    check("coherence-is-the-product-test",
          coherent == ["doctrine/claim_verification/claims.jsonl",
                       "doctrine/live_document_size/canonical_catalogs.jsonl"])

    # RED 3 — the harm is where the REAL record size approaches the permitted one, and `claims.jsonl`
    # writes the largest records in the class, which is why it stopped first.
    claims = by_path.get("doctrine/claim_verification/claims.jsonl")
    others = [r["mean_record_bytes"] for path, r in by_path.items()
              if r["mean_record_bytes"] and path != "doctrine/claim_verification/claims.jsonl"]
    check("claims-writes-the-largest-records",
          claims is not None and claims["mean_record_bytes"] > max(others))

    # RED 4 — `.36a` found `claims.jsonl` to be the one registry whose byte bound funded fewer
    # records than `max_records` declared: 12 against 64. `.36b` closed that by derivation, so the
    # pin inverts. It is stated as "this registry is not a binding case" rather than "no registry
    # is", because the list is transient for a registry holding one or two records — the authority
    # registry enters it while a single-use authority is banked and leaves when it is retired, and
    # pinning the whole list would make an unrelated slice fail here.
    check("claims-is-no-longer-a-binding-case",
          "doctrine/claim_verification/claims.jsonl"
          not in summary["byte_bound_funds_fewer_than_declared"])
    check("claims-byte-bound-now-funds-more-than-declared",
          claims is not None
          and claims["records_funded_at_real_size"] >= claims["max_records"])

    # RED 5 — the derivation itself, which is what `.36b` shipped: the declared record capacity is
    # exactly fundable, so both bounds stop first at the same place (`.2a`'s relocation test), and
    # the per-record ceiling covers the largest record the registry actually holds. Mutating any one
    # of the three numbers breaks this, which is the property a raise alone would not have bought.
    check("claims-capacity-is-exactly-fundable",
          claims is not None
          and claims["max_records"] * claims["max_record_bytes"] == claims["max_bytes"])
    largest_claim_record = max(
        (len(line.encode("utf-8")) + 1
         for line in open(os.path.join(root, "doctrine/claim_verification/claims.jsonl"),
                          encoding="utf-8").read().splitlines()[1:] if line.strip()),
        default=0,
    )
    check("claims-per-record-ceiling-covers-the-largest-record",
          claims is not None and 0 < largest_claim_record <= claims["max_record_bytes"])

    # RED 6 — LIVE-DOCUMENT-PRESSURE-HEADROOM.36b.1. `.36b` published that the derivation "spends the
    # class portable envelope", and an audit refused it by arithmetic. The residual is DERIVED here so
    # the claim cannot be wrong again by assertion, and the portable cap is read out of the checker
    # that compiles it. A cap read from a description instead of from the producer is exactly the
    # blindness `CLAIM_VERIFICATION.md` §3 names; mutating the checker's number moves this case.
    reach = reachable_records(root, claims) if claims else None
    check("portable-cap-is-read-from-the-checker-not-a-literal",
          reach is not None and reach["portable_max_bytes"] == 262144)
    check("residual-headroom-is-positive-so-the-envelope-is-not-spent",
          reach is not None and reach["residual_records"] > 0)
    check("residual-headroom-is-small-enough-that-a-lifecycle-is-still-owed",
          reach is not None and reach["residual_records"] <= 4)

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
