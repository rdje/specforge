#!/usr/bin/env python3
"""Census which persisted `signal_description` tables the CURRENT classifier would still
type that way (read-only).

`PROSE-NAME-CELL-DECLARATION.4`. That leaf opened on a plain reading of a persisted
artifact: eMMC `table_0020` is a bus-speed-mode matrix
(`Mode Name | Data Rate | IO Voltage | Bus Width | Frequency | Max Data Transfer`) whose
`table_kind` is `signal_description`, and whose `HS400` row mints a signal. The obvious
conclusion is that the SourceIR classifier types a characteristics matrix as a signal
table, and that the fix belongs at the table level.

It does not. **A persisted artifact is evidence about the producer that wrote it, never
about the one running now** (`docs/knowledge/declared-spelling-is-the-document-spelling.md`).
This census applies the current classifier's own two `SignalDescription` paths to every
persisted table already carrying that kind, and reports the disagreement per stratum. The
answer is asymmetric enough to decide the leaf: **0 disagreements in the proof-carrying
stratum, 124 in the legacy one.**

The two paths mirrored here are `classified_table_kind`
(crates/specforge/src/ir/source.rs), verbatim in structure:

  1. CAPTION — the caption contains the word `table` and one of the closed signal nouns
     (`signal`/`signals`/`port`/`ports`/`pin`/`pins`).
  2. HEADER — a NAME role, plus either a DIRECTION role or (an explicit signal noun in some
     header AND a WIDTH role). Every closed role must match a header's WHOLE normalized
     label, so `Mode Name` offers no `name` role and `Bus Width` no `width` role.

This census classifies a POPULATION and is not a check on the Rust implementation: it
mirrors the classifier, so their agreement carries no information (`CLAIM_VERIFICATION.md`
section 2). The independent leg is the in-crate control
`a_matrix_that_qualifies_every_role_with_its_subject_is_not_a_signal_table`, which runs the
real `classified_table_kind` over this exact shape and is observed RED only when BOTH of
the conditions above are relaxed. The mirror's own cross-check is its 100% agreement on the
proof-carrying stratum: a divergence there would mean this file has drifted from the
classifier.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/source_ir/*/source_ir.json` joined to
`generated/evidence_ir/*/evidence_ir.json` for the stratum and the declaration count.

Usage:
    python3 scripts/measure_signal_table_classification_drift.py
    python3 scripts/measure_signal_table_classification_drift.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

# Verbatim from crates/specforge/src/ir/source.rs.
SIGNAL_NOUNS = frozenset({"signal", "signals", "port", "ports", "pin", "pins"})
NAME_ROLES = ("name", "signal", "signal name", "port", "port name", "pin", "pin name")
WIDTH_ROLES = ("width", "bit width", "bits", "size")
DIRECTION_ROLES = ("direction", "dir")

EVIDENCE_SCHEMA_SOURCE = "crates/specforge/src/ir/evidence.rs"
EVIDENCE_SCHEMA_CONSTANT = "EVIDENCE_IR_SCHEMA_VERSION"


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def read_schema_constant(root: str) -> int:
    with open(os.path.join(root, EVIDENCE_SCHEMA_SOURCE), "r", encoding="utf-8") as handle:
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


def classifier_label(value: str) -> str:
    """`classifier_label`: lowercase, split on non-alphanumeric except `/`, re-joined."""
    words, current = [], []
    for character in value.lower():
        if (character.isascii() and character.isalnum()) or character == "/":
            current.append(character)
        elif current:
            words.append("".join(current))
            current = []
    if current:
        words.append("".join(current))
    return " ".join(words)


def classifier_has_phrase(value: str, phrases) -> bool:
    normalized = f" {classifier_label(value)} "
    return any(f" {classifier_label(p)} " in normalized for p in phrases)


def header_has_role(headers, roles) -> bool:
    """`header_has_role`: WHOLE-label equality, or the balanced parenthesised qualifier."""
    wanted = {classifier_label(role) for role in roles}
    for header in headers:
        if classifier_label(header) in wanted:
            return True
        trimmed = header.strip()
        open_index = trimmed.find("(")
        if open_index < 0 or not trimmed.endswith(")"):
            continue
        qualifier = trimmed[open_index + 1 : -1]
        depth = 0
        balanced = True
        for character in qualifier:
            if character == "(":
                depth += 1
            elif character == ")":
                depth -= 1
                if depth < 0:
                    balanced = False
                    break
        if balanced and depth == 0 and classifier_label(qualifier) in wanted:
            return True
    return False


def header_names_signals(headers) -> bool:
    return any(
        any(word in SIGNAL_NOUNS for word in classifier_label(h).split()) for h in headers
    )


def current_signal_description_path(table: dict):
    """Which of the classifier's two SignalDescription paths fires, if any."""
    caption = table.get("caption_text") or ""
    headers = [c["text"] for row in table.get("header_rows", []) for c in row]
    caption_words = set(classifier_label(caption).split())
    if classifier_has_phrase(caption, ["table"]) and (caption_words & SIGNAL_NOUNS):
        return "caption"
    if header_has_role(headers, NAME_ROLES) and (
        header_has_role(headers, DIRECTION_ROLES)
        or (header_names_signals(headers) and header_has_role(headers, WIDTH_ROLES))
    ):
        return "header"
    return None


def census(root: str) -> dict:
    current_schema = read_schema_constant(root)
    agree = collections.Counter()
    disagree = collections.defaultdict(list)
    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        key = os.path.basename(os.path.dirname(path))
        evidence_path = os.path.join(root, "generated/evidence_ir", key, "evidence_ir.json")
        schema, declarations = None, collections.Counter()
        if os.path.exists(evidence_path):
            with open(evidence_path, "r", encoding="utf-8") as handle:
                evidence = json.load(handle)
            schema = evidence.get("schema_version")
            declarations = collections.Counter(
                r["table_id"]
                for r in (evidence.get("table_signal_declaration_provenance") or [])
            )
        stratum = "current" if schema == current_schema else "legacy"
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        for table in source.get("structured_tables", []):
            if table.get("table_kind") != "signal_description":
                continue
            fired = current_signal_description_path(table)
            if fired:
                agree[(stratum, fired)] += 1
                continue
            disagree[stratum].append({
                "document": key, "table_id": table["table_id"],
                "caption": table.get("caption_text"),
                "headers": [c["text"] for row in table.get("header_rows", []) for c in row],
                "declarations": declarations.get(table["table_id"], 0),
            })
    return {"current_schema": current_schema, "agree": agree, "disagree": disagree}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    result = census(repo_root())

    if args.json:
        json.dump({
            "current_evidence_schema_version": result["current_schema"],
            "agree": [{"stratum": s, "path": p, "tables": n}
                      for (s, p), n in sorted(result["agree"].items())],
            "disagree": {s: rows for s, rows in sorted(result["disagree"].items())},
        }, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    print("=== persisted signal_description tables vs the current classifier (read-only) ===")
    for (stratum, fired), count in sorted(result["agree"].items()):
        print(f"  still typed signal_description  {stratum:8s} via {fired:8s}: {count}")
    for stratum in ("current", "legacy"):
        rows = result["disagree"].get(stratum, [])
        declaring = [r for r in rows if r["declarations"] > 0]
        print()
        print(f"--- {stratum.upper()}: {len(rows)} table(s) the current classifier would NOT type "
              f"signal_description, {len(declaring)} of which declare a signal today")
        total = sum(r["declarations"] for r in rows)
        print(f"    they mint {total} declaration(s) in that stratum")
        for row in declaring:
            # A Docling-trapped table can carry a whole page in its header row; the first few
            # headers are what decides the classification, so print those and say how many follow.
            shown = row["headers"][:6]
            extra = len(row["headers"]) - len(shown)
            suffix = f" (+{extra} more)" if extra > 0 else ""
            print(f"    {row['document'][:30]:32s} {row['table_id']:12s} "
                  f"declares={row['declarations']:3d} caption={str(row['caption'])[:30]!r:32s} "
                  f"{shown}{suffix}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
