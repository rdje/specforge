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
- Active unit: `SWD-SERIAL-EXTRACTION.7e.i` — protocol-aware convergence snapshots and fact counts.
- Current state: `.7b`/`.7c` carry all four protocol collections exactly through canonical IntentIR; `.7d`
  assigns every record one stable ordered adapter residual with provenance and explicit missing bindings. The
  directly lowerable subset is empty, while unrelated ISF remains byte-identical, renderable, and FSMGen-strict.
  Full `.7d` CI is green at 1,771 passed / five ignored. The status ledger's first post-migration segment also
  preserves 12 aged-out records exactly and leaves 60 live records. Convergence snapshots still omit all four
  protocol collections and can miss a protocol-only change.
- Next action: activate `.7e.i`; add all four collections to Evidence/Semantic/Intent convergence snapshots and
  fact counts, then prove protocol-only changes are visible without perturbing unrelated convergence behavior.
- In-flight uncommitted: none after the `.7d` commit. Generated migration state remains repository-local and
  intentionally ignored; the exact rollback and disposable loader workspace are absent.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
