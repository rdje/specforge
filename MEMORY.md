# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Derive the current repository revision on read with `git rev-parse HEAD`; never store a
  latest-commit shadow that the recording commit would immediately invalidate.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver for memory,
  knowledge-map, task-acceptance, README policy, and live-document containment); hooks + CI
  run it too. Retrieval starts at bounded `KNOWLEDGE_MAP.md`, then searches its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.10a` — terminal task-evidence design complete and
  verified; commit pending.
- Current state: ADR 0018 selects a `.10b.i` committed exact-source/verifier boundary followed by `.10b.ii`
  byte-identical capsule + bounded closed root/index. The task file is now 2,502 lines / 237,944 bytes with
  197 lines / 12,731 bytes of pre-rollover headroom. Active `PDF-VARIANT-DIGESTION` is 207 bytes before warning and may
  not accept another append until a new task tree owns active-tree containment after `.10` closes.
- Next action: commit `.10a`, then activate `.10b.i`; implement the neutral contract/checker, pin the complete
  live-source identity and provenance, exercise fail-closed mutations, and commit before root replacement.
- In-flight uncommitted: verified `.10a` design/live-doc changes only; no task content moved and no archive,
  executable, threshold, ceiling, product, or canonical artifact changed. `.project-data/tmp` contains only
  `.gitkeep` plus `xcrun_db`.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
