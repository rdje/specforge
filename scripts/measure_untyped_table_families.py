#!/usr/bin/env python3
"""Families of the untyped tables no canonical record cites (EXTRACTION-REPRESENTATION-GAPS.3.b, .3.b.ii). Read-only.

Population: every table SourceIR typed `unknown` whose region SemanticIR lists in `captured_region_residuals` (no
canonical record cites it), on the retained chains whose EvidenceIR is at the current schema. Each table is placed in
one family by its first header row's words, its caption's words, or, with no header row, by the shape of its body;
the first family whose test matches wins, in the order below. Each family names its owner or its non-contract
disposition. The tables no rule places were read one by one (`.3.b.ii`); their dispositions are recorded in
docs/research/category-family-census.md.

Usage: python3 scripts/measure_untyped_table_families.py [--json PATH] [--samples N]
"""

from __future__ import annotations

import argparse
import collections
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RETAINED = ROOT / "doctrine/chain_currency/retained_bundles.json"
GENERATED = ROOT / "generated"
CURRENT_EVIDENCE_SCHEMA = 3


def words(text: str) -> list[str]:
    return re.findall(r"[a-z0-9]+", text.lower())


def cells(row) -> list[str]:
    return [" ".join(cell["text"].split()) for cell in row]


def any_word(header: list[list[str]], vocabulary: set[str]) -> bool:
    return any(vocabulary & set(cell) for cell in header)


HUNGARIAN_FIELD = re.compile(r"(?:b|bm|w|i|id|dw|bcd)[A-Z][A-Za-z]*")
REGISTER_LIKE = re.compile(r"[A-Z][A-Z0-9_]{1,}")


def is_register_figure_piece(texts: list[str], body: list[list[str]]) -> bool:
    """A piece of a register-layout figure: cells that are only offsets, register mnemonics and their labels."""
    cells_seen = [text for text in texts + (body[0] if body else []) if text]
    if not cells_seen:
        return False
    def piece(text: str) -> bool:
        return all(
            word.lower() in {"offset", "component", "location", "register", "access"}
            or REGISTER_LIKE.fullmatch(word) or re.fullmatch(r"0x[0-9a-fA-F]+", word)
            for word in text.split()
        )
    return all(piece(text) for text in cells_seen)


def is_glossary(texts: list[str], body: list[list[str]]) -> bool:
    """A two-column list of short terms and their definitions: `AFU | Attached functional unit.`"""
    pairs = [texts] + [row for row in body if len(row) == 2]
    terms = [pair for pair in pairs if re.fullmatch(r"[A-Z][A-Za-z0-9/]{0,7}", pair[0] or "")
             and len((pair[1] or "").split()) >= 2 and len((pair[1] or "").split()) <= 14]
    return len(terms) * 2 > len(pairs) and len(pairs) >= 2


def classify(header_rows, body_rows, col_count, signals: set[str], caption: str = "") -> str:
    """The family of one table, by its first header row's words or, with none, by its body's shape."""
    texts = cells(header_rows[0]) if header_rows else []
    body = [cells(row) for row in body_rows]
    flat = [text for row in body for text in row]
    if not header_rows:
        if not body or all(not text for text in flat):
            return "empty"
        if col_count <= 1 or all(len(row) <= 1 for row in body):
            return "figure_fragment"
        if flat and sum(bool(re.fullmatch(r"[\d\s\[\]:]+", text)) or not text for text in flat) * 2 >= len(flat):
            return "figure_fragment"
        texts = body[0]
    header = [words(text) for text in texts]
    if not any(header):
        return "residue"
    if any(re.search(r"\.{4,}|(?:\. ){3,}", text) for text in texts + flat[:20]):
        return "document_apparatus"
    if re.fullmatch(r"(cycle \d+|t\d+\s*-\s*t\d+|t\d+)", (texts[0] if texts else "").lower()):
        return "waveform_narration"
    if any(re.match(r"(figure|table) \d", text.lower()) for text in texts):
        return "figure_fragment"
    if sum(bool(re.fullmatch(r"\[?\d+(:\d+)?\]?|bit \d+|\d+( \d+)+", text.lower())) for text in texts) * 2 > len(texts):
        return "figure_fragment"
    if any_word(header, {"revision", "rev", "issue", "confidentiality", "change", "changes", "version", "term",
                         "terms", "abbreviation", "definition", "meaning", "acronym", "glossary", "reference",
                         "document", "page", "chapter", "section", "convention", "information", "date"}) \
            and not any_word(header, {"signal", "bits", "width", "offset"}):
        return "document_apparatus"
    if any_word(header, {"instruction", "instructions", "latency", "throughput", "pipelines", "op", "comment"}):
        return "software_or_performance"
    if any_word(header, {"bit"}) and sum(text.lower().endswith("signal") for text in texts) >= 1:
        return "field_signal_mapping"
    if any_word(header, {"signal", "signals", "pin"}) or (texts and texts[0].lower() == "name"
                                                        and any_word(header, {"source", "direction"})):
        if any_word(header, {"presence", "present", "axi5"}):
            return "presence_or_compliance_matrix"
        if any_word(header, {"type", "direction", "dir", "source"}):
            return "signal_declaration_untyped"
        names = [row[0] for row in body if row]
        if names and sum(bool(re.search(r"\[\d+(:\d+)?\]$", name)) for name in names) * 2 > len(names):
            return "structure_or_message_layout"
        return "residue"
    if any_word(header, {"default"}) and not any_word(header, {"bits", "bit", "field", "access", "reset", "offset"}):
        return "parameter_definition"
    if any_word(header, {"presence", "present", "property", "compliance", "applicability", "axi5", "required"}) \
            or any(":" in text and ("true" in text.lower() or "false" in text.lower()) for text in texts):
        return "presence_or_compliance_matrix"
    if any_word(header, {"lane", "lanes", "gbps", "gt", "units", "rd", "encoded", "symbol", "db", "loss", "jitter",
                         "voltage", "impedance", "degraded", "specification"}):
        return "physical_coding_or_electrical"
    if any_word(header, {"access", "reset"}) or (any_word(header, {"register"}) and any_word(header, {"bit", "bits", "field", "fields"})):
        return "register_field_table"
    if any(text.split("[")[0].strip() in signals for text in texts if text):
        return "signal_value_matrix"
    if any(re.search(r"\b[A-Z][A-Z0-9_]+\.[A-Za-z][A-Za-z0-9_]*", text) for text in texts) \
            or any(text.lower().startswith("value of") for text in texts):
        return "field_value_encoding"
    if any_word(header, {"width", "bits", "bit", "dw", "field", "byte", "bytes", "position", "decoding", "mapping"}) \
            or HUNGARIAN_FIELD.fullmatch(texts[0] or ""):
        return "structure_or_message_layout"
    if is_register_figure_piece(texts, body):
        return "figure_fragment"
    everything = texts + flat[:24]
    if everything and sum(bool(re.fullmatch(r"(?:\S ){3,}\S", text)) for text in everything if text) * 2 >= len(
            [text for text in everything if text]):
        return "figure_fragment"
    if any(re.search(r"_{6,}|\bchair\b|\binc\.|\bcorp\b", text.lower()) for text in everything) \
            or (len(texts) >= 2 and re.fullmatch(r"\d+(\.\d+)*\.?", texts[0] or "") and re.fullmatch(r"\d+", texts[-1] or "")):
        return "document_apparatus"
    if len(texts) == 2 and is_glossary(texts, body):
        return "document_apparatus"
    first_body_words = {w for text in (body[0] if body else []) for w in words(text)}
    if {"instruction", "instructions"} & first_body_words:
        return "software_or_performance"
    if any_word(header, {"permitted", "legal", "compatible", "compatibility", "constraint", "constraints", "applies",
                         "mandatory", "optional", "rules", "required"}) \
            or any("mandatory" in text.lower() for text in texts):
        return "response_or_requirement_matrix"
    if texts and words(texts[-1])[-1:] in (["operation"], ["outcome"], ["effect"]) and len(texts) >= 3:
        return "signal_value_matrix"
    if any_word(header, {"response", "responses", "ack", "action", "recovery", "behavior", "behaviour"}) \
            or sum(text.rstrip().endswith("?") for text in texts) >= 1:
        return "response_or_requirement_matrix"
    if len(texts) == 2 and words(texts[1])[:1] in (["description"], ["message"], ["meaning"], ["command"]) \
            and len((texts[0] or "").split()) <= 2:
        return "encoding_untyped"
    if any_word(header, {"timeout", "timeouts", "timer", "timers"}):
        return "timing_untyped"
    if any_word(header, {"value", "values", "code", "codes", "encoding"}):
        return "encoding_untyped"
    if any_word(header, {"promoter", "company", "employees", "contributors", "characteristic"}):
        return "document_apparatus"
    caption_words = set(words(caption))
    if caption_words & {"example", "examples"}:
        return "software_or_performance"
    if caption_words & {"permitted", "legal", "rules", "compatibility", "applicability", "mandatory", "required",
                        "constraints", "combinations"}:
        return "response_or_requirement_matrix"
    if texts and (texts[0] or "").lower() == "operation" and len(texts) >= 3:
        return "signal_value_matrix"
    if caption_words & {"messages", "format", "fields", "container", "header", "packet"}:
        return "structure_or_message_layout"
    if caption_words & {"modes", "types", "codes", "events", "encodings", "priority"}:
        return "encoding_untyped"
    titles = {text.lower() for text in texts}
    if titles & {"register", "registers", "register name", "name", "address", "offset", "mnemonic"} \
            or (texts and all(REGISTER_LIKE.fullmatch(text) for text in texts if text) and any(texts)):
        return "register_summary"
    return "residue"


FAMILIES = {
    "structure_or_message_layout": "DOC-INTENT-TAXONOMY.4b (structure and message field layouts)",
    "parameter_definition": "KG-ISF-COMPLETENESS.2a.vii.c (a parameter's stated values and default)",
    "presence_or_compliance_matrix": "EXTRACTION-REPRESENTATION-GAPS.3.c.v (requirement, presence and compliance matrices)",
    "signal_declaration_untyped": "EXTRACTION-REPRESENTATION-GAPS.3.c.viii (a signal table the converter shifted; .3.b.i reads the rest)",
    "register_summary": "read table by table (EXTRACTION-REPRESENTATION-GAPS.3.b.ii; dispositions in the census record)",
    "register_field_table": "read table by table (EXTRACTION-REPRESENTATION-GAPS.3.b.ii; dispositions in the census record)",
    "waveform_narration": "non-contract: the narration of an example waveform, cycle by cycle",
    "signal_value_matrix": "EXTRACTION-REPRESENTATION-GAPS.3.g.iii (behaviour keyed by several signals' values)",
    "field_value_encoding": "EXTRACTION-REPRESENTATION-GAPS.3.g.iii (encodings keyed by several register fields)",
    "physical_coding_or_electrical": "non-contract: line coding, lanes and electrical parameters, outside the digital boundary",
    "document_apparatus": "non-contract: revision histories, glossaries, contents, contributors, comparisons",
    "software_or_performance": "non-contract: instruction timings, programming and worked examples",
    "figure_fragment": "non-contract: fragments of a figure the converter read as a table",
    "empty": "non-contract: no cell text",
    "response_or_requirement_matrix": "EXTRACTION-REPRESENTATION-GAPS.3.c.v (a response or requirement per combination of keys)",
    "timing_untyped": "EXTRACTION-REPRESENTATION-GAPS.3.b.iii (timers and timeouts in tables the ingest left untyped)",
    "field_signal_mapping": "EXTRACTION-REPRESENTATION-GAPS.41.e (a register field bit mapped to a signal bit, in a table)",
    "encoding_untyped": "EXTRACTION-REPRESENTATION-GAPS.3.b.iii (value and meaning tables the ingest left untyped)",
    "residue": "read table by table (EXTRACTION-REPRESENTATION-GAPS.3.b.ii; dispositions in the census record)",
}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", type=Path)
    parser.add_argument("--samples", type=int, default=0)
    args = parser.parse_args()
    keys = sorted(json.loads(RETAINED.read_text())["retained"])
    families = collections.Counter()
    per_family_docs = collections.defaultdict(collections.Counter)
    samples = collections.defaultdict(list)
    signal_rows = collections.Counter()
    for key in keys:
        evidence = json.loads((GENERATED / "evidence_ir" / key / "evidence_ir.json").read_text())
        if evidence.get("schema_version") != CURRENT_EVIDENCE_SCHEMA:
            continue
        source = json.loads((GENERATED / "source_ir" / key / "source_ir.json").read_text())
        semantic = json.loads((GENERATED / "semantic_ir" / key / "semantic_ir.json").read_text())
        uncited = {r.get("region_id") for r in semantic.get("captured_region_residuals", [])}
        signals = {record["signal_name"] for interface in semantic.get("interfaces", [])
                   for record in interface.get("signal_records", [])}
        for table in source["structured_tables"]:
            if table["table_id"] not in uncited or table.get("table_kind", "unknown") != "unknown":
                continue
            family = classify(table["header_rows"], table["body_rows"], table.get("col_count", 0), signals,
                              table.get("caption_text") or "")
            families[family] += 1
            if family == "signal_declaration_untyped":
                names = [row[0]["text"].split("[")[0].split()[0] for row in table["body_rows"] if row and row[0]["text"].strip()]
                signal_rows["rows"] += len(names)
                signal_rows["in_catalog"] += sum(name in signals for name in names)
            per_family_docs[family][key] += 1
            if len(samples[family]) < args.samples:
                first = table["header_rows"][0] if table["header_rows"] else (table["body_rows"][0] if table["body_rows"] else [])
                samples[family].append(f"{key[:28]} {table['table_id']} | {' | '.join(cells(first))[:110]}")
    total = sum(families.values())
    print(f"untyped tables no canonical record cites: {total}")
    for family, count in families.most_common():
        print(f"  {count:5}  {100 * count / total:5.1f}%  {family:32} {FAMILIES[family]}")
        for sample in samples[family]:
            print(f"           {sample}")
    print(f"signal_declaration_untyped rows: {signal_rows['rows']}, of which {signal_rows['in_catalog']} reach the catalog from another table")
    if args.json:
        args.json.write_text(json.dumps({"total": total, "families": dict(families),
                                         "documents": {f: dict(d) for f, d in per_family_docs.items()}}, indent=1, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
