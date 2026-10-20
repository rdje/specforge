#!/usr/bin/env python3
"""Per-category, per-family census of the replayable chains (EXTRACTION-REPRESENTATION-GAPS.3). Read-only.

The denominator is `doctrine/spec_to_intent_category_contract.json`: six document categories, each with semantic
families mapped to the IntentIR surfaces that carry them (four families map to none). The population is every chain
whose normalized bundle is retained (`doctrine/chain_currency/retained_bundles.json`) and whose EvidenceIR is at the
current schema, so every number is what the current binary produces.

For each chain it reports:
  * the document's own category and confidence (the EvidenceIR stage's validation metrics);
  * per family, how many records each IntentIR surface of that family carries;
  * the captured source regions no canonical record cites (IntentIR `captured_region_residuals`), attributed to
    the table kind SourceIR gave the region, or to message/structure fields when EvidenceIR's
    `message_field_records` cite the table (a family with no IntentIR carrier);
  * the untyped records (`constraints`, `behaviors`, `assumptions`) whose statements no typed record cites:
    captured but not understood.

Usage: python3 scripts/measure_category_family_census.py [--json PATH]
"""

from __future__ import annotations

import argparse
import collections
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "doctrine/spec_to_intent_category_contract.json"
RETAINED = ROOT / "doctrine/chain_currency/retained_bundles.json"
GENERATED = ROOT / "generated"
CURRENT_EVIDENCE_SCHEMA = 3

# The typed IntentIR surfaces whose citations make a statement "understood" (as `.3`'s first census defined it).
TYPED_SURFACES = (
    "signal_constraints", "conditional_rules", "temporal_rules", "actor_contracts", "transactions",
    "register_records", "actor_signal_relations", "interfaces", "actor_ports", "signal_polarities",
    "symbol_definitions", "timing_constraints", "protocol_operations", "protocol_states", "protocol_transitions",
    "serial_frame_fields",
    "interface_edge_timings", "regular_states", "state_transitions", "control_blocks", "signal_connectivity",
    "temporal_invariants", "signal_presence_records", "equivalence_rules", "permission_rules",
)
UNTYPED_SURFACES = ("constraints", "behaviors", "assumptions")
SEMANTIC_RECORD_SURFACES = ("invariants", "contracts", "gates", "assertions", "abstractions", "phases")
STATEMENT_ID_KEYS = ("supporting_statement_ids", "source_statement_ids")


def load(path: Path) -> dict:
    return json.loads(path.read_text())


def statement_ids(value, found: set[str]) -> set[str]:
    if isinstance(value, dict):
        for key, item in value.items():
            if key in STATEMENT_ID_KEYS and isinstance(item, list):
                found.update(x for x in item if isinstance(x, str))
            else:
                statement_ids(item, found)
    elif isinstance(value, list):
        for item in value:
            statement_ids(item, found)
    return found


def evidence_metric(key: str, evidence: dict, name: str):
    """A validation metric of the EvidenceIR artifact, or None when no report describes this artifact.

    Only `specforge validate` writes a report; `specforge evidence` rewrites the artifact and leaves the sibling
    `validation_report.json` of the previous build in place. A sibling older than the artifact describes another
    build, so it is refused rather than read (the census once published four categories from such reports).
    """
    reports = list(evidence.get("validation_reports", []))
    artifact = GENERATED / "evidence_ir" / key / "evidence_ir.json"
    sibling = GENERATED / "evidence_ir" / key / "validation_report.json"
    if sibling.exists() and sibling.stat().st_mtime >= artifact.stat().st_mtime - 1:
        reports.append(load(sibling))
    for report in reversed(reports):
        for metric in report.get("metrics", []):
            if metric.get("name") == name:
                return metric.get("value")
    return None


def surface_count(intent: dict, surface: str) -> int:
    """Records on an IntentIR surface; `a[].b` counts b across a, `a[].b.c` counts b entries whose c is stated."""
    head, _, rest = surface.partition("[].")
    value = intent.get(head) or []
    if not rest:
        return len(value) if isinstance(value, (list, dict)) else 1
    parts = rest.split(".")
    items = [child for record in value for child in (record.get(parts[0]) or [])]
    if len(parts) == 1:
        return len(items)
    return sum(1 for item in items if item.get(parts[1]) not in (None, "", [], {}))


def untyped_not_understood(intent: dict, semantic: dict) -> dict[str, list[int]]:
    typed: set[str] = set()
    for surface in TYPED_SURFACES:
        statement_ids(intent.get(surface, []), typed)
    semantic_statements: dict[str, set[str]] = {}
    for surface in SEMANTIC_RECORD_SURFACES:
        for record in semantic.get(surface, []):
            record_id = next((record[k] for k in record if k.endswith("_id")), None)
            if record_id:
                semantic_statements[record_id] = statement_ids(record, set())
    result = {}
    for surface in UNTYPED_SURFACES:
        total = not_understood = 0
        for record in intent.get(surface, []):
            cited = statement_ids(record, set())
            for semantic_id in record.get("supporting_semantic_ids", []):
                cited |= semantic_statements.get(semantic_id, set())
            total += 1
            not_understood += not (cited & typed)
        result[surface] = [total, not_understood]
    return result


def uncarried_regions(intent: dict, source: dict, evidence: dict) -> collections.Counter:
    table_kind = {t["table_id"]: t.get("table_kind", "unknown") for t in source.get("structured_tables", [])}
    diagram_kind = {v["asset_id"]: v.get("diagram_kind", "unknown") for v in source.get("visual_assets", [])}
    field_tables = {
        table for record in evidence.get("message_field_records", [])
        for table in record.get("supporting_table_ids", [])
    }
    attribution: collections.Counter = collections.Counter()
    for residual in intent.get("captured_region_residuals", []):
        region = residual.get("region_id", "")
        kind = residual.get("region_kind", "unknown")
        if kind == "table_region":
            if region in field_tables:
                attribution["table: message/structure fields"] += 1
            else:
                attribution[f"table: {table_kind.get(region, 'unknown')}"] += 1
        else:
            attribution[f"{kind}: {diagram_kind.get(region, 'unknown')}"] += 1
    return attribution


def chain_row(key: str, contract: dict) -> dict | None:
    try:
        source = load(GENERATED / "source_ir" / key / "source_ir.json")
        evidence = load(GENERATED / "evidence_ir" / key / "evidence_ir.json")
        semantic = load(GENERATED / "semantic_ir" / key / "semantic_ir.json")
        intent = load(GENERATED / "intent_ir" / key / "intent_ir.json")
    except FileNotFoundError:
        return None
    if evidence.get("schema_version") != CURRENT_EVIDENCE_SCHEMA:
        return None
    families = {}
    for category in contract["categories"]:
        for family in category["semantic_families"]:
            surfaces = [s for s in family.get("intent_ir_surfaces") or [] if not s.startswith("the applicable")]
            families[f"{category['id']}/{family['family']}"] = {s: surface_count(intent, s) for s in surfaces}
    return {
        "key": key,
        "category": evidence_metric(key, evidence, "document_intent_category") or "not_reported",
        "confidence": evidence_metric(key, evidence, "document_intent_category_confidence") or "-",
        "families": families,
        "message_field_records": len(evidence.get("message_field_records", [])),
        "register_records": len(intent.get("register_records", [])),
        "uncarried_regions": dict(uncarried_regions(intent, source, evidence)),
        "untyped": untyped_not_understood(intent, semantic),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", type=Path, help="also write the per-chain rows to this path")
    args = parser.parse_args()
    contract = load(CONTRACT)
    retained = load(RETAINED)["retained"]
    rows = [row for row in (chain_row(key, contract) for key in sorted(retained)) if row]
    if args.json:
        args.json.write_text(json.dumps(rows, indent=1, sort_keys=True) + "\n")

    print(f"replayable chains: {len(rows)} of {len(retained)} retained")
    by_category = collections.defaultdict(list)
    for row in rows:
        by_category[row["category"]].append(row)
    for category, members in sorted(by_category.items()):
        print(f"\n== {category}: {len(members)} documents")
        for member in members:
            print(f"   {member['key']} ({member['confidence']})")
        family_totals: dict[str, list[int]] = {}
        for member in members:
            for family, surfaces in member["families"].items():
                populated, records = family_totals.setdefault(family, [0, 0])
                total = sum(surfaces.values())
                family_totals[family] = [populated + (total > 0), records + total]
        print("   families (documents with a record / records):")
        for family, (populated, records) in family_totals.items():
            carrier = "" if members[0]["families"][family] else "  NO IntentIR SURFACE"
            print(f"     {family}: {populated}/{len(members)} / {records}{carrier}")
        regions: collections.Counter = collections.Counter()
        for member in members:
            regions.update(member["uncarried_regions"])
        print(f"   captured regions no canonical record cites: {sum(regions.values())}")
        for attribution, count in regions.most_common():
            print(f"     {attribution}: {count}")
        print(f"   EvidenceIR message/structure field records: {sum(m['message_field_records'] for m in members)}")
        for surface in UNTYPED_SURFACES:
            total = sum(m["untyped"][surface][0] for m in members)
            not_understood = sum(m["untyped"][surface][1] for m in members)
            print(f"   untyped {surface}: {not_understood} of {total} cite no statement a typed record cites")


if __name__ == "__main__":
    main()
