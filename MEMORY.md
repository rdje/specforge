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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8e` — independent derived-state closure audit; complete
  with a closure-blocking finding in this commit.
- Current state: all 14 primaries, three secondaries, 17 exact markers, declared authorities, and 72 mutations
  pass, but the older feedback-protocol self-test renderer stores the live FSMGen hash in executable source.
  Full CI is green but does not prove repository-wide current-value exhaustiveness. `.8` remains open.
- Next action: from the clean `.8e` commit, activate `.8f`; render all synthetic feedback required literals from
  contract data and add a changed-pin no-fallback mutation. `.8g` then independently re-audits before `.8` may
  close; `.9a` remains the next archive-route boundary.
- In-flight uncommitted: none after the `.8e` audit commit. `.project-data/tmp` contains only `.gitkeep` plus
  `xcrun_db`; no doctrine code, registry value, product behavior, artifact, donor/submodule, threshold, or ceiling
  changed in the audit.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
