# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.2a` CLOSED `2026-09-12` (CODE + full rebuild)** — the terminal `MustBeStable` was not a default,
  it was a fabrication. **17 of 17 reproduced records on that arm were wrong**: a barrier description, four `… is not present` table cells, a
  tied-low level statement, two explicitly non-required recommendations, a response-duration sentence, and six waveform narrations. Refused in
  the STATEMENT path only (`classify_signal_constraint_kind_typed` → `None`); the ROW path keeps the fallback because `obligation_subject` has
  already proved its clause binds to its row, so its 4 records (`PAUSER must have the same value …`) are correct and merely under-typed.
  **Chain rebuilt for all three documents whose proofs the change stales — APB, AHB, AXI-L, all with held-out bundles: 8 removed, 0 added, 0
  retyped.** kg-bench 156/156; WIRE-BASED-100 golds P=R=F1=1.000 ×4. `.3k.2` split into `.3k.2a` (done), `.3k.2b`, `.3k.2c`.
  Open: `.3k.2b`/`.3k.2c`/`.3k.3`/`.3k.4`/`.3k.5`; **`.3j`**; `INVARIANT-SHAPE-ADMISSION.4`; `PROSE-NAME-CELL-DECLARATION.3`;
  `SIGNAL-DECLARATION-ROW-DROP` `.2c`/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: four commits, and the method finally closed on itself. `.3k` stratified a census by PRODUCER; `.3k.1` found a persisted census
  measures what was PUBLISHED; `.3k.6` built `replay-constraints` for the difference; `.3k.2a` is the first leaf sized with it BEFORE coding —
  published 26, actionable 17, and reading all 17 is what produced the design (refuse in one path, keep in the other) plus two spun-off leaves.
  Corpus replay now **127 of 171** reproduce. 281 fact cards (6 title parts, at `max_parts`); 15 doctrines, 13 at gate tier; core lib 1,449.
- Next action: **`EXTRACTION-QUALITY-GAUGE.3k.2b`** — gate the ungated `generic_value` arm. `extract_protocol_state_value` publishes the first
  non-filler word after `must be `/`shall be ` as a typed value with no check that it IS one. Population 4 reproduced `sigcon_*` and it is
  **2-2**: right — AXI `AWTAGOP must_be_value INVALID` (`must be Invalid`, a real TagOp member) and HBM2 `CKE must_be_value LOW` (`must be held
  LOW`); wrong — RISC-V IOMMU `GSCID` and DTI `DO_NOT_CACHE`, both `INVALIDATED` lifted out of `must be invalidated`, a VERB. So GATE it against
  the document's discovered enum values and the logic levels, never refuse it. Then `.3k.2c` (the `must have the same value …` spelling, 4 row
  records), `.3k.3` (the kind's span), `.3k.5` (refusal-gate span), `.3k.4` (the dynamic path's clause).
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **an evidence-stage change stales the proof of every current-schema artifact whose content moves, and they
  then refuse to LOAD** — rebuild them (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated exactly once,
  upstream-first) or they drop out of every measurement. Only APB/AHB/AXI-L have held-out bundles (`generated/preserved/WIRE-BASED-100.10/`);
  restore, rebuild, `diff -r`, remove, retention back to 24. **Run `replay-constraints` before sizing any extractor change**
  (`[[persisted-census-measures-published-not-current]]`). **A census counts the population of the FUNCTION being changed**
  (`[[constraint-record-producer-strata]]`). **Both deterministic paths derive their own catalog from the statements.** **A Rust change moves
  `flow_census.json`; a new module moves `module_inventory.tsv`; a new command moves `CLI_SURFACE_REGISTRY` in `commands/converge.rs`; a new
  raw-evidence reader moves `information_flow_boundary.tsv`** — all fail closed. **A new Markdown file must be `git add`ed before
  `check_live_document_size.pl` sees it.** **Grep `KNOWLEDGE_MAP.md` for the wall** — `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine
  driver runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`). Never run the fixture suite with the locality gate
  (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
