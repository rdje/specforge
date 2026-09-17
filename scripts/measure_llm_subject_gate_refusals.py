#!/usr/bin/env python3
"""Census how many persisted `llm_sigcon_*` records each positional subject gate would refuse.

`EXTRACTION-QUALITY-GAUGE.3j`.

The five positional spurious-subject gates this family built are wired as `subject_signals.retain(…)`
in the two DETERMINISTIC extractors only. `crates/specforge/src/ir/constraint_extract_llm.rs` grounds
a model proposal on catalog membership and on "did the model invent this", never on WHERE in the
sentence the subject sits. `.3j` asks what wiring them would cost before any of them is wired:

    gate                                      predicate (crates/specforge/src/ir/evidence.rs)
    CORPUS-COVERAGE.2.50a                     is_post_passive_binding_only_subject
    EXTRACTION-QUALITY-GAUGE.3e               is_descriptive_field_cell_spurious_subject
    EXTRACTION-QUALITY-GAUGE.3g               is_dotted_cross_reference_subject
    EXTRACTION-QUALITY-GAUGE.3h               is_value_position_subject
    INVARIANT-SHAPE-ADMISSION.5               obligation_head_is_a_foreign_identifier

The fifth is not independently applicable: it NARROWS gate 2 of `.2.50a` for a serialized table row.
So it is reported as the sub-count of `.2.50a`'s refusals that exist only because of it.

This census MIRRORS the predicates rather than observing them, so its agreement with the
implementation carries no information (`CLAIM_VERIFICATION.md` section 2) — exactly the standing of
`scripts/measure_table_row_foreign_subject.py`. It exists to SELECT a bounded population for hand
adjudication, which is what `.3j` requires; it is not evidence that any gate is right.

Offsets: the Rust predicates index bytes, and `to_ascii_lowercase` preserves byte layout. This mirror
indexes characters, which is identical for ASCII. Every record is checked and any whose text is not
pure ASCII is reported separately rather than silently counted.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/evidence_ir/*/evidence_ir.json`.

Usage:
    python3 scripts/measure_llm_subject_gate_refusals.py
    python3 scripts/measure_llm_subject_gate_refusals.py --json
    python3 scripts/measure_llm_subject_gate_refusals.py --show GATE   # print every refusal
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

# `OBLIGATION_SUBJECT_HELPERS`, verbatim.
HELPERS = {
    "also", "always", "and", "are", "be", "been", "begin", "begins", "but", "first", "had",
    "has", "have", "however", "immediately", "is", "now", "only", "or", "started", "still",
    "subsequently", "then", "therefore", "to", "was", "were",
}
# `sentence_states_an_obligation`, verbatim.
OBLIGATION_MARKERS = ("must", "shall", "cannot", "can not", "will not")
# `is_value_position_subject`'s VALUE_BINDERS, verbatim.
VALUE_BINDERS = (
    "set to", "cleared to", "written to", "programmed to",
    "initialized to", "initialised to", "reset to", "defaults to",
)
# `is_descriptive_field_cell_spurious_subject`'s DESCRIPTIVE_VERBS, verbatim.
DESCRIPTIVE_VERBS = (
    "indicates", "specifies", "describes", "contains", "defines",
    "represents", "reports", "identifies", "provides",
)
MARKER = "this field "
IDENT_WORD = re.compile(r"[A-Za-z0-9_]+")
UPPER_RUN = re.compile(r"[A-Z0-9_]+")


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def is_ident_char(character: str) -> bool:
    return character.isascii() and (character.isalnum() or character == "_")


def is_plain_identifier(subject: str) -> bool:
    return bool(subject) and all(is_ident_char(c) for c in subject)


def uppercase_run_tokens(text: str) -> list[tuple[int, str]]:
    """`uppercase_run_tokens`: maximal `[A-Z0-9_]` runs with their offsets, in reading order."""
    return [(m.start(), m.group(0)) for m in UPPER_RUN.finditer(text)]


def contains_whole_identifier(haystack: str, needle: str) -> bool:
    """`contains_whole_identifier`: case-insensitive occurrence at identifier boundaries."""
    haystack = haystack.lower()
    needle = needle.lower()
    scan = 0
    while True:
        start = haystack.find(needle, scan)
        if start < 0:
            return False
        end = start + len(needle)
        before_ok = start == 0 or not is_ident_char(haystack[start - 1])
        after_ok = end >= len(haystack) or not is_ident_char(haystack[end])
        if before_ok and after_ok:
            return True
        scan = start + 1


def constraint_bearing_sentence(text: str) -> str:
    """`constraint_bearing_sentence`: the first clause stating an obligation, else the whole text."""
    for sentence in re.split(r"[.;•\n]", text):
        lowered = sentence.lower()
        if any(marker in lowered for marker in OBLIGATION_MARKERS):
            return sentence
    return text


def first_passive_binding_lead(text: str) -> int | None:
    """`first_passive_binding_lead`: offset of a modal followed by [not|never] be|remain."""
    lowered = text.lower()
    words = [(m.start(), m.group(0)) for m in IDENT_WORD.finditer(lowered)]
    for index, (offset, word) in enumerate(words):
        if word not in ("must", "shall"):
            continue
        nxt = index + 1
        if nxt < len(words) and words[nxt][1] in ("not", "never"):
            nxt += 1
        if nxt < len(words) and words[nxt][1] in ("be", "remain"):
            return offset
    return None


def trim_non_identifier(token: str) -> str:
    """Rust's `trim_matches(|c| !(c.is_ascii_alphanumeric() || c == '_'))` — both ends."""
    start, end = 0, len(token)
    while start < end and not is_ident_char(token[start]):
        start += 1
    while end > start and not is_ident_char(token[end - 1]):
        end -= 1
    return token[start:end]


def content_head_with_premodifier(text: str) -> tuple[str, str | None] | None:
    """`content_head_with_premodifier`: last non-helper content word, plus the RAW token before it."""
    tokens = text.split()
    for index in range(len(tokens) - 1, -1, -1):
        word = trim_non_identifier(tokens[index])
        if word and word.lower() not in HELPERS:
            return word, (tokens[index - 1] if index >= 1 else None)
    return None


def whole_token_identifier(token: str | None) -> str | None:
    """`whole_token_identifier`: the token itself when it IS a single whole `[A-Z0-9_]{2,}` run."""
    if token is None:
        return None
    runs = uppercase_run_tokens(token)
    if len(runs) != 1:
        return None
    offset, run = runs[0]
    return run if offset == 0 and len(run) == len(token) and len(run) >= 2 else None


def obligation_head_is_a_foreign_identifier(sentence: str, lead: int, subject: str) -> bool:
    """`obligation_head_is_a_foreign_identifier` (INVARIANT-SHAPE-ADMISSION.5 + .3k.7)."""
    found = content_head_with_premodifier(sentence[:lead])
    if found is None:
        return False
    head, premodifier = found
    name = whole_token_identifier(head)
    if name is not None:
        return name.lower() != subject.lower()
    if any(len(run) >= 2 for _, run in uppercase_run_tokens(head)):
        return False
    name = whole_token_identifier(premodifier)
    if name is None:
        return False
    return name.lower() != subject.lower()


def is_post_passive_binding_only_subject(text: str, subject: str) -> tuple[bool, bool]:
    """`is_post_passive_binding_only_subject` → (refuses, refusal_needed_the_.5_narrowing)."""
    if not is_plain_identifier(subject):
        return False, False
    sentence = constraint_bearing_sentence(text)
    lead = first_passive_binding_lead(sentence)
    if lead is None:
        return False, False
    is_row = text.lstrip().startswith("|")
    narrowing = False
    if is_row:
        if not obligation_head_is_a_foreign_identifier(sentence, lead, subject):
            return False, False
        narrowing = True
    refuses = not contains_whole_identifier(sentence[:lead], subject)
    return refuses, (refuses and narrowing)


def is_descriptive_field_cell_spurious_subject(text: str, subject: str) -> bool:
    """`is_descriptive_field_cell_spurious_subject` (EXTRACTION-QUALITY-GAUGE.3e)."""
    if not is_plain_identifier(subject):
        return False
    lowered = text.lower()
    from_index = 0
    while True:
        at = lowered.find(MARKER, from_index)
        if at < 0:
            return False
        after = lowered[at + len(MARKER):].lstrip()
        descriptive = any(
            after.startswith(verb)
            and (len(after) == len(verb) or not is_ident_char(after[len(verb)]))
            for verb in DESCRIPTIVE_VERBS
        )
        if descriptive:
            return not contains_whole_identifier(lowered[:at], subject)
        from_index = at + len(MARKER)


def is_dotted_cross_reference_subject(text: str, subject: str) -> bool:
    """`is_dotted_cross_reference_subject` (EXTRACTION-QUALITY-GAUGE.3g)."""
    if not is_plain_identifier(subject):
        return False
    lowered = text.lower()
    needle = subject.lower()
    scan = 0
    saw_occurrence = False
    while True:
        start = lowered.find(needle, scan)
        if start < 0:
            break
        end = start + len(needle)
        before_ok = start == 0 or not is_ident_char(lowered[start - 1])
        after_ok = end >= len(lowered) or not is_ident_char(lowered[end])
        if before_ok and after_ok:
            saw_occurrence = True
            dotted = start >= 2 and lowered[start - 1] == "." and is_ident_char(lowered[start - 2])
            if not dotted:
                return False
        scan = start + 1
    return saw_occurrence


def is_value_position_subject(text: str, subject: str) -> bool:
    """`is_value_position_subject` (EXTRACTION-QUALITY-GAUGE.3h)."""
    if not is_plain_identifier(subject):
        return False
    lowered = text.lower()
    saw_occurrence = False
    for start, token in uppercase_run_tokens(text):
        if token != subject:
            continue
        saw_occurrence = True
        # Rust: trim_end_matches(|c| c.is_whitespace() || matches!(c, '(' | '"' | '\'' | '`')).
        end = start
        while end > 0 and (lowered[end - 1].isspace() or lowered[end - 1] in "(\"'`"):
            end -= 1
        before = lowered[:end]
        if not any(before.endswith(binder) for binder in VALUE_BINDERS):
            return False
    return saw_occurrence


GATES = (
    ("CORPUS-COVERAGE.2.50a", "is_post_passive_binding_only_subject"),
    ("EXTRACTION-QUALITY-GAUGE.3e", "is_descriptive_field_cell_spurious_subject"),
    ("EXTRACTION-QUALITY-GAUGE.3g", "is_dotted_cross_reference_subject"),
    ("EXTRACTION-QUALITY-GAUGE.3h", "is_value_position_subject"),
)


def records(root: str):
    pattern = os.path.join(root, "generated", "evidence_ir", "*", "evidence_ir.json")
    for path in sorted(glob.glob(pattern)):
        document = os.path.basename(os.path.dirname(path))
        try:
            with open(path, encoding="utf-8") as handle:
                payload = json.load(handle)
        except (OSError, ValueError):
            continue
        for constraint in payload.get("signal_constraints", []) or []:
            if str(constraint.get("constraint_id", "")).startswith("llm_sigcon"):
                yield document, constraint


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument("--show", metavar="GATE", help="print every refusal for one gate id")
    args = parser.parse_args()

    root = repo_root()
    refusals: dict[str, list[dict]] = collections.defaultdict(list)
    narrowing_only: list[dict] = []
    non_ascii: list[dict] = []
    documents: set[str] = set()
    total = 0

    for document, constraint in records(root):
        total += 1
        documents.add(document)
        subject = constraint.get("subject_signal") or ""
        text = constraint.get("source_text") or ""
        row = {
            "document": document,
            "constraint_id": constraint.get("constraint_id"),
            "subject": subject,
            "text": text,
        }
        if not text.isascii():
            non_ascii.append(row)
        passive, needed_narrowing = is_post_passive_binding_only_subject(text, subject)
        if passive:
            refusals["CORPUS-COVERAGE.2.50a"].append(row)
            if needed_narrowing:
                narrowing_only.append(row)
        if is_descriptive_field_cell_spurious_subject(text, subject):
            refusals["EXTRACTION-QUALITY-GAUGE.3e"].append(row)
        if is_dotted_cross_reference_subject(text, subject):
            refusals["EXTRACTION-QUALITY-GAUGE.3g"].append(row)
        if is_value_position_subject(text, subject):
            refusals["EXTRACTION-QUALITY-GAUGE.3h"].append(row)

    # `constraint_id` is unique only WITHIN a document, so the union key carries the document too.
    # Keyed on the id alone it read 6 for a 7-record single-gate population.
    union = {
        (row["document"], row["constraint_id"])
        for rows in refusals.values()
        for row in rows
    }

    if args.show:
        for row in refusals.get(args.show, []):
            print(f"{row['document']} {row['constraint_id']} subject={row['subject']}")
            print(f"    {row['text'][:400]}")
        return 0

    summary = {
        "population": total,
        "documents": len(documents),
        "non_ascii_texts": len(non_ascii),
        "refusals": {gate: len(refusals[gate]) for gate, _ in GATES},
        "invariant_shape_admission_5_only": len(narrowing_only),
        "union_refused": len(union),
    }
    if args.json:
        print(json.dumps(summary, sort_keys=True))
        return 0

    print(f"llm-subject-gate-refusals: {total} llm_sigcon_* records across {len(documents)} documents")
    for gate, predicate in GATES:
        count = len(refusals[gate])
        share = (count * 1000 + total // 2) // total if total else 0
        print(f"  {gate:<32} {count:>4}  ({share // 10}.{share % 10}%)  {predicate}")
    print(f"  {'INVARIANT-SHAPE-ADMISSION.5':<32} {len(narrowing_only):>4}"
          f"        of .2.50a's refusals exist only via the row narrowing")
    print(f"  {'union of all gates':<32} {len(union):>4}  distinct records")
    if non_ascii:
        print(f"  NOTE: {len(non_ascii)} record text(s) are not pure ASCII; "
              f"char offsets may differ from the Rust byte offsets there.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
