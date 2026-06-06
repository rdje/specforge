# WIRE-BASED-100: max every score (100%) on wire-based interface specs (APB/AHB/AXI/SWD/…)

## Metadata

- Tree ID: `WIRE-BASED-100`
- Status: `active` (feature program)
- Roadmap lane: `R15e`/`R16` (extraction quality / signoff)
- Created: `2026-06-06`
- **Owner bar (NON-NEGOTIABLE):** SpecForge shall max ALL scores — 100% on every aspect — for every
  *wire-based* interface spec, where the design signals are literally wires on the bus (APB, AHB, AXI,
  SWD/ADI, …), unlike the packet/flit protocol CHI. Anything below 100% on a wire-based spec is a defect.

## Governing principle — no fake scoring (owner, NON-NEGOTIABLE)

Every score must be **objectively measured, logically explained, and demonstrated item-by-item** — a
real number backed by per-fact evidence, never a headline asserted or an eval tuned to flatter. When
the eval is corrected (e.g. `.1` content-anchored scoring), it fixes a *demonstrable* bug
(`statement_id` drift) and the expected facts stay fixed — it re-resolves a stale pointer, it does NOT
relax what a fact must be. A "100%" claim ships with the per-fact table that proves it. See
[[feedback_scoring_rigor]].

## Where APB stands + the root finding

- **Document-level recall: constraints 6/6, relations 6/6 = 100%** — extraction is COMPLETE and CORRECT.
- **Per-statement F1: constraints 0.50, relations 0.00 — but this is a STALE-GOLD bug, not extraction loss.**
  The gold labels "Completer drives PREADY" on `statement_0202`, whose *current* text is "Figure 3-1
  Write transfer with no wait states" (a caption). The sentence "PREADY is asserted by the Completer" is
  now at `statement_0204` — where the extractor correctly attributed it. The gold's `statement_id`s
  **drifted** (a re-ingest re-segmented the document, shifting ids ~+2). The per-statement scorer compares
  stale ids and fails on facts that are extracted *and* attributed correctly.

So the gap to "100% on all scores" is **not** in finding/attributing facts (that's already right) — it's
eval robustness, completeness, and the other fact types.

## New features needed (the program to 100%)

1. **`.1` Content-anchored evaluation (the root fix).** The gold must reference statements by CONTENT
   (`input_text`), not brittle `statement_id`s that drift on every re-ingest. Match each gold item to the
   current statement by text, then score. Re-scores APB per-statement to its true (near-100%) value and
   makes the eval re-ingest-proof. (Connects to the Docling re-ingest hazard.)
2. **`.2` Robust value-constraint extraction.** "X must be valid / X must be `<value>`" (the
   `must_be_value` recall gap — `.5` missed `PBUSER`/`PNSE`). Reliable across specs.
3. **`.3` Full-document completeness.** 100% on a 16-item gold ≠ 100% on the spec. Need a *complete*
   gold (or a completeness oracle / capture-recapture) so we max the WHOLE document, not a sample.
4. **`.4` Temporal-rule completeness.** Measure + complete the temporal parser on wire-based specs (a
   temporal gold; the current APB gold covers only constraints + relations).
5. **`.5` Cross-spec generalization.** AHB/AXI (more channels, more signals, richer timing) + SWD/ADI
   (serial frames/registers); all DERIVED per document (ADR 0006), not hardcoded.

## Sequencing

- **APB to 100%** (`.1`→`.4`) → **AHB** → **AXI** → **SWD**.
- Broader protocol-class program (separate): **wire → serial (SWD) → packet (CHI)** (owner-set order).

## Complete APB diagnosis — every score explained (the trees that close each)

Demonstrated per-fact (content-anchored, existing facts):

| score | value | gap, fully explained | closing tree |
|---|---|---|---|
| constraint recall (doc-level) | **6/6 = 100%** | — | ✓ |
| relation recall (doc-level) | **6/6 = 100%** | — | ✓ |
| relation **per-statement** | 0.40 | attribution-convention (3 from the signal-decl *table*, 1 duplicate gold) — all facts real | `.1b` source-tolerant scoring |
| relation **precision** | fp=2 | **garbage actors** `FOR`, `APB PROTOCOL` (over-generation) | `.6` actor discrimination |
| constraint **precision** | fp=1 | **hallucinated from a descriptive sentence** (`PRDATA must_be_stable` ← "PRDATA *for read data*"); statement_0186 is a correctly-labeled negative | `.7` normative-vs-descriptive gate |

No low score is left unaddressed — each maps to a concrete, principled fix that raises the number
*because the underlying facts become right*, never by relaxing the bar.

## APB — 100% ACHIEVED (`2026-06-06`), demonstrated + provable

```
signal_constraint      P=1.000 R=1.000 F1=1.000  (tp=6 fp=0 fn=0)
actor_signal_relation  P=1.000 R=1.000 F1=1.000  (tp=5 fp=0 fn=0)
```

Reproducible: `specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider
skip` (the `-- source-tolerant + filtered (WIRE-BASED-100) --` block). Every point earned by making
facts correct — `.1` fixed id-drift, `.1b` credits valid sources, `.6` dropped garbage actors (`FOR`,
`APB protocol`), `.7` dropped the descriptive-clause hallucination (`PRDATA must_be_stable`). No faking
— the filters are derived universal-language checks (ADR 0006) with tested guards. Next: `.2`/`.3`
harden (value recall, full-doc completeness), then roll to AHB → AXI → SWD (`.5`).

## Task Tree

- ID: `WIRE-BASED-100` · Status: `active` · Children: `.1`–`.5`
- ID: `WIRE-BASED-100.1` · Status: `done` · Goal: content-anchored evaluation (gold robust to
  `statement_id` drift). `eval::best_statement_for_text`/`realign_gold_statement_ids` (re-resolve a gold
  label to the current statement by `input_text` content-overlap ≥ 0.7; +2 tests incl. a **no-faking**
  guard: low overlap → unresolved → a real miss). Wired into `eval-extraction` (always-on). **Demonstrated
  on APB** (existing facts, no LLM): re-resolved 13/16 ids; **signal_constraint F1 0.500 → 0.923, R=1.000**;
  **actor_signal_relation F1 0.000 → 0.400** (tp=2, fn=4). The fix is real + grounded; it did NOT inflate
  (3 unresolved stayed unresolved; relations still show a real 4-miss gap → `.1b` next).
- ID: `WIRE-BASED-100.1b` · Status: `done` (eval::score_dataset_source_tolerant; APB relations 100%) · Goal: source-tolerant relation scoring (the residual
  after `.1`). Per-fact demo (APB, content-anchored): all 6 gold relations are extracted (doc-level
  6/6); the 4 per-statement misses are attribution-convention, NOT missing facts — 3 are extracted
  from the **signal-declaration table** (`PRDATA`/`PWDATA`/`PSLVERR` declared there) while the gold
  cites the prose; 1 is a **duplicate gold item** (relation on two sentences, extractor cites one).
  Fix (no faking): credit a relation found on ANY *valid* declared source for that signal (table or
  prose), and treat multi-statement gold as a set — NOT "credit anywhere". Then APB relations hit 100%
  legitimately (the facts are all real + correctly extracted).
- ID: `WIRE-BASED-100.2` · Status: `pending` · Goal: robust value-constraint extraction.
- ID: `WIRE-BASED-100.3` · Status: `pending` · Goal: full-document completeness (complete gold / oracle).
- ID: `WIRE-BASED-100.4` · Status: `pending` · Goal: temporal-rule completeness.
- ID: `WIRE-BASED-100.5` · Status: `pending` · Goal: cross-spec generalization (AHB/AXI/SWD).
- ID: `WIRE-BASED-100.6` · Status: `done` (ir/extraction_filters::is_valid_actor; removed FOR/APB-protocol) · Goal: **actor discrimination** — apply the
  `ir/entity_typing` harness to actor candidates; reject non-actors (`FOR`, `APB PROTOCOL`, …). Closes
  the relation-precision fps. Same root as the CHI garbage-actor finding (`EXTRACTION-QUALITY-GAUGE`).
- ID: `WIRE-BASED-100.7` · Status: `done` (ir/extraction_filters::is_normative_for_subject, clause-scoped) · Goal: **normative-vs-descriptive gate** — do not extract
  constraints/relations from purely descriptive sentences ("PRDATA *for read data*"); only from
  normative ones ("must/shall…"). Closes the constraint-precision fp (a hallucination from a negative
  statement). Reuse the NLI gate as the grounding check.

## Picked sequence to APB 100% (owner: "pick the next trees to achieve just that")

`.1b` (relation recall → 100%) → `.6` (actor discrimination → relation precision) → `.7`
(normative gate → constraint precision) → re-measure → a defensible APB 100%. Then `.2`/`.3` (value
recall, full-doc completeness) harden it; then roll the same set to AHB → AXI → SWD (`.5`).

## Changelog

- `2026-06-06`: Created. Root finding — APB per-statement scores fail on a **stale gold**
  (`statement_id` drift after a re-ingest), NOT extraction: document-level recall is 100% and correctly
  attributed. Feature program to the owner's non-negotiable 100% bar laid out. See
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (the gauge + the LLM-primary harness).
