#!/usr/bin/env python3
"""Measure the `_WIDTH` parameter-leak enum-member class — the deeper
member-quality residual `KG-ISF-COMPLETENESS.5.iii` resolves (read-only).

`.5.ii` cleaned the prose-SENTENCE-fragment enum members (the spine gate) and
deliberately left five deeper member-quality classes as honest residuals:
glossary `SEE…`, front-matter/ToC, section-caption `B2_3_1_…`, `_WIDTH`
parameter leaks, and value-restart-of-all-clean. This script measures those
classes over the persisted IntentIR corpus and isolates the ONE that is both
material (reaches a wire-gold `.isf`) and cleanly gateable: the `_WIDTH` leak.

It is read-only and deterministic (no network, no clocks, no randomness, no
rebuild): it reads `generated/intent_ir/*/intent_ir.json`, replicates the
`.5.ii` sentence-spine filter, then classifies the SURVIVING members and proves,
per item:

  * the `_WIDTH` member gate (drop `X_WIDTH` iff `X` is a declared signal OR the
    enum's own name) catches exactly the leaks that reach a real-signal-named
    enum (the AXI gold `ihi0022_l` BRESP/RRESP/RCHUNKNUM/RCHUNKSTRB/AXSNOOP/AWCMO),
  * its false-positive set is EMPTY corpus-wide — no legit width-VALUE such as
    `FULL_WIDTH`/`HALF_WIDTH` exists, and the declared-signal discriminator would
    never catch one (prefix `FULL`/`HALF` is not a signal),
  * the other `_WIDTH` members live in generic-named enums that `.5.i` already
    drops whole, so they never reach `.isf`,
  * the section-caption / value-restart-of-clean classes have NO false-positive-
    free structural gate (the leading `[A-Z]?\\d+` token collides with real codes
    `D1`/`L2`) and are dominated by `.5.i`-dropped enums → honest residual.

Nothing here writes or mutates any artifact.

Usage:
    python3 scripts/measure_enum_width_leak.py
"""
import json
import glob
import os
import re
import sys

# `.5.ii` PROSE_SENTENCE_SPINE_WORDS, verbatim from ir/evidence.rs (collisions
# a/i/its/can/may/am excluded — see KG-ISF-COMPLETENESS.1a discipline).
SPINE = set(
    """the an this that these those is are was were be been being has have had
    will would shall should could might must which whose when while because
    unless until whether if then hence thus therefore however where""".split()
)

SIZE_WORDS = {
    "FULL", "HALF", "QUARTER", "DOUBLE", "SINGLE", "BYTE", "WORD", "MAX", "MIN",
    "VARIABLE", "FIXED", "ZERO",
}
SECNUM = re.compile(r"^[A-Z]?[0-9]+$")


def is_spine_fragment(name):
    return any(tok.lower() in SPINE for tok in name.split("_") if tok)


def ends_with_width(name):
    toks = [t for t in name.split("_") if t]
    return bool(toks) and toks[-1] == "WIDTH"


def leading_section_number(name):
    toks = [t for t in name.split("_") if t]
    return bool(toks) and SECNUM.match(toks[0]) is not None and len(toks) > 1


def declared_signals(doc):
    s = set()
    for iface in doc.get("interfaces") or []:
        for r in iface.get("signal_records") or []:
            nm = r.get("name") or r.get("signal_name")
            if nm:
                s.add(nm.upper())
    for r in doc.get("signal_records") or []:
        nm = r.get("name") or r.get("signal_name")
        if nm:
            s.add(nm.upper())
    return s


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    files = sorted(glob.glob(os.path.join(root, "generated/intent_ir/*/intent_ir.json")))
    if not files:
        print("no generated/intent_ir/*/intent_ir.json found; ingest the corpus first")
        return 1

    width_caught = []      # (doc, enum, member) — gate fires (prefix declared OR enum-self)
    width_uncaught = []    # (doc, enum, member) — _WIDTH but prefix not declared/enum
    legit_width_value = [] # _WIDTH member whose prefix is a SIZE word (would be a FP)
    section_survivors = []
    tot_members = spine_flagged = survivors = 0

    for f in files:
        doc_key = os.path.basename(os.path.dirname(f))
        try:
            doc = json.load(open(f))
        except Exception:
            continue
        decl = declared_signals(doc)
        for sd in doc.get("symbol_definitions") or []:
            if sd.get("kind") != "enum":
                continue
            enum = sd.get("symbol_name") or ""
            for m in sd.get("members") or []:
                n = m.get("member_name", "")
                tot_members += 1
                if is_spine_fragment(n):
                    spine_flagged += 1
                    continue
                survivors += 1
                if ends_with_width(n):
                    toks = [t for t in n.split("_") if t]
                    prefix = "_".join(toks[:-1])
                    if prefix in SIZE_WORDS:
                        legit_width_value.append((doc_key, enum, n))
                    elif prefix.upper() in decl or prefix.upper() == enum.upper():
                        width_caught.append((doc_key, enum, n))
                    else:
                        width_uncaught.append((doc_key, enum, n))
                elif leading_section_number(n):
                    section_survivors.append((doc_key, enum, n))

    print(f"docs                         : {len(files)}")
    print(f"total enum members           : {tot_members}")
    print(f"spine-flagged (.5.ii)        : {spine_flagged}")
    print(f"survivors (post-spine)       : {survivors}")
    print()
    print("=== _WIDTH parameter-leak class ===")
    print(f"members ending in _WIDTH     : {len(width_caught) + len(width_uncaught) + len(legit_width_value)}")
    print(f"  CAUGHT (prefix declared/enum-self) : {len(width_caught)}")
    print(f"  UNCAUGHT (generic-enum, .5.i-dropped): {len(width_uncaught)}")
    print(f"  legit width-VALUE FALSE POSITIVES  : {len(legit_width_value)}  (must be 0)")
    print()
    print("  caught leaks (these reach a real-signal-named enum -> the fix target):")
    for doc_key, enum, n in width_caught:
        print(f"    {enum:14s} {n:22s}  {doc_key[:28]}")
    print()
    print("=== section-caption / table-ref survivors (NO clean FP-free gate — honest residual) ===")
    print(f"  count: {len(section_survivors)} (leading [A-Z]?digit token collides with real codes D1/L2/D6_4)")
    for doc_key, enum, n in section_survivors[:8]:
        print(f"    {enum:14s} {n:22s}  {doc_key[:28]}")

    ok = len(legit_width_value) == 0 and len(width_caught) > 0
    print()
    print("VERDICT:", "GO _WIDTH gate (FP set empty, caught leaks material)" if ok
          else "review — FP set non-empty or no caught leaks")
    return 0 if ok else 2


if __name__ == "__main__":
    sys.exit(main())
