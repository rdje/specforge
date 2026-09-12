#!/usr/bin/env python3
"""Census the signal declarations that carry no table provenance (read-only).

`ACTOR-NOUN-RELATION-DECLARATION.0`. Two passes mint a `Signal <name> is …` statement
without a `table_signal_declaration_provenance` record: the relation→declaration path
(`synthesize_declarations_from_relations`, the `WIRE-BASED-100.10b` path, which its own
comment calls "the one path that can mint a name the document never declared") and the
prose appositive path. Both are valuable and both are unguarded in the same way.

The tree opened on one instance — AHB declares `Signal Manager is output …`, and `Manager`
is a role in the document's own actor taxonomy — and proposed refusing a name the taxonomy
resolves. This census measures that proposal against the population it would act on, and
against the population it must not touch.

What it reports, per document and per stratum (`EVIDENCE_IR_SCHEMA_VERSION`, read out of
the Rust source so a schema bump fails loudly):

  * `duplicate` — the same signal is also declared from a table, so the statement adds a
    spelling of something already known;
  * `sole source` — no table declares it, so this path is the ONLY reason the signal exists.
    This is what any rule here risks: for two serial-bus specifications it is 100% of their
    signals;
  * `ordinary word` — the name is spelled `Initial-capital + all lower-case`, which no
    document in this corpus uses for a wire;
  * `actor role` — the name resolves through the built-in actor taxonomy, mirrored below.

Both candidate discriminators are reported side by side because they do not select the same
thing, and the difference is the leaf's decision. Shape only, no vocabulary beyond the
taxonomy the product already ships (ADR 0006).

This census classifies a POPULATION and is not a check on the Rust implementation: it
mirrors the taxonomy, so their agreement carries no information (`CLAIM_VERIFICATION.md`
section 2).

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: persisted `generated/evidence_ir/*/evidence_ir.json`.

Usage:
    python3 scripts/measure_untabled_signal_declarations.py
    python3 scripts/measure_untabled_signal_declarations.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

# Verbatim from `builtin_actor_taxonomy_role_in_text` (crates/specforge/src/ir/evidence.rs).
REQUESTER_TERMS = ("manager", "initiator", "master", "requester")
COMPLETER_TERMS = (
    "subordinate", "slave", "responder", "multiplexor", "completer", "target",
)

EVIDENCE_SCHEMA_SOURCE = "crates/specforge/src/ir/evidence.rs"
EVIDENCE_SCHEMA_CONSTANT = "EVIDENCE_IR_SCHEMA_VERSION"
DECLARATION = re.compile(r"^Signal (\S+) is ")


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


def normalize_actor_term(text: str) -> str:
    folded = "".join(
        c if (c.isascii() and (c.isalnum() or c == "_")) else " " for c in text.lower()
    )
    return " ".join(folded.split())


def normalized_text_contains_term(text: str, term: str) -> bool:
    """Whole-token window match, as `normalized_text_contains_term` does."""
    text_tokens, term_tokens = text.split(), term.split()
    if not text_tokens or not term_tokens or len(term_tokens) > len(text_tokens):
        return False
    return any(
        text_tokens[i : i + len(term_tokens)] == term_tokens
        for i in range(len(text_tokens) - len(term_tokens) + 1)
    )


def actor_taxonomy_role(name: str):
    normalized = normalize_actor_term(name)
    if not normalized:
        return None
    requester = any(normalized_text_contains_term(normalized, t) for t in REQUESTER_TERMS)
    completer = any(normalized_text_contains_term(normalized, t) for t in COMPLETER_TERMS)
    if requester and not completer:
        return "requester"
    if completer and not requester:
        return "completer"
    return None


def is_ordinary_word(name: str) -> bool:
    """`Manager`, `Reset`, `In` — an initial capital over an all lower-case remainder.

    Orthography, not vocabulary: the test cannot tell what the word means, only that it is
    spelled the way English prose spells a word rather than the way a document spells a wire.
    """
    return (
        len(name) >= 2
        and name[0].isascii()
        and name[0].isupper()
        and all(c.isascii() and c.islower() for c in name[1:])
    )


def census(root: str) -> dict:
    current_schema = read_schema_constant(root)
    documents = []
    for path in sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json"))):
        key = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        provenance = evidence.get("table_signal_declaration_provenance") or []
        with_provenance = {r["statement_id"] for r in provenance}
        table_declared = {r["signal_name"] for r in provenance}
        rows = []
        for statement in evidence.get("extracted_statements", []):
            if statement["statement_id"] in with_provenance:
                continue
            match = DECLARATION.match(statement["text"])
            if not match:
                continue
            name = match.group(1)
            rows.append({
                "name": name,
                "duplicate": name in table_declared,
                "ordinary_word": is_ordinary_word(name),
                "actor_role": actor_taxonomy_role(name),
                "text": statement["text"][:100],
            })
        if not rows:
            continue
        documents.append({
            "document": key,
            "schema_version": evidence.get("schema_version"),
            "stratum": "current" if evidence.get("schema_version") == current_schema else "legacy",
            "table_declared": sorted(table_declared),
            "rows": rows,
        })
    return {"current_schema": current_schema, "documents": documents}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    result = census(repo_root())

    if args.json:
        json.dump(result, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    print("=== signal declarations carrying no table provenance (read-only) ===")
    for stratum in ("current", "legacy"):
        members = [d for d in result["documents"] if d["stratum"] == stratum]
        rows = [r for d in members for r in d["rows"]]
        duplicate = [r for r in rows if r["duplicate"] and not r["ordinary_word"]]
        word = [r for r in rows if r["ordinary_word"]]
        sole = [r for r in rows if not r["duplicate"] and not r["ordinary_word"]]
        role = [r for r in rows if r["actor_role"]]
        print()
        print(f"--- {stratum.upper()}: {len(rows)} declaration(s) across {len(members)} document(s)")
        print(f"      duplicate of a table declaration : {len(duplicate)}")
        print(f"      SOLE source of that signal       : {len(sole)}")
        print(f"      spelled like an ordinary word    : {len(word)}")
        print(f"      resolves to an actor role        : {len(role)}")
        for document in sorted(members, key=lambda d: d["document"]):
            counts = collections.Counter(
                "word" if r["ordinary_word"] else "dup" if r["duplicate"] else "sole"
                for r in document["rows"]
            )
            print(f"      {document['document'][:44]:46s} "
                  f"dup={counts['dup']:3d} sole={counts['sole']:3d} word={counts['word']}")
        if word:
            print("      == names spelled like an ordinary word")
            for document in members:
                for r in document["rows"]:
                    if r["ordinary_word"]:
                        print(f"         {document['document'][:30]:32s} {r['name']:14s} "
                              f"role={r['actor_role']}  {r['text']!r}")
    # The cost side: no rule here may touch a name a table declares.
    table_names = {n for d in result["documents"] for n in d["table_declared"]}
    print()
    print(f"cost side — distinct table-declared names corpus-wide: {len(table_names)}; "
          f"spelled like an ordinary word: {sum(1 for n in table_names if is_ordinary_word(n))}; "
          f"resolving to an actor role: {sum(1 for n in table_names if actor_taxonomy_role(n))}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
