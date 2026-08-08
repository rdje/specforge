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
- Active unit: `SWD-SERIAL-EXTRACTION.7b` — lossless EvidenceIR→SemanticIR protocol projection.
- Current state: `.7a` accepted ADR 0016 after auditing all four scored surfaces. They must carry exactly through
  SemanticIR/IntentIR and residualize at ISF unless their own records supply every behavioral binding; the
  currently safe direct-lowering subset is empty. Canonical SWD remains the deliberate pre-`.4e` 11/4/13/0
  cache; `.7e` owns a fresh tracked-PDF ingest and promotion because its old normalized Markdown was reclaimed.
- Next action: activate `.7b`; add serde-default/skip-empty SemanticIR fields for the exact four EvidenceIR
  record types, clone them without filtering, report validation counts, and prove backward compatibility plus
  exact record/provenance parity.
- In-flight uncommitted: none after the `.7a` commit. Generated migration state remains repository-local and
  intentionally ignored; the exact rollback and disposable loader workspace are absent.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
