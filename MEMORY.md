# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `KG-ISF-COMPLETENESS` — `.5` is COMPLETE (`.5.i` name gate, `.5.ii` spine gate, `.5.iii`
  `_WIDTH` gate, `.5.iv` measurement, `.5.iv.a` header source). Newly opened and now the lane's blocker:
  `WIRE-BASED-100.8`. Also open: `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/
  `.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`;
  `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`;
  `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **the corpus, not the code, is what limits extraction work now.** 54 of 78 persisted chains
  are legacy schema 1 — the current binary refuses them for canonical use — including the APB/AHB/AXI wire
  golds and NVMe/RISC-V register golds; only 24 are rebuildable, and only SWD/ADI + I2C among the scored
  documents. So `.5.iv.a` ships a real capability that reaches 9 documents and changes nothing on disk
  today: exactly 1 of its 285 accepted tables is in a rebuildable document, and that one mints nothing.
  Worse, `WIRE-BASED-100.8`: `eval-extraction` refuses EVERY document (pre-change binary too), because an
  EvidenceIR proof binds the artifact's own `artifact_layout` and `extract_on_copy` must relocate — so no
  WIRE-BASED-100 number can be re-derived at all.
- Next action: `WIRE-BASED-100.8` — restore the scoring oracle before any further extraction slice, since
  every such slice's mandated before/after gate runs through it. Split it if the proof-kernel fix and the
  `eval-extraction` fix are separately reviewable.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work, one that weakens evidence — **the gate a slice is told to run may itself be
  down; prove the oracle runs BEFORE trusting a green result from it.** `.5.iv.a` also shows a measurement
  can be right and its scope wrong: `.5.iv` censused `table_kind == encoding`, but the shipped scan visits
  `unknown` too, so the population was 2x and its four "junk classes" missed the dominant one (positional
  headers) and the whole glossary class. **Re-derive the population from the SHIPPED path, never from the
  earlier census.** And a named exclusion may be unnecessary: `RESERVED`-only self-eliminates in
  `build_symbol_definitions`' conflicting-value rule, so it did not ship — measure downstream before gating
  upstream. Prepending to `CHANGES.md` shifts the line-pinned `current_claim_census.jsonl` regions
  (re-anchor by CONTENT, never offsets); editing `CHANGES.md`/`MEMORY.md` stales `durability.artifacts`
  digests in `claims.jsonl`; a new fact card moves `fact_card_catalog.json` `planned_outputs` plus the
  `fact-card-catalog-count` assertion — refresh those last. Never infer ownership from a mention: read the
  owner's own `Status`. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/
  `.9`/`.12`/`.13`; `CHANGES-LEDGER-ROLLOVER.4`; `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture
  suite concurrently with the locality gate. `durability.stale_check` is never executed by any gate (`.18`).
