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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4d` — migrate
  `LIVE_ACHIEVEMENT_STATUS.md` while preserving its stable Rust-managed trailer.
- Current state: `.0`–`.4c` and `.5a` are complete. The fifth doctrine governs 560 Markdown files
  across 27 surfaces. `CHANGES.md` and `DEVELOPMENT_NOTES.md` are bounded 89/61-record current views
  over exact 1,798/1,601-record capsules; manifest, index, retrieval, and suffix identity are enforced.
  Status and Rust analysis remain measured planned transitions. Query revision from Git.
- Next action: freeze the remeasured 1,920-bullet status source, derive its 50-bullet root while
  preserving the entire gap/validation trailer and both managed markers, extend manifest/index and
  classifications, then run the real project-validation writer seam before committing `.4d`.
- In-flight uncommitted: `.4c` capsule/current-view migration and synchronized docs until commit;
  none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
