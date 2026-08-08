---
id: swd-protocol-convergence-snapshots-are-blind
title: Convergence snapshots do not yet count the four SWD protocol collections
answers:
  - "does converge detect a protocol only SWD change"
  - "do convergence fact counts include serial frame fields"
  - "do EvidenceSnapshot SemanticSnapshot and IntentSnapshot count protocol states"
  - "what blocks the final fresh SWD canonical promotion"
  - "where is protocol aware convergence accounting tracked"
date: 2026-08-09
status: current
tags: [swd, converge, snapshot, fact-count, completeness]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.7e.i); crates/specforge/src/commands/converge.rs
reverify: "! sed -n '/struct EvidenceSnapshot/,/struct IntentSnapshot/p' crates/specforge/src/commands/converge.rs | rg 'serial_frame_fields|swd_operations|protocol_states|interface_edge_timings'"
---

The convergence command's `EvidenceSnapshot`, `SemanticSnapshot`, and `IntentSnapshot` each record a bounded
set of stage counts and sum those fields in `fact_count`. None currently includes `serial_frame_fields`,
`swd_operations`, `protocol_states`, or `interface_edge_timings`.

Consequently, a run whose only semantic change is on one of those collections can leave the aggregate
convergence fact delta unchanged. Exact stage projection and validation metrics remain correct; this is a
separate convergence-observability gap.

`SWD-SERIAL-EXTRACTION.7e.i` owns adding all four counts to every stage snapshot after the corresponding stage
fields exist, plus tests that a protocol-only change moves the fact count. The fresh canonical ADI ingest and
promotion cannot close under `.7e.ii` until that accounting is green.
