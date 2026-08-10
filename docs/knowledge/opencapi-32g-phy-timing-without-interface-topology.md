---
id: opencapi-32g-phy-timing-without-interface-topology
title: OpenCAPI 32G PHY Signaling is timing-rich but has no grounded digital interface topology
answers:
  - "why does the refreshed OpenCAPI 4.0 32 Gbps PHY Signaling specification emit no ISF"
  - "what happened to the stale OpenCAPI CDR DDJ DL and DL3 ports"
  - "how many timing constraints remain in OpenCAPI 32G PHY Signaling after refresh (60)"
  - "what is the OpenCAPI 32G PHY Signaling visual and NLP capture frontier"
  - "what are the final OpenCAPI 32G PHY Signaling SourceIR and normalized bundle hashes"
date: 2026-08-10
tags: [opencapi, physical-link, timing, signal-authority, evidence-ir, intent-ir, isf, corpus-coverage]
evidence: docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.48); generated/evidence_ir/opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020/evidence_ir.json
reverify: "target/release/specforge validate generated/evidence_ir/opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020/evidence_ir.json && target/release/specforge validate generated/adapters/isf/opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020/adapter.json"
---

The stale OpenCAPI 4.0 32 Gbps PHY Signaling chain treated four glossary or measurement acronyms—`CDR`, `DDJ`,
`DL`, and `DL3`—as synthetic width-one signals. That weak surface produced three interfaces, a generic `TABLE`
enum, and a renderable four-port `channel.isf`, despite zero direction relations, signal constraints, registers,
or signal-table declaration provenance. Physical timing evidence had been mistaken for digital topology.

The current pipeline preserves the grounded engineering surface. EvidenceIR contains 547 statements, two
narrative conditionals, and 60 timing constraints; SemanticIR contains eight actors, 75 invariants, and 21
contracts; IntentIR contains seven actors, 21 behaviors, 75 constraints, four assumptions, and the same 60
timings. It carries zero interfaces, ports, actor-signal relations, or declared signals. The adapter therefore
blocks only on `no signals declared in interface`, emits no target, and carries no false enum or port inventory.

Two guarded CPU ingests reproduce SourceIR `e2c0b9e538ed241d3954bbd3365d2da62a5cba20ef2b1f27951498befbc4859f`
and 130 normalized files / 34,843,407 bytes at bundle hash
`30f705117a5de7b8b0ecd2f11dea4c65d98ef66ac167c82282e4e82ba56e8232`; two final cascades reproduce all eight
downstream artifact/report hashes. Validation keeps the honest recall frontier visible: 39 visuals lack VLM
observations, 27 normative statements remain partially structured, two conditionals lack actor-signal grounding,
and eight timing/conditional source IDs lack typed temporal rules. See also
[[timing-scalar-rows-require-independent-cell-geometry]] and [[timing-table-structural-authority]].
