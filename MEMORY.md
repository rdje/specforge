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
- Active unit: `CORPUS-COVERAGE.2.33d.i` — universal dense-prose signal-authority probe; complete in this commit.
- Current state: the 80-EvidenceIR/79-IntentIR census locates three upstream authority errors: 97 weak
  sentence-start declaration matches, 17 bus-acronym width-one captures, and 11 accepted port/pin-only table
  shapes. Direction synthesis only amplifies names from those catalogs, so no separate downstream heuristic is
  planned if direct convergence and real USB verification pass. The durable report/fact and exact USB before
  signature are current; no production/generated artifact changed. Corpus remains 80 SourceIR / 2 normalized /
  80 EvidenceIR / 79 downstream chains; refresh progress is 33 complete / 23 left.
- Next action: from a clean post-probe commit, activate `CORPUS-COVERAGE.2.33d.ii`; implement only the measured
  formal-predicate, non-bus single-wire, and port/pin inventory-structure repairs plus tie-comment regression.
- In-flight uncommitted: none after the `.2.33d.i` commit. Fresh ignored USB stage artifacts remain the `.d.iv`
  regression input; temp contains only `.gitkeep`.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
