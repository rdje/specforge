---
id: swd-protocol-surfaces-stop-at-evidenceir
title: SWD protocol surfaces reach canonical IntentIR exactly but await ISF accounting
answers:
  - "do SWD serial frame fields reach IntentIR"
  - "does the ISF adapter consume swd_operations or protocol_states"
  - "why can SWD score 100 percent while its protocol is absent downstream"
  - "where do interface_edge_timings stop in the pipeline"
  - "what owns the SWD EvidenceIR to IntentIR projection gap"
  - "why can SWD protocol records not be lowered directly to ISF"
  - "which SWD protocol facts are safe to lower today"
  - "do SWD protocol records reach SemanticIR"
  - "does SemanticIR preserve SWD protocol provenance and order"
  - "does IntentIR preserve SWD protocol provenance and order"
date: 2026-08-08
status: current
tags: [swd, evidence-ir, semantic-ir, intent-ir, isf, projection, completeness]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.7c); docs/decisions/0016-swd-protocol-projection-and-honest-isf-boundary.md; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/commands/validate.rs
reverify: "rg -n 'serial_frame_fields|swd_operations|protocol_states|interface_edge_timings' crates/specforge/src/ir/semantic.rs crates/specforge/src/ir/intent.rs && ! rg -n 'serial_frame_fields|swd_operations|protocol_states|interface_edge_timings' crates/specforge/src/ir/isf_ir.rs"
---

The typed SWD protocol surfaces — `serial_frame_fields`, `swd_operations`, `protocol_states`, and
`interface_edge_timings` — originate on `EvidenceIr`, and `eval-extraction` scores those fields directly.
`SWD-SERIAL-EXTRACTION.7b` adds the same exact record types to SemanticIR, and `.7c` adds them to canonical
IntentIR. Both builders clone each collection without filtering, sorting, rewriting ids, or changing
provenance; empty collections are serde-defaulted and omitted, so older artifacts remain compatible. SemanticIR
and IntentIR validation report all four collection counts.

Therefore the 29/29 SWD derivation score proves fresh extraction fidelity at EvidenceIR, while `.7b` and `.7c`
prove lossless availability through the canonical product boundary. The records still do not reach ISF; `.7d`
owns explicit adapter disposition for every record.

Direct behavioral lowering is currently unsafe. State records have no transitions, guards, initial state,
or encoding; frame/operation/edge records lack complete wire, value, activation, and storage bindings; and
SpecForge's typed ISF step model does not carry the proven FSM idiom's `select` expression. ADR 0016 therefore
requires exact typed carry-through and residualization of every under-specified record. The presently safe
directly lowerable subset is empty. Unrelated, independently licensed ISF may still render, but it cannot
silently stand in for the missing protocol behavior.
