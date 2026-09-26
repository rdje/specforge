---
id: an-unstated-reset-kind-is-a-listed-choice
title: When a document names its reset and states its polarity but not whether it is synchronous, the system contract records the kind as `not_stated` and the `.isf` adapter emits the kind that keeps the stated polarity at the pinned FSMGen (`async` for active-low, `sync` for active-high), listed as `isf_reset_kind_not_stated`; APB-E emits `requester.isf` this way since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.7.d)
answers:
  - "why does APB-E's requester.isf say (reset (PRESETn async active_low))"
  - "what is SystemResetKind::NotStated / reset_kind not_stated"
  - "what is the residual isf_reset_kind_not_stated"
  - "why is the unstated reset kind async and not sync"
  - "why was APB-E blocked with no source-grounded system clock/reset contract"
  - "why does a windowed monitor refuse a value that is not a level (must be valid)"
  - "how many .isf files are emitted (AXI, AHB, APB-E)"
date: 2026-09-26
tags: [isf, reset, system-contract, apb, fsmgen, extraction-representation-gaps, decision, measured]
evidence: crates/specforge/src/ir/semantic.rs (build_system_contract, SystemResetKind::NotStated); crates/specforge/src/ir/isf_ir.rs (the reset timing match, reset_kind_not_stated_residual_packet, windowed_event_refusal); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.7.d, .21, .29-.31); docs/book/src/pipeline/isf-adapter.md (Where do the clock and reset come from?)
reverify: "cargo test --offline -p specforge-core --lib -- an_unstated_reset_kind a_windowed_event_lowers_only_a_signal_becoming_true. jq -r '.system_contract.reset_kind' generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json -> not_stated; subs/fsmgen/bin/fsmgen --quiet --strict --check --json generated/adapters/isf/ihi0024_e_2023_02_amba_5_apb_protocol_specification/requester.isf -> 0 diagnostics (2026-09-26)."
---

**Decided and landed `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.7.d`).** APB-E states *PRESETn is the reset signal
and is active-LOW*, but never whether the reset is synchronous. `build_system_contract` required a stated kind, so it
refused the whole contract, and APB-E's adapter was blocked (*no source-grounded system clock/reset contract*).

**The decision:** an implementation choice, listed, as `.4` already does for a width the document leaves open.

- The contract records `reset_kind: not_stated`, with its timing and target also `not_stated`.
- FSMGen's reset clause has no unstated kind. So the adapter emits the kind that keeps the stated polarity at
  SpecForge's FSMGen pin, where an `async` reset always comes out active-low and a `sync` one active-high (`.21`):
  `async` for APB-E.
- `isf_reset_kind_not_stated` says that this kind is SpecForge's choice, not the document's.
- No FSMGen request is raised: once `.21`'s fix is published, either kind keeps the polarity.

**What emitting APB-E exposed.** A windowed event lowered as `(monitor (within SIGNAL N))` ignored its value. So
*PADDR, PWRITE, and PWDATA must be valid* would have become *PADDR is true within a cycle*. Now a windowed event lowers
only when its value resolves to `1` (`windowed_event_refusal`, in both classifiers).

Those rules carry three more defects, each with its own leaf:
- windows read from diagram labels, *occurs at T1 in Figure 3-1* (`.29`);
- a fusion that joined differing obligations (`.30`);
- the condition lost in *PSEL is asserted, which means that …* (`.31`).

APB-E's `requester.isf` is strict-clean. Three files are emitted now: AXI, AHB and APB-E.
`[[a-hold-keeps-its-release]]` · `[[isf-initiator-perspective-direction]]`.
