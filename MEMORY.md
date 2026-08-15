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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.a` restores structurally justified register-table carriers
  exposed by the first clean complete-population replay; `.f.iv.b` owns replay publication after this repair.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: clean revision `0d218116` replayed 12/12 reviewed PDFs and 48/48 isolated stages. The unpublished
  diagnostic is 12/15/28 IntentIR TP/FP/FN. AMD's packed-layout fabrication is gone; GIC-400 has 15 source-named
  rows with missing access; Arm Debug loses 12 correct facts because its qualified address header stays unknown.
  The task split and durable diagnosis are being committed before code. `MEMORY.md` remains capped at 32,768 B.
- Next action: implement and focus-test parenthesized closed-role matching plus register-map access-carrier
  preservation under `.f.iv.a`, then rerun the complete population under `.f.iv.b` from the clean repair commit.
- In-flight uncommitted: the exact 3,913-file diagnostic root and runtime source map remain repository-local and
  untracked until the carrier repair consumes them. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
