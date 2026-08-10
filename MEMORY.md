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
- Active unit: `FACT-CARD-CAPACITY-HEADROOM` — `.0`/`.2` done, `.1` superseded, `.3` is the frontier.
- Current state: `.2` made the fact-card landing a fixed-size router — 10 lines / 878 bytes at 193 cards, sized
  by the part count instead of the card count — and gave the containment doctrine a `routed_membership` index
  kind with a fixed one-hop completeness proof. **Capacity is still 198 cards / 200 facts: 193 cards, 198 facts,
  2 free fact slots.** Do not write a new fact card or an `answers:`-bearing ADR until `.3` lands.
- Next action: `FACT-CARD-CAPACITY-HEADROOM.3` — re-derive the whole profile against the new shape in one
  transaction (per-part lines, part totals, part count, the `knowledge_cards` file ceiling that `max_cards`
  derives from, `max_facts`, `max_question_keys`), each against its own 90%-rollover rule, with both-sided
  boundary regressions. `CORPUS-COVERAGE.2.51` follows.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
