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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii` is the next eligible behavioral-qualification leaf after
  the completed `.f.ii` implementation/calibration parent. Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: conformance owns the five invariant calibrations plus one semantic negative. The “at least” pair
  must remove exactly one SemanticIR assertion and its three propagated proof claims, ordinary invariance must
  reject, and the exact declared complement must pass. Nine synthetic fault classes reject; unavailable/vacuous
  attempts are unmeasurable and stale/ambiguous/partial attempts invalid. The frozen 24-row/23-text-measurable
  population and rich-PDF exclusions are unchanged.
- Next action: execute `.f.iii`: run the frozen 17-row prospective holdout across declared relations and publish
  identity-disjoint strata, exact denominators, uncertainty, and explicit unavailable/vacuous outcomes.
- In-flight uncommitted: none after the `.f.ii.c` commit; no background job.
- Blockers: none. Product direction is unambiguous. The user-owned `.claude/settings.json` is untouched.
