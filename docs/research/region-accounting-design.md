# Region Accounting & Backward Traceability — instrument design

> Owned by `INTENT-COMPLETENESS-RESEARCH.3`. The **foundational miss detector**
> from [`intent-capture-completeness.md`](intent-capture-completeness.md) §3/§4:
> turn the impossible output-side question ("what did we miss?") into a checkable
> input-side one ("which intent-bearing source region produced nothing?").
> Design only — implementation is a future owned tree.

## Principle

> Every meaning-bearing source region must either link forward to ≥1 extracted
> fact, **or** be explicitly classified as non-intent (with a reason), **or** be
> tagged deferred (a known, declared gap). Anything else — an intent-bearing
> region with an empty backward index — is a **candidate miss**, surfaced as a
> residual. No ground truth required.

The classifier need not be perfect. A *high-recall* "does this region carry
intent?" gate is enough: its job is to convert **silent total misses** into
**visible residuals**. A false positive (a non-intent region flagged) costs a
review item, never a fabricated fact — so the design deliberately errs toward
surfacing.

## Step 1 — enumerate regions (from SourceIR)

SpecForge already segments the document; regions are the atomic source units it
produces. The instrument enumerates, each with a stable id:

| Region kind | Source (SourceIR) | id |
| --- | --- | --- |
| prose unit | `content_elements[]` → sentence split | element id + sentence index |
| table | `structured_tables[]` | `table_id` |
| table row/cell | rows of a `structured_table` | `table_id` + row/col |
| figure | `visual_assets[]` / `visual_evidence[]` | `asset_id` |
| caption | caption of a figure/table | asset/table id + `:caption` |
| section header | `document_sections[]` | section id |

Granularity is tunable: prose at sentence level, tables at table level first
(row/cell later for register/encoding tables). The denominator is **regions**,
not tokens.

## Step 2 — classify each region (intent / non-intent / deferred)

A conservative, explainable classifier (no heavy ML required to start):

- **intent-bearing** (must yield a fact):
  - prose: matches normative/declarative patterns (modal verbs must/shall/
    until/while; signal-declaration shapes; role/relation verbs; timing language)
    — reuse the existing statement-classification signals (`StatementClass`,
    `is_signal_value_constraint`, etc.).
  - tables: `table_kind ∈ {SignalDescription, RegisterMap, TimingParameter,
    Encoding, …}` (intent-carrying kinds).
  - figures: `DiagramKind ∈ {Timing, StateMachine, BlockConnectivity}`.
- **non-intent** (tagged with a reason): boilerplate, TOC, legal, revision
  history, pure prose exposition, `table_kind = Unknown/Layout`,
  illustrative-only figures (existing `VisualEvidenceRole::Illustrative`).
- **deferred** (a *declared* gap, not a silent miss): figure kinds we cannot yet
  decode (no VLM run / unsupported diagram), scanned/OCR-poor regions.

Honesty rule: every region lands in exactly one bucket; "non-intent" and
"deferred" both require an explicit reason string. There is no silent fourth
bucket — that is the whole point.

## Step 3 — build the backward-traceability index (invert existing forward provenance)

Forward provenance already exists; the instrument inverts it. Forward links to
harvest, per fact type:

- `ExtractedStatement.evidence_span_ids`, `…related_visual_evidence_ids`
- `SignalConstraintRecord.supporting_statement_ids`, `…source_text`
- `ActorSignalRelation.source_statement_ids`
- `RegisterRecord` / `TimingConstraintRecord` → originating `table_id`
  (`TableSignalDeclarationProvenanceRecord` already links table→signal decls)
- `SignalSemanticHintRecord` → source kind + region
- VLM-derived records → originating `asset_id`

Construct `backward: region_id → {fact_ids}`. A region with `backward[id] = ∅`
is **unexplained**.

**Gap this exposes in the current code (from the coverage audit):** forward
provenance is rich but *partial* — some synthesized records cite a `table_id`
but not the specific row/cell; some prose-derived facts cite `statement_id` but
the statement→`content_element`→region chain may be lossy. A precondition tree
may be needed: *make every fact carry a resolvable region id*. That itself is a
completeness improvement (provenance completeness ⊂ capture completeness).

## Step 4 — emit residuals + the report

For each **intent-bearing** region with empty backward index, emit an
`UnexplainedRegionResidual { region_id, kind, reason: "intent-bearing region
produced no fact", excerpt, classifier_signals }`. Aggregate into a
`RegionAccountingReport`:

```
RegionAccountingReport {
  regions_total, intent_bearing, non_intent, deferred,
  intent_explained,            // intent-bearing with >=1 fact
  intent_unexplained,          // == candidate misses
  coverage_pct = explained / intent_bearing,
  unexplained: Vec<UnexplainedRegionResidual>,
}
```

This is a section of the unifying `CompletenessReport` (framework §10). `validate`
surfaces it: Info when `intent_unexplained == 0`; Warning listing the unexplained
regions otherwise — each a concrete, located, reviewable residual.

## Where it rides the pipeline

- Most naturally computed at **EvidenceIR** (regions = SourceIR units; facts =
  EvidenceIR records) — same stage as the convergence report, and reuses the
  existing provenance fields.
- Cross-stage variant feeds the **inter-stage conservation** check (framework
  §7): a region explained at EvidenceIR whose fact disappears by SemanticIR/
  IntentIR without a residual = a pipeline-introduced miss.

## How it confirms the (B) blind-spot hypotheses (for free)

The coverage matrix (`.2`) hypothesized prose timing/width/enum/register facts
the table-only extractors miss. Region accounting **measures** this directly: a
normative prose sentence about timing/width/enum that produced no fact shows up
as an unexplained prose region. The hypothesis list becomes a measured residual
set on real specs — no separate effort.

## Safeguards & honest limits

- **Never fabricates:** the instrument only *flags* regions; it never invents
  facts. Flagged regions are review/rescan targets (can route to `nlp-enrich` /
  `extract-contracts` / a VLM rescan).
- **Classifier recall is the ceiling:** a region the classifier wrongly calls
  non-intent is still a silent miss. Mitigation: keep the intent gate
  high-recall (accept false positives), and periodically audit the *non-intent*
  bucket (e.g., LLM completeness critic, framework §8.4) — the non-intent bucket
  is itself a place misses can hide, so it is not above suspicion.
- **Deferred ≠ done:** deferred regions are *declared* gaps that still count
  against full coverage; they are reported separately so "we chose not to decode
  this figure yet" never masquerades as "nothing here."

## Implementation slices this design implies (for `.7`)

1. Region enumeration + id scheme over SourceIR units.
2. Backward-traceability index (invert forward provenance) + a provenance-
   completeness precondition fix if facts don't carry resolvable region ids.
3. Intent/non-intent/deferred classifier (reuse existing statement/table/figure
   signals; high-recall).
4. `UnexplainedRegionResidual` + `RegionAccountingReport` + `validate` surface
   (extend the `CompletenessReport`).
