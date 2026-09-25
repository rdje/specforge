---
id: isf-emission-blocked-by-clock-reset-contract
title: No current chain emits an .isf file — all 27 live adapters are blocked because no chain carries a source-grounded clock/reset contract, and 150 adapter residuals are attributed per side
answers:
  - "why does no live chain emit an isf file"
  - "why is every isf adapter blocked"
  - "why is lowering_status blocked"
  - "which IntentIR families does the isf adapter never read"
  - "how many adapter residuals are SpecForge gaps and how many FSMGen gaps"
  - "does the isf adapter use signal_polarities"
  - "can FSMGen state stable or must-not-change obligations"
  - "are enum members legal isf rule values"
  - "how do I measure what IntentIR carries versus what the isf adapter lowers"
date: 2026-09-25
status: current
tags: [isf, isf-adapter, fsmgen, census, extraction-representation-gaps, measured, clock, reset]
evidence: "scripts/measure_intent_to_isf_reach.py; docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.2 census); crates/specforge/src/ir/adapters.rs (assess_isf_renderability); crates/specforge/src/ir/semantic.rs (build_system_contract); crates/specforge/src/ir/isf_ir.rs (isf_literal_value); subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md (Enum member values; Immediate verification checks); subs/fsmgen/docs/book/src/13d-control-flow.md (assert)"
reverify: "python3 scripts/measure_intent_to_isf_reach.py blocking — expect 27 live chains, every line containing 'no clock/reset contract'; python3 scripts/measure_intent_to_isf_reach.py residuals | tail -1 — expect 'total: 150 packets in 28 families' until an owning leaf moves it."
---

**Measured `2026-09-25` (`EXTRACTION-REPRESENTATION-GAPS.2`).** All 27 live adapters are `lowering_status:
blocked`: 27 have no `system_contract` (20 also no interface signals, 4 also no behaviour). The 7 signal-bearing
documents (AXI, APB 5, AHB, ADIv6, I2C, I2S, Wishbone) are blocked by the contract alone. `build_system_contract`
reads only canonical `Clock X.` / `Reset X …` statements and needs all seven fields; nothing derives them from a
signal table, and `infrastructure_signals` is empty on all 27. FSMGen can state it (13a). Owner:
`EXTRACTION-REPRESENTATION-GAPS.7`.

The adapter reads 18 IntentIR design families and never reads about 20. The untyped `constraints` / `behaviors`
/ `assumptions` (7,477 records) cannot lower until typed (`.3`); `timing_constraints` (237) are physical and
electrical parameters, not requested of FSMGen by decision; `signal_polarities` (51) are never consulted, so
`isf_literal_value` maps `asserted` to `1` for every signal, a latent inversion on active-low signals (`.8`).

Two FSMGen facts that correct older adapter wording: **enum members are legal rule and drive values** (13k *Enum
member values*), and **`(assert …)` supports `(stable SIG)`** (13k *Immediate verification checks*), so the
residual text *"bare stability … has no supported .isf construct"* is stale (`.11`). `(assert …)` is
transaction-level only (13d), and there is no known-value predicate: the FSMGen halves `.15` and `.16`.

The 150 residual packets split: protocol records 51 (`.12`), conflicting or overlapping drives 45 (`.10`; the
`AWSNOOP` case is a configuration condition plus a tie-off for an absent signal), validity 11 (`.2a.vii.e` +
`.16`), stability/exclusion 10 (`.11` + `.15`), value words 5 (`.9`), registers 5 (`.13`), width parameters 9,
upstream stages 14.

Links: [[isf-parameter-width-contract]], [[fsmgen-ignores-signal-direction]].
