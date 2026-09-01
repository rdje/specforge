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
- Next action: pick a new lane; the live extraction frontier is `KG-ISF-COMPLETENESS` beyond `.5`.
  `WIRE-BASED-100`'s `.8` is closed and its remaining leaves (`.5d` in_progress) are corpus-supply work.
  **Unowned and worth owning:** re-ingesting the legacy stratum is what would make the APB/AHB/AXI wire golds
  measurable again — the highest-leverage unblock for every wire claim. Do NOT assume the tracked corpus-refresh
  frontier covers it: `scripts/check_corpus_frontier.sh` reports a 57-document cohort at 52 refreshed / 5
  remaining (`den0034_a`, `lpc_memory_agent…`, `nvme_base_specification_2_0a`, and the two
  `opencapi_3_x_transaction_layer`), and none of the three wire golds is among them — so that frontier is a
  different population from the 54 legacy chains, and the wire re-ingest has no owning leaf yet.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **prove the oracle runs BEFORE trusting a green result**;
  **re-derive a published score before citing it**; **measure a proposed fix before building it** (`.8d`'s
  plausible scope rule fires on every SWD frame field and is wrong on all 11); and **when a change retires a
  producer, the stale surfaces are the ones publishing its NUMBER, not the ones describing its ARTIFACT**.
  Never name a stratum by one schema number — the legacy version differs per stage. Never infer ownership from
  a mention: read the owner's own `Status`. **A probe is not a port:** `.8e` had to correct three of `.8d`'s
  figures because the probe dropped the production gate's own `parse_count_word` rejection — publish from a
  committed derivation (`scripts/measure_swd_frame_phase_scope.py`), never an ad-hoc script.
  **A rescue number is not a rescue:** `.8f`'s permissive detector appears to recover 5 of 11 SWD
  frame fields and every hit is a false positive — read each hit before believing any.
  The full derived-state refresh chain an edit to `CHANGES.md`, the
  book, a fact card or `surfaces.jsonl` sets off — including the ASCII-only rollover `reason` and this file's
  50-line cap — is `[[live-surface-edit-bookkeeping-chain]]`. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite concurrently with the locality gate.
  `durability.stale_check` is never executed by any gate (`.18`).
