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
- Active unit: none — `CORPUS-COVERAGE.2.51` is **done**. `TASK-PART-SEAL-REACHABILITY` is open at `.0`
  (pending, tracking only; it blocks nothing).
- Current state: corpus coverage is **51/56 with five real documents remaining**, at 78 SourceIR / 23 normalized
  / 78 EvidenceIR / 78 downstream chains, 44/44 emitted ISFs FSMGen-strict clean. Refresh #51 rebuilt the Arm
  SMMU Software Guide with an exact attribution: its source structure was already current, so the fresh SourceIR
  differs from the stale one only in the two source-path values, and `CORPUS-CHAIN-CURRENCY.3` had already
  rebuilt the last three stages — so the whole delta is the evidence stage, and it is one thing. Three
  `Enum TABLE …` statements, minted when an older binary named a table after its caption word, are retired by
  `KG-ISF-COMPLETENESS.5.i`; `symbol_definitions` goes 1 → 0 and the adapter loses its last rendered type while
  keeping the same honest block. The document is a self-declared `methodology-guide` (category 6), so the empty
  result is correct. Evidence lives in the new `refreshes-51-56` part; `refreshes-49-56` is closed to further
  writes but stays `active` because the contract's `sealed` state is unreachable
  (`TASK-PART-SEAL-REACHABILITY`).
- Next action: `CORPUS-COVERAGE.2.52` — select and own refresh #52 from the five remaining documents. Under the
  smallest-retained-source rule the next is `opencapi_25gbps_phy_mechanical_spec_v10` at 760 elements, but
  re-measure from current corpus evidence before pinning it.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
