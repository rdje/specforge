#!/usr/bin/env python3
"""Census the `[hi:lo]` bit-range cells a `[hi:lo]`-as-width rule would read.

`SIGNAL-DECLARATION-ROW-DROP.2f`, whose own text requires this before any code: *"Measure the corpus
population of bit-range width cells before implementing — a `Bits` column is common, and the blast
radius is not this one table."*

`parse_table_width_hint_text` reads a number and a parametric string, so `[125:110]` is neither and a
`Bits` column states a width the reader cannot use. `[hi:lo]` is `hi - lo + 1` — universal notation,
no vocabulary — and the leaf was sized at "15 recoveries for 2 phantoms" from one table.

The census reports two things the leaf did not have:

  SCOPE     which tables carry the notation at all, by their own header and caption. The population
            splits into signal tables (a name column headed as a SIGNAL name, the bit range being a
            slice of a wider observation bus) and REGISTER BIT-FIELD tables that SourceIR typed
            `signal_description` (`Bits | Name | Description` under a caption naming a register).
  TRADE     within each scope, how many rows already produce a declaration and how many do not. A row
            that already declares would gain a width; a row that does not would be newly ADMITTED,
            and that is where a phantom comes from.

It also evaluates one candidate refusal for the `Unused` spacer row that blocks the leaf: a name cell
that REPEATS inside its own table. That rule is reported with its full selection so its false
positives are visible rather than assumed.

This census classifies a POPULATION by reading persisted artifacts. The "already declares" join is
against each EvidenceIR's own `table_signal_declaration_provenance`, which is PUBLISHED state: 54 of
the 78 documents cannot be re-run at the evidence stage at all, so this is what SpecForge emitted and
not necessarily what today's extractor would emit (`[[declaration-replay-reads-the-legacy-stratum]]`).
It mirrors no Rust rule and is NOT a check on the implementation.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: the persisted `generated/{source_ir,evidence_ir}/*/` corpus.

Usage:
    python3 scripts/measure_bit_range_width_cells.py
    python3 scripts/measure_bit_range_width_cells.py --json
    python3 scripts/measure_bit_range_width_cells.py --self-test
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

BIT_RANGE = re.compile(r"^\[\s*(\d+)\s*:\s*(\d+)\s*\]$")
# Header spellings that name a column of SIGNAL identities. Document grammar, not a chip, vendor or
# protocol vocabulary (ADR 0006) — the same kind of header keyword the name-column override reads.
SIGNAL_NAME_HEADERS = ("signal name", "signal", "signal role")
# Header spellings that name a column of identities generally, including register fields.
GENERIC_NAME_HEADERS = ("name", "field")
SELF_TEST_CASES = 9


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def bit_range_width(text: str) -> int | None:
    """`[hi:lo]` -> `hi - lo + 1`, or `None` when the cell is not a bit range."""
    match = BIT_RANGE.match(text.strip())
    if not match:
        return None
    high, low = int(match.group(1)), int(match.group(2))
    return high - low + 1 if high >= low else None


def name_column(headers: list[str]) -> tuple[int | None, str]:
    """The name column's index and scope: `signal` when its header names signals, else `generic`."""
    lowered = [header.strip().lower() for header in headers]
    for index, header in enumerate(lowered):
        if header in SIGNAL_NAME_HEADERS:
            return index, "signal"
    for index, header in enumerate(lowered):
        if header in GENERIC_NAME_HEADERS:
            return index, "generic"
    return None, "none"


def table_rows(table: dict) -> list[list[str]]:
    return [
        [str(cell.get("text", "")).strip() for cell in row]
        for row in table.get("body_rows", [])
    ]


def census(root: str) -> dict:
    scopes: dict[str, dict] = {
        scope: {"tables": 0, "rows": 0, "declared": 0, "undeclared": 0, "undeclared_names": {}}
        for scope in ("signal", "generic")
    }
    tables_detail = []
    repeated: dict[str, int] = {}
    unique_rows = 0
    repeated_rows = 0

    for source_path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        key = os.path.basename(os.path.dirname(source_path))
        evidence_path = os.path.join(root, "generated/evidence_ir", key, "evidence_ir.json")
        if not os.path.isfile(evidence_path):
            continue
        with open(source_path, encoding="utf-8") as handle:
            source = json.load(handle)
        with open(evidence_path, encoding="utf-8") as handle:
            evidence = json.load(handle)
        declared_by_table: dict[str, set[str]] = collections.defaultdict(set)
        for record in evidence.get("table_signal_declaration_provenance", []):
            declared_by_table[record["table_id"]].add(record["signal_name"].upper())

        for table in source.get("structured_tables", []):
            if table.get("table_kind") != "signal_description":
                continue
            headers = [
                str(cell.get("text", "")).strip()
                for cell in (table.get("header_rows") or [[]])[0]
            ]
            index, scope = name_column(headers)
            if index is None:
                continue
            rows = table_rows(table)
            names = [row[index] if index < len(row) else "" for row in rows]
            counts = collections.Counter(name for name in names if name)
            for name, count in counts.items():
                if count > 1:
                    repeated[name] = repeated.get(name, 0) + count
                    repeated_rows += count
                else:
                    unique_rows += 1

            ranged = [
                (row, name)
                for row, name in zip(rows, names)
                if any(bit_range_width(cell) is not None for cell in row)
            ]
            if not ranged:
                continue
            bucket = scopes[scope]
            bucket["tables"] += 1
            declared_here = declared_by_table[table["table_id"]]
            declared = sum(1 for _, name in ranged if name.upper() in declared_here)
            undeclared_names: dict[str, int] = {}
            for _, name in ranged:
                if name.upper() not in declared_here:
                    undeclared_names[name] = undeclared_names.get(name, 0) + 1
            bucket["rows"] += len(ranged)
            bucket["declared"] += declared
            bucket["undeclared"] += len(ranged) - declared
            for name, count in undeclared_names.items():
                bucket["undeclared_names"][name] = bucket["undeclared_names"].get(name, 0) + count
            tables_detail.append(
                {
                    "document": key,
                    "table_id": table["table_id"],
                    "scope": scope,
                    "caption": table.get("caption_text"),
                    "headers": headers,
                    "rows": len(ranged),
                    "declared": declared,
                    "undeclared": len(ranged) - declared,
                }
            )

    tables_detail.sort(key=lambda row: (-row["rows"], row["document"], row["table_id"]))
    return {
        "scopes": scopes,
        "tables": tables_detail,
        "repeated_name_rule": {
            "unique_rows": unique_rows,
            "repeated_rows": repeated_rows,
            "repeated_names": dict(sorted(repeated.items(), key=lambda kv: (-kv[1], kv[0]))),
        },
    }


def report(result: dict) -> None:
    print("command: measure_bit_range_width_cells")
    for scope in ("signal", "generic"):
        bucket = result["scopes"][scope]
        label = (
            "signal tables (name column headed as a SIGNAL name)"
            if scope == "signal"
            else "generic tables (name column headed Name/Field — register bit-field shape)"
        )
        print(f"{scope}: {label}")
        print(
            f"  tables={bucket['tables']} rows={bucket['rows']} "
            f"already_declared={bucket['declared']} not_declared={bucket['undeclared']}"
        )
        for name, count in sorted(
            bucket["undeclared_names"].items(), key=lambda kv: (-kv[1], kv[0])
        )[:12]:
            print(f"    not_declared: {count:3}  {name!r}")
    print("tables:")
    for row in result["tables"]:
        print(
            f"  {row['rows']:4} rows ({row['declared']} declared) {row['scope']:7} "
            f"{row['document'][:24]}/{row['table_id']}"
        )
        if row["caption"]:
            print(f"        caption: {row['caption'][:88]}")
    rule = result["repeated_name_rule"]
    print(
        f"repeated_name_rule: {rule['repeated_rows']} row(s) whose name cell repeats inside its own "
        f"table, against {rule['unique_rows']} unique — "
        f"{len(rule['repeated_names'])} distinct text(s)"
    )
    for name, count in list(rule["repeated_names"].items())[:24]:
        print(f"    {count:4}  {name!r}")


def self_test() -> int:
    ran = 0
    failures = []

    def check(label: str, actual, expected) -> None:
        nonlocal ran
        ran += 1
        if actual != expected:
            failures.append(f"{label}: {actual!r} != {expected!r}")

    check("a bit range is inclusive on both ends", bit_range_width("[125:110]"), 16)
    check("a single-bit range is one bit", bit_range_width("[7:7]"), 1)
    check("whitespace inside the brackets is tolerated", bit_range_width("[ 3 : 0 ]"), 4)
    check("a plain number is not a bit range", bit_range_width("16"), None)
    check("a reversed range states no width", bit_range_width("[0:7]"), None)
    check(
        "a signal-name header wins over a later generic one",
        name_column(["Bits", "Signal name", "Name"]),
        (1, "signal"),
    )
    check(
        "a register bit-field header is generic scope",
        name_column(["Bits", "Name", "Description"]),
        (1, "generic"),
    )
    check("a table with no name column is skipped", name_column(["Bits", "Value"]), (None, "none"))
    check(
        "body rows are read as trimmed text",
        table_rows({"body_rows": [[{"text": " a "}, {"text": "[3:0]"}]]}),
        [["a", "[3:0]"]],
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
