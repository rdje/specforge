# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`CORPUS-CHAIN-CURRENCY.9` CLOSED `2026-09-14` (CODE/DOC) — THE PER-STAGE TOTAL SEAL PROBE IS
  ON, AND THE TREE HAS NO ELIGIBLE LEAF LEFT.** `TOTAL_PROBE_STAGES` defaults to `'semantic intent'`. Measured
  before and after on one tree: the gate **4m13.0s → 5m30.1s** (2nd post-change sample 5m24.5s), so **+72-77 s,
  ~+29%**; this check alone **7.4 s → 1m12.7s**. `source-ir`/`evidence` stay SAMPLED — only they replay
  extraction — and the check still prints that it is blind there; `--total` (1m59.2s, CI) covers them.
- **The leaf's real addition is self-test 21, because nothing tested the DEFAULT.** `.6`'s controls 17/17b pass
  the stage set in explicitly, so they proved the MECHANISM while the shipped value was unguarded. Case 21 passes
  no override. Observed RED: `SPECFORGE_PROOF_SEAL_TOTAL_STAGES=''` gives **20/21**, failing only on case 21.
  **General rule: a control that passes its own configuration tests the mechanism, never the shipped default.**
- **`.8` (same day) made that affordable**: the three corpus-replay entrypoints now share ONE binary predicate,
  `scripts/lib/corpus_replay_binary.sh`, at the **release** profile. `--total` **18m45s → 1m59.2s**;
  `check_chain_currency.sh` **28m00s → 12m38.2s** (≈47 min of CI-tier doctrine → **14m37s**). The verdict does
  not move — both profiles accept 27/27, and AXI's 43,419,318-byte `intent --dry-run` is byte-identical at the
  same SHA-256. A cold release build is **36.1 s**, is never paid on a corpus-less machine (the checks skip at
  0.031 s before cargo), and its tree is the SMALLER one (234 MB vs 1.1 GB).
- Next action: **pick a new tree — `CORPUS-CHAIN-CURRENCY` is exhausted** (`.0`-`.9` closed, corpus CURRENT,
  retention exactly 24). Candidates: `TEXT-LAYER-IDENTIFIER-SPLIT.2` (census, no code; `.1` needs a live VLM),
  `SIGNAL-DECLARATION-ROW-DROP.4b`/`.4c`/`.2d`/`.2f`, the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at
  259; directive 16 still gates it on full CI. `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its
  3,000-line ceiling (95.9%) — the next leaf landing there needs a part split first. **`DOCTRINE_ENFORCEMENT.md`
  line 394 is 845 of the 1,024-byte `line_bytes_each` ceiling**; that registry row is the widest line in the
  workflow-standards surface, so route new detail to §10 prose, never into the table cell. Standing hazards,
  each with its procedure in a fact card: **always say which BINARY PROFILE a probe cost was measured with** —
  it is `release` everywhere now (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **A test
  that scores a table against a catalog THAT TABLE FED is circular**
  (`[[a-dropped-declaration-row-is-usually-not-a-signal]]`). **A leaf that changes a READER moves every document
  it does not rebuild, and only a CI-tier sweep sees it.** **~98% of emitted `.isf` signals carry `(width 1)`**.
  **`specforge validate` is NOT idempotent** — exactly one validate per artifact, upstream-first, `--dry-run`
  before every write. **A census boundary keyed on a PERSISTED field is a lower bound**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`). **A new fact card may not reuse a question key another
  card already answers.** **A Rust change moves `flow_census.json`**; a new `scripts/` file must be STAGED or
  claim-verification calls it untracked; **an insert into a book chapter re-pins line-anchored regions in ALL
  THREE of `published_assertions`/`book_quantitative_claims`/`current_claim_census`, and a bulk digest refresh
  must NEVER touch a node carrying `start_line`** — that overwrites a one-line control digest with a whole-file
  one (`[[live-surface-edit-bookkeeping-chain]]`). **The doctrine driver runs no cargo gate**; `evidence.rs`
  tests live in **`-p specforge-core`**. This file's cap is 50 lines.
