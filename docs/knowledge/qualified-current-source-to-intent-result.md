---
id: qualified-current-source-to-intent-result
title: The complete reviewed population now has a qualified current-binary result
answers:
  - "how many reviewed documents have current binary replay evidence"
  - "what are the current source to IntentIR precision recall and provenance counts"
  - "which frozen fabrication defects still reproduce in current SpecForge"
  - "what defect does SPEC-TO-INTENT-ALIGNMENT.6b.ii repair next"
  - "how are external reviewed PDFs replayed without persisting host paths"
  - "where is the 12 document current replay manifest"
  - "did whole population replay mutate canonical generated artifacts"
date: 2026-08-12
status: current
tags: [spec-to-intent-alignment, replay, artifact-currency, provenance, fabrication, trajectory]
evidence: crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/test_data/trajectory/controller_input.json; crates/specforge/test_data/trajectory/trajectory_report.json; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6b.i)
reverify: "cargo test -p specforge --lib ir::trajectory_snapshot && cargo run --quiet -p specforge --example trajectory_snapshot -- --check"
---

`SPEC-TO-INTENT-ALIGNMENT.6b.i` replays all 12 review-locked documents from hash-equal source bytes at production
revision `6b972d76`. Four sources are repository-relative. Eight necessary external sources are supplied through
an untracked runtime map, verified on the repository filesystem device, copied into `.project-data/tmp`, and
represented in durable evidence only by portable id, digest, byte count, and repository-relative scratch path.

The portable manifest pins 12 source identities and all 48 SourceIR/EvidenceIR/SemanticIR/IntentIR artifacts.
The comparable result covers the unchanged 14 reviewed cells. Canonical `generated/`, the frozen reviewed
dataset, and the `.4c` result remain byte-identical. After evidence promotion, the 4,313-file / 1,194,976-KiB
population root and the absolute-path runtime map were deleted exactly and are absent.

Current IntentIR retains 7 true positives and 33 false negatives. False positives are 22, so precision is 7/29;
recall remains 7/40. Provenance closure is 3/29, leaving 26 failures. Conservation remains 21/54, residual
actionability 0/24, and all six categories remain incomplete. Source-region capture is 14/14; evidence capture is
13/14 because the now-honest AIA table of contents has neither a canonical promotion nor a non-contract residual.

Against the frozen `[[source-to-intent-first-reviewed-result]]`, false facts improve 41→22 and provenance failures
45→26. The delta is exactly the 19-record AIA TOC family already retired by
`[[retrospective-baseline-current-replay-boundary]]`; all other reviewed defect counts reproduce. The largest
single family is Arm Debug's 12 register names without their reviewed access modes: 12 wrong canonical keys,
12 misses, and 12 provenance failures. The controller therefore meets replay currency at 12/12, remains
`diverging` on current honesty hard gates, and recommends `SPEC-TO-INTENT-ALIGNMENT.6b.ii`.
