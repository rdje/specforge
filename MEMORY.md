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
- Active unit: `CORPUS-COVERAGE.2.33b` — shared UTF-8 repair and real USB EvidenceIR retry complete, awaiting commit.
- Current state: both declaration catalogs use one valid-prefix boundary helper; the focused 252-test EvidenceIR
  suite and full CI (1,776 pass / five ignored) are green. Rebuilt release `1710e5…` constructs and validates USB
  EvidenceIR from the promoted SourceIR: 918 anchors / 8,267 spans / 507 visuals / 22,029 links / 8,412 statements.
- Next action: commit `.2.33b`, then execute `.2.33c`: build/validate SemanticIR and IntentIR, adapt/render/FSMGen
  strict if possible, classify the 46 unexplained tables and false-looking semantic signal names honestly, update
  corpus counts/book truth, remove the exact rollback and residue, and close `.2.33` without another Docling run.
- In-flight uncommitted: one Rust helper/test, aligned live docs, fresh ignored USB normalized/SourceIR/EvidenceIR,
  and the 4.9 MiB byte-verified rollback under `.cache/task-workspaces/CORPUS-COVERAGE.2.33`.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
