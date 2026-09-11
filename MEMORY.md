# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.1b` CLOSED `2026-09-11`** — `.1`'s deferred cascade is executed. It was not a stale seal to tidy up: 4
  of the 27 proof-carrying documents (AXI, APB, AHB, ADIv6 — every wire-bearing one) could not be loaded at all, so `eval-extraction` refused every
  gold and WIRE-BASED-100 was unmeasurable for three commits. Open in this tree: `.1c`/`.2b`/`.2c`/`.2d`/`.3`. Also open:
  `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`;
  `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD
  `13/29`).
- Current state: **corpus repaired and every wire number re-derived.** 27/27 EvidenceIR artifacts load; `check_chain_currency.sh` reports every
  measurable artifact is exactly what the current binary produces (evidence 24/24/0, semantic·intent·isf 27/27/0, retention exactly 24); APB/AHB/AXI
  score `1.000` on constraint, relation and temporal, SWD re-derives `13/29` exactly, `kg-bench` 156/156. Two gates were silent through the outage and
  both are now owned: the doctrine driver runs **no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`, fixed in `.1a`) and the proof-seal probe
  runs **one document per distinct seal**, which is a sample of the per-document replay topology that actually broke (`.1c`). Census 27 measurable /
  51 legacy.
- Next action: `.2b`, the flow-arrow direction grammar — implemented and verified but NOT committed (parked at the scratchpad while `.1b` landed).
  Restore, re-run the gates, commit. Adjudicated: **83** arrow cells in direction-bearing `signal_description` columns over 78 persisted SourceIR, 2
  documents, 13 forms; **18 admit** under the mirror test, **65 fail closed**. `.2`'s prediction is **half falsified**: the four all-zero documents
  have no arrow cell, and the admitted set covers **5 of the 8** Avalon signals `WIRE-BASED-100.10f` waits on.
- In-flight uncommitted: `.2b`'s Rust change + controls, held at the session scratchpad.
- Blockers: none. Standing hazards: **a green score is evidence only about the facts its gold names** — AXI's six held while it lost 115 declarations,
  regained them, moved 134 → 25 → 21 actors, gained its whole AR side and lost 7 phantom ports. **Measure a candidate rule corpus-wide before
  shipping, and look at what it selects, not how many** — `.10b`'s shape rule had a 9-of-10 false-positive rate no count would show
  (`[[base-name-template-table-is-not-a-catalogue]]`). **Changing the CONTENT of an already-registered EvidenceIR rule field stales every persisted
  proof without moving the ruleset seal** (`[[evidence-rule-field-content-stales-every-proof]]`), and `rebuild_stage_cascade.sh` refuses a deliberate
  content delta by design — run the stages directly, one validate per artifact, upstream-to-downstream. **A relation can become a declaration**:
  `synthesize_directions_from_relations` is the one EvidenceIR path that mints a name no table declared
  (`[[alpha-variant-placeholder-is-not-a-wire]]`). **An identifier may bind to another only when the document says so twice** — ADR 0037 forbids the
  spelling guess (`[[document-stated-identifier-coreference]]`). **The book can carry a false current-behaviour claim for a month**; no gate covers
  behavioural prose. **A SourceIR classification rule may not read its neighbours** (`[[sourceir-classification-is-per-record]]`); **two readers of
  one datum must tokenize identically**; the wire golds' bundles are restored for a rebuild and returned byte-identical
  (`[[retained-bundle-population-is-frozen]]`). **Do not predict a re-derivation's verdict.** Read a gate's cohort rule before treating its ratio as
  coverage (`[[corpus-canonical-currency-and-ownership]]`); never infer ownership from a mention; attribute a regression from producer history, never
  a diff; never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets off
  `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines; a new fact card needs `check_fact_card_catalog.pl --print-plan` folded into
  the contract before `--write` runs; `durability.stale_check` + `validate_canonical_recovery_contract.py` are run by no gate
  (`LIVE-DOCUMENT-PRESSURE-HEADROOM.18`).
