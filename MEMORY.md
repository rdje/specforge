# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`SIGNAL-CATALOG-CAPTURE-GAP.3`** SHIPPED `2026-09-20` — the heading-as-declaration rule
  is in production: **25 declarations in 1 document, 0 in the other 77**, empty-catalog census **33 -> 32**,
  corpus rebuilt in the same commit, stratum still **27 replayed / 27 current / 0 stale**.
- **The shipped rule is 25, not `.2`'s 27, and the gap is the finding.** `.2` measured a SELECTION; a
  DECLARATION needs a sense, and asking which one exposed that `TGD_I` reaches "input" only inside
  *"the data input array [DAT_I()]"* — **a neighbour's direction, agreeing only because the `_I` suffix
  agrees**. Bind an attribute to the name, never to the neighbourhood.
- **`_I`/`_O` REFUSED (ADR 0006) on a control, not an argument**: 1,202 table-declared directed signals
  across 30 documents carry the suffix 0 times — [[direction-suffix-is-one-documents-convention]].
- **A new catalog UNLOCKS readers gated on having one** (`known_signals.is_empty()` early return), so +25
  declarations brought +4 enum statements. Predicted 25, got 29; the A/B made it attributable. Routed to
  `KG-ISF-COMPLETENESS.5.v`.
- **A producer change's blast radius is per-artifact and must be PROBED**: 26 of 27 kept loading, 1 refused.
- **`LIVE-DOCUMENT-PRESSURE-HEADROOM.20` decided and shipped**: a chapter grows in ONE section —
  `evidenceir.md` is 78.8% one *"Closed task trees"* log, and both earlier splits moved the tidy sections.
  Rules now in `COMMIT.md`: name a section for the BEHAVIOUR not the work unit; extend the section owning
  the concern; **when splitting, move the ACCRETING section**.
- Next action: **give `LIVE-DOCUMENT-PRESSURE-HEADROOM`'s post-seal chronology a writable home, THEN bound
  the root window.** The ROOT is the only binding constraint (229 of a 256 target; **exactly one line
  fits**); a node in a small part is free. Its header says to trim the window into the chronology part —
  **false for `.27a` `.28` `.29b` `.29c` `.30a` `.30b` `.30c`**, which closed after that part was sealed, so
  a trim deletes their only record. Verify per row. Full detail + the two findings waiting on it (`.20a`'s
  closure node; `reference/live-docs.md` at **92.7%**, unowned) are in
  [[book-chapter-growth-is-one-accreting-section]].
- Then **`SIGNAL-CATALOG-CAPTURE-GAP.5`** (14 of 78 artifacts have no validation report, so no
  `document_class`), **`KG-ISF-COMPLETENESS.5.v`** (census first), or **`EXTRACTION-QUALITY-GAUGE.3k.9`**.
- `cargo test -p specforge --lib <filter>` reports **0 passed, 474 filtered out** for an `ir/` change and
  exits 0. Use `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- `LEGACY-SOURCE-RECLASSIFICATION.0`+`.1`: **the bundle, not the label, is the binding constraint.**
- **After every push, watch the hosted run to a verdict** (directive `2026-09-20`); a stalled run is not a pass.
- **A parked leaf keeps its blocker until someone re-derives it.** Correcting a parking reason is not an
  argument for shipping. **Do not restate a handed-down number — re-derive it.**
- **Measure the PASS, not the function** (`.2h.2`); **coverage is a PROVENANCE question** (`.5c`);
  **characterise a difference before counting it** (`.11`); **read the ROOT CAUSE, not the symptom** (`.5`).
- `.36d` owns the claim registry lifecycle at the 80% record band. **Never carry the residual as a number** —
  read it from `measure_registry_capacity_coherence.py`.
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- Corpus 27/27 current. A slow gate is **measured, not attributed**: `probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
