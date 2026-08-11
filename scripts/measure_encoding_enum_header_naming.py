#!/usr/bin/env python3
"""Read-only census for KG-ISF-COMPLETENESS.5.iv — can an encoding table's own column
header SOURCE an enum name, rather than only veto a caption-derived one?

Background. `derive_encoding_enum_name` (crates/specforge/src/ir/evidence.rs) picks a
candidate enum name from the table caption or the section title, then keeps it only when
it is independently evidenced — a declared signal, or a reference token of the table's own
header (the `.5.i` gate). So the header is read as a veto and never as a source. This
script measures what changes if it were also a source.

The question `.5.i` makes load-bearing is not "would we recover more enums" but "would the
recovered names collide". `build_symbol_definitions` (ir/semantic.rs) merges symbol members
by enum name, which is how one caption keyword fused unrelated tables into the generic
`TABLE` mega-enum. So the decisive output below is the conflict count: collision groups
whose tables disagree on a shared value -> member mapping.

Reads only persisted `generated/source_ir/*/source_ir.json`. Writes nothing, runs no model,
rebuilds no stage, and is safe to run at any time. Paths resolve from the repository root so
the reproducer moves with the repository.

    python3 scripts/measure_encoding_enum_header_naming.py [--json]
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import re
import sys

# `.5.ii`'s sentence-spine idea, reproduced here for measurement only. A synthesized member
# name carrying an English sentence-spine token is a prose fragment, and the shipped gate
# drops it -- so a table whose members are all fragments would mint no enum however it is
# named. Keeping this in step with the Rust predicate matters only for the survivor count.
SENTENCE_SPINE = {
    "IS", "ARE", "BE", "BEEN", "BEING", "WAS", "WERE", "HAS", "HAVE", "HAD",
    "MUST", "SHALL", "SHOULD", "WILL", "WOULD",
    "THE", "THIS", "THAT", "THESE", "THOSE",
    "WHICH", "WHEN", "WHERE", "WHILE", "IF", "BECAUSE", "THAN", "THEN",
    "OF", "TO", "AND", "OR", "FOR", "FROM", "WITH", "AS", "BY", "IN", "ON", "NOT",
}

# Document-structure words: a header cell naming one of these names a place in the document,
# never a hardware field. These are the tokens whose caption-side twins produced the generic
# mega-enum in the first place.
STRUCTURE_WORDS = {
    "TABLE", "FIGURE", "PAGE", "ANNEX", "SECTION", "CHAPTER", "APPENDIX",
    "NOTE", "COLUMN", "ROW", "DATA", "NA",
}

# Column-role words: they describe what the column holds, not which field it belongs to, so
# they are qualifiers to strip rather than candidate names.
COLUMN_ROLE_WORDS = {
    "VALUE", "VALUES", "DESCRIPTION", "DESC", "MEANING", "ENCODING", "ENCODINGS",
    "NAME", "BITS", "BIT", "FIELD", "FUNCTION", "NOTES", "TYPE",
    "SETTING", "SETTINGS", "STATE", "RESET", "ACCESS", "COMMENT", "COMMENTS", "DEFINITION",
}

# A second column headed by one of these marks the row's right-hand cell as the member name.
DESCRIPTION_HEADERS = {
    "DESCRIPTION", "DESCRIPTIONS", "MEANING", "COMMENT", "COMMENTS", "FUNCTION", "DEFINITION",
}


def repository_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def is_prose_fragment(member: str) -> bool:
    return any(token in SENTENCE_SPINE for token in member.split("_") if token)


def synthesize_member_name(cell: str) -> str:
    return re.sub(r"[^A-Z0-9]+", "_", cell.upper()).strip("_")


def header_field_token(first_cell: str) -> str | None:
    """The single field-like token in a left header cell, or None when it is not unambiguous."""
    tokens = [t.upper() for t in re.findall(r"[A-Za-z][A-Za-z0-9_]{1,}", first_cell)]
    fields = [t for t in tokens if t not in COLUMN_ROLE_WORDS and t not in STRUCTURE_WORDS]
    return fields[0] if len(fields) == 1 else None


def candidate_tables(source_ir: dict):
    """Yield (table_id, candidate_name, {value: member}) for each header-nameable encoding table."""
    for table in source_ir.get("structured_tables", []):
        if table.get("table_kind") != "encoding":
            continue
        header_rows = table.get("header_rows") or []
        if len(header_rows) != 1:
            continue
        cells = [c.get("text", "").strip() for c in header_rows[0]]
        if len(cells) != 2:
            continue
        if cells[1].upper().replace(" ", "_").strip("_") not in DESCRIPTION_HEADERS:
            continue
        name = header_field_token(cells[0])
        if name is None:
            continue
        members: dict[str, str] = {}
        for row in table.get("body_rows", []):
            row_cells = [c.get("text", "").strip() for c in row]
            if len(row_cells) < 2:
                continue
            member = synthesize_member_name(row_cells[1])
            if member and not is_prose_fragment(member):
                members[row_cells[0]] = member
        yield table.get("table_id"), name, members


def measure(root: str) -> dict:
    source_root = os.path.join(root, "generated", "source_ir")
    if not os.path.isdir(source_root):
        raise SystemExit(
            f"no corpus at {os.path.relpath(source_root, root)} — this census reads persisted SourceIRs"
        )

    encoding_tables = 0
    shaped = 0
    survivors = 0
    reserved_only = 0
    per_group: dict[tuple[str, str], list[tuple[str, dict[str, str]]]] = collections.defaultdict(list)
    per_document: collections.Counter = collections.Counter()

    for key in sorted(os.listdir(source_root)):
        path = os.path.join(source_root, key, "source_ir.json")
        if not os.path.isfile(path):
            continue
        with open(path, encoding="utf-8") as handle:
            source_ir = json.load(handle)
        encoding_tables += sum(
            1 for t in source_ir.get("structured_tables", []) if t.get("table_kind") == "encoding"
        )
        for table_id, name, members in candidate_tables(source_ir):
            shaped += 1
            if not members:
                continue
            survivors += 1
            per_document[key] += 1
            per_group[(key, name)].append((table_id, members))
            if set(members.values()) <= {"RESERVED"}:
                reserved_only += 1

    collisions = {k: v for k, v in per_group.items() if len(v) > 1}
    conflicting = []
    for (key, name), tables in sorted(collisions.items()):
        merged: dict[str, str] = {}
        disagreements = []
        for table_id, members in tables:
            for value, member in members.items():
                if value in merged and merged[value] != member:
                    disagreements.append(
                        {"value": value, "first": merged[value], "second": member, "table_id": table_id}
                    )
                merged.setdefault(value, member)
        if disagreements:
            conflicting.append({"document_key": key, "name": name, "disagreements": disagreements})

    return {
        "schema_version": 1,
        "encoding_tables": encoding_tables,
        "header_nameable_tables": shaped,
        "tables_minting_a_nonempty_enum": survivors,
        "documents": len(per_document),
        "candidate_name_pairs": len(per_group),
        "collision_groups": len(collisions),
        "collision_groups_in_conflict": len(conflicting),
        "reserved_only_enums": reserved_only,
        "per_document": dict(per_document.most_common()),
        "conflicts": conflicting,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()

    result = measure(repository_root())

    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
        return 0

    print(f"encoding tables corpus-wide:                       {result['encoding_tables']}")
    print(f"  header-nameable (<FIELD> value | Description):   {result['header_nameable_tables']}")
    print(f"  minting a non-empty enum after the spine gate:   {result['tables_minting_a_nonempty_enum']}"
          f"  in {result['documents']} document(s)")
    print(f"  of those, RESERVED-only (no intent):             {result['reserved_only_enums']}")
    print(f"distinct (document, candidate name) pairs:          {result['candidate_name_pairs']}")
    print(f"  drawing one name from >1 table in one document:  {result['collision_groups']}")
    print(f"  CONFLICTING on a shared value -> member mapping: {result['collision_groups_in_conflict']}")
    if result["conflicts"]:
        print("\nconflicts (these would re-create a merge-by-name conflation):")
        for entry in result["conflicts"]:
            print(f"  {entry['document_key']} :: {entry['name']}")
            for d in entry["disagreements"][:5]:
                print(f"     {d['value']}: {d['first']} vs {d['second']} ({d['table_id']})")
    else:
        print("\nno conflicts: every collision group agrees on every shared value, so merge-by-name")
        print("would reconstruct the field's encoding rather than fuse unrelated tables.")
    print("\nper document:")
    for key, count in result["per_document"].items():
        print(f"  {count:4d}  {key}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
