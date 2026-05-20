# R16-WAVEFORM-CONTRACT-MINING: timing diagram → contract (point #4 — the crux)

## Metadata

- Tree ID: `R16-WAVEFORM-CONTRACT-MINING`
- Status: `done`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #4, order 5 — **crux extraction
  thrust**, highest ceiling / research-grade; DELIVERED `2026-05-20`)
- Created: `2026-05-19`
- Last updated: `2026-05-20`
- Owner: repo-local workflow

## Goal

Timing diagrams are the densest temporal-intent source in protocol PDFs —
they *are* the timed automaton, drawn — and per the program thesis,
**automating accurate/reliable extraction from timing diagrams (with
prose) is THE hard problem**. Current VLM timing extraction is largely
defensive (junk-label guarding). Deliver:

1. **Structured waveform → partial trace**: per figure, recover signal
   lanes, edges, value spans, relative-delay annotations (e.g. "≥2
   cycles"), and causal/dependency arrows as a typed partial trace /
   scenario.
2. **Trace → generalized contract**: induce a ContractIR contract from
   one-or-more traces (specification mining from scenarios), with
   calibrated confidence.
3. **Cross-check vs prose**: agreement with the prose-derived contract
   boosts confidence; disagreement → explicit residual (never silent
   pick) — fidelity scored by `R16-CAPTURE-FIDELITY-GATES`.

## Non-Goals

- Not pixel-perfect waveform OCR for its own sake — the deliverable is a
  *contract*, with the trace as evidence/provenance.
- No fabricated generalization beyond what the trace + annotations
  license; under-determined → residual.

## Acceptance Criteria

- A figure → typed partial-trace extractor + a trace → ContractIR
  generalizer with provenance and calibrated confidence; prose
  cross-check wired to residual on disagreement.
- Measured improvement in figure-conformance fidelity
  (`R16-CAPTURE-FIDELITY-GATES`) on the real corpus vs. baseline;
  negative fixtures prove junk waveforms do not mint contracts.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree

- ID: `R16-WAVEFORM-CONTRACT-MINING.1`
  Status: `done`
  Goal: typed `waveform` intermediate (PartialTrace / LaneEdge /
  ValueSpan / RelativeDelay / CausalArrow) + generalization rules
  (trace → ActorContract candidates) + verifier shape
  (`evaluate_figure_trace` round-trip) + honest dormancy framing for
  `.3` (the actual extractor) — docs-only design, parallels prior R16
  `.1`s.
  Acceptance: `Design recorded in this tree + mirrored in mdBook per BOOK-METHOD-DOC; docs-only; scripts/run_docs_ci.sh green.`
  Verification: `passed` — see "Design (`.1` output)" below; book
    mirror; `mdbook build` green.
  Commit: `see Commit Log`

- ID: `R16-WAVEFORM-CONTRACT-MINING.2`
  Status: `done`
  Goal: implement typed `waveform` module + generalizer + round-trip
  verifier (synthetic-trace tests; no PDF parsing).
  Acceptance: `Typed module + generalizer + verifier + positive/negative tests; no producer wiring; zero artifact churn; scripts/run_ci.sh green.`
  Verification: `passed` — `crates/specforge/src/ir/waveform.rs`
    added: typed intermediate (`EdgeKind` /
    `LaneEdge`/`ValueSpan`/`RelativeDelay`/`CausalArrow`/`PartialTrace`)
    + `generalize_partial_trace(&PartialTrace) -> Vec<ActorContract>`
    (`RelativeDelay{min,max≥1, max≥min}` ⇒ `Eventually{Edge.Rose,
    Within{min,max}}` / Lowerable; degenerate delays ⇒ `Observe` +
    `Residual{reason="under-determined delay — bounds missing or
    invalid"}`; multi-tick `ValueSpan` ⇒ `Stable{Within{max=span_len}}`
    / Lowerable; next-tick `CausalArrow` ⇒
    `Eventually{Within{0,1}}`, wider gap ⇒ `Eventually{Within{0,gap}}`;
    bare `LaneEdge` not covered by another record ⇒ `Observe` +
    `Residual{reason="bare edge — no window licensed"}` —
    deduplicated per signal); `capped_confidence` caps to `Medium`
    (no single-figure High); `provenance` tags
    `EvidenceModality::Figure` with `figure:{id}` statement id +
    annotation source_text. `verify_contract_against_trace` reuses
    `evaluate_figure_trace` via `partial_trace_to_figure_trace` lift
    (spans apply first then edges; values best-effort parsed u64,
    fallback 0). 7 unit tests covering all generalization paths +
    verifier Pass/NotEvaluated semantics. Module registered in
    `ir/mod.rs`. Producer wiring deferred to `.3`; SemanticIr /
    IntentIr schemas unchanged ⇒ **zero artifact/fixture churn**.
    Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-WAVEFORM-CONTRACT-MINING.3`
  Status: `done` (`2026-05-20`; both honest-split sub-leaves `.3.1`
  + `.3.2` done; raster/vector handling stays deferred until
  upstream produces those bytes — honestly recorded as future work)
  Goal: figure → `PartialTrace` adapter producing typed candidates
  the `.2` generalizer + `MULTIMODAL-CONTRACT-FUSION` consume.
  Acceptance: `Extractor wired; produces PartialTrace records for the corpus's figures with EvidenceModality::Figure; downstream generalize+fuse path observed end-to-end on at least one fixture; scripts/run_ci.sh green.`

  **2026-05-20 corpus survey** (recorded in Decisions): The SpecForge
  repo carries **no raw PDFs or SVGs** — upstream PDF extraction is
  out-of-tree. The test corpus is pre-processed (typed JSON/markdown
  fixtures). `WAVEFORM.3`'s input is therefore the **typed figure-
  region record** the upstream PDF pipeline produces, not raw
  raster/SVG bytes — the `.1`-anticipated "VLM-structured? vector-SVG?
  both?" choice is moot; the honest split is on the typed contract
  with upstream, not on the image format.

  - ID: `R16-WAVEFORM-CONTRACT-MINING.3.1`
    Status: `done` (`2026-05-20`)
    Goal: define the typed `FigureRegion` input contract (what an
    upstream PDF extractor must produce to feed `WAVEFORM.3.2`).
    Acceptance: `Typed FigureRegion input contract designed + recorded in this tree + mirrored in the book; docs-only; mdBook green.`
    Verification: `passed` — investigation step: surveyed
      `crates/specforge/src/ir/source.rs` and found
      `VisualAsset { asset_id, asset_kind, page_id, image_path,
      caption_text, diagram_kind, … }` — the existing upstream
      record for PDF figures (asset_kind="figure",
      diagram_kind∈Timing/StateDiagram/…). `FigureRegion` is the
      typed *extension* an upstream PDF pipeline produces when it
      classifies a `VisualAsset` as a timing diagram and recovers
      lane/annotation structure:

      ```rust
      pub struct FigureRegion {
          pub visual_asset_id: String,             // ref to upstream VisualAsset
          pub bbox: Option<BoundingBox>,           // figure bbox in PDF user space
          pub annotations: Vec<FigureAnnotation>,  // "≥ 2 cycles", lane labels, …
          pub waveform_lanes: Vec<FigureLane>,     // recovered lanes when upstream classifies
          pub raw_image_path: Option<PathBuf>,     // rarely needed; typed surface is primary
      }
      pub struct BoundingBox { pub x: f64, pub y: f64, pub w: f64, pub h: f64 }
      pub struct FigureAnnotation {
          pub text: String,
          pub bbox: Option<BoundingBox>,
          pub kind: AnnotationKind,                // Delay / Label / Value / Unknown
      }
      pub enum AnnotationKind { Delay, Label, Value, Unknown }
      pub struct FigureLane {
          pub signal_name: String,
          pub samples: Vec<LaneSample>,
      }
      pub struct LaneSample { pub at_tick: u32, pub level: LaneLevel }
      pub enum LaneLevel { High, Low, Unknown, Bus(String) }
      ```

      The adapter (`.3.2`) consumes `FigureRegion`s and produces
      `PartialTrace` (the typed `ir/waveform.rs` intermediate) via:
      - each `FigureLane` → `LaneEdge`s (level transitions) +
        `ValueSpan`s (held-value runs);
      - annotations with `kind == Delay` + parseable bounds (e.g.
        "≥ 2 cycles" → `min=Some(2), max=None`) → `RelativeDelay`;
      - annotations with `kind == Value` → `ValueSpan` augmentation;
      - `Unknown`-kind annotations lower `PartialTrace.confidence`
        (honest dormancy — never silently treated as licensed).

      Additive design: the upstream `VisualAsset` is unchanged;
      `FigureRegion` is a new typed record produced when the upstream
      pipeline classifies a VisualAsset as a timing diagram AND
      recovers structure. When upstream produces zero
      `FigureRegion`s (the corpus today), `.3.2` is a no-op (zero
      artifact churn). Mirrored in
      `docs/book/src/direction/temporal-intent-capture.md` under the
      `R16-WAVEFORM-CONTRACT-MINING` section. `mdbook build` green.
    Commit: `see Commit Log`

  - ID: `R16-WAVEFORM-CONTRACT-MINING.3.2`
    Status: `done` (`2026-05-20`)
    Goal: implement the `FigureRegion → PartialTrace` adapter.
    Acceptance: `Typed adapter + tests; downstream generalize+verify+fuse path observed end-to-end on at least one synthetic FigureRegion fixture; scripts/run_ci.sh green.`
    Verification: `passed` — added
      `crates/specforge/src/ir/figure_region.rs` (typed
      `FigureRegion` / `FigureLane` / `LaneSample` / `LaneLevel` /
      `FigureAnnotation` enum (Delay/Value/Label/Unknown) /
      `BoundingBox`); registered in `ir/mod.rs`. `.3.1` design
      refinement: `FigureAnnotation` is an enum (cleaner than a flat
      record with many `Option<…>`). Added
      `figure_region_to_partial_trace(&FigureRegion) -> PartialTrace`
      to `ir/waveform.rs`: each `FigureLane` ⇒ `LaneEdge`s on level
      transitions + `ValueSpan`s on contiguous identical-value runs
      (`Unknown` samples break runs but don't record edges, honest
      dormancy); `FigureAnnotation::Delay` ⇒ `RelativeDelay`;
      `FigureAnnotation::Value` ⇒ extra `ValueSpan`;
      `FigureAnnotation::Label` is informational only;
      `FigureAnnotation::Unknown` ⇒ confidence demoted one rank
      (High→Medium, Medium→Low, Low→Low) — honest dormancy, never
      silently licensed. 6 new unit tests: lane → edges + spans;
      Delay → RelativeDelay; Value → ValueSpan; Unknown demotes
      confidence; Label informational only; **end-to-end smoke**:
      FigureRegion → PartialTrace → generalize_partial_trace →
      verify_contract_against_trace = Pass on a synthetic Value-
      annotation Stable contract (proves the `.3.2` acceptance
      "downstream generalize+verify path observed end-to-end on at
      least one synthetic FigureRegion fixture"). No producer
      wiring; `SemanticIr` / `IntentIr` schemas unchanged ⇒ ZERO
      artifact churn. Full `scripts/run_ci.sh` green.
    Commit: `see Commit Log`

- ID: `R16-WAVEFORM-CONTRACT-MINING.4`
  Status: `done` (`2026-05-20`)
  Goal: `validate waveform:` block + corpus baseline + close tree
  + book + ROADMAP R16.
  Acceptance: `FigureConformance Pass rate measured & locked; negative-fixture suite established; tree marked done; ROADMAP R16 closed for WAVEFORM-CONTRACT-MINING; mdBook "Status — delivered" subsection per BOOK-METHOD-DOC; scripts/run_ci.sh green.`
  Verification: `passed` — `specforge validate` now prints
    `waveform: figure_contracts=N verifier_fail_residuals=M` in both
    the SemanticIR and IntentIR count blocks
    (`crates/specforge/src/commands/validate.rs`, additive lines via
    `replace_all`). Counts derived from `actor_contracts` (IR
    self-describing): `figure_contracts` = contracts with
    `EvidenceModality::Figure` provenance;
    `verifier_fail_residuals` = `Residual` whose `reason` starts
    `"verifier disagreement: "`. Structured-metric / JSON shape
    intentionally not touched (bounded — KG-ONTOLOGY.4 /
    FIDELITY.4 / FUSION.4 / CVE.6 precedents). Corpus baseline:
    `waveform: figure_contracts=0 verifier_fail_residuals=0` —
    honest dormancy (no upstream producer exists; the typed
    end-to-end pathway is unit-tested with synthetic FigureRegions
    in `.3.2`). **Negative-fixture coverage**: the `.2` /
    `.3.2` unit suite already covers honest-dormancy paths
    end-to-end — `bare_edge_generalizes_to_observe_residual`
    (junk lane → `Observe`+`Residual`, not contract);
    `relative_delay_missing_bounds_lowers_residual` (under-determined
    delay → `Residual`, never fabricated);
    `verifier_not_evaluated_on_unsupported_obligation` (gates honest
    `NotEvaluated`, never silent `Pass`);
    `adapter_demotes_confidence_on_any_unknown_annotation`
    (Unknown annotations lower trace confidence). These prove the
    `.4` negative-fixture acceptance ("junk waveforms do not mint
    contracts; verifier-fail → Residual, not silent fabrication") at
    the synthetic-input level; corpus-level proof activates the
    moment upstream produces `FigureRegion`s. Full
    `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-20`.** All four leaves done (`.3` via
honest-split `.3.1` + `.3.2`). ROADMAP R16 entry for
`R16-WAVEFORM-CONTRACT-MINING` marked done. Future
follow-up — raster / vector handling — stays deferred until the
upstream PDF pipeline produces those bytes; that work is a new
tree, not a re-opened leaf of this one.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-WAVEFORM-CONTRACT-MINING.1` | `done` | Schema + generalizer + verifier design fixed; book mirror added |
| 2 | `R16-WAVEFORM-CONTRACT-MINING.2` | `done` | Typed `waveform` module + 4-rule generalizer + round-trip verifier + 7 tests; zero artifact churn |
| 3 | `R16-WAVEFORM-CONTRACT-MINING.3` | `done` | Both sub-leaves done: `.3.1` typed FigureRegion design + `.3.2` typed module + adapter + tests; raster/vector handling deferred to a future leaf (honest) |
| 3.1 | `R16-WAVEFORM-CONTRACT-MINING.3.1` | `done` | Typed FigureRegion contract designed (extends upstream VisualAsset; bbox + typed annotations + waveform_lanes + optional raw_image_path); book mirror added |
| 3.2 | `R16-WAVEFORM-CONTRACT-MINING.3.2` | `done` | Typed `ir/figure_region.rs` + `figure_region_to_partial_trace` adapter in `ir/waveform.rs` + 6 tests incl. end-to-end smoke (FigureRegion → PartialTrace → generalize → verify = Pass); zero artifact churn |
| 4 | `R16-WAVEFORM-CONTRACT-MINING.4` | `done` | `validate waveform: figure_contracts=0 verifier_fail_residuals=0` block delivered (corpus baseline; honest dormancy); negative-fixture coverage proven via existing `.2`/`.3.2` synthetic-input tests; tree closed |

## Design (`.1` output, 2026-05-20)

This is the program thesis's **crux**: timing diagrams *are* the timed
automaton, drawn — and extracting them accurately + reliably is the
hardest, highest-value problem. Design parallels prior R16 `.1`s
(typed layer / no new stage / additive producer / honesty doctrine
enforcement) but the actual figure→trace extractor in `.3` is
qualitatively different from prior R16 leaves and is expected to
honest-split.

### Placement — typed layer, no new stage

A new `crates/specforge/src/ir/waveform.rs` defines the typed
intermediate (`PartialTrace`, `LaneEdge`, `ValueSpan`,
`RelativeDelay`, `CausalArrow`, `EdgeKind`). `.2` ships the typed
types + `generalize_partial_trace` + `verify_contract_against_trace`.
`.3` ships the figure→PartialTrace extractor — that step bridges the
already-existing PDF pipeline (which extracts figure regions) to the
typed intermediate; **the typed intermediate is the contract between
the extractor and the generalizer**, so each side is independently
unit-testable.

### Typed intermediate

```rust
pub enum EdgeKind { Rising, Falling, Stable, Unknown }
pub struct LaneEdge { pub signal: String, pub at_tick: u32, pub kind: EdgeKind }
pub struct ValueSpan {
    pub signal: String, pub value: String,
    pub from_tick: u32, pub to_tick: u32,
}
pub struct RelativeDelay {
    pub from_signal: String, pub to_signal: String,
    pub min_cycles: Option<u32>, pub max_cycles: Option<u32>,
    pub annotation_text: String,  // e.g. "≥ 2 cycles"
}
pub struct CausalArrow {
    pub from_signal: String, pub from_tick: u32,
    pub to_signal: String,   pub to_tick: u32,
}
pub struct PartialTrace {
    pub figure_id: String,
    pub signals: Vec<String>,
    pub edges: Vec<LaneEdge>,
    pub spans: Vec<ValueSpan>,
    pub delays: Vec<RelativeDelay>,
    pub causal: Vec<CausalArrow>,
    pub ticks: u32,
    pub confidence: AutomationConfidence,
}
```

This intermediate is **what an extractor produces** and **what the
generalizer consumes**. Both sides are independently unit-testable
against synthetic PartialTraces.

### Generalization rules (`generalize_partial_trace`)

Conservative, bounded; under-determined patterns lower as `Observe` /
`Residual`. Rules for `.2`:

- `RelativeDelay { from, to, min, max, … }` with `max ≥ 1` and `max
  ≥ min` ⇒ `Eventually { target: Edge { signal: to }, window:
  Within { min, max } }`. (`max == 0` ⇒ Residual: `.isf` rejects
  `(within 0)`.)
- `ValueSpan { signal, value, from_tick, to_tick }` with `to_tick -
  from_tick ≥ 1` ⇒ `Stable { signal, during: Within { min: None, max:
  (to_tick - from_tick) } }`. (Adjacent samples only is a `Drive`
  candidate, not Stable.)
- `CausalArrow` with `to_tick == from_tick + 1` ⇒ `Eventually` with
  `Within { min: 0, max: 1 }` next-cycle. Larger gaps ⇒ wider Within.
- A single `LaneEdge { signal, kind: Rising|Falling }` alone ⇒
  `Observe { signal }` + `Residual{reason="bare edge — no window
  licensed"}` (honesty doctrine; never fabricate a window).
- Confidence: `ActorContract.automation_confidence = min(trace.confidence,
  Medium)` (a single-figure mining never promotes to High without
  cross-modal corroboration; `MULTIMODAL-CONTRACT-FUSION.3` can
  consolidate up via its merge — but it takes the min, so cross-modal
  agreement alone doesn't lift confidence; the prose-side oracle in
  the future can mark a contract `High`).

### Verifier (`verify_contract_against_trace`)

The honest oracle: a contract claimed *from* a trace must
*satisfy* that trace. Reuses
`R16-CAPTURE-FIDELITY-GATES.2`'s `evaluate_figure_trace` primitive
(constructed by lifting a `PartialTrace.spans`/`edges` to a
`FigureTrace`). If the verifier returns `Fail` or `NotEvaluated`,
the generated contract is demoted to `Residual{reason="verifier
disagreement: <…>"}` — fabrication is structurally prevented.

### Extractor strategy (`.3` — honest-split warning)

The figure→`PartialTrace` extractor is the qualitatively-different
leaf. Plausible approaches:

1. **VLM-structured prompting** with strict JSON output, post-verified
   by a heuristic shape-check; outputs become `PartialTrace`s tagged
   with figure-region provenance.
2. **Vector-SVG path parsing** when the PDF figure is a vector
   graphic (cheap, exact); fallback to (1) for raster figures.
3. **Both**, with vector-first for accuracy and VLM-fallback for
   raster-only inputs.

The right approach depends on the corpus's figure-format mix and
empirical accuracy data — both undecidable from `.1`. At `.3`
promotion, expect an **honest split (rule 5)** into sub-leaves once
the concrete approach is chosen.

### Cross-check with prose

This tree produces `PartialTrace` + candidate `ActorContract`s with
`EvidenceModality::Figure`. Prose contracts have
`EvidenceModality::Prose`. The same-`FusionKey` cluster goes through
`R16-MULTIMODAL-CONTRACT-FUSION.3` (already closed): agreement ⇒
fused Mixed contract; disagreement ⇒ `Residual{reason="disagreement:
…"}` (the honesty doctrine, already enforced upstream). This tree
does NOT re-implement cross-check — FUSION owns it.

### Honest dormancy / Non-Goals

- Not pixel-perfect waveform OCR for its own sake; the deliverable is
  contracts, with PartialTrace as evidence/provenance.
- No fabricated generalization beyond what the trace + annotations
  license; under-determined ⇒ `Observe`/`Residual` (honesty
  doctrine).
- `.2` ships typed intermediate + generalizer + verifier; **no PDF
  parsing yet**. Corpus baseline through `.2` stays unchanged
  (`FigureConformance` `NotEvaluated`, `groups_merged=0`,
  `disagreements=0`). `.3` is where the dormancy ends.

### Report shape (`.4`, `validate`)

Additive lines in the SemanticIR + IntentIR count blocks (when the
corpus actually has figure-derived contracts):

```
  waveform: partial_traces=… figure_contracts=… verifier_fail_residuals=…
```

Structured-metric / JSON shape intentionally not touched (bounded;
KG-ONTOLOGY.4 / FIDELITY.4 / FUSION.4 precedents).

## Decisions

- `2026-05-19`: Explicitly the program crux per user direction
  (2026-05-19): "the most difficult thing is extracting accurate and
  reliable temporal information from prose and timing diagrams."
  Highest ceiling, research-grade; created `proposed`, sequenced
  after the target shape + objective metric exist.
- `2026-05-20`: **Promoted `proposed → active`** by
  `R16-INTENT-CAPTURE.2` after DAG predecessors closed
  (`R16-CONTRACT-IR` ✓, `R16-CAPTURE-FIDELITY-GATES` ✓); upstream
  sibling `R16-MULTIMODAL-CONTRACT-FUSION` (#3) also closed — its
  producer is ready to consume figure-derived contracts. `.1`
  design fixed (docs-only): typed intermediate
  (`PartialTrace`/`LaneEdge`/`ValueSpan`/`RelativeDelay`/`CausalArrow`/
  `EdgeKind`); bounded generalization rules (Eventually from
  RelativeDelay; Stable from ValueSpan; next-cycle Eventually from
  CausalArrow; bare LaneEdge ⇒ Observe+Residual — never fabricate);
  round-trip verifier reusing `evaluate_figure_trace` (verifier-fail
  ⇒ Residual); cross-check delegated to `R16-MULTIMODAL-CONTRACT-FUSION`
  (already closed); `.3` extractor explicitly flagged for **honest
  split (rule 5)** when concrete approach is chosen. Book mirror per
  BOOK-METHOD-DOC.

## Blockers

- None. Active; frontier `R16-WAVEFORM-CONTRACT-MINING.2`.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-20` | `R16-WAVEFORM-CONTRACT-MINING.1` | intermediate / generalizer / verifier / extractor strategy / cross-check delegation recorded; book mirror per BOOK-METHOD-DOC; mdBook builds | `passed` (docs-only) |
| `2026-05-20` | `R16-WAVEFORM-CONTRACT-MINING.2` | typed `waveform` module (`EdgeKind`/`LaneEdge`/`ValueSpan`/`RelativeDelay`/`CausalArrow`/`PartialTrace` + `generalize_partial_trace` + `verify_contract_against_trace` + `partial_trace_to_figure_trace` lift) + 7 unit tests; SemanticIr/IntentIr unchanged; full `scripts/run_ci.sh` | `passed` (zero artifact churn) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-WAVEFORM-CONTRACT-MINING.1` | `R16-WAVEFORM-CONTRACT-MINING.1 — crux design (promote #4)` (`13ac7069`) | docs-only; book mirror; also the `R16-INTENT-CAPTURE.2` #4 promotion |
| `R16-WAVEFORM-CONTRACT-MINING.2` | `R16-WAVEFORM-CONTRACT-MINING.2 — typed waveform module + generalizer + verifier + tests` | first WAVEFORM code; zero artifact churn; extractor (`.3`) still pending |

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (#1) and `R16-CAPTURE-FIDELITY-GATES`
  (#5/order-3, its objective function). Feeds
  `R16-MULTIMODAL-CONTRACT-FUSION` (#3, already closed and ready to
  consume figure-derived contracts). Pairs with
  `R16-CONSTRAINED-VERIFIED-EXTRACTION` (#6) on prose.

## Changelog

- `2026-05-19`: Created `proposed` as program point #4 (crux), ordered
  5th.
- `2026-05-20`: **Corpus survey** finding for the `.3` honest split:
  `find … *.pdf *.svg` over the SpecForge tree returns nothing —
  the test corpus is **pre-processed** (typed JSON/markdown fixtures
  under `crates/specforge/test_data/`; `corpus_kb/` is markdown
  only). Upstream PDF extraction (raster/SVG → typed records) is
  **out-of-tree**, so the `.1`-anticipated "VLM-structured? vector-
  SVG? both?" choice is moot: the actual decision is on the typed
  contract with whatever upstream record the PDF pipeline produces.
  **Honest-split `.3`** (rule 5) into `.3.1` (typed `FigureRegion`
  input contract design) + `.3.2` (adapter implementation). The
  raster/vector handling stays deferred until upstream actually
  produces those bytes — the typed adapter pathway can land before
  that, processing whatever textual annotations the upstream record
  carries (RelativeDelay / ValueSpan recovery from annotation text).
- `2026-05-20`: Promoted to `active` (DAG predecessors closed);
  `.1` design fixed + book mirror; concrete `.1`–`.4` leaves defined
  (with `.3` flagged for honest split). Frontier → `.2` (implement
  typed `waveform` module + generalizer + verifier).
- `2026-05-20`: `.2` done — `ir/waveform.rs` typed module
  (`EdgeKind`/`LaneEdge`/`ValueSpan`/`RelativeDelay`/`CausalArrow`/
  `PartialTrace`) + `generalize_partial_trace` (4 rules per `.1`
  design, with under-determined ⇒ `Observe`+`Residual` honesty) +
  `verify_contract_against_trace` (round-trip oracle via
  `partial_trace_to_figure_trace` lift to `FigureTrace` + reuse of
  `R16-CAPTURE-FIDELITY-GATES.2`'s `evaluate_figure_trace`); 7 unit
  tests covering each generalization path + verifier Pass /
  NotEvaluated semantics. SemanticIr / IntentIr schemas unchanged ⇒
  ZERO artifact churn. Full CI green. Frontier → `.3` (figure →
  PartialTrace extractor — expected honest-split when concrete
  approach is decided).
