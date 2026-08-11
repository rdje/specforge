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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.4a`; `.0` through `.3` are committed. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.4a` is complete and fully verified: its strict portable schema plus generic Rust engine score
  pinned four-stage snapshots with exact multiset metrics, first-stage diagnosis, provenance/residual/modality
  accounting, and six controlled-fault classes. No extractor or canonical corpus artifact changed, and no
  category-support claim was made.
- Next action: commit `.4a` through `COMMIT.md`, then pivot cleanly to `.4b` and lock two review-complete
  documents per category against the frozen evaluator without tuning extraction.
- In-flight uncommitted: the completed `.4a` code/schema, public/retrieval/live-doc alignment, and derived
  projections passed the full CI gate and await only the commit workflow. No background job runs.
- Blockers: none. Live-provider output remains unavailable and must report `unmeasurable`, never passing. The
  user-owned `.claude/settings.json` is untouched.
