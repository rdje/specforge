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
- Active unit: `SOURCE-IR-REPRODUCIBILITY.2` is the next child; `.0` and `.1` are done and `.4`/`.5` are new.
  Tracking-only: `SPEC-TO-INTENT-ALIGNMENT.9`, `PROVIDER-MODEL-STORE-LOCALITY.1`, `STATUS-LEDGER-ROLLOVER.2`,
  `TASK-PART-SEAL-REACHABILITY.0`, `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.1` censused every persisted `generated/source_ir/*` artifact at `085582c0`. The 78 partition
  into 24 live (schema 3 — exactly the chain-currency retained-bundle set) and 54 legacy that no current ingest
  can reproduce at all. All 24 live sources resolved on the repository volume, so the whole live stratum was
  re-ingested with no sampling inside it: 11 reproduce exactly and 13 do not, and the 13 hold 11,379 of the live
  population's 22,088 persisted content elements. Drift adds 1,804 elements (1,802 `body_text`, all
  figure-interior text) and drops 31 — 28 re-segmentation, 3 emitted nowhere. The decisive cost is captions:
  bindings fall 1,191 to 1,152, so re-ingesting is a trade, not a refresh. Not noise — the largest drift
  reproduced identically in two further runs. Producer self-test is 14/14 with two observed RED perturbations,
  nothing under `generated/` was written, and the book's over-broad reproducibility claim is corrected.
  `SOURCE-IR-REPRODUCIBILITY.5` now owns the re-ingest-versus-retain decision, which is the director's call.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.2` — make reviewed source anchors resolve by content identity
  rather than ordinal position, with a RED control proving an anchor that no longer matches its excerpt still
  fails closed. `.1` measured why this is now urgent rather than hypothetical: 13 live documents have moved
  ordinals, so every fixture pinned to an `elem_NNNNN` is a latent scoring failure. `.3` (chain-currency blind
  spot) and `.4` (bundle reproducibility fingerprint) follow; `.5` needs a decision, not a capability.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
