#!/usr/bin/env python3
"""Which signal tables put their direction in a DIFFERENT column on different rows? (read-only)

`SIGNAL-DECLARATION-ROW-DROP.2j.1`. Two leaves of this tree are blocked on the same missing number.
`.2j` closed its rule half by refusing the direction-abbreviation arm and handed on a second
deliverable — *size ADIv6 `table_0108`'s column garble and route it*. `.2h.2` is blocked outright on
*"a corpus census of per-row layout drift"*, because CoreSight TMC `table_0074` is **mixed, not
rotated** — six rows name-last, the seventh name-first — so `.2e`'s whole-table offset cannot serve
it, and one table is not a grammar.

This producer is that census, and it is deliberately built out of the ONE vocabulary in a signal
table that is closed and unambiguous: the direction values themselves.

## Why not a name heuristic, measured rather than assumed

The obvious census asks which column holds the signal NAME and whether that column is stable. Two
cheap name tests were tried against this corpus first and both fail, in OPPOSITE directions:

  * "the cell is a single identifier-shaped token" misses `PWDATA_S [31:0]`, `sample_req ,` and
    `PRDATA_S Output` — the name is there, with a bit range, a comma, or a fused direction beside it;
  * "the cell's FIRST token is identifier-shaped" then accepts `Test Clock`, `Port Connected` and
    `Return Clock` — ordinary description prose whose first word is a plain word.

So a name test either under- or over-selects, and the error is not small: the first cut of this
census reported 30 tables and 116 rows, the second 11 rows, and neither number survived reading its
own selection. **Direction values have no such problem.** `Input`/`Output`/`In`/`Out`/`Inout`/… is a
closed set, a whole-cell match needs no judgement, and a table whose direction cell moves between
rows is exactly the table whose columns drifted — whatever the name column is doing.

A table is reported when its body rows disagree about which column holds the direction. A table where
every row agrees is CONSISTENT, whether that column is the first, the last, or the one the header
names: a uniform offset is what `.2e` already serves.

Read-only and deterministic: no network, no clock, no randomness, no write.

Usage:
    python3 scripts/measure_direction_column_drift.py              # the census
    python3 scripts/measure_direction_column_drift.py --rows       # every drifted table, row by row
    python3 scripts/measure_direction_column_drift.py --json
    python3 scripts/measure_direction_column_drift.py --self-test  # the RED cases
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from build_bounded_decision_baseline import repo_root  # noqa: E402

DIRECTION_VALUES = frozenset({
    "in", "out", "io", "i", "o", "input", "output", "inout",
    "in/out", "input/output", "bidirectional", "bidir",
})


def normalise(text: str | None) -> str:
    return " ".join((text or "").split())


def direction_column(row: list[dict]) -> int | None:
    """The single column of this row whose WHOLE cell is a direction value, if there is exactly one."""
    hits = [i for i, cell in enumerate(row) if normalise(cell.get("text")).lower() in DIRECTION_VALUES]
    return hits[0] if len(hits) == 1 else None


def census(root: str) -> dict:
    tables: list[dict] = []
    shapes: Counter = Counter()
    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        declared_by_table: dict[str, list[str]] = {}
        evidence = os.path.join(root, f"generated/evidence_ir/{document}/evidence_ir.json")
        if os.path.exists(evidence):
            with open(evidence, "r", encoding="utf-8") as handle:
                for record in json.load(handle).get("table_signal_declaration_provenance") or []:
                    declared_by_table.setdefault(record.get("table_id"), []).append(record.get("signal_name"))
        for table in source.get("structured_tables") or []:
            if table.get("table_kind") != "signal_description":
                continue
            body = table.get("body_rows") or []
            if not body:
                continue
            columns = [direction_column(row) for row in body]
            present = [c for c in columns if c is not None]
            if not present:
                shapes["no_direction_value"] += 1
                continue
            if len(set(present)) == 1:
                shapes["consistent"] += 1
                continue
            shapes["drifted"] += 1
            declared = sorted(declared_by_table.get(table["table_id"], []))
            tables.append({
                "document": document,
                "table_id": table["table_id"],
                "caption": normalise(table.get("caption_text")),
                "header": [normalise(c.get("text")) for c in (table.get("header_rows") or [[]])[0]],
                "body_rows": len(body),
                "declarations": len(declared),
                "declared_names": declared,
                "direction_columns": {str(k): v for k, v in sorted(Counter(columns).items(), key=lambda kv: (kv[0] is None, kv[0]))},
                "rows": [[normalise(c.get("text")) for c in row] for row in body],
            })
    tables.sort(key=lambda t: (-(t["body_rows"] - t["declarations"]), t["document"], t["table_id"]))
    return {"shapes": dict(shapes), "tables": tables}


def summarise(result: dict) -> dict:
    tables = result["tables"]
    return {
        "signal_description_tables": sum(result["shapes"].values()),
        "consistent": result["shapes"].get("consistent", 0),
        "no_direction_value": result["shapes"].get("no_direction_value", 0),
        "drifted_tables": len(tables),
        "drifted_documents": len({t["document"] for t in tables}),
        "drifted_body_rows": sum(t["body_rows"] for t in tables),
        "drifted_declarations": sum(t["declarations"] for t in tables),
        "rows_with_no_declaration": sum(t["body_rows"] - t["declarations"] for t in tables),
    }


def render(result: dict, show_rows: bool) -> None:
    summary = summarise(result)
    print("=== SIGNAL-DECLARATION-ROW-DROP.2j.1 — per-row direction-column drift (read-only) ===")
    print(f"  signal_description tables {summary['signal_description_tables']}"
          f"   consistent {summary['consistent']}"
          f"   no direction value {summary['no_direction_value']}"
          f"   DRIFTED {summary['drifted_tables']}")
    print()
    print(f"== DRIFTED: {summary['drifted_tables']} tables across {summary['drifted_documents']} documents,"
          f" {summary['drifted_body_rows']} body rows,"
          f" {summary['drifted_declarations']} declarations,"
          f" {summary['rows_with_no_declaration']} rows with none")
    print()
    for table in result["tables"]:
        undeclared = table["body_rows"] - table["declarations"]
        print(f"  {table['document'][:40]:42s} {table['table_id']:11s}"
              f" rows={table['body_rows']:3d} declared={table['declarations']:3d}"
              f" undeclared={undeclared:3d} direction column={table['direction_columns']}")
    print()
    print("  `rows - declared` is an UPPER BOUND on the loss and not a loss count: a body row may be a")
    print("  note or a continuation, and a declaration may be a PHANTOM. CoreSight TMC table_0074 reads")
    print("  1 undeclared row and is in fact worse — it mints DATA, which is no signal of that table,")
    print("  while ATIDM[6:0] and AFREADYM go missing. Adjudicate a table before costing it.")
    if not show_rows:
        print("\n  --rows prints every drifted table row by row; the adjudication is in")
        print("  docs/research/direction-column-drift-census.md")
        return
    for table in result["tables"]:
        print(f"\n-- {table['document']} {table['table_id']} — {table['caption'] or '(no caption)'}")
        print(f"   header: {table['header']}")
        print(f"   declared: {table['declared_names']}")
        for index, row in enumerate(table["rows"]):
            print(f"   row {index:2d}: {[c[:40] for c in row]}")


PINNED = {
    "drifted_tables": 9,
    "drifted_documents": 5,
    "drifted_body_rows": 91,
    "drifted_declarations": 40,
    "rows_with_no_declaration": 51,
}


def self_test(root: str) -> int:
    failures: list[str] = []
    passed = 0

    def check(case: str, condition: bool) -> None:
        nonlocal passed
        if condition:
            passed += 1
        else:
            failures.append(case)

    # RED 1 — the direction test is WHOLE-CELL. A description mentioning a direction word must not be
    # read as a direction cell, or every prose column becomes one.
    check("whole-cell-only", direction_column([
        {"text": "ATVALIDM"}, {"text": "Output"},
        {"text": "Valid signals in this cycle from the output of the master"}]) == 1)
    # RED 2 — a row with two direction cells is AMBIGUOUS and contributes no opinion, rather than the
    # first one it finds. CoreSight ihi0029 table_0037 has such rows, and guessing would invent drift.
    check("two-direction-cells-is-no-opinion", direction_column([
        {"text": "PWDATADBG[31:0]"}, {"text": "Output"}, {"text": "Input"}, {"text": "0"}]) is None)
    # RED 3 — a UNIFORM offset is not drift. It is exactly what `.2e`'s whole-table offset serves, and
    # reporting it would bury the nine real cases under the corpus.
    uniform = [[{"text": "Output"}, {"text": "desc"}, {"text": "ATVALIDM"}],
               [{"text": "Input"}, {"text": "desc"}, {"text": "ATREADYM"}]]
    check("uniform-offset-is-not-drift", len({direction_column(r) for r in uniform}) == 1)
    # RED 4 — drift is disagreement BETWEEN rows, whatever the header says. TMC table_0074 is headed
    # `Type`, not `Direction`, which is why a header-scoped census missed it entirely.
    drifted = [[{"text": "Output"}, {"text": "desc"}, {"text": "ATVALIDM"}],
               [{"text": "AFREADYM"}, {"text": "Output"}, {"text": "desc"}]]
    check("drift-is-row-disagreement", len({direction_column(r) for r in drifted}) == 2)

    # RED 5 — the live census still has the shape this leaf adjudicated.
    summary = summarise(census(root))
    for key, expected in PINNED.items():
        check(f"census-pin-{key}", summary[key] == expected)

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"direction column drift: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rows", action="store_true", help="print every drifted table row by row")
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    args = parser.parse_args()
    root = repo_root()
    if args.self_test:
        return self_test(root)
    result = census(root)
    if args.json:
        json.dump({"summary": summarise(result), **result}, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result, args.rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
