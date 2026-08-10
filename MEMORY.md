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
- Active unit: `CORPUS-COVERAGE.2.46a`; parenthetical `data`-head signal-authority repair exposed by refresh #46.
- Current state: three CPU ingests reproduce 40 pages / 54 visuals / 49 tables / 50 sections / 172 elements at
  53% peak sampled memory used. The first cascade removes stale `BDF`/`DL`, all generic phases/gates, and the lone
  relation, but `Vital Product Data (VPD)` still becomes a one-bit emitted port. Across 80 EvidenceIR artifacts,
  `data (ACRONYM)` produces exactly six width-one declarations: genuine `SDA`/`USDA`/`SDAH`/`SD`, all qualified
  by `serial` or `high-speed`, and false `VPD`/Wishbone `DO`, qualified by `product`/`output`.
- Next action: commit `.2.46a` ownership, implement the measured adjacent-qualifier gate with paired tests, replay
  OpenCAPI Discovery/Wishbone/I2C/I2S, then rebuild and sign off parent `.2.46`.
- In-flight uncommitted: fresh #46 generated chain, exact seven-file rollback, three-ingest identity evidence,
  and `.2.46a` task/roadmap/memory ownership. No background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
