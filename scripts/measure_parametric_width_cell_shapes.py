#!/usr/bin/env python3
"""Census the width cells `parse_table_width_hint_text` admits as `Parametric`
(read-only).

`PROSE-NAME-CELL-DECLARATION.3` — a parametric width is an expression the
integrator sets (`ceil(DATA_WIDTH/8)`, `clog2(Num_RP_AR)`). The parser admits any
cell holding at least one ASCII letter, so a table whose width column has been
handed a *description* yields declarations such as

    Signal Clock is width The bus clock times all bus transfers. All signal
    timings are related to the rising edge of HCLK . See Clock on page 7-72.

The leaf was deferred with its rule in hand and its own instruction attached:
**re-measure before re-applying**, because the population it was sized against was
removed by an unrelated change to which column a table's names are in
(`PROSE-NAME-CELL-DECLARATION.2`). This script is that measurement, tracked so the
next re-derivation is a command rather than an excavation.

Two populations, never conflated:

  * **declarations** — what the reader actually emitted, read straight out of the
    persisted `extracted_statements`. No mirror is involved, so this number is not
    an estimate.
  * **width cells** — what the parser would admit if the row reached it. This half
    replays the reader's column selection over the persisted SourceIR, so it IS a
    mirror and carries a join control: the widths it derives must reproduce the
    widths the persisted declarations state. A join rate below 100% on the current
    stratum means this census has drifted from the reader
    (`docs/knowledge/declared-population-is-not-the-candidate-row-population.md`).

**Stratify or the number is meaningless.** The boundary is
`EVIDENCE_IR_SCHEMA_VERSION`, read out of the Rust source so a schema bump fails
loudly rather than silently re-labelling half the corpus. The strata are never
summed: a legacy artifact records a producer that no longer exists
(`docs/knowledge/persisted-table-kind-is-a-classifier-generation-artefact.md`).

Every distinct cell is printed verbatim with its token count and its terminator
shape, so a candidate discriminator is adjudicated against what it selects rather
than trusted by count. No document, vendor, protocol or English vocabulary appears
anywhere below (ADR 0006).

This census classifies the POPULATION. It is not a check on the Rust
implementation and must never be cited as one: it shares its understanding of the
reader with the code it would be checking (`CLAIM_VERIFICATION.md` §2). The
independent leg is the in-crate control suite, which runs the real parser over
these exact cell forms.

Read-only and deterministic: no network, no clock, no randomness, no write, no
rebuild. Boundary: persisted `generated/source_ir/*/source_ir.json`, every table
whose persisted `table_kind` is `signal_description` **union** every table named in
that document's `table_signal_declaration_provenance`. The union is not tidiness:
`effective_table_kind` promotes an `unknown` table through corpus memory, so AXI
`table_0251` — `Name | Width | Source | Description`, 24 rows, persisted `unknown` —
declares 24 signals that the `table_kind` boundary alone cannot see. The join
control below is what found that, and a `table_kind`-only boundary reports 92.8%.

Usage:
    python3 scripts/measure_parametric_width_cell_shapes.py
    python3 scripts/measure_parametric_width_cell_shapes.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

EVIDENCE_SCHEMA_SOURCE = "crates/specforge/src/ir/evidence.rs"
EVIDENCE_SCHEMA_CONSTANT = "EVIDENCE_IR_SCHEMA_VERSION"

# `NAME_COLUMN_OVERRIDE_MARGIN` in crates/specforge/src/ir/evidence.rs.
NAME_COLUMN_OVERRIDE_MARGIN = 2
# `is_signal_name_column_header`.
NAME_HEADER_TERMS = ("signal", "name", "port", "pin")
# The width-column header keywords `synthesize_signal_declarations` matches.
WIDTH_HEADER_TERMS = ("width", "size", "bits")
# `parse_table_width_hint_text`'s explicit refusals.
WIDTH_CELL_REFUSALS = ("-", "N/A", "n/a")
# `PROSE_TOKEN_FLOOR` in `width_expression_reads_as_prose` (PROSE-NAME-CELL-DECLARATION.3).
PROSE_TOKEN_FLOOR = 6
# `NAME_PLACEHOLDER_DELIMITERS` (SIGNAL-DECLARATION-ROW-DROP.2a).
NAME_PLACEHOLDER_DELIMITERS = (("<", ">"), ("(", ")"), ("[", "]"), ("{", "}"))

DECLARATION = re.compile(r"^Signal (\S+) is (?:(?:input|output) )?width (.+)\.$")

SENTENCE_TERMINATORS = ".?!"


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


# ── the reader's own cell reading, mirrored ─────────────────────────────────────────

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


def signal_names_in_name_cell(raw_name: str) -> list[str]:
    """`signal_names_in_name_cell`: a comma family, or the single leading token."""
    parts = raw_name.split()
    single = [trim_to_identifier_characters(parts[0]) if parts else ""]
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


def name_cell_is_read_whole(raw_name: str) -> bool:
    """`name_cell_is_read_whole` (PROSE-NAME-CELL-DECLARATION.2)."""
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

    def recurs(token: str) -> bool:
        return lowered.count(token) > 1

    def is_fragment(token: str) -> bool:
        return sum(1 for c in token if c.isascii() and c.isalpha()) <= 1

    return any(recurs(t) for t in lowered) and all(
        recurs(t) or is_fragment(t) for t in lowered
    )


def leading_name_token_is_placeholder(raw_name: str) -> bool:
    """`leading_name_token_is_placeholder`: a metavariable, not an identifier."""
    parts = raw_name.split()
    if not parts:
        return False
    token = parts[0]
    if len(token) < 3:
        return False
    return any(
        token[0] == opener and token[-1] == closer
        for opener, closer in NAME_PLACEHOLDER_DELIMITERS
    )


def is_signal_name_column_header(header: str) -> bool:
    return any(term in header for term in NAME_HEADER_TERMS)


def header_texts(table: dict) -> list[str]:
    header_rows = table.get("header_rows") or []
    return [cell["text"].lower() for cell in header_rows[0]] if header_rows else []


def column_selection(table: dict) -> tuple[int, int, int | None]:
    """Replay `synthesize_signal_declarations`' name/offset/width column selection.

    Returns `(name_col, offset, width_col)`, with `width_col` already remapped by the
    rotation offset exactly as the reader remaps it.
    """
    headers = header_texts(table)
    name_col = 0
    for index, header in enumerate(headers):
        if is_signal_name_column_header(header):
            name_col = index
            break
    width_col = None
    for index, header in enumerate(headers):
        if any(term in header for term in WIDTH_HEADER_TERMS):
            width_col = index
            break

    rows = table.get("body_rows") or []
    col_count = max((len(row) for row in rows), default=0)

    def distinct_signal_tokens(col: int) -> int:
        tokens = set()
        for row in rows:
            if col >= len(row):
                continue
            raw = row[col]["text"].strip()
            if not name_cell_is_read_whole(raw):
                continue
            parts = raw.split()
            if not parts:
                continue
            token = trim_to_identifier_characters(parts[0]).upper()
            if is_hardware_signal_token(token):
                tokens.add(token)
        return len(tokens)

    header_distinct = distinct_signal_tokens(name_col)
    best_col, best_distinct = name_col, header_distinct
    for col in range(col_count):
        distinct = distinct_signal_tokens(col)
        # `Iterator::max_by_key` keeps the LAST maximum; `>=` reproduces that tie-break.
        if distinct >= best_distinct:
            best_col, best_distinct = col, distinct

    offset = 0
    if (
        best_col != name_col
        and best_distinct >= 2
        and best_distinct >= header_distinct + NAME_COLUMN_OVERRIDE_MARGIN
    ):
        # SIGNAL-DECLARATION-ROW-DROP.2e — an ALIGNED header whose first scan matched a decoy
        # moves the name column only; a SHIFTED header rotates every other column with it.
        winner_is_named = best_col < len(headers) and is_signal_name_column_header(
            headers[best_col]
        )
        offset = 0 if winner_is_named else best_col - name_col
        name_col = best_col

    if width_col is not None and offset != 0 and col_count > 0:
        width_col = (width_col + offset) % col_count
    return name_col, offset, width_col


def parse_table_width_hint_text(text: str) -> tuple[str, str] | None:
    """`parse_table_width_hint_text`: `('numeric', …)` / `('parametric', …)` / `('prose', …)` / None.

    The mirror must move with the producer or its join rate is meaningless, so the prose refusal
    `PROSE-NAME-CELL-DECLARATION.3` shipped is reproduced here — but reported as its own kind rather
    than folded into `None`, so the population the guard acts on stays visible and countable. To the
    reader `prose` and `None` are the same answer: no width.
    """
    trimmed = text.strip()
    if not trimmed or trimmed in WIDTH_CELL_REFUSALS or all(c == "." for c in trimmed):
        return None
    if trimmed.isascii() and trimmed.isdigit():
        bits = int(trimmed)
        return ("numeric", trimmed) if bits > 0 else None
    if any(c.isascii() and c.isalpha() for c in trimmed):
        if width_expression_reads_as_prose(trimmed):
            return ("prose", trimmed)
        return ("parametric", trimmed)
    return None


def width_expression_reads_as_prose(expression: str) -> bool:
    """`width_expression_reads_as_prose`: more than six tokens AND a terminator before whitespace."""
    if len(expression.split()) <= PROSE_TOKEN_FLOOR:
        return False
    return terminator_before_whitespace(expression)


def width_hint_from_covered_signal_cell(text: str) -> tuple[str, str] | None:
    """`parse_width_hint_from_covered_signal_cell`."""
    trimmed = text.strip()
    if not trimmed:
        return None
    tokens = trimmed.split()
    if tokens:
        first = trim_to_identifier_characters(tokens[0])
        if is_hardware_signal_token(first) and len(tokens) > 1:
            hint = parse_table_width_hint_text(" ".join(tokens[1:]))
            if hint is not None:
                return hint
    return parse_table_width_hint_text(trimmed)


def row_width_hint(
    row: list[dict], headers: list[str], width_col: int | None
) -> tuple[str, str] | None:
    """`infer_signal_table_row_width_hint`."""
    if width_col is not None and width_col < len(row):
        hint = parse_table_width_hint_text(row[width_col]["text"])
        if hint is not None:
            return hint
    check_signal_col = next(
        (i for i, h in enumerate(headers) if "check signal" in h), None
    )
    covered_signal_col = next(
        (
            i
            for i, h in enumerate(headers)
            if "signals covered" in h or ("covered" in h and "signal" in h)
        ),
        None,
    )
    if (
        check_signal_col is not None
        and covered_signal_col is not None
        and covered_signal_col < len(row)
    ):
        hint = width_hint_from_covered_signal_cell(row[covered_signal_col]["text"])
        if hint is not None:
            return hint
    return None


# ── the shape taxonomy for an admitted parametric cell ──────────────────────────────

def terminator_before_whitespace(expr: str) -> bool:
    """A sentence terminator immediately followed by whitespace — an author ending a clause."""
    return any(
        expr[i] in SENTENCE_TERMINATORS and expr[i + 1].isspace()
        for i in range(len(expr) - 1)
    )


def terminator_at_end(expr: str) -> bool:
    return bool(expr) and expr.rstrip()[-1:] in SENTENCE_TERMINATORS


def expression_tokens(expr: str) -> int:
    return len(expr.split())


def cell_features(expr: str) -> dict:
    return {
        "tokens": expression_tokens(expr),
        "terminator_before_whitespace": terminator_before_whitespace(expr),
        "terminator_at_end": terminator_at_end(expr),
    }


# ── the census ──────────────────────────────────────────────────────────────────────

def declaration_population(root: str, current_schema: int) -> dict:
    """What the reader EMITTED: every persisted `Signal X is [dir ]width W.` statement."""
    strata = {
        "current": {"documents": {}, "numeric": 0, "parametric": collections.Counter()},
        "legacy": {"documents": {}, "numeric": 0, "parametric": collections.Counter()},
    }
    for path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        document = os.path.basename(os.path.dirname(path))
        key = "current" if evidence.get("schema_version") == current_schema else "legacy"
        bucket = strata[key]
        for statement in evidence.get("extracted_statements") or []:
            match = DECLARATION.match(statement["text"].strip())
            if not match:
                continue
            expr = match.group(2)
            if expr.isascii() and expr.isdigit():
                bucket["numeric"] += 1
                continue
            bucket["parametric"][(document, expr)] += 1
            bucket["documents"].setdefault(document, 0)
            bucket["documents"][document] += 1
    return strata


def declaring_table_ids(root: str, document: str) -> set[str]:
    """The tables that DID declare — `effective_table_kind` can promote an `unknown` one."""
    path = os.path.join(root, "generated/evidence_ir", document, "evidence_ir.json")
    if not os.path.exists(path):
        return set()
    with open(path, "r", encoding="utf-8") as handle:
        evidence = json.load(handle)
    return {
        record["table_id"]
        for record in evidence.get("table_signal_declaration_provenance") or []
    }


def cell_population(root: str, current_schema: int) -> dict:
    """What the PARSER would admit: every width cell of every `signal_description` table."""
    schema_by_document = {}
    for path in glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json")):
        with open(path, "r", encoding="utf-8") as handle:
            schema_by_document[os.path.basename(os.path.dirname(path))] = json.load(
                handle
            ).get("schema_version")

    strata = {
        key: {
            "tables": 0,
            "tables_with_width_column": 0,
            "cells_examined": 0,
            "numeric": 0,
            "refused": 0,
            "parametric": [],
            "prose": [],
            "derived_widths": {},
        }
        for key in ("current", "legacy")
    }

    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        key = "current" if schema_by_document.get(document) == current_schema else "legacy"
        bucket = strata[key]
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        declaring_tables = declaring_table_ids(root, document)
        for table in source.get("structured_tables") or []:
            if (
                table.get("table_kind") != "signal_description"
                and table.get("table_id") not in declaring_tables
            ):
                continue
            rows = table.get("body_rows") or []
            if not rows or (table.get("col_count") or 0) < 2:
                continue
            bucket["tables"] += 1
            headers = header_texts(table)
            name_col, _offset, width_col = column_selection(table)
            if width_col is not None:
                bucket["tables_with_width_column"] += 1
            table_id = table.get("table_id", "")
            for row in rows:
                if name_col >= len(row):
                    continue
                raw_name = row[name_col]["text"].strip()
                names = signal_names_in_name_cell(raw_name)
                token = names[0] if names else ""
                if not is_hardware_signal_token(token) or leading_name_token_is_placeholder(
                    raw_name
                ):
                    continue
                bucket["cells_examined"] += 1
                hint = row_width_hint(row, headers, width_col)
                if hint is None:
                    bucket["refused"] += 1
                    continue
                kind, value = hint
                if kind == "numeric":
                    bucket["numeric"] += 1
                elif kind == "prose":
                    bucket["prose"].append(
                        {
                            "document": document,
                            "table_id": table_id,
                            "name": token,
                            "expr": value,
                            **cell_features(value),
                        }
                    )
                else:
                    bucket["parametric"].append(
                        {
                            "document": document,
                            "table_id": table_id,
                            "name": token,
                            "expr": value,
                            **cell_features(value),
                        }
                    )
                if kind == "prose":
                    continue
                for name in names:
                    bucket["derived_widths"].setdefault((document, table_id, name), value)
    return strata


def join_control(root: str, current_schema: int, cells: dict) -> dict:
    """The census's own control: every declared width must be one this mirror derived."""
    result = {
        key: {"declarations": 0, "joined": 0, "misses": []} for key in ("current", "legacy")
    }
    for path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        document = os.path.basename(os.path.dirname(path))
        key = "current" if evidence.get("schema_version") == current_schema else "legacy"
        provenance = {
            record["statement_id"]: record
            for record in evidence.get("table_signal_declaration_provenance") or []
        }
        derived = cells[key]["derived_widths"]
        refused_as_prose = {
            (entry["document"], entry["table_id"], entry["name"])
            for entry in cells[key]["prose"]
        }
        for statement in evidence.get("extracted_statements") or []:
            match = DECLARATION.match(statement["text"].strip())
            if not match:
                continue
            record = provenance.get(statement["statement_id"])
            if record is None:
                continue
            result[key]["declarations"] += 1
            want = match.group(2)
            got = derived.get((document, record["table_id"], record["signal_name"]))
            if got == want:
                result[key]["joined"] += 1
            else:
                result[key]["misses"].append(
                    {
                        "document": document,
                        "table_id": record["table_id"],
                        "name": record["signal_name"],
                        "declared": want,
                        # A width the shipped guard refuses is not a drift: it is the change
                        # this census exists to measure, arriving from the other direction.
                        "derived": (
                            "<refused as prose>"
                            if (document, record["table_id"], record["signal_name"])
                            in refused_as_prose
                            else got
                        ),
                    }
                )
    return result


def render(strata_decl: dict, strata_cells: dict, control: dict, current_schema: int) -> None:
    print("=== parametric width-cell census (read-only, persisted artifacts) ===")
    print(f"  current EvidenceIR schema : {current_schema}")
    for key in ("current", "legacy"):
        decl = strata_decl[key]
        cells = strata_cells[key]
        join = control[key]
        rate = (
            f"{100.0 * join['joined'] / join['declarations']:.1f}%"
            if join["declarations"]
            else "n/a"
        )
        label = "CURRENT (proof-carrying)" if key == "current" else "LEGACY (inspection-only)"
        print()
        print(f"--- {label} stratum")
        print(f"    signal_description tables      : {cells['tables']}"
              f" ({cells['tables_with_width_column']} with a width column)")
        print(f"    declaring rows examined        : {cells['cells_examined']}")
        print(f"      width refused                : {cells['refused']}")
        print(f"      width numeric                : {cells['numeric']}")
        print(f"      width parametric             : {len(cells['parametric'])}")
        print(f"      width REFUSED AS PROSE       : {len(cells['prose'])}"
              f"   (PROSE-NAME-CELL-DECLARATION.3)")
        print(f"    DECLARATIONS carrying a width  : {sum(decl['parametric'].values()) + decl['numeric']}"
              f"  (numeric {decl['numeric']}, parametric {sum(decl['parametric'].values())})")
        print(f"    join control (declared width reproduced by this mirror): "
              f"{join['joined']}/{join['declarations']}  ({rate})")
        prose_misses = sum(
            1 for miss in join["misses"] if miss["derived"] == "<refused as prose>"
        )
        if prose_misses:
            print(f"      of which refused as prose by the shipped guard: {prose_misses}")
        for miss in join["misses"][:12]:
            print(f"      MISS {miss['document'][:34]:34s} {miss['table_id']:12s} "
                  f"{miss['name']:18s} declared={miss['declared']!r} derived={miss['derived']!r}")
        if len(join["misses"]) > 12:
            print(f"      ... and {len(join['misses']) - 12} more")

        prose = cells["prose"]
        if prose:
            distinct_prose = collections.Counter(
                (entry["document"], entry["table_id"], entry["name"], entry["expr"])
                for entry in prose
            )
            print(f"    the prose the guard refuses — {len(distinct_prose)} distinct forms, "
                  f"with the name the row would have declared:")
            for (document, table_id, name, expr), count in sorted(
                distinct_prose.items(), key=lambda item: (-item[1], item[0])
            ):
                print(f"      {count:4d}  {document[:26]:26s} {table_id:11s} "
                      f"name={name!r} tok={len(expr.split()):3d}  {expr!r}")

        parametric = cells["parametric"]
        if not parametric:
            continue
        by_tokens = collections.Counter(entry["tokens"] for entry in parametric)
        print("    parametric width cells by token count:")
        for tokens in sorted(by_tokens):
            print(f"      {tokens:3d} token(s) : {by_tokens[tokens]:4d}")
        # What each condition WOULD cost on its own, measured over the admitted expressions: these
        # are the real widths the shipped conjunction saves, and the reason it is not a threshold.
        tokens_only = [entry for entry in parametric if entry["tokens"] > 6]
        terminator_only = [
            entry for entry in parametric if entry["terminator_before_whitespace"]
        ]
        end_only = [entry for entry in parametric if entry["terminator_at_end"]]
        print(f"    an admitted expression each condition would cost ALONE: "
              f">6 tokens {len(tokens_only)}; terminator+whitespace {len(terminator_only)}; "
              f"terminator at end {len(end_only)}")
        for entry in sorted(tokens_only + terminator_only, key=lambda e: -e["tokens"])[:12]:
            print(f"      tok={entry['tokens']:3d}  {entry['document'][:30]:30s} "
                  f"{entry['table_id']:12s} {entry['expr']!r}")
        distinct = collections.Counter(
            (entry["document"], entry["table_id"], entry["expr"]) for entry in parametric
        )
        print(f"    distinct (document, table, expression) forms: {len(distinct)}")
        print("    every distinct form, verbatim, with token count / terminator shape:")
        for (document, table_id, expr), count in sorted(
            distinct.items(), key=lambda item: (-item[1], item[0])
        ):
            features = cell_features(expr)
            flags = "".join(
                (
                    "T" if features["terminator_before_whitespace"] else "-",
                    "E" if features["terminator_at_end"] else "-",
                )
            )
            print(f"      {count:4d}  {document[:30]:30s} {table_id:12s} "
                  f"tok={features['tokens']:3d} {flags}  {expr!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()

    root = repo_root()
    current_schema = read_schema_constant(root)
    strata_decl = declaration_population(root, current_schema)
    strata_cells = cell_population(root, current_schema)
    control = join_control(root, current_schema, strata_cells)

    if args.json:
        payload = {
            "current_schema": current_schema,
            "declarations": {
                key: {
                    "numeric": value["numeric"],
                    "parametric": sum(value["parametric"].values()),
                    "distinct": [
                        {"document": document, "expr": expr, "count": count}
                        for (document, expr), count in sorted(value["parametric"].items())
                    ],
                }
                for key, value in strata_decl.items()
            },
            "cells": {
                key: {
                    "tables": value["tables"],
                    "tables_with_width_column": value["tables_with_width_column"],
                    "cells_examined": value["cells_examined"],
                    "refused": value["refused"],
                    "numeric": value["numeric"],
                    "parametric": value["parametric"],
                    "prose": value["prose"],
                }
                for key, value in strata_cells.items()
            },
            "join_control": {
                key: {
                    "declarations": value["declarations"],
                    "joined": value["joined"],
                    "misses": value["misses"],
                }
                for key, value in control.items()
            },
        }
        json.dump(payload, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    render(strata_decl, strata_cells, control, current_schema)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
