#!/usr/bin/env python3
"""Which encoding tables no record cites minted an enum (EXTRACTION-REPRESENTATION-GAPS.3.c). Read-only.

An `encoding` table that reaches no canonical record is listed in IntentIR `captured_region_residuals`. The
accounting counts a table as carried only when a record names it in `supporting_table_ids`, and a symbol definition
carries no table: its members cite synthetic `Enum X M = v.` statements. So this script attributes each such table
to the enum it minted, by replay, with the calibrated locator the enum instruments already use
(`located_tables`, `KG-ISF-COMPLETENESS.5.xiii.a.7`), and sorts it into:

  * minted, reaches IntentIR  — carried, and reported uncarried only because the provenance is missing;
  * minted, not in IntentIR   — minted at EvidenceIR and withdrawn before IntentIR;
  * minted nothing            — the table states an encoding no enum carries.

Usage: python3 scripts/measure_uncarried_encoding_tables.py
"""

from __future__ import annotations

import collections
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

import adjudicate_enum_member_name_column as enum_instruments  # noqa: E402

GENERATED = ROOT / "generated"


def load(path: Path) -> dict:
    return json.loads(path.read_text())


def main() -> None:
    retained = load(ROOT / "doctrine/chain_currency/retained_bundles.json")["retained"]
    verdicts: collections.Counter = collections.Counter()
    for key in sorted(retained):
        source = load(GENERATED / "source_ir" / key / "source_ir.json")
        evidence = load(GENERATED / "evidence_ir" / key / "evidence_ir.json")
        intent = load(GENERATED / "intent_ir" / key / "intent_ir.json")
        if evidence.get("schema_version") != 3:
            continue
        uncarried = {r["region_id"] for r in intent.get("captured_region_residuals", [])}
        tables = source.get("structured_tables", [])
        encoding = [t for t in tables if t.get("table_kind") == "encoding" and t["table_id"] in uncarried]
        if not encoding:
            continue
        minted = enum_instruments.minted_enums(evidence)
        located = {
            name: enum_instruments.located_tables(tables, name, members, True) or set()
            for name, members in minted.items()
        }
        in_intent = {symbol["symbol_name"] for symbol in intent.get("symbol_definitions", [])}
        for table in encoding:
            names = sorted(name for name, found in located.items() if table["table_id"] in found)
            if not names:
                verdict = "minted nothing"
            elif any(name in in_intent for name in names):
                verdict = "minted, reaches IntentIR"
            else:
                verdict = "minted, not in IntentIR"
            verdicts[verdict] += 1
            caption = (table.get("caption_text") or "").strip()
            print(f"{key}\t{table['table_id']}\t{verdict}\t{','.join(names)}\t{caption}")
    print(f"\nuncarried encoding tables: {sum(verdicts.values())}", file=sys.stderr)
    for verdict, count in verdicts.most_common():
        print(f"  {verdict}: {count}", file=sys.stderr)


if __name__ == "__main__":
    main()
