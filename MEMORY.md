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
  knowledge-map, task-acceptance, and README policy); hooks + CI run it too. Retrieval:
  `KNOWLEDGE_MAP.md`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3` — adopt the complete
  live-document containment doctrine and resulting-tree surface registry/checker.
- Current state: `.0`–`.2` are complete. README is a 127-line/4,834-byte landing page,
  guarded at 150 lines/5,800 bytes with reader/author route closure. No historical live
  document has yet been deleted or migrated. Query revision from Git.
- Next action: split `.3` into registry, checker, and fail-closed proof leaves; adopt the
  neutral doctrine and record complete measured transition debt without widening it.
- In-flight uncommitted: `.2` until its verified commit; none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
