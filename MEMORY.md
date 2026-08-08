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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.ii` — close direct decision/fact-card
  navigation and compose it with question-key retrieval.
- Current state: `.0`–`.5b` and `.5c.i` are complete. The fifth doctrine governs 564 Markdown files
  across 29 surfaces. `docs/TASK_TREE.md` is a derived 121-real-tree catalog at 375 lines / 27,228
  bytes / 207 max-line bytes; the template is separate, every task is linked once, and seven
  fail-closed cases plus derive-and-diff run unconditionally. Task-index transition debt is closed.
- Next action: reprove decision-index membership, design the bounded human fact-card index against
  136 cards and generated question-key routes, then implement only the direct-navigation layer.
- In-flight uncommitted: `.5c.i` generator, catalog migration, transition ratchet, and synchronized
  docs until commit; none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
