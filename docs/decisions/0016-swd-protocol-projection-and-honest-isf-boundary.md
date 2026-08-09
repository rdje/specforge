# 0016 — SWD protocol projection is lossless; ISF lowering requires complete behavioral bindings

- Date: 2026-08-09
- Status: accepted
- Deciders: SpecForge architecture audit under `SWD-SERIAL-EXTRACTION.7a`

## Context

EvidenceIR carries four independently scored protocol surfaces: `serial_frame_fields`,
`swd_operations`, `protocol_states`, and `interface_edge_timings`. Their records preserve field width,
range, phase, direction, ordering, response branches, state identity/action, edge identity, and source
statement provenance. At the `.7a` decision audit, `eval-extraction` read them directly, but SemanticIR,
IntentIR, and the typed ISF adapter did not carry or account for them.

The current records do not license an executable state machine or serial transaction by themselves.
`ProtocolStateRecord` has no transition, guard, initial-state, or encoding fields. A serial-frame record
does not bind the frame to a declared wire or provide literal values for all control fields. An SWD
operation does not bind activation, ports, or field-level steps. An interface-edge record does not name a
sample destination, a drive value, or an activation condition. In addition, SpecForge's typed ISF
transaction-step model supports `switch`, `set`, sample, shift, and related steps, but does not represent
the empirically proven FSM idiom's `select` expression.

At the `.7a` decision audit, canonical SWD EvidenceIR contained 11 frame fields, four operations, 13 states,
and zero interface-edge records even though the fresh `.4e` proof scored 29/29. This was not an extractor
regression: `.4e` used a disposable repository-local CPU re-ingest and deliberately did not promote it
while the cross-stage path-portability defect was open. The old canonical SourceIR's normalized Markdown
leaf was reclaimed, so rebuilding EvidenceIR from that artifact now fails closed. The tracked source PDF
remains available for a fresh ingest after projection implementation.

## Decision

1. SemanticIR and IntentIR will carry the four record collections losslessly, using the exact typed
   records and provenance rather than translating them into weaker generic records. The fields are
   additive, serde-defaulted, and omitted while empty so older artifacts remain loadable and unrelated
   documents retain their current serialized shape.
2. Each stage validator will report the four collection counts. Cross-stage tests will require exact
   equality, including record order and provenance, so projection cannot silently filter or reinterpret
   evidence.
3. The ISF adapter must account for every projected protocol record. A record may be lowered only when
   its own typed data supplies every binding required by a supported ISF construct. Otherwise the adapter
   emits an explicit residual decision that identifies the surface and missing semantic bindings.
4. No current SWD protocol record is directly behaviorally lowerable under that rule. The adapter may
   still emit unrelated, independently licensed ISF content; protocol residuals do not make that content
   unrenderable, but they prevent the artifact from implying complete protocol lowering.
5. After the projection and adapter-accounting slices land, the tracked ADI source will be freshly
   re-ingested on CPU into repository-local storage. The complete SourceIR→EvidenceIR→SemanticIR→IntentIR→
   adapter chain will then be promoted and verified, including the canonical 29/29 extraction score.

## Consequences

- A perfect extraction score and canonical-product availability become separately visible claims.
- The source's typed facts survive unchanged to the IntentIR product boundary even when executable ISF
  needs semantics the source record has not yet captured.
- Adding transitions, guards, initial state, encodings, signal/value bindings, or a typed ISF `select`
  expression remains legitimate future work; none may be guessed as part of projection.
- Existing generic ISF output remains usable while its residual packet states exactly which protocol
  behavior was not lowered.
- Canonical freshness was restored under `.7e.ii` by a real ingest, not by editing or treating the stale cache
  as current; the promoted chain now has exact 11/4/13/1 parity through IntentIR.

## Links

- `docs/tasks/SWD-SERIAL-EXTRACTION.md`
- `docs/knowledge/swd-protocol-surfaces-reach-intentir.md`
- `docs/knowledge/swd-canonical-protocol-artifact-is-current.md`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/intent.rs`
- `crates/specforge/src/ir/isf_ir.rs`
- `crates/specforge/src/ir/adapters.rs`
