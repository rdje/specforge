# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `WIRE-BASED-100.8` — `.8a` is COMPLETE (the scoring oracle runs again). Open children:
  `.8c` (the stale SWD score the restored oracle exposed) then `.8b` (legacy stratum → UNMEASURABLE
  disposition). Also open: `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **the oracle is back, and it immediately falsified a published number.** An EvidenceIR's proof
  is taken over its public fields and `artifact_layout` is one of them, so the proof bound the artifact's storage
  path and every relocated copy was refused — which is what `eval-extraction` must do to leave the corpus
  untouched. Relocation is now a supported, re-proving operation. First re-derivation since `2026-08-09`
  (`--provider skip`): I2C declared-signal 1.000 (6/6); SWD constraint 1.000, relation 1.000,
  protocol_operation 4/4, interface_edge_timing 1/1 — **but serial_frame_field 0/11 and protocol_state 0/13,
  i.e. 5/29 against a published 29/29.** Cause identified and NOT a regression: `89d8dee7` (`2026-08-12`)
  retired the fixed-phase frame and named-operation carriers on ADR-0006 genericity grounds and said so in its
  own ledger entry. Only the 24 rebuildable chains are scoreable; the 54 legacy schema-1 chains (APB/AHB/AXI
  golds) are still refused.
- Next action: `WIRE-BASED-100.8c` — correct every live surface that still presents SWD `29/29` as current
  (`WIRE-BASED-100.5j`, `SWD-SERIAL-EXTRACTION`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, book
  `quality/extraction-eval.md` §SWD protocol facts), keeping the retired number as dated history, and give the
  residual recall frontier an owning leaf or an explicit deferral. Then `.8b`.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards, unchanged: **prove the oracle runs BEFORE trusting a green
  result from it** — and now also **re-derive a published score before citing it**, because a genericity trade
  can retire a number that no live surface then corrects. Two frames answering the same question are still two
  populations — label which one a number came from; a count read off a dump is not derived. Prepending to
  `CHANGES.md` shifts the line-pinned `current_claim_census.jsonl` regions (re-anchor by CONTENT, never
  offsets); editing `CHANGES.md`/`MEMORY.md` stales `durability.artifacts` digests in `claims.jsonl`; a new fact
  card moves `fact_card_catalog.json` `planned_outputs` plus the `fact-card-catalog-count` assertion — refresh
  those last. Never infer ownership from a mention: read the owner's own `Status`. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `CHANGES-LEDGER-ROLLOVER.4`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite concurrently with the locality gate.
  `durability.stale_check` is never executed by any gate (`.18`).
