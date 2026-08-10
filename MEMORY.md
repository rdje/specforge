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
- Active unit: none; the whole `CORPUS-CHAIN-CURRENCY` tree (`.0`–`.3`) is complete and committed.
- Current state: the `CHAIN-CURRENCY` doctrine is registered CI-tier and GREEN on both legs. The corpus is 78
  documents at 22/78/78/78 measured-current stages with **44/44 emitted ISFs FSMGen-strict clean**; the 22
  retained normalized bundles are now declared in `doctrine/chain_currency/retained_bundles.json` and gated, so
  the remaining 56 unmeasurable documents can only shrink, one per refresh, as each backfills its own bundle.
- Next action: `FACT-CARD-CAPACITY-HEADROOM.0` — the fact plane is four facts from its `max_facts` 200 bound
  (193 cards / 196 facts), and a routine slice writes cards; measure both authorities and accept an ADR before
  anything widens. Then `CORPUS-COVERAGE.2.51` selects one of the six remaining real chip-spec documents.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
