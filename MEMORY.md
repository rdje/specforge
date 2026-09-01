# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `WIRE-BASED-100.8` — `.8a` (oracle restored), `.8b` (legacy stratum is a disposition, not an
  abort) and `.8c` (the retired SWD score, corrected everywhere) are COMPLETE, and `CHANGES-LEDGER-ROLLOVER.7`
  rolled the change ledger in `.8b`'s transaction. `.8d` is `deferred` with its consequence recorded, so
  **`WIRE-BASED-100.8` is CLOSED**. Also open: `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: the scoring oracle works, refuses nothing silently, and has already falsified two published
  things. **SWD is 5/29, not 29/29** (`89d8dee7` retired a frame extractor that recognised the protocol by
  NAME — `swdio`/`swclk` + a fixed `SerialFramePhase` enum — which ADR 0006 forbids, so the 29/29 never
  measured generic capability). **And the legacy stratum is not "schema 1":** censused, the 54 legacy documents
  are SourceIR 1 / EvidenceIR 2 / SemanticIR 1 / IntentIR 1 against 3/3/2/2 current, so ZERO schema-1
  EvidenceIRs exist and the shorthand was false of the artifact `eval-extraction` refuses. Only the 24
  rebuildable chains are scoreable; the APB/AHB/AXI `1.000`s are unverifiable until re-ingest.
- Next action: pick a new lane. `WIRE-BASED-100`'s `.8` is closed and its remaining leaves (`.5d` in_progress)
  are corpus-supply work; the live extraction frontier is `KG-ISF-COMPLETENESS` beyond `.5`, and the corpus
  refresh frontier (re-ingesting the 54 legacy chains) is what would make the APB/AHB/AXI wire golds measurable
  again — the single highest-leverage unblock for every wire claim.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **prove the oracle runs BEFORE trusting a green result**;
  **re-derive a published score before citing it**; **when a change retires a producer, the stale surfaces are
  the ones publishing its NUMBER, not the ones describing its ARTIFACT** (`89d8dee7` superseded four fact cards
  and four book chapters and still left the score in the roadmap, the book's eval chapter, two trees and three
  cards). New: **never name a stratum by one schema number** — the legacy version differs per stage, and that
  shorthand propagated through six surfaces including one I wrote hours earlier. A count read off a dump is not
  derived; two frames answering one question are two populations. Prepending to `CHANGES.md` shifts the
  line-pinned `current_claim_census.jsonl` regions and editing the book shifts `book_quantitative_claims.jsonl`
  ones (re-anchor by CONTENT, never offsets; register the new `CHANGES.md` line 1 as excluded evidence; a new
  book candidate line needs its own region record and `expected_candidate_lines`); a book line-count change
  stales `surfaces.jsonl` `shipped_behavior` aggregate authority, whose `rationale` is capped at 512 bytes;
  changing `surfaces.jsonl` stales three `surface_registry` source pins AND `claims.jsonl` digests; a new fact
  card moves `fact_card_catalog.json` `planned_outputs` AND the `fact-card-catalog-count` published assertion —
  refresh those last, then re-run the gate. A ledger rollover additionally RETIRES the census evidence records
  whose regions were sealed (the bytes live on in the segment) and needs one new record for the new line 1 —
  and **a rollover plan's `reason` must be pure ASCII**: the manifest writer emits without a UTF-8 layer, so one
  em dash fails the staged identity check and the whole transaction rolls back (`CHANGES-LEDGER-ROLLOVER.7`).
  Never infer ownership from a mention: read the owner's own `Status`.
  Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`;
  `CHANGES-LEDGER-ROLLOVER.4`; `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite concurrently with
  the locality gate. `durability.stale_check` is never executed by any gate (`.18`).
