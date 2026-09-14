# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`CORPUS-CHAIN-CURRENCY.7` step 2 DONE `2026-09-14` — APB-e's chain is REBUILT and the
  corpus is one document less stale.** The tree stays `active`; steps 3 (I2C) and 4 (activate) remain.
- **Procedure followed exactly**: EvidenceIR was already current and byte-unchanged, so the rebuild started
  at `semantic` and ran `semantic` → `validate` → `intent` → `validate` → `adapt --target isf`, **one
  validate per artifact, strictly upstream-first**. `intent --dry-run` was run against the rebuilt
  SemanticIR BEFORE writing intent, to prove the loader accepts it. Pre-rebuild snapshot held at
  `generated/preserved/CORPUS-CHAIN-CURRENCY.7/pre-rebuild/` (6 files, `8d1b9e13…8791f`).
- **What moved — exactly the prediction**: `interface_signal_conflicts` 0 → 1 (`PADDRCHK`,
  `ADDR_WIDTH/8` vs `ceil(ADDR_WIDTH/8)`), two `actor_ports` lose `width_hint`, one `interfaces` and one
  `signal_connectivity` record change. **What did NOT move, measured**: the emitted `.isf` `source_text` is
  **byte-identical at 4,029 bytes**; `adapter.json` has no moved section; the APB wire gold is unchanged
  (`signal_constraint` 1.000, filtered `actor_signal_relation` 1.000); retention is exactly **24**; the
  default seal gate passes; all three stages replay CONTENT SAME.
- **Measured at the gate that found it**: with `SPECFORGE_PROOF_SEAL_TOTAL_STAGES='semantic intent'`, the
  probes go **25/27 → 26/27** accepted at both stages. **`um10204…i2c` is the only refusal left.**
- **Left as-is and stated**: `generated/adapters/isf/<apb-e>/validation_report.json` predates the rebuild —
  `validate` takes IR artifacts not adapters, the documented order ends at `adapt`, and chain-currency
  excludes `validation_reports`, so nothing is stale by any gate; the file still deserves an owner.
- Next action: `CORPUS-CHAIN-CURRENCY.7` **step 3** — I2C. Its bundle is reclaimed, so `evidence` cannot
  replay it and a RE-INGEST replaces the document wholesale and moves the retention declaration: decide
  first whether that belongs here or to `CORPUS-COVERAGE`. Step 4 (set `TOTAL_PROBE_STAGES='semantic
  intent'`) follows step 3 and nothing else. Alternatives: `SIGNAL-CATALOG-CAPTURE-GAP.6`,
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`, `SIGNAL-DECLARATION-ROW-DROP.4c`/`.2d`/`.4b`, the unsized `.3j`.
  `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding. **The corpus under
  `generated/` changed** (APB-e semantic/intent/adapters rebuilt) — it is untracked, so the commit is
  documentation; the artifact state is recorded here and in the tree.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 253; directive 16 still gates it on
  full CI. **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part split
  first. Standing hazards, each with its procedure in a fact card: **a leaf that changes a READER moves every document it does not rebuild, and only a
  CI-tier sweep sees it** (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **CI-tier costs, measured `2026-09-14`:
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
