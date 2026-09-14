# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`CORPUS-CHAIN-CURRENCY.7` CLOSED `2026-09-14` (DATA/DOC) — THE CORPUS IS CURRENT.** Both
  drifted documents rebuilt; the probe reports **27 of 27 accepted at semantic AND intent, exit 0**.
- **I2C's bundle was NEVER reclaimed, and three of this tree's nodes said it was.** The retention
  declaration lists I2C among the declared **24 retained**, and the sweep had said so too — it appears under
  `evidence: 24 replayed … 1 stale`, i.e. replayed and CONTENT-stale, not among the 54 unmeasurable. The
  error was reading the summary line's parenthetical as if it applied to the failing document. **The repair
  was a plain chain rebuild, not a re-ingest, and retention never moved.**
- **The I2C rebuild PUBLISHES a precision win**: `signal_constraints` **9 → 3**, `fact_provenance` 21 → 15.
  The 8 removed records are exactly what the `.3k` series refused and I2C never received — *"Every byte put
  on the USDA line must be eight bits long"* → `USDA must_be_high`; *"…a 2-wire push-pull serial bus that
  operates from DC to 5 MHz"* → `USCL must_be_stable`; *"If the data line (SDA) is stuck LOW…"* →
  `SDA must_be_low` from a fault condition. All four stages replay CONTENT SAME.
- **THE ACTIVATION IS STILL HELD, and for a NEW reason: the check probes with a DEBUG binary.** Switching the
  stage set on measured **13m01s** against **15.99 s** sampled. `check_proof_seal_currency.sh` runs
  `cargo build --bin specforge` and probes `target/debug/specforge`; **`.5`'s 66 s sizing used
  `target/release/specforge`** (1.2 s per probe against ~14 s). `.5`'s conclusion that the SCOPE is a
  per-stage question survives; the number that made it look gate-affordable does not. **Always say which
  binary a probe cost was measured with.** Corrected in `.5`, in `.7`, in the script header and in
  `[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`; activation reverted to inert in the
  same slice that measured it.
- Next action: **`CORPUS-CHAIN-CURRENCY.8`** — measure BOTH binary profiles COLD and WARM before moving the
  check's build (release is ~35 s warm + ~1.2 s/probe ≈ 100 s, against ~780 s of debug probing; but a cold
  release build is minutes and lands on every clean checkout and CI runner). Alternatives:
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`, `SIGNAL-DECLARATION-ROW-DROP.4b`/`.4c`/`.2d`/`.2f`, the unsized `.3j`.
  `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 257; directive 16 still gates it on
  full CI. **Corpus state: CURRENT — APB-e and I2C both rebuilt `2026-09-14`; 27/27 accepted at semantic and intent; retention exactly 24.** **
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling (95.9%)** — the next leaf landing there needs a part split first.
  Standing hazards, each with its procedure in a fact card: **a probe cost means nothing without its BINARY PROFILE** — the doctrine checks build and
  probe `target/debug`, ~12x slower per probe than `target/release`
  (`[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`). **A test that scores a table against a catalog THAT TABLE FED is circular**
  (`[[a-dropped-declaration-row-is-usually-not-a-signal]]`). **The reader's runtime accounting only exists in REBUILT documents.** **A leaf that
  changes a READER moves every document it does not rebuild, and only a CI-tier sweep sees it.** **CI-tier costs, measured `2026-09-14`:
  `check_chain_currency.sh` 28m00s, `check_proof_seal_currency.sh --total` 18m45s** — budget hours, run detached. **~98% of emitted `.isf` signals
  carry `(width 1)`**. **`specforge validate` is NOT idempotent** — exactly one validate per artifact, upstream-first, with a `--dry-run` before every
  write. **A census boundary keyed on a PERSISTED field is a lower bound on what the reader reads**
  (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`). **A new fact card may not reuse a question key another card already answers.** **A Rust change
  moves `flow_census.json`**; a new `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing a live surface sets off a fixed
  bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo gate**; `evidence.rs` tests live in
  **`-p specforge-core`**. This file's cap is 50 lines.
