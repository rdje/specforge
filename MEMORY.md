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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.iii` — next clean-tree leaf; activate it
  only after the `.5c.ii` commit is durable.
- Current state: `.0`–`.5b` and `.5c.i`–`.5c.ii` are complete. The fifth doctrine governs 566 tracked
  Markdown files across 29 surfaces. Decisions retain a 9/9 bounded index. The new 150-line / 28,392-
  byte fact-card catalog links all 136 cards exactly once plus the collection README; nine focused
  cases and derive-and-diff run unconditionally. The generated question map has 137 cross-layer facts
  because ADR 0007 participates separately and remains transition debt for `.5d` sharding.
- Next action: activate `.5c.iii`, census research and the remaining `git:query` canonical
  collections, then add only complete bounded one-hop indexes without copying canonical prose.
- In-flight uncommitted: `.5c.ii` catalog, controls, and synchronized docs until commit; none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
