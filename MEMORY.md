# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.4c` premise CORRECTED `2026-09-14`** — and the correction lifts
  a hold two trees were carrying. The leaf opened reading `.4a` as a regression; tracing the two widths to
  their tables shows it is not one.
- **The conflict is REAL.** APB-e states `PADDRCHK`'s width in two different tables: `table_0014`
  (*Table 5-1 Check signal descriptions*) says `ceil(ADDR_WIDTH/8) a`; `table_0017` (the APB-version matrix,
  body rotated by one) says `ADDR_WIDTH/8`. Those are **not** the same width when `ADDR_WIDTH` is not a
  multiple of 8, so `.4a` is right to report a `width_mismatch` and this session's earlier framing was wrong.
- **The real defect is what follows the conflict, and its blast radius is SMALLER than `.4c` first said**: a
  conflicted signal loses `width_hint` from `actor_ports`/`interfaces`/`signal_connectivity`, but the emitted
  `.isf` does NOT move — `PADDRCHK` already ships `(output PADDRCHK (width 1))`, as do **31 of APB-e's 32**
  signals, and the one current-stratum conflict AHB `HBURST` (`3` vs `HBURST_WIDTH`) likewise ships
  `(width 1)` for a 3-bit signal. **The width-1 default is a far larger pre-existing defect `.4c` does not
  own**, and a width restored into a `(width 1)` emitter would be invisible — so size `.4c` against that.
- **HOLD LIFTED: APB-e MAY be rebuilt.** `CORPUS-CHAIN-CURRENCY.7` no longer has `.4c` as a prerequisite; the
  rebuild costs one SemanticIR `width_hint` and no emitted `.isf` byte. Corrected in `.4c`, in `.7`, in
  `check_proof_seal_currency.sh`'s header and in `[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`.
- **Corpus `width_mismatch` population**: **1** current (AHB `HBURST`) and 48 legacy, of which the LTI and
  AXI-H ones are garbage widths from the rotation defect (`LAVALID`, `RESETn`, `V` read as "widths") rather
  than real disagreements. 38 `direction_mismatch` records also exist, all legacy.
- Next action: `CORPUS-CHAIN-CURRENCY.7` — rebuild APB-e (unblocked), then decide whether I2C's RE-INGEST
  belongs to this tree or `CORPUS-COVERAGE` (it replaces a document wholesale and moves retention), then set
  `TOTAL_PROBE_STAGES='semantic intent'`. Or `SIGNAL-CATALOG-CAPTURE-GAP.6`,
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`, the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 252; directive 16 still gates it on
  full CI. **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part split
  first. Standing hazards, each with its procedure in a fact card: **a leaf that changes a READER moves every document it does not rebuild, and only a
  CI-tier sweep sees it** (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **CI-tier costs, measured `2026-09-14`:
  `check_chain_currency.sh` 28m00s, `check_proof_seal_currency.sh --total` 18m45s** — budget hours, run detached. **~98% of emitted `.isf` signals
  carry `(width 1)`**, so any width-recall work is invisible at that boundary until the default is fixed. **An evidence-stage change that MOVES a
  current-schema artifact's content stales its proof and it then refuses to LOAD** — rebuild `evidence → validate → semantic → validate → intent →
  adapt`, each validated once, upstream-first, restoring the bundle from
  `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then `diff -rq`, remove, retention back to **24**. **A census
  boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control that says so. **A new fact card may not reuse a question key
  another card already answers.** **A census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **never
  from a mirror**. **A Rust change moves `flow_census.json`**; a new `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing
  a live surface sets off a fixed bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo gate**;
  `evidence.rs` tests live in **`-p specforge-core`**; never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This
  file's cap is 50 lines.
