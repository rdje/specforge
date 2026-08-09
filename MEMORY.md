# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8a` — newer derived-state contract probe; complete in
  this commit.
- Current state: the donor's newer neutral revision is pinned and compared. Existing SpecForge surface-level
  generators/currentness remain valid, but field-level derive-on-read / verified-copy declarations are absent.
  The durable report identifies the exact Rust prerequisite and FSMGen gitlink comparisons plus the bounded
  neutral registry/checker contract; task-scoped corpus counts are no longer repeated in this resume pointer.
- Next action: from the clean `.8a` commit, activate `.8b`; implement the measured field registry, neutral
  fail-closed checks, Rust/gitlink authority adapter, normative wording, and reader alignment without widening
  any surface ceiling. `.8c` then independently audits and closes the reopened program.
- In-flight uncommitted: none after the `.8a` commit. The `.2.33d.iii` corpus leaf remains pending behind this
  director-requested bounded adoption review; `.project-data/tmp` contains only `.gitkeep` plus `xcrun_db`.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
