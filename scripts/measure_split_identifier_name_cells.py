#!/usr/bin/env python3
"""Census the name cells that are ONE identifier the text layer split in two (read-only).

`TEXT-LAYER-IDENTIFIER-SPLIT.0` — a specification writes `c_opcode`; its PDF typesets the
underscore, the text layer does not carry it, and the reader sees `c opcode` and keeps the
first token. The wire is not dropped and not refused: it is renamed to `c`, and so is every
other row of the table, so the column can offer exactly ONE distinct name however it is
scored and loses the table to whichever neighbour offers more.

**The signature is a property of the COLUMN, not of the cell.** `Clock source` and
`c opcode` are the same shape in isolation; what separates them is that a split column
repeats its leading token down every row while its continuations differ —
`c opcode`/`c param`/`c valid` all lead with `c` — whereas a prose column repeats nothing
(`Clock source`, `Reset controller`). This census measures that signature and nothing else;
it proposes no rule, because a rule that joins two words into an identifier is the most
expensive mistake available at this reader (ADR 0006).

It also asks whether the evidence to rejoin is REACHABLE, which decides whether a repair is
possible at all: for each candidate, whether the document's own text anywhere spells
`<lead>_<continuation>`. For TileLink the answer is no — its 36 underscored spellings live
inside timing-diagram figures, which the ingest captures as images, so the joining is stated
in the document and not in any text SpecForge reads.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/source_ir/*/source_ir.json`, every table whose persisted
`table_kind` is `signal_description` union every table named in that document's
`table_signal_declaration_provenance`.

This census classifies the POPULATION. It is not a check on the Rust implementation and must
never be cited as one (`CLAIM_VERIFICATION.md` §2).

Usage:
    python3 scripts/measure_split_identifier_name_cells.py
    python3 scripts/measure_split_identifier_name_cells.py --json
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
    column_selection,
    declaring_table_ids,
    header_texts,
    is_hardware_signal_token,
    is_signal_name_column_header,
    read_schema_constant,
    repo_root,
    trim_to_identifier_characters,
)

# A split column must repeat its leading token over at least this many rows. Two rows is the
# smallest number that can repeat at all; the census prints every case so the floor is a
# reporting boundary and not a tuned threshold.
MIN_REPEATED_ROWS = 2


def split_shape(raw: str) -> tuple[str, str] | None:
    """`(lead, continuation)` when the cell is a leading identifier plus one more word."""
    tokens = raw.split()
    if len(tokens) < 2:
        return None
    lead = trim_to_identifier_characters(tokens[0])
    if not is_hardware_signal_token(lead):
        return None
    tail = trim_to_identifier_characters(tokens[1])
    if not tail or not all((c.isascii() and c.isalnum()) or c == "_" for c in tail):
        return None
    return lead, tail


def document_joined_spellings(source: dict) -> set[str]:
    """Every `<a>_<b>` token the document's own text or tables spell, lower-cased."""
    joined = set()

    def scan(text: str) -> None:
        for token in text.replace("\t", " ").split():
            stripped = trim_to_identifier_characters(token)
            if "_" in stripped and is_hardware_signal_token(stripped):
                joined.add(stripped.lower())

    for element in source.get("content_elements") or []:
        scan(element.get("text") or "")
    for table in source.get("structured_tables") or []:
        for rows in (table.get("header_rows") or [], table.get("body_rows") or []):
            for row in rows:
                for cell in row:
                    scan(cell["text"])
    return joined


def census(root: str, current_schema: int) -> dict:
    schema_by_document = {}
    for path in glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json")):
        with open(path, "r", encoding="utf-8") as handle:
            schema_by_document[os.path.basename(os.path.dirname(path))] = json.load(
                handle
            ).get("schema_version")

    strata = {key: {"tables": 0, "columns": []} for key in ("current", "legacy")}

    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        key = "current" if schema_by_document.get(document) == current_schema else "legacy"
        bucket = strata[key]
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        declaring = declaring_table_ids(root, document)
        joined = document_joined_spellings(source)
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
            chosen_col, _offset, _width_col = column_selection(table)
            col_count = max((len(row) for row in rows), default=0)
            for col in range(col_count):
                shapes = []
                for row in rows:
                    if col >= len(row):
                        continue
                    shape = split_shape(row[col]["text"].strip())
                    if shape:
                        shapes.append(shape)
                if len(shapes) < MIN_REPEATED_ROWS:
                    continue
                leads = collections.Counter(lead for lead, _ in shapes)
                lead, repeats = leads.most_common(1)[0]
                if repeats < MIN_REPEATED_ROWS:
                    continue
                members = [(a, b) for a, b in shapes if a == lead]
                continuations = {b.lower() for _, b in members}
                if len(continuations) < len(members):
                    # The continuations must distinguish the rows; a repeat means the cells
                    # are not each naming something different and this is not a catalog.
                    continue
                recoverable = sorted(
                    c for c in continuations if f"{lead.lower()}_{c}" in joined
                )
                bucket["columns"].append(
                    {
                        "document": document,
                        "table_id": table.get("table_id", ""),
                        "caption": (table.get("caption_text") or "")[:60],
                        "col": col,
                        "header": headers[col] if col < len(headers) else "",
                        "is_header_name_col": col == header_col,
                        "is_chosen_col": col == chosen_col,
                        "declares": table.get("table_id", "") in declaring,
                        "rows": len(rows),
                        "lead": lead,
                        "members": [f"{a} {b}" for a, b in members],
                        "recoverable_joins": recoverable,
                    }
                )
    return strata


def render(strata: dict, current_schema: int) -> None:
    print("=== split-identifier name-cell census (read-only, persisted artifacts) ===")
    print(f"  current EvidenceIR schema : {current_schema}")
    for key in ("current", "legacy"):
        bucket = strata[key]
        label = "CURRENT (proof-carrying)" if key == "current" else "LEGACY (inspection-only)"
        columns = bucket["columns"]
        print()
        print(f"--- {label} stratum")
        print(f"    tables examined                      : {bucket['tables']}")
        print(f"    columns with a repeated leading token: {len(columns)}")
        print(f"      ... that the reader CHOSE as names : "
              f"{sum(1 for c in columns if c['is_chosen_col'])}")
        print(f"      ... the header designates as names : "
              f"{sum(1 for c in columns if c['is_header_name_col'])}")
        print(f"      ... in a table that DECLARES       : "
              f"{sum(1 for c in columns if c['declares'])}")
        print(f"      ... with the join spelled in-document anywhere: "
              f"{sum(1 for c in columns if c['recoverable_joins'])}")
        if not columns:
            continue
        by_document = collections.Counter(c["document"] for c in columns)
        print("    by document:")
        for document, count in by_document.most_common():
            print(f"      {count:4d}  {document}")
        print("    every column, verbatim:")
        for entry in sorted(
            columns, key=lambda c: (not c["declares"], c["document"], c["table_id"], c["col"])
        ):
            role = []
            if entry["is_header_name_col"]:
                role.append("header-name")
            if entry["is_chosen_col"]:
                role.append("CHOSEN")
            if entry["declares"]:
                role.append("declares")
            print(f"      {entry['document'][:28]:28s} {entry['table_id']:11s} "
                  f"col {entry['col']} header={entry['header']!r} "
                  f"lead={entry['lead']!r} [{','.join(role) or 'inert'}]")
            print(f"         cells: {entry['members'][:8]}")
            if entry["recoverable_joins"]:
                print(f"         the document ALSO spells: "
                      f"{[entry['lead'].lower() + '_' + c for c in entry['recoverable_joins']][:8]}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    root = repo_root()
    current_schema = read_schema_constant(root)
    strata = census(root, current_schema)
    if args.json:
        json.dump({"current_schema": current_schema, "strata": strata}, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(strata, current_schema)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
