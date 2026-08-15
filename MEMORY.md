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
- Active unit: `CLAIM-VERIFICATION-ADOPTION.3b.3.2` owns the adjudicated mdBook quantitative result.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.3b.3.2` covers all 304 candidates with exact regions: eight registered workflow-capacity lines,
  75 current actionable lines with all three evidence legs named missing, and 221 exact authored, example/
  identity, or dated exclusions. Frozen replay and the nineteen-case suite pass; the outer census is unchanged.
  `[claim: current-claim-census-frozen]`
- Next action: commit `.3b.3.2`, verify a clean tree, then activate `.3b.3.3` and independently freeze the exact
  result as the authority `.3b.4` may consume. `MEMORY.md` stays capped.
- In-flight uncommitted: `.3b.3.2` adjudication, bound repair, and lockstep docs await gates; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
