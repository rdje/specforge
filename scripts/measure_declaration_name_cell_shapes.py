#!/usr/bin/env python3
"""Census the SHAPE of the name cell behind every table-declared signal (read-only).

`PROSE-NAME-CELL-DECLARATION.0`. `synthesize_signal_declarations` reads a row's name
cell by taking its **first whitespace token**, trimming that token's non-identifier
characters, and asking `is_hardware_signal_token`. That is right for a footnote marker
(`HSELx a`) and for a PDF text-layer split of one identifier
(`waitrequest waitrequest _ n`). It is wrong for a cell that is a **phrase**, where the
first word is minted as a wire.

The number this tree opened on -- "345 candidate rows across 28 documents" -- is a
property of a FILTER over persisted SourceIR rows, not of the reader. Most candidate
rows never mint anything, because the reader discards a row that offers neither a
direction nor a width. This census measures the population the reader actually
produced: it joins every entry of the persisted
`EvidenceIR.table_signal_declaration_provenance` back to the source row that carries
its name, and classifies that row's name cell by shape.

Stratification is not optional here. A persisted artifact is evidence about the
producer that wrote it, never about the one running now
(`docs/knowledge/declared-spelling-is-the-document-spelling.md`): 51 of the 78 persisted
pairs were written by a producer that no longer exists. The stratum boundary is read out
of the Rust source that defines it -- `EVIDENCE_IR_SCHEMA_VERSION`, the same predicate
`eval-extraction` applies -- so a schema bump makes this census fail loudly instead of
silently agreeing with itself. CURRENT and LEGACY are reported separately and never summed.

The join is its own control. Replaying the reader's name-column selection is only
trustworthy if it recovers the names the reader actually emitted, so the join rate is
printed per stratum and per document. Measured 2026-09-12 it is 604/604 on the CURRENT
stratum; a regression there means this census -- not the reader -- has drifted. The LEGACY
stratum joins 1,251 of 2,085, and the shortfall is the uppercase-folding producer of
`SIGNAL-DECLARATION-ROW-DROP.3` showing through: a folded name is not the cell's spelling,
so it cannot join.

Shape classes, each a property of the cell's own text -- no document, vendor, or protocol
vocabulary and no English word list appears below (ADR 0006):

  * `comma-family`     -- the reader itself admitted two or more names from the cell.
  * `single-token`     -- one whitespace token; the ordinary case.
  * `footnote-marked`  -- every later token is one alphanumeric character (`HSELx a`).
  * `bracket-suffixed` -- every later token carries no letter (`ARMPAM [10:0]`).
  * `repeated-token`   -- a token recurs and every other token is a fragment: one
    identifier the PDF text layer broke apart (`waitrequest waitrequest _ n`).
  * `phrase`           -- everything else. This is the population under adjudication.

Every non-`single-token` cell is printed verbatim with its document and table, because
the adjudication is the deliverable and a count is not: the standing finding of this
repository is that a cheap structural rule over-fires until someone looks at *what* it
selects (`docs/knowledge/base-name-template-table-is-not-a-catalogue.md`).

This census classifies a POPULATION. It is not a check on the Rust implementation and
must never be cited as one: it mirrors the reader's own name-column selection, so their
agreement carries no information (`CLAIM_VERIFICATION.md` section 2).

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/evidence_ir/*/evidence_ir.json` joined to
`generated/source_ir/*/source_ir.json`. Paths resolve from the repository root, so the
reproducer moves with the repository.

Usage:
    python3 scripts/measure_declaration_name_cell_shapes.py
    python3 scripts/measure_declaration_name_cell_shapes.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

# Mirrors the header keywords `synthesize_signal_declarations` matches for its name column.
NAME_HEADER_TERMS = ("signal", "name", "port", "pin")
# `NAME_COLUMN_OVERRIDE_MARGIN` in crates/specforge/src/ir/evidence.rs.
NAME_COLUMN_OVERRIDE_MARGIN = 2

EVIDENCE_SCHEMA_SOURCE = "crates/specforge/src/ir/evidence.rs"
EVIDENCE_SCHEMA_CONSTANT = "EVIDENCE_IR_SCHEMA_VERSION"

SHAPE_ORDER = (
    "single-token",
    "comma-family",
    "footnote-marked",
    "bracket-suffixed",
    "repeated-token",
    "phrase",
)


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def read_schema_constant(root: str) -> int:
    """Read `const EVIDENCE_IR_SCHEMA_VERSION: u32 = <n>;` out of the source that owns it."""
    path = os.path.join(root, EVIDENCE_SCHEMA_SOURCE)
    with open(path, "r", encoding="utf-8") as handle:
        text = handle.read()
    match = re.search(
        rf"^\s*(?:pub\s+)?const\s+{re.escape(EVIDENCE_SCHEMA_CONSTANT)}\s*:\s*u32\s*=\s*(\d+)\s*;",
        text,
        re.MULTILINE,
    )
    if not match:
        raise SystemExit(
            f"error: {EVIDENCE_SCHEMA_CONSTANT} not found in {EVIDENCE_SCHEMA_SOURCE}; "
            "the stratum boundary cannot be trusted"
        )
    return int(match.group(1))


# ── the reader's own name-cell reading, mirrored ────────────────────────────────────

def is_hardware_signal_token(token: str) -> bool:
    """`is_hardware_signal_token`: an opaque ASCII identifier."""
    if not token:
        return False
    first = token[0]
    if not ((first.isascii() and first.isalpha()) or first == "_"):
        return False
    return all((c.isascii() and c.isalnum()) or c == "_" for c in token[1:])


def trim_to_identifier_characters(token: str) -> str:
    """`trim_matches(|c| !c.is_ascii_alphanumeric() && c != '_')` -- both ends."""

    def keep(character: str) -> bool:
        return (character.isascii() and character.isalnum()) or character == "_"

    start, end = 0, len(token)
    while start < end and not keep(token[start]):
        start += 1
    while end > start and not keep(token[end - 1]):
        end -= 1
    return token[start:end]


def first_name_token(raw_name: str) -> str:
    parts = raw_name.split()
    return trim_to_identifier_characters(parts[0]) if parts else ""


def signal_names_in_name_cell(raw_name: str) -> list[str]:
    """`signal_names_in_name_cell`: a comma family, or the single leading token."""
    single = [first_name_token(raw_name)]
    if "," not in raw_name:
        return single
    names = []
    for element in raw_name.split(","):
        element = element.strip()
        if len(element.split()) != 1 or not is_hardware_signal_token(element):
            return single
        names.append(element)
    if len(names) < 2:
        return single
    lowered = [name.lower() for name in names]
    shortest = min(len(name) for name in lowered)
    shared_prefix = 0
    while shared_prefix < shortest and all(
        name[shared_prefix] == lowered[0][shared_prefix] for name in lowered
    ):
        shared_prefix += 1
    shared_suffix = 0
    while shared_suffix < shortest and all(
        name[len(name) - 1 - shared_suffix] == lowered[0][len(lowered[0]) - 1 - shared_suffix]
        for name in lowered
    ):
        shared_suffix += 1
    if shared_prefix < 2 and shared_suffix < 2:
        return single
    return names


def header_texts(table: dict) -> list[str]:
    header_rows = table.get("header_rows") or []
    return [cell["text"].lower() for cell in header_rows[0]] if header_rows else []


def name_column(table: dict) -> int:
    """The reader's name column: header keyword, then the content-based rotation override."""
    headers = header_texts(table)
    header_col = 0
    for index, header in enumerate(headers):
        if any(term in header for term in NAME_HEADER_TERMS):
            header_col = index
            break
    rows = table.get("body_rows") or []
    col_count = max((len(row) for row in rows), default=0)

    def distinct_signal_tokens(col: int) -> int:
        tokens = set()
        for row in rows:
            if col >= len(row):
                continue
            parts = row[col]["text"].split()
            if not parts:
                continue
            token = trim_to_identifier_characters(parts[0]).upper()
            if is_hardware_signal_token(token):
                tokens.add(token)
        return len(tokens)

    header_distinct = distinct_signal_tokens(header_col)
    best_col, best_distinct = header_col, header_distinct
    for col in range(col_count):
        distinct = distinct_signal_tokens(col)
        # `Iterator::max_by_key` keeps the LAST maximum; `>=` reproduces that tie-break.
        if distinct >= best_distinct:
            best_col, best_distinct = col, distinct
    if (
        best_col != header_col
        and best_distinct >= 2
        and best_distinct >= header_distinct + NAME_COLUMN_OVERRIDE_MARGIN
    ):
        return best_col
    return header_col


# ── the shape taxonomy ──────────────────────────────────────────────────────────────

def classify_name_cell(raw_name: str) -> str:
    tokens = raw_name.split()
    if len(signal_names_in_name_cell(raw_name)) > 1:
        return "comma-family"
    if len(tokens) <= 1:
        return "single-token"
    later = tokens[1:]
    if all(len(t) == 1 and t.isascii() and t.isalnum() for t in later):
        return "footnote-marked"
    if all(not any(c.isascii() and c.isalpha() for c in t) for t in later):
        return "bracket-suffixed"
    lowered = [t.lower() for t in tokens]
    recurring = {t for t in lowered if lowered.count(t) > 1}
    # A text-layer split is one identifier the PDF broke apart, so the WHOLE cell must be
    # accounted for by the repetition: every token that does not recur has to be a fragment
    # (a lone alphanumeric, or a token carrying no letter). Requiring only "some token
    # recurs" shelved three prose blobs here -- a sentence repeats a word as a matter of
    # course -- and a misfiled false positive is exactly what this census exists to prevent.
    def is_fragment(token: str) -> bool:
        letters = [c for c in token if c.isascii() and c.isalpha()]
        return len(letters) <= 1
    if recurring and all(t in recurring or is_fragment(t) for t in lowered):
        return "repeated-token"
    return "phrase"


# ── the census ──────────────────────────────────────────────────────────────────────

def census(root: str) -> dict:
    current_schema = read_schema_constant(root)
    documents = []
    for evidence_path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        key = os.path.basename(os.path.dirname(evidence_path))
        source_path = os.path.join(root, "generated/source_ir", key, "source_ir.json")
        if not os.path.exists(source_path):
            # Every persisted EvidenceIR names the SourceIR it was built from. A missing one means
            # the corpus is half-written, and a census that silently skipped it would under-report.
            raise SystemExit(f"error: {key} has an EvidenceIR but no persisted SourceIR at {source_path}")
        with open(evidence_path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        provenance = evidence.get("table_signal_declaration_provenance") or []
        if not provenance:
            continue
        with open(source_path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        tables = {t["table_id"]: t for t in source.get("structured_tables", [])}
        by_table = collections.defaultdict(list)
        for record in provenance:
            by_table[record["table_id"]].append(record["signal_name"])

        joined, unjoined, ambiguous = [], 0, 0
        for table_id, names in by_table.items():
            table = tables.get(table_id)
            if table is None:
                unjoined += len(names)
                continue
            col = name_column(table)
            # The trapped-row path (`synthesize_trapped_row_signal_declarations`) reads a
            # table's header rows, so both row families are candidate carriers.
            candidates = []
            for row in (table.get("body_rows") or []):
                if col < len(row):
                    raw = row[col]["text"].strip()
                    candidates.append((raw, signal_names_in_name_cell(raw), "body"))
            for row in (table.get("header_rows") or []):
                if col < len(row):
                    raw = row[col]["text"].strip()
                    candidates.append((raw, signal_names_in_name_cell(raw), "header"))
            for name in names:
                hits = [c for c in candidates if name in c[1]]
                if not hits:
                    unjoined += 1
                    continue
                if len(hits) > 1:
                    ambiguous += 1
                joined.append({"table_id": table_id, "cell": hits[0][0], "signal_name": name,
                               "row_family": hits[0][2]})
        documents.append({
            "document": key,
            "schema_version": evidence.get("schema_version"),
            "stratum": "current" if evidence.get("schema_version") == current_schema else "legacy",
            "declarations": len(provenance),
            "joined": len(joined),
            "unjoined": unjoined,
            "ambiguous": ambiguous,
            "rows": joined,
        })
    return {"current_schema": current_schema, "documents": documents}


def candidate_row_approximation(root: str) -> dict:
    """The pre-census filter this tree opened on, restated so it re-derives.

    Candidate rows -- NOT declarations: a signal-description row whose name cell has two or
    more whitespace tokens and whose first token is an identifier, excluding the comma
    families the reader admits. Reported only so the difference from the real population is
    a number rather than an assertion.
    """
    rows, documents = 0, set()
    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        key = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        for table in source.get("structured_tables", []):
            if table.get("table_kind") != "signal_description":
                continue
            headers = header_texts(table)
            col = 0
            for index, header in enumerate(headers):
                if any(term in header for term in NAME_HEADER_TERMS):
                    col = index
                    break
            for row in (table.get("body_rows") or []):
                if col >= len(row):
                    continue
                raw = row[col]["text"].strip()
                if len(raw.split()) < 2:
                    continue
                if not is_hardware_signal_token(first_name_token(raw)):
                    continue
                if len(signal_names_in_name_cell(raw)) > 1:
                    continue
                rows += 1
                documents.add(key)
    return {"rows": rows, "documents": len(documents)}


def summarise(result: dict) -> dict:
    strata = {}
    for stratum in ("current", "legacy"):
        members = [d for d in result["documents"] if d.get("stratum") == stratum]
        shapes = collections.Counter()
        forms = collections.defaultdict(collections.Counter)
        for document in members:
            for row in document["rows"]:
                shape = classify_name_cell(row["cell"])
                shapes[shape] += 1
                if shape != "single-token":
                    forms[shape][(document["document"], row["table_id"], row["cell"])] += 1
        strata[stratum] = {
            "documents": len(members),
            "declarations": sum(d["declarations"] for d in members),
            "joined": sum(d["joined"] for d in members),
            "unjoined": sum(d["unjoined"] for d in members),
            "ambiguous": sum(d["ambiguous"] for d in members),
            "shapes": shapes,
            "forms": forms,
            "members": members,
        }
    return strata


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()

    root = repo_root()
    result = census(root)
    strata = summarise(result)
    approximation = candidate_row_approximation(root)

    if args.json:
        payload = {
            "current_evidence_schema_version": result["current_schema"],
            "candidate_row_approximation": approximation,
            "strata": {
                name: {
                    "documents": data["documents"],
                    "declarations": data["declarations"],
                    "joined": data["joined"],
                    "unjoined": data["unjoined"],
                    "ambiguous": data["ambiguous"],
                    "shapes": {shape: data["shapes"].get(shape, 0) for shape in SHAPE_ORDER},
                    "forms": [
                        {"shape": shape, "document": document, "table_id": table_id,
                         "cell": cell, "declarations": count}
                        for shape, members in sorted(data["forms"].items())
                        for (document, table_id, cell), count in sorted(
                            members.items(), key=lambda item: (-item[1], item[0])
                        )
                    ],
                    "per_document": [
                        {"document": d["document"], "declarations": d["declarations"],
                         "joined": d["joined"], "unjoined": d["unjoined"],
                         "shapes": {shape: count for shape, count in sorted(
                             collections.Counter(
                                 classify_name_cell(r["cell"]) for r in d["rows"]
                             ).items())}}
                        for d in sorted(data["members"], key=lambda d: d["document"])
                    ],
                }
                for name, data in strata.items()
            },
        }
        json.dump(payload, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    print("=== declaration name-cell shape census (read-only, persisted provenance) ===")
    print(f"  current EvidenceIR schema : {result['current_schema']}")
    print("  candidate-row approximation (filter over SourceIR rows, NOT declarations): "
          f"{approximation['rows']} rows across {approximation['documents']} documents")
    for name in ("current", "legacy"):
        data = strata[name]
        total = data["joined"] + data["unjoined"]
        rate = (100.0 * data["joined"] / total) if total else 0.0
        print()
        print(f"--- {name.upper()} stratum: {data['documents']} documents declaring from tables")
        print(f"    declarations            : {data['declarations']}")
        print(f"    joined to a name cell   : {data['joined']}  ({rate:.1f}%)")
        print(f"    unjoined                : {data['unjoined']}")
        print(f"    ambiguous (name recurs) : {data['ambiguous']}")
        for shape in SHAPE_ORDER:
            print(f"      {data['shapes'].get(shape, 0):5d}  {shape}")
        print("    per document:")
        for document in sorted(data["members"], key=lambda d: d["document"]):
            shapes = collections.Counter(classify_name_cell(r["cell"]) for r in document["rows"])
            phrase = shapes.get("phrase", 0)
            print(f"      {document['document'][:44]:46s} decl={document['declarations']:5d} "
                  f"joined={document['joined']:5d} phrase={phrase:3d}")
        for shape in SHAPE_ORDER:
            if shape == "single-token" or not data["forms"].get(shape):
                continue
            members = sorted(data["forms"][shape].items(), key=lambda item: (-item[1], item[0]))
            print(f"    == {shape}: {sum(c for _, c in members)} declarations, "
                  f"{len(members)} distinct cells")
            for (document, table_id, cell), count in members:
                print(f"      {count:4d}  {document[:34]:36s} {table_id:12s} {cell!r}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
