# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey. History lives in
> `git log`; work state lives in the task-trees (`docs/tasks/`); durable facts/decisions live in
> `docs/decisions/`. Do **not** append session narration — overwrite the "Current state" block.

## How to resume (any AI, any harness)
- Derive the current revision on read with `git rev-parse HEAD`; never store a latest-commit shadow.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (doctrines are mechanically
  gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`); follow `COMMIT.md`
  after every slice (unit id in the commit subject).
- Non-negotiable doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md` (no code change without
  an owning task-tree first; signoff quality; zero ROADMAP↔code↔mdBook drift; push ~every 200 commits;
  artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh`; hooks + CI run it too. Retrieval starts at bounded
  `KNOWLEDGE_MAP.md`, then its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SCRATCH-RESIDUE-CONTAINMENT.1` and `SOURCE-IR-REPRODUCIBILITY.2` are both next; `.0` of the
  former and `.0`/`.1` of the latter are done. Tracking-only: `SPEC-TO-INTENT-ALIGNMENT.9`,
  `PROVIDER-MODEL-STORE-LOCALITY.1`, `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `SOURCE-IR-REPRODUCIBILITY.1` censused all 78 persisted `generated/source_ir/*` artifacts at
  `085582c0` — 24 live (schema 3, exactly the chain-currency retained-bundle set) and 54 legacy no ingest can
  reproduce. All 24 live sources resolved on the repository volume, so the whole stratum was re-ingested: 11
  reproduce exactly and 13 do not, holding 11,379 of 22,088 persisted content elements. Drift adds 1,804
  elements (all figure-interior) and drops 31 — 28 re-segmentation, 3 emitted nowhere — and caption bindings
  fall 1,191 to 1,152, so re-ingesting is a trade, not a refresh (`.5` owns that call). Not noise: the largest
  drift reproduced identically twice more. `SCRATCH-RESIDUE-CONTAINMENT.0` then established named-or-chained
  scratch reachability and reclaimed 5,290 files / 2.55 GiB with an empty residue, retaining the two
  chain-bearing behavioral-holdout roots (14.1 GiB) for `.1` to decide rather than deleting them on a sweep.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.2` — make reviewed source anchors resolve by content identity
  rather than ordinal position, with a RED control proving an anchor that no longer matches its excerpt still
  fails closed; 13 live documents have moved ordinals, so every `elem_NNNNN` pin is a latent scoring failure.
  `SCRATCH-RESIDUE-CONTAINMENT.1` (retain-or-reclaim the 14.1 GiB of holdout evidence) is the cheaper
  alternative pick and needs a decision rather than a capability.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
