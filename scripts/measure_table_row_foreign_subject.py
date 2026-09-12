#!/usr/bin/env python3
"""Census the persisted signal constraints a serialized table row mis-subjects (read-only).

`INVARIANT-SHAPE-ADMISSION.5`.

`.3` established that an obligation binds to the nominal IMMEDIATELY preceding its modal, and read
signal-description rows accordingly. Adjudicating its 20 clauses found the statement paths already
reading the same rows — and getting some of them wrong, because they scan the WHOLE serialized row
for a declared-signal subject:

    | HBURST | Subordinate | HBURST_WIDTH | … HBURST_WIDTH must be 0 or 3. |
        -> dyn_sigcon_0013  HBURST must_be_value 0

`is_post_passive_binding_only_subject` is the predicate that would refuse exactly this — its own
doc-comment says a passive obligation binds to a subject that PRECEDES the modal — but its **gate 2
exempts a table row**: "a table row supplies subject context from its other cells". That is true when
the obligation clause has no subject of its own and false when it has a DIFFERENT one.

This census measures the narrowing, not the removal. The exemption is kept for every shape where a
row legitimately supplies the subject, and dropped for exactly one: the clause is headed by an
UPPERCASE-RUN IDENTIFIER that is not the record's subject.

A first draft of the rule ("the subject heads no obligation clause in the row") refused 45 records,
and adjudicating them is what produced the identifier condition — it conflated three grammars the
existing gates already handle correctly:

  * `the LASECSID signal must be 0`   head `signal`     — a descriptor; the identifier is adjacent
  * `Controller must set PREQ LOW`    head `Controller` — an ACTIVE obligation (gate 3 already keeps it)
  * `This field shall be 0h`          head `field`      — the NVMe field-cell class (`.3e`)

Requiring the head to be an uppercase-run identifier keeps all three and refuses only the real
mis-subjects.

This census classifies a POPULATION. It mirrors the rule rather than observing it, so its agreement
with the implementation carries no information (`CLAIM_VERIFICATION.md` section 2); the
implementation's evidence is its control with an observed RED and the rebuilt-artifact diff.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/evidence_ir/*/evidence_ir.json`.

Usage:
    python3 scripts/measure_table_row_foreign_subject.py
    python3 scripts/measure_table_row_foreign_subject.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

# `constraint_bearing_sentence`'s clause separators, verbatim.
CLAUSE_SPLIT = re.compile(r"[.;•\n]")
# `resolve_pronoun_subject_anaphora`'s helper words — skipped when walking back to the subject head.
HELPERS = {
    "also", "always", "and", "are", "be", "been", "begin", "begins", "but", "first", "had",
    "has", "have", "however", "immediately", "is", "now", "only", "or", "started", "still",
    "subsequently", "then", "therefore", "to", "was", "were",
}


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def constraint_bearing_sentence(text: str) -> str:
    for sentence in CLAUSE_SPLIT.split(text):
        lowered = sentence.lower()
        if "must" in lowered or "shall" in lowered:
            return sentence
    return text


def first_passive_binding_lead(text: str) -> int | None:
    """`first_passive_binding_lead`: the offset of a modal followed by [not|never] be|remain."""
    lowered = text.lower()
    words = [(m.start(), m.group(0)) for m in re.finditer(r"[A-Za-z0-9_]+", lowered)]
    for index, (offset, word) in enumerate(words):
        if word not in ("must", "shall"):
            continue
        nxt = index + 1
        if nxt < len(words) and words[nxt][1] in ("not", "never"):
            nxt += 1
        if nxt < len(words) and words[nxt][1] in ("be", "remain"):
            return offset
    return None


def contains_whole_identifier(haystack: str, needle: str) -> bool:
    pattern = r"(?<![A-Za-z0-9_])" + re.escape(needle) + r"(?![A-Za-z0-9_])"
    return re.search(pattern, haystack, re.IGNORECASE) is not None


def uppercase_run_identifier(token: str) -> str | None:
    """The token, when it IS a single maximal `[A-Z0-9_]` run of at least two characters."""
    runs = re.findall(r"[A-Z0-9_]+", token)
    if len(runs) == 1 and runs[0] == token and len(token) >= 2:
        return token
    return None


def subject_head(sentence: str, lead: int) -> str | None:
    for token in reversed(sentence[:lead].split()):
        word = re.sub(r"^[^A-Za-z0-9_]+|[^A-Za-z0-9_]+$", "", token)
        if not word or word.lower() in HELPERS:
            continue
        return word
    return None


def verdict(text: str, subject: str) -> tuple[str, str | None]:
    if not re.fullmatch(r"[A-Za-z0-9_]+", subject):
        return ("not_plain_identifier", None)
    if not text.lstrip().startswith("|"):
        return ("not_a_table_row", None)
    sentence = constraint_bearing_sentence(text)
    lead = first_passive_binding_lead(sentence)
    if lead is None:
        return ("active_obligation", None)
    if contains_whole_identifier(sentence[:lead], subject):
        return ("subject_precedes_the_lead", None)
    head = subject_head(sentence, lead)
    if head is None:
        return ("subjectless_clause", None)
    identifier = uppercase_run_identifier(head)
    if identifier is None:
        return ("common_noun_head", head)
    if identifier.upper() == subject.upper():
        return ("head_is_the_subject", head)
    return ("foreign_identifier_head", head)


def census(root: str) -> dict:
    counts: collections.Counter = collections.Counter()
    refused = []
    scanned = 0
    for path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        key = os.path.basename(os.path.dirname(path))
        try:
            with open(path, "r", encoding="utf-8") as handle:
                evidence = json.load(handle)
        except (OSError, ValueError):
            continue
        for record in evidence.get("signal_constraints", []):
            scanned += 1
            outcome, head = verdict(record["source_text"], record["subject_signal"])
            counts[outcome] += 1
            if outcome == "foreign_identifier_head":
                refused.append({
                    "document": key,
                    "constraint_id": record["constraint_id"],
                    "subject_signal": record["subject_signal"],
                    "head": head,
                    "constraint_kind": record["constraint_kind"],
                    "clause": constraint_bearing_sentence(record["source_text"]).strip()[:160],
                })
    return {"scanned": scanned, "counts": counts, "refused": refused}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    result = census(repo_root())

    if args.json:
        json.dump(
            {
                "constraints_scanned": result["scanned"],
                "by_verdict": dict(sorted(result["counts"].items())),
                "refused": result["refused"],
            },
            sys.stdout,
            indent=2,
            sort_keys=True,
        )
        sys.stdout.write("\n")
        return 0

    print("=== serialized-row constraints by what heads their obligation (read-only) ===")
    print(f"persisted signal constraints scanned: {result['scanned']}")
    for outcome, count in sorted(result["counts"].items(), key=lambda item: -item[1]):
        print(f"    {outcome:28s} {count:5d}")
    print()
    print(f"refused by the narrowing: {len(result['refused'])}")
    for row in result["refused"]:
        print(f"    {row['document'][:28]:30s} {row['constraint_id']:18s} "
              f"subject={row['subject_signal']:13s} head={row['head']:14s} "
              f"{json.dumps(row['constraint_kind'])}")
        print(f"        {row['clause']!r}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
