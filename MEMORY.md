# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey. History lives in
> `git log`; work state lives in the task-trees (`docs/tasks/`); durable facts/decisions live in
> `docs/decisions/`. Do **not** append session narration — overwrite the "Current state" block.

## How to resume (any AI, any harness)
- Derive the current revision on read with `git rev-parse HEAD`; never store a latest-commit shadow.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (doctrines are mechanically
  gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`); follow `COMMIT.md`
  after every slice (unit id in the commit subject).
- Non-negotiable doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md` (no code change without
  an owning task-tree first; signoff quality; zero ROADMAP↔code↔mdBook drift; push ~every 200 commits;
  artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh`; hooks + CI run it too. Retrieval starts at bounded
  `KNOWLEDGE_MAP.md`, then its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.4c`; `.0` through `.4b` are complete. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.4b` locks 12 reviewed documents / two per category and 14 complete bounded cells without
  extractor tuning. External dataset identities are portable; the authorized source library resolves at
  `/Volumes/SSD/Documents/livework/chipdoc` on the repository's SSD. `CORPUS-FRONTIER` now separates source
  locality from exact refreshed/remaining lifecycle state.
- Next action: run the `.4a` evaluator over the frozen `.4b` population, commit the exact result snapshot and
  stage-loss diagnosis, and name the evidence-ranked upstream versus downstream blocker without changing gold,
  selection, extraction behavior, or source artifacts.
- In-flight uncommitted: none after the `.4b` commit. No background job runs.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
