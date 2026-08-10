---
id: gic-overview-guide-refresh-is-authority-empty
title: The GIC Overview Guide carries interrupt constraints, not an 83-interface hardware surface
answers:
  - "why does the refreshed Generic Interrupt Controller Overview Guide emit no ISF"
  - "what happened to the stale 83 interfaces in the GIC Overview Guide"
  - "are GICv2 GICv3 and GICv4 CPU-family rows timing constraints (no)"
  - "what grounded intent remains in the GIC Overview Guide"
  - "what are the final GIC Overview Guide reproducibility hashes"
  - "what is corpus refresh 49"
  - "how many corpus refreshes remain after the GIC Overview Guide"
date: 2026-08-10
tags: [gic, methodology-guide, interface-authority, timing-authority, evidence-ir, intent-ir, isf, corpus-coverage]
evidence: docs/tasks/corpus-coverage/refreshes-49-56.md; generated/evidence_ir/198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide/evidence_ir.json; generated/intent_ir/198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide/intent_ir.json
reverify: "target/release/specforge validate generated/evidence_ir/198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide/evidence_ir.json && target/release/specforge validate generated/intent_ir/198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide/intent_ir.json && target/release/specforge validate generated/adapters/isf/198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide/adapter.json"
---

The stale chain grouped CPU names, exception levels, register identifiers, and interrupt labels into 83 heuristic
interfaces. It also promoted 26 generic section headings into phases and 54 whole prose sentences into gates.
Those compatibility surfaces produced 83 behaviors and a renderable `controller.isf` even though EvidenceIR had
zero declared signals, actor-signal relations, signal constraints, or registers. They were population, not typed
hardware-boundary authority.

The current generic authority gates preserve the engineering content without the invented topology. EvidenceIR
retains 439 statements, 33 links, and 12 narrative conditionals. SemanticIR retains seven actors, 87 invariants,
four contracts, one assertion, nine abstractions, and 46 decomposition candidates; IntentIR retains three actors,
four grounded contract behaviors, 88 constraints, nine assumptions, and one recognition-only transaction. Both
stages contain zero interfaces or typed timings. Adapter lowering therefore blocks only on `no signals declared
in interface`, emits no target, and removes the stale `controller.isf`.

The only pre-semantic typed delta is also a correction: a table mapping GICv2/GICv3/GICv4 to compatible Arm CPU
families is now classified as a feature matrix instead of a timing-parameter table. Its three records had copied
CPU-family prose into `typ_value` and the GIC version into `unit`; none expressed a physical time quantity. Current
EvidenceIR consequently carries zero timings instead of three.

Two guarded CPU ingests reproduce 45 pages / 44 visuals / 13 tables / 58 sections / 430 elements and a 139-file /
30,731,394-byte normalized bundle at SHA-256
`b1905855cbacf03a4a2fc07644f2418749d85a00d1d02e78fe3bb46cde1e4b56`. Final validated SourceIR is
`421166d5f13c914229462e034cc0311e90b0891e9cc783fa08f5f9d364181dd5`; the eight downstream artifact/report
hashes combine to `99bd34c14c5bb638df35e183ab4dea68f6e7a2769f5a6ea81fff915d33a89a00` across both runs. Corpus
coverage is 49 done / seven remaining at stages 80/21/80/79, with all 58 retained emitted ISFs strict-clean.
