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
  run it too. Retrieval: `KNOWLEDGE_MAP.md`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5d.ii` — next clean-tree leaf; activate it only
  after the `.5d.i` commit is durable.
- Current state: `.0`–`.5c` and `.5d.i` are complete. ADR 0009 plus the executable shard contract lock
  one bounded Knowledge Map landing and deterministic direct-link question shards. The current 139
  facts / 980 unique keys fit in six simulated shards; eight focused and 55 common lifecycle cases run
  unconditionally. The live generator/topology remains current `.5d` transition debt until `.5d.ii`.
- Next action: activate `.5d.ii`; atomically migrate the portable generator/checker, hook staging,
  complete root/shard membership, obsolete-shard cleanup, doctrine surface, and every reader literal.
- In-flight uncommitted: `.5d.i` contract, collision correction, and synchronized docs until commit;
  none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
