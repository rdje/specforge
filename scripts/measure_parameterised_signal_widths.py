#!/usr/bin/env python3
"""Census the symbolic signal widths the `.isf` adapter would lower (read-only).

`KG-ISF-COMPLETENESS.2a.vii` asks that a signal whose width a specification
writes as a parameter (`HRDATA | DATA_WIDTH`) lower to `(width DATA_WIDTH)` plus
a `(params (DATA_WIDTH <default>))` declaration, instead of today's `(width 1)`.
Before that is coded, this census answers three questions over every persisted
IntentIR:

  1. Which chains can the change reach at all? `specforge adapt` refuses a
     schema-1 (legacy/proofless) IntentIR, so only schema-2 chains are live;
     the rest are frozen until re-ingest and are reported separately.
  2. For each signal the adapter emits (first-seen name across the interface
     records, as `IsfModule::from_intent_ir` dedups), what do its width hints
     say, gathered from EVERY interface record and every actor port?
       `concrete`   only numeric hints
       `parameter`  exactly one symbolic hint, an identifier, and no numeric one
       `derived`    exactly one symbolic hint that is an expression
                    (`DATA_WIDTH/8`, `ceil(ADDR_WIDTH/8)`, `clog2(Num_RP_AR)`)
       `conflict`   more than one distinct hint with a symbolic one among them
       `none`       no hint at all
  3. Which `parameter` names are also declared SIGNAL names in the same
     document? A width cannot be a wire, so such a name is a mis-read cell, not
     a parameter (LTI's `LAADDR | LAVALID`).

Usage: python3 scripts/measure_parameterised_signal_widths.py [--signals]
  --signals  also list every parameter/derived signal with its hint.
"""

import collections
import glob
import json
import os
import re
import sys

IDENTIFIER = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def classify(hints):
    symbolic = sorted(h for h in hints if not h.isdigit())
    if not hints:
        return "none", None
    if not symbolic:
        return "concrete", None
    if len(hints) > 1:
        return "conflict", "|".join(sorted(hints))
    hint = symbolic[0]
    return ("parameter" if IDENTIFIER.match(hint) else "derived"), hint


def census(path):
    ir = json.load(open(path))
    hints = collections.defaultdict(set)
    order = []
    for interface in ir.get("interfaces", []):
        for record in interface.get("signal_records", []):
            name = record["signal_name"]
            if name not in hints:
                order.append(name)
            hints[name]
            if record.get("width_hint") is not None:
                hints[name].add(str(record["width_hint"]))
    for port in ir.get("actor_ports", []):
        if port["signal_name"] in hints and port.get("width_hint") is not None:
            hints[port["signal_name"]].add(str(port["width_hint"]))
    declared = set(order)
    rows = []
    for name in order:
        kind, hint = classify(hints[name])
        rows.append((name, kind, hint, kind == "parameter" and hint in declared))
    return ir.get("schema_version"), rows


def main():
    show_signals = "--signals" in sys.argv[1:]
    totals = {True: collections.Counter(), False: collections.Counter()}
    parameters = {True: collections.Counter(), False: collections.Counter()}
    for path in sorted(glob.glob(os.path.join(ROOT, "generated/intent_ir/*/intent_ir.json"))):
        doc = path.split(os.sep)[-2]
        schema, rows = census(path)
        live = schema == 2
        counts = collections.Counter(kind for _, kind, _, _ in rows)
        totals[live].update(counts)
        symbolic = [row for row in rows if row[1] in ("parameter", "derived", "conflict")]
        if not symbolic:
            continue
        names = sorted({hint for _, kind, hint, _ in rows if kind == "parameter"})
        parameters[live].update(names)
        wires = sorted({hint for _, _, hint, is_wire in rows if is_wire})
        print(
            f"{'live  ' if live else 'legacy'} {doc}: parameter={counts['parameter']} "
            f"derived={counts['derived']} conflict={counts['conflict']} "
            f"distinct_parameters={len(names)}"
            + (f" signal_named_widths={wires}" if wires else "")
        )
        if show_signals:
            for name, kind, hint, is_wire in symbolic:
                print(f"    {kind:9} {name}: {hint}{'  (names a declared signal)' if is_wire else ''}")
    for live in (True, False):
        label = "live (schema 2)" if live else "legacy (schema 1, frozen)"
        t = totals[live]
        print(
            f"TOTAL {label}: signals={sum(t.values())} concrete={t['concrete']} none={t['none']} "
            f"parameter={t['parameter']} derived={t['derived']} conflict={t['conflict']} "
            f"distinct_parameter_names_per_doc={sum(parameters[live].values())}"
        )


if __name__ == "__main__":
    main()
