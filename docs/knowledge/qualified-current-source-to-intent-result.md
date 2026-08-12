---
id: qualified-current-source-to-intent-result
title: The complete reviewed population now has a qualified current-binary result
answers:
  - "how many reviewed documents have current binary replay evidence"
  - "what are the current source to IntentIR precision recall and provenance counts"
  - "which frozen fabrication defects still reproduce in current SpecForge"
  - "what did the register access carrier improve in the current replay"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6c improve"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6d improve"
  - "what defect does SPEC-TO-INTENT-ALIGNMENT.6e repair next"
  - "how are external reviewed PDFs replayed without persisting host paths"
  - "where is the 12 document current replay manifest"
  - "did whole population replay mutate canonical generated artifacts"
date: 2026-08-12
status: current
tags: [spec-to-intent-alignment, replay, artifact-currency, provenance, fabrication, trajectory]
evidence: crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/test_data/trajectory/controller_input.json; crates/specforge/test_data/trajectory/trajectory_report.json; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6d.ii.a)
reverify: "cargo test -p specforge --lib test_support::trajectory_snapshot"
---

`SPEC-TO-INTENT-ALIGNMENT.6d.ii.a` publishes the replay of all 12 review-locked documents from hash-equal source
bytes at production revision `b977a51f`. Four sources are repository-relative. Eight necessary external sources use
an untracked runtime map, verified on the repository filesystem device, copied into `.project-data/tmp`, and
represented in durable evidence only by portable id, digest, byte count, and repository-relative scratch path.

The portable manifest pins 12 source identities and all 48 SourceIR/EvidenceIR/SemanticIR/IntentIR artifacts.
The comparable result covers the unchanged 14 reviewed cells. Canonical `generated/`, the frozen reviewed
dataset, and the `.4c` result remain byte-identical. After evidence promotion, the exact 3,913-file /
1,090,948-KiB population root and the absolute-path runtime map were deleted and are absent.

Current IntentIR has 24 true positives, two false positives, and 16 false negatives: precision is 24/26 and
recall is 24/40. Provenance closure is 29/29. Conservation is 72/88 and residual actionability is 4/24.
Physical-link is supported and the other five categories remain incomplete.
Source/evidence capture stays 14/14 and 13/14; source disposition is 6/14 and required-modality document
accounting is 4/12.

Against `.6c.ii`, the closed decibel-domain disposition changes exactly two analog channel-loss cells. Three
`dB`/`dB_RMS` keys leave canonical digital timing and become exact, source-linked, actionable residuals at both
SemanticIR and IntentIR. The other 12 cells, all 24 prior true positives, and all 29 provenanced canonical records
remain exact. The controller meets replay currency and provenance closure, remains `diverging` on two unrelated
fabricated register records, and recommends `SPEC-TO-INTENT-ALIGNMENT.6e` after the owner-mandated production-
genericity remediation parent closes.
