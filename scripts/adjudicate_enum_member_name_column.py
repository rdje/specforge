#!/usr/bin/env python3
"""Adjudicate `KG-ISF-COMPLETENESS.5.xiii` — which column names an enum's members.

`infer_encoding_column_indices` (`crates/specforge/src/ir/evidence.rs`) picks `name_col` as the FIRST
header matching `name|meaning|description|state|transfer|response|type`, so AXI `Table A7.3` names its
members from `Description` while the document's own member name sits in the `Opcode` cell.

## The handed-down population is threshold-sensitive, and that is why it is re-derived here

`.5.xiii` was opened on an *"upper bound: 262 enum-minting tables across 34 documents"* whose shape
test — *"another column holds short, single-token, mostly-DISTINCT cells"* — was never written down.
Re-deriving it with that description in hand lands anywhere in **255-268 tables / 32-33 documents**
depending only on the length cut-off and the distinctness fraction chosen. The published figure sits
inside that band and cannot be reproduced. **A shape test with unrecorded thresholds is not a
measurement**, so this instrument declares no new threshold at all.

## It reuses the calibrated test this repository already owns

`synthesize_signal_declarations` answers the same question for SIGNAL tables and has been calibrated
against the corpus twice: `name_cell_is_read_whole` (`PROSE-NAME-CELL-DECLARATION.2`) decides whether a
cell is a NAME or prose left over, the column score is the count of DISTINCT names it yields, and
`NAME_COLUMN_OVERRIDE_MARGIN = 2` is the lead required before content may overrule a header —
deliberately not 1, because *"a prose column reaches it whenever one more sentence happens to open with
a capitalised word"*. Transposing that to the member seam needs no new vocabulary, no new constant and
no new threshold (ADR 0006).

## What it reports, and why the last two rows matter more than the first

    population      minting (table, enum) pairs whose `name_col` header is descriptive and whose
                    content the calibrated override would re-point by the established margin.
    multi-key       of those, the tables whose VALUE column repeats a code. A flat enum cannot
                    represent them, so re-pointing the name column alone would replace a naming
                    defect with a value COLLISION — which `.5.vii`'s census already calls provably
                    wrong, since no encoding assigns one code to two members.
    collisions      minted enums that ALREADY publish one code on two members. These survive
                    `.5.vii.a`'s fallback removal precisely because the cell does parse; the value
                    column is simply the wrong column.

Read-only; no rebuild, no VLM, no Docling. Usage:
  python3 scripts/adjudicate_enum_member_name_column.py [--json] [--document KEY] [--limit N]
"""
import argparse
import collections
import json
import os
import pathlib
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from adjudicate_conflated_enum_identity import (  # noqa: E402
    header_texts,
    infer_encoding_column_indices,
    minted_enums,
    sanitize_member,
)
from adjudicate_positional_enum_values import replay_table_with_provenance  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
DESCRIPTIVE_HEADER_WORDS = ("meaning", "description")
# `evidence.rs::NAME_COLUMN_OVERRIDE_MARGIN` — content overrules a header only on a decisive lead.
NAME_COLUMN_OVERRIDE_MARGIN = 2
SPLIT_BIT_HEADER = re.compile(r"^(.*?)\s*\[\s*\d+\s*\]$")


def is_hardware_signal_token(token):
    """`evidence.rs::is_hardware_signal_token`."""
    if not token:
        return False
    first, rest = token[0], token[1:]
    return (first.isascii() and (first.isalpha() or first == "_")) and all(
        character.isascii() and (character.isalnum() or character == "_") for character in rest
    )


def signal_names_in_name_cell(raw_name):
    """`evidence.rs::signal_names_in_name_cell` — a comma-separated family, else the first token."""
    def first_token(text):
        token = (text.split() or [""])[0]
        return "".join(
            character for character in token if character.isalnum() or character == "_"
        ) if token else ""

    single = [first_token(raw_name)]
    if "," not in raw_name:
        return single
    names = []
    for element in raw_name.split(","):
        element = element.strip()
        if len(element.split()) != 1 or not is_hardware_signal_token(element):
            return single
        names.append(element)
    return names if len(names) >= 2 else single


def name_cell_is_read_whole(raw_name):
    """`evidence.rs::name_cell_is_read_whole` — a name the reader consumes ENTIRELY, else prose.

    Mirrored exactly, because the whole point of this instrument is that the enum seam should reuse
    the test the signal seam already calibrated rather than invent a second one.
    """
    if len(signal_names_in_name_cell(raw_name)) >= 2:
        return True
    tokens = raw_name.split()
    if len(tokens) <= 1:
        return True
    later = tokens[1:]
    if all(len(t) == 1 and t.isascii() and t.isalnum() for t in later):
        return True
    if all(not any(c.isascii() and c.isalpha() for c in t) for t in later):
        return True
    lowered = [t.lower() for t in tokens]
    def recurs(token):
        return lowered.count(token) > 1
    def is_fragment(token):
        return sum(1 for c in token if c.isascii() and c.isalpha()) <= 1
    return any(recurs(t) for t in lowered) and all(recurs(t) or is_fragment(t) for t in lowered)


def member_name_score(table, column):
    """Distinct member names a column yields, counting only cells the reader consumes WHOLE."""
    names = set()
    for row in table.get("body_rows") or []:
        if column >= len(row):
            continue
        raw = (row[column].get("text") or "").strip()
        if not raw or not name_cell_is_read_whole(raw):
            continue
        member = sanitize_member(raw)
        if member:
            names.add(member)
    return len(names)


def column_cells(table, column):
    return [
        (row[column].get("text") or "").strip()
        for row in table.get("body_rows") or []
        if column < len(row)
    ]


def audit():
    rows, collisions = [], []
    for evidence_path in sorted((ROOT / "generated" / "evidence_ir").glob("*/evidence_ir.json")):
        document = evidence_path.parent.name
        source_path = ROOT / "generated" / "source_ir" / document / "source_ir.json"
        if not source_path.exists():
            continue
        evidence = json.loads(evidence_path.read_text(encoding="utf-8"))
        source = json.loads(source_path.read_text(encoding="utf-8"))
        replayable = evidence.get("schema_version") == 3
        minted = minted_enums(evidence)

        for name, members in minted.items():
            by_value = collections.Counter(value for _, value in members)
            colliding = sum(count - 1 for count in by_value.values() if count > 1)
            if colliding:
                collisions.append({
                    "document": document, "enum": name, "members": len(members),
                    "colliding": colliding, "replayable": replayable,
                })

        seen = set()
        for table in source.get("structured_tables", []):
            headers = header_texts(table)
            col_count = int(table.get("col_count") or 0)
            for name, members in minted.items():
                entries = replay_table_with_provenance(table, name)
                if not ({entry["pair"] for entry in entries} & set(members)):
                    continue
                key = (table.get("table_id"), name)
                if key in seen:
                    continue
                seen.add(key)
                name_col, value_col = infer_encoding_column_indices(table, name)
                chosen = member_name_score(table, name_col)
                best_col, best = name_col, chosen
                for column in range(col_count):
                    if column == value_col:
                        continue
                    score = member_name_score(table, column)
                    if score > best:
                        best_col, best = column, score
                values = [cell for cell in column_cells(table, value_col) if cell]
                header = headers[name_col] if name_col < len(headers) else ""
                value_header = headers[value_col] if value_col < len(headers) else ""
                rows.append({
                    "document": document, "table": table.get("table_id"), "enum": name,
                    "replayable": replayable,
                    "name_col": name_col, "value_col": value_col, "best_col": best_col,
                    "header": header,
                    "best_header": headers[best_col] if best_col < len(headers) else "",
                    "value_header": value_header,
                    "chosen_score": chosen, "best_score": best,
                    "descriptive": any(w in header for w in DESCRIPTIVE_HEADER_WORDS),
                    "misnamed": best_col != name_col and best - chosen >= NAME_COLUMN_OVERRIDE_MARGIN,
                    "rows": len(values), "distinct_values": len(set(values)),
                    "duplicate_values": len(values) - len(set(values)),
                    "multikey": len(values) != len(set(values)),
                    "value_col_is_one_bit": bool(SPLIT_BIT_HEADER.match(value_header.strip())),
                })
    return rows, collisions


def report(rows, collisions, limit):
    tables = lambda subset: len({(r["document"], r["table"]) for r in subset})
    population = [r for r in rows if r["misnamed"] and r["descriptive"]]
    multikey = [r for r in population if r["multikey"]]
    print(f"minting (table, enum) pairs: {len(rows)} across {tables(rows)} table(s)")
    print("\nPOPULATION — descriptive `name_col` the calibrated override would re-point by margin "
          f"{NAME_COLUMN_OVERRIDE_MARGIN}:")
    print(f"  {len(population)} pair(s) / {tables(population)} table(s) / "
          f"{len({r['document'] for r in population})} document(s); "
          f"{tables([r for r in population if r['replayable']])} replayable table(s)")
    print("  No threshold is declared here. The score is `name_cell_is_read_whole` + distinct names,")
    print("  and the margin is the one `synthesize_signal_declarations` already calibrated.")

    print("\nCANNOT BE A FLAT ENUM — the value column repeats a code, so re-pointing the NAME alone")
    print("would trade a naming defect for a value COLLISION:")
    print(f"  {len(multikey)} pair(s) / {tables(multikey)} table(s) "
          f"({100 * len(multikey) / max(len(population), 1):.1f}% of the population)")
    all_multikey = [r for r in rows if r["multikey"]]
    print(f"  across the WHOLE minting population: {tables(all_multikey)} of {tables(rows)} tables "
          f"({100 * tables(all_multikey) / max(tables(rows), 1):.1f}%)")

    split = [r for r in rows if r["value_col_is_one_bit"]]
    print("\nVALUE COLUMN IS ONE BIT OF A SPLIT FIELD — a different defect with the same symptom:")
    print(f"  {len(split)} pair(s) / {tables(split)} table(s), "
          f"{sum(1 for r in split if r['replayable'])} replayable pair(s)")

    replayable_collisions = [c for c in collisions if c["replayable"]]
    print("\nALREADY PUBLISHED WITH A COLLIDING CODE — one code on two members of the SAME enum.")
    print("These survive `.5.vii.a` precisely because the cell DOES parse; the column is wrong:")
    print(f"  {len(collisions)} enum(s), of which {len(replayable_collisions)} REPLAYABLE:")
    for record in sorted(replayable_collisions, key=lambda c: -c["colliding"])[:limit]:
        print(f"    {record['document'][:38]:38s} {record['enum'][:14]:14s} "
              f"members={record['members']:<3} colliding={record['colliding']}")

    print(f"\nreplayable population, adjudicated by hand in `.5.xiii` (top {limit}):")
    for record in [r for r in population if r["replayable"]][:limit]:
        print(f"  {record['document'][:34]:34s} {record['table']} {record['enum'][:12]:12s} "
              f"{record['header']!r} -> {record['best_header']!r}  multikey={record['multikey']}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--document")
    parser.add_argument("--limit", type=int, default=12)
    args = parser.parse_args()
    rows, collisions = audit()
    if args.document:
        rows = [r for r in rows if r["document"] == args.document]
        collisions = [c for c in collisions if c["document"] == args.document]
    if args.json:
        json.dump({"rows": rows, "collisions": collisions}, sys.stdout, indent=1, default=str)
        print()
        return 0
    report(rows, collisions, args.limit)
    return 0


if __name__ == "__main__":
    sys.exit(main())
