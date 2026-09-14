#!/usr/bin/env python3
"""Census the name cells whose identifier the text layer split without a separator (read-only).

`TEXT-LAYER-IDENTIFIER-SPLIT.2` — a timing specification writes `t_PERIOD` as a base letter with
a typeset subscript. The PDF text layer keeps both runs and loses the size change, so SpecForge
reads the cell `t PERIOD` and the reader keeps the first token: the parameter is renamed to `t`,
and so is every other row, exactly as `.0` measured for the underscore case.

**The join here is CONCATENATION, not an insertion, and that is the whole difficulty.** `.0`'s
class is `c opcode` -> `c_opcode`: the document's own text spells the underscore, so a candidate
can be tested against evidence, and a wrong join at least leaves a visible `_`. Here the join is
`t PERIOD` -> `tPERIOD`, which leaves no mark of having guessed.

WHAT THIS CENSUS MEASURES — and the comparison IS the finding, so it reports both:

  TIER A (subscript)  the continuation is UPPER-CASE: `t PERIOD`, `C DEVICE`.
  TIER B (any word)   the continuation is any word: `t PERIOD`, and also `a opcode`.

  Tier B is what a naive "join two adjacent tokens" rule would take. It is reported because
  **tier B selects TileLink's entire signal catalog, whose correct join is an UNDERSCORE** —
  `a opcode` is `a_opcode`, never `aopcode`. The two classes have the SAME cell shape, so the
  shape cannot say which join to apply, and applying the wrong one is silently wrong.

  THE DISCRIMINATOR, measured rather than assumed: whether the document's own text or tables
  anywhere spell the CONCATENATION. That is the only in-document evidence a concatenating join
  could be grounded in, and it is what separates the two classes even though their cells match.

Scope. A cell can only become a declaration if it is in the column the reader reads, so the
population is the cells in the HEADER-DESIGNATED or READER-CHOSEN column. Cells elsewhere are
counted and summarised as the false-positive surface the signature would have to survive, never
dropped silently.

Boundary, identical to `.0` so the two populations are comparable: persisted
`generated/source_ir/*/source_ir.json`, every table whose persisted `table_kind` is
`signal_description` union every table named in that document's
`table_signal_declaration_provenance`.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.

This census classifies the POPULATION. It is not a check on the Rust implementation and must
never be cited as one (`CLAIM_VERIFICATION.md` §2).

Usage:
    python3 scripts/measure_subscript_split_name_cells.py
    python3 scripts/measure_subscript_split_name_cells.py --json
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

# A subscript lead is the base symbol a subscript hangs off: `t`, `C`, `V`, `a`. Two characters is
# the widest thing that still reads as a base symbol; beyond that the first token is a word, and a
# word plus a word is prose. The census prints every selection, so this is a reporting boundary.
MAX_LEAD_CHARACTERS = 2

# One character after a base symbol is overwhelmingly a column artefact (`t N`, `C V`); admitting
# it would make the population unreadable without saying anything about the class.
MIN_CONTINUATION_CHARACTERS = 2


def token_pairs(raw: str, require_upper_continuation: bool) -> list[tuple[str, str]]:
    """Every `(lead, continuation)` the cell offers under one tier of the signature.

    A cell may offer more than one, because the same cell can carry a comma family — eMMC's
    `t TLH , t THL` is two split identifiers and a family in one cell, which is the shape `.0`
    flagged and no rule proposed so far reads.
    """
    tokens = raw.split()
    pairs: list[tuple[str, str]] = []
    index = 0
    while index < len(tokens) - 1:
        lead = trim_to_identifier_characters(tokens[index])
        tail = trim_to_identifier_characters(tokens[index + 1])
        if (
            lead
            and tail
            and len(lead) <= MAX_LEAD_CHARACTERS
            and lead.isascii()
            and lead.isalpha()
            and len(tail) >= MIN_CONTINUATION_CHARACTERS
            and tail.isascii()
            and tail.isalnum()
            and not tail.isdigit()
            and is_hardware_signal_token(lead + tail)
            and (tail.upper() == tail if require_upper_continuation else True)
        ):
            pairs.append((lead, tail))
            index += 2
            continue
        index += 1
    return pairs


def document_concatenated_spellings(source: dict) -> set[str]:
    """Every identifier-shaped token the document's own text or tables spell, lower-cased.

    Reachability for THIS class is a concatenation, so the haystack is every token, not only the
    ones carrying a separator: `tTLH` is an ordinary identifier wherever it appears.
    """
    spelled: set[str] = set()

    def scan(text: str) -> None:
        for token in text.replace("\t", " ").replace(",", " ").split():
            stripped = trim_to_identifier_characters(token)
            if len(stripped) >= 3 and is_hardware_signal_token(stripped):
                spelled.add(stripped.lower())

    for element in source.get("content_elements") or []:
        scan(element.get("text") or "")
    for table in source.get("structured_tables") or []:
        for rows in (table.get("header_rows") or [], table.get("body_rows") or []):
            for row in rows:
                for cell in row:
                    scan(cell["text"])
    return spelled


def census(root: str, current_schema: int) -> dict:
    schema_by_document = {}
    for path in glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json")):
        with open(path, "r", encoding="utf-8") as handle:
            schema_by_document[os.path.basename(os.path.dirname(path))] = json.load(
                handle
            ).get("schema_version")

    result = {
        "boundary_tables": 0,
        "name_column_cells": [],
        "other_column_cells": 0,
        "other_column_documents": collections.Counter(),
    }

    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        stratum = "current" if schema_by_document.get(document) == current_schema else "legacy"
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        declaring = declaring_table_ids(root, document)
        spelled: set[str] | None = None
        for table in source.get("structured_tables") or []:
            if (
                table.get("table_kind") != "signal_description"
                and table.get("table_id") not in declaring
            ):
                continue
            rows = table.get("body_rows") or []
            if not rows or (table.get("col_count") or 0) < 2:
                continue
            result["boundary_tables"] += 1
            headers = header_texts(table)
            header_col = None
            for index, header in enumerate(headers):
                if is_signal_name_column_header(header):
                    header_col = index
                    break
            chosen_col, _offset, _width_col = column_selection(table)
            for row_index, row in enumerate(rows):
                for col, cell in enumerate(row):
                    raw = cell["text"].strip()
                    relaxed = token_pairs(raw, require_upper_continuation=False)
                    if not relaxed:
                        continue
                    if col not in (header_col, chosen_col):
                        result["other_column_cells"] += 1
                        result["other_column_documents"][document] += 1
                        continue
                    if spelled is None:
                        spelled = document_concatenated_spellings(source)
                    strict = token_pairs(raw, require_upper_continuation=True)
                    result["name_column_cells"].append(
                        {
                            "document": document,
                            "stratum": stratum,
                            "table_id": table.get("table_id"),
                            "declares": table.get("table_id") in declaring,
                            "row": row_index,
                            "column": col,
                            "header": headers[col] if col < len(headers) else "",
                            "text": raw,
                            "tier_a_subscript": [
                                {"joined": a + b, "spelled_in_document": (a + b).lower() in spelled}
                                for a, b in strict
                            ],
                            "tier_b_any_word": [
                                {"joined": a + b, "spelled_in_document": (a + b).lower() in spelled}
                                for a, b in relaxed
                            ],
                        }
                    )
    result["other_column_documents"] = dict(result["other_column_documents"])
    return result


def _reachable(cell: dict, tier: str) -> bool:
    return any(entry["spelled_in_document"] for entry in cell[tier])


def render(result: dict) -> None:
    cells = result["name_column_cells"]
    tier_a = [c for c in cells if c["tier_a_subscript"]]
    tier_b = cells
    reach_a = [c for c in tier_a if _reachable(c, "tier_a_subscript")]
    reach_b = [c for c in tier_b if _reachable(c, "tier_b_any_word")]

    print(f"boundary tables: {result['boundary_tables']}")
    print(
        f"cells OUTSIDE the name column that match tier B: {result['other_column_cells']} "
        f"across {len(result['other_column_documents'])} document(s) — the false-positive surface "
        "a shape-only rule would have to survive; they cannot reach a declaration and are not "
        "listed here."
    )
    print()
    print("NAME-COLUMN POPULATION (header-designated or reader-chosen column)")
    print(f"  tier A, UPPER-CASE continuation : {len(tier_a):4d} cell(s), "
          f"{len(reach_a)} with an in-document concatenation")
    print(f"  tier B, any-word continuation   : {len(tier_b):4d} cell(s), "
          f"{len(reach_b)} with an in-document concatenation")
    print()

    by_document = collections.Counter(c["document"] for c in tier_b)
    print("  tier B by document:")
    for document, count in sorted(by_document.items(), key=lambda kv: (-kv[1], kv[0])):
        hits = sum(1 for c in tier_b if c["document"] == document and _reachable(c, "tier_b_any_word"))
        print(f"    {count:4d}  {hits:3d} reachable  {document}")
    print()

    print("  every name-column cell whose concatenation the DOCUMENT ITSELF spells:")
    if not reach_b:
        print("    (none)")
    for cell in reach_b:
        joined = ", ".join(
            entry["joined"] for entry in cell["tier_b_any_word"] if entry["spelled_in_document"]
        )
        flags = "DECLARES" if cell["declares"] else "no declaration"
        print(
            f"    {cell['document']}/{cell['table_id']} r{cell['row']}c{cell['column']}"
            f" [{cell['header']}] {cell['stratum']}, {flags}"
        )
        print(f"       {cell['text']!r} -> {joined}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the raw census as JSON")
    args = parser.parse_args()

    root = repo_root()
    result = census(root, read_schema_constant(root))
    if args.json:
        json.dump(result, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
