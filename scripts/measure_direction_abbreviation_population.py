#!/usr/bin/env python3
"""Is a whole-cell direction ABBREVIATION a notation, or a coincidence? (read-only)

`SIGNAL-DECLARATION-ROW-DROP.2j` — `.2h.0` censused `i`/`o`/`io`/`in`/`out` as whole-cell direction
values and refused them: **0 true positives against 18 false**, because AMBA LTI and AXI-Stream write
`O` for *Optional* in presence matrices. That census covered the `literal_direction_column` FALLBACK
population — the arm that fires only when **no** header names a direction — and the refusal is
plainly right there, since a presence matrix does not head its column `Direction`.

`BOUNDED-DECISION-PROVIDER.1` noticed the refusal also reaches the population where the header DOES
say `Direction`, and lost a row to it (ADIv6 `nSRSTOUT`, whose cell reads `Out`). That hazard cannot
arise under such a header, so the question reopened: is there a population worth reading there?

**This producer answers it, and the answer closes the leaf by REFUSING the rule.** This tree's
standing bar is that a rule needs a population — the bar `.2b` applied to the leftward arrow and
`.2h.0` applied to these same abbreviations. Measured over every persisted `SourceIR`, the population
is **one usable row, in one table, in one document** — and that table is the column-garbled
`table_0108`, so four of its five abbreviation cells sit in the NAME column rather than the direction
one, which is evidence of the garble and not of a notation.

The comparison is what makes it decisive: in the same 155 tables the **full** words that production
already reads appear in **837** rows. The notation is used heavily; the abbreviation is not used.

Read-only and deterministic: no network, no clock, no randomness, no write.

Usage:
    python3 scripts/measure_direction_abbreviation_population.py
    python3 scripts/measure_direction_abbreviation_population.py --json
    python3 scripts/measure_direction_abbreviation_population.py --self-test
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from measure_parametric_width_cell_shapes import header_texts, repo_root  # noqa: E402

# The abbreviations `.2h.0` refused, and the full words production already reads. Kept apart so the
# census reports the comparison rather than a bare count: a notation nobody writes is not a notation.
ABBREVIATIONS = {"in", "out", "i", "o", "io"}
FULL_WORDS = {"input", "output", "inout"}


def census(root: str) -> dict:
    abbreviations: list[dict] = []
    full_words = 0
    tables = 0
    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        for table in source.get("structured_tables") or []:
            if table.get("table_kind") != "signal_description":
                continue
            headers = header_texts(table)
            if not any("direction" in header for header in headers):
                continue
            tables += 1
            for row in table.get("body_rows") or []:
                for column, cell in enumerate(row):
                    value = cell["text"].strip()
                    lowered = value.lower()
                    if lowered in ABBREVIATIONS:
                        abbreviations.append({
                            "document": document,
                            "table_id": table.get("table_id"),
                            "column": column,
                            "cell": value,
                            "headers": headers,
                            # The only rows a rule could USE are those in a column the header names
                            # a direction; the rest are the garble showing through.
                            "in_a_direction_column": "direction" in (
                                headers[column] if column < len(headers) else ""
                            ),
                        })
                    elif lowered in FULL_WORDS:
                        full_words += 1
    usable = [row for row in abbreviations if row["in_a_direction_column"]]
    return {
        "tables_with_a_direction_header": tables,
        "abbreviation_cells": abbreviations,
        "abbreviation_count": len(abbreviations),
        "usable_abbreviation_count": len(usable),
        "abbreviation_documents": sorted({row["document"] for row in abbreviations}),
        "abbreviation_tables": sorted({
            (row["document"], row["table_id"]) for row in abbreviations
        }),
        "full_word_rows": full_words,
        "by_literal": dict(collections.Counter(row["cell"] for row in abbreviations)),
    }


def render(result: dict) -> None:
    print("=== SIGNAL-DECLARATION-ROW-DROP.2j — whole-cell direction abbreviations (read-only) ===")
    print(f"  signal_description tables whose header names a direction: "
          f"{result['tables_with_a_direction_header']}")
    print(f"  whole-cell FULL WORD rows (the notation production already reads): "
          f"{result['full_word_rows']}")
    print(f"  whole-cell ABBREVIATION cells: {result['abbreviation_count']}"
          f"  -> {result['by_literal']}")
    print(f"  of which in a column the header names a direction (USABLE): "
          f"{result['usable_abbreviation_count']}")
    print(f"  documents: {result['abbreviation_documents']}")
    print(f"  tables:    {[f'{d}/{t}' for d, t in result['abbreviation_tables']]}")
    for row in result["abbreviation_cells"]:
        mark = "USABLE" if row["in_a_direction_column"] else "garble"
        print(f"    [{mark}] {row['document'][:26]:28s} {row['table_id']:11s}"
              f" col={row['column']} cell={row['cell']!r}")
    print()
    print("  VERDICT: a rule needs a population. One usable row, in one table, in one document —")
    print("  and that table is column-garbled, which is why four of the five cells are in the NAME")
    print("  column. `.2h.0`'s refusal stands for the header-named population too, for a different")
    print("  reason than it stands for the fallback one: there is nothing here to read.")


# The population as `.2j` measured it. A corpus rebuild that moves these is a real event: the verdict
# is "one row is not a grammar", and it stops being true the moment there is more than one.
PINNED = {
    "tables_with_a_direction_header": 155,
    "abbreviation_count": 5,
    "usable_abbreviation_count": 1,
    "full_word_rows": 837,
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

    result = census(root)
    for key, expected in PINNED.items():
        check(f"population-pin-{key}", result[key] == expected)

    # RED 1 — the verdict rests on the USABLE count, not the raw one. A producer that counted all
    # five would report a population five times the real one and could argue for the rule.
    check("usable-is-narrower-than-raw",
          result["usable_abbreviation_count"] < result["abbreviation_count"])

    # RED 2 — the comparison is what makes the refusal decisive. If the full words were also rare the
    # census would be measuring a column shape nobody uses rather than an unused abbreviation.
    check("full-words-dwarf-the-abbreviation",
          result["full_word_rows"] > 100 * result["usable_abbreviation_count"])

    # RED 3 — one document, one table. "One row is not a grammar" is only true while that holds.
    check("population-is-one-table-in-one-document",
          len(result["abbreviation_documents"]) == 1 and len(result["abbreviation_tables"]) == 1)

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"direction-abbreviation population: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    args = parser.parse_args()
    root = repo_root()
    if args.self_test:
        return self_test(root)
    result = census(root)
    if args.json:
        json.dump(result, sys.stdout, indent=2, sort_keys=True, default=list)
        sys.stdout.write("\n")
        return 0
    render(result)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
