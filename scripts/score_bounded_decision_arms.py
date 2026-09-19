#!/usr/bin/env python3
"""Arm B — the best honest LOCAL alternative — scored against arm A on the frozen set (read-only).

`BOUNDED-DECISION-PROVIDER.1a`, sub-arm **B1: the repaired rule**. `.1` froze 1,257 labelled rows and
measured what the current deterministic rules score on them; this producer scores a candidate
*deterministic repair* of those same rules on the same rows, so "the model improved things" has a
local competitor rather than only a known-defective incumbent.

**Why a separate producer, and not a mode on the baseline.** `scripts/build_bounded_decision_baseline.py`
is pinned by SHA-256 in `doctrine/claim_verification/claims.jsonl` as the producer control for
`bounded-decision-baseline-frozen`, and its RED case pins a line range inside it. Editing it to add an
arm would move both. The baseline and the arms are also different questions — one is a measured state,
the other a proposal — so they get different files. This module imports the frozen-set reader from the
baseline rather than re-reading it, so the two cannot drift into two opinions about the population.

**Nothing here is shipped.** Every predicate below is a CANDIDATE scored against the frozen set. No
production rule is touched, so `.1`'s pinned baseline stays valid and shipping stays a separate
decision with its own adjudication (`.1a`'s recommendation, not its measurement).

## What B1 proposes, per decision

`caption_admission` — three rules, none of which is a vocabulary list on its own:

  R1  **a title is not a statement.** A caption-shaped text with no sentence terminator anywhere has
      no finite main clause; it is a label plus a noun phrase, and the deontic words inside it
      qualify a noun. This is what separates `Table A8.2: Opcodes which must be cache line sized`
      from `Other combinations are not permitted.`
  R2  **a cross-reference reports, it does not oblige.** A sentence that OPENS with a figure/table
      label followed by a reporting verb (`shows`, `lists`, `summarizes`, …) describes its referent;
      a deontic inside it belongs to the thing referred to. Deliberately anchored to the sentence
      OPENING rather than to word order: `The bit combinations that Table 3-7 does not show, are not
      permitted.` has a reporting verb before its deontic and is a real prohibition, and an
      order-only test refuses it.
  R3  **negated permission is deontic.** Route `r1`'s phrase list carries `must`/`shall`/`required`
      and misses `(is|are) not permitted` and `no … (is|are) allowed`, which is how four real
      prohibitions in the frozen set are lost.

`declaration_row` — one rule, and it is deliberately small:

  R4  **a whole-cell direction abbreviation under a `Direction` header is a direction.** Scoped to a
      row the reader dropped for `no_direction_and_no_width`, so the name was already an identifier.
      `SIGNAL-DECLARATION-ROW-DROP.2h.0` refused these abbreviations on a census of the FALLBACK
      population — columns whose header names no direction — where a presence matrix writing `O` for
      *Optional* is the hazard. That hazard cannot arise under a header that says `Direction`. The
      corpus-wide population is `SIGNAL-DECLARATION-ROW-DROP.2j`'s census, not this producer's.

## The blast radius is measured, not assumed

R3 changes route `r1`, which every statement passes through — not only captions. `--blast-radius`
reads the four documents' full statement population and reports what each admitted form would NEWLY
admit, with every row printed rather than sampled, because a cheap structural rule over-fires until
its selection is inspected (`[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]`).
That mode is the only one that reads untracked `generated/` state; scoring reads the tracked frozen
set alone.

Read-only and deterministic: no network, no clock, no randomness, no write.

Usage:
    python3 scripts/score_bounded_decision_arms.py                 # arm A vs arm B1, per decision
    python3 scripts/score_bounded_decision_arms.py --json
    python3 scripts/score_bounded_decision_arms.py --blast-radius  # what R3 newly admits, in full
    python3 scripts/score_bounded_decision_arms.py --self-test     # RED cases
"""

from __future__ import annotations

import argparse
import copy
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from build_bounded_decision_baseline import (  # noqa: E402
    CAPTION,
    DOCUMENTS,
    MODAL_PHRASES,
    POSITIVE,
    PREDICT_POSITIVE,
    REPORTING_VERB,
    f1,
    read_frozen,
    repo_root,
)

# R3 — the two deontic forms route `r1` misses. Kept to exactly what the frozen set's own false
# negatives need, and every wider candidate that was tried is recorded in `REFUSED_FORMS` below with
# the count that refused it.
NOT_PERMITTED = re.compile(r"\b(?:is|are|was|were)\s+not\s+permitted\b", re.IGNORECASE)
NO_X_ALLOWED = re.compile(r"\bno\b[^.]{0,80}?\b(?:is|are)\s+allowed\b", re.IGNORECASE)

# Adjudicated and REFUSED, with the measurement that refused each. Carried here rather than in prose
# so `--blast-radius` can re-derive the refusal instead of asking a reader to trust it.
REFUSED_FORMS = {
    "prohibited": re.compile(r"\bprohibited\b", re.IGNORECASE),
    "not_legal_or_valid": re.compile(r"\b(?:is|are)\s+not\s+(?:legal|valid)\b", re.IGNORECASE),
}

# A sentence OPENING that is a figure/table cross-reference: the label, then anything up to the first
# verb, then a reporting verb. `[^.]` keeps the scan inside the sentence.
CROSS_REFERENCE_OPENING = re.compile(
    r"^\s*(?:Figure|Table)\s+[A-Za-z]?[0-9][A-Za-z0-9.\-]*[^.]{0,80}?"
    r"\b(?:shows?|lists?|summari[sz]es?|provides?|describes?|illustrates?|gives?|defines?|details?"
    r"|contains?|displays?|indicates?|presents?|specifies|specify|explains?|outlines?|introduces?)\b",
    re.IGNORECASE,
)

# Direction words a cell may carry WHOLE. The abbreviations are the point: the full words are already
# read by production, and `.2h.0` refused the abbreviations on a population this rule does not touch.
DIRECTION_CELL_VALUES = {"in", "out", "i", "o", "io", "input", "output", "inout"}


def sentences(text: str) -> list[str]:
    """Split on a sentence terminator followed by a space, plus a trailing terminator."""
    parts = re.split(r"(?<=\.)\s+", text.strip())
    return [p for p in parts if p.strip()]


def has_sentence_terminator(text: str) -> bool:
    stripped = text.strip()
    return stripped.endswith(".") or ". " in stripped


def carries_deontic(sentence: str) -> bool:
    lowered = sentence.lower()
    if any(phrase in lowered for phrase in MODAL_PHRASES):
        return True
    return bool(NOT_PERMITTED.search(sentence) or NO_X_ALLOWED.search(sentence))


def b1_admits_caption(text: str) -> bool:
    """R1 + R2 + R3, in that order."""
    if not has_sentence_terminator(text):
        return False  # R1: a title has no finite main clause
    for sentence in sentences(text):
        if CROSS_REFERENCE_OPENING.match(sentence):
            continue  # R2: this sentence reports; its deontic belongs to the referent
        if carries_deontic(sentence):
            return True  # R3 is inside `carries_deontic`
    return False


def b1_declares_row(record: dict) -> bool:
    """R4 — only for a row the reader dropped because it could read neither attribute."""
    if record.get("reader_reason") != "no_direction_and_no_width":
        return False
    for index, header in enumerate(record.get("headers") or []):
        if "direction" not in header.lower():
            continue
        cells = record.get("cells") or []
        if index < len(cells) and cells[index].strip().lower() in DIRECTION_CELL_VALUES:
            return True
    return False


def arm_a(record: dict) -> bool:
    return record["reader"] in PREDICT_POSITIVE[record["decision"]]


def arm_b1(record: dict) -> bool:
    if record["decision"] == "caption_admission":
        return b1_admits_caption(record["text"])
    return arm_a(record) or b1_declares_row(record)


ARMS = {"A": arm_a, "B1": arm_b1}


def score_arm(records: list[dict], predict) -> dict:
    result: dict = {}
    for decision in ("declaration_row", "caption_admission"):
        subset = [r for r in records if r["decision"] == decision]
        positive = POSITIVE[decision]
        matrix = {"tp": 0, "fp": 0, "fn": 0, "tn": 0}
        errors = []
        for record in subset:
            says, truth = predict(record), record["gold"] == positive
            if says and truth:
                matrix["tp"] += 1
            elif says and not truth:
                matrix["fp"] += 1
                errors.append({**record, "error": "false_positive"})
            elif not says and truth:
                matrix["fn"] += 1
                errors.append({**record, "error": "false_negative"})
            else:
                matrix["tn"] += 1
        precision, recall, positive_f1 = f1(matrix["tp"], matrix["fp"], matrix["fn"])
        _, _, negative_f1 = f1(matrix["tn"], matrix["fn"], matrix["fp"])
        result[decision] = {
            "population": len(subset),
            "confusion": matrix,
            "precision": round(precision, 5),
            "recall": round(recall, 5),
            "macro_f1": round((positive_f1 + negative_f1) / 2, 5),
            "errors": matrix["fp"] + matrix["fn"],
            "error_rows": errors,
        }
    return result


def blast_radius(root: str) -> dict:
    """What R3 would NEWLY admit outside the caption stratum, printed in full rather than sampled."""
    admitted: dict[str, list] = {"not_permitted": [], "no_x_allowed": []}
    refused: dict[str, list] = {name: [] for name in REFUSED_FORMS}
    scanned = 0
    for document in DOCUMENTS:
        path = os.path.join(root, "generated/evidence_ir", document, "evidence_ir.json")
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        for statement in evidence.get("extracted_statements") or []:
            text = " ".join(statement["text"].split())
            scanned += 1
            if any(phrase in text.lower() for phrase in MODAL_PHRASES):
                continue  # route r1 already admits it; R3 changes nothing
            entry = (document, statement["statement_id"], text[:160])
            if NOT_PERMITTED.search(text):
                admitted["not_permitted"].append(entry)
            elif NO_X_ALLOWED.search(text):
                admitted["no_x_allowed"].append(entry)
            for name, pattern in REFUSED_FORMS.items():
                if pattern.search(text):
                    refused[name].append(entry)
    return {"scanned": scanned, "admitted": admitted, "refused": refused}


# `.1`'s pre-registered C1 margin. Carried here so the requirement it places on arm C is DERIVED
# from the measured arms rather than computed by hand — the exact slip this mode exists to stop.
C1_MARGIN = 0.05


def bar_requirement(arms: dict) -> dict:
    """What C1 asks of arm C, per decision, given the arms actually measured."""
    result: dict = {}
    for decision in ("declaration_row", "caption_admission"):
        best_local = max(arms["A"][decision]["macro_f1"], arms["B1"][decision]["macro_f1"])
        target = round(best_local + C1_MARGIN, 5)
        matrix = arms["B1"][decision]["confusion"]
        # Walk arm B1's remaining false negatives, correcting them one at a time with no new false
        # positive — the most generous shape any arm C could take.
        needed, reachable = None, matrix["fn"]
        for corrected in range(matrix["fn"] + 1):
            _, _, positive = f1(matrix["tp"] + corrected, matrix["fp"], matrix["fn"] - corrected)
            _, _, negative = f1(matrix["tn"], matrix["fn"] - corrected, matrix["fp"])
            if round((positive + negative) / 2, 5) >= target and needed is None:
                needed = corrected
        result[decision] = {
            "best_local_macro_f1": best_local,
            "c1_target": target,
            "c1_target_exceeds_one": target > 1.0,
            "rows_arm_c_must_correct": needed,
            "rows_available_to_correct": reachable,
        }
    return result


def render_bar(requirement: dict) -> None:
    print()
    print("== what C1 asks of arm C, derived from the arms above (margin "
          f"{C1_MARGIN} absolute macro-F1 over the BEST of A and B)")
    for decision, entry in requirement.items():
        print(f"   {decision}")
        print(f"     best local arm        : {entry['best_local_macro_f1']:.5f}")
        print(f"     arm C must reach      : {entry['c1_target']:.5f}"
              + ("   — ABOVE 1.0, so no arm can reach it"
                 if entry["c1_target_exceeds_one"] else ""))
        if entry["rows_arm_c_must_correct"] is None:
            print(f"     rows it must correct  : unreachable — correcting all "
                  f"{entry['rows_available_to_correct']} remaining rows still falls short")
        else:
            print(f"     rows it must correct  : {entry['rows_arm_c_must_correct']}"
                  f" of the {entry['rows_available_to_correct']} arm B1 still misses")


def render(arms: dict, records: list[dict]) -> None:
    print("=== BOUNDED-DECISION-PROVIDER.1a — arm A against arm B1 on the frozen set ===")
    for decision in ("declaration_row", "caption_admission"):
        print()
        print(f"== {decision}: {arms['A'][decision]['population']} rows")
        print(f"   {'arm':4s} {'tp':>4s} {'fp':>4s} {'fn':>4s} {'tn':>4s} {'errors':>7s}"
              f" {'precision':>10s} {'recall':>8s} {'macro-F1':>9s}")
        for name in ("A", "B1"):
            entry = arms[name][decision]
            matrix = entry["confusion"]
            print(f"   {name:4s} {matrix['tp']:4d} {matrix['fp']:4d} {matrix['fn']:4d}"
                  f" {matrix['tn']:4d} {entry['errors']:7d} {entry['precision']:10.4f}"
                  f" {entry['recall']:8.4f} {entry['macro_f1']:9.5f}")
        delta = arms["B1"][decision]["macro_f1"] - arms["A"][decision]["macro_f1"]
        print(f"   B1 - A macro-F1 delta: {delta:+.5f}")
        print("   arm B1's remaining errors:")
        for row in arms["B1"][decision]["error_rows"]:
            where = (
                f"{row['table_id']} row {row['row_index']}"
                if decision == "declaration_row"
                else row["statement_id"]
            )
            subject = (
                "/".join(row["candidate_names"]) or "<no name>"
                if decision == "declaration_row"
                else row["text"][:88]
            )
            print(f"     {row['error']:15s} {row['document'][:26]:28s} {where:22s} {subject}")
        fixed = [
            r for r in arms["A"][decision]["error_rows"]
            if not any(
                (x.get("statement_id"), x.get("table_id"), x.get("row_index"))
                == (r.get("statement_id"), r.get("table_id"), r.get("row_index"))
                for x in arms["B1"][decision]["error_rows"]
            )
        ]
        print(f"   rows arm B1 corrected: {len(fixed)}")
        for row in fixed:
            where = row.get("statement_id") or f"{row['table_id']} row {row['row_index']}"
            print(f"     {row['error']:15s} {where}")


def render_blast(result: dict) -> None:
    print("=== R3 blast radius — statements route r1 does NOT already admit ===")
    print(f"  statements scanned across the four documents: {result['scanned']}")
    for name, rows in result["admitted"].items():
        print(f"\n== ADMITTED by `{name}`: {len(rows)} — every row, not a sample")
        for document, statement_id, text in rows:
            print(f"   {document[:26]:28s} {statement_id:16s} {text}")
    for name, rows in result["refused"].items():
        print(f"\n== REFUSED form `{name}`: would have admitted {len(rows)}")
        for document, statement_id, text in rows:
            print(f"   {document[:26]:28s} {statement_id:16s} {text}")


def self_test(root: str) -> int:
    _rubric, records = read_frozen(root)
    failures: list[str] = []
    passed = 0

    def check(case: str, condition: bool) -> None:
        nonlocal passed
        if condition:
            passed += 1
        else:
            failures.append(case)

    arms = {name: score_arm(records, predict) for name, predict in ARMS.items()}

    # RED 1 — arm A reproduced here must equal the baseline's own pinned matrices. Two producers
    # scoring the same decision must not drift into two answers.
    check("arm-a-reproduces-the-baseline", arms["A"]["declaration_row"]["confusion"]
          == {"tp": 567, "fp": 0, "fn": 22, "tn": 55}
          and arms["A"]["caption_admission"]["confusion"]
          == {"tp": 2, "fp": 5, "fn": 4, "tn": 602})

    # RED 2 — R1 must refuse a title even when it carries a modal. Dropping it re-admits the two
    # false positives that are titles, which is the failure this rule exists for.
    check("r1-refuses-a-titled-modal",
          not b1_admits_caption("Table A8.2: Opcodes which must be cache line sized and Regular"))

    # RED 3 — R2 must be anchored to the sentence OPENING, not to word order. An order-only test
    # refuses this real prohibition because `show` precedes `are not permitted`.
    check("r2-is-anchored-to-the-opening", b1_admits_caption(
        "Table 3-7 shows the mapping. The bit combinations that Table 3-7 does not show, "
        "are not permitted."))

    # RED 4 — R2 must still refuse a deontic that IS subordinate to a cross-reference opening.
    check("r2-refuses-a-reported-deontic", not b1_admits_caption(
        "Figure B5-11 on page B5-141 shows, as a state diagram, the sequence that a JTAG device "
        "must recognize."))

    # RED 5 — R3's two forms must each fire, and the refused forms must NOT be in the admitting set.
    check("r3-admits-negated-permission",
          b1_admits_caption("Figure B5-3 shows the sequence. Other combinations are not permitted.")
          and b1_admits_caption(
              "Figure B5-3 shows the sequence. No additional SWDIOTMS LOW cycles are allowed."))
    check("r3-does-not-carry-the-refused-forms", not b1_admits_caption(
        "Figure D2-3 shows a prohibited case. The read data in that transfer is not valid."))

    # RED 6 — R4 must fire only under a header that names a direction, and only on a row the reader
    # dropped for the reason that means its name WAS an identifier.
    fires = {"decision": "declaration_row", "reader_reason": "no_direction_and_no_width",
             "headers": ["Signal", "Direction a", "Description", "Notes"],
             "cells": ["nSRSTOUT", "Out", "Subsystem Reset", "Active LOW."]}
    wrong_header = {**fires, "headers": ["Signal", "Type", "Description", "Notes"]}
    wrong_reason = {**fires, "reader_reason": "name_not_an_identifier"}
    check("r4-needs-a-direction-header", b1_declares_row(fires)
          and not b1_declares_row(wrong_header) and not b1_declares_row(wrong_reason))

    # RED 7 — arm B1 must not introduce a false positive on this set. A repair that buys recall with
    # precision is refused by the adoption bar's own C3, and by this product's standing preference
    # for a countable miss over a silent phantom.
    check("b1-adds-no-false-positive", all(
        arms["B1"][d]["confusion"]["fp"] <= arms["A"][d]["confusion"]["fp"]
        for d in ("declaration_row", "caption_admission")))

    # RED 8 — B1 must actually differ from A, or this producer is measuring nothing.
    check("b1-differs-from-a", any(
        arms["B1"][d]["confusion"] != arms["A"][d]["confusion"]
        for d in ("declaration_row", "caption_admission")))

    # RED 9 — the frozen set must be the one `.1` pinned; scoring a mutated population silently
    # would make every number above unfalsifiable.
    mutated = copy.deepcopy(records)[:-1]
    check("population-size-is-load-bearing",
          score_arm(mutated, arm_b1)["caption_admission"]["population"]
          != arms["B1"]["caption_admission"]["population"])

    # RED 10 — the C1 requirement must be DERIVED from the measured arms. Writing it by hand is how
    # this producer's own record first published "9 of 21" for a target that needs 13.
    requirement = bar_requirement(arms)
    check("c1-requirement-is-derived",
          requirement["caption_admission"]["c1_target_exceeds_one"]
          and requirement["caption_admission"]["rows_arm_c_must_correct"] is None
          and requirement["declaration_row"]["rows_arm_c_must_correct"] == 13
          and requirement["declaration_row"]["rows_available_to_correct"] == 21)

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"bounded-decision arms: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the scored arms as JSON")
    parser.add_argument("--blast-radius", action="store_true",
                        help="what R3 newly admits across the four documents, in full")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    args = parser.parse_args()
    root = repo_root()

    if args.self_test:
        return self_test(root)
    if args.blast_radius:
        render_blast(blast_radius(root))
        return 0

    _rubric, records = read_frozen(root)
    arms = {name: score_arm(records, predict) for name, predict in ARMS.items()}
    if args.json:
        stripped = {
            name: {d: {k: v for k, v in entry.items() if k != "error_rows"}
                   for d, entry in decisions.items()}
            for name, decisions in arms.items()
        }
        stripped["c1_requirement"] = bar_requirement(arms)
        json.dump(stripped, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(arms, records)
    render_bar(bar_requirement(arms))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
