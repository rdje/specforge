# Trajectory And Automatic Steering

SpecForge is meant to move specifications toward executable intent. A green build, a stable convergence loop,
or a strict-valid `.isf` file does not by itself prove movement toward that objective: all three can be true
while important source intent was never captured.

The project therefore treats trajectory as a source-understanding control problem. The accepted design is
tracked by `SPEC-TO-INTENT-ALIGNMENT`; the runtime controller is **not shipped yet**.

## What “on track” means

Trajectory is evaluated as a vector rather than one progress score:

- source coverage across prose, tables, document structure, figures, captions, and diagrams;
- semantic precision and recall on held-out, category-representative facts;
- unexplained intent-bearing regions and structural completeness violations;
- conservation of source-grounded facts through EvidenceIR, SemanticIR, and IntentIR;
- provenance, conflicts, assumptions, and explicit residual accounting;
- participation of every production extractor in the canonical workflow;
- robustness across vendors, layouts, models, and semantics-preserving source transformations;
- measurement population, uncertainty, provider availability, and artifact currency; and
- eventual executable lowering and behavioral validation after upstream capture is adequate.

A single blended percentage is deliberately rejected. It could let a large test-count or currency gain hide a
semantic regression. Hard honesty and conservation failures cannot be averaged away.

## Five trajectory states

- **Converging** — hard invariants hold and at least one important objective deficit improves on adequate,
  preferably held-out evidence without a material regression elsewhere.
- **Diverging** — a hard invariant fails, source-grounded content disappears without a residual, fabricated
  facts increase, held-out semantic quality worsens, or a required producer leaves the canonical path.
- **Stalled** — important measured gaps remain and a sequence of comparable revisions shows no meaningful gain.
- **Mixed** — material gains and losses coexist; the trade-off is surfaced for review rather than averaged.
- **Unmeasurable** — the required source oracle, gold, provider observation, or comparable baseline is absent.
  The correct next task is to build the missing instrument, not to report success.

## The intended control loop

```text
objective contract
  → versioned trajectory snapshot
  → exact/statistical comparison
  → first-failing-stage diagnosis
  → ranked, reproducible task proposal
  → task-tree-owned implementation
  → verification and a new snapshot
```

Ranking is transparent: hard honesty/currentness defects first, then confirmed source evidence loss, held-out
semantic regression, missing high-value oracles, persistent residual families, and finally breadth or
efficiency. The report must state the affected documents, modalities, first failing stage, evidence strength,
uncertainty, expected impact, and reproduction command.

Automation may report, fail exact hard gates, generate replay plans, and propose the next task. It may not
silently approve or mutate canonical semantic truth. Task-tree and review boundaries remain in force.

## Existing building blocks

This is composition, not a clean-sheet measurement program. SpecForge already has exact convergence snapshots,
artifact-chain currency, held-gold P/R/F1, Lincoln–Petersen and Chao recall estimates, source-region accounting,
closure invariants, category-aware completeness, residual reporting, and strict target validation.

Their blind spots are also known:

- a stable fixed point can be incomplete;
- capture–recapture cannot see a fact every extractor misses and is optimistic when tiers are correlated;
- a linked region can still be only partly understood;
- competency questions prove that the schema can answer a question, not that all source instances were found;
- strict-valid target output proves validity, not PDF understanding.

The controller keeps those instruments separate and complementary.

## Current production-path caveat

`converge` is the default orchestration command, but it does not yet compose every production extractor. Its
ordinary path runs visual enrichment, EvidenceIR construction, `nlp-enrich`, downstream IR construction,
constraint promotion, and an NLI quality measurement. The standalone `extract-contracts`, `signal-resolve`,
and `recover-register-bits` capabilities are not directly invoked, and the NLI measurement is not the same as
the IntentIR demotion path. Until every capability is integrated, deliberately scheduled, or explicitly
reported omitted, “default end to end” must not be read as “all shipped extraction capabilities.”

## Verification strategy for the controller

The evaluator must prove it notices bad outcomes:

- metamorphic fixtures preserve facts under harmless layout changes and change facts or residuals when source
  meaning changes;
- controlled omissions, fabrications, broken provenance, and disabled stages are injected to ensure the
  trajectory gates reject them;
- each stage promotion is checked as a concrete translation, not trusted because the transformer passed its
  own unit tests;
- statistical drift detection is enabled only after enough comparable snapshots exist; otherwise the report
  says `insufficient_history`.

## Implementation and verification status

The durable objective/priority decision, controller design, code-path audit, and this public contract are the
documentation slice `SPEC-TO-INTENT-ALIGNMENT.0`. Implementation is decomposed into per-category contracts,
canonical-path capability accounting, real multimodal production, held-out vertical slices, and finally the
versioned trajectory controller. Until those leaves close, the chapter describes an accepted direction—not a
claim that automatic steering already runs.

The detailed design and literature mapping live in
[`docs/research/specforge-trajectory-control.md`](../../../research/specforge-trajectory-control.md).
