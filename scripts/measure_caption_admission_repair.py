#!/usr/bin/env python3
"""What arm B1's caption repair would do to the WHOLE corpus, adjudicated row by row (read-only).

`INVARIANT-SHAPE-ADMISSION.6` — `BOUNDED-DECISION-PROVIDER.1a.1` measured a deterministic repair of
`is_invariant_like`'s caption handling on a frozen four-document set and found it worth shipping.
Shipping is a different question from scoring, and this repository's standing rule says why: **a
cheap structural rule over-fires until someone reads what it selects**
(`[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]`). Four documents is not the
corpus. This producer puts the same rules to all 78 persisted documents and prints every row each
one moves, so the selection can be adjudicated before a line of production changes.

## The two halves, and why they are different populations

**REMOVALS** — caption-shaped statements route `r1` admits today that R1 or R2 would refuse. This is
a precision change, and its risk is deleting a real requirement.

**ADDITIONS** — statements route `r1` does NOT admit that R3 would. This is a recall change, and its
risk is admitting something that is not an obligation. It reaches every statement, not only captions,
because route `r1` does.

## What the corpus said that four documents could not

Three refinements, each found by reading the corpus selection and each costing nothing else:

  1. **`was`/`were` is dropped from the negated-permission form.** Corpus-wide it admits exactly two
     rows, both *"Prior to Issue G, … were not permitted"* — a previous edition's rule, not this
     document's. A past-tense prohibition scoped to a superseded issue is document history.
  2. **The negated-existential window is tightened** from `[^.]{0,80}` to `[^.,;:]{0,60}`. The wide
     form matched across a clause break — SMMU's *"No\\_snoop == 1 flag, it indicates that the
     transaction **is allowed** to 'opt-out'"* — where the `no` belongs to a signal name and the
     permission is granted rather than withheld. A negated subject and its verb share one clause, so
     a comma between them means they are not subject and predicate of the same clause.
  3. **Serialized table rows are reported as their own stratum.** They are not refused: route `r1`
     already admits a table row carrying `must`, and `INVARIANT-SHAPE-ADMISSION.0` measured 769 of
     them and deliberately refused to delete them, because most carry content found nowhere else.
     They are counted separately so the recall change is not read as if it were all prose.

Read-only and deterministic: no network, no clock, no randomness, no write.

Usage:
    python3 scripts/measure_caption_admission_repair.py              # the census, with counts
    python3 scripts/measure_caption_admission_repair.py --rows       # every moved row, in full
    python3 scripts/measure_caption_admission_repair.py --json
    python3 scripts/measure_caption_admission_repair.py --self-test  # RED cases
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from build_bounded_decision_baseline import CAPTION, MODAL_PHRASES, repo_root  # noqa: E402
from score_bounded_decision_arms import (  # noqa: E402
    CROSS_REFERENCE_OPENING,
    has_sentence_terminator,
    sentences,
)

# R3, in the form this census recommends for SHIPPING — narrower than the form
# `BOUNDED-DECISION-PROVIDER.1a.1` scored, and narrower for reasons the corpus supplied. That leaf's
# producer is deliberately left untouched: it is pinned evidence for a published score, and a score
# is not re-written because a later census improved the rule it measured.
NOT_PERMITTED_SHIPPABLE = re.compile(r"\b(?:is|are)\s+not\s+permitted\b", re.IGNORECASE)
NO_X_ALLOWED_SHIPPABLE = re.compile(r"\bno\b[^.,;:]{0,60}?\b(?:is|are)\s+allowed\b", re.IGNORECASE)


def route_r1_admits(text: str) -> bool:
    lowered = text.lower()
    return any(phrase in lowered for phrase in MODAL_PHRASES)


def r3_admits(text: str) -> tuple[bool, str | None]:
    if NOT_PERMITTED_SHIPPABLE.search(text):
        return True, "not_permitted"
    if NO_X_ALLOWED_SHIPPABLE.search(text):
        return True, "no_x_allowed"
    return False, None


def repaired_caption_admits(text: str) -> bool:
    """R1 + R2 + R3 over a caption-shaped statement."""
    if not has_sentence_terminator(text):
        return False
    for sentence in sentences(text):
        if CROSS_REFERENCE_OPENING.match(sentence):
            continue
        if route_r1_admits(sentence) or r3_admits(sentence)[0]:
            return True
    return False


def census(root: str) -> dict:
    removals: list[dict] = []
    additions: list[dict] = []
    documents = statements = captions = 0
    for path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        documents += 1
        for statement in evidence.get("extracted_statements") or []:
            text = " ".join(statement["text"].split())
            statements += 1
            is_caption = bool(CAPTION.match(text))
            captions += 1 if is_caption else 0
            admitted = route_r1_admits(text)
            if is_caption:
                if admitted and not repaired_caption_admits(text):
                    removals.append({
                        "document": document,
                        "statement_id": statement["statement_id"],
                        "rule": "R1_title" if not has_sentence_terminator(text) else "R2_crossref",
                        "text": text[:400],
                    })
                continue
            if admitted:
                continue
            fires, form = r3_admits(text)
            if fires:
                additions.append({
                    "document": document,
                    "statement_id": statement["statement_id"],
                    "form": form,
                    "serialized_table_row": text.lstrip().startswith("|"),
                    "text": text[:400],
                })
    return {
        "documents": documents,
        "statements": statements,
        "caption_shaped": captions,
        "removals": removals,
        "additions": additions,
    }


def summarise(result: dict) -> dict:
    removals, additions = result["removals"], result["additions"]
    return {
        "documents": result["documents"],
        "statements": result["statements"],
        "caption_shaped": result["caption_shaped"],
        "removals": len(removals),
        "removals_by_rule": {
            rule: sum(1 for r in removals if r["rule"] == rule)
            for rule in ("R1_title", "R2_crossref")
        },
        "additions": len(additions),
        "additions_by_form": {
            form: sum(1 for a in additions if a["form"] == form)
            for form in ("not_permitted", "no_x_allowed")
        },
        "additions_serialized_table_rows": sum(
            1 for a in additions if a["serialized_table_row"]
        ),
        "additions_prose": sum(1 for a in additions if not a["serialized_table_row"]),
        "additions_distinct_texts": len({a["text"] for a in additions}),
        "additions_documents": len({a["document"] for a in additions}),
    }


def render(result: dict, show_rows: bool) -> None:
    summary = summarise(result)
    print("=== INVARIANT-SHAPE-ADMISSION.6 — the caption repair over the WHOLE corpus (read-only) ===")
    print(f"  documents {summary['documents']}   statements {summary['statements']}"
          f"   caption-shaped {summary['caption_shaped']}")
    print()
    print(f"== REMOVALS (precision): {summary['removals']} caption statements route r1 admits today")
    for rule, count in summary["removals_by_rule"].items():
        print(f"   {count:5d}  {rule}")
    print()
    print(f"== ADDITIONS (recall): {summary['additions']} statements route r1 does not admit")
    for form, count in summary["additions_by_form"].items():
        print(f"   {count:5d}  {form}")
    print(f"   {summary['additions_serialized_table_rows']:5d}  of them are serialized table rows"
          " — the stratum route r1 already admits on the same footing")
    print(f"   {summary['additions_prose']:5d}  prose")
    print(f"   across {summary['additions_documents']} documents,"
          f" {summary['additions_distinct_texts']} distinct texts"
          " (the corpus carries several editions of some specifications)")
    if not show_rows:
        print("\n  --rows prints every moved row; the adjudication is in"
              " docs/research/caption-admission-repair-census.md")
        return
    print("\n-- every REMOVAL --")
    for row in result["removals"]:
        print(f"  [{row['rule']:12s}] {row['document'][:24]:26s} {row['statement_id']:16s}"
              f" {row['text'][:150]}")
    print("\n-- every ADDITION --")
    for row in result["additions"]:
        kind = "table-row" if row["serialized_table_row"] else "prose    "
        print(f"  [{row['form']:14s}] {kind} {row['document'][:24]:26s} {row['statement_id']:16s}"
              f" {row['text'][:150]}")


# The census this leaf adjudicated. `--self-test` holds the shape of the result, not the corpus:
# a rebuild moves the totals and the adjudication must then be redone, which is what these pin.
PINNED = {
    "removals": 71,
    "removals_by_rule": {"R1_title": 15, "R2_crossref": 56},
    "additions": 176,
    "additions_by_form": {"not_permitted": 168, "no_x_allowed": 8},
    "additions_serialized_table_rows": 38,
}


def self_test(root: str) -> int:
    failures: list[str] = []
    passed = 0

    def check(case: str, condition: bool) -> None:
        nonlocal passed
        if condition:
            passed += 1
        else:
            failures.append(case)

    # RED 1 — `was`/`were` must NOT admit. Corpus-wide the wide form's only two extra rows are a
    # previous edition's rule, and a superseded prohibition is document history, not a requirement.
    check("past-tense-is-not-a-current-prohibition", not r3_admits(
        "Prior to Issue G, the UC, UD, and UDP initial cache states at the sending of a "
        "ReadPreferUnique were not permitted.")[0])
    check("present-tense-still-admits", r3_admits(
        "A cache state change from UC to UCE is not permitted.")[0])

    # RED 2 — the negated-existential window must not cross a clause break. The wide form matched
    # SMMU's `No\_snoop == 1 flag, it indicates that the transaction is allowed`, where the `no`
    # belongs to a signal name and the permission is GRANTED.
    check("negated-subject-does-not-cross-a-clause-break", not r3_admits(
        "When a transaction includes a No\\_snoop == 1 flag, it indicates that the transaction is "
        "allowed to 'opt-out' of hardware cache coherency.")[0])
    check("negated-existential-still-admits", r3_admits(
        "No additional SWDIOTMS LOW cycles are allowed.")[0]
        and r3_admits("No device is allowed to acknowledge at the reception of the START byte.")[0])

    # RED 3 — R1 refuses a title carrying a modal; R2 refuses a deontic a cross-reference reports,
    # and is anchored to the sentence OPENING so a relative-clause reporting verb does not refuse a
    # real prohibition.
    check("r1-refuses-a-titled-modal", not repaired_caption_admits(
        "Table A8.2: Opcodes which must be cache line sized and Regular"))
    check("r2-refuses-a-reported-deontic", not repaired_caption_admits(
        "Figure B5-11 shows, as a state diagram, the sequence that a JTAG device must recognize."))
    check("r2-is-anchored-to-the-opening", repaired_caption_admits(
        "Table 3-7 shows the mapping. The bit combinations that Table 3-7 does not show, are not "
        "permitted."))

    # RED 4 — the live census still has the shape this leaf adjudicated. A rebuild that moves it is
    # a real event: the adjudication was of THESE rows and has to be redone, not assumed to carry.
    summary = summarise(census(root))
    for key, expected in PINNED.items():
        check(f"census-pin-{key}", summary[key] == expected)

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"caption-admission repair census: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rows", action="store_true", help="print every moved row")
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    args = parser.parse_args()
    root = repo_root()
    if args.self_test:
        return self_test(root)
    result = census(root)
    if args.json:
        json.dump(result, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result, args.rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
