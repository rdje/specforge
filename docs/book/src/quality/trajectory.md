# Trajectory And Automatic Steering

SpecForge is meant to move specifications toward executable intent. A green build, a stable convergence loop,
or a strict-valid `.isf` file does not by itself prove movement toward that objective: all three can be true
while important source intent was never captured.

The project therefore treats trajectory as a source-understanding control problem. The accepted design is
tracked by `SPEC-TO-INTENT-ALIGNMENT`. The generic state/ranking engine now ships; the first composed product
snapshot and task proposal are still pending.

The controller's reviewed input boundary is the
[Source-to-Intent Completeness Contract](../source-to-intent-contract.md). It fixes category-specific modalities,
canonical semantic families, strict held-gold floors, and non-applicability rules before snapshots or task
ranking can be implemented.

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

The first controller-specific instrument now ships as `ir::source_to_intent_eval`. It consumes a versioned,
review-locked, repository-relative dataset of bounded SourceIR/EvidenceIR/SemanticIR/IntentIR snapshots and
emits deterministic per-cell, per-document, per-category, and global stage-loss results. Its mutation controls
reject omission, fabrication, provenance loss, silent stage drops, missing modalities, and inactionable
residuals. The first 12-document reviewed population is locked without extractor tuning and its exact outcome
is now published. All six categories are `incomplete`; 10/14 cells first fail at SourceIR → EvidenceIR and four
at EvidenceIR → SemanticIR, while no cell first fails at SemanticIR → IntentIR. The automatic controller does
not yet run on product evidence.

## The controller engine

`SPEC-TO-INTENT-ALIGNMENT.5a` implements the policy in a strict version-1 JSON contract at
`doctrine/trajectory_controller_input_schema.json` and the matching `ir::trajectory` API. The input must contain
each of the nine dimensions listed above exactly once. Every metric preserves its numerator and denominator,
target, improvement direction, material-change threshold, hard/required status, oracle, population, uncertainty,
and repository-relative evidence. Unknown fields and missing dimensions fail closed.

The five states are deliberately asymmetric:

| State | Deterministic rule |
| --- | --- |
| `diverging` | a current hard gate/metric fails, or a comparable exact metric materially regresses |
| `mixed` | material improvements and regressions coexist without a hard failure hiding them |
| `converging` | hard gates hold and at least one exact comparable deficit improves with no regression |
| `stalled` | deficits remain unchanged across the declared comparable-snapshot window |
| `unmeasurable` | a required oracle/comparison is absent or history is too short for the requested claim |

An estimated point cannot claim a trend from a history containing only point values. Until paired uncertainty
is present, classification reports the missing comparison. This is why CUSUM, ADWIN, or bootstrap-based rules
do not appear merely because their names are useful: they become authoritative only with compatible history.

Gap ranking is lexicographic, not weighted: hard invariants, source evidence loss, semantic regression, missing
oracles, persistent residuals, then breadth/efficiency. Within a tier, affected population, causal confidence,
reversible slice size, and uncertainty remain visible. The winning gap must already name a real task-tree ID.
For example, the authority and one metric have this shape:

```json
{
  "measure": { "numerator": 7, "denominator": 40 },
  "target": {
    "operator": "at_least",
    "value": { "numerator": 1, "denominator": 1 }
  },
  "uncertainty": { "kind": "exact" },
  "authority": {
    "mode": "report_only",
    "canonical_semantic_mutation_allowed": false,
    "task_tree_review_required": true
  }
}
```

The example is illustrative; `.5a` contains no current product values. `.5b` must attach the frozen `.4c`
report and `.2` capability evidence without changing either authority, persist the first complete snapshot, and
open the recommended owned leaf. The controller can report and propose; it cannot edit canonical IR.

## Current production-path caveat

`converge` is the default orchestration command, but it does not compose every production extractor. Its
ordinary path runs visual enrichment, EvidenceIR construction, `nlp-enrich`, downstream IR construction,
constraint promotion, and an NLI quality measurement. The standalone `extract-contracts`, `signal-resolve`,
and `recover-register-bits` capabilities are not directly invoked, and the NLI measurement is not the same as
the IntentIR demotion path.

That boundary is now executable rather than inferential. Every successful run emits 17 machine-readable
capability rows spanning all 16 production subcommands, with `integrated`, `scheduled`, or `omitted`
participation plus a per-run state and reason. A complete CLI partition test fails if a new subcommand is not
classified or a production command is absent from the ledger. “Default end to end” therefore means exactly
the integrated rows that report `executed`, never every feature that happens to compile.

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

The durable objective/priority decision, controller design, code-path audit, and this public trajectory contract
are the documentation slice `SPEC-TO-INTENT-ALIGNMENT.0`. The per-category source-to-IntentIR acceptance contract
is `.1`; the guarded canonical-path capability ledger is `.2`; typed visual activation is `.3`; the vertical
evaluator foundation is `.4a`; `.4b` locks the balanced reviewed population; `.4c` publishes the exact
incomplete result and upstream blocker diagnosis; and `.5a` supplies the generic multi-metric controller engine.
The `.5b` composition/snapshot leaf remains. Until it closes, automatic steering does not run on product evidence.

The detailed design and literature mapping live in
[`docs/research/specforge-trajectory-control.md`](../../../research/specforge-trajectory-control.md).
