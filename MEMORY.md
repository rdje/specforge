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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.3`; `.0` through `.3` are complete pending this slice's commit. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: timing VLM notes now optionally produce typed EvidenceIR `FigureRegion`s. SemanticIR grounds
  lanes to the document signal catalog, converts through `PartialTrace`, and requires verifier `Pass` before
  lowerability; IntentIR carries the result. Validation counts available/unavailable regions. A reviewed NXP
  UM11732 fixture proves the persisted real-PDF path and rejection of a model-only lane without claiming a live
  provider run. Unenriched corpus artifacts stay compatible and currently contain zero VLM notes. The required
  Rust-analysis rollover is applied root-last and preserves its exact 14-record segment through the archive chain.
- Next action: commit `.3` through `COMMIT.md`, then pivot cleanly to `.4` and build held-out reviewed
  source-to-IntentIR gold with per-stage survival/loss, provenance, residual, recall, and precision accounting.
- In-flight uncommitted: `.3` code, reviewed fixture, exact Rust-analysis rollover, book/roadmap/retrieval/live-doc
  updates, and derived projections passed the final doctrine/CI gate and await only the commit workflow. No
  background job runs.
- Blockers: none. A live VLM endpoint remains unavailable, so `.3` proves production wiring with an explicitly
  reviewed fixture and leaves live-provider recall to `.4`. The user-owned `.claude/settings.json` is untouched.
