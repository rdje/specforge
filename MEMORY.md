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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.b` is closing clean population publication; `.f.v` is the
  durable next frontier after this slice commits.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: the 12-source / 48-stage replay from clean `e125aac7` publishes 39/0/1 IntentIR TP/FP/FN,
  42/42 provenance, 117/118 conservation, zero fabrications, and one APB drop. All controller hard gates pass;
  `.7` ranks first and `.6e` is superseded. The four-value driver contract is focused-test guarded.
- Next action: finish focused publication/docs/doctrine checks, commit `.f.iv.b`, then activate `.f.v` final
  behavioral signoff. `MEMORY.md` remains capped at 32,768 B.
- In-flight uncommitted: `.f.iv.b` result/manifest/controller/code/docs are staged in the worktree. All three
  replay roots and the runtime map are removed with residue absent; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
