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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4a` — measure the four rolling-ledger
  grammars and consumers, then lock one shared lossless rollover/archive protocol.
- Current state: `.0`–`.3c` and `.5a` are complete. The fifth doctrine governs all 555 Markdown files
  across 24 surfaces; 48 same-volume fixtures prove every local lifecycle/control path, and the
  mdBook currency verifier binds two repaired current claims to code. README is 128 lines/4,908 bytes.
  No historical live document has yet been deleted or migrated. Query revision from Git.
- Next action: census whole-record boundaries, readers/writers, and cross-links for `CHANGES.md`,
  `DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and `RUST_CODEBASE_ANALYSIS.md`.
- In-flight uncommitted: `.5a` mdBook truth repair and currency lock until commit; none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
