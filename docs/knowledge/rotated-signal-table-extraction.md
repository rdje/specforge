---
id: rotated-signal-table-extraction
title: Misaligned signal tables (name column rotated to last) are extracted by content-based column detection
answers:
  - "why was a signal not extracted from a signal table (e.g. AHB HREADY)"
  - "how does specforge handle a signal table whose name column is not first"
  - "what is content-based name-column detection / rotation offset remapping"
  - "how was AHB HREADY recovered for the temporal antecedent"
  - "what does synthesize_signal_declarations do when the body is rotated"
  - "how is an unless/except exception clause handled in a temporal condition"
date: 2026-06-07
tags: [extraction, signal-declaration, temporal, wire-based-100, docling, adr-0006]
evidence: docs/tasks/WIRE-BASED-100.md (.5h); crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations); crates/specforge/src/ir/semantic.rs (parse_temporal_condition_predicates)
reverify: ./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule
---

The PDF backend (docling) sometimes **rotates** a signal table's body so the Signal-name column is not
where the header says — e.g. AHB `table_0009` has header `[Name|Destination|Width|Description]` but the
name is in the **LAST** body column (`HRDATA`/`HREADY`/`HRESP`/…), with Destination/Width shifted left.
Same shape as APB `table_0016`. Header-position name-column detection then reads an actor/width cell as
the "name" and **skips every row**, so a signal whose ONLY source is such a table (AHB `HREADY`) is
never declared.

Fix (`WIRE-BASED-100.5h`, the `.3a`-deferred extractor work): `synthesize_signal_declarations` finds the
name column by **content** — the body column with the most *distinct* hardware-signal tokens — and remaps
the other header-derived columns (width/direction/source/destination) by the same **rotation offset**.
It overrides only on a clear content disagreement (a different column out-densities the header name col),
so aligned tables are unchanged (offset 0). Purely positional/structural — no signal name hardcoded
(ADR 0006). This recovered `HREADY`, which made the temporal antecedent `HREADY=HIGH` resolvable.

Companion fix (same leaf): `parse_temporal_condition_predicates` drops a trailing `unless`/`except`
**exception** clause — "valid when HREADY is HIGH, unless HRESP is ERROR" → antecedent `HREADY=HIGH` only
(an exception is a negative caveat, not a conjunctive antecedent). Together these took **AHB temporal to
`P=R=F1=1.000`** (fresh re-ingested evidence), so AHB now matches APB on all three aspects (constraints +
relations + temporal). See `[[apb-signal-catalog-fully-extracted]]` (the `.3a` gauge-side fix) and
`[[indexed-signal-family-canonicalization]]` (the `.4` prose-ref resolver).
