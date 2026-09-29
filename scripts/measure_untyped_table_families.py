#!/usr/bin/env python3
"""Families of the untyped tables no canonical record cites (EXTRACTION-REPRESENTATION-GAPS.3.b). Read-only.

Population: every table SourceIR typed `unknown` whose region SemanticIR lists in `captured_region_residuals` (no
canonical record cites it), on the retained chains whose EvidenceIR is at the current schema. Each table is placed in
one family by its first header row's words, or, with no header row, by the shape of its body; the first family whose
test matches wins, in the order below. Each family names its owner or its non-contract disposition.

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


def classify(header_rows, body_rows, col_count, signals: set[str]) -> str:
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
    titles = {text.lower() for text in texts}
    if titles & {"register", "registers", "register name", "name", "address", "offset", "mnemonic"} \
            or (texts and all(REGISTER_LIKE.fullmatch(text) for text in texts if text) and any(texts)):
        return "register_summary"
    return "residue"


FAMILIES = {
    "structure_or_message_layout": "DOC-INTENT-TAXONOMY.4b (structure and message field layouts)",
    "parameter_definition": "KG-ISF-COMPLETENESS.2a.vii.c (a parameter's stated values and default)",
    "presence_or_compliance_matrix": "EXTRACTION-REPRESENTATION-GAPS.3.c.v (requirement, presence and compliance matrices)",
    "signal_declaration_untyped": "EXTRACTION-REPRESENTATION-GAPS.3.b.i (a signal table SourceIR left untyped declares nothing)",
    "register_summary": "EXTRACTION-REPRESENTATION-GAPS.3.b.ii (register indexes and register-figure pieces, table by table)",
    "register_field_table": "EXTRACTION-REPRESENTATION-GAPS.3.b.ii (register field tables the field-table reader missed)",
    "waveform_narration": "non-contract: the narration of an example waveform, cycle by cycle",
    "signal_value_matrix": "EXTRACTION-REPRESENTATION-GAPS.3.g.iii (behaviour keyed by several signals' values)",
    "field_value_encoding": "EXTRACTION-REPRESENTATION-GAPS.3.g.iii (encodings keyed by several register fields)",
    "physical_coding_or_electrical": "non-contract: line coding, lanes and electrical parameters, outside the digital boundary",
    "document_apparatus": "non-contract: revision histories, glossaries, contents, references",
    "software_or_performance": "non-contract: instruction timings and programming examples",
    "figure_fragment": "non-contract: fragments of a figure the converter read as a table",
    "empty": "non-contract: no cell text",
    "residue": "unadjudicated; EXTRACTION-REPRESENTATION-GAPS.3.b.ii reads it",
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
            family = classify(table["header_rows"], table["body_rows"], table.get("col_count", 0), signals)
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
