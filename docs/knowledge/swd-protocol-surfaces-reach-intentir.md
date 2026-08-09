---
id: swd-protocol-surfaces-reach-intentir
title: SWD protocol surfaces reach IntentIR, ISF residuals, and convergence accounting exactly
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
  - "does every SWD protocol record receive an ISF adapter disposition"
  - "what are the isf_protocol residual packet prefixes"
date: 2026-08-09
status: current
tags: [swd, evidence-ir, semantic-ir, intent-ir, isf, projection, completeness]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.7e.i); docs/decisions/0016-swd-protocol-projection-and-honest-isf-boundary.md; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/ir/adapters.rs; crates/specforge/src/commands/converge.rs
reverify: "rg -n 'serial_frame_fields|swd_operations|protocol_states|interface_edge_timings|protocol_residual_decisions' crates/specforge/src/ir/semantic.rs crates/specforge/src/ir/intent.rs crates/specforge/src/ir/adapters.rs crates/specforge/src/commands/converge.rs && ! rg -n 'serial_frame_fields|swd_operations|protocol_states|interface_edge_timings' crates/specforge/src/ir/isf_ir.rs"
---

The typed SWD protocol surfaces — `serial_frame_fields`, `swd_operations`, `protocol_states`, and
`interface_edge_timings` — originate on `EvidenceIr`, and `eval-extraction` scores those fields directly.
`SWD-SERIAL-EXTRACTION.7b` adds the same exact record types to SemanticIR, and `.7c` adds them to canonical
IntentIR. Both builders clone each collection without filtering, sorting, rewriting ids, or changing
provenance; empty collections are serde-defaulted and omitted, so older artifacts remain compatible. SemanticIR
and IntentIR validation report all four collection counts.

Therefore the 29/29 SWD derivation score proves fresh extraction fidelity at EvidenceIR, while `.7b` and `.7c`
prove lossless availability through the canonical product boundary. `.7d` accounts for every projected record
with one ordered, stable residual packet whose id identifies its surface and upstream record. Supporting
statement ids and the surface-specific missing bindings remain visible in `adapter.json`.

Direct behavioral lowering is currently unsafe. State records have no transitions, guards, initial state,
or encoding; frame/operation/edge records lack complete wire, value, activation, and storage bindings; and
SpecForge's typed ISF step model does not carry the proven FSM idiom's `select` expression. ADR 0016 therefore
requires exact typed carry-through and residualization of every under-specified record. The presently safe
directly lowerable subset is empty. Protocol residuals do not enter `IsfIr`, alter emitted counts/source, or
make otherwise blocked input renderable. Unrelated, independently licensed ISF may still render and pass
FSMGen strict, while the adapter artifact makes the missing protocol behavior explicit.

`SWD-SERIAL-EXTRACTION.7e.i` also makes the pipeline convergence snapshot retain exact ordered copies of all
four collections at EvidenceIR, SemanticIR, and IntentIR. Each record contributes once to its stage's
`fact_count`, so additions and removals affect the aggregate monotone guard. Snapshot equality compares the
complete typed vectors, so same-cardinality content changes and reordering are visible too. Empty collections
add zero and preserve prior behavior for documents without protocol records.

The promoted canonical ADI chain proves the complete boundary on real data: the four collections have exact
11/4/13/1 membership at EvidenceIR, SemanticIR, and IntentIR; the adapter emits 29 corresponding protocol
residual packets in stable order and remains renderable; the generated ISF passes FSMGen strict with zero
diagnostics. This is canonical availability plus honest disposition, not a claim of executable SWD lowering.
