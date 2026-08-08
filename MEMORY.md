# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
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
- Active unit: `SWD-SERIAL-EXTRACTION.4e` — completed by this commit; Chapter B4 interface-edge timing is
  typed, source-grounded, and scored.
- Current state: fresh repository-local CPU re-ingest/evidence proves one complete target/SWDIO/SWCLK
  rising-edge record from `statement_1948`; all four SWD protocol tasks score source-tolerant 1.000 over
  29 facts, neighboring WIRE gates hold, and `kg-bench` is 156/156. `.7` owns the EvidenceIR→IntentIR/ISF
  projection gap. The live build also exposed project-wide persisted absolute stage pointers (335 local
  generated artifacts still name the deleted root), recorded in the task and Knowledge Map.
- Next action: from clean `.4e`, open a dedicated artifact-path-portability task-tree and fix the
  cross-stage serialization/migration contract before resuming `SWD-SERIAL-EXTRACTION.7`.
- In-flight uncommitted: none after this commit; no background job is running. Disposable re-ingest
  workspaces must be removed after the final producer and residue census.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
