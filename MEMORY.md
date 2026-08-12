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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` is complete and awaiting its required commit;
  `.6d.ii.d.i`, `.6d.ii.c`, `FSMGEN-REFRESH-INTEGRATE-8.1`, `.6d.ii.b`, `.6d.ii.a`, and `.6d.i` are committed
  complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: document identifiers are opaque from EvidenceIR through ISF lowering, exact one-way declaration
  grounding and alpha controls are implemented, all 78 persisted chains are reconciled, and 17/17 emitted ISFs
  pass FSMGen strict. Full CI passes all doctrines, warning-denied Rust checks and docs, 1,890/6/0 Rust tests,
  mdBook test/build, and the final locality residue check. ADR 0037 and public/live documentation agree. The exact
  562-file rollback was consumed and removed.
- Next action: commit `.6d.ii.d.ii`, verify a clean handoff, then activate
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii` to neutralize compiled prompts and corpus organization.
- In-flight uncommitted: the complete verified `.d.ii` code, generated artifacts, fixture authority, task-tree,
  ADR, Knowledge Map projection, live docs, and mdBook transaction; no scratch workspace or background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
