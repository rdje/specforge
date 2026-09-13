#!/usr/bin/env python3
"""Census the tables whose name column is ZEROED by the whole-cell score, and what
wins the column instead (read-only).

`PROSE-NAME-CELL-DECLARATION.2` made a cell score for its column only when the reader
consumes it whole, so a column of prose stops scoring like a column of names. That rule
has a recall side, and `.5` is it: a column of GENUINE names whose cells carry a space
scores **zero** too, and the content-based override then hands the table to whichever
neighbour holds bare tokens — including a `Type` column of single letters.

TileLink is the found case. Its four channel tables head
`Signal | Type | Width | Description` over a body whose names are `c opcode`, `d param`,
`c valid`; column 0 scores 0, the `Type` column's `{C, D, V, R}` wins by 4, and the whole
table rotates, so the width column lands on `Description` and every declaration is named
after a single letter.

This script measures the population before anything is designed, per the leaf's own
instruction. It answers three questions and prints every case verbatim:

  1. how many tables have a header-designated name column that scores **0** under the
     whole-cell rule while still offering two or more leading identifiers — the columns
     the rule zeroes rather than merely lowers;
  2. in how many of those the override then takes the column somewhere else, and where;
  3. what the winning column looks like when it does — in particular its **injectivity**,
     distinct tokens over rows, because a catalog names each row once while a category
     column repeats. That ratio is the first candidate discriminator and this census
     exists to say whether it separates the corpus or only this table.

**The reader mirror is imported, not copied.** `measure_parametric_width_cell_shapes.py`
already replays the reader's column selection and carries the join control that proves it
still matches the producer. A second copy would drift silently, which is the failure this
repository keeps finding in its own instruments; so this script imports that one and adds
only its own questions.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/source_ir/*/source_ir.json`, every table whose persisted
`table_kind` is `signal_description` union every table named in that document's
`table_signal_declaration_provenance` — the same boundary, for the same reason.

This census classifies the POPULATION. It is not a check on the Rust implementation and
must never be cited as one (`CLAIM_VERIFICATION.md` §2).

Usage:
    python3 scripts/measure_name_column_whole_cell_score.py
    python3 scripts/measure_name_column_whole_cell_score.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from measure_parametric_width_cell_shapes import (  # noqa: E402
    NAME_COLUMN_OVERRIDE_MARGIN,
    declaring_table_ids,
    header_texts,
    is_hardware_signal_token,
    is_signal_name_column_header,
    name_cell_is_read_whole,
    read_schema_constant,
    repo_root,
    trim_to_identifier_characters,
)


def leading_identifier(cell_text: str) -> str:
    parts = cell_text.split()
    return trim_to_identifier_characters(parts[0]) if parts else ""


def column_profile(rows: list[list[dict]], col: int) -> dict:
    """Both scores for one column: the reader's current one, and the one it replaced."""
    whole, leading, cells = set(), set(), 0
    for row in rows:
        if col >= len(row):
            continue
        raw = row[col]["text"].strip()
        if not raw:
            continue
        cells += 1
        token = leading_identifier(raw).upper()
        if not is_hardware_signal_token(token):
            continue
        leading.add(token)
        if name_cell_is_read_whole(raw):
            whole.add(token)
    return {
        "cells": cells,
        "whole_distinct": len(whole),
        "leading_distinct": len(leading),
        "injectivity": round(len(whole) / cells, 3) if cells else 0.0,
    }


def census(root: str, current_schema: int) -> dict:
    schema_by_document = {}
    for path in glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json")):
        with open(path, "r", encoding="utf-8") as handle:
            schema_by_document[os.path.basename(os.path.dirname(path))] = json.load(
                handle
            ).get("schema_version")

    strata = {
        key: {"tables": 0, "zeroed": [], "zeroed_and_overridden": []}
        for key in ("current", "legacy")
    }

    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        key = "current" if schema_by_document.get(document) == current_schema else "legacy"
        bucket = strata[key]
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        declaring = declaring_table_ids(root, document)
        for table in source.get("structured_tables") or []:
            if (
                table.get("table_kind") != "signal_description"
                and table.get("table_id") not in declaring
            ):
                continue
            rows = table.get("body_rows") or []
            if not rows or (table.get("col_count") or 0) < 2:
                continue
            bucket["tables"] += 1
            headers = header_texts(table)
            header_col = 0
            for index, header in enumerate(headers):
                if is_signal_name_column_header(header):
                    header_col = index
                    break
            col_count = max((len(row) for row in rows), default=0)
            profiles = [column_profile(rows, col) for col in range(col_count)]
            if header_col >= len(profiles):
                continue
            named = profiles[header_col]
            # The population: a header-designated column the whole-cell rule takes to ZERO
            # while its cells still lead with identifiers the old score would have counted.
            if not (named["whole_distinct"] == 0 and named["leading_distinct"] >= 2):
                continue

            best_col, best = header_col, named["whole_distinct"]
            for col in range(col_count):
                if profiles[col]["whole_distinct"] >= best:
                    best_col, best = col, profiles[col]["whole_distinct"]
            overridden = (
                best_col != header_col
                and best >= 2
                and best >= named["whole_distinct"] + NAME_COLUMN_OVERRIDE_MARGIN
            )
            entry = {
                "document": document,
                "table_id": table.get("table_id", ""),
                "caption": (table.get("caption_text") or "")[:70],
                "headers": headers,
                "rows": len(rows),
                "header_col": header_col,
                "header_profile": named,
                "winning_col": best_col if overridden else header_col,
                "winning_profile": profiles[best_col] if overridden else named,
                "overridden": overridden,
                "declares": table.get("table_id", "") in declaring,
                "sample_name_cells": [
                    row[header_col]["text"].strip()
                    for row in rows[:4]
                    if header_col < len(row)
                ],
                "sample_winning_cells": (
                    [row[best_col]["text"].strip() for row in rows[:4] if best_col < len(row)]
                    if overridden
                    else []
                ),
            }
            bucket["zeroed"].append(entry)
            if overridden:
                bucket["zeroed_and_overridden"].append(entry)
    return strata


def render(strata: dict, current_schema: int) -> None:
    print("=== name columns the whole-cell score zeroes (read-only, persisted artifacts) ===")
    print(f"  current EvidenceIR schema : {current_schema}")
    print(f"  NAME_COLUMN_OVERRIDE_MARGIN: {NAME_COLUMN_OVERRIDE_MARGIN}")
    for key in ("current", "legacy"):
        bucket = strata[key]
        label = "CURRENT (proof-carrying)" if key == "current" else "LEGACY (inspection-only)"
        print()
        print(f"--- {label} stratum")
        print(f"    tables examined                       : {bucket['tables']}")
        print(f"    header name column scored ZERO        : {len(bucket['zeroed'])}")
        print(f"      ... and the override moved the column: {len(bucket['zeroed_and_overridden'])}")
        declaring = [e for e in bucket["zeroed"] if e["declares"]]
        print(f"      ... of which actually declare        : {len(declaring)}")
        if not bucket["zeroed"]:
            continue
        by_document = collections.Counter(e["document"] for e in bucket["zeroed"])
        print("    by document:")
        for document, count in by_document.most_common():
            print(f"      {count:4d}  {document}")
        print("    every case, verbatim:")
        for entry in sorted(
            bucket["zeroed"], key=lambda e: (not e["declares"], e["document"], e["table_id"])
        ):
            flag = "DECLARES" if entry["declares"] else "inert   "
            move = (
                f"col {entry['header_col']} -> {entry['winning_col']}"
                if entry["overridden"]
                else f"col {entry['header_col']} kept"
            )
            print(f"      [{flag}] {entry['document'][:30]:30s} {entry['table_id']:11s} "
                  f"rows={entry['rows']:3d}  {move}")
            print(f"                 headers  : {entry['headers']}")
            print(f"                 name col : {entry['sample_name_cells']}")
            if entry["overridden"]:
                win = entry["winning_profile"]
                print(f"                 winner   : {entry['sample_winning_cells']}  "
                      f"distinct={win['whole_distinct']} cells={win['cells']} "
                      f"injectivity={win['injectivity']}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    root = repo_root()
    current_schema = read_schema_constant(root)
    strata = census(root, current_schema)
    if args.json:
        json.dump(
            {"current_schema": current_schema, "strata": strata},
            sys.stdout,
            indent=2,
            sort_keys=True,
        )
        sys.stdout.write("\n")
        return 0
    render(strata, current_schema)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
