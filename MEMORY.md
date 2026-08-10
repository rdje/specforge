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
- Active unit: `CORPUS-COVERAGE.2`; `.2.44` OpenCAPI 4.0 32 Gbps PHY Mechanical refresh is complete.
- Current state: guarded CPU ingest and two validated cascades complete Source 262→212 diagram-label cleanup;
  Evidence 350→267 additionally removes 33 synthetic ToC enum statements; current authority removes five stale
  interfaces plus generic/false phase and gate projections. Final IntentIR has three actors / zero interfaces /
  three behaviors / 45 constraints / two assumptions. Adapter blocks honestly on no signals and removes the stale
  nine-port `.isf`; 65/65 remaining ISFs pass FSMGen strict. Stage census is 80/13/80/79; 44 refreshes are complete
  with 12 remaining. WIRE/I2C/SWD, KG 156/156, full CI 1,791/five ignored, mdBook, doctrines, path census, and
  locality pass.
- Next action: select and authenticate the smallest roadmap-aligned refresh #45 candidate, create its owned leaf,
  then run the guarded repository-local ingest and deterministic cascade.
- In-flight uncommitted: none. No background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
