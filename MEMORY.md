# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.1e` CLOSED `2026-09-14` (PROBE/DOC)** — three table-level
  discriminators tried, three refuted, and the third one **corrected `.1d`'s own adjudication** before it
  was itself refuted. **The `.1` branch is CLOSED**: the refusal is correct and its residue is named.
- **The correction, published rather than left standing**: AXI `table_0092` was adjudicated `real` in `.1d`
  ("a signal-name grid"). The grounding test scored it **0.00** against a 297-name catalog, and checking
  the names proved the test right — **`AXLEN`/`AXSIZE`/`AXPROT` are UNDECLARED while `AWLEN`/`ARLEN`/
  `AWSIZE`/`ARSIZE`/`AWPROT` are all declared**, and the caption reads *"Signals that should be the same in
  an exclusive sequence"*. The cells are AXI's **`Ax` METAVARIABLE** for a signal FAMILY, not wire names.
  `.1d`'s count is corrected **14/45 → 10 real / 49 phantom = 17% precision**.
- **Then grounding itself is refuted, for a METHODOLOGICAL reason, not a tuning one.** Scored over all 58
  current-stratum boundary tables it looks strong (28 at ≥0.50) — but that is **CIRCULAR**: a table that
  DECLARED its own signals scores against a catalog it fed and trivially reaches 1.00 (AXI
  `table_0246`–`0249` at 0.97–1.00). Excluding declaring tables leaves **11** tables: **9 at 0.00, one at
  0.12** (garbled, phantom), **one at 0.55** (ADIv6 `table_0058`, REAL), **none between**. Any threshold in
  `(0.12, 0.55]` selects exactly ONE table — a rule fitted to a single positive.
- **It is also structurally blind to the best candidates**: ADIv6 `table_0041` holds real signals and is
  excluded entirely because it PARTIALLY declares — a table that fed the catalog cannot be scored against
  it, and partially-declaring tables are where recall would come from.
- **What remains, so it is not re-derived**: ~10 recoverable rows in three ADIv6 tables against 49 phantoms,
  with no vocabulary-free, non-circular table-level test separating them. Reopen only with a discriminator
  that survives all ten tables.
- Next action: `CORPUS-CHAIN-CURRENCY.7` step 3 (I2C RE-INGEST — decide first whether it belongs there or to
  `CORPUS-COVERAGE`; it moves the retention declaration), `TEXT-LAYER-IDENTIFIER-SPLIT.1`,
  `SIGNAL-DECLARATION-ROW-DROP.4b`/`.4c`/`.2d`/`.2f`, or the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 256; directive 16 still gates it on
  full CI. Corpus state: APB-e rebuilt `2026-09-14`; **`um10204…i2c` is the only artifact the current build refuses**, and it needs a RE-INGEST (bundle
  reclaimed). **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part
  split first. Standing hazards, each with its procedure in a fact card: **a test that scores a table against a catalog THAT TABLE FED is circular** —
  exclude declaring tables before believing a distribution (`[[a-dropped-declaration-row-is-usually-not-a-signal]]`). **The reader's runtime accounting
  only exists in REBUILT documents** — 4 of 27 — so a census over it is exact but narrow. **A leaf that changes a READER moves every document it does
  not rebuild, and only a CI-tier sweep sees it** (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **CI-tier costs, measured
  `2026-09-14`: `check_chain_currency.sh` 28m00s, `check_proof_seal_currency.sh --total` 18m45s** — budget hours, run detached. **~98% of emitted
  `.isf` signals carry `(width 1)`**. **`specforge validate` is NOT idempotent** — exactly one validate per artifact, upstream-first. **A census
  boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control. **A new fact card may not reuse a question key another card
  already answers.** **A Rust change moves `flow_census.json`**; a new `scripts/` file must be STAGED or claim-verification calls it untracked.
  **Editing a live surface sets off a fixed bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo
  gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`).
  This file's cap is 50 lines.
