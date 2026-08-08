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
- Active unit: `MDBOOK-DOCTEST-HYGIENE.0` — completed by this commit; the latent optional mdBook doctest gap is
  measured and durably owned before any fence or CI change.
- Current state: canonical full CI and `mdbook build docs/book` are green, but `mdbook test docs/book` exposes
  26 pre-existing misclassified illustrative blocks across four chapters. The new tree splits semantic fence
  classification from later enforcement; `ARTIFACT-PATH-PORTABILITY.2` remains clean at commit `77818b84` and
  its 335-file generated-data baseline remains untouched.
- Next action: execute `MDBOOK-DOCTEST-HYGIENE.1`: classify the 26 failing examples by language/executability
  and make mdBook test/build green without changing their rendered teaching content.
- In-flight uncommitted: none after this commit; no background job or disposable residue.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
