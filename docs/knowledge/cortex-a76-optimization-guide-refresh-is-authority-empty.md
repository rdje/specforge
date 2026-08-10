---
id: cortex-a76-optimization-guide-refresh-is-authority-empty
title: The Cortex-A76 optimization guide is constraint evidence, not a 537-output instruction interface
answers:
  - "why does the refreshed Cortex-A76 Software Optimization Guide emit no ISF"
  - "what happened to the stale Cortex-A76 consumer.isf with 537 outputs"
  - "are instruction mnemonics in a software optimization guide declared interface signals (no)"
  - "what grounded content survives the Cortex-A76 optimization guide refresh"
  - "what are the final Cortex-A76 SourceIR normalized and downstream reproducibility hashes"
  - "what is corpus refresh 47 and why is its adapter honestly blocked"
date: 2026-08-10
tags: [cortex-a76, optimization-guide, instruction-topology, adapter, source-ir, corpus-coverage]
evidence: docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.47); generated/evidence_ir/pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide/evidence_ir.json; generated/intent_ir/pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide/intent_ir.json
reverify: "target/release/specforge validate generated/evidence_ir/pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide/evidence_ir.json && target/release/specforge validate generated/intent_ir/pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide/intent_ir.json && target/release/specforge validate generated/adapters/isf/pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide/adapter.json"
---

The stale Cortex-A76 Software Optimization Guide chain looked complete because it was richly populated, not
because the document declared a hardware boundary. It grouped 228 instruction names into interfaces, projected
18 generic phases and 23 generic gates, derived 37 behaviors, and emitted 537 output ports in `consumer.isf`.
Those names are instruction mnemonics and performance-table entries for software optimization; no grounded
relation, declaration, or signal inventory makes them top-level interface signals.

The current release preserves the guide's useful engineering content while removing that projection. SourceIR
contains 46 pages, 71 visuals, 68 tables, 68 sections, and 249 elements. EvidenceIR contains 960 statements and
one conditional with no timing, register, or relation record. SemanticIR contains four actors, 15 invariants,
and 44 decompositions but no interface, port, phase, gate, relation, or timing surface. IntentIR contains 15
constraints and two assumptions but no actor, behavior, interface, or timing surface. Adapter lowering therefore
blocks only on `no signals declared in interface`, emits no target, and retains exactly the two explicit
canonicalization residuals.

Three guarded CPU ingests reproduce the source boundary. The committed release pins SourceIR SHA-256
`a396a074543148fcd80c1d4540746ac10ac8d944d65c42658bc51d0a1fd9d478`, normalized-manifest SHA-256
`872e7dbaa0dcf8d9234b32d0d81d0b87a818f405abe55491f4568670eb4b1bae`, and a combined downstream content
hash of `b4b50237f0477486dee9683714f0b43562469ebe8f2661d8538caf9d6ea39ade` across two cascades. The first replay
also exposed the separate shared timing-table authority defect closed by [[timing-table-structural-authority]].
