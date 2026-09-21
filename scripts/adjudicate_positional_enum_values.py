#!/usr/bin/env python3
"""Adjudicate the positional enum VALUES `KG-ISF-COMPLETENESS.5.v` counted.

`.5.v` flagged **160 minted enums whose values are exactly `0..n-1` and appear in no column of
their source table**, and handed `.5.vii` one instruction: decide between refusing the value and
keeping it, on a measurement rather than an intuition, because *"a positional gate firing on a
real code table would be worse than the residual"*.

`synthesize_encoding_declarations_for_enum` (`crates/specforge/src/ir/evidence.rs`) mints
`Enum <name> <member> = <value>.`, where

    let value: u32 = row.get(value_col)
        .and_then(|cell| parse_encoding_numeric_literal(&cell.text))
        .unwrap_or(row_idx as u32);

so a cell the parser cannot read silently becomes the row's ORDINAL. The document is then quoted
as stating a code it never stated.

## Why this instrument replaces `.5.v`'s shape test

`.5.v` inferred the fallback from the SHAPE of the finished value list (`0..n-1`, absent from every
column). That test is wrong in both directions:

- it MISSES an enum whose fabricated values are not contiguous — `row_idx` is the index into
  `table.body_rows`, while the `.5.ii` spine gate and the `.5.iii` `_WIDTH` gate SKIP rows, so a
  gated table's fabricated values have gaps — and it misses every enum that mixes stated and
  fabricated values, which no shape can distinguish;
- it can FLAG a genuine `0,1,2,…` code table whose document really does state those codes.

This script does not infer. It REPLAYS the producer per table, carrying one extra bit per member —
whether `parse_encoding_numeric_literal` returned a value or the row index was substituted — and
attributes each minted member to the table the producer walked. Provenance is then read off, not
guessed, and a genuine `0..n-1` code table is correctly reported as document-stated.

Attribution is `.5.vi`'s, reused unchanged from `adjudicate_conflated_enum_identity`: the minted
members are segmented into consecutive runs, each an ordered subsequence of one table's replay,
with the tables in increasing order. It declines the 7 enums `.5.vi` could only resolve by set
cover, because a set cover names the tables but not which table produced WHICH member, and this
question is per member.

## What the census reports

    value_col provenance   WHICH branch of `infer_encoding_column_indices` fixed the column whose
                           cell failed to parse — a matched header word, the col-0 literal sniff,
                           or the bare `unwrap_or(0)` default.
    cell shape             what is actually in a header-matched value cell that does not parse.
    corroboration          whether the fabricated number appears ANYWHERE in its own source row.
                           This is the fairness test `.5.v` asked for: if the ordinal is what the
                           document meant, the row should say so somewhere.
    collision              a fabricated value equal to a value the SAME enum states elsewhere. No
                           encoding assigns one code to two members, so a collision is a defect
                           that needs no interpretation — the independently damning signal, as the
                           >60-character member was for `.5.v`.

Read-only; no rebuild, no VLM, no Docling. Usage:
  python3 scripts/adjudicate_positional_enum_values.py [--json] [--document KEY] [--limit N]
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
    VALUE_HEADER_WORDS,
    contains_reference_token,
    header_texts,
    infer_encoding_column_indices,
    looks_like_encoding_literal,
    minted_enums,
    parse_encoding_numeric_literal,
    sanitize_member,
    subsequence_prefix,
)

ROOT = pathlib.Path(__file__).resolve().parent.parent

# Why the value column is where it is. The three arms are the three ways
# `infer_encoding_column_indices` can fix `value_col`, in its own order.
VALUE_COL_ARMS = ("header_match", "literal_sniff_col0", "defaulted_col0")


def value_col_arm(table, enum_name):
    """WHICH arm of `infer_encoding_column_indices` chose `value_col`.

    The function returns only the index, so a caller cannot tell a column the document headed
    `Encoding` from column 0 claimed by `unwrap_or(0)` because nothing matched. The distinction
    decides whether a fabricated value is a parser gap in a real value column or a table that has
    no value column at all.
    """
    lowered_name = enum_name.lower()
    for index, header in enumerate(header_texts(table)):
        if (
            any(word in header for word in VALUE_HEADER_WORDS)
            or contains_reference_token(header, lowered_name)
            or f"{lowered_name}[" in header
        ):
            return index, "header_match"
    for row in table.get("body_rows") or []:
        if row and looks_like_encoding_literal(row[0].get("text")):
            return 0, "literal_sniff_col0"
    return 0, "defaulted_col0"


def replay_table_with_provenance(table, enum_name):
    """`.5.vi`'s `replay_table`, carrying WHERE each value came from.

    The `.5.ii`/`.5.iii` per-member gates stay unreplicated for the same reason `.5.vi` gives:
    they only remove members, so a minted run is a subsequence of the ungated replay.
    """
    name_col, value_col = infer_encoding_column_indices(table, enum_name)
    _, arm = value_col_arm(table, enum_name)
    entries = []
    for row_idx, row in enumerate(table.get("body_rows") or []):
        if name_col >= len(row):
            continue
        raw = (row[name_col].get("text") or "").strip()
        if not raw:
            continue
        member = sanitize_member(raw)
        if not member:
            continue
        value = None
        if value_col < len(row):
            value = parse_encoding_numeric_literal(row[value_col].get("text"))
        entries.append({
            "pair": (member, row_idx if value is None else value),
            "positional": value is None,
            "row_idx": row_idx,
            "row": row,
            "value_col": value_col,
            "arm": arm,
        })
    return entries


def matched_entries(entries, wanted):
    """The entries of one table's replay that match a prefix of `wanted`, in producer order."""
    position, matched = 0, []
    for entry in entries:
        if position < len(wanted) and entry["pair"] == wanted[position]:
            matched.append(entry)
            position += 1
    return matched


def attribute_members(members, replays, restarts_allowed):
    """`.5.vi`'s `attribute_in_table_order`, returning the ENTRY behind each minted member.

    The identity leaf needed only the set of contributing tables; a value-provenance question
    needs the individual row, so the same walk is re-run keeping what it matched.
    """
    position, table_index, restarts, attributed = 0, 0, 0, []
    while position < len(members):
        best = None
        for index in range(table_index, len(replays)):
            reach = subsequence_prefix(
                [entry["pair"] for entry in replays[index][1]], members[position:]
            )
            if reach > (best[1] if best else 0):
                best = (index, reach)
        if best is None or best[1] == 0:
            if restarts < restarts_allowed and table_index > 0:
                table_index, restarts = 0, restarts + 1
                continue
            return None
        table_id, entries = replays[best[0]]
        for entry in matched_entries(entries, members[position:position + best[1]]):
            attributed.append((table_id, entry))
        position += best[1]
        table_index = best[0] + 1
    return attributed


# The shapes a header-matched value cell takes when it does not parse. These are descriptive
# classes for the report, not a production predicate: each is a fact about what the publisher
# wrote, and none of them is proposed as a gate.
CELL_SHAPES = (
    ("empty", lambda t: not t),
    ("dash_or_not_applicable", lambda t: re.fullmatch(r"[-–—]+|n/?a|none", t, re.I) is not None),
    ("bit_range", lambda t: re.fullmatch(r"\[?\s*\d+\s*:\s*\d+\s*\]?", t) is not None),
    ("literal_range", lambda t: re.search(r"(0[bx][0-9a-fx_]+|\b[01]{2,})\s*[-–—]\s*\S", t, re.I)
     is not None),
    ("multi_value", lambda t: "," in t or ";" in t),
    ("dont_care_pattern", lambda t: re.fullmatch(r"[01xX?_\s]*[xX?][01xX?_\s]*", t) is not None),
    ("prose_over_60_chars", lambda t: len(t) > 60),
)


# A literal the document DOES state, wrapped in a quoting convention
# `parse_encoding_numeric_literal` does not strip — it trims `[` and `]` and nothing else. This is a
# PARSER gap, not a fallback defect: the value exists, so the repair is to read it, never to drop
# the member. Measured separately for that reason.
#
# `KG-ISF-COMPLETENESS.5.viii` — CORRECTED, and the correction changes the value rather than the
# count. This mirrored `parse_encoding_value_literal`'s predecessor: strip the wrapper, re-parse.
# That is wrong, because `parse_encoding_numeric_literal` tries `parse::<u32>()` FIRST, so the
# stripped `'1110'` reads as decimal one thousand one hundred ten and its all-`0`/`1` binary arm is
# unreachable below eleven digits. The documents state the radix outright — *"Binary values in
# sentences are shown in single quotation marks. For example '1010'."* and *"Hexadecimal values are
# preceded by an x and enclosed in single quotation marks. For example x'0A00'."* — so the wrapper
# IS the radix and `'1110'` is 14. The `x'...'` half was additionally invisible to the old regex,
# which required a quote as the first character; it is counted here.
QUOTED_BINARY = re.compile(r"^'([01][01 _]*)'$")
QUOTED_HEX = re.compile(r"^[xX]\s*'([0-9a-fA-F][0-9a-fA-F _]*)'$")


def quote_wrapped_literal(text):
    """The value a quote-wrapped cell states, read in the radix its wrapper declares, else `None`.

    Mirrors `parse_encoding_value_literal` (`crates/specforge/src/ir/evidence.rs`) exactly; the two
    are cross-checked over every distinct quoted cell in the persisted corpus by `.5.viii`.
    """
    trimmed = (text or "").strip()
    for pattern, radix in ((QUOTED_BINARY, 2), (QUOTED_HEX, 16)):
        match = pattern.match(trimmed)
        if not match:
            continue
        compact = match.group(1).replace(" ", "").replace("_", "")
        value = int(compact, radix)
        return value if value <= 0xFFFFFFFF else None
    return None


def cell_shape(text):
    trimmed = (text or "").strip()
    for label, predicate in CELL_SHAPES:
        if predicate(trimmed):
            return label
    return "other_words"


def audit_document(document, evidence, source, promoted):
    # `CORPUS-CHAIN-CURRENCY.11`: a persisted LEGACY `evidence_ir.json` is evidence about ITSELF,
    # never about the reader. 51 of the 78 documents cannot be replayed at all (their normalized
    # bundle is reclaimed), so the enums they carry were minted by binaries that no longer exist.
    # Every count below is therefore split by stratum, because the unsplit total is a fact about the
    # corpus on disk and the split one is a fact about the producer.
    replayable = evidence.get("schema_version") == 3
    tables = source.get("structured_tables", [])
    findings = []
    for name, members in minted_enums(evidence).items():
        replays = []
        for table in tables:
            entries = replay_table_with_provenance(table, name)
            if {entry["pair"] for entry in entries} & set(members):
                replays.append((table.get("table_id"), entries))
        attributed = (attribute_members(members, replays, 0)
                      or attribute_members(members, replays, 3))
        if attributed is None or len(attributed) != len(members):
            findings.append({
                "document": document, "enum": name, "members": len(members),
                "attributed": False, "promoted": name in promoted,
                "replayable": replayable,
            })
            continue

        flags = [entry["positional"] for _, entry in attributed]
        stated_values = {value for (_, value), positional in zip(members, flags) if not positional}
        # The instrument's own residual. Attribution picks ONE table per run, but a minted pair can be
        # producible by several — and when they disagree on parsed-versus-ordinal, the ordered walk can
        # pick the wrong one. AHB `HREADYOUT` is the measured case: its members are the bare digits `0`
        # and `1`, seven tables in that document can mint `('0', 0)` / `('1', 1)`, and the walk chose the
        # one where those cells parse. The producer walks the one where they do not, so the census
        # UNDER-counted it by two. The error direction is therefore a floor, not a band around the
        # headline, and this count is reported rather than left for the next reader to rediscover.
        offered = collections.defaultdict(set)
        for table in tables:
            for entry in replay_table_with_provenance(table, name):
                if entry["pair"] in set(members):
                    offered[entry["pair"]].add(entry["positional"])
        ambiguous = sum(1 for pair in members if offered.get(pair) == {True, False})
        arms = collections.Counter()
        shapes = collections.Counter()
        corroborated = 0
        quote_wrapped = 0
        for (_, value), (_, entry) in zip(members, attributed):
            if not entry["positional"]:
                continue
            arms[entry["arm"]] += 1
            cells = [(cell.get("text") or "") for cell in entry["row"]]
            if entry["arm"] == "header_match" and entry["value_col"] < len(cells):
                shapes[cell_shape(cells[entry["value_col"]])] += 1
            if (entry["value_col"] < len(cells)
                    and quote_wrapped_literal(cells[entry["value_col"]]) is not None):
                quote_wrapped += 1
            # The fairness test: does the document's own row carry this number anywhere?
            if any(parse_encoding_numeric_literal(cell) == value for cell in cells):
                corroborated += 1
        findings.append({
            "document": document, "enum": name, "members": len(members), "attributed": True,
            "fabricated": sum(flags),
            "collisions": sum(1 for (_, value), positional in zip(members, flags)
                              if positional and value in stated_values),
            "corroborated": corroborated,
            "quote_wrapped": quote_wrapped,
            "ambiguous_provenance": ambiguous,
            "arms": dict(arms),
            "cell_shapes": dict(shapes),
            "tables": len({table_id for table_id, _ in attributed}),
            "promoted": name in promoted,
            "replayable": replayable,
        })
    return findings


def load_rows(document_filter):
    rows = []
    for evidence_path in sorted((ROOT / "generated" / "evidence_ir").glob("*/evidence_ir.json")):
        document = evidence_path.parent.name
        if document_filter and document != document_filter:
            continue
        source_path = ROOT / "generated" / "source_ir" / document / "source_ir.json"
        if not source_path.exists():
            continue
        intent_path = ROOT / "generated" / "intent_ir" / document / "intent_ir.json"
        promoted = set()
        if intent_path.exists():
            intent = json.loads(intent_path.read_text(encoding="utf-8"))
            promoted = {symbol.get("symbol_name")
                        for symbol in intent.get("symbol_definitions", [])}
        rows.extend(audit_document(
            document,
            json.loads(evidence_path.read_text(encoding="utf-8")),
            json.loads(source_path.read_text(encoding="utf-8")),
            promoted,
        ))
    return rows


def report(rows, limit):
    attributed = [row for row in rows if row["attributed"]]
    declined = [row for row in rows if not row["attributed"]]
    members = sum(row["members"] for row in attributed)
    fabricated = sum(row["fabricated"] for row in attributed)
    print(f"minted enums:            {len(rows)} across "
          f"{len({row['document'] for row in rows})} document(s)")
    print(f"  attributed per member: {len(attributed)}")
    print(f"  declined (set cover):  {len(declined)}  "
          f"— `.5.vi` resolves these by cover, which names tables but not rows")
    print(f"attributed members:      {members}")
    print(f"  value from the document: {members - fabricated}")
    print(f"  value from `row_idx`:    {fabricated}  "
          f"({100 * fabricated / members:.1f}% of every minted value)")

    print("\nSTRATUM — `CORPUS-CHAIN-CURRENCY.11`: a legacy artifact is evidence about ITSELF,")
    print("never about the reader. 51 of 78 documents cannot be replayed, so the enums they carry")
    print("were minted by binaries that no longer exist:")
    print(f"  {'stratum':34s} {'enums':>6s} {'members':>9s} {'fabricated':>11s}")
    for label, want in (("replayable (the CURRENT producer)", True),
                        ("legacy (artifact-only evidence)", False)):
        subset = [r for r in rows if r.get("replayable") is want]
        done = [r for r in subset if r["attributed"]]
        print(f"  {label:34s} {len(subset):6d} {sum(r['members'] for r in subset):9d} "
              f"{sum(r['fabricated'] for r in done):11d}")
    print("  Read the totals above and below as facts about the CORPUS ON DISK. Only the replayable")
    print("  row is a fact about what SpecForge does today.")

    print("\nper enum — how much of its value surface the document actually states:")
    classes = (
        ("every value fabricated", lambda r: r["members"] and r["fabricated"] == r["members"]),
        ("some values fabricated", lambda r: 0 < r["fabricated"] < r["members"]),
        ("every value document-stated", lambda r: r["fabricated"] == 0),
    )
    print(f"  {'class':30s} {'enums':>6s} {'members':>9s} {'fabricated':>11s} {'promoted':>9s}")
    for label, predicate in classes:
        subset = [row for row in attributed if predicate(row)]
        print(f"  {label:30s} {len(subset):6d} {sum(r['members'] for r in subset):9d} "
              f"{sum(r['fabricated'] for r in subset):11d} "
              f"{sum(1 for r in subset if r['promoted']):9d}")

    arms = collections.Counter()
    for row in attributed:
        arms.update(row["arms"])
    print("\nwhy the column whose cell failed to parse was chosen as the value column:")
    for arm in VALUE_COL_ARMS:
        print(f"  {arm:24s} {arms[arm]:6d}  ({100 * arms[arm] / max(fabricated, 1):4.1f}%)")

    shapes = collections.Counter()
    for row in attributed:
        shapes.update(row["cell_shapes"])
    total_shapes = sum(shapes.values())
    print(f"\nwhat is in a HEADER-MATCHED value cell that does not parse ({total_shapes} cells):")
    for shape, count in shapes.most_common():
        print(f"  {shape:24s} {count:6d}  ({100 * count / max(total_shapes, 1):4.1f}%)")

    quote_wrapped = sum(row["quote_wrapped"] for row in attributed)
    quote_docs = {row["document"] for row in attributed if row["quote_wrapped"]}
    print("\nRECOVERABLE — the cell IS a literal, in a quoting convention the parser never strips:")
    print(f"  {quote_wrapped} of {fabricated} fabricated values "
          f"({100 * quote_wrapped / max(fabricated, 1):.1f}%) in {len(quote_docs)} document(s).")
    print("  These are not residuals. The document states the code, so the repair is to READ it —")
    print("  CLOSED by `.5.viii`, which reads it in the radix the documents themselves declare")
    print("  (*\"Binary values in sentences are shown in single quotation marks\"*; `x'...'` is hex).")
    print("  `.5.vii` prescribed stripping the wrapper and re-parsing, and that is WRONG on 314 of")
    print("  these 357 slots: the shared reader tries `parse::<u32>()` first, so a stripped `'1110'`")
    print("  reads as decimal 1110 where the document states 14 — and on 57 slots it is strictly")
    print("  WORSE than the ordinal it replaces. A count-based bar (`quote-wrapped goes to 0`) cannot")
    print("  tell the two repairs apart; only the VALUES can. The reading is confined to the member")
    print("  seam: `looks_like_encoding_literal` would move `value_col`, hence `name_col`, hence the")
    print("  members, and `derive_header_sourced_enum_name` would mint new enums — the")
    print("  non-subtractive trap `.5.vi.a` was reverted for. `.5.x` owns that clause.")

    ambiguous = sum(row["ambiguous_provenance"] for row in attributed)
    amb_enums = sum(1 for row in attributed if row["ambiguous_provenance"])
    print("\nINSTRUMENT RESIDUAL — minted pairs several tables could produce, disagreeing on origin:")
    print(f"  {ambiguous} of {members} member slots ({100 * ambiguous / max(members, 1):.1f}%) "
          f"in {amb_enums} enums.")
    print("  The ordered walk picks one. AHB `HREADYOUT` is the measured miss: members named `0` and")
    print("  `1`, seven candidate tables, and the walk chose one where those cells parse while the")
    print("  producer walks one where they do not. The direction is an UNDER-count, so the fabricated")
    print("  total above is a FLOOR.")

    corroborated = sum(row["corroborated"] for row in attributed)
    print("\nFAIRNESS — is the ordinal what the document meant? (`.5.v`'s standing warning)")
    print(f"  the number appears somewhere in its own source row: {corroborated:6d}  "
          f"({100 * corroborated / max(fabricated, 1):4.1f}%)")
    print(f"  the number appears nowhere in that row at all:      "
          f"{fabricated - corroborated:6d}  "
          f"({100 * (fabricated - corroborated) / max(fabricated, 1):4.1f}%)")

    collisions = [row for row in attributed if row["collisions"]]
    print("\nPROVABLY WRONG — a fabricated value equal to one the SAME enum states elsewhere")
    print("(no encoding assigns one code to two members, so this needs no interpretation):")
    print(f"  member slots: {sum(row['collisions'] for row in collisions)} "
          f"in {len(collisions)} enums "
          f"({sum(1 for row in collisions if row['promoted'])} promoted)")
    for width, label in ((1, "single-table enums (this leaf's defect alone)"),
                         (2, "multi-table enums (compounded by `.5.vi` conflation)")):
        subset = [row for row in collisions
                  if (row["tables"] == 1 if width == 1 else row["tables"] > 1)]
        print(f"    {label:48s} {sum(r['collisions'] for r in subset):6d} slots "
              f"in {len(subset)} enums")

    print("\nIF THE FALLBACK WERE REMOVED (a member whose value does not parse is not minted):")
    emptied = [row for row in attributed
               if row["members"] and row["fabricated"] == row["members"]]
    narrowed = [row for row in attributed if 0 < row["fabricated"] < row["members"]]
    print(f"  enum statements removed: {fabricated}")
    print(f"  enums emptied entirely:  {len(emptied)}  "
          f"({sum(1 for row in emptied if row['promoted'])} promoted today)")
    print(f"  enums narrowed:          {len(narrowed)}  "
          f"({sum(1 for row in narrowed if row['promoted'])} promoted today)")
    print("  the rule is purely SUBTRACTIVE at the member seam: it can only skip a row, so it")
    print("  cannot rename an enum, change which signal wins, or mint one. `.5.vi.a` was caught")
    print("  by the opposite, and a landing slice must still prove this by replay.")

    print(f"\nworst enums by fabricated members (top {limit}):")
    for row in sorted(attributed, key=lambda r: -r["fabricated"])[:limit]:
        print(f"  {row['document'][:40]:40s} {row['enum'][:18]:18s} "
              f"members={row['members']:<5} fabricated={row['fabricated']:<5} "
              f"collisions={row['collisions']:<4} tables={row['tables']:<3} "
              f"promoted={int(row['promoted'])}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--document")
    parser.add_argument("--limit", type=int, default=25)
    args = parser.parse_args()

    rows = load_rows(args.document)
    if args.json:
        json.dump(rows, sys.stdout, indent=1, default=str)
        print()
        return 0
    report(rows, args.limit)
    return 0


if __name__ == "__main__":
    sys.exit(main())
