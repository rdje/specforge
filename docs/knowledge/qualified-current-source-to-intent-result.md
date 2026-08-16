---
id: qualified-current-source-to-intent-result
title: The a4a0 canonical-recovery replay is the current 40/0/0 reviewed population
answers:
  - "how many reviewed documents have current binary replay evidence"
  - "what are the current source to IntentIR precision recall and provenance counts"
  - "which frozen fabrication defects still reproduce in current SpecForge"
  - "what did the register access carrier improve in the current replay"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6c improve"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6d improve"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.b publish"
  - "what did SPEC-TO-INTENT-ALIGNMENT.7c.ii publish"
  - "is canonical recovery complete in the current reviewed population"
  - "which reviewed categories are currently supported"
  - "what task does the current trajectory controller rank next"
  - "what defect does SPEC-TO-INTENT-ALIGNMENT.6e repair next"
  - "how are external reviewed PDFs replayed without persisting host paths"
  - "where is the 12 document current replay manifest"
  - "did whole population replay mutate canonical generated artifacts"
  - "is the b977 reviewed result current for the latest production revision"
  - "is the a4a0 reviewed result current after inference antecedent recovery"
date: 2026-08-16
status: current
tags: [spec-to-intent-alignment, replay, artifact-currency, provenance, fabrication, trajectory]
evidence: crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/test_data/trajectory/controller_input.json; crates/specforge/test_data/trajectory/trajectory_report.json; docs/tasks/spec-to-intent-alignment/canonical-recovery.md (.7c.ii)
reverify: "cargo test --offline -p specforge-conformance --lib test_support::trajectory_snapshot"
---

`SPEC-TO-INTENT-ALIGNMENT.7c.ii` publishes all 12 review-locked documents from hash-equal source bytes at
production revision `a4a08cd44932d6a15573170a13bc7ede38b0af14`. Four sources are repository-relative. Eight
necessary external sources use an authorized untracked runtime map, are verified on the repository filesystem
device, copied into `.project-data/tmp`, and represented in durable evidence only by portable id, digest, byte
count, and repository-relative scratch path.

The portable manifest pins 12 source identities and all 48 SourceIR/EvidenceIR/SemanticIR/IntentIR artifacts.
The comparable result covers the unchanged 14 reviewed cells. Canonical `generated/`, the frozen reviewed
dataset, and its builder fixture remain byte-identical. The diagnostic and publication replay roots plus the
absolute-path runtime map were deleted and are absent. Exact cleanup removed 6,189 files / 2,294,248 KiB; no
shared cache was touched.

EvidenceIR, SemanticIR, and IntentIR each have 40 true positives, zero false positives, and zero false negatives:
precision and recall are both 40/40. Provenance closure is 43/43. Conservation is 120/120, fabricated canonical
facts are 0/40, unexplained drops are zero, and residual actionability is 4/24. Wire-protocol and physical-link
are supported; the other four categories remain incomplete. Source/evidence capture is 14/14 and 12/14; source
disposition is 8/14 and required-modality document accounting is 6/12.

Against the first `.7c` diagnostic replay, exactly one reviewed cell changes. APB's already recovered EvidenceIR
`PSEL|must_be_asserted|<missing>` record now survives SemanticIR and IntentIR through `.7c.i`'s exact source-local
grounding path. Every other cell is unchanged. The record remains polarity-neutral, carries exact statement
provenance, and creates no interface signal, declaration, alias, or authority for another same-subject record.

All replay, fabrication, canonical-provenance, conservation, capability, oracle, scope, and currency hard gates
pass. The controller state is `unmeasurable` because comparable history is still insufficient; it does not infer
convergence from one improved point. The first ranked gap is `SPEC-TO-INTENT-ALIGNMENT.8`, which owns typed,
source-linked, actionable required promotion-loss residuals. Planned `.6e` remains superseded by `.f.iv.a`
structural repair plus `.f.iv.b` whole-population proof. This `.7c.ii` authority is the current revision-bound
published baseline; `.7` canonical recovery is complete.
