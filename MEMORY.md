# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`CORPUS-CHAIN-CURRENCY.4` CLOSED `2026-09-14` (PROBE/DOC)** — both its questions answered by
  running the thing: what is drifted, and what it costs to know.
- **COST, measured, warm build: `check_chain_currency.sh` 28m00s; `check_proof_seal_currency.sh --total`
  18m45s.** Together ≈ 47 min — that IS "`--all` did not finish in 50 minutes"; stop restating it as a
  mystery. Neither is affordable per commit, both are per push, and directive 16 already puts them there.
  **CHAIN-CURRENCY stays CI-tier.**
- **DRIFT: evidence 23/24 current, semantic 25/27, intent 25/27, isf-adapter 25/27; retention exactly 24.**
  I2C evidence is CONTENT stale (reproduces at `1ada364a`); APB-e semantic is CONTENT stale AND refused by
  `intent`/`isf-adapter` (reproduces at `956fbcce` — `SIGNAL-DECLARATION-ROW-DROP.4c`). Neither is this
  session's doing and both were proven so from their own byte-unchanged upstream artifact.
- **The real finding is about the GATE, and it is `.5`.** `PROOF-SEAL-CURRENCY` runs at gate tier and probes
  **one representative per DISTINCT SEAL per stage** — and the census says the corpus carries **1 distinct
  seal per stage across 27 artifacts**. Its sample size is **1 in 27**, and today it passed in a tree where
  `specforge intent <apb-e>/semantic_ir.json --dry-run` is REFUSED. `--total` catches both, with the right
  diagnostic. The refusal is a property of the recorded DERIVATION TOPOLOGY, which the seal digest does not
  cover: **fold that into the census key and a refusing document earns its own probe** — one extra probe
  instead of twenty-six. Raising `--total` to gate tier is NOT the answer (18m45s).
- Next action: `CORPUS-CHAIN-CURRENCY.5` (measure the distinct-key count under a topology-bearing key BEFORE
  changing it — a key that makes every document distinct has silently become `--total` at gate tier), then
  `SIGNAL-DECLARATION-ROW-DROP.4c` (it blocks APB-e's rebuild), `SIGNAL-CATALOG-CAPTURE-GAP.6`,
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`. **Do NOT rebuild APB-e before `.4c`**; I2C needs a re-ingest, not a
  replay (its bundle is reclaimed). `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 249; directive 16 still gates it on
  full CI. **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part split
  first. Standing hazards, each with its procedure in a fact card: **a leaf that changes a READER moves every document it does not rebuild, and
  only a CI-tier sweep sees it** (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`) — when a slice changes a reader, the documents it
  did NOT rebuild are the ones to check. **An evidence-stage change that MOVES a current-schema artifact's content stales its proof and it then refuses
  to LOAD** — use that refusal as the signal for WHICH documents move; rebuild `evidence → validate → semantic → validate → intent → adapt`, each
  validated once, upstream-first, restoring the bundle from `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then
  `diff -rq`, remove, retention back to **24**. **A census boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control that says so. **A new fact card may not reuse a question key
  another card already answers.** **A census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **never
  from a mirror**. **A Rust change moves `flow_census.json`**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new
  `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`; the book's line-pinned regions re-anchor BY CONTENT, and the claim IDs that move ARE the
  `Published-claims:` declaration. **The doctrine driver runs no cargo gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run the
  fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
