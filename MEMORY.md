# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.2g` CLOSED `2026-09-13` (CODE)** — **`replay-constraints` judged two of the three deterministic
  producers and said nothing about the third.** It now takes the document's own `SourceIr` and composes the table-row pass the way the build
  does: **171 → 183 persisted, 125 → 137 reproduced, not-reproduced unchanged at 46 — all 12 `row_sigcon_*` reproduce**, re-derived without a
  rebuild. **The half that had to be measured: a LEGACY `SourceIr` loads with every `table_kind` neutralized to `Unknown`**, so a pass keyed on
  `SignalDescription` selects NOTHING and its empty result is indistinguishable from "this document states none" — LTI marks 25 tables
  `signal_description` and passes 0 of 88 after a legacy load. Gated on the schema; the report publishes **26 judged / 51 not**.
  **That falsified two populations `.3k.2d` published one commit earlier** (`.3k.2e` "17 of 17 wrong", `.3k.2i` "4 right / 5 wrong"): both were
  mirrored off the PERSISTED `table_kind`, and the real producer mints 12 row records corpus-wide and not one more. Both nodes re-sized in place.
  Open: `.3k.2e`/`.3k.2f`/`.3k.2h`/`.3k.2i`, `.3k.3`/`.3k.4`/`.3k.5`; **`.3j`**; `INVARIANT-SHAPE-ADMISSION.4`; `PROSE-NAME-CELL-DECLARATION.3`;
  `SIGNAL-DECLARATION-ROW-DROP` `.2c`/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5`.
- Current state: eight commits. **The family's own instrument is now its sizing authority for all three deterministic producers, and the last
  two leaves each corrected a population the one before published.** The standing lesson is sharper than "measure first": measure with the
  PRODUCER. `.3k.2d` refuted a mechanism taken from a test string; `.3k.2g` refuted two counts taken from a persisted FIELD the producer no
  longer trusts. 306 fact cards; 15 doctrines, 13 at gate tier; core lib 1,464; corpus replay **137 of 183** (26 documents row-judged, 51 not).
- Next action: **`EXTRACTION-QUALITY-GAUGE.3k.2f`** — wire `.3d`'s equality and `.3k.1`'s magnitude refusals into the ROW path, which calls
  neither. Demonstrated with the real reader: a row cell saying *"must be equal to the value of …"* publishes `must_be_value`, from a sentence
  the statement path refuses outright. Corpus population is **0** — size it as a live class with an empty published population, `.3k.1`'s
  footing, not as a record count. Then `.3k.2e` (same footing, now re-sized to 0 actionable), `.3k.2h`, `.3k.2i`, `.3k.3`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **an evidence-stage change stales the proof of every current-schema artifact whose content moves and they
  then refuse to LOAD** — rebuild (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated exactly once,
  upstream-first). Only APB/AHB/AXI-L have held-out bundles (`generated/preserved/WIRE-BASED-100.10/`): restore, rebuild, `diff -r`, remove,
  retention back to 24. **Run `replay-constraints` before sizing any extractor change** (`[[persisted-census-measures-published-not-current]]`)
  — it now judges all three deterministic producers, but read `row_stratum_unjudged_documents`: 51 documents' classifications are neutralized on
  load, so a classification-keyed census over them measures a population the producer cannot reach
  (`[[legacy-source-classifications-are-neutralized-on-load]]`). **A census counts the population of the FUNCTION being changed**
  (`[[constraint-record-producer-strata]]`), **enumerate a phrase/modal set in full before publishing a zero**, and **never size from a mirror**
  (`[[one-modal-vocabulary-per-constraint-record]]`). **`extract_protocol_state_value` binds from the FIRST `must be ` in the whole text.**
  **A Rust change moves `flow_census.json` and the re-derivation must ATTRIBUTE the delta to the owning leaf**; a new module moves
  `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new raw-evidence reader `information_flow_boundary.tsv` — all fail closed.
  **Editing a live surface sets off a fixed bookkeeping chain** — `[[live-surface-edit-bookkeeping-chain]]`; deleting a governed book line also
  RETIRES its region record and lowers `expected_candidate_lines`. **The doctrine driver runs no cargo gate**
  (`[[doctrine-driver-runs-no-cargo-gate]]`). Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). Cap: 50 lines.
