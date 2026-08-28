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
- Active unit: `STATUS-LEDGER-ROLLOVER.4`, then `SOURCE-IR-REPRODUCIBILITY.8` or `.9`. Open:
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`–`.10`/`.13`; `STATUS-LEDGER-ROLLOVER` `.2`/`.4`;
  `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`; `SCRATCH-RESIDUE-CONTAINMENT.1`. Tracking-only:
  `SCRATCH-RESIDUE-CONTAINMENT.1`, `SPEC-TO-INTENT-ALIGNMENT.9`, `PROVIDER-MODEL-STORE-LOCALITY.1`,
  `TASK-PART-SEAL-REACHABILITY.0`, `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.4a` re-derived the measurement `.3`/`.4` were sized on — the root held **70** records at a
  1,369.7-byte mean, not the published 64 / 1,605; `64` is this surface's record warning threshold (80 x 80%).
  The honest budget is 1,351.6 bytes once the 6,871-byte prologue/trailer overhead is charged, and **no
  tracked producer reports a rolling ledger's live record count**. `.3` then rolled the ledger: 26 records
  sealed as `segment-0011-2026-08-28.md`, root **70 -> 44 records / 102,748 -> 76,758 bytes** (66.75% of
  target), older members byte-identical. The post-cut record mean is still **1,588.3 bytes**, above budget,
  which a rollover cannot fix. `.4a` also repaired a stale assertion inside the claim registry itself. The
  standing ingest defect is unchanged and unfixed.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `STATUS-LEDGER-ROLLOVER.4` — check the derived budget (`(health_bytes - live-view
  overhead) / live_limits.records`) and publish the live record count, deriving both from the registry so
  neither can go stale. Then `SOURCE-IR-REPRODUCIBILITY.8`/`.9`, which both gate `.7`.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none. Owned, not fixed: `CLAIM-VERIFICATION-ADOPTION.7` (a published count does not re-derive),
  `.8` (census registry 109 of a declared 128, +1 per ledger-prepending slice),
  `SOURCE-IR-REPRODUCIBILITY.13` (frozen reviewed fixture not re-derivable), `docs/research/*.md` 63 of 64
  files and this pointer 44 of 50 lines (`LIVE-DOCUMENT-PRESSURE-HEADROOM.4`).
