#!/usr/bin/env python3
"""Census the notations that decide a signal-description row's fate in the
authoritative declaration reader (read-only).

`SIGNAL-DECLARATION-ROW-DROP.0` measured that `synthesize_signal_declarations`
discards **482 of 2,637 rows (18.3%)** corpus-wide through its
`(direction, width)` `_ => continue` arm, and `.1` made that loss countable at
runtime (`extraction_manifest.declaration_row_accounting`). The runtime
instrument can only speak about the proof-carrying stratum: a legacy proofless
`source_ir.json` is inspection-only and the `evidence` command refuses it, so
the two documents that actually carry the notations below (Avalon, GIC-600) are
unreachable from the product. This census reads the persisted SourceIR corpus
directly, so the population every `.2` child must adjudicate is enumerable
whether or not its document can be rebuilt.

Four notations, one per `.2` child, each a property of a cell's own shape — no
document, vendor, or protocol vocabulary appears anywhere below (ADR 0006):

  * `placeholder`  (`.2a`) — a name cell whose leading token is wrapped in a
    matched bracket pair (`<name> _in`) is a metavariable: the row is a template,
    not a declaration. Stripping the delimiters is what makes such a token look
    like an identifier, which is why the reader used to accept it.
  * `flow-arrow`   (`.2b`) — a direction-bearing cell that states the signal's
    *flow* (`<driving actor> -> <receiving actor>`) rather than its port sense.
  * `enumerated-width` (`.2c`) — a width cell that lists the legal widths
    (`8, 16, 32, 64`) instead of naming one.
  * `literal-direction-column` (`.2h`) — a COLUMN, not a cell: one whose body
    cells are the literal direction words the reader already understands
    (`Input`, `Output`, `InOut`), in a table whose header carries no `direction`
    keyword, so `synthesize_signal_declarations` never looks at it. Reported per
    table with the rows that would newly gain a direction, because the unit of
    adjudication here is the table: a protocol-VERSION matrix can carry the same
    words as a property value rather than a port sense.

Every distinct cell form is printed verbatim with its count, so the selection can
be adjudicated by hand rather than trusted by count — the standing finding of
this repository is that a cheap structural rule over-fires until someone looks at
*what* it selects (`docs/knowledge/base-name-template-table-is-not-a-catalogue.md`).

This census classifies the POPULATION. It is not a check on the Rust
implementation and must never be cited as one: it shares its understanding of the
actor-role taxonomy with the code it would be checking, so their agreement would
carry no information (`CLAIM_VERIFICATION.md` §2). The independent leg is the
in-crate control suite, which runs the real reader over these exact cell forms.

Read-only and deterministic: no network, no clock, no randomness, no write, no
rebuild. Boundary: persisted `generated/source_ir/*/source_ir.json`, tables whose
persisted `table_kind` is `signal_description` — the same boundary `.0` used.

Usage:
    python3 scripts/measure_declaration_row_notations.py
    python3 scripts/measure_declaration_row_notations.py --json
"""
import collections
import glob
import json
import os
import sys

# Right-flowing arrow spellings, longest first so `-->` is one arrow and not
# `->` preceded by `-`. Mirrors FLOW_ARROW_FORMS in crates/specforge/src/ir/evidence.rs.
RIGHT_ARROWS = ["⟶", "⇒", "→", "==>", "-->", "=>", "->"]
# A reverse or bidirectional marker anywhere in the cell disqualifies it: the cell
# states no single flow this reader can close.
DISQUALIFYING_MARKERS = ["⟵", "⇐", "←", "↔", "⇔", "<-", "<="]
# The builtin actor taxonomy, verbatim from builtin_actor_taxonomy_role_in_text.
REQUESTER_TERMS = ["manager", "initiator", "master", "requester"]
COMPLETER_TERMS = [
    "subordinate", "slave", "responder", "multiplexor", "completer", "target",
]
BRACKET_PAIRS = [("<", ">"), ("(", ")"), ("[", "]"), ("{", "}")]

# The literal direction vocabulary `infer_signal_direction_from_actor_text` and the
# explicit-direction-column branch already read, plus the single-letter and
# abbreviated spellings a `Type` column uses. Closed class, no document vocabulary.
LITERAL_DIRECTION_WORDS = {
    "input", "output", "inout", "in", "out", "i", "o", "io",
    "bidirectional", "bidir",
}
# A column qualifies when most of its non-empty cells are literal direction words.
LITERAL_DIRECTION_MIN_CELLS = 3
LITERAL_DIRECTION_MIN_SHARE = 0.6

NAME_HEADER_TERMS = ["signal", "name", "port", "pin"]
WIDTH_HEADER_TERMS = ["width", "size", "bits"]
DIRECTION_HEADER_GROUPS = [
    ["direction"],
    ["source", "driver"],
    ["destination", "dest"],
]


def normalize_actor_term(text):
    """`normalize_actor_term` from crates/specforge/src/ir/prior_memory.rs."""
    folded = "".join(
        ch if (ch.isascii() and (ch.isalnum() or ch == "_")) else " "
        for ch in text.lower()
    )
    return " ".join(folded.split())


def normalized_text_contains_term(text, term):
    """Whole-token window match, as `normalized_text_contains_term` does."""
    text_tokens = text.split()
    term_tokens = term.split()
    if not text_tokens or not term_tokens or len(term_tokens) > len(text_tokens):
        return False
    return any(
        text_tokens[i:i + len(term_tokens)] == term_tokens
        for i in range(len(text_tokens) - len(term_tokens) + 1)
    )


def actor_taxonomy_role(text):
    normalized = normalize_actor_term(text)
    if not normalized:
        return None
    requester = any(normalized_text_contains_term(normalized, t) for t in REQUESTER_TERMS)
    completer = any(normalized_text_contains_term(normalized, t) for t in COMPLETER_TERMS)
    if requester and not completer:
        return "requester"
    if completer and not requester:
        return "completer"
    return None


def port_sense(actor_text, column_kind):
    """`infer_signal_direction_from_actor_text`, for one side of an arrow."""
    lowered = actor_text.lower()
    if normalize_actor_term(actor_text) in ("tie off", "tieoff"):
        return "input"
    if "output" in lowered:
        return "output"
    if "input" in lowered:
        return "input"
    role = actor_taxonomy_role(actor_text)
    if role is None:
        return None
    return {
        ("source", "requester"): "output",
        ("source", "completer"): "input",
        ("destination", "requester"): "input",
        ("destination", "completer"): "output",
    }[(column_kind, role)]


def split_on_single_flow_arrow(text):
    """Return (left, right) for exactly one right-flow arrow, else (None, count)."""
    hits = []
    index = 0
    while index < len(text):
        for arrow in RIGHT_ARROWS:
            if text.startswith(arrow, index):
                hits.append((index, arrow))
                index += len(arrow)
                break
        else:
            index += 1
    if len(hits) != 1:
        return None, len(hits)
    at, arrow = hits[0]
    return (text[:at], text[at + len(arrow):]), 1


def classify_flow_arrow(text):
    """Verdict for one direction-bearing cell; None when it carries no arrow."""
    if any(marker in text for marker in DISQUALIFYING_MARKERS):
        return ("closed:bidirectional_or_reverse_marker", None)
    parts, arrow_count = split_on_single_flow_arrow(text)
    if arrow_count == 0:
        return (None, None)
    if parts is None:
        return ("closed:multiple_arrows", None)
    left, right = parts
    from_source = port_sense(left, "source")
    from_destination = port_sense(right, "destination")
    if from_source is None or from_destination is None:
        return ("closed:unresolved_actor", None)
    if from_source != from_destination:
        return ("closed:contradictory", None)
    return ("admitted", from_source)


def leading_token_is_placeholder(name_cell):
    """A leading name token wrapped in a matched bracket pair is a metavariable."""
    tokens = name_cell.split()
    if not tokens:
        return False
    token = tokens[0]
    return len(token) >= 3 and (token[0], token[-1]) in BRACKET_PAIRS


def is_enumerated_width(text):
    """A width cell listing >= 2 positive integers names a set, not a width."""
    stripped = text.strip()
    if "," not in stripped:
        return False
    members = [part.strip() for part in stripped.split(",")]
    if len(members) < 2:
        return False
    return all(member.isdigit() and int(member) > 0 for member in members)


def row_direction_is_already_known(row, headers, direction_cols):
    """True when the reader already gets a direction for this row.

    Mirrors the first three arms of `synthesize_signal_declarations`' priority
    chain — the explicit literal, the source-like actor text, the destination-like
    actor text — plus the flow-arrow arm, which is all this census can speak for.
    """
    for column in direction_cols:
        if column >= len(row):
            continue
        text = row[column]["text"].strip()
        lowered = text.lower()
        if "output" in lowered or "input" in lowered:
            return True
        header = headers[column] if column < len(headers) else ""
        kind = "destination" if ("destination" in header or "dest" in header) else "source"
        if port_sense(text, kind) is not None:
            return True
        verdict, _ = classify_flow_arrow(text)
        if verdict == "admitted":
            return True
    return False


def literal_direction_columns(table, headers):
    """Columns whose cells ARE the literal direction words, in a table with no `direction` header."""
    if any("direction" in header for header in headers):
        return []
    body = table.get("body_rows", [])
    if not body:
        return []
    columns = []
    for column in range(max(len(row) for row in body)):
        values = [
            row[column]["text"].strip().lower()
            for row in body
            if column < len(row) and row[column]["text"].strip()
        ]
        if len(values) < LITERAL_DIRECTION_MIN_CELLS:
            continue
        literal = sum(1 for value in values if value in LITERAL_DIRECTION_WORDS)
        if literal < max(LITERAL_DIRECTION_MIN_CELLS,
                         int(LITERAL_DIRECTION_MIN_SHARE * len(values))):
            continue
        columns.append(column)
    return columns


def header_texts(table):
    header_rows = table.get("header_rows") or []
    if not header_rows:
        return []
    return [cell["text"].lower() for cell in header_rows[0]]


def first_header_matching(headers, terms):
    for index, header in enumerate(headers):
        if any(term in header for term in terms):
            return index
    return None


def census(root):
    verdicts = collections.Counter()
    forms = collections.defaultdict(collections.Counter)
    rows_seen = 0
    tables_seen = 0
    paths = sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json")))
    for path in paths:
        document = os.path.basename(os.path.dirname(path))
        with open(path, encoding="utf-8") as handle:
            source_ir = json.load(handle)
        for table in source_ir.get("structured_tables", []):
            if table.get("table_kind") != "signal_description":
                continue
            tables_seen += 1
            headers = header_texts(table)
            name_col = first_header_matching(headers, NAME_HEADER_TERMS) or 0
            width_col = first_header_matching(headers, WIDTH_HEADER_TERMS)
            direction_cols = []
            for terms in DIRECTION_HEADER_GROUPS:
                col = first_header_matching(headers, terms)
                if col is not None and col not in direction_cols:
                    direction_cols.append(col)

            def record(notation, verdict, column, text):
                key = (notation, verdict)
                verdicts[key] += 1
                header = headers[column] if column < len(headers) else ""
                forms[key][(document, header, text)] += 1

            for column in literal_direction_columns(table, headers):
                gained = [
                    row for row in table.get("body_rows", [])
                    if column < len(row)
                    and row[column]["text"].strip().lower() in LITERAL_DIRECTION_WORDS
                    and not row_direction_is_already_known(row, headers, direction_cols)
                ]
                if not gained:
                    continue
                key = ("literal-direction-column", "unread")
                verdicts[key] += len(gained)
                header = headers[column] if column < len(headers) else ""
                forms[key][(document, header,
                            f"{table.get('table_id')} headers={headers}")] += len(gained)

            for row in table.get("body_rows", []):
                rows_seen += 1
                if name_col < len(row):
                    name_cell = row[name_col]["text"].strip()
                    if leading_token_is_placeholder(name_cell):
                        record("placeholder", "metavariable", name_col, name_cell)
                for column in direction_cols:
                    if column >= len(row):
                        continue
                    text = row[column]["text"].strip()
                    verdict, sense = classify_flow_arrow(text)
                    if verdict is None:
                        continue
                    record(
                        "flow-arrow",
                        verdict + (":" + sense if sense else ""),
                        column,
                        text,
                    )
                    break
                if width_col is not None and width_col < len(row):
                    text = row[width_col]["text"].strip()
                    if is_enumerated_width(text):
                        record("enumerated-width", "width_set", width_col, text)
    return {
        "documents": len(paths),
        "signal_description_tables": tables_seen,
        "body_rows": rows_seen,
        "verdicts": verdicts,
        "forms": forms,
    }


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    result = census(root)
    if "--json" in sys.argv[1:]:
        payload = {
            "documents": result["documents"],
            "signal_description_tables": result["signal_description_tables"],
            "body_rows": result["body_rows"],
            "verdicts": [
                {"notation": notation, "verdict": verdict, "cells": count}
                for (notation, verdict), count in sorted(result["verdicts"].items())
            ],
            "forms": [
                {
                    "notation": notation,
                    "verdict": verdict,
                    "document": document,
                    "column_header": header,
                    "cell": cell,
                    "cells": count,
                }
                for (notation, verdict), members in sorted(result["forms"].items())
                for (document, header, cell), count in sorted(
                    members.items(), key=lambda item: (-item[1], item[0])
                )
            ],
        }
        json.dump(payload, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    print("=== declaration-row notation census (read-only, persisted SourceIR) ===")
    print(f"  documents                 : {result['documents']}")
    print(f"  signal_description tables : {result['signal_description_tables']}")
    print(f"  body rows examined        : {result['body_rows']}")
    for notation in ("placeholder", "flow-arrow", "enumerated-width",
                     "literal-direction-column"):
        keys = [key for key in result["verdicts"] if key[0] == notation]
        total = sum(result["verdicts"][key] for key in keys)
        print()
        print(f"--- {notation}: {total} cells")
        for key in sorted(keys):
            print(f"    {result['verdicts'][key]:4d}  {key[1]}")
        for key in sorted(keys):
            print(f"  == {key[1]}")
            members = sorted(
                result["forms"][key].items(), key=lambda item: (-item[1], item[0])
            )
            for (document, header, cell), count in members:
                print(f"    {count:4d}  {document[:34]:36s} header={header!r:26s} {cell!r}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
