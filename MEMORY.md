# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-CATALOG-CAPTURE-GAP.6` CLOSED `2026-09-14` (PROBE/DOC, no rule)** — and it moved the
  question to the tree that owns it.
- **The two-column relationship shape is almost empty**: 3 tables corpus-wide, **0 in the current stratum**,
  and one of the three is a FALSE POSITIVE (CoreSight `table_0047` is `name | description` —
  `TSCLK | Interface clock` — admitted because "Interface clock" starts with an identifier-shaped token).
  The real population is **2 tables in 1 legacy document, 18 names**: AMBA LPI's `Table 2-2`/`Table 3-2`.
- **The rows do not drop on the table shape, and the trace proves it.** The table is typed, `name_col` is
  correctly 0, `PACTIVE`/`PSTATE`/`PREQ`/`PACCEPT`/`PDENY` are all accepted identifiers, and
  `check_signal_col` is even FOUND at column 1 (`associated check signal` contains `check signal`) — while
  `covered_signal_col` is not (`standard p-channel signal` has no `covered`). The row then leaves by
  **`NoDirectionAndNoWidth`**: it offers no ATTRIBUTE. Nothing upstream is wrong.
- **The real question is `SIGNAL-DECLARATION-ROW-DROP`'s and is now opened as `.1d`: must a declaration carry
  an ATTRIBUTE to carry an IDENTITY?** Every `.2*` leaf so far answered it by teaching the reader to read one
  more attribute; a check-relationship table has none to read, because the fact it states is a RELATION that
  presupposes both signals. **Size it against all 482 dropped rows, not against LPI's ten** — a rule that
  admits an identity with no attribute mints a signal from any identifier-shaped cell, which is exactly what
  `.2a`'s placeholder refusal and `ACTOR-NOUN-RELATION-DECLARATION.1`'s orthography rule exist to stop.
- Next action: `SIGNAL-DECLARATION-ROW-DROP.1d` (classify the 482 dropped rows by what each DOES offer;
  the guard must be sized in the same census), or `CORPUS-CHAIN-CURRENCY.7` step 3 (I2C RE-INGEST — decide
  first whether it belongs here or to `CORPUS-COVERAGE`; it moves the retention declaration), or
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`, `SIGNAL-DECLARATION-ROW-DROP.4c`/`.2d`/`.4b`, the unsized `.3j`.
  `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 254; directive 16 still gates it on
  full CI. Corpus state: APB-e rebuilt `2026-09-14`; **`um10204…i2c` is the only artifact the current build refuses**, and it needs a RE-INGEST (bundle
  reclaimed). **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part
  split first. Standing hazards, each with its procedure in a fact card: **a leaf that changes a READER moves every document it does not rebuild, and
  only a CI-tier sweep sees it** (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **CI-tier costs, measured `2026-09-14`:
  `check_chain_currency.sh` 28m00s, `check_proof_seal_currency.sh --total` 18m45s** — budget hours, run detached. **~98% of emitted `.isf` signals
  carry `(width 1)`**, so width-recall work is invisible at that boundary until the default is fixed. **`specforge validate` is NOT idempotent** —
  validating an upstream a second time invalidates every downstream artifact built before it; exactly one validate per artifact, upstream-first.
  **A census boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control that says so. **A new fact card may not reuse a question key
  another card already answers.** **A census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **never
  from a mirror**. **A Rust change moves `flow_census.json`**; a new `scripts/` file must be STAGED or claim-verification calls it untracked.
  **Editing a live surface sets off a fixed bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo
  gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`).
  This file's cap is 50 lines.
