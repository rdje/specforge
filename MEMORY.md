# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.8` SHIPPED `2026-09-13`** (after `.3k.7` and `.3k.10` the same day).
  The statement path and the row reader publish one obligation twice, differing only in `source_text` — the LAST field
  of the merge key, so the key cannot see one fact. The key is split: `signal_constraint_assertion_key` is every
  identity field except the provenance and the merge key is built from it, so the replay's reproduction identity is
  byte-for-byte unchanged; the dedup then drops an APPENDED record whose provenance is CONTAINED in an established
  record asserting the same thing. Corpus **201/128 → 192/119**, APB-E the only document that moves: **27 → 18**
  through evidence/semantic/intent and `.isf` **56 → 38 rules with 12 distinct bodies before and after**.
- **Each of the three leaves re-derived its own node's population and each was wrong.** `.3k.7`: 12 records, not 8.
  `.3k.10`: three prototypes, and the corpus refuted the first two (the second cost a true record, which only AXI-L's
  REBUILD revealed — the replay could not see it, because AXI-L drops out of the comparison the moment its content
  moves). `.3k.8`: 82 same-fact records corpus-wide, of which only 9 are the cross-producer class, in one document.
  **Build the rejected version and measure it; re-derive a predecessor's population before it sizes your change.**
- **`SIGNAL-DECLARATION-ROW-DROP.4` (opened by `.3k.7`) is the largest open gap.**
  `parse_explicit_signal_declaration`'s `index != tokens.len()` discards a WHOLE declaration, direction included, when
  it cannot finish the width: **83 declared signals across 10 documents never reach the SemanticIR catalog** (17
  arithmetic widths, 66 unstated) and the grounding filter demotes every obligation about them.
  `scripts/measure_declared_signals_missing_from_semantic.py`, `[[arithmetic-width-drops-the-declaration]]`.
  **Measure a constraint change at EVIDENCE, not at SemanticIR.**
- Next action: `SIGNAL-DECLARATION-ROW-DROP.4`, or the remaining `.3k` leaves — `.3k.12` (a predicate between a signal
  and its level; size it against every logic-level record, and do NOT widen a skip list to fit one sentence) and the
  unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
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
