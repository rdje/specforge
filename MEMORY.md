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
- Active unit: `ARTIFACT-PATH-PORTABILITY.0` — completed by this commit; the move-portability repair is
  measured, decomposed, and task-tree owned.
- Current state: the deleted old root remains in 335/506 generated JSON/Markdown files across all four IR
  stages, adapters, SourceIR, and prior memory (262,996 scalar values; 262,592 evidence `source_path`).
  Builders persist canonical absolute inputs, while validation/learning/recovery consumers reopen those
  fields; some silently skip missing lineage. No generated artifact has been rewritten yet.
- Next action: execute `ARTIFACT-PATH-PORTABILITY.1`: decide and test one safe relative serialization,
  current-root resolution, legacy-rebase, external-input, and escape/ambiguity refusal contract.
- In-flight uncommitted: none after this commit; no background job or disposable residue.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
