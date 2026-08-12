---
id: swd-protocol-convergence-snapshots-are-exact
title: Convergence snapshots retain and count the exact four SWD protocol collections
answers:
  - "does converge detect a protocol only SWD change"
  - "do convergence fact counts include serial frame fields"
  - "do EvidenceSnapshot SemanticSnapshot and IntentSnapshot count protocol states"
  - "does convergence detect a same count protocol rewrite"
  - "does convergence preserve protocol record order"
date: 2026-08-09
status: superseded
tags: [swd, converge, snapshot, fact-count, completeness]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.7e.i, .7e.ii); crates/specforge/src/commands/converge.rs
reverify: "cargo test -p specforge --lib protocol_only_changes_are_visible_in_every_convergence_snapshot && rg -n 'serial_frame_fields|protocol_operations|protocol_states|interface_edge_timings' crates/specforge/src/commands/converge.rs"
---

> Superseded in schema detail on `2026-08-12`: snapshot conservation remains exact, but the operation carrier
> is now `protocol_operations` with document-derived generic records. See
> [[evidenceir-generic-protocol-semantics]].

`EvidenceSnapshot`, `SemanticSnapshot`, and `IntentSnapshot` retain exact ordered copies of
`serial_frame_fields`, `swd_operations`, `protocol_states`, and `interface_edge_timings`. Each collection length
contributes to that stage's `fact_count`, and the pass-level EvidenceIR fact count includes the same records.
Empty collections contribute zero, preserving the prior behavior of documents without protocol records.

Cardinality and identity have separate jobs. The fact count exposes additions/removals to the monotone guard;
full snapshot equality detects same-cardinality field changes and record reordering. The regression builds a
real EvidenceIR→SemanticIR→IntentIR fixture with two fields plus one operation, state, and edge, then proves the
exact five-record delta at all three stages as well as rewrite/order detection.

The fresh canonical ADI run exercises this accounting on real data: EvidenceIR, SemanticIR, and IntentIR each
carry exact 11/4/13/1 protocol vectors; convergence stabilizes after two passes with Evidence fact count 6,805
and knowledge fact count 29,699. Protocol-only change is therefore no longer a convergence blind spot.
