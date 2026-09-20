#!/usr/bin/env python3
"""Which identifier-shaped section headings does a DIRECTION-BEARING body license? (read-only)

`SIGNAL-CATALOG-CAPTURE-GAP.2`. `.1` established that exactly one of the 33 empty-catalog documents
is a capture miss, and located its declarations: Wishbone declares its interface as a
**heading-as-declaration convention** — 32 sections whose title IS the wire name (`CLK_I`, `RST_O`,
`DAT_I()`, …), each followed by prose giving direction and meaning. SpecForge reads declarations
from tables and from a formal prose predicate; this document uses neither.

`.1` also proved the title SHAPE alone cannot become the product rule, and named the documents that
prove it: USB 3.2's 13 compound headings are hub port-feature selectors, the SMMU software guide's
three are register names, ADIv6's two are `IMPLEMENTATION_DEFINED` boilerplate, AMBA DTI's are
message names, Bosch CAN's twelve are section labels, and another document's are link states. A
rule keyed on the title alone would declare every one of them a wire.

So this census measures the SHAPE and a candidate DISCRIMINATOR separately, and reports the
selection each makes, so the discriminator can be read rather than assumed:

  * SHAPE  — a `document_sections` title that is a bare identifier token, the `.1` predicate
    `^[A-Z][A-Z0-9_]{1,23}(\\(\\)|\\[.*\\])?$`, counted separately for COMPOUND titles (containing
    `_` or an array/call suffix), which is the form a wire name takes.
  * BODY   — the section's own content elements state a DIRECTION for that identifier: the closed
    direction vocabulary the declaration reader already owns, full words only, no abbreviation
    (`SIGNAL-DECLARATION-ROW-DROP.2h.0` measured `i`/`o`/`in`/`out` at 0 true positives and 18 false
    ones, and `.2j` re-refused them corpus-wide).

The body is the content elements between this heading and the next, in reading order — the
document's own sectioning, not a window of N elements, so a long section is not truncated and a
short one does not borrow its neighbour's prose.

Read-only and deterministic: no network, no clock, no randomness, no write.

Usage:
    python3 scripts/measure_heading_declaration_shape.py            # the census
    python3 scripts/measure_heading_declaration_shape.py --titles   # every selected heading
    python3 scripts/measure_heading_declaration_shape.py --json
    python3 scripts/measure_heading_declaration_shape.py --self-test
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from build_bounded_decision_baseline import repo_root  # noqa: E402

# `SIGNAL-CATALOG-CAPTURE-GAP.1`'s probe B, verbatim.
IDENTIFIER_TITLE = re.compile(r"^[A-Z][A-Z0-9_]{1,23}(\(\)|\[[^\]]*\])?$")

# The declaration reader's own direction words (`LITERAL_DIRECTION_CELL_VALUES`), full words only.
DIRECTION_WORDS = ("input", "output", "inout")


def normalise(text: str | None) -> str:
    return " ".join((text or "").split())


def is_identifier_title(title: str) -> bool:
    return bool(IDENTIFIER_TITLE.match(normalise(title)))


def is_compound(title: str) -> bool:
    """A wire name's shape: an underscore, or an array/call suffix."""
    title = normalise(title)
    return "_" in title or title.endswith("()") or title.endswith("]")


def states_a_direction(body: str) -> bool:
    """Whole-word only: `outputs` and `inputs` are prose about a class, not this wire's sense."""
    lowered = body.lower()
    return any(re.search(rf"\b{word}\b", lowered) for word in DIRECTION_WORDS)


def section_bodies(source: dict) -> list[tuple[str, str]]:
    """Every (title, body) pair, the body being this heading's own elements in reading order."""
    elements = sorted(
        source.get("content_elements") or [],
        key=lambda element: element.get("reading_order") or 0,
    )
    pairs: list[tuple[str, list[str]]] = []
    for element in elements:
        if element.get("kind") == "section_header":
            pairs.append((normalise(element.get("text")), []))
        elif pairs:
            pairs[-1][1].append(normalise(element.get("text")))
    return [(title, " ".join(body)) for title, body in pairs]


def census(root: str) -> dict:
    documents = []
    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        shape, compound, licensed, titles = 0, 0, 0, []
        for title, body in section_bodies(source):
            if not is_identifier_title(title):
                continue
            shape += 1
            compound_title = is_compound(title)
            compound += 1 if compound_title else 0
            directed = states_a_direction(body)
            licensed += 1 if (compound_title and directed) else 0
            titles.append({
                "title": title,
                "compound": compound_title,
                "states_a_direction": directed,
                "body_head": body[:110],
            })
        if shape:
            documents.append({
                "document": document,
                "shape": shape,
                "compound": compound,
                "licensed": licensed,
                "titles": titles,
            })
    documents.sort(key=lambda entry: (-entry["licensed"], -entry["compound"], entry["document"]))
    return {"documents": documents}


def summarise(result: dict) -> dict:
    documents = result["documents"]
    return {
        "documents_with_any_identifier_heading": len(documents),
        "shape_headings": sum(entry["shape"] for entry in documents),
        "compound_headings": sum(entry["compound"] for entry in documents),
        "licensed_headings": sum(entry["licensed"] for entry in documents),
        "licensed_documents": len([e for e in documents if e["licensed"]]),
    }


def render(result: dict, show_titles: bool) -> None:
    summary = summarise(result)
    print("=== SIGNAL-CATALOG-CAPTURE-GAP.2 — heading-as-declaration shape vs discriminator ===")
    print(f"  documents with any identifier-shaped heading  {summary['documents_with_any_identifier_heading']}")
    print(f"  SHAPE: identifier-shaped headings             {summary['shape_headings']}")
    print(f"  …of those COMPOUND (a wire name's form)       {summary['compound_headings']}")
    print(f"  …and whose own section states a DIRECTION     {summary['licensed_headings']}"
          f"  across {summary['licensed_documents']} document(s)")
    print()
    print("  per document — shape / compound / licensed:")
    for entry in result["documents"]:
        print(f"    {entry['document'][:52]:54s} {entry['shape']:4d} {entry['compound']:4d} {entry['licensed']:4d}")
    if not show_titles:
        print("\n  --titles prints every selected heading with its body head, which is what a rule")
        print("  of this kind has to be adjudicated on.")
        return
    for entry in result["documents"]:
        if not entry["compound"]:
            continue
        print(f"\n-- {entry['document']}")
        for title in entry["titles"]:
            if not title["compound"]:
                continue
            mark = "LICENSED" if title["states_a_direction"] else "refused "
            print(f"   {mark} {title['title']:26s} {title['body_head']}")


def self_test() -> int:
    failures: list[str] = []
    passed = 0

    def check(case: str, condition: bool) -> None:
        nonlocal passed
        if condition:
            passed += 1
        else:
            failures.append(case)

    # RED 1 — the shape is `.1`'s, unchanged: a bare identifier, optionally with an array or call
    # suffix, and nothing else.
    check("shape-accepts-a-wire-name", is_identifier_title("CLK_I"))
    check("shape-accepts-a-call-suffix", is_identifier_title("DAT_I()"))
    check("shape-refuses-a-sentence", not is_identifier_title("Clock input"))
    check("shape-refuses-lower-case", not is_identifier_title("clk_i"))
    # RED 2 — COMPOUND is the wire-name form and is what separates `CLK_I` from a bare word.
    check("compound-needs-an-underscore-or-suffix", is_compound("RST_O") and not is_compound("RUN"))
    # RED 3 — the direction test is WHOLE-WORD. A section about "outputs" in general does not state
    # this wire's sense, and admitting it is how a prose rule over-fires.
    check("direction-is-whole-word", states_a_direction("This is an output of the MASTER."))
    check("direction-refuses-a-plural-class", not states_a_direction("The outputs are described below."))
    check("direction-refuses-silence", not states_a_direction("Indicates a valid data transfer cycle."))
    # RED 4 — a body is its OWN section's elements: a heading with no body borrows nothing from the
    # section before or after it.
    source = {"content_elements": [
        {"kind": "section_header", "text": "AAA_I", "reading_order": 1},
        {"kind": "paragraph", "text": "No sense stated here.", "reading_order": 2},
        {"kind": "section_header", "text": "BBB_O", "reading_order": 3},
        {"kind": "paragraph", "text": "This is an output.", "reading_order": 4},
    ]}
    bodies = dict(section_bodies(source))
    check("body-is-its-own-section", not states_a_direction(bodies["AAA_I"])
          and states_a_direction(bodies["BBB_O"]))

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"heading-declaration shape: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--titles", action="store_true", help="print every selected heading")
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    root = repo_root()
    result = census(root)
    if args.json:
        json.dump({"summary": summarise(result), **result}, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result, args.titles)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
