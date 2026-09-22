#!/usr/bin/env python3
"""Adjudicate the cross-table-conflated enums `KG-ISF-COMPLETENESS.5.v` counted.

`.5.v` measured 231 of 550 minted enums with **no single source table**, and left the
adjudication to `.5.vi` with an explicit warning: the obvious rule — refuse an enum whose
members span tables — must first be tested against the legitimate case, **a specification that
splits one code table across a page break**. This script answers that question per item, and it
answers a second one the census could not: **where did the name come from?**

`derive_encoding_enum_name` builds `identity_context = caption + section_title + header` and
names the table after any declared signal referenced anywhere in that blob. A **section title is
shared by every table in the section**, so a single heading can name a dozen unrelated tables the
same thing, and `build_symbol_definitions` then merges them by name. A caption or a header, by
contrast, belongs to the one table. That distinction is structural and document-grounded
(ADR 0006, no name lists), so it is the discriminator this adjudication measures.

## Instrument

For each minted enum the script REPLAYS the producer per table — `infer_encoding_column_indices`
plus the member/value synthesis of `synthesize_encoding_declarations_for_enum` — and asks which
tables can produce which of its members. The two per-member gates (`.5.ii` sentence-spine,
`.5.iii` `_WIDTH` leak) are deliberately NOT replicated: they only ever REMOVE members, so a
minted run is a subsequence of the ungated replay and matching against the ungated sequence is
both sufficient and immune to gate drift.

Attribution reconstructs the producer's walk: the minted members are segmented into consecutive
runs, each an ordered subsequence of one table's replay, with the tables in increasing order. A
cover of size 1 means the census's subsequence matcher missed a table the producer really used; a
cover of size >1 is a genuine conflation, and the script then asks whether the document ITSELF
says those fragments are one table (`Table B2.2 Continued from previous page`, the marker
`PDF-VARIANT-DIGESTION.12a` already trusts), whether only the page layout suggests it, or whether
they are plainly separate tables that merely share a name source.

Read-only. Usage:
  python3 scripts/adjudicate_conflated_enum_identity.py [--json] [--document KEY] [--limit N]
"""
import argparse
import collections
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
ENUM = re.compile(r"^Enum (\S+) (\S+) = (\d+)\.$")
VERDICTS = (
    "single_table",
    "stated_continuation",
    "adjacent_same_shape",
    "distinct_tables",
    "unattributed",
)
MULTI_TABLE_VERDICTS = VERDICTS[1:4]
U32_MAX = 0xFFFFFFFF

NAME_HEADER_WORDS = ("name", "meaning", "description", "state", "transfer", "response", "type")
VALUE_HEADER_WORDS = ("value", "encoding", "code", "binary", "hex", "bit")


def contains_reference_token(text, token):
    """`evidence.rs::contains_reference_token` — `token` occurs on ASCII-alphanumeric boundaries."""
    if not token:
        return False
    start = 0
    while True:
        index = text.find(token, start)
        if index < 0:
            return False
        before = text[index - 1] if index > 0 else ""
        after_index = index + len(token)
        after = text[after_index] if after_index < len(text) else ""
        if not before.isalnum() and not after.isalnum():
            return True
        start = index + 1


def parse_encoding_numeric_literal(text):
    """`evidence.rs::parse_encoding_numeric_literal`, including its u32 overflow behaviour."""
    trimmed = (text or "").strip().strip("[]")
    if not trimmed:
        return None

    def bounded(value):
        return value if 0 <= value <= U32_MAX else None

    if re.fullmatch(r"[0-9]+", trimmed):
        return bounded(int(trimmed))
    lowered = trimmed.lower()
    for prefix, base in (("0b", 2), ("0x", 16)):
        if lowered.startswith(prefix):
            body = lowered[len(prefix):]
            try:
                return bounded(int(body, base))
            except ValueError:
                return None
    for marker, base in (("'b", 2), ("'h", 16)):
        if marker in lowered:
            body = lowered.split(marker, 1)[1]
            try:
                return bounded(int(body, base))
            except ValueError:
                return None
    if lowered and all(c in "01" for c in lowered):
        try:
            return bounded(int(lowered, 2))
        except ValueError:
            return None
    return None


def looks_like_encoding_literal(text):
    """`evidence.rs::looks_like_encoding_literal`."""
    lowered = (text or "").strip().lower().replace(" ", "")
    if not lowered:
        return False
    if lowered.startswith("0b") or lowered.startswith("0x") or "'b" in lowered or "'h" in lowered:
        return True
    return all(c in "01xz_?" for c in lowered) and any(c in "01" for c in lowered)


def header_texts(table):
    rows = table.get("header_rows") or []
    if not rows:
        return []
    return [(cell.get("text") or "").lower() for cell in rows[0]]


def infer_encoding_column_indices(table, enum_name):
    """`evidence.rs::infer_encoding_column_indices`."""
    headers = header_texts(table)
    lowered_name = enum_name.lower()

    name_col = next(
        (i for i, h in enumerate(headers) if any(w in h for w in NAME_HEADER_WORDS)), None
    )
    value_col = next(
        (
            i
            for i, h in enumerate(headers)
            if any(w in h for w in VALUE_HEADER_WORDS)
            or contains_reference_token(h, lowered_name)
            or f"{lowered_name}[" in h
        ),
        None,
    )
    if value_col is None:
        for row in table.get("body_rows") or []:
            if row and looks_like_encoding_literal(row[0].get("text")):
                value_col = 0
                break
    if value_col is None:
        value_col = 0
    if name_col is None or name_col == value_col:
        name_col = next(
            (i for i in range(int(table.get("col_count") or 0)) if i != value_col), None
        )
    return (0 if name_col is None else name_col, value_col)


def sanitize_member(raw):
    """`synthesize_encoding_declarations_for_enum`'s member sanitizer."""
    kept = "".join(c.upper() if (c.isascii() and (c.isalnum() or c == "_")) else "_" for c in raw)
    return kept.strip("_")


def producer_name_column(table, enum_name, current_producer):
    """`(name_col, value_col)` as the producer that MINTED the artifact chose them, or `None`.

    `KG-ISF-COMPLETENESS.5.xiii.a` made the member name column a per-table adjudication
    (`adjudicate_member_name_column`), and a multi-key table now mints nothing. A legacy artifact
    was minted by the header rule and must be replayed by it; a current one by the landed rule.
    Replaying either stratum with the other's rule stops matching its members, which is how the
    instruments sharing this replay lost AXI's re-named tables the moment its chain was rebuilt.
    """
    name_col, value_col = infer_encoding_column_indices(table, enum_name)
    if not current_producer:
        return name_col, value_col
    # Imported late: that module imports this one.
    from adjudicate_enum_member_name_column import landed_verdict
    verdict, column = landed_verdict(table, name_col, value_col)
    return None if verdict == "multi_key" else (column, value_col)


def replay_table(table, enum_name, current_producer=False):
    """The `(member, value)` sequence the producer would mint for this table under this name.

    The `.5.ii`/`.5.iii` per-member gates are intentionally omitted — they only remove members,
    so every genuinely minted run is a subsequence of this list. `current_producer` selects the
    member-name rule of the binary that minted the artifact (`producer_name_column`).
    """
    columns = producer_name_column(table, enum_name, current_producer)
    if columns is None:
        return []
    name_col, value_col = columns
    out = []
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
        out.append((member, row_idx if value is None else value))
    return out


def minted_enums(evidence):
    out = collections.OrderedDict()
    for statement in evidence.get("extracted_statements", []):
        match = ENUM.match(statement.get("text", ""))
        if match:
            out.setdefault(match.group(1), []).append((match.group(2), int(match.group(3))))
    return out


# `Table 2-1`, `Table A7.4:`, `Figure 8.3.`, `Chapter 4` — a label word followed by a numbering token.
# The token may open with a volume/annex letter (`A7.4`, `B13.12`), so requiring it to start with a
# digit silently misses every Arm-style caption; it must merely CONTAIN one, or the label word is
# read as caption prose and the numbering furniture is mistaken for table identity.
CAPTION_LABEL = re.compile(
    r"^\s*([A-Za-z]+)\.?\s+([A-Za-z]{0,2}[0-9]+(?:[-–.][0-9A-Za-z]+)*)[.:)]?\s*"
)
TRAILING_CONTINUED = re.compile(r"\(\s*continued\s*\)\s*$", re.IGNORECASE)


def split_caption_label(caption):
    """Separate a caption's LABEL (`Table 2-1`, `Figure 8.3`) from its descriptive text.

    The label is document furniture: every table in the document carries the same leading word,
    so a name taken from it says nothing about this table. Splitting on `<word> <number>` is
    document grammar, not a vocabulary list (ADR 0006) — the word is whatever this publisher uses.
    """
    text = caption or ""
    match = CAPTION_LABEL.match(text)
    if not match:
        return "", text
    return text[: match.end()], text[match.end():]


def name_source(table, section_title, enum_name):
    """Which part of `identity_context` carries the enum name for THIS table.

    `derive_encoding_enum_name` searches caption + section title + header as one blob. Only two of
    those four parts are facts about the table itself:

    - `caption_body` and `header` — this table's own identity;
    - `caption_label` — the publisher's numbering furniture, identical across the document;
    - `section` — the heading, shared by every table under it.
    """
    lowered = enum_name.lower()
    label, body = split_caption_label(table.get("caption_text") or "")
    header_blob = " ".join(
        (cell.get("text") or "")
        for row in (table.get("header_rows") or [])
        for cell in row
    ).lower()

    def carries(text):
        text = text.lower()
        return contains_reference_token(text, lowered) or f"{lowered}[" in text

    sources = []
    if carries(label):
        sources.append("caption_label")
    if carries(body):
        sources.append("caption_body")
    if carries(header_blob):
        sources.append("header")
    if carries(section_title or ""):
        sources.append("section")
    return sources


TABLE_OWNED_SOURCES = ("caption_body", "header")


def page_number(page_id):
    match = re.search(r"(\d+)", page_id or "")
    return int(match.group(1)) if match else None


def section_title_for_page(sections_by_page, page):
    best = None
    for section_page in sorted(sections_by_page):
        if page is not None and section_page <= page:
            best = sections_by_page[section_page]
        else:
            break
    return best or ""


def subsequence_prefix(replay, wanted):
    """How many leading entries of `wanted` appear, in order, within `replay`.

    Greedy matching is optimal for longest-subsequence-prefix, and it is what the producer does:
    it walks the table's rows once and emits the members its gates do not reject.
    """
    position = 0
    for pair in replay:
        if position < len(wanted) and pair == wanted[position]:
            position += 1
    return position


def attribute_in_table_order(members, replays, restarts_allowed):
    """Segment the minted members into runs, each an ordered subsequence of ONE table's replay.

    The producer walks the document's tables once per convergence pass, so the tables backing a
    minted enum appear in increasing table order within a pass. `restarts_allowed` models the
    extra passes: the scan may return to the start of the table list that many times.
    """
    position = 0
    table_index = 0
    restarts = 0
    chosen = []
    while position < len(members):
        best = None
        for index in range(table_index, len(replays)):
            matched = subsequence_prefix(replays[index][1], members[position:])
            if matched > (best[1] if best else 0):
                best = (index, matched)
        if best is None or best[1] == 0:
            if restarts < restarts_allowed and table_index > 0:
                table_index = 0
                restarts += 1
                continue
            return None
        chosen.append(replays[best[0]][0])
        position += best[1]
        table_index = best[0] + 1
    return chosen


def minimal_cover(members, contributions):
    """Fewest tables whose replayed pairs account for every minted member (greedy, ties by order)."""
    remaining = set(members)
    chosen = []
    pool = dict(contributions)
    while remaining:
        best_key, best_gain = None, 0
        for key, pairs in pool.items():
            gain = len(remaining & pairs)
            if gain > best_gain:
                best_key, best_gain = key, gain
        if best_key is None:
            break
        chosen.append(best_key)
        remaining -= pool.pop(best_key)
    return chosen, remaining


def caption_reference(caption):
    """The caption's table reference and whether it declares itself a continuation.

    `evidence.rs::caption_table_ref` reads exactly this — `Table B2.2 Continued from previous page`
    — and `PDF-VARIANT-DIGESTION.12a` already trusts it in production to rejoin a split table. It
    is the document's OWN statement that two fragments are one table, which is the only evidence
    that settles the question; a shared header shape does not.
    """
    match = CAPTION_LABEL.match(caption or "")
    if not match:
        return None, False
    # Build the reference from the captured groups, never from the matched slice: the slice keeps
    # whatever punctuation followed the number, so `Table A7.4: ARSNOOP encodings` and
    # `Table A7.4 Continued from previous page` would not compare equal and the one pairing the
    # document actually states would be missed.
    reference = f"{match.group(1)} {match.group(2)}".lower()
    # Two dialects, and reading only one of them undercounts the legitimate case. Arm writes the
    # marker right after the label (`Table B2.2 Continued from previous page`, 209 captions in 7
    # documents); most other publishers put it at the end (`Table 2-12 Configurable options …
    # (continued)`, 112 captions in 20 documents, only 1 caption carrying both).
    words = (caption or "").split()
    continued = (
        any(word.strip(".,()").lower() == "continued" for word in words[1:4])
        or bool(TRAILING_CONTINUED.search(caption or ""))
    )
    return reference, continued


def continuation_class(tables):
    """`stated` when the document says these fragments are one table; `adjacent` when only the
    layout suggests it; `None` when they are plainly separate tables.

    The distinction matters because the two are not the same finding. A document-stated
    continuation is the legitimate case `.5.vi` was told to protect. Identical headers on
    consecutive pages are NOT that: a register-field chapter emits `Field | Description | Access |
    Reset` once per register, so the shape repeats for tables describing entirely different
    registers.
    """
    if len(tables) < 2:
        return None
    references = [caption_reference(t.get("caption_text")) for t in tables]
    named = [ref for ref, _ in references if ref]
    # `stated` requires the document to SAY it, not merely to file the tables under one reference.
    # Three register tables captioned `Chapter B15. …` share a chapter, and a chapter is not a
    # table; only the `Continued` marker is the statement, and the shared reference then says which
    # table it continues.
    if (
        any(continued for _, continued in references)
        and len(named) == len(tables)
        and len(set(named)) == 1
    ):
        return "stated"

    headers = [tuple(header_texts(t)) for t in tables]
    non_empty = [h for h in headers if h]
    if not non_empty or len(set(non_empty)) != 1:
        return None
    captions = {(t.get("caption_text") or "").strip() for t in tables}
    captions.discard("")
    if len(captions) > 1:
        return None
    pages = [page_number(t.get("page_id")) for t in tables]
    if any(p is None for p in pages):
        return None
    pages = sorted(pages)
    if any(b - a > 1 for a, b in zip(pages, pages[1:])):
        return None
    ids = sorted(int(re.sub(r"\D", "", t.get("table_id") or "0") or 0) for t in tables)
    if all(b - a == 1 for a, b in zip(ids, ids[1:])):
        return "adjacent"
    return None


def audit_document(document, evidence, source, promoted):
    tables = source.get("structured_tables", [])
    by_id = {t.get("table_id"): t for t in tables}
    sections_by_page = {}
    for section in source.get("document_sections", []):
        page = page_number(section.get("page_id"))
        if page is not None:
            sections_by_page[page] = section.get("title") or ""

    findings = []
    for name, members in minted_enums(evidence).items():
        replays = []
        contributions = {}
        for table in tables:
            replay = replay_table(table, name, evidence.get("schema_version") == 3)
            pairs = set(replay) & set(members)
            if pairs:
                replays.append((table.get("table_id"), replay))
                contributions[table.get("table_id")] = pairs
        # The ordered reconstruction is the faithful one; the set cover is the fallback that keeps
        # an item adjudicable when convergence-pass interleaving defeats the order model.
        uncovered = set()
        chosen = attribute_in_table_order(members, replays, restarts_allowed=0)
        mode = "ordered"
        if chosen is None:
            chosen = attribute_in_table_order(members, replays, restarts_allowed=3)
            mode = "ordered_multipass"
        if chosen is None:
            chosen, uncovered = minimal_cover(members, contributions)
            mode = "set_cover"
        # One table can back several runs of the same enum; the identity question is which
        # DISTINCT tables the name fused, so collapse repeats while keeping first-seen order.
        chosen = list(dict.fromkeys(chosen))
        chosen_tables = [by_id[tid] for tid in chosen]
        sources = collections.Counter()
        per_table_sources = []
        table_owned = 0
        for table in chosen_tables:
            title = section_title_for_page(sections_by_page, page_number(table.get("page_id")))
            found = name_source(table, title, name)
            sources[",".join(found) if found else "none"] += 1
            if any(source in TABLE_OWNED_SOURCES for source in found):
                table_owned += 1
            per_table_sources.append(found)

        if uncovered:
            verdict = "unattributed"
        elif len(chosen) == 1:
            verdict = "single_table"
        else:
            continued = continuation_class(chosen_tables)
            verdict = {
                "stated": "stated_continuation",
                "adjacent": "adjacent_same_shape",
            }.get(continued, "distinct_tables")

        findings.append({
            "document": document,
            "enum": name,
            "members": len(members),
            "long_members": sum(1 for m, _ in members if len(m) > 60),
            "tables": len(chosen),
            "table_ids": chosen,
            "uncovered": len(uncovered),
            "attribution": mode,
            "verdict": verdict,
            "name_sources": dict(sources),
            "table_owned_name_tables": table_owned,
            "per_table_sources": per_table_sources,
            "promoted": name in promoted,
        })
    return findings


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--document")
    parser.add_argument("--limit", type=int, default=30)
    args = parser.parse_args()

    rows = []
    for evidence_path in sorted((ROOT / "generated" / "evidence_ir").glob("*/evidence_ir.json")):
        document = evidence_path.parent.name
        if args.document and document != args.document:
            continue
        source_path = ROOT / "generated" / "source_ir" / document / "source_ir.json"
        if not source_path.exists():
            continue
        intent_path = ROOT / "generated" / "intent_ir" / document / "intent_ir.json"
        promoted = set()
        if intent_path.exists():
            intent = json.loads(intent_path.read_text(encoding="utf-8"))
            promoted = {s.get("symbol_name") for s in intent.get("symbol_definitions", [])}
        rows.extend(audit_document(
            document,
            json.loads(evidence_path.read_text(encoding="utf-8")),
            json.loads(source_path.read_text(encoding="utf-8")),
            promoted,
        ))

    if args.json:
        json.dump(rows, sys.stdout, indent=1)
        print()
        return 0

    verdicts = collections.Counter(r["verdict"] for r in rows)
    print(f"minted enums:              {len(rows)} across "
          f"{len({r['document'] for r in rows})} document(s)")
    for verdict in VERDICTS:
        subset = [r for r in rows if r["verdict"] == verdict]
        promoted = sum(1 for r in subset if r["promoted"])
        print(f"  {verdict:24s} {verdicts[verdict]:4d}   "
              f"members={sum(r['members'] for r in subset):6d}  "
              f">60ch={sum(r['long_members'] for r in subset):5d}  promoted={promoted}")

    modes = collections.Counter(r["attribution"] for r in rows)
    print("\nattribution mode (how the minted members were traced back to tables):")
    for key in ("ordered", "ordered_multipass", "set_cover"):
        print(f"  {key:24s} {modes[key]}")

    conflated = [r for r in rows if r["verdict"] in MULTI_TABLE_VERDICTS]
    print("\nname source, per contributing table, by verdict "
          "(caption_label = the publisher's `Table N-M` furniture):")
    groups = {
        verdict: [r for r in rows if r["verdict"] == verdict]
        for verdict in VERDICTS
        if verdict != "unattributed"
    }
    keys = set()
    tallies = {}
    for label, subset in groups.items():
        tally = collections.Counter()
        for row in subset:
            for key, count in row["name_sources"].items():
                tally[key] += count
        tallies[label] = tally
        keys |= set(tally)
    header = "  " + f"{'source':40s}" + "".join(f"{v[:12]:>14s}" for v in groups)
    print(header)
    for key in sorted(keys, key=lambda k: -sum(t[k] for t in tallies.values())):
        print("  " + f"{key:40s}" + "".join(f"{tallies[v][key]:14d}" for v in groups))

    print("\nRULE SIMULATIONS — which parts of `identity_context` may name a table?")
    print("A rule is judged by what it costs the enums it should NOT touch, not by what it")
    print("removes from the ones it should. `single_table`, `stated_continuation` and")
    print("`adjacent_same_shape` are the populations a correct rule must leave alone.")
    rules = (
        ("own caption text or header", ("caption_body", "header")),
        ("anything but the caption label", ("caption_body", "header", "section")),
        ("anything but the section heading", ("caption_label", "caption_body", "header")),
    )
    print("Each rule REMOVES a part of `identity_context`, so a table whose name the instrument")
    print("found nowhere is unaffected by construction and counts as surviving; those tables are")
    print("reported separately as the instrument's own residual, not as a cost of the rule.")
    unresolved_total = sum(
        1 for row in rows for found in row["per_table_sources"] if not found
    )
    print(f"  instrument residual: {unresolved_total} contributing tables with no located "
          f"name source, of {sum(r['tables'] for r in rows)}")
    for title, allowed in rules:
        print(f"\n  rule: a name may come from {title}")
        print(f"    {'population':26s} {'total':>6s} {'refused':>8s} {'promoted':>9s} "
              f"{'members':>8s} {'narrowed':>9s} {'tables cut':>11s} {'left 1 table':>13s}")
        for label, subset in groups.items():
            refused, narrowed, cut, resolved = [], [], 0, 0
            for row in subset:
                surviving = sum(
                    1 for found in row["per_table_sources"]
                    if not found or any(source in allowed for source in found)
                )
                cut += row["tables"] - surviving
                if surviving == 0:
                    refused.append(row)
                elif surviving < row["tables"]:
                    narrowed.append(row)
                    if surviving == 1:
                        resolved += 1
            print(f"    {label:26s} {len(subset):6d} {len(refused):8d} "
                  f"{sum(1 for r in refused if r['promoted']):9d} "
                  f"{sum(r['members'] for r in refused):8d} {len(narrowed):9d} "
                  f"{cut:11d} {resolved:13d}")

    print("\nper-item adjudication — every enum the caption-label rule would refuse outright")
    print("in a population it must not touch:")
    allowed = ("caption_body", "header", "section")
    for label in ("single_table", "stated_continuation", "adjacent_same_shape"):
        for row in groups[label]:
            if any(not found or any(s in allowed for s in found)
                   for found in row["per_table_sources"]):
                continue
            print(f"  {row['document'][:40]:40s} {row['enum'][:18]:18s} "
                  f"members={row['members']:<4} >60ch={row['long_members']:<4} "
                  f"promoted={int(row['promoted'])} sources={row['name_sources']}")

    spread = collections.Counter(min(r["tables"], 20) for r in conflated)
    print("\ntables per multi-table enum (20 = 20 or more):")
    for key in sorted(spread):
        print(f"  {key:3d} tables  {spread[key]}")

    print("\nwidest multi-table enums:")
    for row in sorted(conflated, key=lambda r: -r["tables"])[:args.limit]:
        print(f"  {row['document'][:40]:40s} {row['enum'][:18]:18s} "
              f"tables={row['tables']:<4} members={row['members']:<5} "
              f">60ch={row['long_members']:<4} promoted={int(row['promoted'])} "
              f"{row['verdict']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
