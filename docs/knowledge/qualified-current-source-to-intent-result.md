---
id: qualified-current-source-to-intent-result
title: The complete reviewed population now has a qualified current-binary result
answers:
  - "how many reviewed documents have current binary replay evidence"
  - "what are the current source to IntentIR precision recall and provenance counts"
  - "which frozen fabrication defects still reproduce in current SpecForge"
  - "what did the register access carrier improve in the current replay"
  - "what defect does SPEC-TO-INTENT-ALIGNMENT.6c repair next"
  - "how are external reviewed PDFs replayed without persisting host paths"
  - "where is the 12 document current replay manifest"
  - "did whole population replay mutate canonical generated artifacts"
date: 2026-08-12
status: current
tags: [spec-to-intent-alignment, replay, artifact-currency, provenance, fabrication, trajectory]
evidence: crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/test_data/trajectory/controller_input.json; crates/specforge/test_data/trajectory/trajectory_report.json; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6b.ii.b)
reverify: "cargo test -p specforge --lib ir::trajectory_snapshot && cargo run --quiet -p specforge --example trajectory_snapshot -- --check"
---

`SPEC-TO-INTENT-ALIGNMENT.6b.ii.b` replays all 12 review-locked documents from hash-equal source bytes at
production revision `bb152dfb`. Four sources are repository-relative. Eight necessary external sources use
an untracked runtime map, verified on the repository filesystem device, copied into `.project-data/tmp`, and
represented in durable evidence only by portable id, digest, byte count, and repository-relative scratch path.

The portable manifest pins 12 source identities and all 48 SourceIR/EvidenceIR/SemanticIR/IntentIR artifacts.
The comparable result covers the unchanged 14 reviewed cells. Canonical `generated/`, the frozen reviewed
dataset, and the `.4c` result remain byte-identical. After evidence promotion, the 3,913-file / 1,090,884-KiB
population root and the absolute-path runtime map were deleted exactly and are absent.

Current IntentIR has 19 true positives, ten false positives, and 21 false negatives: precision is 19/29 and
recall is 19/40. Provenance closure is 17/29, leaving twelve failures. Conservation is 57/78, residual
actionability remains 0/24, and all six categories remain incomplete. Source/evidence capture stays 14/14 and
13/14; source disposition improves 0/14→1/14 and required-modality document accounting 0/12→1/12.

Against `.6b.i`, the `[[register-record-access-and-table-provenance]]` carrier makes all twelve Arm Debug
register/access records exact and provenanced. AMD IOMMU and GIC-400 each close table provenance while retaining
their existing false key; no other cell changes. The controller meets replay currency at 12/12, remains
`diverging` on ten fabricated and twelve unprovenanced records, and recommends
`SPEC-TO-INTENT-ALIGNMENT.6c`. That leaf owns I2S receiver timing: five constraints retain correct values but
lose their reviewed `ns` unit, producing five false positives, five false negatives, and five provenance failures.
