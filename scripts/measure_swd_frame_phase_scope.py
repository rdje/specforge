#!/usr/bin/env python3
"""WIRE-BASED-100.8e — derive, rather than assert, the numbers behind .8d's deferral.

`.8d` deferred generic recovery of the SWD serial-frame fields on the ground that no
statement-scope or section-scope rule can supply the right phase. That conclusion holds, but
three of the figures published with it were read off a probe instead of derived, and the probe
was not a faithful port of the production gate:

  * it dropped `parse_count_word`, so "two or three phases" was accepted as the phase name
    "three";
  * it stripped non-alphabetic characters anywhere in a token instead of trimming only the ends,
    as Rust's `trim_matches` does.

This script ports `stated_phase_name` and `parse_count_word` exactly, checks the ported count-word
list against the Rust source so the two cannot drift silently, and prints every figure `.8d`
publishes. Run it instead of quoting the numbers.

    python3 scripts/measure_swd_frame_phase_scope.py
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EVIDENCE_RS = ROOT / "crates/specforge/src/ir/evidence.rs"
DOC_KEY = "ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification"
EVIDENCE_IR = ROOT / "generated/evidence_ir" / DOC_KEY / "evidence_ir.json"
GOLD = ROOT / "crates/specforge/test_data/llm_eval/seed_swd_derivation.json"

# Ported from `parse_count_word`. Verified against the Rust source by `check_port_against_source`.
COUNT_WORDS = {
    "single": 1, "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6,
    "seven": 7, "eight": 8, "nine": 9, "ten": 10, "eleven": 11, "twelve": 12,
}


def check_port_against_source() -> None:
    """Fail loudly if the Rust count-word list stops matching the ported one."""
    source = EVIDENCE_RS.read_text(encoding="utf-8")
    body = re.search(r"fn parse_count_word.*?\n\}", source, re.S)
    if not body:
        sys.exit("cannot locate parse_count_word in evidence.rs; the port cannot be checked")
    found = dict(re.findall(r'"([a-z]+)"(?:\s*\|\s*"[a-z]+")*\s*=>\s*Some\((\d+)\)', body.group(0)))
    aliases = re.findall(r'"([a-z]+)"\s*\|\s*"([a-z]+)"\s*=>\s*Some\((\d+)\)', body.group(0))
    rust = {word: int(value) for word, value in found.items()}
    for first, second, value in aliases:
        rust[first] = rust[second] = int(value)
    if rust != COUNT_WORDS:
        sys.exit(f"parse_count_word drifted: rust={sorted(rust)} ported={sorted(COUNT_WORDS)}")


def _trim(token: str, drop) -> str:
    """Rust `str::trim_matches`: remove matching characters from BOTH ENDS only."""
    start, end = 0, len(token)
    while start < end and drop(token[start]):
        start += 1
    while end > start and drop(token[end - 1]):
        end -= 1
    return token[start:end]


def _not_alpha(character: str) -> bool:
    return not (character.isascii() and character.isalpha())


def _not_label_char(character: str) -> bool:
    return not (character.isascii() and (character.isalnum() or character in "-/"))


def parse_count_word(token: str) -> int | None:
    trimmed = _trim(token, lambda c: not (c.isascii() and c.isalnum())).lower()
    if trimmed in COUNT_WORDS:
        return COUNT_WORDS[trimmed]
    try:
        return int(trimmed)
    except ValueError:
        return None


def stated_phase_name(text: str) -> str | None:
    """Faithful port of `stated_phase_name` in crates/specforge/src/ir/evidence.rs."""
    words = text.split()
    for index in range(1, len(words)):
        marker = _trim(words[index], _not_alpha)
        if marker.lower() not in ("phase", "phases"):
            continue
        label = _trim(words[index - 1], _not_label_char)
        if not label or parse_count_word(label) is not None or label.lower() in ("a", "an", "the", "of"):
            continue
        return label
    return None


def main() -> int:
    check_port_against_source()
    evidence = json.loads(EVIDENCE_IR.read_text(encoding="utf-8"))
    statements = evidence["extracted_statements"]
    spans = {span["span_id"]: span for span in evidence["evidence_spans"]}
    anchors = sorted(evidence["section_anchors"], key=lambda anchor: anchor["line_start"])
    gold = json.loads(GOLD.read_text(encoding="utf-8"))
    items = gold["items"] if isinstance(gold, dict) and "items" in gold else gold
    frame_items = [item for item in items if item["task"] == "serial_frame_field"]

    phase_at = [stated_phase_name(statement["text"]) for statement in statements]
    print(f"statements carrying a stated phase name: {sum(1 for p in phase_at if p)} of {len(statements)}")

    def section_for(line: int):
        owner = None
        for anchor in anchors:
            if anchor["line_start"] <= line:
                owner = anchor
            else:
                break
        return owner

    index_of = {statement["statement_id"]: n for n, statement in enumerate(statements)}
    by_id = {statement["statement_id"]: statement for statement in statements}
    stems = {"request": "request", "acknowledge": "ack", "data": "data"}

    correct = wrong = unresolved = 0
    literal = generous = 0
    titles: list[str] = []
    print(f"\n{'field':8s} {'gold':11s} {'inherited':11s} {'dist':>5s}  {'verdict':8s} section title")
    for item in frame_items:
        name = item["gold"][0]["name"]
        gold_phase = str(item["gold"][0].get("phase")).lower()
        position = index_of.get(item["statement_id"])
        if position is None:
            unresolved += 1
            print(f"{name:8s} {gold_phase:11s} {'-':11s} {'-':>5s}  ABSENT")
            continue
        inherited = distance = None
        for back in range(position, -1, -1):
            if phase_at[back]:
                inherited, distance = phase_at[back], position - back
                break
        if inherited is None:
            unresolved += 1
            verdict = "NOPHASE"
        elif inherited.lower() == gold_phase:
            correct += 1
            verdict = "correct"
        else:
            wrong += 1
            verdict = "WRONG"

        statement = by_id[item["statement_id"]]
        span = spans.get(statement["evidence_span_ids"][0]) if statement["evidence_span_ids"] else None
        anchor = section_for(span["line_start"]) if span else None
        title = anchor["title"] if anchor else ""
        titles.append(title)
        if title.lower().startswith(gold_phase) or f" {gold_phase}" in title.lower():
            literal += 1
        if stems[gold_phase] in title.lower():
            generous += 1
        print(f"{name:8s} {gold_phase:11s} {str(inherited):11s} {str(distance):>5s}  {verdict:8s} {title[:52]}")

    total = len(frame_items)
    print(f"\nnearest-preceding-phase rule: correct {correct} / wrong {wrong} / unresolvable {unresolved} of {total}")
    print(f"section-title rule: {literal} of {total} under a literal title-states-the-phase reading, "
          f"{generous} of {total} even counting any appearance of the phase word or its stem")
    print(f"section titles containing the word 'phase': {sum(1 for t in titles if 'phase' in t.lower())} of {len(titles)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
