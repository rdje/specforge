#!/usr/bin/env python3
"""Families of the IntentIR assumptions and behaviors no typed record understands (EXTRACTION-REPRESENTATION-GAPS.3.d).

Read-only. Population: every IntentIR `assumptions` and `behaviors` record, on the retained chains whose EvidenceIR is
at the current schema, that cites no statement a typed record cites — the "captured but not understood" rows of
`scripts/measure_category_family_census.py`, whose predicate this script imports rather than restates. Each record is
placed in one family, the first whose test matches, in this order:

  pass_assumption           an assumption that cites no statement: IntentIR's own statement of a modeling choice
                            (`actor_channel is treated as a backend-neutral channel abstraction in this IntentIR
                            pass.`, written by `crates/specforge/src/ir/intent.rs`), by design and not document intent;
  table_row                 the statement is a Markdown table row the statement extractor lifted as prose;
  apparatus                 it sits in a glossary or contents section, or opens by referring to the document itself
                            (`This specification …`, `Figure 4-10 …`, `Note …`);
  software_agent            its subject, after a list marker and a leading `If`/`When` clause, names a software agent
                            from the closed list SOFTWARE_AGENTS;
  assumed_by_word           an assumption no earlier family took: SemanticIR lifts a statement to an abstraction by its
                            words (`assume`, `treated as`), so what remains is a hardware or protocol rule; each is
                            given one of six SHAPES (EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b), first match wins:
                            convention (the guide, document, example or reader is the subject), diagram_convention
                            (a timing-diagram key), narrative (an example, a modelling or analysis premise),
                            treated_as (an equivalence rule), stated_default (a value or state assumed) and other;
  permission_or_description a behavior no earlier family took: a permission (`may`) or a description of hardware or
                            protocol behavior that no typed reader reads.

SOFTWARE_AGENTS is a measurement vocabulary, not a production rule: no field of the IR says whether an actor is
software (SemanticIR actors are role nouns: `agent`, `controller`, `host`, `manager`), and that absence is one of
`.3.d`'s findings. Each family's owner or disposition is recorded in docs/research/category-family-census.md.

Usage: python3 scripts/measure_unread_intent_records.py [--json PATH] [--samples N]
"""

from __future__ import annotations

import argparse
import collections
import json
import random
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import measure_category_family_census as census  # noqa: E402

FAMILIES = ("pass_assumption", "table_row", "apparatus", "software_agent", "assumed_by_word", "permission_or_description")
SOFTWARE_AGENTS = re.compile(
    r"\b(software|drivers?|debuggers?|operating systems?|os|firmware|hypervisors?|connection managers?|"
    r"programmers?|applications?|users?)\b"
)
DOCUMENT_REFERENCE = re.compile(
    r"^(this (specification|document|section|chapter|appendix|table|figure)\b|(figure|table) [a-z]?\d|note\b|"
    r"see\b|for (more|further) information\b|the following (table|figure)\b)"
)
SHAPES = (
    ("convention", re.compile(
        r"^(this|the) (guide|document|specification|example|section|chapter|appendix|figure|table)\b[^.]*\bassum"
        r"|\bwe (assume|refer)\b|assum\w*\s+(that\s+)?(you|the reader)\b|\bare assumed:\s*$")),
    ("treated_as", re.compile(r"\btreated as\b")),
    ("diagram_convention", re.compile(r"timing diagram|shaded|waveform")),
    ("narrative", re.compile(r"\bexample\b|\bassuming\b|assume[sd]? that\b|assumes the\b|assumed to (be )?(using|sent|received)")),
    ("stated_default", re.compile(r"assumed to be|is assumed|are assumed|assumed as|\bassumed\b|\bassumes?\b")),
)
LIST_MARKER = re.compile(r"^(?:[-*•⚫o]\s+|\(?[0-9a-z]{1,3}[.)]\s+|[ivx]{1,4}\.\s+)+", re.IGNORECASE)
NON_CONTRACT_SECTIONS = {"glossary", "table_of_contents"}


def unmarked(text: str) -> str:
    """The statement without its leading list markers (`- a.`, `7.`, `- o`)."""
    return LIST_MARKER.sub("", text.strip()).strip()


def subject_region(text: str) -> str:
    """The words before the statement's first modal or finite verb, after a leading `If`/`When` clause."""
    lowered = unmarked(text).lower()
    lowered = re.sub(r"^(if|when|while|unless)\b[^,]*,\s*", "", lowered)
    return re.split(r"[,:;]|\b(must|shall|should|may|can|will|is|are|does|do|sets|waits|reads|writes)\b", lowered,
                    maxsplit=1)[0]


def shape(text: str) -> str:
    """The shape of an `assumed_by_word` record (`.3.d.ii.b`), first match wins; `other` when none matches."""
    lowered = unmarked(text).lower()
    return next((name for name, pattern in SHAPES if pattern.search(lowered)), "other")


def family(record: dict) -> str:
    if record["surface"] == "assumptions" and not record["cited_statements"]:
        return "pass_assumption"
    text = unmarked(record["text"])
    if text.startswith("|"):
        return "table_row"
    if record["section_kind"] in NON_CONTRACT_SECTIONS or DOCUMENT_REFERENCE.search(text.lower()):
        return "apparatus"
    if SOFTWARE_AGENTS.search(subject_region(record["text"])):
        return "software_agent"
    return "assumed_by_word" if record["surface"] == "assumptions" else "permission_or_description"


def chain_records(key: str) -> list[dict] | None:
    paths = {stage: census.GENERATED / stage / key / f"{stage}.json"
             for stage in ("source_ir", "evidence_ir", "semantic_ir", "intent_ir")}
    try:
        source, evidence, semantic, intent = (census.load(paths[stage])
                                              for stage in ("source_ir", "evidence_ir", "semantic_ir", "intent_ir"))
    except FileNotFoundError:
        return None
    if evidence.get("schema_version") != census.CURRENT_EVIDENCE_SCHEMA:
        return None
    typed: set[str] = set()
    for surface in census.TYPED_SURFACES:
        census.statement_ids(intent.get(surface, []), typed)
    semantic_statements: dict[str, set[str]] = {}
    for surface in census.SEMANTIC_RECORD_SURFACES:
        for record in semantic.get(surface, []):
            record_id = next((record[k] for k in record if k.endswith("_id")), None)
            if record_id:
                semantic_statements[record_id] = census.statement_ids(record, set())
    statement_class = {s["statement_id"]: s["class"] for s in evidence["extracted_statements"]}
    span_line = {span["span_id"]: span.get("line_start") for span in evidence["evidence_spans"]}
    statement_line = {
        s["statement_id"]: min([span_line.get(span) or 10**9 for span in s.get("evidence_span_ids", [])] or [10**9])
        for s in evidence["extracted_statements"]
    }
    anchors = sorted((anchor.get("line_start") or 0, anchor["title"]) for anchor in evidence["section_anchors"])
    section_kind = {section.get("title"): section.get("kind") for section in source.get("document_sections", [])}
    category = census.evidence_metric(key, evidence, "document_intent_category") or "not_reported"

    def section_of(line: int) -> str | None:
        title = None
        for start, heading in anchors:
            if start > line:
                break
            title = heading
        return title

    records = []
    for surface in ("assumptions", "behaviors"):
        for record in intent.get(surface, []):
            cited = census.statement_ids(record, set())
            for semantic_id in record.get("supporting_semantic_ids", []):
                cited |= semantic_statements.get(semantic_id, set())
            if cited & typed:
                continue
            section = section_of(min([statement_line.get(s, 10**9) for s in cited] or [10**9]))
            row = {
                "key": key,
                "category": category,
                "surface": surface,
                "text": record.get("statement", ""),
                "statement_classes": sorted({statement_class.get(s, "unknown") for s in cited}),
                "cited_statements": len(cited),
                "section": section,
                "section_kind": section_kind.get(section),
            }
            row["family"] = family(row)
            if row["family"] == "assumed_by_word":
                row["shape"] = shape(row["text"])
            records.append(row)
    return records


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", type=Path, help="also write every record with its family to this path")
    parser.add_argument("--samples", type=int, default=0, help="print N seeded random members of each family")
    args = parser.parse_args()
    retained = census.load(census.RETAINED)["retained"]
    records = [row for key in sorted(retained) for row in (chain_records(key) or [])]
    if args.json:
        args.json.write_text(json.dumps(records, indent=1, sort_keys=True) + "\n")
    by_surface = collections.Counter(row["surface"] for row in records)
    print(f"unread records: {len(records)} ({by_surface['assumptions']} assumptions, {by_surface['behaviors']} behaviors)")
    counts = collections.Counter((row["family"], row["surface"]) for row in records)
    for name in FAMILIES:
        members = [row for row in records if row["family"] == name]
        categories = collections.Counter(row["category"] for row in members)
        timing = sum("timing_constraint" in row["statement_classes"] for row in members)
        print(f"  {name}: {len(members)} ({counts[(name, 'assumptions')]} assumptions, "
              f"{counts[(name, 'behaviors')]} behaviors; timing-class {timing}) "
              + ", ".join(f"{category} {n}" for category, n in categories.most_common()))
        if name == "assumed_by_word":
            shapes = collections.Counter(row["shape"] for row in members)
            print("      shapes: " + ", ".join(f"{shape_name} {n}" for shape_name, n in shapes.most_common()))
        if args.samples:
            for row in random.Random(f"3.d:{name}").sample(members, min(args.samples, len(members))):
                print(f"      {row['key'][:28]}: {row['text'][:160]}")


if __name__ == "__main__":
    main()
