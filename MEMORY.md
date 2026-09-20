# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`SIGNAL-CATALOG-CAPTURE-GAP.3`** SHIPPED `2026-09-20` — the heading-as-declaration rule
  is in production. **25 declarations in 1 document, 0 in the other 77**; empty-catalog census **33 -> 32**;
  Wishbone's `IntentIR` gains an explicit interface of exactly those 25 wires. Corpus rebuilt in the same
  commit; stratum still **27 replayed / 27 current / 0 stale**.
- **The shipped rule is 25, not `.2`'s 27, and the gap is the finding.** `.2` measured a SELECTION (does the
  body state a direction anywhere); a DECLARATION needs a sense, and asking which one exposed that `TGD_I`
  reaches "input" only inside *"the data input array [DAT_I()]"* — **a neighbour's direction, agreeing only
  because the `_I` suffix agrees**. Bind the attribute to the name, never to the neighbourhood.
- **`_I`/`_O` REFUSED (ADR 0006), on a control not an argument**: **1,202 table-declared directed signals
  across 30 documents carry the suffix 0 times**. Settle a "is this universal grammar?" question by asking
  the corpus how universal it is — [[direction-suffix-is-one-documents-convention]].
- **A producer change's blast radius is per-artifact and must be PROBED, not assumed**: 26 of 27 kept
  loading, 1 refused. `.5b` found the same. Snapshot fully, rebuild stage-major with one `validate` each.
- **A new catalog UNLOCKS readers that were gated on having one** — `scan_encoding_tables_by_signal_anchor`
  returns early on `known_signals.is_empty()`, so +25 declarations brought +4 enum statements. Predicted 25,
  got 29; the A/B (call edge removed reproduces 1,501 exactly) is what made it attributable.
  **Expect second-order gains on a document that had nothing.** Routed to `KG-ISF-COMPLETENESS.5.v`.
- `cargo test -p specforge --lib <filter>` reports **0 passed, 474 filtered out** for an `ir/` change and
  exits 0. Use `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- Next action: **`SIGNAL-CATALOG-CAPTURE-GAP.5`** — 14 of 78 `EvidenceIR` artifacts carry no validation
  report, so they have no `document_class`; decide currency gap vs contract gap. Else
  **`KG-ISF-COMPLETENESS.5.v`** (census first, no rule) or **`EXTRACTION-QUALITY-GAUGE.3k.9`**, whose
  refusal reason still stands (67 documents moved to fix 18 names in 2, published effect 0).
- `LEGACY-SOURCE-RECLASSIFICATION.0`+`.1`: **the bundle, not the label, is the binding constraint**;
  the graph refuses a CORE MUTATOR but accepts a pure classifier + command-side write.
- **After every push, watch the hosted run to a verdict and repair until green** (directive `2026-09-20`,
  in `COMMIT.md`); a stalled run is not a pass.
- **A parked leaf keeps its blocker until someone re-derives it** — `.3`'s blocker (the corpus rebuild)
  was discharged by `RETAINED-BUNDLE-POPULATION-FROZEN.3` a day before it was written down. **Correcting a
  parking reason is not an argument for shipping. Do not restate a handed-down number — re-derive it.**
- **Measure the PASS, not the function** (`.2h.2`); **coverage is a PROVENANCE question** (`.5c`);
  **characterise a difference before counting it** (`.11`); **read the ROOT CAUSE, not the symptom** (`.5`).
- `.36d` owns the claim registry's lifecycle and triggers at the 80% record band (**17 of 21**).
  **Never carry the residual as a number** — read it from `measure_registry_capacity_coherence.py`.
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051), `.3` open; do not buy
  `TYPESAFE_API_KEY`. Corpus rebuilds use `rebuild_stage_cascade.sh`'s snapshot discipline.
- Corpus 27/27 current. A slow gate is **measured, not attributed**: `probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
