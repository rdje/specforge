---
id: isf-emission-blocked-by-clock-reset-contract
title: Two of 27 live chains emit an .isf file (AXI, AHB) since their clock/reset contract is read from their own sentences; the other 25 stay blocked, and 150 adapter residuals were attributed per side
answers:
  - "why does no live chain emit an isf file"
  - "which live chains emit an isf file"
  - "where does the isf reset polarity come from"
  - "does fsmgen honour a stated reset polarity"
  - "why is every isf adapter blocked"
  - "why is lowering_status blocked"
  - "which IntentIR families does the isf adapter never read"
  - "how many adapter residuals are SpecForge gaps and how many FSMGen gaps"
  - "does the isf adapter use signal_polarities"
  - "why was AWLOCK lowered as 1 when the spec says deasserted"
  - "what value does an isf rule drive when the spec states none"
  - "what is isf_obligations_not_lowered"
  - "can FSMGen state stable or must-not-change obligations"
  - "are enum members legal isf rule values"
  - "how do I measure what IntentIR carries versus what the isf adapter lowers"
date: 2026-09-25
status: current
tags: [isf, isf-adapter, fsmgen, census, extraction-representation-gaps, measured, clock, reset]
evidence: "scripts/measure_intent_to_isf_reach.py; crates/specforge/src/ir/evidence.rs (synthesize_system_reset_prose_statements); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.7.a, .7.b, .21, .22); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.2 census); crates/specforge/src/ir/adapters.rs (assess_isf_renderability); crates/specforge/src/ir/semantic.rs (build_system_contract); crates/specforge/src/ir/isf_ir.rs (isf_literal_value); subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md (Enum member values; Immediate verification checks); subs/fsmgen/docs/book/src/13d-control-flow.md (assert)"
reverify: "python3 scripts/measure_intent_to_isf_reach.py blocking — expect 27 live chains, 2 'renderable' (AXI, AHB) and every other line containing 'no clock/reset contract'; python3 scripts/measure_intent_to_isf_reach.py residuals | tail -1 — expect 'total: 143 packets in 25 families' (150/28 at .2; .8 left 145/25 without updating this line; .7.b lowered the two SYSCOREQ/SYSCOACK rules) until an owning leaf moves it."
---

**Updated `2026-09-25` (`EXTRACTION-REPRESENTATION-GAPS.7.b`): AXI and AHB now emit `manager.isf`.** A prose
reader takes a reset's kind and polarity from the sentence that calls a declared signal *the reset signal*, and
the contract merges a kind and a polarity stated apart: `(reset (ARESETn async active_low))`,
`(reset (HRESETn async active_low))`, both accepted by FSMGen `--strict`. APB-E states a polarity but no kind
(`.7.d`); Wishbone, I2C, I2S and ADIv6 are `.7.c`/`.7.e`. FSMGen at the pin drops a stated reset polarity (an
`async` reset is always active low, keyed on the name under `--strict`): `.21`. Emission exposed an older adapter
defect, rules that drop or invent their condition: `.22`.

**Measured `2026-09-25` (`EXTRACTION-REPRESENTATION-GAPS.2`).** All 27 live adapters are `lowering_status:
blocked`: 27 have no `system_contract` (20 also no interface signals, 4 also no behaviour). The 7 signal-bearing
documents (AXI, APB 5, AHB, ADIv6, I2C, I2S, Wishbone) are blocked by the contract alone. `build_system_contract`
reads only canonical `Clock X.` / `Reset X …` statements and needs all seven fields; nothing derives them from a
signal table, and `infrastructure_signals` is empty on all 27. FSMGen can state it (13a). Owner:
`EXTRACTION-REPRESENTATION-GAPS.7`.

The adapter reads 18 IntentIR design families and never reads about 20. The untyped `constraints` / `behaviors`
/ `assumptions` (7,477 records) cannot lower until typed (`.3`); `timing_constraints` (237) are physical and
electrical parameters, not requested of FSMGen by decision; `signal_polarities` (51) were never consulted.

**Fixed `2026-09-25` (`.8`): the adapter invented drive values.** Its three rule loops ignored the constraint kind
and gave every value-less obligation `1` (AXI *AWLOCK must be deasserted* → `(AWLOCK 1)`; APB *PADDR must remain
stable* → `(PADDR 1)`; *AWCMO must be 0b000* → `1`), and `isf_literal_value` mapped `asserted` to `1` for every
signal. FSMGen strict accepted all of it. Now a drive is a stated level resolved through the signal's stated
polarity (`resolve_level`); the rest is listed in `isf_obligations_not_lowered`. On the 6 affected chains the rules
went 250 → 95 with 44 AXI values corrected and 166 obligations listed (53 for unstated polarity, `.17`).

Two FSMGen facts that correct older adapter wording: **enum members are legal rule and drive values** (13k *Enum
member values*), and **`(assert …)` supports `(stable SIG)`** (13k *Immediate verification checks*), so the
residual text *"bare stability … has no supported .isf construct"* is stale (`.11`). `(assert …)` is
transaction-level only (13d), and there is no known-value predicate: the FSMGen halves `.15` and `.16`.

The 150 residual packets split: protocol records 51 (`.12`), conflicting or overlapping drives 45 (`.10`; the
`AWSNOOP` case is a configuration condition plus a tie-off for an absent signal), validity 11 (`.2a.vii.e` +
`.16`), stability/exclusion 10 (`.11` + `.15`), value words 5 (`.9`), registers 5 (`.13`), width parameters 9,
upstream stages 14.

Links: [[isf-parameter-width-contract]], [[fsmgen-ignores-signal-direction]].
