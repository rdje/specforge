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
evidence: docs/tasks/spec-to-intent-alignment/behavioral-qualification.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; docs/decisions/0038-proof-carrying-genericity-kernel.md
reverify: perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/spec_to_intent_task_evidence.json --report
---

`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f` is active, and `.f.i` is its precise frontier. The bounded program freezes
the oracle and governed population first; then it separately qualifies alpha-renaming plus adversarial document
and symbol identity, reviewed structure-preserving paraphrase/layout changes, semantic negative controls,
identity-disjoint held-out strata, complete-population replay, and final signoff.

These transformations do not share one equality relation. Alpha-renaming preserves normalized canonical
decisions and proof topology while copied symbols rename consistently. Reviewed paraphrase can preserve meaning
while source spans and raw proof bytes change. Negative controls must change a fact or residual. Missing retained
source, an ambiguous transform, or an inadequate held-out denominator is explicitly unmeasurable rather than a
pass.

Transform recipes, expected relations, held-out membership, and comparison evidence belong exclusively to
`specforge-conformance`. The production core receives a transformed source only as ordinary current-document
input and cannot observe its oracle label or expected result. This prevents behavioral qualification from
creating a new conformance-to-core authority path.
