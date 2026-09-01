# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `WIRE-BASED-100.8` — `.8a` (oracle restored) and `.8c` (the retired SWD score, corrected
  everywhere) are COMPLETE. Open children: `.8b` (legacy stratum → UNMEASURABLE disposition instead of a
  whole-run abort) then `.8d` (recover frame fields generically by phase-scope binding). Also open:
  `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: the scoring oracle runs again (relocation is now a proof-carrying operation), and its first
  run retired a published score. **SWD is 5/29, not 29/29** — operations 4/4 and edge timing 1/1 hold, frame
  fields 0/11 and states 0/13 do not — because `89d8dee7` (`2026-08-12`) removed a frame extractor that
  recognised the protocol by NAME (`swdio`/`swclk`/`packet request` + a fixed `SerialFramePhase` enum), which
  ADR 0006 forbids. So the 29/29 never measured generic capability. Corrected on every surface, retired number
  kept as dated history. Only the 24 rebuildable chains are scoreable; the 54 legacy schema-1 chains (APB/AHB/AXI
  golds) are refused, so those `1.000`s are also unverifiable today.
- Next action: `WIRE-BASED-100.8b` — stop one legacy document aborting a whole `eval-extraction` run; report it
  as UNMEASURABLE with its re-ingest route and withhold its gold items from scoring, while any OTHER failure
  still aborts. Then `.8d`.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **prove the oracle runs BEFORE trusting a green result**, and
  **re-derive a published score before citing it**. New, and general: **when a change retires a producer, the
  surfaces that go stale are the ones publishing its NUMBER, not the ones describing its ARTIFACT** — `89d8dee7`
  superseded four fact cards and four book chapters and still left the score standing in the roadmap, the
  extraction-eval chapter, two trees and three cards. Two frames answering the same question are still two
  populations; a count read off a dump is not derived. Prepending to `CHANGES.md` shifts the line-pinned
  `current_claim_census.jsonl` regions and editing the book shifts `book_quantitative_claims.jsonl` ones
  (re-anchor by CONTENT, never offsets, and register the new `CHANGES.md` line 1 as excluded evidence); a book
  line-count change also stales `surfaces.jsonl` `shipped_behavior` aggregate authority, whose `rationale` is
  capped at 512 bytes; changing `surfaces.jsonl` stales three separate `surface_registry` source pins AND
  `claims.jsonl` digests — refresh those last, then re-run the gate. Never infer ownership from a mention: read
  the owner's own `Status`. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`;
  `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `CHANGES-LEDGER-ROLLOVER.4`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite concurrently with the locality gate.
  `durability.stale_check` is never executed by any gate (`.18`).
