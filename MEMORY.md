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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6c.ii` closing commit; `.0` through `.6c.i` and
  `FSMGEN-REFRESH-INTEGRATE-6.1` are committed complete, while `.6c.ii`/parent `.6c` are verified complete in the
  current tree. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.6c.ii` qualifies 12/12 unchanged sources and 48/48 isolated stages at `74a658b3`. I2S changes
  exactly 0/5/5/5→5/0/0/0 TP/FP/FN/unprovenanced; seven OpenCAPI records gain provenance only; all 19 prior true
  positives survive. Current TP/FP/FN are 24/5/16, provenance 29/29, conservation 72/88. Controller v5 selects
  `.6d`, starting with three OpenCAPI analog fabrications. The 3,913-file / 1,090,908-KiB root and map are absent;
  full CI passes all eight doctrines, 1,866 tests / five ignored / zero failed, rustdoc, mdBook, and locality.
- Next action: commit `.6c.ii`, verify a clean tree and zero-byte message file, then activate controller-selected
  `SPEC-TO-INTENT-ALIGNMENT.6d` before diagnosing the three OpenCAPI analog records.
- In-flight uncommitted: completed and fully verified `.6c.ii` closure awaiting its commit; no scratch residue or
  background process.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
