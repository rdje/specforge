#!/usr/bin/env python3
"""Read-only census for KG-ISF-COMPLETENESS.5.iv.a — which encoding tables does the shipped
header-SOURCED enum-naming path accept, and is every exclusion it applies false-positive-free?

`.5.iv` measured that a table's own column header may source an enum name and decided GO on the
lever, NO-GO on the naive predicate: four junk classes had to be excluded first. This census
measures the exclusion predicate that actually shipped, over the whole persisted corpus, and
re-derives the two numbers the leaf turns on — how many tables the path accepts, and how many of
those sit in a document whose chain can still be rebuilt.

Two corrections to the `.5.iv` census are built in and are the reason this is a separate reproducer:

1. **The population is twice as large.** `.5.iv` counted only `table_kind == "encoding"` tables. The
   shipped scan (`scan_encoding_tables_by_signal_anchor`, crates/specforge/src/ir/evidence.rs) skips
   only signal-description, register-map, and timing-parameter tables, and `table_looks_like_encoding`
   then admits any table whose header carries a name column and a value column. So `unknown`-kind
   tables are in scope too, and they carry a junk class `.5.iv` never saw: glossaries, notation
   legends, and abbreviation tables.
2. **The dominant junk class is positional, not the four classes `.5.iv` named.** A header word such
   as `Bytes`, `Offset`, `Index`, or `bits` declares the left column to hold a POSITION or an ADDRESS,
   so the table lays out where a field sits rather than what its values mean.

Reads only persisted `generated/source_ir/*/source_ir.json`. Writes nothing, runs no model, rebuilds
no stage, and is safe to run at any time. Paths resolve from the repository root so the reproducer
moves with the repository.

`--reserved-split` additionally models what `build_symbol_definitions` (ir/semantic.rs) does to the
`RESERVED`-only tables: it accumulates members by NAME per document and drops any member whose value
disagrees with the one already accumulated. That merge is why a table's own row count does not decide
whether its `RESERVED` member survives, and why the `.5.iv.a` correction of `2026-08-31` exists — the split
was first published from a per-table dump instead of from this model.

    python3 scripts/measure_header_sourced_enum_naming.py [--json] [--reserved-split]
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import re
import sys

# The shipped scan visits every table except these three kinds.
SKIPPED_TABLE_KINDS = {"signal_description", "register_map", "timing_parameter"}

# Mirrors `HEADER_DOCUMENT_STRUCTURE_WORDS` in crates/specforge/src/ir/evidence.rs.
DOCUMENT_STRUCTURE_WORDS = {
    "table", "figure", "page", "annex", "section", "chapter", "appendix",
    "note", "column", "row", "record", "data", "na",
}

# Mirrors `HEADER_COLUMN_ROLE_WORDS`.
COLUMN_ROLE_WORDS = {
    "value", "values", "description", "descriptions", "desc", "meaning", "encoding", "encodings",
    "name", "field", "function", "notes", "type", "setting", "settings", "state", "reset",
    "access", "comment", "comments", "definition",
}

# Mirrors `HEADER_POSITIONAL_COLUMN_WORDS`.
POSITIONAL_COLUMN_WORDS = {
    "bit", "bits", "byte", "bytes", "offset", "index", "address", "addresses", "range", "position",
}

# Mirrors `HEADER_DESCRIPTION_COLUMN_WORDS`.
DESCRIPTION_COLUMN_WORDS = {
    "description", "descriptions", "meaning", "comment", "comments", "function", "definition",
}


def repository_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def header_identifier_tokens(text: str) -> list[str]:
    """Mirrors `header_identifier_tokens`: a letter followed by >=1 more identifier character."""
    tokens: list[str] = []
    index = 0
    while index < len(text):
        if not (text[index].isascii() and text[index].isalpha()):
            index += 1
            continue
        start = index
        index += 1
        while index < len(text) and text[index].isascii() and (
            text[index].isalnum() or text[index] == "_"
        ):
            index += 1
        if index - start >= 2:
            tokens.append(text[start:index])
    return tokens


def parse_encoding_numeric_literal(text: str) -> int | None:
    """Mirrors `parse_encoding_numeric_literal`."""
    trimmed = text.strip().strip("[]")
    if not trimmed:
        return None
    try:
        return int(trimmed)
    except ValueError:
        pass
    lowered = trimmed.lower()
    for prefix, base in (("0b", 2), ("0x", 16)):
        if lowered.startswith(prefix):
            try:
                return int(lowered[2:], base)
            except ValueError:
                return None
    for separator, base in (("'b", 2), ("'h", 16)):
        if separator in lowered:
            try:
                return int(lowered.split(separator, 1)[1], base)
            except ValueError:
                return None
    if all(character in "01" for character in lowered):
        try:
            return int(lowered, 2)
        except ValueError:
            return None
    return None


def is_positional_range_cell(text: str) -> bool:
    """Mirrors `is_positional_range_cell`: `high:low`, optionally bracketed."""
    trimmed = text.strip().lstrip("[").rstrip("]")
    if ":" not in trimmed:
        return False
    high, _, low = trimmed.partition(":")
    high, low = high.strip(), low.strip()
    return bool(high) and bool(low) and high.isdigit() and low.isdigit()


def infer_value_column(headers: list[str], enum_name: str) -> int:
    """Mirrors the value-column half of `infer_encoding_column_indices` for the two-column shape."""
    lowered = [h.lower() for h in headers]
    name = enum_name.lower()
    for index, header in enumerate(lowered):
        if any(
            cue in header
            for cue in ("value", "encoding", "code", "binary", "hex", "bit")
        ):
            return index
        if name in header:
            return index
    return 0


def classify(table: dict) -> tuple[str, str | None]:
    """Return (verdict, enum_name). Mirrors `derive_header_sourced_enum_name`."""
    header_rows = table.get("header_rows") or []
    if len(header_rows) != 1:
        return "not-the-shape", None
    cells = [c.get("text", "") for c in header_rows[0]]
    if len(cells) != 2:
        return "not-the-shape", None

    description_role = "_".join(cells[1].split()).lower()
    if description_role not in DESCRIPTION_COLUMN_WORDS:
        return "not-the-shape", None

    field: str | None = None
    for token in header_identifier_tokens(cells[0]):
        lowered = token.lower()
        if lowered in POSITIONAL_COLUMN_WORDS:
            return "declined-positional-header", None
        if lowered in COLUMN_ROLE_WORDS or lowered in DOCUMENT_STRUCTURE_WORDS:
            continue
        if field is not None:
            return "declined-ambiguous-header", None
        field = token
    if field is None:
        return "declined-ambiguous-header", None

    value_column = infer_value_column(cells, field)
    carries_literal = False
    for row in table.get("body_rows", []):
        if value_column >= len(row):
            continue
        text = (row[value_column].get("text") or "").strip()
        if not text:
            continue
        if is_positional_range_cell(text):
            return "declined-layout-range", None
        if parse_encoding_numeric_literal(text) is not None:
            carries_literal = True
    if not carries_literal:
        return "declined-no-encoding-literal", None
    return "accepted", field


# `.5.ii`'s sentence-spine member gate, reproduced for measurement only. A member carrying an English
# sentence-spine token is a captured sentence, and the shipped gate drops it, so a table whose members are
# all fragments mints nothing however it is named.
SENTENCE_SPINE = {
    "IS", "ARE", "BE", "BEEN", "BEING", "WAS", "WERE", "HAS", "HAVE", "HAD",
    "MUST", "SHALL", "SHOULD", "WILL", "WOULD",
    "THE", "THIS", "THAT", "THESE", "THOSE",
    "WHICH", "WHEN", "WHERE", "WHILE", "IF", "BECAUSE", "THAN", "THEN",
    "OF", "TO", "AND", "OR", "FOR", "FROM", "WITH", "AS", "BY", "IN", "ON", "NOT",
}

# The unassigned-encoding marker whose exclusion `.5.iv` proposed and `.5.iv.a` declined to ship. It appears
# here only as a MEASUREMENT subject: no production path may branch on a spec-assigned value word (ADR 0006).
RESERVED_MARKER = "RESERVED"


def synthesize_member_name(cell: str) -> str:
    return re.sub(r"[^A-Z0-9]+", "_", cell.upper()).strip("_")


def is_prose_fragment(member: str) -> bool:
    return any(token in SENTENCE_SPINE for token in member.split("_") if token)


def minted_members(table: dict, enum_name: str) -> list[tuple[str, int]]:
    """(member, value) pairs the member seam would synthesize: value parses, else the row index."""
    headers = [c.get("text", "") for c in (table.get("header_rows") or [[]])[0]]
    value_column = infer_value_column(headers, enum_name)
    name_column = 1 if value_column == 0 else 0
    minted: list[tuple[str, int]] = []
    for index, row in enumerate(table.get("body_rows", [])):
        if max(value_column, name_column) >= len(row):
            continue
        member = synthesize_member_name(row[name_column].get("text", "").strip())
        if not member or is_prose_fragment(member):
            continue
        parsed = parse_encoding_numeric_literal(row[value_column].get("text", "").strip())
        minted.append((member, index if parsed is None else parsed))
    return minted


def legacy_census_module():
    """The `.5.iv` census, imported rather than re-implemented so its predicate stays single-sourced.

    The `.5.iv` frame is a DIFFERENT population from the shipped one: it selects header-nameable tables
    with no positional-header class, no layout-range clause, and no encoding-literal requirement, and it
    restricts to `table_kind == "encoding"`. Reproducing it by hand here would let the two drift, and the
    numbers being corrected are that frame's.
    """
    import importlib.util

    sibling = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                           "measure_encoding_enum_header_naming.py")
    spec = importlib.util.spec_from_file_location("measure_encoding_enum_header_naming", sibling)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def reserved_split(root: str, frame: str) -> dict:
    """Model `build_symbol_definitions`' merge-by-name and report what happens to RESERVED-only tables.

    `frame` is `"legacy_census"` — the population `.5.iv` censused, whose claim is the one corrected — or
    `"shipped"`, the population the shipped predicate accepts, which is the frame the decision governs.
    They are different sets and give different totals; the conclusion holds in both.
    """
    source_root = os.path.join(root, "generated", "source_ir")
    legacy = legacy_census_module() if frame == "legacy_census" else None
    accepted: list[tuple[str, str, str, list[tuple[str, int]]]] = []
    for key in sorted(os.listdir(source_root)):
        path = os.path.join(source_root, key, "source_ir.json")
        if not os.path.isfile(path):
            continue
        with open(path, encoding="utf-8") as handle:
            source_ir = json.load(handle)
        if legacy is not None:
            by_id = {t.get("table_id"): t for t in source_ir.get("structured_tables", [])}
            for table_id, name, members in legacy.candidate_tables(source_ir):
                if not members:
                    continue
                minted = minted_members(by_id[table_id], name)
                if minted:
                    accepted.append((key, table_id, name, minted))
            continue
        for table in source_ir.get("structured_tables", []):
            if table.get("table_kind") in SKIPPED_TABLE_KINDS:
                continue
            verdict, name = classify(table)
            if verdict != "accepted":
                continue
            members = minted_members(table, name)
            if members:
                accepted.append((key, table.get("table_id"), name, members))

    per_group: dict[tuple[str, str], list[tuple[str, int]]] = collections.defaultdict(list)
    for key, _table_id, name, members in accepted:
        per_group[(key, name)].extend(members)

    reserved_only = [t for t in accepted if {m for m, _ in t[3]} <= {RESERVED_MARKER}]
    eliminated, survived = [], []
    for key, table_id, name, _members in reserved_only:
        accumulator: dict[str, int] = {}
        conflicting: set[str] = set()
        for member, value in per_group[(key, name)]:
            if member in accumulator and accumulator[member] != value:
                conflicting.add(member)
            accumulator.setdefault(member, value)
        (eliminated if RESERVED_MARKER in conflicting else survived).append(f"{key}:{table_id}:{name}")

    single = [t for t in accepted if len({m for m, _ in t[3]}) == 1]
    return {
        "frame": "the .5.iv header-nameable census (the corrected claim's frame)"
                 if frame == "legacy_census"
                 else "the population the shipped predicate accepts (the decision's frame)",
        "tables_minting_at_least_one_member": len(accepted),
        "reserved_only_tables": len(reserved_only),
        "reserved_member_eliminated_by_merge": len(eliminated),
        "reserved_member_survives": len(survived),
        "eliminated": sorted(eliminated),
        "survived": sorted(survived),
        "single_distinct_member_tables": len(single),
        "legitimate_single_member_comparators":
            len([t for t in single if {m for m, _ in t[3]} != {RESERVED_MARKER}]),
    }


def measure(root: str) -> dict:
    source_root = os.path.join(root, "generated", "source_ir")
    if not os.path.isdir(source_root):
        raise SystemExit(
            f"no corpus at {os.path.relpath(source_root, root)} — this census reads persisted SourceIRs"
        )

    verdicts: collections.Counter = collections.Counter()
    accepted_per_document: collections.Counter = collections.Counter()
    accepted_per_kind: collections.Counter = collections.Counter()
    accepted_names: collections.Counter = collections.Counter()
    accepted_rebuildable = 0
    rebuildable_documents = 0
    scanned = 0

    for key in sorted(os.listdir(source_root)):
        path = os.path.join(source_root, key, "source_ir.json")
        if not os.path.isfile(path):
            continue
        with open(path, encoding="utf-8") as handle:
            source_ir = json.load(handle)
        # A document is rebuildable when its chain can be replayed: a current-schema SourceIR whose
        # normalized markdown bundle was retained. Everything else is legacy and needs a re-ingest.
        rebuildable = source_ir.get("schema_version") == 3 and os.path.isdir(
            os.path.join(source_root, key, "normalized")
        )
        rebuildable_documents += 1 if rebuildable else 0
        for table in source_ir.get("structured_tables", []):
            if table.get("table_kind") in SKIPPED_TABLE_KINDS:
                continue
            scanned += 1
            verdict, name = classify(table)
            if verdict == "not-the-shape":
                continue
            verdicts[verdict] += 1
            if verdict != "accepted":
                continue
            accepted_per_document[key] += 1
            accepted_per_kind[table.get("table_kind")] += 1
            accepted_names[name] += 1
            accepted_rebuildable += 1 if rebuildable else 0

    accepted = verdicts["accepted"]
    return {
        "schema_version": 1,
        "tables_scanned": scanned,
        "candidates": sum(verdicts.values()),
        "accepted": accepted,
        "declined": sum(v for k, v in verdicts.items() if k != "accepted"),
        "verdicts": dict(sorted(verdicts.items())),
        "accepted_by_table_kind": dict(sorted(accepted_per_kind.items())),
        "accepted_documents": len(accepted_per_document),
        "accepted_distinct_names": len(accepted_names),
        "rebuildable_documents": rebuildable_documents,
        "accepted_in_rebuildable_documents": accepted_rebuildable,
        "accepted_per_document": dict(accepted_per_document.most_common()),
        "accepted_names": dict(sorted(accepted_names.items())),
    }


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument(
        "--reserved-split",
        action="store_true",
        help="model build_symbol_definitions' merge and report the RESERVED-only split in both frames",
    )
    args = parser.parse_args()

    root = repository_root()
    if args.reserved_split:
        frames = [reserved_split(root, "legacy_census"), reserved_split(root, "shipped")]
        if args.json:
            print(json.dumps(frames, indent=2, sort_keys=True))
            return 0
        for frame in frames:
            print(f"frame: {frame['frame']}")
            print(f"  tables minting at least one member:        {frame['tables_minting_at_least_one_member']}")
            print(f"  RESERVED-only tables:                      {frame['reserved_only_tables']}")
            print(f"    RESERVED eliminated by the merge:        {frame['reserved_member_eliminated_by_merge']}")
            print(f"    RESERVED surviving:                      {frame['reserved_member_survives']}")
            print(f"  single-distinct-member tables:             {frame['single_distinct_member_tables']}")
            print(f"    legitimate (non-RESERVED) comparators:   {frame['legitimate_single_member_comparators']}")
            print()
        return 0

    result = measure(root)

    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
        return 0

    print(f"tables the shipped scan visits:                     {result['tables_scanned']}")
    print(f"  reaching the header path (the measured shape):    {result['candidates']}")
    print(f"  ACCEPTED (an enum name is sourced):               {result['accepted']}"
          f"  in {result['accepted_documents']} document(s),"
          f" {result['accepted_distinct_names']} distinct name(s)")
    print(f"  accepted by table_kind:                           {result['accepted_by_table_kind']}")
    print("  declined:")
    for verdict, count in result["verdicts"].items():
        if verdict == "accepted":
            continue
        print(f"     {count:5d}  {verdict}")
    print()
    print(f"rebuildable documents in the corpus:                {result['rebuildable_documents']}")
    print(f"  accepted tables inside them:                      "
          f"{result['accepted_in_rebuildable_documents']}")
    if result["accepted_in_rebuildable_documents"] == 0:
        print("  -> no persisted chain can change: every accepted table is in a legacy document")
        print("     whose SourceIR the current binary refuses for canonical use until it is re-ingested.")
    print()
    print("accepted per document:")
    for key, count in result["accepted_per_document"].items():
        print(f"  {count:4d}  {key}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
