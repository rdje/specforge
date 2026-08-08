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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.i` — rebuild the task-tree catalog as a
  bounded complete one-hop index without mirrored execution history.
- Current state: `.0`–`.4e`, `.5a`, and `.5b` are complete. The fifth doctrine governs 563 Markdown
  files across 29 surfaces. `.5b` decomposed every collection/projection and currentness dependency;
  the task catalog currently has 122 files, 117 unique links, four missing real trees, one author
  template, and an 18,932-byte widest row. The archive-index enforcement cap remains unchanged.
- Next action: define the concise task-row schema and derive a complete 121-real-tree catalog from
  task metadata, then add a fail-closed membership/width verifier before replacing the current index.
- In-flight uncommitted: `.5b` design/census, scratch-locality fact, and synchronized docs until commit;
  none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
