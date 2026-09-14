#!/usr/bin/env python3
"""Classify what a DROPPED declaration row actually offers (read-only).

`SIGNAL-DECLARATION-ROW-DROP.1d` — the reader's `(direction, width)` arm discards a row that
offers neither, and that arm is the whole of this tree's measured loss. Every `.2*` leaf so far
has answered it by teaching the reader to READ one more attribute: a flow arrow (`.2b`), a
rotated column (`.2a`/`.2e`), a bit range (`.2f`). `SIGNAL-CATALOG-CAPTURE-GAP.6` found a class
with nothing to read — a check-signal relationship table states that `PACTIVE` is checked by
`PACTIVECHK` and gives no direction and no width, because the fact it states is a RELATION that
presupposes both signals.

So the question is whether a declaration must carry an ATTRIBUTE to carry an IDENTITY, and the
first thing it needs is a classification of what the dropped rows offer instead.

**The population is the reader's own accounting, not a scan.** `.1` made the drop countable at
runtime (`extraction_manifest.declaration_row_accounting`), so a dropped row here is one the real
reader really dropped, with the reason it recorded. That accounting exists only in documents
rebuilt since `.1` shipped — **4** of the 27 proof-carrying documents — and this script reports
that boundary rather than papering over it. `.0`'s corpus-wide figure of 482 dropped rows comes
from a direct scan of all 78 persisted artifacts and is NOT classifiable this way until more
documents carry the accounting; the two numbers are never mixed.

Each dropped row is joined back to its source row by name cell within its own table, and
classified by what its OTHER cells hold — shape only, no document, vendor, protocol or English
vocabulary (ADR 0006):

  * `names-another-signal` — some other cell is a lone identifier: the relationship shape
  * `bit-range`           — some other cell is `[hi:lo]`
  * `enumerated`          — some other cell is a comma-separated list of numbers
  * `arrow`               — some other cell carries a flow arrow
  * `prose`               — some other cell has two or more words carrying letters
  * `nothing`             — no other cell offers anything

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.

Usage:
    python3 scripts/measure_dropped_row_offerings.py
    python3 scripts/measure_dropped_row_offerings.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from measure_parametric_width_cell_shapes import (  # noqa: E402
    header_texts,
    is_hardware_signal_token,
    repo_root,
    signal_names_in_name_cell,
    trim_to_identifier_characters,
)

# Right-flowing and disqualifying arrow spellings, mirroring FLOW_ARROW_FORMS in
# crates/specforge/src/ir/evidence.rs.
ARROWS = ("⟶", "⇒", "→", "==>", "-->", "=>", "->", "⟵", "⇐", "←", "↔", "⇔", "<-", "<=")
BIT_RANGE = re.compile(r"\[\s*\d+\s*:\s*\d+\s*\]")
ENUMERATED = re.compile(r"^\s*\d+\s*(,\s*\d+\s*)+$")


def cell_offering(text: str) -> str | None:
    """What one non-name cell offers, or None."""
    trimmed = text.strip()
    if not trimmed:
        return None
    if any(arrow in trimmed for arrow in ARROWS):
        return "arrow"
    if BIT_RANGE.search(trimmed):
        return "bit-range"
    if ENUMERATED.match(trimmed):
        return "enumerated"
    tokens = trimmed.split()
    if len(tokens) == 1:
        token = trim_to_identifier_characters(tokens[0])
        if is_hardware_signal_token(token) and any(c.isalpha() for c in token):
            return "names-another-signal"
        return None
    if sum(1 for t in tokens if any(c.isalpha() for c in t)) >= 2:
        return "prose"
    return None


# Strongest first: a cell that names a signal is the finding this leaf exists for, and a row
# offering several things is reported by the strongest, with the full set kept for adjudication.
PRECEDENCE = ("names-another-signal", "bit-range", "enumerated", "arrow", "prose")


def census(root: str) -> dict:
    rows = []
    documents = 0
    for path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        accounting = (evidence.get("extraction_manifest") or {}).get(
            "declaration_row_accounting"
        )
        if not accounting:
            continue
        documents += 1
        source_path = os.path.join(root, "generated/source_ir", document, "source_ir.json")
        if not os.path.exists(source_path):
            continue
        with open(source_path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        tables = {t.get("table_id"): t for t in source.get("structured_tables") or []}
        for entry in accounting:
            table = tables.get(entry.get("table_id"))
            if table is None:
                continue
            headers = header_texts(table)
            for dropped in entry.get("dropped_rows") or []:
                name_cell = (dropped.get("name_cell") or "").strip()
                reason = dropped.get("reason")
                match = None
                for row in table.get("body_rows") or []:
                    if any(cell["text"].strip() == name_cell for cell in row):
                        match = row
                        break
                offerings = []
                if match is not None:
                    for cell in match:
                        if cell["text"].strip() == name_cell:
                            continue
                        offering = cell_offering(cell["text"])
                        if offering:
                            offerings.append(offering)
                strongest = next(
                    (kind for kind in PRECEDENCE if kind in offerings), "nothing"
                )
                rows.append(
                    {
                        "document": document,
                        "table_id": entry.get("table_id"),
                        "caption": (table.get("caption_text") or "")[:60],
                        "headers": headers,
                        "name_cell": name_cell,
                        "reason": reason,
                        "joined": match is not None,
                        "offers": strongest,
                        "all_offers": sorted(set(offerings)),
                        "cells": [c["text"][:30] for c in match] if match else [],
                    }
                )
    return {"documents_with_accounting": documents, "rows": rows}


def render(result: dict) -> None:
    rows = result["rows"]
    print("=== dropped declaration rows, by what the row OFFERS (read-only) ===")
    print(f"  documents carrying the reader's own accounting: {result['documents_with_accounting']}")
    print(f"  dropped rows recorded                         : {len(rows)}")
    joined = sum(1 for r in rows if r["joined"])
    rate = f"{100.0 * joined / len(rows):.1f}%" if rows else "n/a"
    print(f"  joined back to their source row               : {joined}  ({rate})")
    print("  NOTE: this is the reader's accounting, which only rebuilt documents carry.")
    print("        `.0`'s corpus-wide 482 comes from a scan of all 78 artifacts and is a")
    print("        different population; the two are never summed.")
    by_reason = collections.Counter(r["reason"] for r in rows)
    print("  by recorded reason:")
    for reason, count in by_reason.most_common():
        print(f"    {count:5d}  {reason}")
    print()
    for reason in [r for r, _ in by_reason.most_common()]:
        subset = [r for r in rows if r["reason"] == reason]
        by_offer = collections.Counter(r["offers"] for r in subset)
        print(f"== {reason}: {len(subset)} rows")
        for offer, count in by_offer.most_common():
            print(f"     {count:5d}  {offer}")
        for offer, _ in by_offer.most_common():
            print(f"   -- {offer} --")
            seen = set()
            for entry in subset:
                if entry["offers"] != offer:
                    continue
                key = (entry["document"], entry["table_id"], entry["name_cell"])
                if key in seen:
                    continue
                seen.add(key)
                print(f"      {entry['document'][:26]:26s} {entry['table_id']:11s} "
                      f"name={entry['name_cell']!r} offers={entry['all_offers']}")
                print(f"         hdr={entry['headers']}")
                print(f"         row={entry['cells']}")
        print()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    result = census(repo_root())
    if args.json:
        json.dump(result, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
