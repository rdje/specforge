#!/usr/bin/env python3
"""Census what IntentIR carries against what the `.isf` adapter lowers (read-only).

`EXTRACTION-REPRESENTATION-GAPS.2` asks, family by family and over every live chain, how much of what
SpecForge carries in IntentIR reaches `.isf`, and for the rest whether the IR lacks an operand (a
SpecForge gap) or FSMGen's published contract cannot state it (an FSMGen gap). This script re-derives the
three measurements that answer it:

  1. families   every IntentIR field: records and documents over the live chains, and whether the ISF
                adapter's production code reads it at all (`crates/specforge/src/ir/isf_ir.rs`,
                `crates/specforge/src/ir/adapters.rs`, test modules excluded);
  2. blocking   why each live adapter is `blocked` rather than `renderable`, replaying the three reasons
                `assess_isf_renderability` checks (clock/reset contract, interface signals, behaviour);
  3. residuals  every adapter residual packet, grouped by family (its id with the record id removed),
                with its record and document counts and one example of its stated reason.

A chain is live when its persisted IntentIR has schema version 2 (`adapt` refuses the legacy stratum).
Attribution is not computed here: it needs FSMGen's published contract and is recorded, with citations,
in `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md`.

Usage: python3 scripts/measure_intent_to_isf_reach.py [families|blocking|residuals|all]
"""

import collections
import glob
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIVE_SCHEMA = re.compile(r'"schema_version"\s*:\s*2\b')
ADAPTER_SOURCES = ("crates/specforge/src/ir/isf_ir.rs", "crates/specforge/src/ir/adapters.rs")
# A residual packet id is `<family>_<record id>`, and the record id is one of the upstream id shapes below
# (`sigcon_0002`, `rule_10`, `transition_state_0001`, ...). Only that trailing id is removed, so every
# family keeps the words that distinguish it.
RECORD_SUFFIX = re.compile(
    r"_(?:sigcon|rule|constraint|transition_state|protocol_state|named_state|protocol_operation"
    r"|interface_edge_timing)_[0-9]+$"
)


def live_chains():
    """(document key, IntentIR) for every live chain, in key order."""
    for path in sorted(glob.glob(os.path.join(ROOT, "generated/intent_ir/*/intent_ir.json"))):
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        if LIVE_SCHEMA.search(text[:4000]):
            yield os.path.basename(os.path.dirname(path)), json.loads(text)


def adapter_reads():
    """IntentIR fields the adapter's production code reads (`intent_ir.<field>`, across line breaks)."""
    fields = set()
    for relative in ADAPTER_SOURCES:
        with open(os.path.join(ROOT, relative), encoding="utf-8") as handle:
            source = handle.read()
        tests = re.search(r"#\[cfg\(test\)\]\s*\n\s*mod tests", source)
        production = source[: tests.start()] if tests else source
        fields.update(re.findall(r"\bintent_ir\s*\.\s*([a-z_]+)", production))
    return fields


def record_count(value):
    if isinstance(value, list):
        return len(value)
    if isinstance(value, dict):
        lists = [v for v in value.values() if isinstance(v, list)]
        return sum(len(v) for v in lists) if lists else (1 if value else 0)
    return 0 if value is None else 1


def families(chains):
    read = adapter_reads()
    records, documents = collections.Counter(), collections.Counter()
    for _, ir in chains:
        for field, value in ir.items():
            count = record_count(value)
            records[field] += count
            documents[field] += 1 if count else 0
    print(f"{'adapter':8s} {'family':34s} {'records':>8s} {'documents':>9s}")
    for field in sorted(records, key=lambda f: (f in read, -records[f], f)):
        print(f"{'reads' if field in read else 'unread':8s} {field:34s} {records[field]:8d} {documents[field]:9d}")


def blocking(chains):
    tally = collections.Counter()
    for key, ir in chains:
        reasons = []
        contract = ir.get("system_contract")
        if not contract:
            reasons.append("no clock/reset contract")
        elif contract.get("reset_polarity") == "unknown":
            reasons.append("reset polarity unresolved")
        if not sum(len(i.get("signal_records", [])) for i in ir.get("interfaces", [])):
            reasons.append("no interface signals")
        behaviour = ("temporal_rules", "conditional_rules", "signal_constraints", "control_blocks")
        if not any(ir.get(field) for field in behaviour):
            reasons.append("no behaviour")
        tally[tuple(reasons) or ("renderable",)] += 1
    for reasons, count in tally.most_common():
        print(f"{count:3d}  {' + '.join(reasons)}")


def residuals(chains):
    records, documents, example = collections.Counter(), collections.defaultdict(set), {}
    for key, _ in chains:
        path = os.path.join(ROOT, "generated/adapters/isf", key, "adapter.json")
        if not os.path.exists(path):
            continue
        with open(path, encoding="utf-8") as handle:
            adapter = json.load(handle)
        for packet in adapter.get("residual_decisions", []):
            family = RECORD_SUFFIX.sub("", packet["packet_id"])
            records[family] += 1
            documents[family].add(key)
            example.setdefault(family, packet["why_unresolved"][:160])
    for family, count in records.most_common():
        print(f"{count:5d}  documents={len(documents[family]):2d}  {family}")
        print(f"       example: {example[family]}")
    print(f"total: {sum(records.values())} packets in {len(records)} families")


def main(argv):
    mode = argv[1] if len(argv) > 1 else "all"
    if mode not in ("families", "blocking", "residuals", "all"):
        print(__doc__, file=sys.stderr)
        return 2
    chains = list(live_chains())
    print(f"live chains: {len(chains)}")
    for name, run in (("families", families), ("blocking", blocking), ("residuals", residuals)):
        if mode in (name, "all"):
            print(f"\n== {name}")
            run(chains)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
