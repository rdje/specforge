---
id: source-to-intent-first-reviewed-result
title: The first reviewed source-to-IntentIR result is incomplete in all six categories and fails upstream
answers:
  - "what is the first reviewed source-to-IntentIR evaluation result"
  - "which source-to-IntentIR categories are supported incomplete or unmeasurable"
  - "what are the exact source-to-IntentIR precision recall and stage-loss totals"
  - "where does the first reviewed source-to-IntentIR population fail"
  - "is upstream extraction or ISF expressiveness the next measured blocker"
  - "how many reviewed source regions and modalities were found"
  - "where is the persisted source-to-IntentIR result snapshot"
date: 2026-08-11
status: current
tags: [spec-to-intent-alignment, evaluation, stage-loss, evidence-ir, intent-ir, blocker-diagnosis]
evidence: crates/specforge/test_data/source_to_intent_vertical/result_snapshot.json; crates/specforge/src/ir/source_to_intent_eval.rs; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.4c)
reverify: "cargo test -p specforge --lib ir::source_to_intent_eval::tests::reviewed_result_snapshot_is_current_and_names_the_upstream_loss_boundary && cargo run --quiet -p specforge --example source_to_intent_eval -- crates/specforge/test_data/source_to_intent_vertical/reviewed_dataset.json | cmp - crates/specforge/test_data/source_to_intent_vertical/result_snapshot.json"
---

`SPEC-TO-INTENT-ALIGNMENT.4c` evaluates the review-complete [[source-to-intent-reviewed-population]] without
changing selection, gold, extraction, or source/stage artifacts. All six categories are `incomplete`; none is
`supported` or `unmeasurable`. All 14 bounded source-region queries and all 14 required-modality captures are
exact, so the result is not a missing-input or missing-oracle diagnosis. No cell has a fully accounted canonical,
residual, or non-applicable disposition, which leaves required-modality accounting at 0/12 documents.

Across the canonical queries, IntentIR has 7 true positives, 41 false positives, and 33 false negatives:
precision is 7/48 (14.5833%) and recall is 7/40 (17.5%). Canonical provenance closure is 3/48 (6.25%). Stage
conservation or residualization is 21/54 (38.8889%): SourceIR → EvidenceIR conserves 7/40 reviewed canonical
facts and accounts for all 33 unexplained drops, while the seven facts present in EvidenceIR remain 7/7 through
both later boundaries. Residual actionability is 0/24; eight cells lack the required actionable residual at
both SemanticIR and IntentIR. The report records 41 fabricated canonical facts.

The first failing stage is SourceIR → EvidenceIR for 10/14 cells and EvidenceIR → SemanticIR for 4/14; no cell
first fails at SemanticIR → IntentIR. The repeated canonical counts across EvidenceIR, SemanticIR, and IntentIR
show that later promotion preserves the small typed subset, while missing/incorrect EvidenceIR typing and absent
residual dispositions dominate the loss. Therefore the next measured constraint is upstream source-to-evidence
fact formation and residualization, not a demonstrated ISF/FSMGen expressiveness gap. This evaluation ends at
IntentIR and presents no source-grounded value that survives correctly to IntentIR but fails only in the adapter.

The byte-pinned report is
`crates/specforge/test_data/source_to_intent_vertical/result_snapshot.json` (104,669 bytes; SHA-256
`6b72f1fc2a5616542965f264bd87f23534b5274d19332c8a68bc6b88848547eb`). The generic example runner reproduces
it from the frozen dataset, and the focused Rust test compares the complete pretty-JSON byte stream.
