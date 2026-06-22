#!/usr/bin/env python3
"""measure_cat3_topology_recall.py — DOC-INTENT-TAXONOMY.4c.i reproducer.

Read-only structural gauge of the cat-3 (platform / system-IP) TOPOLOGY-CAPTURE
recall over the persisted IntentIR corpus. Quantifies how faithfully the two typed
topology surfaces capture a multi-component subsystem's interconnect, so the cat-3
ISF-construct question (verified FSMGen FR vs honest non-target, deferred from `.4c`)
can be decided from evidence rather than a guess.

The two topology surfaces (per `generated/intent_ir/<key>/intent_ir.json`):
  - signal_connectivity[]   : a typed producer->consumer component graph
      {signal_name, producer_actor_names[], consumer_actor_names[], ...}
  - infrastructure_signals[]: the clock/reset distribution shape
      {signal_name, kind, recovered_source_actor_ids[], distributed_to_actor_ids[], ...}

Faithfulness metrics (a real interconnect netlist is DENSE and FULLY-CONNECTED):
  - edges per actor                  (density of the captured component graph)
  - both-endpoint edge fraction      (an edge with a producer AND >=1 consumer is a
                                       real connection; a half edge is an unusable stub)
  - clean-endpoint fraction          (endpoint name not None / not backslash-escaped —
                                       a name-quality gauge, the `.4c` "noisy" claim)
  - infra-with-distribution / -source(clock/reset distribution + source resolution)

The cat-1 wire docs are profiled as a REFERENCE BASELINE: the same surface is dense
and clean there (the protocol signal graph), which proves the surface is *capable* of
faithful topology and isolates the cat-3 shortfall as a capture-recall problem, not an
absent surface.

ADR-0006: every metric is a structural count/shape; the only per-document input is the
one-off cat-3 / cat-1 measurement LABELING below (a measurement label, exactly like
DOC-INTENT-TAXONOMY.1 — NOT shipped runtime code; the runtime recognizer `.3` is
structural). No chip-name list drives any extraction.

Run from the repo root:  python3 scripts/measure_cat3_topology_recall.py
"""
from __future__ import annotations

import json
import os
import sys

ROOT = "generated/intent_ir"

# --- one-off measurement labeling (DOC-INTENT-TAXONOMY.1 reconstructed; not runtime) ---
# Cat-3 = platform / system-IP topology & integration (15 docs). Borderline docs that
# the `.1` census honest-caveats flag are annotated; the corpus verdict is robust to
# their inclusion (see the "core (borderlines removed)" aggregate the report cites).
CAT3 = {
    "100336_0106_00_2019_02_08_gic_600_technical_reference_manual": "GIC-600 TRM",
    "100798_0401_00_2020_07_31_cortex_a76_technical_reference_manual": "Cortex-A76 TRM [borderline cat-4]",
    "100806_0100_00_2017_08_01_coresight_soc_600_technical_reference_manual": "CoreSight SoC-600 TRM v1",
    "100806_0200_00_2017_12_08_coresight_soc_600_technical_reference_manual": "CoreSight SoC-600 TRM v2",
    "100806_0701_17_2025_06_30_coresight_soc_600_technical_reference_manual": "CoreSight SoC-600 TRM v7",
    "101130_0002_02_2018_11_30_coresight_sdc_600_technical_reference_manual": "CoreSight SDC-600 TRM",
    "101542_0102_08_2023_07_05_arm_mmu_700_technical_reference_manual": "MMU-700 TRM",
    "ddi0461_b_2010_12_10_coresight_trace_memory_controller_technical_reference_manual": "CoreSight TMC TRM",
    "ddi0471_a_2011_06_23_gic_400_technical_reference_manual": "GIC-400 TRM",
    "den0068_2018_07_23_coresight_base_system_architecture": "CoreSight Base System Arch",
    "ihi0029_e_2017_02_27_coresight_architecture_specification": "CoreSight Architecture",
    "ihi0069_g_2021_02_arm_generic_interrupt_controller_architecture_specification": "GIC Architecture [borderline cat-2]",
    "ihi0070_e_a_2023_01_31_arm_system_memory_management_unit_architecture_specification": "SMMU Architecture [borderline cat-2]",
    "ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification": "ARM Debug Interface v6 [borderline cat-1]",
    "ihi0076_a_2018_05_02_advanced_communications_channel_architecture_specification": "Advanced Comms Channel Arch",
}
# Docs whose cat-3 membership the `.1` census flags as genuinely borderline.
CAT3_BORDERLINE = {
    "100798_0401_00_2020_07_31_cortex_a76_technical_reference_manual",
    "ihi0069_g_2021_02_arm_generic_interrupt_controller_architecture_specification",
    "ihi0070_e_a_2023_01_31_arm_system_memory_management_unit_architecture_specification",
    "ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification",
}
# Clean cat-1 wire reference baseline (the WIRE-BASED-100 gold docs).
CAT1_WIRE = [
    "ihi0022_l_2025_08_amba_axi_protocol_specification",
    "ihi0024_e_2023_02_amba_5_apb_protocol_specification",
    "ihi0033_c_2021_09_amba_5_ahb_protocol_specification",
    "ihi0051_b_2021_04_amba_axi_stream_protocol_specification",
]


def _is_clean(name) -> bool:
    """An endpoint name is clean iff it is a non-empty string with no escape artifact."""
    return isinstance(name, str) and bool(name.strip()) and "\\" not in name


def metrics(key: str) -> dict:
    path = os.path.join(ROOT, key, "intent_ir.json")
    with open(path) as fh:
        d = json.load(fh)
    actors = len(d.get("actors") or [])
    sc = d.get("signal_connectivity") or []
    inf = d.get("infrastructure_signals") or []

    both = esc = endpoints = clean = 0
    for e in sc:
        prod = e.get("producer_actor_names") or []
        cons = e.get("consumer_actor_names") or []
        if prod and cons:
            both += 1
        names = prod + cons
        if any(isinstance(x, str) and "\\" in x for x in names):
            esc += 1
        for x in names:
            endpoints += 1
            if _is_clean(x):
                clean += 1
    inf_dist = sum(1 for s in inf if (s.get("distributed_to_actor_ids") or []))
    inf_src = sum(1 for s in inf if (s.get("recovered_source_actor_ids") or []))
    return dict(
        actors=actors, sc=len(sc), both=both, esc=esc,
        endpoints=endpoints, clean=clean,
        inf=len(inf), inf_dist=inf_dist, inf_src=inf_src,
    )


def aggregate(keys) -> dict:
    fields = ["actors", "sc", "both", "esc", "endpoints", "clean", "inf", "inf_dist", "inf_src"]
    acc = {f: 0 for f in fields}
    acc["docs"] = 0
    for k in keys:
        if not os.path.exists(os.path.join(ROOT, k, "intent_ir.json")):
            print(f"  WARNING: missing {k}", file=sys.stderr)
            continue
        m = metrics(k)
        acc["docs"] += 1
        for f in fields:
            acc[f] += m[f]
    return acc


def _pct(n: int, d: int) -> str:
    return f"{(100 * n // d) if d else 0}%"


def show(name: str, a: dict) -> None:
    epa = a["sc"] / a["actors"] if a["actors"] else 0.0
    print(f"\n=== {name} ({a['docs']} docs) ===")
    print(f"  actors={a['actors']}  signal_connectivity edges={a['sc']}  edges/actor={epa:.3f}")
    print(f"  both-endpoint edges={a['both']} ({_pct(a['both'], a['sc'])})   "
          f"escaped-name edges={a['esc']}   clean endpoints={a['clean']}/{a['endpoints']} ({_pct(a['clean'], a['endpoints'])})")
    print(f"  infrastructure_signals={a['inf']}  with-distribution={a['inf_dist']}  "
          f"with-RESOLVED-source={a['inf_src']}")


def main() -> int:
    if not os.path.isdir(ROOT):
        print(f"error: {ROOT} not found — run from the repo root with a built corpus", file=sys.stderr)
        return 2

    print("=== CAT-3 platform/system-IP per-doc topology capture ===")
    hdr = f"{'sc':>4}{'both':>5}{'esc':>4}{'inf':>4}{'idist':>6}{'isrc':>5}{'act':>5}{'e/act':>7}  label"
    print(hdr)
    for key, label in CAT3.items():
        m = metrics(key)
        epa = m["sc"] / m["actors"] if m["actors"] else 0.0
        print(f"{m['sc']:>4}{m['both']:>5}{m['esc']:>4}{m['inf']:>4}{m['inf_dist']:>6}"
              f"{m['inf_src']:>5}{m['actors']:>5}{epa:>7.2f}  {label}")

    show("CAT-3 platform/system-IP", aggregate(list(CAT3)))
    show("CAT-3 core (borderlines removed)", aggregate([k for k in CAT3 if k not in CAT3_BORDERLINE]))
    show("CAT-1 wire reference baseline", aggregate(CAT1_WIRE))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
