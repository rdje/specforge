# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.1a` CLOSED `2026-09-11`** — `.1` shipped with
  `cargo clippy --all-targets -- -D warnings` RED on the line it added, having ticked a NO-REGRESSION
  box that cited clippy clean. Repaired; the clippy leg of `.1`'s box is withdrawn in place.
  Open in this tree: `.2`/`.3`. Also open: `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS`
  beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: **the doctrine enforcer runs no cargo gate at all.** `scripts/check_doctrines.sh`
  registers fourteen repository doctrines and invokes cargo nowhere; `fmt`/`clippy`/`test` live in
  `scripts/run_ci.sh` (`:30` runs clippy with `-D warnings`). Over `48def695` the full doctrine report
  reads all-PASS while two targets fail to compile under clippy — and unlike CI-tier `CHAIN-CURRENCY`,
  clippy does not even print `DEFER`, so nothing says it was skipped — a leaf citing a toolchain oracle must
  run it in-session (`[[doctrine-driver-runs-no-cargo-gate]]`). Census 27 measurable / 51 legacy.
- Next action: `[[SIGNAL-DECLARATION-ROW-DROP]]`.2, measured and ready to split — `.2a` metavariable name
  cells / `.2b` flow-arrow direction / `.2c` enumerated width set / `.2d` the actor-taxonomy gap. Over 78
  persisted SourceIR: **83** flow-arrow cells in direction-bearing `signal_description` columns (18 admit
  under a mirror test; 65 closed — 16 two-flow, 49 actors outside the taxonomy); **5** live metavariable
  name cells; **7** enumerated widths. Both affected documents are legacy, so no persisted chain moves.
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
