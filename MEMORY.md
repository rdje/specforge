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
- Active unit: `FSMGEN-REFRESH-INTEGRATE-7.1` closing commit; `SPEC-TO-INTENT-ALIGNMENT.6c.ii` and
  `FSMGEN-REFRESH-INTEGRATE-6.1` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: a pruned fetch and independent live remote query prove FSMGen `HEAD`/`main` are already the pinned
  `a51dcdad0a7e752e638abfe3ab414f7f3911889d`; the range is 0 commits / 0 changed paths, so cycle 7 has no gitlink,
  contract, product, Knowledge Map, or current-status delta. The clean detached pin passes all seven focused strict
  canaries, and every declared pin copy agrees. Controller v5 still selects `SPEC-TO-INTENT-ALIGNMENT.6d`, starting
  with the three OpenCAPI analog fabrications; current TP/FP/FN remain 24/5/16 with provenance 29/29.
- Next action: commit the verified FSMGen no-op refresh, verify a clean tree and zero-byte message file, then activate
  controller-selected `SPEC-TO-INTENT-ALIGNMENT.6d` before diagnosing the three OpenCAPI analog records.
- In-flight uncommitted: completed and focused-verified `FSMGEN-REFRESH-INTEGRATE-7.1` closure awaiting doctrine and
  mdBook gates plus its commit; no submodule-local change, project-data residue, or background process.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
