---
id: inference-antecedent-local-grounding-stops-at-semantic-layer-d
title: The recovered APB antecedent reaches EvidenceIR and then stops at SemanticIR record grounding
answers:
  - "why does recovered APB PSEL disappear between EvidenceIR and SemanticIR"
  - "where did the inference antecedent loss move after SPEC-TO-INTENT-ALIGNMENT.7b"
  - "why must source-local PSEL not become a global interface signal"
  - "what does SPEC-TO-INTENT-ALIGNMENT.7c.i repair"
  - "does the Layer D grounding filter need to be weakened"
  - "why is the first SPEC-TO-INTENT-ALIGNMENT.7c replay not publishable"
date: 2026-08-16
status: current
tags: [spec-to-intent-alignment, evidence-ir, semantic-ir, grounding, apb, canonical-recall]
evidence: docs/tasks/spec-to-intent-alignment/canonical-recovery.md; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; docs/knowledge/semantic-grounding-filter-is-catalog-independent.md
reverify: "python3 -B scripts/replay_source_to_intent_population.py --help && cargo test --offline -p specforge-core --lib signal_constraints_for_undeclared_signals_are_filtered_by_layer_d"
---

The first complete replay after `SPEC-TO-INTENT-ALIGNMENT.7b` processes every reviewed source and isolated stage.
APB EvidenceIR now contains the exact `PSEL|must_be_asserted|<missing>` fact with its supporting statement and
source text. EvidenceIR closes at 40/0/0 TP/FP/FN with zero fabricated canonical facts, while SemanticIR and
IntentIR remain 39/0/1. Conservation advances to 118/119 instead of closing.

The remaining loss is explicit. `SemanticIr::build` partitions signal constraints through Layer D, which admits
subjects found in the document-wide declared interface catalog and retains rejected records in
`semantic_ungrounded_records_not_promoted`. The APB span establishes `PSEL` only through the bounded same-clause
`signal, ID,` appositive. That clause-local identity deliberately does not create an interface signal, mutate the
catalog, or alias `PSEL` to the separately declared `PSELX`, so the recovered constraint enters the residual.

Layer D must not be weakened. Its catalog-independent uniform filter correctly demotes ordinary undeclared
records, including in empty-catalog documents. Adding `PSEL` globally would instead widen one sentence's local
evidence into document authority and could promote unrelated records sharing the subject spelling.

`SPEC-TO-INTENT-ALIGNMENT.7c.i` therefore owns an exact record-scoped carry rule. An otherwise undeclared
constraint may cross the SemanticIR gate only if the same source text, supporting statement identity, subject,
and closed inference-appositive derivation revalidate through the EvidenceIR grammar that created it. The rule
must not create interface/catalog/alias state or authorize another constraint. `.7c.ii` owns the clean complete
replay and publication retry after that repair.
