#!/usr/bin/env python3
"""Census how a statement becomes an invariant, and what shape the result has (read-only).

`ANCHORLESS-INVARIANT-DROP.0`, which opened on a wrong premise and corrected it here.

`ACTOR-NOUN-RELATION-DECLARATION.1` removed ADIv6's phantom signal `In`, and 13 SemanticIR
invariants disappeared with it. The obvious reading is that they lost their interface
anchor. They did not: **560 of ADIv6's 593 invariants carry no anchor at all** and survive
perfectly well. What the phantom was supplying is an ADMISSION GATE.

`is_invariant_like` (crates/specforge/src/ir/semantic.rs) admits a statement by three routes:

  r1  a modal phrase — `must`, `shall`, `always`, `never`, `until`, … No signal needed.
  r2  the statement MENTIONS A DECLARED SIGNAL *and* carries a weak phrase — `handshake`,
      `asserted`, `deasserted`, `transition`, `state`, `timing`, `observed`.
  r3  related visual evidence whose role is Normative or Ambiguous.

The 13 were r2: every one contains the word `state`, and the "declared signal" they mentioned
was the phantom `In`. **A phantom declaration does not only add phantom records — it widens
r2 for every sentence containing that word.**

Measuring r2 exposed something larger, which is why this census reports SHAPE as well as
route. An `InvariantRecord` is meant to be a normative constraint and flows to IntentIR
`constraints`, the product boundary. A quarter of them are not statements:

  * a `markdown table row` — text beginning with `|`, a serialized table row. Many are the
    very signal-description rows the declaration reader already turned into
    `Signal X is …`, so they are published twice: once as a declaration and once as markup.
  * a `figure/table caption or cross-reference` — text beginning `Figure <n>` / `Table <n>`,
    up to and including a constraint whose entire text is `Figure 1.`

Both classifications are shape-only and exact: a leading `|`, or a leading figure/table
label followed by a number. The census prints samples so the call is adjudicated rather than
trusted, and reports how many carry a modal verb, since that is the one reading under which
a table row might still be a requirement.

This census classifies a POPULATION. It is not a check on the Rust implementation: it mirrors
`is_invariant_like`'s phrase lists, so their agreement carries no information
(`CLAIM_VERIFICATION.md` section 2). Route attribution is derived from the persisted statement
text alone, so r2 and r3 are distinguished by elimination — a statement with no modal and no
weak phrase must have been admitted by visual evidence.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/semantic_ir/*/semantic_ir.json` and
`generated/intent_ir/*/intent_ir.json`, stratified by `generated/evidence_ir/*`.

Usage:
    python3 scripts/measure_invariant_admission_shape.py
    python3 scripts/measure_invariant_admission_shape.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

# Verbatim from `is_invariant_like` (crates/specforge/src/ir/semantic.rs).
MODAL_PHRASES = (
    "must", "shall", "always", "never", "required", "remains", "remain", "until",
    "only when", "cannot", "must not", "shall not",
)
WEAK_PHRASES = (
    "handshake", "asserted", "deasserted", "transition", "state", "timing", "observed",
)

EVIDENCE_SCHEMA_SOURCE = "crates/specforge/src/ir/evidence.rs"
EVIDENCE_SCHEMA_CONSTANT = "EVIDENCE_IR_SCHEMA_VERSION"
CAPTION = re.compile(r"^(Figure|Table)\s+[A-Za-z]?[0-9]")


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


def contains_phrase(text: str, phrase: str) -> bool:
    """`contains_phrase`: whole-word containment; `text` is already lower-case."""
    start = 0
    while True:
        index = text.find(phrase, start)
        if index < 0:
            return False
        before = text[index - 1] if index > 0 else ""
        after_index = index + len(phrase)
        after = text[after_index] if after_index < len(text) else ""
        if not (before.isascii() and before.isalnum()) and not (
            after.isascii() and after.isalnum()
        ):
            return True
        start = index + 1


def contains_any_phrase(text: str, phrases) -> bool:
    return any(contains_phrase(text, phrase) for phrase in phrases)


def admission_route(statement: str) -> str:
    lowered = statement.lower()
    if contains_any_phrase(lowered, MODAL_PHRASES):
        return "r1_modal"
    if contains_any_phrase(lowered, WEAK_PHRASES):
        return "r2_weak_phrase_and_declared_signal"
    return "r3_visual_evidence"


def statement_shape(statement: str) -> str:
    stripped = statement.strip()
    if stripped.startswith("|"):
        return "markdown_table_row"
    if CAPTION.match(stripped):
        return "caption_or_cross_reference"
    return "prose"


def constraint_text(record: dict) -> str:
    for key in ("statement", "text", "constraint_text"):
        value = record.get(key)
        if value:
            return value
    return ""


def census(root: str) -> dict:
    current_schema = read_schema_constant(root)
    strata = collections.defaultdict(lambda: {
        "semantic": collections.Counter(),
        "intent": collections.Counter(),
        "documents": collections.Counter(),
        "samples": collections.defaultdict(list),
    })
    for path in sorted(glob.glob(os.path.join(root, "generated/semantic_ir/*/semantic_ir.json"))):
        key = os.path.basename(os.path.dirname(path))
        evidence_path = os.path.join(root, "generated/evidence_ir", key, "evidence_ir.json")
        if not os.path.exists(evidence_path):
            continue
        with open(evidence_path, "r", encoding="utf-8") as handle:
            schema = json.load(handle).get("schema_version")
        stratum = "current" if schema == current_schema else "legacy"
        bucket = strata[stratum]
        with open(path, "r", encoding="utf-8") as handle:
            semantic = json.load(handle)
        for invariant in semantic.get("invariants", []):
            route = admission_route(invariant["statement"])
            shape = statement_shape(invariant["statement"])
            bucket["semantic"][(route, shape)] += 1
        intent_path = os.path.join(root, "generated/intent_ir", key, "intent_ir.json")
        if not os.path.exists(intent_path):
            continue
        with open(intent_path, "r", encoding="utf-8") as handle:
            intent = json.load(handle)
        for record in intent.get("constraints", []):
            text = constraint_text(record)
            if not text:
                continue
            shape = statement_shape(text)
            bucket["intent"][shape] += 1
            if shape != "prose":
                bucket["documents"][key] += 1
                if len(bucket["samples"][shape]) < 8:
                    bucket["samples"][shape].append({"document": key, "text": text[:110]})
            if contains_any_phrase(text.lower(), MODAL_PHRASES):
                bucket["intent"][(shape, "with_modal")] += 1
    return {"current_schema": current_schema, "strata": strata}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    result = census(repo_root())

    if args.json:
        payload = {"current_evidence_schema_version": result["current_schema"], "strata": {}}
        for stratum, bucket in result["strata"].items():
            payload["strata"][stratum] = {
                "semantic_by_route_and_shape": {
                    f"{route}/{shape}": count
                    for (route, shape), count in sorted(bucket["semantic"].items())
                },
                "intent_by_shape": {
                    (k if isinstance(k, str) else f"{k[0]}/{k[1]}"): v
                    for k, v in sorted(bucket["intent"].items(), key=lambda i: str(i[0]))
                },
                "worst_documents": bucket["documents"].most_common(10),
                "samples": {k: v for k, v in bucket["samples"].items()},
            }
        json.dump(payload, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    print("=== invariant admission route and published shape (read-only) ===")
    for stratum in ("current", "legacy"):
        bucket = result["strata"].get(stratum)
        if not bucket:
            continue
        semantic = bucket["semantic"]
        total = sum(semantic.values())
        print()
        print(f"--- {stratum.upper()} stratum: {total} SemanticIR invariant(s)")
        for route in ("r1_modal", "r2_weak_phrase_and_declared_signal", "r3_visual_evidence"):
            rows = {s: c for (r, s), c in semantic.items() if r == route}
            subtotal = sum(rows.values())
            if not subtotal:
                continue
            non_prose = subtotal - rows.get("prose", 0)
            print(f"    {route:38s} {subtotal:6d}  non-prose {non_prose:5d} "
                  f"(table rows {rows.get('markdown_table_row', 0)}, "
                  f"captions {rows.get('caption_or_cross_reference', 0)})")
        intent = bucket["intent"]
        published = sum(v for k, v in intent.items() if isinstance(k, str))
        if published:
            rows = intent.get("markdown_table_row", 0)
            captions = intent.get("caption_or_cross_reference", 0)
            print(f"    IntentIR constraints (the product boundary): {published}")
            print(f"        markdown table rows                    : {rows} "
                  f"({intent.get(('markdown_table_row', 'with_modal'), 0)} carry a modal verb)")
            print(f"        figure/table captions or cross-refs    : {captions} "
                  f"({intent.get(('caption_or_cross_reference', 'with_modal'), 0)} carry a modal verb)")
            print(f"        NOT a statement                        : {rows + captions} "
                  f"({100.0 * (rows + captions) / published:.1f}%)")
            print("    worst documents:")
            for document, count in bucket["documents"].most_common(6):
                print(f"        {count:5d}  {document[:46]}")
            for shape, samples in sorted(bucket["samples"].items()):
                print(f"    == {shape} sample")
                for sample in samples:
                    print(f"        {sample['document'][:22]:24s} {sample['text']!r}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
