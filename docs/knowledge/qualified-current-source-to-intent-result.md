---
id: qualified-current-source-to-intent-result
title: The e125 repaired reviewed population replay is the revision-bound published baseline
answers:
  - "how many reviewed documents have current binary replay evidence"
  - "what are the current source to IntentIR precision recall and provenance counts"
  - "which frozen fabrication defects still reproduce in current SpecForge"
  - "what did the register access carrier improve in the current replay"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6c improve"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6d improve"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.b publish"
  - "what defect does SPEC-TO-INTENT-ALIGNMENT.6e repair next"
  - "how are external reviewed PDFs replayed without persisting host paths"
  - "where is the 12 document current replay manifest"
  - "did whole population replay mutate canonical generated artifacts"
  - "is the b977 reviewed result current for the latest production revision"
  - "is the e125 reviewed result current after inference antecedent recovery"
date: 2026-08-16
status: current
tags: [spec-to-intent-alignment, replay, artifact-currency, provenance, fabrication, trajectory]
evidence: crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/test_data/trajectory/controller_input.json; crates/specforge/test_data/trajectory/trajectory_report.json; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6d.ii.f.iv.b)
reverify: "cargo test -p specforge-conformance --lib test_support::trajectory_snapshot"
---

`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.b` publishes all 12 review-locked documents from hash-equal source bytes at
production revision `e125aac7`. Four sources are repository-relative. Eight necessary external sources use
an untracked runtime map, verified on the repository filesystem device, copied into `.project-data/tmp`, and
represented in durable evidence only by portable id, digest, byte count, and repository-relative scratch path.

The portable manifest pins 12 source identities and all 48 SourceIR/EvidenceIR/SemanticIR/IntentIR artifacts.
The comparable result covers the unchanged 14 reviewed cells. Canonical `generated/`, the frozen reviewed
dataset, and the `.4c` result remain byte-identical. The successful 3,094-file / 1,147,260-KiB authority root,
two interrupted attempt roots, and the absolute-path runtime map were deleted and are absent. Across the three
roots, exact cleanup removed 5,616 files / 2,038,580 KiB.

Current IntentIR has 39 true positives, zero false positives, and one false negative: precision is 39/39 and
recall is 39/40. Provenance closure is 42/42. Conservation is 117/118 and residual actionability is 4/24.
Physical-link is supported and the other five categories remain incomplete.
Source/evidence capture is 14/14 and 12/14; source disposition is 7/14 and required-modality document accounting
is 5/12.

Against the prior `b977a51f` authority, only two register cells change. GIC-400 moves from 0/1/15 to 15/0/0
TP/FP/FN and closes disposition. AMD moves from one packed-layout fabrication to an honest empty canonical cell;
its spurious table capture disappears, explaining the 13/14→12/14 capture trade. Every other cell is identical.
APB's missing polarity-neutral `PSEL|must_be_asserted|<missing>` fact is the sole canonical miss and unexplained
drop. `.7a` corrected the earlier unsupported `asserted`→`HIGH` oracle refinement without changing the 39/0/1
population score.

All replay, fabrication, and canonical-provenance hard gates pass. The controller state is `unmeasurable` because
comparable history is still absent; it does not infer convergence from one improved point. The first ranked gap
is `SPEC-TO-INTENT-ALIGNMENT.7`, the sole APB source-to-EvidenceIR loss. Planned `.6e` is superseded by `.f.iv.a`
structural repair plus `.f.iv.b` whole-population proof. This authority is now the revision-bound published
baseline rather than current evidence for the latest production revision. `.7b` added the closed inference-
antecedent producer and rebuilt all 24 retained artifact chains with zero stale stages and zero public delta.
That fixed-input reconciliation proves currency of retained chains, not the reviewed 12-source population.
`.7c` must perform all 48 fresh stages before the published 39/0/1 result or controller state may change.
