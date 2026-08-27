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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.8a` is the frontier under active `.8`. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`, `CLAIM-VERIFICATION-ADOPTION.1a`, and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.8` activation decomposed the published 4/24 residual actionability at pinned `.7c.ii`. Four
  observations are actionable through the existing non-applicable timing carrier; eight belong to four canonical
  cells already exact at every promoted stage, so no residual is required and emitting one would contradict the
  promoted fact; twelve are genuinely required and absent across the six cells that are the whole current
  hard-failure set. Production owns one typed residual carrier and none for captured prose, table, or visual
  regions with no canonical carrier. Activation also found this gap's published reproduction command names the
  wrong crate and runs zero tests at exit zero. No production, evaluator, fixture, result, or controller
  authority changed.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: execute `.8a` — freeze when a residual is required at each promoted stage, the typed
  source/cause/stage/replay grammar, the refusal matrix, chain and replay obligations, and the reproduction-command
  repair, before `.8b` touches accounting or `.8c` touches production.
- In-flight uncommitted: none after the `.8` activation commit; the change ledger was rolled to
  `segment-0012-2026-08-27` in the same transaction and no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
