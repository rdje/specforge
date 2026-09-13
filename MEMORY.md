# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.4c` OPENED `2026-09-14`** — a real precision regression that is
  already in the code and has not shipped only because its document was never rebuilt. Found by the detached
  chain-currency sweep, not by any gate.
- **`.4a` turned one signal's width into a CONFLICT between two spellings of itself.** APB-e states
  `PADDRCHK` three times: `width ceil(ADDR_WIDTH/8) a.` (×2) and `width ADDR_WIDTH/8.`. Before `.4a` the
  footnote-marked form was unreadable, so only `ADDR_WIDTH/8` was seen. Now both are read, the interface
  reader calls it a `width_mismatch`, and **PADDRCHK loses its width entirely** —
  `interface_signal_conflicts` 0 → 1 and the `width_hint` disappears from every `actor_ports`,
  `interfaces` and `signal_connectivity` record. `X/8` against `ceil(X/8)` is a value against a safer form
  of itself. **DO NOT REBUILD APB-e until `.4c` decides the comparison** — the persisted SemanticIR is
  currently the BETTER artifact, and `intent` already refuses to replay from it.
- **Proven not mine, the same way I2C was**: the delta reproduces identically at `956fbcce`, the commit
  before this session, from APB-e's own (byte-unchanged) EvidenceIR.
- **The detached sweep is the instrument `CORPUS-CHAIN-CURRENCY.4` asked for and it is EARNING its cost.**
  Result so far: evidence **23/24 current** (I2C stale, 54 unmeasurable — bundle reclaimed); semantic
  **25/27 current** (APB-e content-stale, I2C blocked upstream, 51 unmeasurable); intent blocked for both.
  It has been running over **75 minutes** and is still in the intent/isf stages — the wall-clock figure is
  `.4`'s first deliverable and must be recorded when it lands.
- Next action: consume the sweep and size `CORPUS-CHAIN-CURRENCY.4` with its real cost; then
  `SIGNAL-DECLARATION-ROW-DROP.4c` (it blocks APB-e's rebuild), `SIGNAL-CATALOG-CAPTURE-GAP.6`,
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`, or the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. **A detached `scripts/check_chain_currency.sh` is still
  running** (session scratchpad `chain_currency.log`); it writes nothing and may be re-run.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 248; directive 16 still gates it on
  full CI. **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf that lands there needs a part
  split first. Standing hazards, each with its procedure in a fact card: **an evidence-stage change that MOVES a current-schema artifact's content
  stales its proof and it then refuses to LOAD** — use that refusal as the signal for WHICH documents move; rebuild `evidence → validate → semantic →
  validate → intent → validate → adapt`, each validated once, upstream-first, restoring the bundle from
  `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then `diff -rq`, remove, retention back to **24**. **A leaf that
  changes a READER moves every document it does not rebuild, and only the CI-tier sweep can see it** — two such drifts are now tracked
  (`CORPUS-CHAIN-CURRENCY.4`, `SIGNAL-DECLARATION-ROW-DROP.4c`). **A census boundary keyed on a PERSISTED field is a lower bound on what the reader
  reads** (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control that says so. **A new fact card may not reuse a question
  key another card already answers.** **A census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`),
  **never from a mirror**. **A Rust change moves `flow_census.json`**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`;
  a new `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`; the book's line-pinned regions re-anchor BY CONTENT, and the claim IDs that move ARE the
  `Published-claims:` declaration. **The doctrine driver runs no cargo gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run the
  fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
