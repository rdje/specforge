# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`WIRE-BASED-100.10b` CLOSED `2026-09-11`** — AXI's ISF interface no longer carries a name
  that names no wire. Open in this tree: `.10d`/`.10e`/`.10f` (all opened by `.10b`), `.4a`,
  `.2`/`.3`/`.4`/`.5`. Also open: `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`/`.2`/`.3`;
  `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **The cheap rule was falsified before it shipped, and that is the slice.** A base-name
  template table (AXI `table_0011`) and an alpha-variant placeholder (`AxLEN`) were putting 7 names into the
  ISF interface. The obvious suffix-shape detector selects 10 tables / 42 declarations corpus-wide and **9 of
  those tables are real** (SDC-600's per-component ports beside its wrappers'); the **mirror test** —
  hierarchical qualification re-declares the same port list one level up, a pattern never does — takes it to
  exactly 1 table / 6 declarations. AXI ports `295 → 288`, provenance `468 → 462`, actors `25 → 21`,
  constraints `53 → 44`, rules `133 → 110`, every lost fact carrying a withheld name as its subject. Six
  scored numbers unchanged; 1 of 27 chains moved. Census 27 measurable (34.6%) / 51 legacy.
- Next action: `WIRE-BASED-100.10d` — expand a recognised template's obligations over the prefixes that
  instantiate it (`VALID must be LOW during reset` is real for all 7 channels; 9 AXI constraints were dropped
  with the base name). Or `.10e`, `.4a`, `RETAINED-BUNDLE-POPULATION-FROZEN.1`.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **a green score is evidence only about the facts its gold
  names** — AXI's six held while it lost 115 declarations, regained them, moved 134 → 25 → 21 actors, gained
  its whole AR side and lost 7 phantom ports. **Measure a candidate rule over the whole corpus before
  shipping it, and look at what it selects, not just how many** — `.10b`'s shape rule had a 9-of-10
  false-positive rate that no count would have shown (`[[base-name-template-table-is-not-a-catalogue]]`).
  **A relation can become a declaration**: `synthesize_directions_from_relations` is the one EvidenceIR path
  that mints a name no table declared (`[[alpha-variant-placeholder-is-not-a-wire]]`). **A SourceIR
  classification rule may not read its neighbours** (`[[sourceir-classification-is-per-record]]`); **two
  readers of the same data must tokenize identically**; a producer change stales every persisted proof at
  and below its stage, repaired by `source_proof_migrate`, not a re-ingest, and the three wire golds' bundles
  are restored for the chain and returned byte-identical (`[[retained-bundle-population-is-frozen]]`).
  **Do not predict a re-derivation's verdict.** Read a gate's cohort rule before treating its ratio as
  coverage (`[[corpus-canonical-currency-and-ownership]]`). Never infer ownership from a mention; attribute a
  regression from producer history, never a diff; never run the fixture suite with the locality gate
  (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets off `[[live-surface-edit-bookkeeping-chain]]`;
  this file's cap is 50 lines. `durability.stale_check` is never executed by any gate
  (`LIVE-DOCUMENT-PRESSURE-HEADROOM.18`), and no gate runs `scripts/validate_canonical_recovery_contract.py`.
