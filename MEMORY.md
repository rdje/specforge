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
- Active unit: `LIVE-DOC-STOP-RISK` — `.0`, `.0a`, `.1` done; only `.1a` (retire spent authorities) remains.
- Current state: the whole "bound with no compliant exit" class is now gated. `.0`/ADR 0030 gave `ROADMAP.md`
  the missing **exit** — the archive is a bounded capsule series, so a rollover is copy-root-to-dated-capsule
  + one contract record + one index row + rewrite, all gate-proved. `.0a`/ADR 0031 closed the **entrance** —
  each H2 declares its own bound and remedy, and their legal sum fits the health target (243/256); the root is
  156 lines, every section under its warning. `.1`/ADR 0032 generalized it: every `collection` surface must
  declare `aggregate ≥ files × per-file` on both bands, with one exemption (`aggregate_composition`) that must
  *sum* to the declared bounds — `fact_index` is its only user. Ten aggregates were re-derived under exact
  authorities; `task_evidence` can now get the file capacity its 81.9% warning needs. Pressure lines name the
  headroom to the ceiling. Suites: roadmap 9 → 49 cases, live-doc 68 → 81.
- Next action: `LIVE-DOC-STOP-RISK.1a` — delete the ten spent `increase` records from
  `doctrine/live_document_size/ceiling_increase_authorities.jsonl` (deletions only; a banked authority fails
  closed). That closes the tree; `CORPUS-COVERAGE.2.51` is the next product slice.
  Note before appending: `CHANGES.md` is at 88.4% of its line health target — 90% forces a rollover.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
