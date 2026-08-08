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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4b` — atomically migrate `CHANGES.md` under
  the shared lossless rolling-ledger protocol.
- Current state: `.0`–`.4a` and `.5a` are complete. The fifth doctrine governs all 557 Markdown files
  across 24 surfaces; its composed ledger checker reconstructs 6,667 records / 6,425,335 source bytes
  exactly and pins four reviewed live windows plus consumers and archive routes. README remains
  bounded. No historical record moved in `.4a`; query revision from Git.
- Next action: freeze the pinned `CHANGES.md` as its immutable source capsule, derive the 87-record
  stable-root survivor, create the bounded manifest/index, switch its registry state, and prove exact
  retrieval before removing historical bytes from the root in the same `.4b` commit.
- In-flight uncommitted: `.4a` protocol/checker/docs until commit; none afterward.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
