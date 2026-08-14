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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii` corrected qualification is durably complete;
  the next eligible unit is `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a`. Tracking-
  only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`, `DECISION-RECORD-CAPACITY-HEADROOM.1`, and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: corrected 51-attempt evidence is 34 pass / one fail / 16 unmeasurable / zero invalid. Unchanged
  PDF and adversarial identity pass 17/17. Fifteen alpha rows lack typed opaque declarations, one is vacuous, and
  the valid six-signal I2C alpha pair fails below SourceIR with 11,170→11,111 cumulative claims and 12,441
  undeclared downstream paths. Aggregate SHA is `fbc8331bac463bc400c079873406e5ffe9388a73c513a4811a6499edab5a4e06`;
  17/17 checker mutations and 19 focused tests pass / two provider tests intentionally ignored. Full CI passes
  all nine doctrines, all 11 production-genericity components, 1,972 Rust tests / eight ignored / zero failed,
  five compile-fail doctests, Clippy/Rustdoc, mdBook, and final locality.
- Next action: start the task-owned `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a` production symbol-spelling/ordering
  remediation before `.f.iv` replay.
- In-flight uncommitted: none at handoff. The recursive repo-local held-out evidence chain is intentionally
  retained in `.project-data/tmp`; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
