# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.2c` CLOSED `2026-09-12` (CODE)** — a specification writes *"must have the same value"* when it
  means *"does not change"*. APB states it that way and the phrase table had no form of it, so 4 `row_sigcon_*` obligations fell to the untyped
  fallback and were published as `must_be_stable` by accident. Typed as **`MustNotChange`**, and **placed BEHIND the validity arm**: a
  signal-description cell routinely states both, the first matching arm types the whole record, and ahead of validity this phrase retyped APB
  `sigcon_0009`/`0010` out of `must_be_value VALID`. Both the phrase and its position carry their own RED control. Measured: **4 retyped, 0
  added, 0 removed, one document**. `.3k.2` is now `.3k.2a`-`.3k.2d`, with `.3k.2d` the only one open.
  Open: `.3k.2d`/`.3k.3`/`.3k.4`/`.3k.5`; **`.3j`**; `INVARIANT-SHAPE-ADMISSION.4`; `PROSE-NAME-CELL-DECLARATION.3`;
  `SIGNAL-DECLARATION-ROW-DROP` `.2c`/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: six commits. The family's method is settled and is worth keeping: size with `replay-constraints`, READ every record in the
  actionable population, let the reading choose the rule, and route what the reading finds into its own leaf. Every leaf so far has corrected a
  number a previous one published, including its own. **Arm ORDER in `classify_signal_constraint_kind` is now load-bearing** — the first match
  types the whole record and a table cell states several obligations — so a new phrase needs a placement control, not just a membership one.
  281 fact cards; 15 doctrines, 13 at gate tier; core lib 1,456; corpus replay 125 of 171.
- Next action: **`EXTRACTION-QUALITY-GAUGE.3k.2d`** — a NEGATED value binding is unreadable.
  `extract_protocol_state_value` binds on `must be `/`shall be `/`must remain `/`shall remain ` and has no negated form, so *"The DV operand
  must not be 1 for IODIR"* matches nothing, falls to the untyped default, and since `.3k.2a` publishes nothing at all. The vocabulary already
  has the slot (`MustBeValue` + `negated`); re-check the `WIRE-BASED-100.5b` double-negative guard, which reserves `negated` for kinds whose
  plain form is affirmative. Derive the population from the NOT-REPRODUCED set with `replay-constraints`. Then `.3k.3` (the kind's span — its
  published 4 predate three rebuilds, and it also inherits the `WTAGUPDATE` value-span residual), `.3k.5`, `.3k.4`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **an evidence-stage change stales the proof of every current-schema artifact whose content moves and they
  then refuse to LOAD** — rebuild (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated exactly once,
  upstream-first). Only APB/AHB/AXI-L have held-out bundles (`generated/preserved/WIRE-BASED-100.10/`): restore, rebuild, `diff -r`, remove,
  retention back to 24. **Run `replay-constraints` before sizing any extractor change**
  (`[[persisted-census-measures-published-not-current]]`). **A census counts the population of the FUNCTION being changed**
  (`[[constraint-record-producer-strata]]`). **`extract_protocol_state_value` binds from the FIRST `must be ` in the whole text.** **Both
  deterministic paths derive their own catalog from the statements.** **A Rust change moves `flow_census.json`; a new module
  `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new raw-evidence reader `information_flow_boundary.tsv`** — all fail closed.
  **A new Markdown file must be `git add`ed before `check_live_document_size.pl` sees it.** **Grep `KNOWLEDGE_MAP.md` for the wall** —
  `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`). Never run
  the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
