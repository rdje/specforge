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
- Active unit: `MEMORY-RESUME-POINTER-BYTE-CAP.2` is pending behind the staged `.1` ceiling transaction. Product
  PNT remains `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a`, whose production remediation is clean at `2cdcd131`.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.1` fixes the pointer maximum at exactly 32,768 bytes across both executable authorities while
  preserving its 50-line, 160-byte content-line, and overwrite-only constraints. Its exact old/new live-size
  authority is transaction-scoped and must be retired after this increase becomes Git history.
- Next action: commit `.1`, delete the then-consumed ceiling-increase authority under `.2`, verify/commit that clean
  state, then resume the field-aware comparator replay against production revision `2cdcd131`.
- In-flight uncommitted: none after the `.1` policy commit; `.2` is the required next transaction. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
