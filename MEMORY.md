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
- Active unit: `CORPUS-COVERAGE.2`; current-binary corpus refresh program, with slice `.2.45` complete.
- Current state: guarded CPU ingest and two validated cascades for `.2.45` are complete. Source 231→224 removes diagram labels;
  Evidence 395→385 additionally removes only three synthetic acronym declarations while all 70 physical timing
  constraints hold. Current authority removes two stale interfaces plus false/generic phase and gate projections.
  Final IntentIR has four actors / zero interfaces / four behaviors / 27 constraints / two assumptions / 70
  timing constraints. Adapter blocks honestly on no signals and removes the stale three-port `.isf`; 64/64
  remaining ISFs pass FSMGen strict. Stage census is 80/14/80/79; 45 refreshes are complete with 11 remaining.
  WIRE/I2C/SWD, KG 156/156, full CI 1,791/five ignored, mdBook, six doctrines, paths, and locality pass.
- Next action: select and authenticate refresh #46 from the 11 remaining real chip-spec candidates, create its
  owning leaf from a clean repository, and continue the one-document guarded-cascade workflow.
- In-flight uncommitted: none after the `.2.45` recording commit; no background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
