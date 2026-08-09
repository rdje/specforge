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
- Active unit: `CORPUS-COVERAGE.2.33d.ii` — grounded dense-prose signal-authority repair; complete in this commit.
- Current state: both declaration catalogs require canonical `Signal <identifier> is <predicate>` grammar,
  parenthetical `bus` no longer means a one-bit wire, and ordinary port/pin tables require compact inventory
  structure. A direct EvidenceIR regression proves `AT`/`USB`/`ENHANCED`/`NO` cannot re-enter through relations,
  directions, or table provenance; initiator tie behavior remains lexicographically last. Serial/table/full-lib,
  WIRE, KG, Clippy, full-CI, book, doctrine, and locality gates pass. Corpus remains 80 SourceIR / 2 normalized /
  80 EvidenceIR / 79 downstream chains; refresh progress is 33 complete / 23 left.
- Next action: from the clean `.2.33d.ii` commit, activate `.2.33d.iii` and close the conditional downstream
  heuristic as measured unnecessary; then `.2.33d.iv` rebuilds and signs off the real USB cascade.
- In-flight uncommitted: none after the `.2.33d.ii` commit. Retained USB stages remain the `.d.iv` regression
  input; `.project-data/tmp` contains only `.gitkeep` plus the current project-owned `xcrun_db` cache.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
