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
residuals. The first 12-document reviewed population was locked without extractor tuning and its original exact
outcome remains the retrospective baseline. Current-product claims come only from a later hash-pinned replay;
the current replay has one supported category and five incomplete categories.

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

The example is illustrative; `.5a` contains no product values. `.5b` first attached the frozen `.4c` report and
`.2` capability evidence in a separate composition module, so controller policy remained untuned. `.6a` added
one independent current-binary replay; `.6b.i` replaced that operational input with a hash-pinned 12-document
current result; `.6b.ii.b` qualified the register-access/table-provenance repair against the same population;
`.6c.ii` qualifies the timing-unit/table-provenance repair, `.6d.ii.a` publishes the physical-applicability
qualification, and `.6d.ii.f.iv.b` publishes the later structural-register repair. The generic controller engine
is unchanged. The controller can report and propose; it cannot edit
canonical IR. The reviewed-corpus composer and exact assertions now compile only below `#[cfg(test)]`; they are
not part of the production IR module graph.

## Evidence-composed current snapshot and retrospective boundary

The tracked input and report live under `crates/specforge/test_data/trajectory/`. The composition tool derives
them from the qualified current result, a provider-free `.2` capability observation, and the portable 12-source /
48-stage replay manifest. A `converge` test compares all 17 observation rows with the live registry. Strict replay
validation rejects source-hash, population, artifact-path, result, and cleanup mutants; the generic runner
reproduces the report byte for byte:

```console
cargo test --quiet -p specforge-conformance --lib test_support::trajectory_snapshot
cargo run --quiet -p specforge --example trajectory_controller -- \
  crates/specforge/test_data/trajectory/controller_input.json \
  | cmp - crates/specforge/test_data/trajectory/trajectory_report.json
```

The state is `unmeasurable`, while `history_status` is `insufficient_history`. All three hard gates now pass:
replay currency is 12/12, fabricated canonical facts are zero, and provenance closes 42/42. Missing comparable
history prevents the controller from claiming a trend or stall even though the current quality snapshot is
materially better. The `.4c` counts remain the retrospective baseline; the `.f.iv.b` replay from clean production
revision `e125aac7` is the current product authority.

| Dimension | First exact observation | Status |
| --- | --- | --- |
| Source capture | source regions 14/14; required-modality captures 12/14 | capture deficit |
| Semantic correctness | canonical IntentIR precision 39/39 | meets target |
| Semantic completeness | recall 39/40; supported categories 1/6 | deficit |
| Stage conservation | conserved or residualized crossings 117/118 | deficit |
| Provenance/honesty | closure 42/42; fabricated-fact rate 0/39 | meets target |
| Production participation | accounted 17/17; integrated or scheduled 12/17 | deficit, fully reported |
| Generalization/robustness | reviewed category-oracle coverage 6/6 | meets target |
| Operational confidence | complete review 12/12; current replay 12/12; provider-free execution 5/10 | replay target met; execution deficit |
| Executable readiness | required-modality document accounting 5/12 | deficit |

The current replay, fabrication, and provenance hard gates all have zero violations. One unexplained canonical
stage drop remains, and four of 24 required residual observations are actionable. The metric controller ranks the
remaining leaves as follows:

1. `.7` — recover the sole APB source-to-evidence canonical loss (`source_evidence_loss`);
2. `.8` — make the remaining 20/24 required residual observations actionable (`persistent_residual`); and
3. `.9` — measure and resolve five omitted capability islands (`breadth_efficiency`).

The metric recommendation is `.7`. Planned `.6e` is superseded because `.f.iv.a` supplied the structural repair
and `.f.iv.b` supplied its whole-population proof. Final behavioral `.f.v` remains a release-signoff invariant and
therefore precedes controller-ranked work. All task IDs exist in the task tree, the complete metric ordering
remains in the report, and review is still required before canonical mutation.

The word “current” above is revision-bound. The tracked 24/2/16 result is valid for its pinned `b977a51f`
production revision; fixed-input artifact-chain currency does not extend it across later PDF-to-SourceIR changes.
The first clean `.f.iv` full-PDF replay at `0d218116` therefore remains an unpublished diagnostic. It completed all
12 sources and 48 stages, but Arm Debug lost 12 correct register/access facts and GIC-400 emitted 15 source-named
registers without access. AMD's former packed-layout false register correctly disappeared. Actual execution
hashes in the replay manifest prove fresh stage movement; the cell fixture's `original_sha256` values are frozen
review identities, not current artifact hashes.

`.f.iv.a` now closes both generic carrier defects. One parenthesized qualifier may follow a closed header role,
and an already-classified register map may retain access from an explicit header or one unambiguous closed-literal
column. Direct PDF replay restores Arm 12/12 and GIC-400 15/15 while AMD remains empty. Exact retained-chain
reconciliation is 24/24 current and zero stale at every stage. `.f.iv.b` now publishes that repaired population.
All 12 sources and 48 stages complete under an evidence-recorded CPU/16-page ingest policy. Aggregate IntentIR
moves to 39/0/1 TP/FP/FN; GIC-400 supplies 15 exact facts and AMD supplies no false register. APB's pre-existing
missing `PSEL=HIGH` obligation is the sole canonical miss and unexplained drop. The replay driver's exact four
post-`--` values are guarded by a focused executable test.

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
facts were captured. This proved one current repair already existed; `.6b.i` subsequently replayed the complete
population rather than extrapolating from it.

### Replaying the complete reviewed population

The population runner reads the unchanged reviewed dataset and an untracked runtime map for the eight necessary
external PDFs. The map contains caller-authorized absolute inputs only while the run is active; it is never
persisted in tracked evidence. Every external file must share the repository filesystem device, is copied below
the fresh output root, and is verified against the reviewed SHA-256 before SourceIR runs. For example:

```json
{
  "schema_version": 1,
  "sources": [
    {
      "portable_id": "example-spec.pdf",
      "path": "<same-volume-authority>/example-spec.pdf"
    }
  ]
}
```

```console
python3 -B scripts/replay_source_to_intent_population.py \
  --output-root .project-data/tmp/<fresh-population-root> \
  --external-source-map .project-data/tmp/<untracked-runtime-map>.json \
  --replay-id <portable-replay-id> \
  --owner <task-leaf-id> \
  --dataset-id <portable-dataset-id>
```

The runner refuses a dirty production-Rust tree, incomplete external-map coverage, source hash drift, off-volume
authority, an existing output root, and unsafe paths. It writes one isolated four-stage tree per document, then
uses the review fixture's data-defined projections and the same Rust evaluator to produce the comparable result.
The tracked manifest contains portable source identities, all 48 stage hashes, tool hashes, current dataset/result
identities, and cleanup evidence. For a controlled qualification run, ingestion environment overrides are
recorded in every document command, making the policy part of the evidence rather than hidden process state. That
3,913-file / 1,090,948-KiB workspace and runtime map were deleted exactly; canonical `generated/`, the
reviewed dataset, and the frozen `.4c` result stayed byte-identical.

The access-carrier replay moves IntentIR TP/FP/FN from 7/22/33 to 19/10/21. All twelve Arm Debug register/access
keys are exact and retain `table_0044`; AMD IOMMU and GIC-400 each gain table provenance without changing their
false canonical key. No other cell changes. Provenance closure becomes 17/29 and conservation 57/78. The largest
remaining bounded family is I2S receiver timing: five otherwise-correct constraints lose their `ns` unit, so
five false positives, five misses, and five unprovenanced records remain. `.6c` owns that repair.

The replay also exposed and drove closure of an operational defect. The 400-page Arm source was killed twice on
the old default Docling single-pass path after model loading. Bounded activation now derives from fixed total RAM,
a measured 75-MB/page working-set estimate, a 40% budget, and a 399-page maximum. On the 24-GiB host the default
is threshold 131 / batch 64. An override-free replay completed with exact path-normalized SourceIR and
validation-neutral EvidenceIR/SemanticIR/IntentIR fidelity. Unknown page count now fails closed; an operating-
system signal has an actionable typed diagnostic separate from both normal exit failure and the measured RAM
guard. This operational repair does not change the reviewed semantic result or controller ordering: `.6c`
remains the next semantic leaf.

The subsequent timing-carrier replay runs the same 12 unchanged sources and 48 isolated stages at committed
production revision `74a658b3`. Its only reviewed semantic change is the I2S receiver-timing cell: five
missing-unit false positives plus five misses become five exact `ns` true positives. Seven already-emitted
OpenCAPI records—four correct digital-skew facts and three incorrect analog channel-loss facts—gain direct table
provenance without changing their keys. No other reviewed cell changes. Aggregate TP/FP/FN becomes 24/5/16,
provenance closes 29/29, conservation becomes 72/88, source disposition becomes 4/14, and required-modality
document accounting becomes 2/12. All 19 prior true positives survive. The remaining five fabrications are the
three OpenCAPI analog records plus one AMD IOMMU and one GIC-400 register key; `.6d` owns the controller-selected
three-record family before `.7` can address recall loss.

The physical-applicability replay then runs those same sources and stages at committed production revision
`b977a51f`. Exactly the two analog channel-loss cells change: three `dB`/`dB_RMS` observations leave canonical
digital timing and become exact, source-linked, actionable residuals at SemanticIR and IntentIR. The other 12
cells, all 24 prior true positives, and 29/29 provenance remain exact. TP/FP/FN become 24/2/16; source
disposition becomes 6/14; required-modality document accounting becomes 4/12; residual actionability becomes
4/24; and physical-link becomes the first supported category. Two unrelated register fabrications and 16
unexplained stage drops remain.

The repaired-register publication replays the same 12 reviewed sources from `e125aac7`. GIC-400 changes from
0/1/15 to 15/0/0 TP/FP/FN, while AMD changes from a 0/1/0 fabrication to an honest empty 0/0/0 cell. No other
reviewed cell changes. Required-modality capture decreases from 13/14 to 12/14 because AMD's formerly spurious
table capture is no longer counted as successful capture; this is an honest metric trade, not a regression hidden
by the aggregate. IntentIR becomes 39/0/1, provenance 42/42, conservation 117/118, source disposition 7/14, and
required-modality document accounting 5/12. All hard gates pass. The three attempt roots—two interrupted and one
promoted—contained 5,616 files / 2,038,580 KiB and were deleted with the runtime map after publication; the
portable manifest retains the successful root's 3,094-file / 1,147,260-KiB exact cleanup census.

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
gate. The production-genericity program under `.6d.ii` is now complete: identity/spelling/prompt/corpus repair,
proof-carrying structural enforcement, 27 adversarial controls, and behavioral identity, alpha, paraphrase,
negative, held-out, and population evidence compose. The final matrix is 35 pass / zero fail / 16 unmeasurable /
zero invalid, and the reviewed population is 39/0/1 IntentIR TP/FP/FN with zero fabrication. This signs off one
specification-instance-neutral production core, not perfect recovery or product completion. The report-only
controller has zero hard-gate violations, remains `unmeasurable` for convergence due to insufficient comparable
history, and ranks `.7` first for the sole APB source-to-EvidenceIR canonical loss.

The completed whole-pipeline audit is
[`docs/research/production-genericity-pipeline-audit.md`](../../../research/production-genericity-pipeline-audit.md).
The frozen behavioral oracle is
[`docs/research/behavioral-genericity-qualification-design.md`](../../../research/behavioral-genericity-qualification-design.md).
The corrected held-out result is
[`docs/research/behavioral-held-out-qualification.md`](../../../research/behavioral-held-out-qualification.md).
The detailed trajectory design and literature mapping live in
[`docs/research/specforge-trajectory-control.md`](../../../research/specforge-trajectory-control.md).
