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
- Active unit: `CLAIM-VERIFICATION-ADOPTION.0` owns and maps the requested fifth portable architecture.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: claim-verification `.0` maps 39 current governed surfaces, 14 derived-state contracts, 33 tracked
  checker files / 24 self-test fronts, the missing review template, and the dedicated bounded-registry path. The
  standard is not yet adopted. `SPEC-CLARIFICATION-LOOP.3` remains the product frontier after this explicit
  governance directive is satisfied.
- Next action: commit `.0` cleanly, then pivot to `DECISION-RECORD-CAPACITY-HEADROOM.1/.2` because the required
  claim-verification ADR cannot be added safely at the current 42/44 decision-file pressure; resume claim `.1`
  from that clean prerequisite. `MEMORY.md` remains capped at 32,768 B.
- In-flight uncommitted: none after the `.0` commit; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
