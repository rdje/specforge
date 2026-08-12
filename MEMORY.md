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
- Active unit: closing `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c`; `FSMGEN-REFRESH-INTEGRATE-8.1`,
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b`, `.6d.ii.a`, `.6d.i`,
  `.6c.ii`, `FSMGEN-REFRESH-INTEGRATE-6.1`, and the verified no-op `FSMGEN-REFRESH-INTEGRATE-7.1` are committed
  complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.6d.ii.c` is implemented and fully verified. EvidenceIR schema 2 carries only generic,
  source-derived protocol operations, frame phases, participant direction, states, and actors; schema-1 named
  protocol carriers fail closed. All 78 persisted chains are reconciled, 24/24 replayable EvidenceIR and 78/78
  downstream chains are current, and full CI passes 1,876 tests / six ignored / zero failed.
- Next action: commit `.6d.ii.c`, verify the tree clean, then activate `.6d.ii.d` and audit identity-/spelling-driven
  decisions in priors, SemanticIR, prompts, validation, and corpus commands.
- In-flight uncommitted: completed, verified `.6d.ii.c` commit set only; no background job or scratch workspace.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
