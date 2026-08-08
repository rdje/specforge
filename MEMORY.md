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
- Active unit: `SWD-SERIAL-EXTRACTION.7c` — lossless SemanticIR→IntentIR protocol projection.
- Current state: `.7b` carries the four exact EvidenceIR protocol record collections into SemanticIR through
  additive default-empty/skip-empty fields and unfiltered clones; validation reports each count. Focused schema,
  394 SemanticIR, 18 semantic-validation, warning-deny Clippy, and live ADI 11/4/13/0 projection checks pass.
  IntentIR and the adapter still contain no consumer, exactly as owned by `.7c`/`.7d`; `.7e.i` now owns the
  separately found absence of all four counts from convergence snapshots before fresh `.7e.ii` promotion.
- Next action: activate `.7c`; add the same exact four collections to IntentIR, clone from SemanticIR, expose
  validation counts, and prove EvidenceIR→SemanticIR→IntentIR record/provenance parity plus legacy loading.
- In-flight uncommitted: none after the `.7b` commit. Generated migration state remains repository-local and
  intentionally ignored; the exact rollback and disposable loader workspace are absent.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
