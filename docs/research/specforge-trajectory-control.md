# SpecForge trajectory control: measuring convergence, stall, and divergence

- Date: 2026-08-11
- Status: implemented through the first evidence-composed snapshot and owned recommendation (`.5b`)
- Owner: `SPEC-TO-INTENT-ALIGNMENT.0` (design), `.5` (implementation)

## Direct answer

SpecForge can steer itself toward specification-to-executable intent, but not by counting completed tasks,
pipeline passes, tests, or emitted files. It needs a closed loop over **source-understanding outcomes**:

```text
objective contract → measure → compare → diagnose → rank next gap → owned task → verify → new snapshot
```

The loop may automatically detect, gate, diagnose, and recommend. It must not silently change canonical
semantic truth. Promotion remains review-gated because the extraction target is normative intent, not an
ordinary optimization variable.

## What already exists

| Instrument | What it establishes | What it cannot establish alone |
| --- | --- | --- |
| Exact `converge` snapshots | one run reached a fixed point; same-count rewrites and protocol record order are visible | that the fixed point contains all source intent |
| `CHAIN-CURRENCY` and corpus frontier | persisted artifacts match the current binary and the refresh set is honest | semantic correctness or recall |
| Held-gold extraction P/R/F1 | exact correctness on labeled fact kinds and documents | unseen facts, unsupported categories, or modalities absent from gold |
| Lincoln–Petersen + Chao estimates | a lower bound on misses visible to multiple tagged extractor tiers | systematic blind spots shared by every tier; tier correlation makes recall optimistic |
| Region accounting | intent-bearing source regions with no linked fact | whether a linked fact fully represents the region |
| Closure invariants | exact/gated structural holes such as register tiling or missing functional attributes | open-ended prose intent and category-wide population recall |
| Category-aware completeness | different document purposes receive different applicable dimensions | a complete per-category answer key |
| ISF strict checks | emitted syntax and current target invariants are valid | whether the PDF was understood or whether omitted behavior exists |

The instruments are complementary. The existing completeness research explicitly establishes that no one
of them is sufficient: capture–recapture cannot see facts every extractor misses, soft coverage can hide an
unaccounted region, and competency questions prove schema answerability rather than population completeness.

## The objective contract

The controller needs a reviewed, machine-readable contract. For each supported document category it records:

- required evidence modalities: prose, tables, structure, figures, captions, timing/state diagrams;
- required IntentIR semantic families: actors, signals, fields, registers, relations, state, behavior, timing,
  constraints, transactions, symbols, storage, conflicts, assumptions, and residuals as applicable;
- non-applicable families, with a reason instead of a zero score;
- source→EvidenceIR→SemanticIR→IntentIR conservation expectations;
- the evaluation oracle and minimum evidence population for each claim;
- hard invariants that no optimization may trade away;
- eventual executable-lowering requirements, evaluated only after upstream semantic capture is adequate.

This is the Goal/Question/Metric discipline applied to SpecForge: every metric answers a named question under
an explicit goal, so an attractive measurement cannot silently become the objective. Basili and Weiss's
goal-directed measurement method is the direct precedent
([IEEE DOI 10.1109/TSE.1984.5010301](https://doi.org/10.1109/TSE.1984.5010301); [NASA/SEL summary](https://ntrs.nasa.gov/citations/19990021255)).

## The trajectory vector

Persist these dimensions per reviewed revision, category, document, modality, and fact kind. Keep the vector;
do not publish an authoritative weighted total.

1. **Source capture** — pages/elements/tables/figures/captions represented; intent-bearing regions accounted;
   modality presence with no producing extractor is a blind-spot signal.
2. **Semantic correctness** — held-gold precision/recall/F1, false-fact severity, inter-annotator reliability,
   and source entailment on promoted claims.
3. **Semantic completeness** — unexplained regions, exact closure violations, competency-question coverage,
   capture–recapture miss bounds, and category-required surfaces absent.
4. **Stage conservation** — every source-grounded fact survives the required IR boundaries or gains a typed,
   source-linked residual explaining the loss.
5. **Provenance and honesty** — provenance closure, unresolved-conflict/residual inventory, fabricated-fact
   count, and residual aging. A falling fact count is not automatically bad when fabrication was removed.
6. **Production-path participation** — every production extractor is integrated, explicitly scheduled, or
   reported omitted; standalone capability is not counted as default end-to-end capability.
7. **Generalization and robustness** — held-out vendor/layout/category results, metamorphic invariance, seeded
   miss/fabrication detection, deterministic replay, and cross-provider/model variance.
8. **Operational confidence** — current provider availability, labeled population size, confidence intervals,
   artifact currency, and the age of the last comparable measurement.
9. **Executable readiness** — source-grounded IntentIR facts lowered, residualized, and behaviorally validated.
   This is retained as an endpoint dimension but does not outrank upstream capture while ADR 0033 applies.

Every value carries its denominator, oracle, sample size, confidence/uncertainty, artifact fingerprint, and
measurement version. A percentage without those fields is non-authoritative.

## State classification

Let `H` be hard invariants and `Q` the objective vector between comparable reviewed snapshots.

- **Converging**: every `H` holds; at least one material deficit in `Q` improves beyond its noise/uncertainty;
  no other material dimension regresses; the improvement appears on held-out or newly covered evidence, not
  only the training fixture.
- **Diverging**: an `H` failure appears; a source-grounded fact disappears without a residual; fabricated facts
  increase; held-out semantic precision/recall declines significantly; a modality/category loses its producer;
  or the canonical workflow silently stops exercising a required capability.
- **Stalled**: important measurable deficits remain, but no material deficit improves during a configured
  sequence of comparable snapshots or completed leaves.
- **Mixed**: material improvements and regressions coexist. The controller exposes the Pareto trade-off and
  requires a decision; it does not average the regression away.
- **Unmeasurable**: a required gold set, source-presence oracle, provider run, or comparable baseline is absent.
  Missing evidence creates an oracle-building task, not a success score.

The state is computed hierarchically—document/fact kind, then category/modality, then program. A category that
diverges cannot be hidden by many easy documents improving elsewhere.

## Trend and change detection

Use the simplest valid comparison for each measurement:

- **Exact paired deltas** for deterministic artifacts, stage counts, identities, provenance, and residuals.
- **Paired item outcomes and bootstrap confidence intervals across documents** for held-gold semantic metrics;
  do not treat correlated facts from one PDF as independent corpus samples.
- **CUSUM** for small persistent shifts once a stable, comparable history exists. NIST documents CUSUM as more
  sensitive than ordinary Shewhart charts for small mean shifts
  ([NIST Engineering Statistics Handbook](https://www.itl.nist.gov/div898/handbook/pmc/section3/pmc323.htm)).
- **ADWIN** when provider/model or corpus streams genuinely change over time; it adapts the comparison window
  and supplies false-positive/false-negative bounds
  ([Bifet and Gavaldà, SDM 2007](https://doi.org/10.1137/1.9781611972771.42)).

CUSUM/ADWIN are later instruments, not decorations. They require enough compatible snapshots and predeclared
false-alarm tolerances. Until then, report `insufficient_history`, never an invented trend.

## Make the evaluator prove it can see failure

The controller itself can become a Goodhart target. Protect it with three independent techniques:

1. **Metamorphic testing.** Semantics-preserving PDF transformations—pagination, harmless whitespace, stable
   table row order where order is irrelevant—must preserve facts. Meaning-changing transformations—remove a
   normative sentence, invert a polarity, delete a table row—must remove/change a fact or create a residual.
   This addresses missing exact oracles by asserting relations between executions
   ([Chen, Cheung, and Yiu, 1998](https://arxiv.org/abs/2002.12543)).
2. **Evaluator mutation.** Seed controlled omissions, fabrications, broken provenance, stage drops, and disabled
   extractor participation. The trajectory gates must kill those mutants. Mutation adequacy measures whether
   a test surface detects deliberately injected faults
   ([Jia and Harman, IEEE TSE 2011](https://doi.org/10.1109/TSE.2010.62)).
3. **Per-run translation validation.** For each stage promotion and eventual ISF lowering, validate the concrete
   translation result against the source-stage contract instead of assuming the transformer is correct. This
   follows the translation-validation approach of checking each compiler run
   ([Pnueli, Siegel, and Singerman, 1998](https://doi.org/10.1007/BFb0054170)).

## Automatic diagnosis and next-task selection

For every red or unknown dimension, emit a typed `TrajectoryGap`:

```text
objective_id · category · document/modality · observed · target · uncertainty
first_failing_stage · likely_owner · reproduction · hard/soft · estimated_impact
```

Rank candidates transparently and lexicographically:

1. hard honesty, provenance, currency, or deterministic conservation failure;
2. confirmed source evidence lost before IntentIR;
3. high-impact held-out semantic precision/recall regression;
4. missing oracle or blind spot with high value of information;
5. persistent residual family blocking many documents;
6. breadth, speed, storage, or convenience work.

Within a tier, prefer larger affected populations, stronger causal evidence, smaller reversible slices, and
lower uncertainty. The report must show why the selected candidate outranks the others. Weighted search may
be offered for exploration, but not as the authoritative decision because weights can conceal unacceptable
regressions; Pareto/multi-objective comparison preserves the trade-off
([Li, Chen, and Yao, 2020](https://arxiv.org/abs/2002.09040)).

The automatic action is a proposed, reproducible task packet or rescan plan. The task-tree workflow accepts,
reshapes, or rejects it before implementation. This is a monitor/analyze/plan/execute feedback loop under a
shared knowledge base, consistent with autonomic-computing control principles
([Kephart and Chess, 2003](https://doi.org/10.1109/MC.2003.1160055)).

## First implementation sequence

1. Formalize per-category objective contracts (`SPEC-TO-INTENT-ALIGNMENT.1`).
2. Record which production capabilities participate in the canonical path (`.2`).
3. Close the real multimodal producer gap (`.3`).
4. Establish held-out vertical slices and stage-loss accounting (`.4`).
5. Persist versioned trajectory snapshots; classify state; protect evaluators with seeded negatives; emit the
   reviewable next-task recommendation (`.5`). `.5a` supplies the strict nine-dimension state/ranking engine;
   `.5b` attaches the frozen product evidence and emits the first owned recommendation, `.6`.

The first snapshot is `diverging` because current exact hard gates count 41 fabricated canonical facts, 45
provenance failures, and 33 unexplained stage drops. Its history status remains `insufficient_history`, so this
baseline makes no direction or stall claim. The report ranks `.6` honesty/provenance ahead of `.7` source loss,
`.8` residual actionability, and `.9` capability breadth, with report-only review authority unchanged.

The controller should initially report and recommend. Only exact hard invariants should fail CI. Statistical
or heuristic signals graduate to gates after their false-alarm behavior is calibrated on real history.
