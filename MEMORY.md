# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`CORPUS-CHAIN-CURRENCY.6` SHIPPED `2026-09-14` (CODE) — mechanism landed, activation held.**
- **Shipped**: `probe_scope_for <stage>` in `check_proof_seal_currency.sh` makes the probe scope a PER-STAGE
  decision (`--total` still forces total everywhere), with per-stage reporting. Self-test **17 restated per
  stage** — its stub now refuses only at `source_ir/`, so it still proves the blindness the sampled tier pays
  for — and new **17b** proves the catch at a TOTAL stage. Mini corpus gained a semantic stage. **20/20.**
- **It ships INERT, and that is the leaf's real finding.** Activating it was tried and MEASURED: the gate
  then FAILS on the live corpus, naming APB-e and I2C at both stages — correctly, because both artifacts
  genuinely are refused by their own consumers. **A gate that fails closed over a known-broken corpus is
  right AND unlandable**: it blocks every commit until a repair that is itself blocked (APB-e by
  `SIGNAL-DECLARATION-ROW-DROP.4c`; I2C by a reclaimed bundle → needs a RE-INGEST, not a replay). So the
  mechanism lands with its controls and the contract is NOT widened to accommodate the failure.
  **Activation is one constant**: `TOTAL_PROBE_STAGES` → `'semantic intent'`.
- **Observed RED both ways**: with the stage set emptied, 17b fails (*"a TOTAL stage passed over a document
  its own loader refuses"*); with it activated on the live corpus, the check is RED on exactly the two
  documents. Default gate run: **15.9 s**, green, same verdict as before the change.
- **`.7` opened with the repair ORDER** — `.4c` decides first, then rebuild APB-e, then decide whether I2C's
  re-ingest belongs here or to `CORPUS-COVERAGE` (it replaces a document wholesale and moves retention), then
  flip the constant. **Footnote-stripping alone does NOT settle `.4c`**: `ADDR_WIDTH/8` vs
  `ceil(ADDR_WIDTH/8)` differ by the `ceil` too, so that leaf owns a real expression-equivalence question.
- Next action: `SIGNAL-DECLARATION-ROW-DROP.4c` (census every `width_mismatch` in the corpus first; it
  unblocks `CORPUS-CHAIN-CURRENCY.7`), or `SIGNAL-CATALOG-CAPTURE-GAP.6`, `TEXT-LAYER-IDENTIFIER-SPLIT.1`,
  the unsized `.3j`. **Do NOT rebuild APB-e before `.4c`.** `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 251; directive 16 still gates it on
  full CI. **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part split
  first. Standing hazards, each with its procedure in a fact card: **a leaf that changes a READER moves every document it does not rebuild, and only a
  CI-tier sweep sees it** (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`) — when a slice changes a reader, the documents it did
  NOT rebuild are the ones to check. **CI-tier costs, measured `2026-09-14`: `check_chain_currency.sh` 28m00s, `check_proof_seal_currency.sh --total`
  18m45s** — budget hours, run detached. **An evidence-stage change that MOVES a current-schema artifact's content stales its proof and it then refuses
  to LOAD** — rebuild `evidence → validate → semantic → validate → intent → adapt`, each validated once, upstream-first, restoring the bundle from
  `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then `diff -rq`, remove, retention back to **24**. **A census
  boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control that says so. **A new fact card may not reuse a question key
  another card already answers.** **A census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **never
  from a mirror**. **A Rust change moves `flow_census.json`**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new
  `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run
  the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
