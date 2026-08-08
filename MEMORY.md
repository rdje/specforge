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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5f.i` — next clean-tree leaf after `.5e`
  is durable.
- Current state: `.0`–`.5e` are complete. `ROADMAP.md` is a 153-line bounded current view; the exact
  1,487-line source is a verified immutable capsule behind a bounded index/manifest. All 23 current
  rows and three roadmap lifecycle surfaces are mechanically enforced with no transition debt.
- Next action: commit `.5e.ii`, then activate `.5f.i`; measure and lock the FSMGen-feedback record
  grammar, direction/status semantics, consumers, source identity, live window, and archive protocol.
- In-flight uncommitted: `.5e.ii` roadmap/capsule/manifest, migrated contract, surface/readers, and
  live/book/task/fact synchronization until commit; none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
