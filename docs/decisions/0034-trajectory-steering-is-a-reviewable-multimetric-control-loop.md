---
id: trajectory-steering-is-a-reviewable-multimetric-control-loop
title: Trajectory steering is a reviewable multi-metric control loop, not one blended score
date: 2026-08-11
status: accepted
scope: objective, measurement, evaluation, convergence, automation, task-selection
evidence: docs/research/specforge-trajectory-control.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md; docs/research/intent-capture-completeness.md; docs/research/recall-estimation-and-report.md; crates/specforge/src/commands/converge.rs
answers:
  - "how will SpecForge automatically detect convergence or divergence"
  - "should SpecForge use one weighted progress score"
  - "what is the SpecForge trajectory controller"
  - "how can SpecForge automatically choose the next task without gaming metrics"
  - "what distinguishes converging stalled divergent mixed and unmeasurable"
  - "can automatic steering mutate canonical IntentIR without review"
  - "which metrics govern specification-to-executable-intent progress"
---

# ADR 0034: Trajectory steering is a reviewable multi-metric control loop, not one blended score

## Context

SpecForge already measures many local properties: exact pipeline stabilization, artifact currency, held-gold
precision/recall/F1, capture–recapture recall estimates, source-region accounting, closure invariants,
category-aware completeness, strict ISF validity, and task/doctrine completion. These instruments answer
different questions and carry different blind spots. They are not currently composed into a durable trajectory
history that can say whether a commit improved the specification-to-executable-intent objective.

A single weighted score would hide trade-offs and invite metric gaming: a large currency or test-count gain
could mask a semantic-recall loss, and strict-valid output could mask missing source intent. Conversely, one
hard regression must not be averaged away by several easy improvements.

## Decision

1. Define a machine-readable objective contract by document category, evidence modality, typed IR surface,
   provenance/residual rule, and required evaluation oracle.
2. Persist a versioned trajectory snapshot for each reviewed corpus/baseline revision. Keep the objective
   vector visible; do not collapse it into an authoritative scalar score.
3. Classify trajectory as:
   - `converging`: no hard invariant regresses and at least one material objective deficit improves with
     adequate evidence;
   - `diverging`: a hard honesty/currentness invariant fails, or a statistically supported semantic metric
     worsens without an accepted trade-off;
   - `stalled`: important measured deficits remain while a configured review window shows no material gain;
   - `mixed`: gains and losses are both material and neither dominates;
   - `unmeasurable`: the required oracle, provider observation, gold, or modality coverage is absent.
4. Compare deterministic artifacts with exact paired deltas. Use uncertainty-aware paired statistics for
   sampled gold and online drift detectors only after enough comparable observations exist; never manufacture a
   trend from a tiny history.
5. Protect the instruments with negative fixtures, metamorphic relations, and evaluator mutation tests. A
   steering metric that cannot detect a seeded omission or fabrication is not authoritative.
6. Rank candidate work lexicographically: hard invariant repair; source evidence loss; high-impact semantic
   recall gap; missing oracle/high value of information; then breadth and efficiency. Expose all ranking inputs
   and uncertainty rather than hiding them in weights.
7. Automation may report, gate, diagnose, generate a replay plan, and propose the next task-tree leaf. It may not
   silently approve or mutate canonical semantic truth; existing review and task-tree doctrine remains the
   authority boundary.

## Consequences

- “All tests pass” and “the pipeline stabilized” are no longer treated as top-level convergence claims.
- A stalled or unmeasurable category becomes actionable rather than being counted as success or failure.
- Automatic PNT selection can be grounded in objective deficits and expected information gain while remaining
  reviewable and reversible.
- The controller reuses existing instruments; implementation work is primarily composition, history, oracle
  expansion, and drift logic rather than a replacement validation system.

## Links

- Design: [`specforge-trajectory-control.md`](../research/specforge-trajectory-control.md)
- Task tree: [`SPEC-TO-INTENT-ALIGNMENT`](../tasks/SPEC-TO-INTENT-ALIGNMENT.md)
- Prior completeness research: [`intent-capture-completeness.md`](../research/intent-capture-completeness.md)
