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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6b.iii` at its closure commit; `.0` through `.6b.ii.b` and
  `FSMGEN-REFRESH-INTEGRATE-6.1` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.6b.iii` resolves default bounded activation from fixed total RAM, a conservative measured
  75-MB/page estimate, a 40% budget, and a 399-page risk cap; explicit nonnegative overrides stay exact. The
  24-GiB host resolves threshold 131 and batch size 64. Unknown PDF page count fails closed, and Unix signal
  termination is distinct from an ordinary nonzero exit without being mislabeled OOM. An override-free 400-page
  replay has exact path-normalized four-stage fidelity; its 795-file / 184,164-KiB root is absent. Full CI passes
  all eight doctrines, 1,866 tests / five ignored / zero failed, rustdoc, mdBook, and locality.
- Next action: commit `.6b.iii`, verify a clean handoff, then activate `.6c` and localize the controller-selected
  five-record I2S missing-`ns` carrier loss before changing production code.
- In-flight uncommitted: completed `.6b.iii` code/tests/docs awaiting commit; no scratch or background process.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
