# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.2a` CLOSED `2026-09-11`** — a bracketed metavariable
  (`<name> _in`) is no longer a declaration; two phantom signals (`name`, `any`) stop being minted.
  `.1a` before it repaired the clippy gate `.1` left red. Open in this tree: `.2b`/`.2c`/`.2d`/`.3`.
  Also open: `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: **two slices, both from adjudicating a rule's selection before shipping it.** Grading
  `.2b`'s 18 admitted rows by hand found that 2 of the 7 it newly recovers are template metavariables —
  `.2a` landed first so `.2b` cannot mint `name` twice with opposite senses. Separately, the doctrine
  enforcer **runs no cargo gate at all**: `check_doctrines.sh` registers fourteen doctrines and invokes
  cargo nowhere, so `48def695` read all-PASS while clippy failed to compile two targets, and unlike
  CI-tier `CHAIN-CURRENCY` clippy does not even print `DEFER` (`[[doctrine-driver-runs-no-cargo-gate]]`).
  Census 27 measurable / 51 legacy.
- Next action: `.2b`, the flow-arrow direction grammar — population already measured and adjudicated by
  `python3 scripts/measure_declaration_row_notations.py`: **83** arrow cells in direction-bearing
  `signal_description` columns of 78 persisted SourceIR, 2 documents, 13 distinct forms; **18 admit**
  under the mirror test (both sides resolve AND agree), **65 fail closed** (16 two-flow, 49 actors
  outside the taxonomy → `.2d`). `.2`'s prediction is **half falsified before implementation**: the four
  all-zero documents contain no arrow cell at all, and the admitted set covers **5 of the 8** Avalon
  signals `WIRE-BASED-100.10f` waits on — `CHANNEL`/`DATA`/`ERROR` are not among them.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **a green score is evidence only about the facts its gold names** — AXI's six held while it lost 115 declarations,
  regained them, moved 134 → 25 → 21 actors, gained its whole AR side and lost 7 phantom ports. **Measure a candidate rule corpus-wide before shipping,
  and look at what it selects, not how many** — `.10b`'s shape rule had a 9-of-10 false-positive rate no count would show
  (`[[base-name-template-table-is-not-a-catalogue]]`). **A relation can become a declaration**: `synthesize_directions_from_relations` is the one
  EvidenceIR path that mints a name no table declared (`[[alpha-variant-placeholder-is-not-a-wire]]`). **An identifier may bind to another only when the
  document says so twice** — ADR 0037 forbids the spelling guess (`[[document-stated-identifier-coreference]]`). **The book can carry a false
  current-behaviour claim for a month**: the temporal chapter still described a resolver deleted on `2026-08-12`, and no gate covers behavioural prose.
  **A SourceIR classification rule may not read its neighbours** (`[[sourceir-classification-is-per-record]]`); **two readers of one datum must tokenize
  identically**; a producer change stales every persisted proof at and below its stage, repaired by `source_proof_migrate`, not a re-ingest, and the wire
  golds' bundles are restored for the chain and returned byte-identical (`[[retained-bundle-population-is-frozen]]`). **Do not predict a re-derivation's
  verdict.** Read a gate's cohort rule before treating its ratio as coverage (`[[corpus-canonical-currency-and-ownership]]`); never infer ownership from a
  mention; attribute a regression from producer history, never a diff; never run the fixture suite with the locality gate
  (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets off `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines; a new fact card
  needs `check_fact_card_catalog.pl --print-plan` folded into the contract before `--write` runs; `durability.stale_check` +
  `validate_canonical_recovery_contract.py` are run by no gate (`LIVE-DOCUMENT-PRESSURE-HEADROOM.18`).
