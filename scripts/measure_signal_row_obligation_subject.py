#!/usr/bin/env python3
"""Census the obligation clauses in signal-description table rows, by what heads them (read-only).

`INVARIANT-SHAPE-ADMISSION.3`.

`.2` established that a published table-row constraint is a MATRIX ROW whose subject lives in
the header, and that serialization discards the header. This census measures the one shape the
declaration reader already understands end to end: a **signal-description row**, whose name cell
the reader has already turned into `Signal X is ...`, and whose description cell may state an
obligation.

The question is whose obligation it is. English binds an obligation to the nominal that
immediately precedes its modal, so each `must`/`shall` clause in a description cell is classified
by its SUBJECT HEAD — the last content token before the modal, helper words skipped:

  * `self`      the head IS the signal the row declares
                  (`| PSTRB | ... | ... PSTRB must not be active during a read transfer. |`)
  * `absent`    the clause OPENS with the modal, so it has no subject of its own and only the
                row's header can supply one
                  (`| RRESP | RRESP_WIDTH | 0b000 (OKAY) | ... Must be valid when RVALID ... |`)
  * `pronoun`   the head is `it`/`they`/`that`/`which`/`this` - a referent this census does not
                resolve. `| HWRITE | ... it must remain constant ... |` means the signal;
                `| HSELx | ... When the Subordinate is initially selected, it must also monitor
                the status of HREADY ... |` means the Subordinate. Telling them apart is anaphora,
                not a rule.
  * `other`     the head is some OTHER nominal, so the obligation is not about this row's signal
                  (`| HBURST | Subordinate | HBURST_WIDTH | ... HBURST_WIDTH must be 0 or 3. |`)

`self` and `absent` are the admissible population. `other` is the one that matters for precision:
the whole-statement subject scan can attribute such a clause to the row's name-cell signal and
mint a constraint the document never states.

This census classifies a POPULATION. It is not a check on the Rust implementation: it mirrors the
rule rather than observing it, so their agreement carries no information
(`CLAIM_VERIFICATION.md` section 2). The implementation's evidence is the rebuilt-artifact diff
and its own controls.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/source_ir/*/source_ir.json` joined to
`generated/evidence_ir/*/evidence_ir.json`, stratified by evidence schema version.

Usage:
    python3 scripts/measure_signal_row_obligation_subject.py
    python3 scripts/measure_signal_row_obligation_subject.py --json
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

# The obligation modals the constraint extractor itself recognizes (`constraint_bearing_sentence`).
MODALS = ("must", "shall")
# The clause separators that same function uses.
CLAUSE_SPLIT = re.compile(r"[.;•\n]")
# `subject_head_of_clause` helper words, from the existing head idiom in `evidence.rs`.
HELPERS = {
    "also", "always", "and", "are", "be", "been", "begin", "begins", "but", "first", "had",
    "has", "have", "however", "immediately", "is", "now", "only", "or", "started", "still",
    "subsequently", "then", "therefore", "to", "was", "were",
}
PRONOUNS = {"it", "its", "they", "them", "that", "this", "these", "those", "which", "who"}


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


def leading_identifier(text: str) -> str:
    """The row-name token the declaration reader takes: first whitespace token, trimmed."""
    token = (text.split() or [""])[0]
    return re.sub(r"^[^A-Za-z0-9_]+|[^A-Za-z0-9_]+$", "", token).upper()


def clause_modal_position(clause: str) -> int | None:
    lowered = clause.lower()
    positions = [
        match.start()
        for modal in MODALS
        for match in re.finditer(rf"\b{modal}\b", lowered)
    ]
    return min(positions) if positions else None


def subject_head(clause: str) -> str | None:
    """Last content token before the clause's modal; `None` when the modal opens the clause."""
    position = clause_modal_position(clause)
    if position is None:
        return None
    head_text = clause[:position]
    for token in reversed(head_text.split()):
        word = re.sub(r"^[^A-Za-z0-9_]+|[^A-Za-z0-9_]+$", "", token)
        if not word or word.lower() in HELPERS:
            continue
        return word
    return None


def classify(clause: str, signal: str) -> str:
    position = clause_modal_position(clause)
    if position is None:
        return "no_obligation"
    head = subject_head(clause)
    if head is None:
        return "absent"
    if head.upper() == signal.upper():
        return "self"
    if head.lower() in PRONOUNS:
        return "pronoun"
    return "other"


def description_column(header: list[str], cells: list[str], name_index: int) -> int | None:
    for index, text in enumerate(header):
        if "description" in text.lower() and index != name_index and index < len(cells):
            return index
    others = [(len(cell), index) for index, cell in enumerate(cells) if index != name_index]
    return max(others)[1] if others else None


def census(root: str) -> dict:
    current_schema = read_schema_constant(root)
    counts: collections.Counter = collections.Counter()
    rows_examined = 0
    documents = 0
    samples: dict[str, list] = collections.defaultdict(list)
    for evidence_path in sorted(
        glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))
    ):
        key = os.path.basename(os.path.dirname(evidence_path))
        with open(evidence_path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        if evidence.get("schema_version") != current_schema:
            continue
        source_path = os.path.join(root, "generated/source_ir", key, "source_ir.json")
        if not os.path.exists(source_path):
            continue
        documents += 1
        with open(source_path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        declared_by_table: dict[str, set] = collections.defaultdict(set)
        for record in evidence.get("table_signal_declaration_provenance", []):
            declared_by_table[record["table_id"]].add(record["signal_name"].upper())
        tables = {table["table_id"]: table for table in source.get("structured_tables", [])}
        for table_id, declared in sorted(declared_by_table.items()):
            table = tables.get(table_id)
            if not table:
                continue
            header_rows = table.get("header_rows") or [[]]
            header = [cell["text"] for cell in header_rows[0]]
            for row in table.get("body_rows", []):
                cells = [cell["text"] for cell in row]
                name_index = next(
                    (i for i, cell in enumerate(cells) if leading_identifier(cell) in declared),
                    None,
                )
                if name_index is None:
                    continue
                rows_examined += 1
                description_index = description_column(header, cells, name_index)
                if description_index is None:
                    continue
                signal = leading_identifier(cells[name_index])
                for clause in CLAUSE_SPLIT.split(cells[description_index]):
                    verdict = classify(clause, signal)
                    if verdict == "no_obligation":
                        continue
                    counts[verdict] += 1
                    if len(samples[verdict]) < 12:
                        samples[verdict].append(
                            {
                                "document": key,
                                "table_id": table_id,
                                "signal": signal,
                                "head": subject_head(clause) or "",
                                "clause": clause.strip()[:140],
                            }
                        )
    return {
        "current_schema": current_schema,
        "documents": documents,
        "rows_examined": rows_examined,
        "counts": counts,
        "samples": samples,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    result = census(repo_root())
    counts = result["counts"]

    if args.json:
        json.dump(
            {
                "current_evidence_schema_version": result["current_schema"],
                "documents": result["documents"],
                "rows_examined": result["rows_examined"],
                "clauses_by_subject_head": dict(sorted(counts.items())),
                "samples": {k: v for k, v in sorted(result["samples"].items())},
            },
            sys.stdout,
            indent=2,
            sort_keys=True,
        )
        sys.stdout.write("\n")
        return 0

    print("=== signal-description row obligations, by what heads them (read-only) ===")
    print(f"current-stratum documents          : {result['documents']}")
    print(f"declared signal-description rows   : {result['rows_examined']}")
    total = sum(counts.values())
    print(f"obligation clauses in those rows   : {total}")
    for verdict in ("absent", "self", "pronoun", "other"):
        count = counts.get(verdict, 0)
        share = (100.0 * count / total) if total else 0.0
        print(f"    {verdict:8s} {count:5d}  ({share:5.1f}%)")
    admissible = counts.get("absent", 0) + counts.get("self", 0)
    print(f"    admissible (absent + self)      : {admissible}")
    for verdict in ("absent", "self", "pronoun", "other"):
        rows = result["samples"].get(verdict, [])
        if not rows:
            continue
        print(f"    == {verdict} sample")
        for row in rows:
            print(
                f"        {row['document'][:22]:24s} {row['signal']:12s} "
                f"head={row['head']!r:16s} {row['clause']!r}"
            )
    return 0


if __name__ == "__main__":
    sys.exit(main())
