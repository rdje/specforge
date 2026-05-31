# INTENT-COMPLETENESS-RESEARCH: theory + design for detecting & bounding intent-capture misses

## Metadata

- Tree ID: `INTENT-COMPLETENESS-RESEARCH`
- Status: `active`
- Roadmap lane: `R15e` (KG-quality evaluation) — cross-cutting into R15c/R15d/R16
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

A **research-first** program (user directive `2026-05-31`: "accuracy is of
utmost importance… really think it through before coding"): establish the
theoretical foundation and concrete, implementable instruments for **detecting,
characterizing, and bounding misses** in design-intent extraction from digital
chip PDFs — turning "accuracy" from a hope into a *measured, gap-surfaced
property*. Coding is downstream and mechanical; this tree's product is the
*right thing to code* + the evidence it is right.

The durable artifact is [`docs/research/intent-capture-completeness.md`](../research/intent-capture-completeness.md)
(the A-to-Z framework). The central reframe it establishes:

> A miss cannot be found by inspecting the output (the miss is what is *not*
> there). It **can** be found on the input side: every intent-bearing source
> region must produce a fact or be tagged non-intent — anything else is a
> candidate miss; plus domain **closure invariants** and a **capture–recapture**
> recall estimator give ground-truth-free miss detection and recall bounds.

## Non-Goals

- NOT writing extraction/detection code here — this tree produces theory, the
  closed ontology, the miss taxonomy, the instrument designs, and a prioritized
  implementation backlog. Each instrument → its own implementation task-tree
  (no code change without an owning tree).
- NOT a one-off web search — first-principles thinking grounded in (and verified
  against) the relevant academic literature.
- NOT inventing accuracy numbers — define the denominator + the recall
  instrument so any number is defensible.

## Acceptance Criteria

- The framework artifact exists, is reviewed, and covers: the precision/recall
  asymmetry; the closed intent ontology (recall denominator); the source/region
  model; the operational definition of a miss; the miss taxonomy with a detector
  per kind; domain closure invariants; inter-stage conservation; recall
  estimation without ground truth; the unifying completeness report; academic
  grounding; and a prioritized instrument backlog.
- Academic grounding is verified (real references) for the load-bearing ideas
  (capture–recapture recall estimation, local-closed-world KG completeness,
  competency questions, requirements traceability, table/figure understanding,
  spec/property mining).
- A prioritized backlog of implementation trees with dependencies + expected
  accuracy lift is produced and registered, so coding can begin on the
  highest-value instrument with confidence it targets a real miss class.

## Task Tree

- ID: `INTENT-COMPLETENESS-RESEARCH`
  Status: `active`
  Goal: theory + design + prioritized backlog for miss detection / recall bounding
  Children: `.1`, `.2`, `.3`, `.4`, `.5`, `.6`, `.7`

- ID: `INTENT-COMPLETENESS-RESEARCH.1`
  Status: `done`
  Goal: >
    Frame the problem from first principles; establish the precision/recall
    asymmetry and the "miss = unexplained intent-bearing region OR violated
    completeness invariant" reframe; create the framework artifact + this tree;
    register. Docs/research only.
  Acceptance: framework artifact created with all §0–§13 sections; tree registered.
  Verification: >
    passed (`2026-05-31`) — wrote `docs/research/intent-capture-completeness.md`
    (problem statement; precision↔recall asymmetry + the four sources of a
    completeness oracle; the source-side reframe; closed intent ontology as the
    recall denominator; region accounting; operational miss definition; miss
    taxonomy with a detector per kind; domain closure invariants; inter-stage
    conservation; recall estimation without ground truth incl. capture–recapture;
    safeguards already in place; the unifying completeness report; academic
    grounding; prioritized instrument backlog; honest accuracy framing). Tree
    created + registered. Docs-only (CI invariant).
  Commit: `see Commit Log`

- ID: `INTENT-COMPLETENESS-RESEARCH.2`
  Status: `done`
  Goal: >
    Formalize the **closed design-intent ontology** (the recall denominator) +
    the intent-category × source-modality × extractor **coverage matrix**;
    identify empty cells (systematic blind spots). Cross-check against the
    existing IR types so the ontology reflects what the code already models.
  Acceptance: ontology + coverage matrix documented; empty cells listed as candidate blind-spot trees.
  Verification: >
    passed (`2026-05-31`) — wrote `docs/research/intent-ontology-coverage.md`:
    the closed ontology (~20 categories, grouped), the coverage matrix (category
    → stage:field → producer → modality, code-grounded via an Explore audit of
    `ir/*` + `commands/*`), and the gap analysis. **Load-bearing gap claims
    verified against the code** (residual-honesty / AUDIT-DOC-RECONCILE): two
    audit claims FAILED verification and were corrected — `protocol_graph` is NOT
    dead (`semantic.rs:371` populates it from `actor_contracts` via
    `project_handshake_pairs`), and reset synchronicity IS modeled
    (`SystemResetKind::{Synchronous,Asynchronous}`, `semantic.rs:972`).
    **Confirmed (A) ontology gaps:** clock domains (no `ClockDomain` type) and
    enumerations-as-a-queryable-record (only synthetic statements + the recovered
    ISF symbol surface). **(B) single-modality blind spots** recorded as
    hypotheses the `.3`/`.4` detectors will confirm empirically (not hand-claimed
    as misses). Docs/research only (CI invariant).
  Commit: `see Commit Log`

- ID: `INTENT-COMPLETENESS-RESEARCH.3`
  Status: `done`
  Goal: >
    Design the **region-accounting / backward-traceability** instrument (§3): how
    to partition the document, classify regions (intent / non-intent / deferred),
    link facts backward, and emit the "unexplained intent-bearing region"
    residual. Specify the data model + how it rides the existing provenance.
  Acceptance: instrument design + data model spec; ready to become an implementation tree.
  Verification: >
    passed (`2026-05-31`) — wrote `docs/research/region-accounting-design.md`:
    region enumeration from SourceIR units (prose/table/cell/figure/caption/
    section + id scheme); the intent/non-intent/deferred classifier (high-recall,
    explainable, reusing existing `StatementClass`/`table_kind`/`DiagramKind`
    signals); the backward-traceability index built by **inverting the existing
    forward provenance** (evidence_span_ids / supporting_statement_ids /
    table_id …), flagging a provenance-completeness precondition where facts lack
    a resolvable region id; the `UnexplainedRegionResidual` + `RegionAccounting
    Report` shapes feeding the `CompletenessReport`; computed at EvidenceIR; and
    how it auto-confirms the `.2` (B) blind-spot hypotheses for free. Docs/
    research only.
  Commit: `see Commit Log`

- ID: `INTENT-COMPLETENESS-RESEARCH.4`
  Status: `done`
  Goal: >
    Enumerate the **domain closure invariants** (§6) + the full **miss taxonomy**
    (§5) with a concrete detector per kind (symbol closure, register tiling,
    handshake pairing, encoding coverage, direction closure, cross-modal,
    pipeline conservation). Each detector spec'd to be exact + ground-truth-free.
  Acceptance: invariant/detector catalog, each with inputs/outputs/residual shape.
  Verification: >
    passed (`2026-05-31`) — wrote `docs/research/miss-detectors-catalog.md`: 6
    detector families (A region-level incl. mutation/sensitivity coverage; B
    field-level schema/star-pattern; C structural closure — symbol/tiling/
    handshake/encoding/producer/infra; D cross-modal + two-source fusion; E
    pipeline conservation; F systematic-blind-spot / coverage-matrix empty cell),
    each with inputs / check / typed residual, and **tagged EXACT vs GATED** per
    the `.6` PCA correction. Added the HNEN canonicalization + provenance-
    completeness preconditions and the GoldMine mine-then-formally-filter step.
    Gave the cheapest-exact-first implementation ordering feeding `.7`. Docs/
    research only (CI invariant).
  Commit: `see Commit Log`

- ID: `INTENT-COMPLETENESS-RESEARCH.5`
  Status: `pending`
  Goal: >
    Design the **recall estimation** layer (§8): capture–recapture over the two
    independent extractors (assumptions, estimator, reporting); the
    competency-question battery; gold-fixture integration (R15e); the LLM
    completeness critic. Specify the unifying **CompletenessReport** (§10).
  Acceptance: recall-gauge + competency-battery + CompletenessReport designs.
  Verification: pending
  Commit: pending

- ID: `INTENT-COMPLETENESS-RESEARCH.6`
  Status: `done`
  Goal: >
    Literature survey — verify/expand §11 into real references and extract the
    specific borrowed result for each (KG completeness / LCWA / AMIE;
    capture–recapture in software inspection; competency questions; requirements
    traceability; table/figure understanding; spec/property mining; datasheet
    extraction). (May fan out as a research workflow if the user opts in.)
  Acceptance: verified reference list + the concrete result adopted from each.
  Verification: >
    passed (`2026-05-31`) — ran the `intent-completeness-lit-survey` **parallel
    research workflow** (user opted in; 9 agents / 8 disciplines + synthesis,
    ~283k tokens, web-grounded). Wrote `docs/research/literature-grounding.md`:
    DBLP/ACM/arXiv-verified references per instrument (Razniewski et al. survey
    2024; AMIE/PCA WWW'13 + Predicting-Completeness WSDM'17; Eick ICSE'92 /
    Briand-ElEmam TSE'00 / Petersson-Wohlin JSS'04 / Chao Biometrics'87;
    Grüninger-Fox IJCAI-95; Gotel-Finkelstein ICRE'94; DocLayNet/PubTables/GriTS;
    Daikon/GoldMine/Chockler-Kupferman-Vardi; D2S-FLOW + AssertionForge 2025). The
    survey CONFIRMS the core reframe as the field consensus and forces 5
    corrections + adds ~10 techniques — all folded into the framework: §8.1
    capture–recapture → Chao Mh + ≥3 heterogeneous extractors + bias-direction +
    systematic-blind-spot-invisibility; §6 PCA is heuristic-and-gated (not exact);
    §8.2 CQs bound schema not population; mutation/sensitivity coverage + star-
    pattern + two-source fusion + HNEN + the **STOP-OR-REINSPECT** bridge to the
    shipped convergence loop. Docs/research only.
  Commit: `see Commit Log`

- ID: `INTENT-COMPLETENESS-RESEARCH.7`
  Status: `done`
  Goal: >
    Synthesize → a **prioritized implementation backlog** (which instruments to
    build first, dependencies, expected accuracy lift), each scoped as a future
    owning task-tree. Decide the first implementation slice with the user.
  Acceptance: prioritized backlog registered; first implementation tree proposed.
  Verification: >
    passed (`2026-05-31`) — prioritized backlog (cheapest-exact-first, from the
    catalog `.4`): (1) structural closure invariants — symbol closure + register
    tiling [EXACT]; (2) region accounting [foundational]; (3) CompletenessReport
    rails (extend the shipped convergence report); (4) mutation/sensitivity
    coverage + schema/star-pattern; (5) cross-modal + two-source fusion +
    capture–recapture (Chao Mh, needs ≥3 heterogeneous extractors) + the
    STOP-OR-REINSPECT bridge into the convergence loop; (6) competency-question
    battery; (7) the (A) systematic-blind-spot gaps from `.2` (clock domains,
    first-class enum record). **User decision (`2026-05-31`): start with closure
    invariants** → opened `COMPLETENESS-CLOSURE-INVARIANTS` (register tiling first
    [EXACT], symbol closure second [GATED], surfaced as `validate` findings).
    Research `.5` (recall estimator + CompletenessReport design) remains the one
    open research leaf; it will be elaborated alongside the rails when detector
    coverage warrants. Docs-only.
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `INTENT-COMPLETENESS-RESEARCH.1` | `done` | framework + tree landed |
| 2 | `INTENT-COMPLETENESS-RESEARCH.2` | `done` | closed ontology + coverage matrix (`intent-ontology-coverage.md`); 2 gaps confirmed, 2 audit errors corrected |
| 3 | `INTENT-COMPLETENESS-RESEARCH.3` | `done` | region-accounting design (`region-accounting-design.md`) |
| 4 | `INTENT-COMPLETENESS-RESEARCH.4` | `done` | miss-detector catalog (`miss-detectors-catalog.md`), exact-vs-gated |
| 5 | `INTENT-COMPLETENESS-RESEARCH.5` | `pending` | recall estimation + CompletenessReport design — the one open research leaf |
| 6 | `INTENT-COMPLETENESS-RESEARCH.6` | `done` | literature survey (workflow); grounding + 5 corrections folded in |
| 7 | `INTENT-COMPLETENESS-RESEARCH.7` | `done` | backlog set; user chose closure invariants → `COMPLETENESS-CLOSURE-INVARIANTS` opened |

The order is a default, not a commitment — `.2`/`.3`/`.4`/`.6` are largely
independent and could be parallelized (incl. via a research workflow if opted
in). `.7` depends on the rest.

## Decisions

- `2026-05-31`: research-first, owned as a task-tree (user directive: think it
  through before coding). The durable product is the framework artifact + a
  prioritized backlog, NOT code.
- `2026-05-31`: the load-bearing theoretical move is **source-side miss
  detection** (unexplained intent-bearing region) + **domain closure invariants**
  + **capture–recapture** — all ground-truth-free — because recall has no output-
  side oracle.
- `2026-05-31`: completeness is delivered as **visible residuals + a calibrated
  estimate**, never as a claim of perfection — the residual-honesty doctrine
  scaled from precision to recall.

## Open Questions

- Denominator-first (§2 ontology) vs detector-first (§3 region accounting) as the
  opening implementation slice? (Region accounting yields visible value fastest;
  the ontology makes the recall number rigorous. Likely region accounting first,
  ontology in parallel.)
- Should `.6` literature survey run as a parallel research **workflow** (multiple
  agents, one discipline each)? Needs explicit user opt-in.
- How much of capture–recapture's independence assumption holds between the
  pattern and LLM extractors (they share inputs)? — quantify in `.5`.

## Blockers

- None (research; no live-provider or corpus dependency until implementation).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` | framework artifact `docs/research/intent-capture-completeness.md` created (§0–§13: precision/recall asymmetry, closed ontology denominator, region accounting, operational miss definition, taxonomy+detectors, closure invariants, inter-stage conservation, recall estimation, completeness report, academic grounding, instrument backlog); tree registered; docs-only | `passed` |
| `2026-05-31` | `.2` | `docs/research/intent-ontology-coverage.md` — closed ontology (~20 categories) + code-grounded coverage matrix (Explore audit of `ir/*`+`commands/*`); gap claims verified code-is-truth (2 corrected: protocol_graph populated, reset-kind modeled); confirmed gaps = clock domains + first-class enum record; (B) blind spots recorded as detector-confirmable hypotheses; docs-only | `passed` |
| `2026-05-31` | `.3` | `docs/research/region-accounting-design.md` — region enumeration + intent/non-intent/deferred classifier + backward-traceability (invert forward provenance) + `UnexplainedRegionResidual`/`RegionAccountingReport`; provenance-completeness precondition flagged; docs-only | `passed` |
| `2026-05-31` | `.4` | `docs/research/miss-detectors-catalog.md` — 6 detector families (region/field/structural/cross-modal/conservation/blind-spot), each inputs/check/residual + EXACT-vs-GATED tag; HNEN + provenance preconditions; mine-then-filter; cheapest-exact-first ordering; docs-only | `passed` |
| `2026-05-31` | `.6` | `docs/research/literature-grounding.md` — parallel research workflow (9 agents/8 disciplines); DBLP/ACM/arXiv-verified refs; confirms core reframe; 5 corrections + ~10 techniques folded into the framework (Chao Mh + heterogeneity; PCA heuristic-gated; CQ schema≠population; mutation coverage; STOP-OR-REINSPECT bridge); docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `INTENT-COMPLETENESS-RESEARCH.1` | `INTENT-COMPLETENESS-RESEARCH.1 — frame the intent-capture completeness problem + research framework` | docs/research + docs/tasks only |
| `INTENT-COMPLETENESS-RESEARCH.2` | `INTENT-COMPLETENESS-RESEARCH.2 — closed intent ontology + code-grounded coverage matrix` | docs/research only; 2 audit errors corrected via code verification |
| `INTENT-COMPLETENESS-RESEARCH.{3,4,6}` | `INTENT-COMPLETENESS-RESEARCH.{3,4,6} — region accounting + miss-detector catalog + verified literature grounding` | docs/research only; `.6` via parallel research workflow |

## Changelog

- `2026-05-31`: `.3`+`.4`+`.6` — region-accounting design + miss-detector catalog
  (exact-vs-gated) + the literature-grounding workflow (verified refs; 5
  corrections + ~10 techniques folded into the framework). Frontier → `.5`
  (recall estimation + CompletenessReport) then `.7` (prioritized backlog → first
  implementation tree, decided WITH the user).
- `2026-05-31`: `.2` — wrote `docs/research/intent-ontology-coverage.md` (closed
  ontology + coverage matrix, code-grounded). Verified gap claims against the
  code: corrected 2 audit errors (protocol_graph is populated; reset-kind is
  modeled); confirmed clock-domains + first-class-enum-record as real gaps; (B)
  single-modality blind spots recorded as hypotheses the detectors will confirm.
  Frontier → `.3` (region accounting).
- `2026-05-31`: Created — research-first program for detecting & bounding
  intent-capture misses (user directive: accuracy of utmost importance, think it
  through before coding). Wrote the A-to-Z framework
  (`docs/research/intent-capture-completeness.md`); the central reframe is
  source-side miss detection + closure invariants + capture–recapture (all
  ground-truth-free). Frontier → `.2` (closed ontology) / `.3` (region
  accounting).
