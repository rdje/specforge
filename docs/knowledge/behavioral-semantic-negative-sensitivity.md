---
id: behavioral-semantic-negative-sensitivity
title: Semantic negatives require a detected delta and an exact unaffected complement
answers:
  - "how does the behavioral genericity gate prove negative-control sensitivity"
  - "what exact semantic delta does the at least timing variant produce"
  - "why must the invariant comparator reject a semantic negative first"
  - "which nine semantic negative fault classes are tested"
  - "how are missing stale vacuous ambiguous or partial behavioral attempts classified"
  - "can a declared semantic delta hide another undeclared change"
date: 2026-08-14
status: current
tags: [genericity, metamorphic-testing, negative-control, proof-ledger, conformance]
evidence: doctrine/production_genericity/semantic_negative_matrix.json; doctrine/production_genericity/reviewed_recipes/um11732_semantic_negative_timing.json; doctrine/production_genericity/reviewed_recipe_manifest.json; crates/specforge-conformance/src/behavioral_genericity.rs; scripts/check_behavioral_genericity_contract.py; docs/research/behavioral-genericity-qualification-design.md
reverify: "python3 -B scripts/check_behavioral_genericity_contract.py --self-test && cargo test --offline -p specforge-conformance behavioral_genericity::tests -- --nocapture"
---

A semantic-negative pass is not permission to ignore an expected difference. The conformance harness first runs
the ordinary invariant comparator and requires it to fail. It then verifies every digest-pinned required fact and
dependent proof delta, removes only those exact declarations from cloned artifacts, and demands equality of all
remaining serialized leaves, collection structure, proof counts, and proof topology. A missing declared delta,
an invariant pass, or any additional undeclared delta fails the run.

The real UM11732 calibration rewrites “must not be less than” as “must be at least.” Seven carried source-text
conclusions remain accounted for, but the transformed SemanticIR has zero assertions instead of one. The exact
removed record is supported by `statement_0090`; its `semantic.statement_lift.assertions.v1` proof claim is also
absent from the cumulative SemanticIR, IntentIR, and ISF-adapter ledgers. Ordinary invariance rejects that pair;
after removing exactly the one declared record and those three proof occurrences, all five stages compare equal.

The separate machine matrix mutates complete synthetic five-stage artifacts with one fault at a time: omission,
contradiction, relation reversal, value change, timing change, undeclared symbol, misleading familiar name,
proof-rule corruption, and disabled stage. Every mutation must be observed, rejected with its declared state and
failure class, and exactly reversible to prove the untouched complement did not drift.

Attempt disposition is closed too. Unavailable source authority/provider and a vacuous baseline are
`unmeasurable`; stale contract/population, ambiguous/nonbijective transforms, and partial/escaped runs are
`invalid`. Neither state can be summarized as a pass, and none of these oracle labels or expected deltas enters
the production core.
