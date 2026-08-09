# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Derive the current repository revision on read with `git rev-parse HEAD`; never store a
  latest-commit shadow that the recording commit would immediately invalidate.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver for memory,
  knowledge-map, task-acceptance, README policy, and live-document containment); hooks + CI
  run it too. Retrieval starts at bounded `KNOWLEDGE_MAP.md`, then searches its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.9a` — bounded archive-route topology design; verified,
  commit pending.
- Current state: ADR 0017 selects a fixed four-route landing plus bounded per-ledger indexes/manifests. The shared
  index has 218 bytes before rollover versus a 301-byte last route. Per-ledger projections fit unchanged controls;
  the current checker does not yet prove complete acyclic predecessor/successor chronology. This task file is
  229,064 bytes / 82.2% of health with 21,611 bytes of pre-rollover headroom; `.10a`/`.10b` own containment.
- Next action: commit `.9a`, then activate `.9b` and atomically migrate every route/consumer while
  adding complete-chain, exact index-order, direct-retrieval, overflow, and retired-manifest residue checks.
- In-flight uncommitted: `.9a` design/current-truth documents only; no archive route/member, consumer, executable
  checker, product artifact, threshold, or ceiling has changed. `.project-data/tmp` contains only `.gitkeep` plus
  `xcrun_db`.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
