# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.12` CLOSED `2026-09-14` (PROBE/DOC, no rule)** — the class it
  named has a corpus population of **ONE** and the level walk's stop is correct.
- **Sized before touching the walk, which was the instruction.** Over all 261,858 persisted statements and
  constraint source texts (`scripts/measure_logic_level_walk_blockers.py`), "a word that stops the backward
  walk while a DECLARED signal sits within three words beyond it" is **14 cases / 11 unrelated words**:
  adverbs (`always`, `again`, `therefore` — crossing right), verbs (`remain`, `remains` — right), `can`
  (a PERMISSION `.3k.13` gated a day earlier — wrong), `cannot` (a NEGATION; crossing publishes the
  OPPOSITE — wrong), `this`, `write`, and the named `absent` — **once**. No list admits the adverbs without
  also admitting `cannot` or `can`.
- **`PDENY` blocks because it is NOT DECLARED, which is a catalog defect and not the walk's.** AMBA LPI
  holds 5 signals (`PACCEPT`, `PREQ`, `QACCEPTN`, `QDENY`, `QREQN`) while its own `Table 3-2` — typed
  `signal_description` — names ten in five rows and declares none, because it states neither direction nor
  width. Opened as **`SIGNAL-CATALOG-CAPTURE-GAP.6`**: a PARTIAL catalog, and a two-column RELATIONSHIP
  table (`X | XCHK`) is a general shape no current reader can see. **When a level binding looks lost, check
  the catalog before the walk** (`[[logic-level-walk-stops-at-eleven-unrelated-words]]`).
- **`CORPUS-CHAIN-CURRENCY.4` is INDEPENDENTLY CONFIRMED**: the real gate, run detached, reports
  `um10204…i2c evidence — the persisted artifact is NOT what the current binary produces:
  conditional_rules(13->13) fact_provenance(21->15) signal_constraints(9->3)` — the same numbers this
  session measured by hand. The sweep is still running; its WALL-CLOCK COST is the leaf's first
  deliverable and must be recorded when it lands.
- Next action: consume the chain-currency sweep and size `CORPUS-CHAIN-CURRENCY.4`; then
  `SIGNAL-CATALOG-CAPTURE-GAP.6`, `TEXT-LAYER-IDENTIFIER-SPLIT.1`, the unsized `.3j`, or
  `SIGNAL-DECLARATION-ROW-DROP.2d`/`.4b`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. **A detached `scripts/check_chain_currency.sh` is running**
  (log: session scratchpad `chain_currency.log`); it writes nothing and may be re-run.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 247; directive 16 still gates it on
  full CI. **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf that lands there needs a part
  split first (`ACTIVE-TASK-EVIDENCE-CONTAINMENT`). Standing hazards, each with its procedure in a fact card: **an evidence-stage change that MOVES a
  current-schema artifact's content stales its proof and it then refuses to LOAD** — use that refusal as the signal for WHICH documents move; rebuild
  `evidence → validate → semantic → validate → intent → validate → adapt`, each validated once, upstream-first, restoring the bundle from
  `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then `diff -rq`, remove, retention back to **24**. **A census
  boundary keyed on a PERSISTED field is a lower bound on what the reader reads** (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate
  is the control that says so. **A new fact card may not reuse a question key another card already answers** — the generator fails closed on it. **A
  census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **never from a mirror**. **A Rust change
  moves `flow_census.json`; the re-derivation must ATTRIBUTE the delta to the owning leaf**; a new module moves `module_inventory.tsv`; a new command
  `CLI_SURFACE_REGISTRY`; a new `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing a live surface sets off a fixed
  bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`; the book's line-pinned regions re-anchor BY CONTENT, and the claim IDs that move ARE
  the `Published-claims:` declaration. **The doctrine driver runs no cargo gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run the
  fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
