---
id: inference-antecedent-state-loss
title: The current APB canonical miss is an independently explicit inference-antecedent state loss
answers:
  - "why is APB PSEL HIGH missing from the current reviewed population"
  - "where is the sole source to EvidenceIR canonical loss"
  - "why must PSEL not inherit VALID from the which means consequence"
  - "what does SPEC-TO-INTENT-ALIGNMENT.7 repair"
  - "what is the first task in canonical recovery"
  - "how can one compound sentence contain two independent signal facts"
date: 2026-08-16
status: current
tags: [spec-to-intent-alignment, evidence-ir, signal-constraint, apb, canonical-recall]
evidence: crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/src/ir/evidence.rs; docs/tasks/spec-to-intent-alignment/canonical-recovery.md
reverify: "cargo test --offline -p specforge-core --lib inference_antecedent_signal_is_not_the_obligation_subject && rg -n -F 'PSEL|must_be_value|HIGH' crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json && rg -n 'consequent_after_inference_marker|inference_antecedent_signal_is_not_the_obligation_subject' crates/specforge/src/ir/evidence.rs"
---

The current reviewed result has one canonical false negative and one unexplained SourceIR-to-EvidenceIR drop:
APB `PSEL|must_be_value|HIGH`. The complete source region is already captured. Its compound sentence says that
`PSEL` is asserted and, after `which means`, that `PADDR`, `PWRITE`, and `PWDATA` must be valid.

The consequence path in `extract_signal_constraints` intentionally calls `consequent_after_inference_marker`.
Its existing precision control proves that `PSEL` is not a subject of the later validity obligation. That behavior
is correct: emitting `PSEL|must_be_value|VALID` would borrow the suffix's value and fabricate the wrong fact.

The causal loss is the missing companion interpretation. No independent producer preserves the prefix's own
explicit copular state `PSEL is asserted` after the consequence is isolated. `SPEC-TO-INTENT-ALIGNMENT.7` is
therefore not permission to widen consequence subjects. `.7a` first freezes a declaration-grounded antecedent-
state grammar, polarity and ambiguity controls, affected-chain scope, and population replay contract. `.7b`
owns production repair and currency; `.7c` owns comparable publication.
