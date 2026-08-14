---
id: behavioral-genericity-qualification-program
title: Behavioral genericity qualification uses bounded relational oracles in conformance
answers:
  - "How is SPEC-TO-INTENT-ALIGNMENT.6d.ii.f decomposed?"
  - "Who may own behavioral transforms and held-out labels?"
  - "What is the current behavioral genericity frontier?"
  - "Why is behavioral genericity not one byte equality test?"
date: 2026-08-14
status: current
tags: [genericity, metamorphic-testing, conformance, task-tree]
evidence: docs/tasks/spec-to-intent-alignment/behavioral-qualification.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md; doctrine/production_genericity/behavioral_qualification.json; crates/specforge-conformance/src/behavioral_genericity.rs; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; docs/decisions/0038-proof-carrying-genericity-kernel.md
reverify: cargo test --offline -p specforge-conformance behavioral_genericity --no-fail-fast && python3 -B scripts/check_behavioral_genericity_contract.py && perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/spec_to_intent_task_evidence.json --report
---

`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f` is active. `.f.i` has frozen the oracle and governed population, and
`.f.ii.a` implements the conformance-owned alpha-renaming plus adversarial document/symbol identity harness.
`.f.ii.b` is the precise frontier for reviewed paraphrase and harmless-layout relations. Later
children separately qualify reviewed structure-preserving paraphrase/layout changes, semantic negative
controls, identity-disjoint held-out strata, complete-population replay, and final signoff.

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

The implemented harness copies hash-pinned input into repository-local scratch, executes and reloads SourceIR,
EvidenceIR, SemanticIR, IntentIR, and ISF adapter pairs, compares every serialized field and proof claim, and
emits digest-pinned JSON evidence. Provider-free alpha plus explicit unchanged/adversarial Docling-backed PDF
calibrations pass; these focused calibrations are not the governed held-out or complete-population result.
