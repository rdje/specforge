#!/usr/bin/env python3
"""Census the words that stop the logic-level walk while a declared signal sits just
beyond them (read-only).

`EXTRACTION-QUALITY-GAUGE.3k.12` — `logic_level_bindings` finds a level token within
`MAX_GAP` words after a binding verb and walks BACKWARD to the identifier it belongs to,
skipping the scaffolding English puts between them (`the`, `its`, `input`, `signal`, a
subscript, the verb itself). Anything else stops the walk, and the binding is lost.

The leaf asked whether PREDICATE ADJECTIVES — `absent`, `present`, `unused`, `reserved` —
are a class the walk should cross, after AMBA LPI's *"with the QDENY output absent or tied
low"* failed to bind while its twin one figure away bound. **Size it before changing the
walk** was the instruction, and this is that sizing.

It reports, for every persisted statement and constraint source text in the corpus, each
word that stopped the walk while a DECLARED signal sat within three further words — the
cases where crossing would actually recover something — with the exact window printed for
individual adjudication. It proposes nothing: a skip list tuned to one sentence is a mirror
of that sentence, which is the standing finding this leaf inherited.

It mirrors `logic_level_bindings`' own constants — `BIND_VERBS`, `MAX_GAP`, `DESCRIPTORS`,
and `token_logic_level` via `LOGIC_HIGH_VALUES`/`LOGIC_LOW_VALUES` — and is therefore an
instrument for the POPULATION, never a check on the Rust (`CLAIM_VERIFICATION.md` section 2).
The independent leg is the in-crate control suite.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: every persisted `generated/evidence_ir/*/evidence_ir.json`, both strata, because
a blocked walk is a property of a SENTENCE and the sentence is the same whichever producer
wrote the artifact around it. The declared-signal set is each document's own.

Usage:
    python3 scripts/measure_logic_level_walk_blockers.py
    python3 scripts/measure_logic_level_walk_blockers.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import itertools
import json
import os
import re
import sys

# Verbatim from `logic_level_bindings` in crates/specforge/src/ir/evidence.rs.
BIND_VERBS = ("drive", "driven", "drives", "set", "sets", "tied", "held", "pulled", "forced")
MAX_GAP = 6
DESCRIPTORS = (
    "a", "an", "the", "its", "their", "all", "both", "of", "to", "be", "is", "are", "was",
    "were", "must", "shall", "input", "inputs", "output", "outputs", "signal", "signals",
    "bit", "bits", "pin", "pins", "value", "and", "or",
)
# `LOGIC_HIGH_VALUES` / `LOGIC_LOW_VALUES` in crates/specforge/src/ir/normative_vocab.rs.
LOGIC_HIGH_VALUES = ("1", "1'b1", "high", "hi", "true")
LOGIC_LOW_VALUES = ("0", "1'b0", "low", "lo", "false")
# How far past the blocker a declared signal may sit and still count as recoverable. Three
# words is the width of the scaffolding the walk already skips; beyond it a "recovery" is a
# different clause, which is the thing the walk exists to refuse.
REACH_WINDOW = 4

DECLARATION = re.compile(r"^Signal (\S+) is ")


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def token_logic_level(word: str) -> str | None:
    """`token_logic_level`: the whole token, or its leading uppercase run. Alphabetic only."""
    run = "".join(itertools.takewhile(str.isupper, word))
    for candidate in (word.lower(), run.lower()):
        if len(candidate) < 2 or not candidate.isalpha():
            continue
        if candidate in LOGIC_HIGH_VALUES:
            return "MustBeHigh"
        if candidate in LOGIC_LOW_VALUES:
            return "MustBeLow"
    return None


def words_of(text: str) -> list[str]:
    """The reader's tokenization: an opaque identifier is one grammar unit."""
    return [word for word in re.split(r"[^A-Za-z0-9_]", text) if word]


def declared_signals(evidence: dict) -> set[str]:
    names = {
        match.group(1)
        for statement in evidence.get("extracted_statements") or []
        if (match := DECLARATION.match(statement["text"].strip()))
    }
    names |= {
        record["signal_name"]
        for record in evidence.get("table_signal_declaration_provenance") or []
    }
    return names


def blockers_in(text: str, declared: set[str]) -> list[dict]:
    words = words_of(text)
    binds = [i for i, word in enumerate(words) if word.lower() in BIND_VERBS]
    if not binds:
        return []
    found = []
    for index, word in enumerate(words):
        if token_logic_level(word) is None:
            continue
        if index >= 1 and words[index - 1].lower() == "active":
            continue
        if not any(index > bind and index - bind <= MAX_GAP for bind in binds):
            continue
        cursor, bound, blocker, blocker_at = index, False, None, None
        while cursor > 0:
            cursor -= 1
            candidate = words[cursor]
            if token_logic_level(candidate) is not None:
                break
            if candidate in declared:
                bound = True
                break
            lowered = candidate.lower()
            if lowered.isdigit() or lowered in DESCRIPTORS or lowered in BIND_VERBS:
                continue
            blocker, blocker_at = candidate, cursor
            break
        if bound or blocker is None:
            continue
        reach, step = None, blocker_at
        while step > 0 and blocker_at - step < REACH_WINDOW - 1:
            step -= 1
            if words[step] in declared:
                reach = words[step]
                break
            if token_logic_level(words[step]) is not None:
                break
        if reach is None:
            continue
        low = max(0, blocker_at - 6)
        found.append(
            {
                "blocker": blocker,
                "would_reach": reach,
                "window": " ".join(words[low : index + 2]),
            }
        )
    return found


def census(root: str) -> dict:
    statements = 0
    cases: dict[tuple[str, str, str], dict] = {}
    for path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        document = os.path.basename(os.path.dirname(path))
        declared = declared_signals(evidence)
        texts = [s["text"] for s in evidence.get("extracted_statements") or []]
        texts += [
            c["source_text"]
            for c in evidence.get("signal_constraints") or []
            if c.get("source_text")
        ]
        for text in texts:
            statements += 1
            for hit in blockers_in(text, declared):
                key = (hit["blocker"].lower(), document, hit["window"])
                cases.setdefault(key, {"document": document, **hit})
    return {"statements_scanned": statements, "cases": list(cases.values())}


def render(result: dict) -> None:
    cases = result["cases"]
    print("=== logic-level walk blockers (read-only, persisted artifacts) ===")
    print(f"  statements and constraint source texts scanned : {result['statements_scanned']}")
    print(f"  MAX_GAP={MAX_GAP}  REACH_WINDOW={REACH_WINDOW}")
    print(f"  distinct cases where crossing would reach a declared signal: {len(cases)}")
    by_blocker = collections.Counter(case["blocker"].lower() for case in cases)
    print(f"  distinct blocking words: {len(by_blocker)}")
    print()
    for blocker, count in by_blocker.most_common():
        print(f"== {blocker!r} ({count})")
        for case in cases:
            if case["blocker"].lower() != blocker:
                continue
            print(f"   [{case['document'][:26]:26s}] -> {case['would_reach']}:  …{case['window']}…")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    result = census(repo_root())
    if args.json:
        json.dump(result, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
