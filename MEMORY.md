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
- Active unit: `FACT-CARD-CAPACITY-HEADROOM` — `.0`/`.2`/`.2a`/`.3` done, `.1` superseded, `.3a` is the frontier.
- Current state: `.3` re-derived the whole fact plane as one profile (ADR 0029). Measuring found the advertised
  198-card capacity was never reachable — 198 cards need 10,026 lines against a 10,000-line aggregate — and that
  the breach had no legal exit, because cards are canonical and never deleted or rolled over. Pressure now sits
  only on dimensions with a remedy; aggregates are the file bound times the per-file bound; the profile derives
  from one parameter (`max_parts` 6). Capacity is **336 cards / 44 decision records / 379 facts**, currently
  193 / 30 / 199, with every fact-plane rollover warning gone. Writing fact cards and decision records is
  unblocked.
- Next action: `FACT-CARD-CAPACITY-HEADROOM.3a` — delete the four `ceiling_increase_authorities` records `.3`
  consumed. The containment checker treats an authority that outlives its increase as banked and fails closed,
  so the gate does not pass again until they are retired. Then the tree closes and `CORPUS-COVERAGE.2.51`
  follows.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
