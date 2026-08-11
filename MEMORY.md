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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6a`; `.0` through `.5b` are committed complete. `.6a` is
  signoff-complete pending its clean-boundary commit. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: isolated current-binary AIA replay is implemented and captured. Same 827,669-byte source and
  reviewed 20×2 TOC yield `timing_parameter`→`unknown`, Evidence/Semantic/Intent timing 19→0, and TP 0→0. The
  controller now hard-gates replay coverage at 1/12 rather than presenting frozen 41/45/33 counts as current.
- Next action: commit `.6a` at its now-green clean boundary; then honor the user's queued FSMGEN
  submodule refresh by locating its existing task/KM route, owning the pin bump, updating, verifying, and
  committing it before resuming `.6b` on the remaining 11 current-binary replays.
- In-flight uncommitted: signoff-complete `.6a` replay tool/evidence, controller authority correction, and lockstep
  docs. Full CI passes 1,857 tests / five ignored / zero failed and all eight doctrines. Exact replay/source
  scratch roots are deleted with residue absent. No background job runs.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
