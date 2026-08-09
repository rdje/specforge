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
- Active unit: `CORPUS-COVERAGE.2.34b`; guarded USB4 Inter-Domain Service refresh is source-route blocked.
- Current state: `.2.34a` proves `.cache/local-references/chipdoc` still resolves to a 935 MiB Git checkout on the
  boot filesystem, while no checkout or selected PDF exists in the external SSD project directory. The retained
  chain remains unchanged at one `USB4` adapter signal / two rules.
- Next action: obtain the intended external-SSD `chipdoc` checkout route, or explicit director authorization for
  one read-only copy of the hashed PDF into repository-local project data; then preserve the baseline and ingest.
- In-flight uncommitted: none after the `.2.34a` probe commit; no source copy, link rewrite, pipeline command,
  rollback, or generated artifact changed. USB #33 remains final with zero rollback residue.
- Blockers: source locality conflicts with the director's SSD-only project statement. The user-owned
  `.claude/settings.json` remains untouched.
