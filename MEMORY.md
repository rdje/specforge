# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.1d` CLOSED `2026-09-14` (PROBE/DOC)** — the tree's own thesis
  asked directly and **answered NO at the row level: 24% precision**.
- **Must a declaration carry an ATTRIBUTE to carry an IDENTITY? Measured: no.** Population is the READER's
  own accounting (`scripts/measure_dropped_row_offerings.py`), not a scan — `.1` made the drop countable, so
  a dropped row here is one the real reader really dropped. **Only 4 of 27 documents carry it** (those
  rebuilt since `.1`): **71 rows, 100% joined**, 59 `no_direction_and_no_width` + 12 `name_not_an_identifier`.
  `.0`'s corpus-wide **482 is a different population** (a scan of all 78) — never summed.
- **14 real against 45 phantom.** Real: AXI `table_0092` (a signal-name GRID — every cell a real AXI signal),
  ADIv6 `table_0058` (pin equivalence `SWDIOTMS|SWDIO|TMS`), `table_0039`, `table_0041`. Phantom: AXI
  `table_0199` (PAS encoding), `table_0265` (a LEGEND — `Y`/`YM`/`YS`/`O`/`NS`), `table_0183`/`0184`
  (PARAMETER tables — `LOOP_W_WIDTH` as a wire), AHB `table_0014`/`0019` (HBURST/HPROT encodings), ADIv6
  `table_0108` (garbled body). **The refusal is right.**
- **The answer lives at the TABLE, and two shape tests are already refuted.** Every one of the ten tables is
  UNIFORMLY real or UNIFORMLY phantom — not one has a mix. *Every header is a name header* selects only
  `table_0058` (AXI's headers are literal signal names, `axid`/`axaddr`, matching no keyword); *every body
  cell is an identifier* admits the legend (`Mandatory` is a lone word the identifier test accepts). What
  holds over the four is that their cells are drawn from the document's OWN declared catalog — a GROUNDING
  test, not a shape test. Opened as `.1e`; measure before writing a rule.
- Next action: `SIGNAL-DECLARATION-ROW-DROP.1e` (size the grounding test against all ten tables and every
  `signal_description` table), or `CORPUS-CHAIN-CURRENCY.7` step 3 (I2C RE-INGEST — decide first whether it
  belongs there or to `CORPUS-COVERAGE`; it moves the retention declaration), `TEXT-LAYER-IDENTIFIER-SPLIT.1`,
  `SIGNAL-DECLARATION-ROW-DROP.4c`/`.2d`/`.2f`/`.4b`, the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 255; directive 16 still gates it on
  full CI. Corpus state: APB-e rebuilt `2026-09-14`; **`um10204…i2c` is the only artifact the current build refuses**, and it needs a RE-INGEST (bundle
  reclaimed). **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part
  split first. Standing hazards, each with its procedure in a fact card: **the reader's runtime accounting only exists in REBUILT documents** — 4 of 27
  — so a census over it is exact but narrow (`[[a-dropped-declaration-row-is-usually-not-a-signal]]`). **A leaf that changes a READER moves every
  document it does not rebuild, and only a CI-tier sweep sees it**
  (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **CI-tier costs, measured `2026-09-14`: `check_chain_currency.sh` 28m00s,
  `check_proof_seal_currency.sh --total` 18m45s** — budget hours, run detached. **~98% of emitted `.isf` signals carry `(width 1)`**, so width-recall
  work is invisible at that boundary until the default is fixed. **`specforge validate` is NOT idempotent** — exactly one validate per artifact,
  upstream-first. **A census boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control that says so. **A new fact card may not reuse a question key
  another card already answers.** **A Rust change moves `flow_census.json`**; a new `scripts/` file must be STAGED or claim-verification calls it
  untracked. **Editing a live surface sets off a fixed bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no
  cargo gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run the fixture suite with the locality gate
  (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
