---
id: qualified-current-source-to-intent-result
title: The complete reviewed population now has a qualified current-binary result
answers:
  - "how many reviewed documents have current binary replay evidence"
  - "what are the current source to IntentIR precision recall and provenance counts"
  - "which frozen fabrication defects still reproduce in current SpecForge"
  - "what did the register access carrier improve in the current replay"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6c improve"
  - "what defect does SPEC-TO-INTENT-ALIGNMENT.6d repair next"
  - "how are external reviewed PDFs replayed without persisting host paths"
  - "where is the 12 document current replay manifest"
  - "did whole population replay mutate canonical generated artifacts"
date: 2026-08-12
status: current
tags: [spec-to-intent-alignment, replay, artifact-currency, provenance, fabrication, trajectory]
evidence: crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/test_data/trajectory/controller_input.json; crates/specforge/test_data/trajectory/trajectory_report.json; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6c.ii)
reverify: "cargo test -p specforge --lib ir::trajectory_snapshot && cargo run --quiet -p specforge --example trajectory_snapshot -- --check"
---

`SPEC-TO-INTENT-ALIGNMENT.6c.ii` replays all 12 review-locked documents from hash-equal source bytes at
production revision `74a658b3`. Four sources are repository-relative. Eight necessary external sources use
an untracked runtime map, verified on the repository filesystem device, copied into `.project-data/tmp`, and
represented in durable evidence only by portable id, digest, byte count, and repository-relative scratch path.

The portable manifest pins 12 source identities and all 48 SourceIR/EvidenceIR/SemanticIR/IntentIR artifacts.
The comparable result covers the unchanged 14 reviewed cells. Canonical `generated/`, the frozen reviewed
dataset, and the `.4c` result remain byte-identical. After evidence promotion, the 3,913-file / 1,090,908-KiB
population root and the absolute-path runtime map were deleted exactly and are absent.

Current IntentIR has 24 true positives, five false positives, and 16 false negatives: precision is 24/29 and
recall is 24/40. Provenance closure is 29/29. Conservation is 72/88, residual actionability remains 0/24, and
all six categories remain incomplete. Source/evidence capture stays 14/14 and 13/14; source disposition is 4/14
and required-modality document accounting is 2/12.

Against `.6b.ii.b`, the `[[timing-caption-unit-and-table-provenance]]` carrier makes all five I2S receiver-timing
records exact and provenanced. Four correct OpenCAPI digital-skew records and three incorrect analog channel-loss
records gain table provenance without changing their keys; no other reviewed cell changes. All 19 prior true
positives survive. The controller meets replay currency and provenance closure, remains `diverging` on five
fabricated records, and recommends `SPEC-TO-INTENT-ALIGNMENT.6d`. Its first bounded family is the three OpenCAPI
analog channel-loss records; the other two fabrications are one AMD IOMMU and one GIC-400 register key.
