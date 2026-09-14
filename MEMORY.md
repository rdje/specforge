# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`CORPUS-CHAIN-CURRENCY.8` CLOSED `2026-09-14` (CODE/DOC) — THE CORPUS-REPLAY BINARY IS ONE
  PREDICATE AND ITS PROFILE IS `release`.** `check_chain_currency.sh`, `check_proof_seal_currency.sh` and
  `rebuild_stage_cascade.sh` each carried their own `cargo build … --bin specforge` + `target/debug/specforge`;
  they now share `scripts/lib/corpus_replay_binary.sh`, which the `.11` contract requires (a gate and a remedy
  that disagree about WHICH LOADER ANSWERS leave a debt nothing can clear).
- **The whole framing of `.8` was measured false in three places.** A cold release build of this workspace is
  **36.1 s**, not minutes; it is **never** paid on a clean checkout or a hosted CI runner, because all three
  entrypoints skip an absent `generated/` at **0.031 s with no cargo invocation at all**; and the release tree
  is the SMALLER one, **234 MB** against debug's 1.1 GB. What actually decides it: a build is a FIXED cost and a
  probe is a PER-DOCUMENT one, so the debug argument was right at 4 probes and wrong at 54. Crossover ≈ 2 large
  documents.
- **Published wins, all re-derived here:** `check_proof_seal_currency.sh --total` **18m45s → 1m59.2s**;
  `check_chain_currency.sh` **28m00s → 12m38.2s** (≈47 min of CI-tier doctrine → **14m37s**); the sampled gate
  check 15.9 s → 7.4 s; the activated stage set 12m57.9s → 69.8 s. **The verdict does not move and that was
  measured**: both profiles accept 27/27 at semantic and intent, and AXI's 43,419,318-byte `intent --dry-run` is
  byte-identical at the same SHA-256. The corpus was already built by RELEASE binaries (its provenance cards say
  so); only the gates interrogating it were debug.
- Next action: **`CORPUS-CHAIN-CURRENCY.9`** — activate `TOTAL_PROBE_STAGES='semantic intent'`. Both obstacles are
  gone; what it still owes is the TIER decision's own before/after on the WHOLE gate, which is **4m17s–4m45s**
  with the set inert, so the activation is ~+62 s (~24%). Alternatives: `TEXT-LAYER-IDENTIFIER-SPLIT.1`/`.2`,
  `SIGNAL-DECLARATION-ROW-DROP.4b`/`.4c`/`.2d`/`.2f`, the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at
  258; directive 16 still gates it on full CI. **Corpus state: CURRENT — 27/27 accepted at semantic and intent,
  retention exactly 24.** `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)
  — the next leaf landing there needs a part split first. Standing hazards, each with its procedure in a fact
  card: **always say which BINARY PROFILE a probe cost was measured with** — it is now `release` everywhere
  (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **A test that scores a table against a
  catalog THAT TABLE FED is circular** (`[[a-dropped-declaration-row-is-usually-not-a-signal]]`). **A leaf that
  changes a READER moves every document it does not rebuild, and only a CI-tier sweep sees it.** **~98% of
  emitted `.isf` signals carry `(width 1)`**. **`specforge validate` is NOT idempotent** — exactly one validate
  per artifact, upstream-first, with a `--dry-run` before every write. **A census boundary keyed on a PERSISTED
  field is a lower bound** (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`). **A new fact card may not
  reuse a question key another card already answers.** **A Rust change moves `flow_census.json`**; a new
  `scripts/` file must be STAGED or claim-verification calls it untracked — and **an insert into a book chapter
  re-pins every line-anchored claim region below it** (`published_assertions`, `book_quantitative_claims`,
  `current_claim_census`, plus the `shipped_behavior` aggregate authority). **Editing a live surface sets off a
  fixed bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo
  gate**; `evidence.rs` tests live in **`-p specforge-core`**. This file's cap is 50 lines.
