---
id: behavior-temporal-lowering-broader-corpus
title: Bar #5/#6 (behavior/temporal lowering, no silent drop) HOLDS on the broader 78-doc corpus — the 160 undeclared-named-subject rule drops are field content + prose/VLM noise, not wire intent; the adapter's silent skip is correct (KG-ISF-COMPLETENESS.4)
answers:
  - "are behavior or temporal rules silently dropped to the .isf on the broader 78-doc corpus"
  - "did doubling the corpus (36->78) introduce a new ISF lowering silent-drop gap"
  - "should specforge record an adapter residual for rules whose subject is not a declared signal"
  - "what are the undeclared-named-subject rule drops at ISF lowering (conditional_rules / signal_constraints / temporal_invariants)"
  - "why do NVMe/CCIX/DTI register-field obligations not lower to the .isf"
  - "is the ATP ihi0082 ARVALID/RVALID/RREADY drop a lowering gap"
  - "is bar #5 (no silent drop of behavior/temporal) still honest after CORPUS-COVERAGE.0"
date: 2026-06-17
tags: [kg-isf-completeness, isf, isf-adapter, lowering, fidelity, bar-5, bar-6, temporal, conditional-rules, signal-constraints, no-silent-drop, residual-honesty, measured, wire-based-100, adr-0006, no-build]
evidence: docs/research/behavior-temporal-lowering-completeness-78doc.md; crates/specforge/src/ir/isf_ir.rs (from_intent_ir signal_names:673-719; conditional_rules:1055 / signal_constraints:1075 / temporal_invariants:1092 each continue-skip with no residual; temporal_rules/actor_contracts residualize); docs/tasks/KG-ISF-COMPLETENESS.md (.4)
reverify: "Read-only python over generated/intent_ir/*/intent_ir.json (78 docs), replicate signal_names (interfaces[].signal_records names minus clock/reset) + the three (rule) filters -> conditional_rules 2237/516 lower/1670 no-consequent/51 undeclared-named; signal_constraints 369/287/0/82; temporal_invariants 29343/286/29030 empty-subject/27; temporal_rules 320 + actor_contracts 211 residualize. Cross-check each of the 160 undeclared-named subjects vs register_records/message_field_records/actor_ports: 5 on register, 155 nowhere but all field-mnemonic / prose-acronym / hex-literal / garbled-VLM-fragment. 0 undeclared drops on APB/AHB/AXI/SWD."
---

**Measured `2026-06-17` (`KG-ISF-COMPLETENESS.4`, read-only, docs-only).** Re-runs the `.2` ISF-lowering
fidelity gauge (`[[isf-lowering-fidelity-gauge]]`, originally 36 docs) over the full **78-doc** corpus
after `CORPUS-COVERAGE.0` doubled it with register/coherency/command/profile docs. Serves the owner north
star (`[[project_kg_isf_completeness]]`) bar #5 (no silent behavior/temporal drop) + bar #6 (round-trip),
and the standing "re-assess on the broader corpus" candidate.

**The three `(rule)` surfaces `continue`-skip drops with NO residual** when the subject is empty or
undeclared; the `temporal_rules`/`actor_contracts` path residualizes (`[[isf-temporal-lowering-no-silent-drop]]`).
78-doc breakdown: `conditional_rules` 2237/516 lower/1670 no-consequent/**51 undeclared-named**;
`signal_constraints` 369/287/0/**82**; `temporal_invariants` 29343/286/29030 empty-subject/**27**.

**The 30 700 empty-subject / no-consequent drops are correctly silent** ("absence is not an event" — a
ToC heading classified as an invariant has no subject to assert; mass-residualizing them is the dishonest
noise `.2`/`.2b` rejected).

**The 160 undeclared-NAMED-subject drops are field content + noise, NOT wire intent** (item-inspected: 5
already on the register surface; 155 = register/message FIELD mnemonics like NVMe `MTFA`/`HMDLAL`, CCIX
`SAMH`/`ESMD`, RISC-V `DC.tc.SXL` — homed on the field surfaces, which the `.isf` does not lower by design;
DTI message-field obligations leaked into `signal_constraints` because DTI carries **no
`message_field_records`**; prose/hex noise `DMA`/`TLB`/`PCI`/`IBM`/`FFFF`/`Reserved`/`this bit`; and the
ATP `ihi0082` cluster of real AXI names from **garbled** VLM fragments `"RREADY is RBR"`). **0 undeclared
drops on all 4 wire docs.**

**Conclusion: bar #5/#6 HOLDS at 2× corpus scale — NO-BUILD for an adapter lowering-residual.** The
adapter's silent skip of an undeclared-subject rule is **correct**: a field obligation must be routed
upstream to the field surface (not residualized at the adapter, which would mask the upstream issue), and a
prose/hex/VLM-fragment subject must be filtered upstream (not residualized = noise). This **confirms +
extends `.2`/`.2b`** to the doubled corpus. The `[[isf-lowering-fidelity-gauge]]` 36-doc reverify numbers
are superseded by the 78-doc snapshot here.

**Genuine gaps are UPSTREAM (spun out, each needs own ownership):** (1) **DTI `ihi0088` message-field
recognition** — its message-field tables aren't recognized, so field obligations leak to
`signal_constraints` (most actionable next; `EXTRACTION-QUALITY-GAUGE.FIELD`/`PDF-VARIANT-DIGESTION`);
(2) signal-inventory prose noise (`AMBA`/`ARM`/`APCI` minted as DTI signals); (3) ATP VLM-fragment quality.

See `[[isf-lowering-fidelity-gauge]]`, `[[isf-temporal-lowering-no-silent-drop]]`,
`[[relation-completeness-staleness-vs-absence]]`, `[[project_kg_isf_completeness]]`,
`[[feedback_not_complete_attack_substantive_gaps]]`, `[[feedback_scoring_rigor]]`.
