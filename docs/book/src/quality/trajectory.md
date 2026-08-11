# Trajectory And Automatic Steering

SpecForge is meant to move specifications toward executable intent. A green build, a stable convergence loop,
or a strict-valid `.isf` file does not by itself prove movement toward that objective: all three can be true
while important source intent was never captured.

The project therefore treats trajectory as a source-understanding control problem. The accepted design is
tracked by `SPEC-TO-INTENT-ALIGNMENT`. The generic state/ranking engine, retrospective evidence baseline, and
first current-binary replay now ship; the controller has selected the next owned slice without changing
canonical semantics.

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
at EvidenceIR → SemanticIR, while no cell first fails at SemanticIR → IntentIR. Those exact outcomes feed the
first automatic retrospective-baseline report; current-product claims require a later hash-pinned replay.

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

The example is illustrative; `.5a` contains no product values. `.5b` attaches the frozen `.4c` report and `.2`
capability evidence in a separate composition module, so controller policy remains untuned. `.6a` adds an
independent current-binary replay authority without changing the generic engine. The controller can report and
propose; it cannot edit canonical IR.

## Evidence-composed baseline and currentness boundary

The tracked input and report live under `crates/specforge/test_data/trajectory/`. The composition tool derives
them from the byte-pinned `.4c` result, a provider-free `.2` capability observation, and tracked current-binary
replay evidence. A `converge` test compares all 17 observation rows with the live registry, strict replay
validation rejects identity/currency/preservation mutants, and the generic runner reproduces the report byte
for byte:

```console
cargo run --quiet -p specforge --example trajectory_snapshot -- --check
cargo run --quiet -p specforge --example trajectory_controller -- \
  crates/specforge/test_data/trajectory/controller_input.json \
  | cmp - crates/specforge/test_data/trajectory/trajectory_report.json
```

The state is `diverging`, while `history_status` is `insufficient_history`. These fields answer different
questions. Divergence now comes from an exact currentness failure: only 1/12 reviewed documents has a
current-binary replay, leaving 11 unqualified. Missing history independently prevents the controller from
claiming that the state is improving, worsening over time, or stalled. The `.4c` quality counts remain useful
retrospective baseline measures, but they are not presented as whole-population current-product facts.

| Dimension | First exact observation | Status |
| --- | --- | --- |
| Source capture | source regions 14/14; required-modality captures 14/14 | meets target |
| Semantic correctness | canonical IntentIR precision 7/48 | deficit |
| Semantic completeness | recall 7/40; supported categories 0/6 | deficit |
| Stage conservation | frozen conserved or residualized crossings 21/54 | retrospective deficit |
| Provenance/honesty | frozen closure 3/48; fabricated-fact rate 41/48 | retrospective deficit |
| Production participation | accounted 17/17; integrated or scheduled 12/17 | deficit, fully reported |
| Generalization/robustness | reviewed category-oracle coverage 6/6 | meets target |
| Operational confidence | complete review 12/12; current replay 1/12; provider-free execution 5/10 | hard deficit |
| Executable readiness | required-modality document accounting 0/12 | deficit |

The current hard gate records 11 documents without current-binary replay. The frozen baseline still records 41
fabricated canonical facts, 45 provenance-closure violations, and 33 unexplained stage drops, but the controller
does not assume they all survive. It ranks the existing leaves as follows:

1. `.6` — complete current-binary honesty qualification, then repair defects that reproduce (`hard_invariant`,
   11 unreplayed documents);
2. `.7` — recover 33 source-to-evidence canonical losses (`source_evidence_loss`);
3. `.8` — make 0/24 required residual observations actionable (`persistent_residual`); and
4. `.9` — measure and resolve five omitted capability islands (`breadth_efficiency`).

The recommendation is `.6`. All four task IDs existed before evaluation, the complete ordering remains in the
report, and review is still required before implementation.

### What the first current replay proved

`.6a` isolates all four deterministic stages below a fresh `.project-data/tmp` root; it refuses absolute paths,
parent traversal, symlink escapes, existing outputs, and off-repository source authority. The source must first
be copied onto the repository volume and byte-verified. Canonical `generated/` artifacts and the frozen `.4c`
result are never overwritten.

The first replay uses the hash-equal 827,669-byte RISC-V AIA source. The reviewed `table_0004` remains the same
20-row × two-column table of contents, and its expected canonical fact population remains empty. The frozen
artifact labeled it `timing_parameter` and promoted 19 timing records through all three later stages. Current
runtime normalization labels it `unknown`; EvidenceIR, SemanticIR, and IntentIR each promote zero timing
records. Thus false positives and unprovenanced records both move 19→0 without losing a true positive (0→0).

The evidence report retains the source, baseline, and replay hashes plus the reproduction command and exact
cleanup census. The 132 MB replay output, failed empty first root, and copied source were removed after their
facts were captured. This proves one current repair already exists; it does not extrapolate that result to the
remaining 11 documents.

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
incomplete result and upstream blocker diagnosis; `.5a` supplies the generic controller; and `.5b` composes the
first retrospective-baseline snapshot; `.6a` supplies the first current-binary replay and hard artifact-currency
gate. Automatic steering still recommends `.6`, but remains report-only and reviewable.

The detailed design and literature mapping live in
[`docs/research/specforge-trajectory-control.md`](../../../research/specforge-trajectory-control.md).
