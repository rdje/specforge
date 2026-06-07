---
id: axi-constraint-subject-must-be-declared
title: A signal-constraint subject must be a DECLARED signal (drops property/config/doc-meta noise)
answers:
  - "why was a property like RME_Support or MPAM_WIDTH extracted as a signal constraint"
  - "how does specforge reject non-signal constraint subjects (LICENSEE, AXI, RME, MPAM)"
  - "what is the constraint-subject-must-be-declared filter"
  - "how was AXI constraint precision fixed"
  - "where is the declared-signal gate applied (pattern + dynamic constraint paths)"
date: 2026-06-07
tags: [axi, extraction, signal-constraint, precision, wire-based-100, adr-0006]
evidence: docs/tasks/WIRE-BASED-100.md (.5i); crates/specforge/src/ir/evidence.rs (extract_signal_constraints, extract_dynamic_signal_constraints)
reverify: ./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_axi.json --provider skip 2>/dev/null | grep -A1 source-tolerant
---

Large specs (AXI: 310 signals) carry heavy **property/config prose** that looks constraint-shaped but is
NOT about a signal: "When RME_Support is False, GDI_Support must be False", "MPAM_WIDTH must be 11",
"granted to LICENSEE …". The subject extractor read `RME`/`GDI`/`MPAM`/`LICENSEE`/`AXI`/`AMBA` as
constraint subjects → AXI constraint precision ~0.375 (recall was already 1.000).

Fix (`WIRE-BASED-100.5i`): a constraint subject must be a **declared signal** — the document's own
catalog (`collect_known_signal_names`) decides what is a signal. Applied in BOTH constraint producers:
`extract_signal_constraints` (pattern) AND `extract_dynamic_signal_constraints` (dynamic — where the AXI
FPs actually came from, `dyn_sigcon_*`). Measured: **27 of 28 garbage subjects are NOT declared; all real
signal subjects ARE** — so the filter cleanly separates them. ADR-0006-safe (filters against the doc's own
declared signals, no hardcoded list). **Gated on a non-empty catalog** so a tiny no-declaration fixture is
not silently emptied (kg-bench safe).

Result: AXI `signal_constraint` source-tolerant `P=R=F1=1.000` (tp=3 fp=0 fn=0) on the seed; APB/AHB stay
1.000 (their subjects are declared; APB's PSTRB-LOW `dyn_sigcon_` survives). +2 hermetic tests; kg-bench
green. See `[[axi-channel-structure]]`. Note AXI relations/temporal golds are follow-on (the spec is large
— 514 relations).
