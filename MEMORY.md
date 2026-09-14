# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`CORPUS-CHAIN-CURRENCY.5` CLOSED `2026-09-14` (PROBE/DOC)** — its own proposal refuted by the
  first measurement it made, and the real lever found and measured.
- **A topology-bearing seal key IS `--total`.** `.4` proposed folding the recorded derivation topology into
  the seal-census key so a refusing document earns its own probe. Measured: that key gives **27 distinct
  keys for 27 documents** at evidence, semantic and intent — and so does the narrower ROOT-only key (39/89/139
  root ids), because every root's `output_sha256`/`inputs_sha256` are digests over its own document's content.
- **The lever is the per-stage TIER, and it is cheap.** `--total`'s 18m45s is not spread evenly: an accepted
  `intent --dry-run` is **1.2 s** and a refusal **0.24 s**, while evidence/source-ir probes replay extraction
  from the normalized bundle. Probing **every** one of the 27 at two stages costs **semantic 30.3 s + intent
  35.5 s = 66 s** and finds **every refusal the corpus currently has** (I2C at semantic; APB-e and I2C at
  intent). Sampling is right at source-ir/evidence and wrong at semantic/intent.
- **`CORPUS-CHAIN-CURRENCY.6` opened to ship it**: total probes at semantic+intent at GATE tier, sampled
  elsewhere; +64 s on the gate, stated rather than hidden. **The load-bearing part is the self-tests, not the
  tier constant** — self-test 17 asserts the sampled probe MISSES a divergent same-seal document, and it must
  become a PER-STAGE assertion rather than be deleted, or the check loses the control that documents its limit.
- Next action: `CORPUS-CHAIN-CURRENCY.6` (implement it; extend the self-tests first), then
  `SIGNAL-DECLARATION-ROW-DROP.4c` (it blocks APB-e's rebuild), `SIGNAL-CATALOG-CAPTURE-GAP.6`,
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`. **Do NOT rebuild APB-e before `.4c`**; I2C needs a re-ingest, not a replay.
  `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 250; directive 16 still gates it on
  full CI. **`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part split
  first. Standing hazards, each with its procedure in a fact card: **a leaf that changes a READER moves every document it does not rebuild, and only a
  CI-tier sweep sees it** (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`) — when a slice changes a reader, the documents it did
  NOT rebuild are the ones to check. **An evidence-stage change that MOVES a current-schema artifact's content stales its proof and it then refuses to
  LOAD** — use that refusal as the signal for WHICH documents move; rebuild `evidence → validate → semantic → validate → intent → adapt`, each
  validated once, upstream-first, restoring the bundle from `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then
  `diff -rq`, remove, retention back to **24**. **A census boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate is the control that says so. **A new fact card may not reuse a question key
  another card already answers.** **A census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **never
  from a mirror**. **A Rust change moves `flow_census.json`**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new
  `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`; the book's line-pinned regions re-anchor BY CONTENT, and the claim IDs that move ARE the
  `Published-claims:` declaration. **The doctrine driver runs no cargo gate**; `evidence.rs` tests live in **`-p specforge-core`**; never run the
  fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
