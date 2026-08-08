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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5d.i` — next clean-tree leaf; activate it only
  after the `.5c.iii` commit is durable.
- Current state: `.0`–`.5c` are complete. The fifth doctrine governs 574 tracked Markdown files across
  30 surfaces. No partitioned canonical collection is query-only: six generated catalogs cover 241
  members, the task catalog covers 122, and the Knowledge Map bundle README covers four. The bounded
  catalog plane is 7 files / 314 lines / 49,305 bytes; external membership and all 53 common lifecycle
  fixtures run unconditionally. The question map remains `.5d` transition debt at 138 facts / 978 keys.
- Next action: activate `.5d.i`, lock the bounded landing-index and generated-shard contract, then
  prove collision, ordering, source identity, reader migration, and derive-and-diff before generation changes.
- In-flight uncommitted: `.5c.iii` catalog plane, external-membership control, and synchronized docs
  until commit; none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
