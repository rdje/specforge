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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.a` is closing its verified structural-carrier repair;
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.b` is the durable next frontier for clean replay/publication.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: qualified closed roles and unique structural access columns restore Arm 12/12 and GIC 15/15
  target facts while AMD stays unknown. Exact ADR 0025 migration is 24/24 current and zero stale through every
  stage; only Arm and one OpenCAPI access carrier change downstream. `MEMORY.md` remains capped at 32,768 B.
- Next action: commit `.f.iv.a`, then under `.f.iv.b` fix the replay driver's five-versus-four positional
  arguments and run/publish the complete 12-source population from the clean repair revision.
- In-flight uncommitted: tracked `.f.iv.a` code/docs only. The five declared scratch roots were removed after
  full CI and their residue is absent; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
