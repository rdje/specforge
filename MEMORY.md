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
- Active unit: `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.2.1` is complete from clean predecessor `3aaa9166` and
  awaits its commit; `.2.2` is the next containment leaf.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: the source-locked alignment contract closes 21 regions, eight planned parts, and all 58 exact
  owner declarations; three independent authorities now self-route through the neutral checker and doctrine.
  Full CI passes, generated artifacts are removed, and the alignment source remains byte-identical to `112bc333`.
- Next action: commit `.2.1`, then execute `.2.2` from the clean boundary to pin every final migration input.
- In-flight uncommitted: completed and verified `.2.1` changes awaiting commit; no background job.
- Blockers: `.6d.ii.f` waits only for bounded task-evidence migration; product direction is unambiguous. The
  user-owned `.claude/settings.json` is untouched.
