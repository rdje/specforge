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
- Active unit: `FSMGEN-REFRESH-INTEGRATE-6.1` signoff complete, awaiting its commit; `SPEC-TO-INTENT-ALIGNMENT.0` through `.6a` are committed
  complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: FSMGen is detached cleanly at target pin `a51dcdad0`. The compatibility repair removes
  adapter-invented Cartesian rule priorities and residualizes uniquely owned named-drive overlaps. Rebuilt
  adapters are chain-current; 44/44 emitted ISFs pass the new strict binary with zero diagnostics. Aggregate CI
  passes all eight doctrines, 1,859 Rust tests / five ignored / zero failed, rustdoc, mdBook, and final locality.
- Next action: commit `.1`, clear the commit message, then resume PNT at `SPEC-TO-INTENT-ALIGNMENT.6b` unless the
  tracked cat-3 `.4c.ii` reassessment ranks first.
- In-flight uncommitted: FSMGen gitlink/pin authorities, ISF emitter compatibility repair, two Knowledge Map
  facts, pending topology reassessment, book/live/task alignment, and regenerated projections; all are verified
  and staged. All exact corpus scratch roots created by the slice have been deleted and confirmed absent.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
