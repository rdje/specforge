# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.4a` SHIPPED `2026-09-13`** (4th leaf today, after
  `EXTRACTION-QUALITY-GAUGE.3k.7`/`.3k.10`/`.3k.8`). `parse_optional_width_hint` consumed exactly ONE whitespace token,
  so `Signal WSTRB is output width DATA_WIDTH / 8.` left `/` and `8` over and `index != tokens.len()` discarded the
  WHOLE declaration — identity and direction with it — while `DATA_WIDTH/8` without spaces parsed. A width is now read
  as an EXPRESSION (numbers, parameters, `+ - * /`, balanced parens, `ceil(…)`), ending at a whitespace-token boundary,
  with trailing prose tolerated only after a STRUCTURED expression. **14 of the 17 arithmetic-width declarations read.**
- **AXI-L: catalog 288 → 296 (with `WSTRB`), `signal_constraints` 55 → 56, `residual_decisions` 1 → 0, `.isf` 288 → 296
  signals / 135 → 138 rules.** The un-demoted record is *"An attached Subordinate must have its WSTRB input tied HIGH"*
  — the REAL obligation `.3k.7` found was being demoted beside the fabrication it removed, so the loop is closed.
  **Only 26 of 78 documents carry a current-schema EvidenceIR the semantic stage accepts**, so AXI-H/CHI/ATB/LTI
  recoveries are real in the reader and latent until re-ingest; the census falls 83 → 75, not 83 → 69.
- **Four leaves, four re-derived populations, four corrections.** `.3k.7`: 12 records, not 8. `.3k.10`: three
  prototypes, the corpus refuting the first two. `.3k.8`: 82 same-fact records corpus-wide of which only 9 are the
  cross-producer class. `.4a`: the leaf it came from had the ordering backwards and said the accounting was a
  precondition; the census script already answered it, so the recall half went first.
  **Build the rejected version and measure it; re-derive a predecessor's population before it sizes your change.**
- Next action: `SIGNAL-DECLARATION-ROW-DROP.4b` — 69 declarations the reader still refuses in SILENCE (MMU-700's 47
  among them), and the question with the blast radius: should a parsed DIRECTION survive an unreadable width? Measure
  as an ADDITION per document first. Or the remaining `.3k` leaves — `.3k.12` (a predicate between a signal and its
  level; do NOT widen a skip list to fit one sentence) and the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 239; directive 16 still gates it on
  full CI. `check_doctrines.sh --all` did not finish in 50 minutes (`CHAIN-CURRENCY` re-executes the real pipeline for every persisted artifact) — budget
  hours and run it detached. Standing hazards, each with its procedure in a fact card: **an evidence-stage change that MOVES a current-schema artifact's
  content stales its proof and it then refuses to LOAD** — use that refusal as the signal for WHICH documents move; rebuild `evidence → validate →
  semantic → validate → intent → validate → adapt`, each validated once, upstream-first, restoring the bundle from
  `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then `diff -rq`, remove, retention back to **24**. A corpus-root tool
  REFUSES an off-volume path. **A census counts the population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **never from a
  mirror**, and **a control asserting a record EXISTS pins what it says, fabrication included** (`[[one-modal-vocabulary-per-constraint-record]]`,
  `[[a-relational-predicate-is-not-a-value]]`). **A Rust change moves `flow_census.json` and the re-derivation must ATTRIBUTE the delta to the owning
  leaf**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new raw-evidence reader `information_flow_boundary.tsv`; a
  new `scripts/` file must be STAGED or claim-verification calls it untracked. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`; the book's line-pinned regions re-anchor BY CONTENT, and the claim IDs that move ARE the `Published-claims:`
  declaration. **A book section about an extraction rule goes in its concern's chapter; a changed rule leaves standing book text.** **The doctrine driver
  runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`); `evidence.rs` tests live in **`-p specforge-core`**; never run the fixture suite with
  the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`).
