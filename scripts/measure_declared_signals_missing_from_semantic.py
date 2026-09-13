#!/usr/bin/env python3
"""Census signals a document DECLARES in EvidenceIR that never reach its SemanticIR catalog.

`SIGNAL-DECLARATION-ROW-DROP.4`, opened by `EXTRACTION-QUALITY-GAUGE.3k.7`.

`parse_explicit_signal_declaration` reads `Signal X is <direction> width <W>.` and ends with
`if index != tokens.len() { return None; }`. A declaration whose width it cannot consume to the last
token therefore yields NOTHING — the direction it already parsed is discarded with it, and there is
no residual, no counter and no validation entry. `build_interfaces` is the only producer of the
interface signal records, `declared_signal_names` is built from them, and the SemanticIR grounding
partition keeps a `signal_constraint` only when that set contains its subject. So an unreadable width
silently deletes the signal's identity AND every obligation the document states about it.

The census reads the two persisted artifacts and reports the difference:

  DECLARED  — `Signal X is …` statements in `generated/evidence_ir/<key>/evidence_ir.json`.
  CATALOG   — `interfaces[].signal_records[].signal_name` in the sibling SemanticIR.
  MISSING   — declared and not in the catalog, with the width text the declaration stated.

The strata matter and the report splits them, because they need different answers. A width that is an
ARITHMETIC expression (`DATA_WIDTH / 8`, `ceil(DATA_WIDTH / 64)`) is a legitimate declaration the
parser stops short of; a width the source row never stated as one (`3'b000 , lavalid`) is an upstream
misread whose refusal is probably correct. Silence is the defect in both cases.

This census classifies a POPULATION by reading persisted artifacts. It mirrors no Rust rule — the
membership test is set difference between two materialized fields — so it is NOT a check on the
implementation and cannot be used as one.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: the persisted `generated/{evidence_ir,semantic_ir}/*/` corpus.

Usage:
    python3 scripts/measure_declared_signals_missing_from_semantic.py
    python3 scripts/measure_declared_signals_missing_from_semantic.py --json
    python3 scripts/measure_declared_signals_missing_from_semantic.py --self-test
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import sys

DECLARATION = re.compile(r"^Signal ([A-Za-z0-9_]+) is (.*?)\.?$")
# An arithmetic or function-call width the parser can read the head of and not the rest.
ARITHMETIC = re.compile(r"[/+*]|\bceil\b|\bfloor\b|\blog\b")
SELF_TEST_CASES = 7


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def declared_widths(statements: list[str]) -> dict[str, list[str]]:
    """Every `Signal X is …` statement, keyed by name, keeping each distinct predicate."""
    declared: dict[str, list[str]] = {}
    for text in statements:
        match = DECLARATION.match(text.strip())
        if not match:
            continue
        name, predicate = match.group(1), match.group(2).strip()
        bucket = declared.setdefault(name, [])
        if predicate not in bucket:
            bucket.append(predicate)
    return declared


def catalog_names(semantic: dict) -> set[str]:
    return {
        record["signal_name"]
        for interface in semantic.get("interfaces", [])
        for record in interface.get("signal_records", [])
    }


def is_arithmetic_width(predicates: list[str]) -> bool:
    return any(ARITHMETIC.search(predicate) for predicate in predicates)


def census(root: str) -> dict:
    documents = []
    scanned = 0
    for evidence_path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        key = os.path.basename(os.path.dirname(evidence_path))
        semantic_path = os.path.join(root, "generated/semantic_ir", key, "semantic_ir.json")
        if not os.path.isfile(semantic_path):
            continue
        with open(evidence_path, encoding="utf-8") as handle:
            evidence = json.load(handle)
        with open(semantic_path, encoding="utf-8") as handle:
            semantic = json.load(handle)
        scanned += 1
        declared = declared_widths([s["text"] for s in evidence.get("extracted_statements", [])])
        catalog = catalog_names(semantic)
        missing = [
            {
                "signal": name,
                "declared_as": predicates,
                "stratum": "arithmetic_width" if is_arithmetic_width(predicates) else "unstated_width",
            }
            for name, predicates in sorted(declared.items())
            if name not in catalog
        ]
        if missing:
            documents.append(
                {
                    "document": key,
                    "declared": len(declared),
                    "catalog": len(catalog),
                    "missing": missing,
                }
            )
    documents.sort(key=lambda row: (-len(row["missing"]), row["document"]))
    rows = [row for document in documents for row in document["missing"]]
    return {
        "documents_scanned": scanned,
        "documents_with_missing": len(documents),
        "missing_total": len(rows),
        "arithmetic_width": sum(1 for row in rows if row["stratum"] == "arithmetic_width"),
        "unstated_width": sum(1 for row in rows if row["stratum"] == "unstated_width"),
        "documents": documents,
    }


def report(result: dict) -> None:
    print("command: measure_declared_signals_missing_from_semantic")
    print(f"documents_scanned: {result['documents_scanned']}")
    print(
        f"missing_total: {result['missing_total']} declared signal(s) across "
        f"{result['documents_with_missing']} document(s) never reach the SemanticIR catalog"
    )
    print(f"  arithmetic_width: {result['arithmetic_width']} (a legitimate width the parser stops short of)")
    print(f"  unstated_width:   {result['unstated_width']} (the source row never stated a width)")
    for document in result["documents"]:
        print(
            f"document: {document['document']} declared={document['declared']} "
            f"catalog={document['catalog']} missing={len(document['missing'])}"
        )
        for row in document["missing"]:
            print(f"  {row['stratum']}: {row['signal']} <- {row['declared_as'][0]!r}")


def self_test() -> int:
    ran = 0
    failures = []

    def check(label: str, actual, expected) -> None:
        nonlocal ran
        ran += 1
        if actual != expected:
            failures.append(f"{label}: {actual!r} != {expected!r}")

    check(
        "declaration parsed",
        declared_widths(["Signal WSTRB is output width DATA_WIDTH / 8."]),
        {"WSTRB": ["output width DATA_WIDTH / 8"]},
    )
    check(
        "distinct predicates are kept, duplicates are not",
        declared_widths(
            [
                "Signal WSTRB is width DATA_WIDTH / 8.",
                "Signal WSTRB is width DATA_WIDTH / 8.",
                "Signal WSTRB is output width DATA_WIDTH / 8.",
            ]
        )["WSTRB"],
        ["width DATA_WIDTH / 8", "output width DATA_WIDTH / 8"],
    )
    check("a non-declaration is ignored", declared_widths(["WSTRB must be HIGH."]), {})
    check("division is arithmetic", is_arithmetic_width(["width DATA_WIDTH / 8"]), True)
    check("a function call is arithmetic", is_arithmetic_width(["width ceil(DATA_WIDTH / 64)"]), True)
    check("a plain parameter is not", is_arithmetic_width(["output width DATA_WIDTH"]), False)
    check(
        "catalog names are read from the interface records",
        catalog_names({"interfaces": [{"signal_records": [{"signal_name": "WSTRBCHK"}]}]}),
        {"WSTRBCHK"},
    )

    if ran != SELF_TEST_CASES:
        failures.append(f"ran {ran} cases, expected {SELF_TEST_CASES}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if failures:
        return 1
    print(f"self-test: {ran}/{SELF_TEST_CASES} cases pass")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    result = census(repo_root())
    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        report(result)
    return 0


if __name__ == "__main__":
    sys.exit(main())
