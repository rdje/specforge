#!/usr/bin/env python3
"""Census: do minted enums take their MEMBERS and VALUES from the right place?

`KG-ISF-COMPLETENESS.5.v`. `.5.i` and `.5.iv.a` gate the enum's NAME — it must be an
independently evidenced candidate, or a column-header reference token of the table. Neither
asks whether the members read afterwards belong to that name, nor where their values came
from. `evidence.rs` mints `Enum <name> <member> = <value>.` and falls back to the ROW INDEX
when no numeric cell parses, so a table stating no code silently acquires `0..n-1`.

Read-only over the persisted corpus. Two independent flags per minted enum:

  MEMBER-COLUMN DRIFT  the enum's name matches one column's header, but its members are the
                       cells of a DIFFERENT column.
  POSITIONAL VALUES    the values are exactly `0..n-1` in mint order AND no column of the
                       source table carries those literals, so they came from `row_idx`.

Both are necessary conditions, not verdicts: a genuine code table may legitimately be
`0..n-1`, and a two-column `name | description` table may legitimately source members from
column 1. The census exists so a rule is written against a measured population rather than
one instance — `.5.i`/`.5.ii`/`.5.iii` each earned their gate that way.

Usage: python3 scripts/measure_enum_member_value_fidelity.py [--json] [--document KEY]
"""
import argparse
import collections
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
ENUM = re.compile(r"^Enum (\S+) (\S+) = (\d+)\.$")


def norm(text):
    """Fold a cell to the shape a minted member name takes: upper, non-alnum -> underscore."""
    return re.sub(r"[^0-9A-Za-z]+", "_", (text or "").strip()).strip("_").upper()


def table_cells(rows):
    return [[cell.get("text", "") for cell in row] for row in rows]


def minted_enums(evidence):
    out = collections.OrderedDict()
    for statement in evidence.get("extracted_statements", []):
        match = ENUM.match(statement.get("text", ""))
        if match:
            out.setdefault(match.group(1), []).append((match.group(2), int(match.group(3))))
    return out


def column_of_header(header_rows, name):
    """The column whose header token equals the enum name, ignoring merged title rows."""
    target = norm(name)
    for row in header_rows:
        # A merged title row repeats one string across every column and names no column.
        if len(set(row)) <= 1 and len(row) > 1:
            continue
        for index, cell in enumerate(row):
            if norm(cell) == target:
                return index
    return None


def column_matching_members(body_rows, members):
    """The column whose cells reproduce the minted member names, in order.

    A SUBSEQUENCE, not a prefix: the minting loop skips members the `.5.ii` sentence-spine and
    `.5.iii` width-parameter gates reject, so a column legitimately carries rows the enum does
    not. Requiring a prefix left 240 of 550 enums with no located source table and made the
    denominator unexplainable; subsequence matching is what the producer actually does.
    """
    wanted = [norm(m) for m, _ in members]
    if not wanted:
        return None
    width = max((len(r) for r in body_rows), default=0)
    for index in range(width):
        column = [norm(r[index]) if index < len(r) else "" for r in body_rows]
        column = [c for c in column if c]
        position = 0
        for cell in column:
            if position < len(wanted) and cell == wanted[position]:
                position += 1
        if position == len(wanted):
            return index
    return None


def values_appear_in_any_column(body_rows, values):
    wanted = {str(v) for v in values}
    width = max((len(r) for r in body_rows), default=0)
    for index in range(width):
        column = {re.sub(r"[^0-9]", "", (r[index] if index < len(r) else "")) for r in body_rows}
        if wanted.issubset({c for c in column if c}):
            return True
    return False


def audit(document, evidence, source):
    findings = []
    tables = source.get("structured_tables", [])
    for name, members in minted_enums(evidence).items():
        values = [v for _, v in members]
        positional_shape = values == list(range(len(values)))
        best = None
        for table in tables:
            header = table_cells(table.get("header_rows", []))
            body = table_cells(table.get("body_rows", []))
            member_col = column_matching_members(body, members)
            if member_col is None:
                continue
            best = (table, header, body, member_col, column_of_header(header, name))
            break
        if best is None:
            findings.append({"document": document, "enum": name, "members": len(members),
                             "source_table": None, "member_column_drift": None,
                             "positional_values": positional_shape and None})
            continue
        table, header, body, member_col, name_col = best
        drift = name_col is not None and name_col != member_col
        positional = positional_shape and not values_appear_in_any_column(body, values)
        findings.append({
            "document": document, "enum": name, "members": len(members),
            "source_table": table.get("table_id"),
            "name_column": name_col, "member_column": member_col,
            "member_column_drift": bool(drift),
            "positional_values": bool(positional),
            "header": header[-1] if header else [],
        })
    return findings


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--document")
    args = parser.parse_args()

    rows = []
    for evidence_path in sorted((ROOT / "generated" / "evidence_ir").glob("*/evidence_ir.json")):
        document = evidence_path.parent.name
        if args.document and document != args.document:
            continue
        evidence = json.loads(evidence_path.read_text(encoding="utf-8"))
        source_path = ROOT / "generated" / "source_ir" / document / "source_ir.json"
        if not source_path.exists():
            continue
        source = json.loads(source_path.read_text(encoding="utf-8"))
        rows.extend(audit(document, evidence, source))

    if args.json:
        json.dump(rows, sys.stdout, indent=1)
        print()
        return 0

    drift = [r for r in rows if r.get("member_column_drift")]
    positional = [r for r in rows if r.get("positional_values")]
    unmatched = [r for r in rows if r.get("source_table") is None]
    both = [r for r in rows if r.get("member_column_drift") and r.get("positional_values")]
    print(f"minted enums:            {len(rows)} across "
          f"{len({r['document'] for r in rows})} document(s)")
    print(f"source table located:    {len(rows) - len(unmatched)}")
    print(f"  member-column drift:   {len(drift)}")
    print(f"  positional values:     {len(positional)}")
    print(f"  both:                  {len(both)}")
    print(f"no source table matched: {len(unmatched)}")
    for row in sorted(drift + [p for p in positional if p not in drift],
                      key=lambda r: (r["document"], r["enum"]))[:40]:
        flags = ",".join(f for f, on in (("drift", row.get("member_column_drift")),
                                         ("positional", row.get("positional_values"))) if on)
        print(f"  {row['document'][:44]:44s} {row['enum'][:22]:22s} "
              f"n={row['members']:<3} name_col={row.get('name_column')} "
              f"member_col={row.get('member_column')} [{flags}]")
    return 0


if __name__ == "__main__":
    sys.exit(main())
