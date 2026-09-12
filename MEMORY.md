# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`PROSE-NAME-CELL-DECLARATION.2` + `PRODUCTION-GRAPH-CENSUS-PIN.0` CLOSED `2026-09-12`.** The two phantom AHB declarations `.0` found
  were a **column** defect, not a name-reader one: `table_0004` is rotated, has two body rows, and `Clock source` scored exactly like `HCLK`, so no
  margin could separate them. Shipped `name_cell_is_read_whole` — a cell scores for its column only when the reader consumes it whole. AHB now
  declares `HCLK`/`HRESETn`; current-stratum phrase declarations **2 → 0**; IntentIR interface signals **40 → 41**, nothing removed. Open in this
  tree: `.1`, `.3`, `.4`. Also open: `PRODUCTION-GRAPH-CENSUS-PIN` `.1`/`.2`; `SIGNAL-DECLARATION-ROW-DROP.2d`;
  `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`;
  `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5`
  (SWD `13/29`).
- Current state: **`.2`'s own proposed rule was falsified by its census before any code was written** — a ratio margin fixes nothing and breaks AHB
  `table_0034`; that is written into the tree so it is not proposed again. Chain fully current after the AHB rebuild: evidence 24/24, semantic
  27/27, intent 27/27, isf-adapter 27/27, retention exactly the declared 24. **A second, unrelated defect surfaced and is now owned**: four
  repository-wide census pins in `tools/production-genericity-graph` had drifted **+18 / +179 / +226 / +3** at `4bb6c1c8`, because the gate-tier
  doctrine that derives them every commit only *prints* them and the one test that compares them runs before a push. Re-pinned; the gap is
  `PRODUCTION-GRAPH-CENSUS-PIN.1`. **15 registered doctrines**, 13 at gate tier. Census 27 measurable / 51 legacy.
- Next action: `PROSE-NAME-CELL-DECLARATION.1` — the phrase refusal rule, **re-measured after `.2`**. Its current-stratum population is now 0, so its
  justification is no longer those two rows: it is the guard `SIGNAL-DECLARATION-ROW-DROP.2c` waits on, plus 11 legacy phrase declarations a
  re-ingest would reproduce. The leaf is explicitly allowed to conclude the column score already covers the case and close **without** a rule; do not
  ship one to fill the slot.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **a check that prints a number is not a check** — it permits every drift, and one did, over the 29 commits since
  `0d4860ba` (`PRODUCTION-GRAPH-CENSUS-PIN`). **A mirror of a producer must move with the producer**: when `.2` changed the reader, the census script's join
  rate fell to 602/604 and that is how it reported its own drift (`[[declared-population-is-not-the-candidate-row-population]]`). **A filter over source rows
  is not a measurement of what the reader produced** — 318 candidate rows, 17 declarations. **Do not A/B a gold across a producer change**: the seal refuses
  the pre-change artifact by design, so the evidence is the artifact diff plus the chain oracle. **A green score is evidence only about the facts its gold
  names** — `seed_ahb` names none of the four signals `.2` moved. **A green GATE can be evidence about one sampled document**
  (`[[evidence-rule-field-content-stales-every-proof]]`); **a persisted legacy artifact is evidence about the producer that wrote it**
  (`[[declared-spelling-is-the-document-spelling]]`) — never sum the two strata. **Measure a candidate rule corpus-wide before shipping, and look at what it
  selects** (`[[base-name-template-table-is-not-a-catalogue]]`). A producer change stales the seals of the documents it touches; restore the bundle from
  `generated/preserved/`, rebuild in the interleaved order with exactly one validate per artifact, and return it byte-identical
  (`[[retained-chain-rebuild-order]]`, `[[retained-bundle-population-is-frozen]]`). **The doctrine driver runs no cargo gate**
  (`[[doctrine-driver-runs-no-cargo-gate]]`). **Re-anchor a digest-bound region by its TEXT, never its line number.** **A relation can become a declaration**
  (`[[alpha-variant-placeholder-is-not-a-wire]]`). **The book can carry a false current-behaviour claim for a month**; no gate covers behavioural prose. **A
  SourceIR classification rule may not read its neighbours** (`[[sourceir-classification-is-per-record]]`). **Do not predict a re-derivation's verdict.**
  Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets off
  `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines.
