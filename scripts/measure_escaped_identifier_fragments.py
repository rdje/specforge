#!/usr/bin/env python3
"""Census the Markdown backslash-escape that fragments identifiers in EvidenceIR text.

`EXTRACTION-QUALITY-GAUGE.3k.9`.

Docling renders an underscore inside an identifier as the Markdown escape `\\_`, which is CORRECT
Markdown. SourceIR never carries it — SourceIR stores structured table cells — but every text the
evidence stage takes from the normalized bundle does, so `PARTITION\\_ACCESS` reaches EvidenceIR and
the identifier tokenizers, which split on any character that is not alphanumeric-or-underscore, cut it
at the backslash and yield `PARTITION`.

Three populations, and they are NOT the same size. The leaf was opened on the third and sized on the
first, which is why it needed re-deriving:

  1. TEXT      — statements carrying an escaped identifier at all. Wide, and mostly harmless: a
                 register name in a section title is provenance, not intent.
  2. CATALOG   — names the document DECLARES that exist only as the head of an escaped compound.
                 This is the one that matters, because a declared name is what every subject gate,
                 every polarity pass and every relation reader treats as authority.
  3. RECORDS   — published constraint records whose subject is such a name.

**The discriminator has a trap, and it is the reason this census exists rather than a one-line
grep.** The repository's standard "standalone wins" idiom — a candidate that occurs even once outside
the suspect position is never touched (`.3g`, `.3h`) — is CIRCULAR here. Fragmentation mints
`Signal PARTITION is width 1.` and `Enum PARTITION NOT_DEFINED = 0.` into the same statement set, and
those synthesized sentences then count as standalone occurrences of `PARTITION`. The contamination
manufactures its own evidence of innocence. This census therefore tests standalone-ness over PROSE
statements only, excluding the synthesized `Signal …` / `Enum …` forms.

This census classifies a POPULATION by reading persisted artifacts. It mirrors no extractor rule: the
declarations it counts are already materialized statements, and the escape is a literal. It is NOT a
check on the Rust implementation.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: the persisted `generated/evidence_ir/*/evidence_ir.json` corpus.

Usage:
    python3 scripts/measure_escaped_identifier_fragments.py
    python3 scripts/measure_escaped_identifier_fragments.py --json
    python3 scripts/measure_escaped_identifier_fragments.py --self-test
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

ESCAPED = re.compile(r"([A-Za-z0-9]+)\\_([A-Za-z0-9\\_]+)")
DECLARATION = re.compile(r"^Signal ([A-Za-z0-9_]+) is")
SYNTHESIZED = re.compile(r"^(Signal |Enum )")
SELF_TEST_CASES = 8


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def declared_names(statements: list[str]) -> set[str]:
    names = set()
    for text in statements:
        match = DECLARATION.match(text)
        if match:
            names.add(match.group(1))
    return names


def prose_only(statements: list[str]) -> list[str]:
    """Statements the document itself contains, excluding this stage's own synthesized ones.

    Excluding them is the whole point: a fragment's own `Signal X is …` declaration would otherwise
    count as a standalone occurrence of X and clear the fragment that created it.
    """
    return [text for text in statements if not SYNTHESIZED.match(text)]


def escaped_heads(prose: str) -> dict[str, set[str]]:
    heads: dict[str, set[str]] = collections.defaultdict(set)
    for match in ESCAPED.finditer(prose):
        heads[match.group(1)].add(match.group(0).replace("\\_", "_"))
    return heads


def occurs_standalone(prose: str, name: str) -> bool:
    pattern = re.compile(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])")
    for match in pattern.finditer(prose):
        if prose[match.end() : match.end() + 2] != "\\_":
            return True
    return False


def spurious_heads(statements: list[str], *, exclude_synthesized: bool = True) -> dict[str, list[str]]:
    """Declared names that exist ONLY as the head of an escaped compound, with their compounds.

    This is the whole classification, in one place, so the self-test can run the REAL decision rather
    than its parts. `exclude_synthesized=False` exists only for that self-test's RED case: it is the
    circular reading this census was re-derived to escape, and it must report nothing.
    """
    names = declared_names(statements)
    prose = "\n".join(prose_only(statements) if exclude_synthesized else statements)
    heads = escaped_heads(prose)
    return {
        head: sorted(compounds)
        for head, compounds in heads.items()
        if head in names and not occurs_standalone(prose, head)
    }


def census(root: str) -> dict:
    documents = sorted(glob.glob(os.path.join(root, "generated/evidence_ir/*/evidence_ir.json")))
    text_documents = 0
    catalog_rows: list[dict] = []
    record_rows: list[dict] = []
    for path in documents:
        key = os.path.basename(os.path.dirname(path))
        with open(path, encoding="utf-8") as handle:
            artifact = json.load(handle)
        statements = [s["text"] for s in artifact.get("extracted_statements", [])]
        if escaped_heads("\n".join(prose_only(statements))):
            text_documents += 1
        spurious = spurious_heads(statements)
        for head, compounds in sorted(spurious.items()):
            catalog_rows.append({"document": key, "name": head, "compounds": compounds})
        for record in artifact.get("signal_constraints", []):
            if record["subject_signal"] in spurious:
                record_rows.append(
                    {
                        "document": key,
                        "constraint_id": record["constraint_id"],
                        "subject_signal": record["subject_signal"],
                    }
                )
    return {
        "documents_scanned": len(documents),
        "text_documents": text_documents,
        "catalog_documents": len({row["document"] for row in catalog_rows}),
        "catalog_names": len(catalog_rows),
        "catalog_rows": catalog_rows,
        "record_rows": record_rows,
    }


def report(result: dict) -> None:
    print("command: measure_escaped_identifier_fragments")
    print(f"documents_scanned: {result['documents_scanned']}")
    print(f"text_population: {result['text_documents']} documents carry an escaped identifier")
    print(
        f"catalog_population: {result['catalog_names']} declared name(s) across "
        f"{result['catalog_documents']} document(s) exist ONLY as the head of an escaped compound"
    )
    print(f"record_population: {len(result['record_rows'])} published constraint subject(s)")
    for row in result["catalog_rows"]:
        print(f"  catalog: {row['document']} {row['name']} <- {row['compounds'][:2]}")
    for row in result["record_rows"]:
        print(f"  record:  {row['document']} {row['constraint_id']} {row['subject_signal']}")


def self_test() -> int:
    ran = 0
    failures = []

    def check(label: str, actual, expected) -> None:
        nonlocal ran
        ran += 1
        if actual != expected:
            failures.append(f"{label}: {actual!r} != {expected!r}")

    check("escaped head", sorted(escaped_heads("PARTITION\\_ACCESS bits")), ["PARTITION"])
    check(
        "escaped expansion",
        escaped_heads("PARTITION\\_ACCESS")["PARTITION"],
        {"PARTITION_ACCESS"},
    )
    check("declared names", declared_names(["Signal PARTITION is width 1."]), {"PARTITION"})
    # The trap: a synthesized declaration must NOT count as a standalone occurrence.
    check(
        "synthesized declarations are excluded",
        prose_only(["Signal PARTITION is width 1.", "the PARTITION\\_ACCESS bits"]),
        ["the PARTITION\\_ACCESS bits"],
    )
    check("no standalone in escaped-only prose", occurs_standalone("the PARTITION\\_ACCESS bits", "PARTITION"), False)
    check("standalone found when real", occurs_standalone("PARTITION is asserted", "PARTITION"), True)
    # `.3k.9.a` — the circularity control, end to end rather than on a part. The unit case above pins
    # `prose_only`; these two pin the CLASSIFICATION, which is what a future reader will run. Without
    # the exclusion the fragment's own synthesized declaration counts as a standalone occurrence of
    # the fragment and clears it, so the population reports EMPTY — which is exactly how this leaf's
    # first re-derivation concluded there was nothing here.
    contaminated = [
        "Signal XQPART is width 1.",
        "Enum XQPART NOT_DEFINED = 0.",
        "the XQPART\\_ACCESS bits select the area",
    ]
    check(
        "classification finds the fragment",
        spurious_heads(contaminated),
        {"XQPART": ["XQPART_ACCESS"]},
    )
    check(
        "RED: counting synthesized declarations clears the fragment that created them",
        spurious_heads(contaminated, exclude_synthesized=False),
        {},
    )

    if ran != SELF_TEST_CASES:
        failures.append(f"ran {ran} cases, expected {SELF_TEST_CASES}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if failures:
        return 1
    print(f"self-test: {ran}/{SELF_TEST_CASES} cases pass")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    result = census(repo_root())
    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        report(result)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
