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
- Active unit: `CORPUS-COVERAGE.2.34b.ii.a`; make fresh-ingest page sidecars path-portable.
- Current state: the repository-relative host-library route now resolves to the director-supplied SSD checkout.
  Guarded CPU ingest produced the expected USB4 semantic cleanup, but the locality gate found all 51 persisted
  page sidecars still named the temporary absolute `normalized.staging` image paths. The complete fresh chain is
  hash-verified in project-local task storage and the canonical USB4 chain is restored to its authenticated
  seven-file baseline. `.2.34b.i` separately restored coherent canonical SWD with all 29 serial facts exact.
- Next action: implement `.ii.a` at the staged-bundle producer seam with focused fail-closed tests, then `.ii.b`
  reruns guarded USB4 ingest/cascade and completes rollback comparison, gates, cleanup, and corpus closure.
- In-flight uncommitted: none after the `.2.34b.i` recovery commit. The authenticated USB4 baseline and exact
  unportable fresh-chain evidence remain under project-local ignored task work for `.ii.a`/`.ii.b`; the SWD
  recovery snapshot has been deleted.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
