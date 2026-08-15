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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a` closure is implemented and evidence-complete; repository
  verification and the closure commit remain before the durable frontier moves to `.f.iv`.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: the exact clean-revision I2C pair passes 5/5 stages with 11,170/11,170 claims over 266,194 leaves.
  Canonical held-out evidence SHA `06e2358c…68a1` records 35 fresh passes, 16 eligibility-preflight unmeasurables,
  zero failures, and zero invalid attempts under production revision `2cdcd131`. The behavioral checker and 17/17
  mutations pass. Full CI passes 9/9 doctrines, all 11 production-genericity components, 1,980 Rust tests with
  eight intentional ignores, five doctests, warning-denied Clippy/Rustdoc, mdBook, and final locality. `MEMORY.md`
  remains capped at exactly 32,768 bytes.
- Next action: run the final explicit doctrine/commit workflow, commit `.f.iii.a`, then begin `.f.iv` complete
  reviewed-population reconciliation. Task metrics, focused synchronization, locality, and diagnostic cleanup pass.
- In-flight uncommitted: comparator/refresh implementation, canonical evidence, graph snapshot, and synchronized
  task/research/live/book/retrieval updates. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
