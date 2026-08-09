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
- Active unit: `FACT-CARD-CATALOG-CONTAINMENT.2.2` — migrate and close bounded fact browsing; implementation and
  focused verification complete, commit pending.
- Current state: the stable catalog is a 161-line / 11,984-byte direct-ID landing over three deterministic 56-card
  title parts; the four outputs total 340 lines / 46,890 bytes with no current warning. ADR 0023's three-line
  scaffold proves the root remains below mandatory rollover at all 198 cards. The exact generated surface is
  active, migrated enforcement passes, and the task tree is closed.
- Next action: finish composed/full gates and commit `.2.2`, then resume
  `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2` at its clean final-source-boundary leaf.
- In-flight uncommitted: migrated catalog output, state/surface switch, UTF-8 digest regression repair, and
  impact-routed closure documentation; canonical facts/evidence, product artifacts, PDF task source/destinations,
  shared inputs, thresholds, and ceilings unchanged.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
