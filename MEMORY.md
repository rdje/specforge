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
- Active unit: `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.1.1` is complete and awaiting commit from clean
  predecessor `5032b378`.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.1.1` accounts exactly for 21 source regions, 58 formal ids, 45 frontier rows, 40 committed
  leaves, 31 stable-path and 92 identifier consumers, the manual writer, and the load-bearing trajectory reader.
  Seven semantic groups fit existing checker caps; the target remains byte-identical to `112bc333`.
- Next action: commit `.1.1`, verify the clean handoff, then activate `.1.2` to accept exact authorities,
  partitions, current-state precedence, bounds, writer/rotation rules, migration stages, and rejection cases.
- In-flight uncommitted: completed census report/task/retrieval/resume updates awaiting commit; no background job.
- Blockers: `.6d.ii.f` waits only for bounded task-evidence migration; product direction is unambiguous. The
  user-owned `.claude/settings.json` is untouched.
