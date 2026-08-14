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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii` is in progress; no task-tree pivot is permitted. Tracking-
  only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`, `DECISION-RECORD-CAPACITY-HEADROOM.1`, and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: all 51 frozen prospective attempts completed. Unchanged PDF replay passes 17/17. Adversarial
  identity is 0/17 only because the comparator omitted identity-derived `stable_artifact_stem`; alpha is 0/15
  with one unmeasurable/one invalid because its transform admitted meaning-bearing role words. The I2C row alone
  has six typed signal declarations. Exact 17-document/90-stratum evidence and 14/14 mutations pass; no signoff.
- Next action: commit the diagnostic checkpoint, then normalize only the declared stem, restrict alpha authority
  to typed opaque declarations, add `eligible_symbol_surface_absent`, and requalify using retained raw pairs when
  their execution identity remains valid.
- In-flight uncommitted: diagnostic executor/evidence/checker and synchronized docs. Repo-local heldout scratch
  is intentionally retained under `.project-data/tmp/spec-to-intent-f-iii-heldout` for the owned repair; no
  background job.
- Blockers: none. Product direction is unambiguous. The user-owned `.claude/settings.json` is untouched.
