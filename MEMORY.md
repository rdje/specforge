# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.9` RE-DERIVED and RE-OWNED `2026-09-13`; disposition DO NOT SHIP YET.**
  Its opening premise was wrong and the real defect is deeper: a Markdown escape (`PARTITION\_ACCESS`) fragments an
  identifier and the fragment is then **DECLARED** — eMMC holds `Signal PARTITION is width 1.` **That defeats the
  repository's standard `standalone wins` discriminator**, because the synthesized declarations ARE the standalone
  occurrences; the contamination manufactures its own evidence of innocence, and the first cut of the re-derivation
  reported 0 where the answer is 18 (`[[escaped-identifier-fragments-the-catalog]]`).
- **The call, on the measurement** (`python3 scripts/measure_escaped_identifier_fragments.py`, shipped, `--self-test`
  6/6): TEXT **67 of 78 documents**; CATALOG **18 declared names in 2 documents**; RECORDS **0 published constraint
  subjects**. The escape is Docling's and correct, SourceIR carries **zero**, and both contaminated documents are
  frozen — so re-ingest cannot fix it and the only fix that reaches them is a change to the shared identifier
  tokenization, moving the identity layer of 67 documents to correct 18 names in 2, for a published effect of zero.
  **A rule nothing exercises does not ship.** It also moves DECLARATIONS, so `replay-constraints` does not cover it
  and a detached `--all` is required. `.3k.9` records what must be true before it ships, in order.
- **Six leaves closed this session (`.3k.2k`, `.3k.3`, `.3k.4`, `.3k.11`, `.3k.13`, `.3k.5`), and the method is the
  result: BUILD THE CHANGE AS A PROTOTYPE AND MEASURE IT WITH `replay-constraints` BEFORE COMMITTING TO ITS DESIGN.**
  Every one had its shipped shape decided by a measurement that contradicted its own node — `.3k.3` found the
  narrowing it was written as would have DELETED a fact; `.3k.4` rejected two wider shapes costing 30 records; `.3k.11`
  had three "obvious" rules refuted, one by `WIRE-BASED-100.5i` going RED. Each node carries its own evidence.
- Next action: pick from the open `.3k` leaves, each carrying its own measured sizing — `.3k.7` (AXI `WSTRB`; the
  obvious repair costs 3 reproduced records), `.3k.8` (statement/row duplication), `.3k.10` (statement-initial fronted
  condition), `.3k.12` (a predicate between a signal and its level); `.3j` is open and unsized, `.3k.9` deliberately not next.
- In-flight uncommitted: none after this commit.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 238;
  directive 16 still gates it on full CI. `scripts/check_doctrines.sh --all` did not finish in 50 minutes —
  `CHAIN-CURRENCY` re-executes the real pipeline for every persisted artifact across four stages; budget hours and run
  it detached. Standing hazards, each with its procedure in a fact card: **an evidence-stage change that MOVES a
  current-schema artifact's content stales its proof and it then refuses to LOAD** — only AHB/AXI-L/APB-E are
  current-schema; rebuild `evidence → validate → semantic → validate → intent → validate → adapt`, each validated once,
  upstream-first, restoring the bundle from `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`,
  then `diff -rq`, remove, retention back to **24**. A corpus-root tool REFUSES an off-volume path — stage comparisons
  under `generated/tmp/`. **A census counts the population of the FUNCTION being changed**
  (`[[constraint-record-producer-strata]]`), **never from a mirror** (`[[one-modal-vocabulary-per-constraint-record]]`),
  and **a control that asserts a record EXISTS pins whatever that record says, fabrication included**
  (`[[a-relational-predicate-is-not-a-value]]`). **A Rust change moves `flow_census.json` and the re-derivation must
  ATTRIBUTE the delta to the owning leaf**; a new module moves `module_inventory.tsv`; a new command
  `CLI_SURFACE_REGISTRY`; a new raw-evidence reader `information_flow_boundary.tsv` — all fail closed. **Editing a live
  surface sets off a fixed bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`; the book's line-pinned
  regions re-anchor BY CONTENT, and the claim IDs that move ARE the `Published-claims:` declaration. **A book section
  about an extraction rule goes in its concern's chapter, and a changed rule leaves standing book text.** **The
  doctrine driver runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`); never run the fixture suite with the
  locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`).
