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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.6a` — completed by this commit; parent `.6` is
  closed and `.7` is the sole remaining adoption frontier.
- Current state: Cargo, canonical scripts, production tempdirs, and Docling/VLM/FSMGen child processes
  use repository-derived same-filesystem roots under the SSD. Exact rollback-safe Python locks/rebuilds
  removed 53 stale old-tree references; the two required Docling models are checksum-proved locally;
  the exact ancestor-only 2.9 GiB boot-volume repository is deleted and absent. Six doctrines, final
  producer residue checks, full CI, and the 36-part book pass. Shared `~/.cargo`, `~/.rustup`, and the
  ambiguous home Hugging Face cache remain protected external inputs.
- Next action: from the clean `.6a` commit, activate `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.7` and
  run the final whole-program closure/retrieval/locality/handoff audit.
- In-flight uncommitted: none after this commit; no background job or disposable temp/log residue.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
