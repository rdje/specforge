# EXTRACTION-QUALITY-GAUGE: measure the extraction-quality gap — and CHI's is large

## Metadata

- Tree ID: `EXTRACTION-QUALITY-GAUGE`
- Status: `active` (gauge established + CHI measured; the fix leaves are open R-lane work)
- Roadmap lane: `R15e`/`R16` (extraction quality / production-readiness)
- Created: `2026-06-06`
- Parent context: `TABLE-GRITS-CONFORMAL.4` ran the NLI-oracle conformal on CHI and it would not
  calibrate. The diagnosis turned out to be a real extraction-quality gap, not a metric problem.

## The gauge (the reusable capability — "something to catch the size of the gap")

The NLI-oracle pass (`nli-verify` over an EvidenceIR) yields, per document, the **fraction of
extracted constraints the source does NOT entail** — a cheap, automatic **extraction-quality gauge**
(a production-readiness signal). It is a *noisy* oracle (some not-entailed are NLI false-negatives on
complex claims), so treat it as an estimate — but it is **hand-validated** (below) and discriminates
sharply across documents:

| doc | constraints | NLI not-entailed | reading |
|---|---|---|---|
| APB (simple peripheral bus) | 14 | ~29% | tolerable |
| **CHI** (cache-coherency interconnect) | 162 | **~83%** | **far from production** |

## CHI measurement — ~80% of constraints are erroneous (hand-validated)

A random sample of 18 not-entailed CHI constraints was **read against source** (the rigorous check,
not the aggregate). **18/18 were genuine errors** — the NLI signal is largely real here, not noise.
Heuristic categorization of all 162 plus the hand-read taxonomy:

| error type | what it is | examples |
|---|---|---|
| **Spurious subject** (largest) | a non-signal extracted as a signal | `B13` (←"Table B13.25"), `MTE` (a feature), `LICENSEE`/`AMBA` (license/trademark boilerplate), `CMO`/`PCMO` (transaction *types*) |
| **Condition-drop** | real signal+obligation, but the conditional/temporal scope is lost | `TXSACTIVE must be asserted` ← "…*after receiving a snoop* / *until the last flit*" (and **not** recovered in `conditional_rules`: 1 of ~6 survives) |
| **Permission→obligation** | a permissive statement turned into a `must` | `EWA must_be_value` ← "permitted, but **not required**" |
| **Relational mis-extraction** | a relation/equality flattened to a value constraint | `DBID must_be_value` ← "TxnID is set to the same value as DBID" |
| **Duplication** (cross-cutting) | same subject+kind re-emitted from each conditional sentence | **30%** of the 162 are duplicates |

Mechanically-detectable floor (regex): spurious-ref **5%** + condition-drop **15%** = 20%; the
hand-read shows the true rate is ~**80%** (only ~17% pass the NLI, and even those are merely
*plausible*). **SpecForge is far from production-ready on hard, dense, conditional specs.**

## Root cause #1 — entity discrimination is broken (the foundational one)

> "SpecForge needs seriously to learn to discriminate signals vs actors vs everything else. If it
> can't find the objects/entities that interact in a chip-spec, we won't go far." — owner, and the
> data agrees emphatically.

The KG is `(actor —verb→ signal)`; it is **only as good as the typing of its nodes**, and on CHI the
nodes are mis-typed garbage:

- **Actors are garbage.** 73 "actors" extracted include `ADDRESS OF`, `ANY`, `APPLICATION`, `ARE
  MEMORY`, `ATTRIBUTE VALUES`, `BY ANY`, `CACHE STATE` — random noun phrases, not components (the real
  actors are `Requester`, `Completer`, `RN-F`, `Home Node`, …).
- **Signal typing is noisy both directions.** `B13` and `CMO` are wrongly *hinted* as signals; the
  *real* signal `TXSACTIVE` is **absent** from the declared (24) + hinted set entirely.
- **64% of the 162 constraint subjects** are in **no** declared/hinted signal set — the extractor is
  largely guessing, and the catalogs that should ground it are themselves unreliable.

So the spurious-subject errors are a symptom of a foundational gap: SpecForge cannot reliably
**classify a token's entity type** (signal | actor | transaction | feature | state | structural-ref |
boilerplate). The ingredients exist (`table_signal_declaration_provenance`, `signal_semantic_hints`,
`actor_signal_relations`) but are noisy and **not enforced** at extraction. **Fix this first — every
downstream relation, constraint, and intent inherits the node typing.**

## Root cause #2 — the constraint unit doesn't match the unit of meaning

CHI is a deeply **conditional / temporal / stateful** protocol ("when X in state S, Y must Z until
W"). SpecForge's unit of extraction — the flat, mostly-**unconditional** `SignalConstraint` —
structurally mismatch that. Adequate on APB (simple, mostly unconditional); it breaks on CHI. Prior
"intent-capture" validation leaned on simpler specs; CHI exposes the real limit.

## Fix backlog (open R-lane work)

- `.1` **Entity discrimination** (THE foundational fix): a typed classifier for every candidate token
  — `signal | actor | transaction | feature | state | structural-ref | boilerplate` — grounded in
  **derived** signals (ADR 0006), not hardcoded names:
  - **declaration site** — signals are declared in pin/port/signal tables (`table_signal_declaration_provenance`);
    actors in a components/glossary section; the *region* a token is defined in types it;
  - **linguistic role** — actors are *subjects* of normative verbs ("the Completer must drive…");
    signals are *objects* ("…drive PREADY") — reuse the actor-signal-relation grammar;
  - **structural cues** — "token after *Table*/*Figure*" → ref; front-matter/legal region → boilerplate;
  - **cross-reference** — a real signal recurs in the signal table *and* prose; a ref appears only in "see …".
  Then **enforce** it: a constraint/relation subject must be a token typed `signal` (and clean the
  noisy catalogs — `TXSACTIVE` must be *in*, `B13`/`CMO` *out*; the garbage actors must be rejected).
- `.2` **Conditional/temporal constraints as first-class**: stop flattening — carry each constraint's
  condition/temporal scope (the extractor discards it *before* the temporal layer can capture it).
  This is the big one.
- `.3` **Permission-vs-obligation gate** + **relational-vs-value disambiguation** (reuse the NLI
  condition-vs-obligation machinery).
- `.4` **Dedup** constraints by (subject, kind, condition).
- `.0` **Wire the gauge** into `converge`/CI as a per-doc quality report (the standing measurement).

## `.1` prototype — built + measured (the architecture is validated)

`ir/entity_typing.rs` + `entity-type` command embody the "human-SpecForge in Rust" structure: Rust
**gathers** each token's grounding evidence (`gather_entity_evidence`), the **LLM judges**
(`propose_entity_type_llm`), Rust **grounds** the judgment (`classify_entity` — the document overrides
the model where authoritative, defers where silent), and the **enforcement gate**
(`is_valid_signal_subject`) keeps only `Signal`-typed subjects. Document-grounded → works on any spec.
Unit-tested (injected `propose`, no provider).

**Measured on real CHI (66 distinct subjects):** typed **27/66 as non-signal** and filtered them —
correctly: `B13/B14/B16`→structural_ref, `AMBA/LICENSEE`→boilerplate, `CMO/PCMO/DVMO`→transaction,
`MTE/MEC/DVM`→feature, and crucially `TXSACTIVE` (the real signal the *catalogs missed*) **recovered**
as signal by the LLM. So **we *can* harness the LLM for discrimination — the architecture holds.**

**But discrimination alone is not sufficient.** Removing the 27 → drops **49/162 (30%)** constraints,
**84% of which were NLI-not-entailed** (genuine garbage). Yet the gauge moves only **17% → 18%**,
because the *kept* signal constraints are still ~82% not-entailed — they are real signals whose
**conditions were dropped** (root cause #2). The path to a working SpecForge is to apply the SAME
harness pattern to *each* error mode (`.2`–`.4`), not entity typing alone. One front proven; several
remain.

## `.2` result + the strategic pivot (`2026-06-06`)

`ir/condition_extract.rs` + `extract-conditions` (Rust gathers → LLM judges the condition clause →
Rust grounds it: a condition is kept only if ≥60% of its content words appear in the source — no
hallucinated conditions; +4 tests). **Measured on CHI:** 25 candidates, **22 conditions captured +
grounded (88%)**, and they flip correctly — `TXSACTIVE` constraints became NLI-entailed once their
condition (`"before or in the same cycle in which the first flit…"`) was restored.

**Trajectory (CHI NLI-entailed):** `17%` (neither) → `22%` (`.2`) → `.1`-only `18%` → **`.1`+`.2` `22%`**.
Each front works; each moves the gauge a few points; they're complementary (`.2` fixes condition-drop,
`.1` removes garbage nodes — `B13` got a condition but stays not-entailed because it's a spurious
*subject*).

**Strategic read:** patching the Pattern extractor error-mode-by-error-mode converges slowly — the
base is ~80% wrong. The proven components (`.1` typed-grounded subjects, `.2` grounded conditions) are
the pieces of a better answer: **compose them into an LLM-PRIMARY, Rust-grounded constraint
*extractor*** that emits `(typed-signal subject, obligation, grounded condition)` in one pass, rather
than bolting fixes onto a flat pattern extractor. That is the real test of the harness thesis and the
recommended next tree (`EXTRACTION-QUALITY-GAUGE.5`, or its own tree) — replace, don't patch. `.3`/`.4`
(permission/relational, dedup) remain useful but secondary to the replacement.

## `.5` result — the thesis is VALIDATED (`2026-06-06`)

`ir/constraint_extract_llm.rs` + `extract-constraints-llm`: per sentence the LLM proposes structured
`(subject, kind, condition)` constraints; Rust grounds each (subject types as `Signal`; condition must
appear in source). Composes `.1`+`.2` into a real extractor that REPLACES the Pattern set. +4 tests.

**Measured on CHI (134 sentences, ~2 min):**

| extractor | constraints | NLI-entailed |
|---|---|---|
| Pattern (baseline) | 162 | 28 = **17%** |
| `.1`+`.2` patched | 113 | ~22% |
| **LLM-primary (`.5`)** | **44** | 28 = **64%** |

**PRECISION jumps ~4× (17%→64%)** — that part is robust. **But recall is traded, not free** (recall
proxy, by subject+kind): the LLM-primary's 28 entailed are a *different* set from the Pattern's — it
covers only **9/22 (41%)** of the Pattern's known-good constraints while finding **18 net-new**. The
divergence is concentrated in `must_be_value` (the error-prone relational class — `DBID`, `TXNID`,
`RETURNNID`… — the LLM mostly dropped; *some correctly* as relational mis-extractions the NLI happened
to pass, *some* maybe genuine misses). So **replace buys large precision at some recall cost** — it
does *not* strictly dominate the Pattern set.

**Honest verdict:** the thesis "*replace can beat patch on quality*" is **supported on precision**
(4×, robust across the same oracle), but "*replace IS net-better*" is **not settled** — that needs a
small CHI **gold** to measure recall/F1. The earlier "kept the same 28" framing was wrong (same
*count*, different *set*); corrected here. The remaining 16 not-entailed + the recall gap are the next
work.

**Strategic conclusion → the path forward:** the harness pattern works and replace shows a large
precision gain, so the program is to *replace* each extraction stage (relations, temporal rules,
registers) with an LLM-primary, Rust-grounded extractor measured by the gauge — **but gate it on `.6`
first** (a CHI gold to confirm recall isn't being sacrificed). Precision-without-recall would be a trap
the gauge alone can't see. Settle that, then scale the replace program. That is the concrete (and
honestly-qualified) path to "human-SpecForge in Rust."

## Task Tree

- ID: `EXTRACTION-QUALITY-GAUGE` · Status: `active` · Children: `.0`–`.4`
- ID: `EXTRACTION-QUALITY-GAUGE.gauge` · Status: `done` · Goal: establish the NLI-oracle not-entailed
  rate as a per-doc extraction-quality gauge; measure CHI (~83%) + APB (~29%), hand-validate (18/18).
- ID: `EXTRACTION-QUALITY-GAUGE.1` · Status: `done` (prototype) · Goal: **entity discrimination** —
  derived typed classifier + LLM judgment + Rust grounding + enforcement. Built (`ir/entity_typing.rs`
  + `entity-type` cmd, tested); measured on CHI: 27/66 subjects filtered correctly (TXSACTIVE
  recovered), drops 30% of constraints (84% were wrong), gauge 17%→18%. Architecture validated;
  follow-up = wire the gate into the real extractor path + improve fine sub-typing.
- ID: `EXTRACTION-QUALITY-GAUGE.2` · Status: `done` (prototype) · Goal: conditional/temporal
  constraints first-class. `ir/condition_extract.rs` + `extract-conditions` cmd (LLM-judged,
  source-grounded), tested; CHI 22/25 captured, gauge 17%→22%. Complementary to `.1`.
- ID: `EXTRACTION-QUALITY-GAUGE.5` · Status: `done` · Goal: **LLM-primary grounded constraint
  EXTRACTOR** composing `.1`+`.2`. Built (`ir/constraint_extract_llm.rs` + `extract-constraints-llm`,
  tested) + measured: CHI 162→44 constraints, **precision 17%→64% (~4×)** but recall TRADED (covers
  41% of Pattern-good + 18 net-new). Thesis supported on precision; net-better needs a CHI gold.
- ID: `EXTRACTION-QUALITY-GAUGE.6` · Status: `done` (CORRECTED) · Goal: settle replace-vs-patch
  recall. **Domain correction (owner + CHI B2.4):** CHI is a packet/flit protocol, NOT a wire bus —
  `DBID`/`TxnID`/`ReturnNID` are *transaction-identifier FIELDS* (contents of flits), **not signals**.
  So the LLM-primary extractor was **correct** to exclude them from *signal* constraints; my earlier
  "genuine miss" verdict was wrong (the Pattern extractor had mis-typed fields *as* signals). The CHI
  "recall loss" was largely **correct field-exclusion**, plus a smaller real gap.
- ID: `EXTRACTION-QUALITY-GAUGE.7` · Status: `done` (clean test) · Goal: settle recall on a clean
  WIRE-BASED spec (no signal/field confound). Ran `.5` on **APB** vs its gold: **recall 4/6 = 67%** —
  the 2 misses (`PBUSER`/`PNSE` `must_be_value VALID`) **are** signals. So there IS a real, **localized
  `must_be_value` recall gap** even on clean signals — separable from CHI's field issue. Two distinct
  problems, both characterized. Next = `.8`.
- ID: `EXTRACTION-QUALITY-GAUGE.8` · Status: `pending` · Goal: close the `must_be_value` recall gap in
  the LLM-primary extractor (it under-extracts value constraints); re-measure on APB + AXI/AHB.
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD` · Status: `pending` · Goal: the SIGNAL-vs-FIELD ontology for
  packet/flit protocols (CHI/CXL/PCIe-class) — model fields (flit contents) distinctly from signals
  (physical wires), per the doc's own B16-Signals vs B2.x-Fields split. The real future target for
  CHI-like PDFs (owner: "in fine we need to handle such cases too").
- ID: `EXTRACTION-QUALITY-GAUGE.3` · Status: `pending` · Goal: permission/relational disambiguation.
- ID: `EXTRACTION-QUALITY-GAUGE.4` · Status: `pending` · Goal: constraint dedup by (subject, kind, condition).
- ID: `EXTRACTION-QUALITY-GAUGE.0` · Status: `pending` · Goal: wire the gauge into converge/CI.

## Changelog

- `2026-06-06`: Created. Gauge established (NLI-oracle not-entailed rate); CHI measured at ~80%
  erroneous (hand-validated 18/18), taxonomy recorded; fix backlog opened. See
  [[conformal-tier-agreement-degenerate]] and `docs/tasks/TABLE-GRITS-CONFORMAL.md`.
