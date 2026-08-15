---
id: behavioral-genericity-qualification-program
title: Behavioral genericity qualification uses bounded relational oracles in conformance
answers:
  - "How is SPEC-TO-INTENT-ALIGNMENT.6d.ii.f decomposed?"
  - "Who may own behavioral transforms and held-out labels?"
  - "What is the current behavioral genericity frontier?"
  - "Why is behavioral genericity not one byte equality test?"
  - "What evidence closed behavioral genericity signoff?"
date: 2026-08-15
status: current
tags: [genericity, metamorphic-testing, conformance, task-tree]
evidence: docs/tasks/spec-to-intent-alignment/behavioral-qualification.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md; doctrine/production_genericity/behavioral_qualification.json; crates/specforge-conformance/src/behavioral_genericity.rs; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; docs/decisions/0038-proof-carrying-genericity-kernel.md
reverify: cargo test --offline -p specforge-conformance behavioral_genericity --no-fail-fast && python3 -B scripts/check_behavioral_genericity_contract.py && perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/spec_to_intent_task_evidence.json --report
---

`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f` is complete. `.f.i` freezes the oracle and governed population; `.f.ii`
implements conformance-owned alpha-renaming, adversarial document identity, digest-pinned reviewed paraphrase and
harmless layout, real semantic-negative calibration, nine-class sensitivity, and typed attempt dispositions.
`.f.iii` qualifies the identity-disjoint holdout and `.f.iii.a` removes the one measured production alpha
coupling. `.f.iv` publishes the repaired complete population, and `.f.v` composes final signoff.

These transformations do not share one equality relation. Alpha-renaming preserves normalized canonical
decisions and proof topology while copied symbols rename consistently. Reviewed paraphrase can preserve meaning
while source spans and raw proof bytes change. Negative controls must change a fact or residual. Missing retained
source, an ambiguous transform, or an inadequate held-out denominator is explicitly unmeasurable rather than a
pass.

The executable contract covers 24 current chains, 23 non-vacuous normalized-text projections, seven reviewed
calibration rows, and 17 prospective holdouts. Full-PDF identity relations cover rich capture; normalized-text
relations explicitly do not. Transform recipes, expected relations, held-out membership, and comparison evidence belong exclusively to
`specforge-conformance`. The production core receives a transformed source only as ordinary current-document
input and cannot observe its oracle label or expected result. This prevents behavioral qualification from
creating a new conformance-to-core authority path.

The harness copies hash-pinned input into repository-local scratch, executes and reloads SourceIR,
EvidenceIR, SemanticIR, IntentIR, and ISF adapter pairs, compares every serialized field and proof claim, and
emits digest-pinned JSON evidence. Provider-free alpha, reviewed paraphrase/layout, and semantic-negative plus
explicit unchanged/adversarial Docling-backed PDF calibrations pass. The real negative removes exactly one
SemanticIR assertion and its cumulative SemanticIR/IntentIR/adapter proof claim; ordinary invariance rejects and
the declared exact complement passes. The governed held-out matrix closes at 35 pass / zero fail / 16 honestly
unmeasurable / zero invalid; the clean 12-source population is 39/0/1 IntentIR TP/FP/FN with 42/42 provenance and
zero fabrication. Composed with the unconditional structural doctrine and 27 adversarial controls, this closes
production-genericity signoff within the governed boundary. It does not claim perfect recall or full PDF-to-ISF
product completion. [[behavioral-reviewed-recipe-boundary]] records the review boundary and
[[behavioral-semantic-negative-sensitivity]] records the negative-control contract.
