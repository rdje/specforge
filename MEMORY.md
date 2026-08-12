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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i` is complete awaiting commit; `.6d.ii.d.iv`, `.6d.ii.d.iii`, `.6d.ii.d.ii`, `.6d.ii.d.i`, `.6d.ii.c`,
  `FSMGEN-REFRESH-INTEGRATE-8.1`, `.6d.ii.b`, `.6d.ii.a`, and `.6d.i` are committed complete. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: ADR 0038 accepts the proof-carrying genericity design. Exact checked inventories classify all 71
  compiled Rust modules and all 168 top-level artifact fields across 38 claim families once. This leaf changes no
  production extraction behavior; `.e.ii`–`.e.vii` own implementation, enforcement, and qualification.
- Next action: commit `.e.i` through `COMMIT.md`; only after the clean boundary, activate `.e.ii` and make the
  core-to-conformance dependency direction compiler-visible while moving named/calibrated authority out of core.
- In-flight uncommitted: verified `.e.i` decision, inventories/checker, exact lossless live-ledger rollover, and
  task/live/book/audit/retrieval alignment awaiting commit; no production-code, generated-artifact, or background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
