#!/usr/bin/env python3
"""Measure `KG-ISF-COMPLETENESS.5.xiii.a.3` — constraints read from a list item or a table row, and
whether the scope their lead-in or caption states survives into the record.

`.5.xiii.a` made three AXI constraints visible — `AWBURST must be INCR`, `AWBURST must be WRAP`,
`ARBURST must be INCR` — that hold only under a list lead-in (*"The rules for atomic transactions
are as follows:"*) or a table caption (*"Table A15.23: DVM Complete request constraints"*), and are
published with no `condition_text`. The same table already published `ARADDR must be zero` the same
way. This instrument counts the class before any repair is chosen.

## What it measures, structurally

Every `signal_constraints` record in every persisted `evidence_ir.json` is traced to its first
supporting statement, and that statement's SHAPE is read off its text:

    bullet      the statement opens with a list marker (`-`, `•`, `*`, `–`)
    table_row   the statement is a serialized table row (`| … |`)
    prose       anything else

A bullet or row then has a SCOPE CARRIER when the document states one before it:

    lead_in     the nearest preceding non-bullet statement ends with `:` — a list lead-in
    item_lead   a preceding bullet of the same list ends with `:` — a nested lead-in whose
                sub-items the text layer flattened to the same level
    caption     a table row whose table is introduced by a `Table …` caption statement

The record CARRIES a condition when its `condition_text` is non-empty. No vocabulary decides
anything here: a colon and a caption are layout, so the census is layout, and whether a carrier
actually RESTRICTS (*"for atomic transactions"*) or is neutral (*"the signals are as follows"*) is
decided by hand on the sample `--sample` prints, never by a word list (ADR 0006).

Read-only; no rebuild, no VLM, no Docling. Usage:
  python3 scripts/measure_constraint_scope_carriers.py [--sample N] [--seed S] [--json]
"""
import argparse
import collections
import json
import pathlib
import random
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
BULLET = re.compile(r"^\s*[-•*–]\s")
TABLE_ROW = re.compile(r"^\s*\|.*\|\s*$", re.S)
CAPTION = re.compile(r"^\s*Table\s+[A-Z]?\d")


def shape(text):
    if BULLET.match(text):
        return "bullet"
    if TABLE_ROW.match(text):
        return "table_row"
    return "prose"


def scope_carrier(statements, index):
    """The statement that scopes statement `index`, and how, or (None, None)."""
    kind = shape(statements[index]["text"])
    if kind == "bullet":
        for back in range(index - 1, -1, -1):
            text = statements[back]["text"].strip()
            if shape(text) == "bullet":
                if text.endswith(":"):
                    return "item_lead", text
                continue
            return ("lead_in", text) if text.endswith(":") else (None, None)
    if kind == "table_row":
        for back in range(index - 1, -1, -1):
            text = statements[back]["text"].strip()
            if shape(text) == "table_row":
                continue
            return ("caption", text) if CAPTION.match(text) else (None, None)
    return None, None


def audit():
    records = []
    for path in sorted((ROOT / "generated" / "evidence_ir").glob("*/evidence_ir.json")):
        evidence = json.loads(path.read_text(encoding="utf-8"))
        statements = evidence.get("extracted_statements", [])
        position = {s["statement_id"]: i for i, s in enumerate(statements)}
        for record in evidence.get("signal_constraints", []):
            ids = record.get("supporting_statement_ids") or []
            if not ids or ids[0] not in position:
                continue
            index = position[ids[0]]
            carrier_kind, carrier = scope_carrier(statements, index)
            records.append({
                "document": path.parent.name,
                "replayable": evidence.get("schema_version") == 3,
                "constraint_id": record.get("constraint_id"),
                "subject": record.get("subject_signal"),
                "kind": (record.get("constraint_kind") or {}).get("kind"),
                "value": (record.get("constraint_kind") or {}).get("value"),
                "shape": shape(statements[index]["text"]),
                "carrier_kind": carrier_kind,
                "carrier": carrier,
                "carries_condition": bool((record.get("condition_text") or "").strip()),
                "text": statements[index]["text"].strip(),
            })
    return records


def report(records, sample, seed):
    strata = (("replayable", True), ("legacy", False))
    print(f"signal_constraints traced to a statement: {len(records)}")
    for label, flag in strata:
        subset = [r for r in records if r["replayable"] == flag]
        print(f"\n{label.upper()} ({len(subset)} records)")
        print(f"  {'shape':10s} {'records':>8s} {'scoped':>7s} {'lead_in':>8s} {'item_lead':>10s} "
              f"{'caption':>8s} {'scoped+carries':>15s}")
        for kind in ("bullet", "table_row", "prose"):
            rows = [r for r in subset if r["shape"] == kind]
            scoped = [r for r in rows if r["carrier_kind"]]
            by = collections.Counter(r["carrier_kind"] for r in scoped)
            carries = sum(r["carries_condition"] for r in scoped)
            print(f"  {kind:10s} {len(rows):>8} {len(scoped):>7} {by['lead_in']:>8} "
                  f"{by['item_lead']:>10} {by['caption']:>8} {carries:>15}")
    if sample:
        scoped = [r for r in records if r["carrier_kind"]]
        rng = random.Random(seed)
        print(f"\nHAND SAMPLE — {min(sample, len(scoped))} scoped records (seed {seed}); adjudicate each "
              "carrier as RESTRICTING or NEUTRAL against the document:")
        for r in rng.sample(scoped, min(sample, len(scoped))):
            print(f"  [{r['carrier_kind']}] {r['document'][:28]} {r['subject']} {r['kind']} "
                  f"{r['value'] or ''}  carries_condition={'yes' if r['carries_condition'] else 'no'}")
            print(f"      scope: {r['carrier'][:120]}")
            print(f"      item:  {r['text'][:120]}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--sample", type=int, default=0)
    parser.add_argument("--seed", type=int, default=13)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    records = audit()
    if args.json:
        json.dump(records, sys.stdout, indent=1)
        print()
        return 0
    report(records, args.sample, args.seed)
    return 0


if __name__ == "__main__":
    sys.exit(main())
