# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.7` SHIPPED `2026-09-13`.** A table row's subject exemption no longer
  survives a clause that has its own subject one token back: `obligation_head_is_a_foreign_identifier` reads the
  PREMODIFIER when the head carries no identifier, and it counts only when the preceding token is the identifier
  **raw** — punctuation ends a phrase rather than opening one. That one condition separates AXI's
  `WTAG bits must be valid … enabled by WSTRB` from LTI's `When LASSIDV is LOW, this signal must be 0`.
  Corpus **200/127 → 199/126**; AXI-L rebuilt **55 → 54**; wire golds still `fp=0`, `F1=1.000`.
- **The node's own sizing was wrong in BOTH directions, and only building the rejected rule found it.** Prototyped
  and diffed per document, the widest rule removes **12** records, not the 8 listed; MMU-700 `dyn_sigcon_0008` was
  already not reproducing (cost in *reproduced persisted* records: 2, not 3); and one it never named, AXI-H
  `sigcon_0043 WSTRB`, is the same defect in the older AXI spec. **Re-derive a predecessor's population before
  trusting it to size your change** (`[[verify-before-publishing]]`).
- **A finding the rebuild produced, now owned: `SIGNAL-DECLARATION-ROW-DROP.4`.** Removing the fabrication moved
  SemanticIR by nothing, because `WSTRB` is not in AXI's declared catalog at all —
  `parse_explicit_signal_declaration`'s `index != tokens.len()` discards a WHOLE declaration, direction included,
  when it cannot finish the width. **83 declared signals across 10 documents never reach the SemanticIR catalog**
  (17 arithmetic widths, 66 unstated) and every obligation about them is demoted:
  `scripts/measure_declared_signals_missing_from_semantic.py`, `[[arithmetic-width-drops-the-declaration]]`.
  **Measure a constraint change at EVIDENCE, not at SemanticIR.**
- Next action: pick from the open `.3k` leaves, each with its own measured sizing — `.3k.8` (statement/row duplication;
  changing the merge key moves reproduction identity corpus-wide), `.3k.10` (statement-initial fronted condition),
  `.3k.12` (a predicate between a signal and its level); `.3j` open and unsized; `.3k.9` deliberately not next (`DO NOT
  SHIP YET`). `SIGNAL-DECLARATION-ROW-DROP.4` is the other live frontier and the larger recall gap.
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
