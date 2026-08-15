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
- Active unit: `CLAIM-VERIFICATION-ADOPTION.4` owns tracked-producer and cited falsification-control closure.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.4` now binds 7 cited controls to 7 exact known-bad regions across 6 governed producers; the
  ignored/untracked producer-shaped census is 0/0. The workflow catalog probe observes a controlled sub-ceiling
  RED result, and the expanded claim-gate suite passes 27/27. `[claim: claim-provenance-gate-active]`
  The completed outer/current and inner/mdBook census authorities remain unchanged.
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: reconcile current/book regions and all claim digests, run focused/book/live-size/locality/doctrine
  gates, commit `.4`, verify clean, then activate `.5`. `MEMORY.md` stays capped.
- In-flight uncommitted: `.4` checker/schema, standard/ADR/fact projections, ledgers, task, memory, toolbox, and
  mdBook updates await exact-region/digest reconciliation and final gates; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
