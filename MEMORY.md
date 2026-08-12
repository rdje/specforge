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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6c.i` at its closure commit; `.0` through `.6b.iii` and
  `FSMGEN-REFRESH-INTEGRATE-6.1` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.6c.i` adds a closed caption-wide timing-unit fallback plus default-empty direct table
  provenance. The five reviewed I2S facts carry exact `ns` and `table_0004` through IntentIR. Five retained
  chains reconcile exactly: 231 timing records gain existing table ids, only those five units change, 15/15
  carrier-neutralized stages and all adapters equal their backups, and currency is 24/24 plus 78/78/78. Full CI
  passes all eight doctrines, 1,866 tests / five ignored / zero failed, rustdoc, mdBook, and locality.
- Next action: commit `.6c.i`, verify a clean handoff, then activate `.6c.ii` and replay all 12 reviewed sources
  at the clean carrier revision before publishing any new result or controller claim.
- In-flight uncommitted: completed, fully verified `.6c.i`; exact probe/reconciliation roots and disposable log
  are absent, full CI is green, and no background process remains.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
