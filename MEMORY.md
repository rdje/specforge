# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`INVARIANT-SHAPE-ADMISSION.3` CLOSED `2026-09-12`** — an obligation in a table cell binds to the nominal
  **immediately before its modal**, so a signal-description row's description cell states a constraint on that row's signal only when the clause
  opens with the modal (`Must be valid when RVALID is asserted` — 1 clause) or names the signal itself (11). Pronoun heads (5) and other nominals (3)
  are refused with a stated reason. Rebuilt APB/AXI/AHB: EvidenceIR `signal_constraints` 74 → **86** (+12, −0), `.isf` rules 177 → 200, prose families
  unmoved (invariants 1,098; constraints 1,117). Open: **`INVARIANT-SHAPE-ADMISSION.5`** (the refusal half) / `.4` (a programme, not a slice);
  **`EXTRACTION-QUALITY-GAUGE.3i`** (opened here); `PROSE-NAME-CELL-DECLARATION.3`; `PRODUCTION-GRAPH-CENSUS-PIN` `.1`/`.2`;
  `SIGNAL-DECLARATION-ROW-DROP` `.2c`/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100`
  `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: **the statement paths were already reading these rows and getting three of them wrong**, which `.3`'s adjudication found and `.5`
  now owns: `HBURST_WIDTH must be 0 or 3` is published as `HBURST must_be_value 0` (`dyn_sigcon_0013`), same for `HPROT` (`0014`), and `HSELx`'s
  condition is lifted from the wrong sentence. `is_post_passive_binding_only_subject` would refuse exactly this but its **gate 2 exempts a table row**.
  **15 registered doctrines**, 13 at gate tier and 2 at CI tier (`PROOF-SEAL-TOTAL`, `CHAIN-CURRENCY`) — both of which see a per-document replay
  divergence that the sampled gate-tier `PROOF-SEAL-CURRENCY` cannot. Census 27 measurable / 51 legacy; production-graph census re-pinned a **4th**
  consecutive time (functions 2,401 → 2,407), which settles `PRODUCTION-GRAPH-CENSUS-PIN.1`'s open question against an exact pin.
- Next action: `INVARIANT-SHAPE-ADMISSION.5` — refuse a serialized-row constraint whose obligation clause heads with a nominal that is not the row's
  signal. **3 clauses, already adjudicated in full** (`HBURST`, `HPROT`, `WTAGUPDATE`); the predicate `obligation_subject` already exists, so the leaf
  is a placement decision — most likely lifting gate 2 of `is_post_passive_binding_only_subject` for the heads-a-different-nominal case only. Budget
  the AMBA rebuild: restore the held-out bundles from `generated/preserved/WIRE-BASED-100.10/` first.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **an obligation binds to the token immediately before its modal** — `HBURST_WIDTH must be …` is not about `HBURST`.
  **A subjectless clause defeats a subject scan by having no subject to find**: `collect_subject_signal_tokens(" Must be valid")` returns
  `Must`/`be`/`valid`, all identifiers, which suppresses the full-text fallback. **`specforge evidence` says only `path does not exist:
  …/normalized/<key>.md` — the bundles are in `generated/preserved/WIRE-BASED-100.10/`** (`[[evidence-rule-field-content-stales-every-proof]]`); `.3`
  wrongly concluded the chain was unrebuildable before grepping the question shards for it. **Grep `KNOWLEDGE_MAP.md` for the wall, not just for the
  feature.** **A leaf's own stated bar can be falsified by its population** — measure before trusting it. **A table row's subject lives in its header,
  which serialization throws away.** **A number that justifies a rule must be counted over the rule's whole population.** **A check that prints a
  number is not a check** (`PRODUCTION-GRAPH-CENSUS-PIN`). **The doctrine driver runs no cargo gate**
  (`[[doctrine-driver-runs-no-cargo-gate]]`) and `check_chain_currency.sh` is CI-tier, not gate-tier. An evidence-stage change stales every persisted
  proof whose content moves (`[[evidence-rule-field-content-stales-every-proof]]`); a semantic-stage change needs no bundle
  (`[[retained-chain-rebuild-order]]`). Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets
  off `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines.
