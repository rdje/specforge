---
id: deterministic-constraint-recall-is-bounded-by-classification
title: Deterministic constraint recall is 15.8% and the bound is statement CLASSIFICATION, not grammar — within what it may read the grammar converts 69.8%
answers:
  - "what is SpecForge's deterministic constraint recall (15.8% on the measured stratum — 379 statements state an obligation about a declared signal and the production producer emits a record from 60)"
  - "why does the deterministic constraint producer emit so few records (it never sees most obligations: extract_signal_constraints opens its loop with if !matches!(statement.class, StatementClass::SignalValueConstraint) { continue; }, and only 86 of the 379 obligation statements carry that class while 195 carry NormativeStatement)"
  - "is the constraint extraction gap a grammar problem or a classification problem (classification — within the statements it is allowed to read the grammar converts 60 of 86, 69.8%, against 60 of 379, 15.8%, across all obligations; the 4.4x difference is the class filter)"
  - "which statement classes do the obligations about declared signals carry (86 SignalValueConstraint, 195 NormativeStatement, 65 ConditionalRule, 24 TimingConstraint, 5 DerivedRule, 3 ExplicitAbstraction, 1 SourceFact)"
  - "is a constraint the signal_constraints surface drops actually lost (not necessarily — of the 319 dropped, 63 are held by the conditional-rule surface and 9 by actor-signal relations, leaving 247 held by no persisted surface at all; reporting 319 as losses overstates it)"
  - "what shapes make up the obligations no persisted surface holds (first-match partition of the 247: 51 table row, 41 ordering such as must wait for X before asserting Y, 47 actor-subject such as The Manager must not issue, and 108 other — the remainder is where plain value obligations like AWBURST must be INCR sit)"
  - "can the deterministic constraint grammar read a plain value obligation (yes, every shape tried — level, bare enum, numeric, stability, remain-asserted-until — once the statement's class admits it; a NEGATED enum value is the one measured residual, minting nothing while its positive twin does)"
  - "how do I re-run the deterministic constraint recall measurement (cargo test -p specforge-core --lib constraint_recall_gap -- --ignored --nocapture)"
date: 2026-09-18
status: current
tags: [extraction-gap-fix, extraction-quality-gauge, recall, classification, census, root-cause, method]
evidence: docs/tasks/EXTRACTION-GAP-FIX.md (.5, .5a); crates/specforge/src/ir/evidence.rs (extract_signal_constraints, extract_normative_signal_constraints, replay_persisted_signal_constraints, constraint_recall_gap_local_measurement, extraction_gap_fix_5_classification); docs/tasks/extraction-quality-gauge/llm-path-sealed.md (.3j.3)
reverify: "cargo test -p specforge-core --lib constraint_recall_gap -- --ignored --nocapture — expect 5 documents / 379 obligation statements / 60 produced / 319 gap / 15.8% recall; 63 conditional / 9 relation / 247 unrepresented; partition 51 / 41 / 47 / 108; classes 86 SignalValueConstraint / 195 NormativeStatement / 65 ConditionalRule / 24 TimingConstraint / 5 DerivedRule / 3 ExplicitAbstraction / 1 SourceFact"
---

`[[llm-primary-recall-ceiling]]` established that the model-primary path can only refine what the
deterministic producer already emitted, which makes deterministic recall the binding constraint on
extraction quality. This is what bounds it.

## The measurement

Run through the **production** producer — the statement path and the row path composed exactly as
`replay_persisted_signal_constraints` composes them — so "not emitted" means today's code mints nothing.

| | |
| --- | ---: |
| statements stating an obligation about a declared signal | 379 |
| …the producer emits a record from | 60 |
| **deterministic recall** | **15.8 %** |

The 60 is the same 60 `.3j.3` found as the model's visible universe, reached from the opposite direction.

**The 319 remainder is not 319 losses.** 63 are held by the conditional-rule surface and 9 by
actor-signal relations; **247** are held by no persisted surface at all. First-match partition of those:
51 table row, 41 ordering, 47 actor-subject, 108 other.

## The root cause is the class filter

`extract_signal_constraints` opens its loop with:

```rust
if !matches!(statement.class, StatementClass::SignalValueConstraint) {
    continue;
}
```

So an obligation the upstream classifier labelled anything else is never offered to the constraint
grammar. Of the 379: **86** carry that class, **195** carry `NormativeStatement`, 65 `ConditionalRule`,
24 `TimingConstraint`, 5 `DerivedRule`, 3 `ExplicitAbstraction`, 1 `SourceFact`.

**Within what it is allowed to read the grammar converts 60 of 86 — 69.8 %. Across all obligations,
60 of 379 — 15.8 %. The 4.4× difference is classification, not parsing.**

## The probe that said otherwise was wrong, and its own controls caught it

A first fixture appeared to show the grammar could not read `XQBURST must be LOW.`. It was invalid: two
sentence shapes the real corpus demonstrably *does* extract from came out empty in it too, because the
fixture built its statement with the wrong class. With the class corrected the grammar reads every shape
tried — level, bare enum, numeric, stability, remain-asserted-until.

**The lesson generalises: validate a negative result against a case known to be positive before believing
it.** A silent zero from an unvalidated harness reads exactly like a defect in the thing under test.

## Why widening the grammar would be the wrong fix

41 of the unrepresented 247 are **ordering** obligations — *"must wait for AWVALID … before asserting
BVALID"* — and the constraint vocabulary `(subject, kind, value, condition)` has no slot for them. Routing
`NormativeStatement` into the constraint path wholesale would mint exactly the fabrications `.3a`, `.3b`,
`.3d` and `.3k.1` exist to refuse. The recoverable recall is in the **108 remainder**, which holds plain
value obligations on declared signals — *"AWBURST must be INCR."*, *"AWTAGOP must not be Match."* — that
the grammar reads once the class admits them. `EXTRACTION-GAP-FIX.5a` adjudicates per class.
