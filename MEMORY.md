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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.i`; `.6c.ii`, `FSMGEN-REFRESH-INTEGRATE-6.1`, and the verified no-op
  `FSMGEN-REFRESH-INTEGRATE-7.1` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.6d.i` is verified complete and ready to commit. A closed first-unit-token `dB`/`dBc` grammar
  retains physical scalar observations with actionable non-applicable disposition while excluding them from
  executable temporal derivation. ADR 0025 reconciles exactly two measurable OpenCAPI chains: 115 timings hold,
  26 gain disposition, 6/6 stages otherwise equal, adapters equal, and currency is 24/24 plus 78/78 downstream.
  Three bundle-less CCIX chains retain 48 legacy matches as an explicit refresh-owned replayability boundary.
- Next action: commit `.6d.i`, verify a clean handoff, then activate `.6d.ii` for the clean 12-source replay and
  exact result/controller publication at the committed carrier revision.
- In-flight uncommitted: the completed `.6d.i` code, generated projections, docs, task evidence, and required
  catalog/ledger maintenance await their single commit; both task scratch roots are absent and no job is running.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
