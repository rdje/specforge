#!/usr/bin/env python3
"""Adjudicate the decimal-before-binary ordering `KG-ISF-COMPLETENESS.5.viii` exposed.

`parse_encoding_numeric_literal` (`crates/specforge/src/ir/evidence.rs`) tries `parse::<u32>()`
**before** its all-`0`/`1` arm, so a bare `0010` reads as **10** and `1110` reads as **1110**. The
binary arm is reachable only when the decimal parse OVERFLOWS `u32`, that is never below eleven
digits, which makes the radix a function of the string's LENGTH with a discontinuity at 11
characters. `.5.viii` fixed the QUOTED notation at the member seam and handed the unquoted cells
here.

## What this instrument answers, and why the handed-down number could not

`.5.xi` was opened on a corpus-wide CELL census — *"230 with a leading zero (2 replayable), 943
without (156 replayable)"* — and concluded the defect *"reaches the current producer in quantity"*.
A cell census cannot support that conclusion. A table cell only becomes a published value when it
lands in the `value_col` that `infer_encoding_column_indices` picks for a table that actually MINTS
an enum, and most all-`0`/`1` cells in this corpus are bit positions, bit ranges and slot numbers
that never reach that seam. The two censuses are therefore reported side by side and only the second
is a fact about what SpecForge publishes:

    corpus cells   every body-row cell the shared reader would read as decimal while being an
                   all-`0`/`1` digit string. Read it as EXPOSURE, never as effect.
    member slots   minted `Enum <name> <member> = <value>.` statements whose value came from such a
                   cell, attributed per member by `.5.vi`'s replay walk. This is the defect.

The cell census additionally re-derives with the producer's OWN trimming. The handed-down 943
counted the raw cell text; `parse_encoding_numeric_literal` trims `[` and `]` first, so a bracketed
`[10]` reaches the same decimal branch and belongs to the same population. That is 67 cells the
opening census did not see — and they matter in the opposite direction, because a bracketed cell is
a bit-POSITION and bit positions are exactly the decimal-intending population the repair must not
disturb.

## The discriminator

A leading zero is a fact about NOTATION: no publisher writes a decimal ten as `0010`. A bare `10`,
`11` or `101` is genuinely ambiguous in isolation, so the column and the row it sits in are asked
for a witness. All three tests are content SHAPE — no header vocabulary, no document key (ADR 0006):

    notational witness    a digit-string cell, all-`0`/`1`, of length >= 2, beginning with `0`,
                          found in the same column at the SAME WIDTH or anywhere in the same ROW.
                          Zero-padding to a fixed width is what binary notation looks like.
    decimal refutation    a digit-string cell in the same column at the same width carrying a digit
                          >= 2. That is what ZERO-PADDED DECIMAL looks like (`01 … 09 10 11 12`), and
                          it must outrank the witness or the repair would corrupt such a column.
    decimal fallback      absent any witness, a digit >= 2 anywhere in the column. A column running
                          `0 1 2 … 11` is a decimal RUN in which `10` and `11` are all-`0`/`1` by
                          coincidence; this is the single largest decimal-intending class.

Width stratification is not decoration. `opencapi_4_0` `table_0092` fuses a bit-position table
(`cmd_flag bit` = `3 2 1 0`) with an encode table (`0010`, `0011`) in ONE column, so an unstratified
`digit >= 2` test refutes the binary reading of a cell the document plainly states in binary. The
strata are width 1 and width 4 and they disagree, which is the document's own layout, not noise.

## The residual is the point of the exercise

Whatever the tests cannot decide is REPORTED, not guessed. `.5.vii.a` established that this
programme prefers an honest residual to a fabricated certainty, and a bare `11` in a column holding
nothing else is exactly that case.

Read-only; no rebuild, no VLM, no Docling. Usage:
  python3 scripts/adjudicate_decimal_before_binary_encoding_values.py [--json] [--document KEY]
                                                                     [--residual] [--limit N]
"""
import argparse
import collections
import json
import os
import pathlib
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from adjudicate_positional_enum_values import (  # noqa: E402
    attribute_members,
    minted_enums,
    replay_table_with_provenance,
)

ROOT = pathlib.Path(__file__).resolve().parent.parent
U32_MAX = 0xFFFFFFFF
DIGITS = re.compile(r"[0-9]+")
NON_BINARY_DIGIT = re.compile(r"[2-9]")


def producer_trim(text):
    """`parse_encoding_numeric_literal`'s own trimming: `.trim()` then `trim_matches('['|']')`."""
    return (text or "").strip().strip("[]")


def decimal_before_binary_digits(text):
    """The digits of a cell the shared reader mis-reads as decimal, else `None`.

    A cell qualifies when it is a digit string the decimal arm accepts, every digit is `0` or `1`,
    and the two radixes DISAGREE. The last clause removes the cells where the defect is invisible:
    a single `0` or `1`, and any all-zero string, read the same either way and are not at issue.
    """
    trimmed = producer_trim(text)
    if len(trimmed) < 2 or not DIGITS.fullmatch(trimmed):
        return None
    decimal = int(trimmed, 10)
    if decimal > U32_MAX or NON_BINARY_DIGIT.search(trimmed):
        return None
    return trimmed if decimal != int(trimmed, 2) else None


def column_digit_cells(table, column):
    """Every digit-string cell of one body column, trimmed as the producer trims."""
    cells = []
    for row in table.get("body_rows") or []:
        if column < len(row):
            text = producer_trim(row[column].get("text"))
            if DIGITS.fullmatch(text):
                cells.append(text)
    return cells


def row_digit_cells(row, column):
    """Every digit-string cell of one row APART from the value cell itself."""
    cells = []
    for index, cell in enumerate(row):
        if index == column:
            continue
        text = producer_trim(cell.get("text"))
        if DIGITS.fullmatch(text):
            cells.append(text)
    return cells


def is_zero_padded_binary(text):
    return len(text) >= 2 and text.startswith("0") and not NON_BINARY_DIGIT.search(text)


VERDICTS = ("binary", "zero_padded_decimal", "decimal", "undecidable")


def adjudicate(digits, column_cells, row_cells):
    """Which radix the document's own layout states for this cell.

    Returns the verdict and the shape facts behind it, so a reader can audit the call rather than
    trust it. The refutation is tested FIRST in both branches: a zero-padded decimal column carries
    the same leading zeros as a binary one, and only the digits above `1` tell them apart.
    """
    width = len(digits)
    same_width = [cell for cell in column_cells if len(cell) == width]
    refuted = any(NON_BINARY_DIGIT.search(cell) for cell in same_width)
    column_witness = any(is_zero_padded_binary(cell) for cell in same_width)
    row_witness = any(is_zero_padded_binary(cell) for cell in row_cells)
    decimal_anywhere = any(NON_BINARY_DIGIT.search(cell) for cell in column_cells)

    facts = {
        "same_width_decimal_refutation": refuted,
        "column_witness": column_witness,
        "row_witness": row_witness,
        "decimal_digit_in_column": decimal_anywhere,
    }
    if digits.startswith("0"):
        return ("zero_padded_decimal" if refuted else "binary"), facts
    if refuted:
        return "decimal", facts
    if column_witness or row_witness:
        return "binary", facts
    return ("decimal" if decimal_anywhere else "undecidable"), facts


def corpus_cell_census():
    """Exposure: every body-row cell the shared reader would mis-read, by stratum and by trimming.

    `raw` repeats the opening census's population (cell text as written) so the two are comparable;
    `trimmed` is the producer's own population and is the correct one.
    """
    census = collections.Counter()
    documents = collections.defaultdict(set)
    for source_path in sorted((ROOT / "generated" / "source_ir").glob("*/source_ir.json")):
        document = source_path.parent.name
        source = json.loads(source_path.read_text(encoding="utf-8"))
        evidence_path = ROOT / "generated" / "evidence_ir" / document / "evidence_ir.json"
        replayable = False
        if evidence_path.exists():
            with evidence_path.open(encoding="utf-8") as handle:
                replayable = json.load(handle).get("schema_version") == 3
        stratum = "replayable" if replayable else "legacy"
        for table in source.get("structured_tables", []):
            for row in table.get("body_rows") or []:
                for cell in row:
                    text = (cell.get("text") or "")
                    digits = decimal_before_binary_digits(text)
                    if digits is None:
                        continue
                    klass = "leading_zero" if digits.startswith("0") else "bare"
                    bracketed = text.strip() != producer_trim(text)
                    census[(klass, stratum, "trimmed")] += 1
                    documents[(klass, "trimmed")].add(document)
                    if not bracketed:
                        census[(klass, stratum, "raw")] += 1
                        documents[(klass, "raw")].add(document)
    return census, documents


def member_slot_census(document_filter):
    """The defect: minted member values that came from a mis-read cell, adjudicated per slot."""
    slots = []
    for evidence_path in sorted((ROOT / "generated" / "evidence_ir").glob("*/evidence_ir.json")):
        document = evidence_path.parent.name
        if document_filter and document != document_filter:
            continue
        source_path = ROOT / "generated" / "source_ir" / document / "source_ir.json"
        if not source_path.exists():
            continue
        evidence = json.loads(evidence_path.read_text(encoding="utf-8"))
        source = json.loads(source_path.read_text(encoding="utf-8"))
        replayable = evidence.get("schema_version") == 3
        tables = source.get("structured_tables", [])
        by_id = {table.get("table_id"): table for table in tables}
        for name, members in minted_enums(evidence).items():
            replays = []
            for table in tables:
                entries = replay_table_with_provenance(table, name)
                if {entry["pair"] for entry in entries} & set(members):
                    replays.append((table.get("table_id"), entries))
            attributed = (attribute_members(members, replays, 0)
                          or attribute_members(members, replays, 3))
            if attributed is None or len(attributed) != len(members):
                continue
            for (member, value), (table_id, entry) in zip(members, attributed):
                if entry["positional"]:
                    continue
                cells = entry["row"]
                if entry["value_col"] >= len(cells):
                    continue
                raw = cells[entry["value_col"]].get("text") or ""
                digits = decimal_before_binary_digits(raw)
                if digits is None:
                    continue
                table = by_id.get(table_id)
                column_cells = column_digit_cells(table, entry["value_col"]) if table else []
                verdict, facts = adjudicate(digits, column_cells, row_digit_cells(cells, entry["value_col"]))
                headers = [(cell.get("text") or "")
                           for cell in ((table.get("header_rows") or [[]])[0] if table else [])]
                slots.append({
                    "document": document, "enum": name, "member": member,
                    "published_value": value, "binary_value": int(digits, 2),
                    "cell": raw, "digits": digits, "table": table_id,
                    "value_col": entry["value_col"], "arm": entry["arm"],
                    "class": "leading_zero" if digits.startswith("0") else "bare",
                    "verdict": verdict, "replayable": replayable,
                    "header": headers[entry["value_col"]] if entry["value_col"] < len(headers) else "",
                    "caption": (table.get("caption_text") or "") if table else "",
                    "column_cells": column_cells[:16],
                    **facts,
                })
    return slots


def parse_success_invariance():
    """Can reordering the arms change WHETHER a cell parses, rather than only its value?

    It cannot, and the corpus is asked to agree. A decimal parse succeeds only at ten digits or
    fewer, where an all-`0`/`1` string is at most 1023 and cannot overflow binary; above ten digits
    the decimal arm already fails and the binary arm already runs. So the set of cells that parse is
    identical either way, and the change is value-affecting ONLY — which is what keeps it out of
    `derive_header_sourced_enum_name` clause 5, the seam `.5.viii` had to avoid and `.5.vi.a` was
    reverted for. This is the property that makes the repair non-additive; it is checked, not assumed.
    """
    divergent = []
    seen = set()
    for source_path in sorted((ROOT / "generated" / "source_ir").glob("*/source_ir.json")):
        source = json.loads(source_path.read_text(encoding="utf-8"))
        for table in source.get("structured_tables", []):
            for row in (table.get("body_rows") or []) + (table.get("header_rows") or []):
                for cell in row:
                    text = (cell.get("text") or "")
                    trimmed = producer_trim(text)
                    if trimmed in seen:
                        continue
                    seen.add(trimmed)
                    if not trimmed or not DIGITS.fullmatch(trimmed):
                        continue
                    if NON_BINARY_DIGIT.search(trimmed):
                        continue
                    decimal_ok = int(trimmed, 10) <= U32_MAX
                    binary_ok = int(trimmed, 2) <= U32_MAX
                    if decimal_ok != (binary_ok or decimal_ok):
                        divergent.append(trimmed)
    return len(seen), divergent


def report(census, documents, slots, residual, limit):
    print("EXPOSURE — body-row cells the shared reader reads as decimal while all-`0`/`1`.")
    print("Read as exposure, NOT as effect: most of these are bit positions that never reach a")
    print("value column. Effect is the member-slot census below.")
    print(f"  {'population':34s} {'leading zero':>13s} {'bare':>8s} {'total':>8s}")
    for label, trimming in (("as written (the opening census)", "raw"),
                            ("bracket-trimmed (the PRODUCER)", "trimmed")):
        lead = sum(census[("leading_zero", s, trimming)] for s in ("replayable", "legacy"))
        bare = sum(census[("bare", s, trimming)] for s in ("replayable", "legacy"))
        print(f"  {label:34s} {lead:13d} {bare:8d} {lead + bare:8d}")
    lead_r = census[("leading_zero", "replayable", "trimmed")]
    bare_r = census[("bare", "replayable", "trimmed")]
    print(f"  {'  of which replayable':34s} {lead_r:13d} {bare_r:8d} {lead_r + bare_r:8d}")
    print("  The bracketed difference is the bit-POSITION population `[10]`, `[11]` — the cells the")
    print("  repair must leave alone, which is why counting them matters.")

    attributed = len(slots)
    replayable = [slot for slot in slots if slot["replayable"]]
    print(f"\nEFFECT — minted member values sourced from such a cell: {attributed}")
    print(f"  in the REPLAYABLE stratum (what SpecForge publishes today): {len(replayable)}")
    print(f"  in the LEGACY stratum (artifacts minted by binaries that are gone): "
          f"{attributed - len(replayable)}")
    print(f"  documents: {len({slot['document'] for slot in slots})}  "
          f"enums: {len({(slot['document'], slot['enum']) for slot in slots})}")

    print("\nADJUDICATION — which radix the document's own layout states:")
    verdicts = collections.Counter(slot["verdict"] for slot in slots)
    for verdict in VERDICTS:
        count = verdicts[verdict]
        print(f"  {verdict:24s} {count:6d}  ({100 * count / max(attributed, 1):4.1f}%)")
    print(f"\n  {'':24s} {'leading zero':>13s} {'bare':>8s}")
    for verdict in VERDICTS:
        lead = sum(1 for s in slots if s["verdict"] == verdict and s["class"] == "leading_zero")
        bare = sum(1 for s in slots if s["verdict"] == verdict and s["class"] == "bare")
        if lead or bare:
            print(f"  {verdict:24s} {lead:13d} {bare:8d}")

    print("\n  which witness carried a BARE cell to a binary verdict:")
    carried = [s for s in slots if s["verdict"] == "binary" and s["class"] == "bare"]
    for label, key in (("same-width zero-padded sibling in the COLUMN", "column_witness"),
                       ("zero-padded sibling in the same ROW", "row_witness")):
        print(f"    {label:44s} {sum(1 for s in carried if s[key]):6d}")

    distinct, divergent = parse_success_invariance()
    print(f"\nNON-ADDITIVE PROOF — over {distinct} distinct trimmed cells, the number whose")
    print(f"  parse SUCCESS would change if the arms were reordered: {len(divergent)}")
    print("  Zero is the required answer. The repair changes values only, so it cannot move")
    print("  `derive_header_sourced_enum_name` clause 5, `infer_encoding_column_indices`, or which")
    print("  enums exist — the identity seams `.5.viii` was confined away from.")

    print("\nRESIDUAL — cells no shape test decides; these stay as they are read today:")
    undecided = [slot for slot in slots if slot["verdict"] == "undecidable"]
    columns = collections.defaultdict(list)
    for slot in undecided:
        columns[(slot["document"], slot["table"], slot["value_col"])].append(slot)
    print(f"  {len(undecided)} slots in {len(columns)} columns.")
    if residual:
        for (document, table, column), group in columns.items():
            first = group[0]
            print(f"    {len(group):3d}x {document[:34]:34s} {table} col{column} "
                  f"header={first['header']!r:22s} column={first['column_cells'][:8]}")

    print(f"\nworst documents by affected member slots (top {limit}):")
    by_document = collections.Counter(slot["document"] for slot in slots)
    for document, count in by_document.most_common(limit):
        subset = [slot for slot in slots if slot["document"] == document]
        verdicts = collections.Counter(slot["verdict"] for slot in subset)
        print(f"  {document[:52]:52s} {count:5d}  "
              f"binary={verdicts['binary']:<5} decimal={verdicts['decimal']:<5} "
              f"undecidable={verdicts['undecidable']}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--document")
    parser.add_argument("--residual", action="store_true",
                        help="list every undecided column rather than only counting them")
    parser.add_argument("--limit", type=int, default=15)
    args = parser.parse_args()

    slots = member_slot_census(args.document)
    if args.json:
        json.dump(slots, sys.stdout, indent=1, default=str)
        print()
        return 0
    census, documents = corpus_cell_census()
    report(census, documents, slots, args.residual, args.limit)
    return 0


if __name__ == "__main__":
    sys.exit(main())
