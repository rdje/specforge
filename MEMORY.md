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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.5b` closing commit; `.0` through `.5b` are verified complete. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.5b` composes the frozen `.4c` result and live-equal provider-free `.2` ledger
  into byte-current input/report artifacts. It yields `diverging` on 41 fabrication, 45 provenance, and 33 drop
  violations with `insufficient_history`, and recommends already-open `.6` ahead of `.7`/`.8`/`.9`. Full CI
  passes all eight doctrines and 1,854 tests / five ignored / zero failed.
- Next action: commit `.5b`, verify the clean boundary and zero transient brief, then activate the controller-
  selected `.6` hard honesty/provenance repair.
- In-flight uncommitted: completed, fully verified `.5b` slice awaiting its commit only. No background job runs.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
