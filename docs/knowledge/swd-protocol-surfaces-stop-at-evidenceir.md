---
id: swd-protocol-surfaces-stop-at-evidenceir
title: SWD protocol surfaces are scored directly from EvidenceIR and do not yet reach IntentIR or ISF
answers:
  - "do SWD serial frame fields reach IntentIR"
  - "does the ISF adapter consume swd_operations or protocol_states"
  - "why can SWD score 100 percent while its protocol is absent downstream"
  - "where do interface_edge_timings stop in the pipeline"
  - "what owns the SWD EvidenceIR to IntentIR projection gap"
date: 2026-08-08
status: current
tags: [swd, evidence-ir, semantic-ir, intent-ir, isf, projection, completeness]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.7); crates/specforge/src/ir/evidence.rs; crates/specforge/src/commands/eval_extraction.rs
reverify: "! rg -n 'serial_frame_fields|swd_operations|protocol_states|interface_edge_timings' crates/specforge/src/ir/semantic.rs crates/specforge/src/ir/intent.rs crates/specforge/src/ir/isf_ir.rs"
---

The typed SWD protocol surfaces — `serial_frame_fields`, `swd_operations`, `protocol_states`, and
`interface_edge_timings` — are fields on `EvidenceIr`. `eval-extraction` loads and scores those fields
directly. The SemanticIR builder, IntentIR builder, and `.isf` adapter contain no consumer for them.

Therefore the 29/29 SWD derivation score proves extraction fidelity at EvidenceIR, not canonical-product
availability. The protocol cannot yet drive the downstream FSM/serial representation. The dedicated
`SWD-SERIAL-EXTRACTION.7` leaf owns a typed Evidence→Semantic→Intent projection audit and only the ISF
lowering supported by the already-proven FSM/serial idioms. This boundary must be closed as one architecture
slice; adding more EvidenceIR-only scores would not close it.
