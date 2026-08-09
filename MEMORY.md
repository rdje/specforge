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
- Active unit: `CORPUS-COVERAGE.2.38a`; commit the verified path-containment repair before resuming parent #38.
- Current state: `validate <artifact>` now backannotates only the explicit repository-owned artifact and its
  adjacent report; it cannot follow embedded canonical output paths. One all-stage regression and a five-stage
  real #38 copy replay keep the complete canonical tree byte-identical. Existing canonical-path tests, Clippy,
  and full CI pass 1,788 tests / five ignored. Original pre-side-effect #38 hashes/counts and the complete
  validator-side-effect state remain in task evidence; the source is authenticated at `08a37c35…162`.
- Next action: commit `.2.38a`, then run a guarded CPU ingest from the authenticated same-SSD source to regenerate
  and refresh the #38 canonical chain before measuring relation/topology authority.
- In-flight uncommitted: verified `.2.38a` code/docs/fact signoff awaits commit; ignored #38 task evidence must
  remain until fresh regeneration and the parent refresh gates are complete.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
