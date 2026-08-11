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
- Active unit: none. `CORPUS-COVERAGE.2.51` and `KG-ISF-COMPLETENESS.5.iv` are both **done**;
  `TASK-PART-SEAL-REACHABILITY` is open at `.0` (pending, tracking only; it blocks nothing).
- Current state: corpus coverage is **51/56 with five real documents remaining**, at 78 SourceIR / 23 normalized
  / 78 EvidenceIR / 78 downstream chains, 44/44 emitted ISFs FSMGen-strict clean. Refresh #51 rebuilt the Arm
  SMMU Software Guide with an exact attribution — source structure already current, so the delta is the
  evidence stage alone, and it is one thing: three `Enum TABLE …` statements retired by
  `KG-ISF-COMPLETENESS.5.i`, taking `symbol_definitions` 1 → 0. Evidence lives in the new `refreshes-51-56`
  part; `refreshes-49-56` is closed to further writes but stays `active` because `sealed` is unreachable.
  That refresh then surfaced `.5.iv`: the enum-name gate reads a table's header as a veto and never as a
  source. Measured read-only (reproducer `scripts/measure_encoding_enum_header_naming.py`) — 134 tables in ten
  documents would gain an enum, and **0 of 28 collision groups conflict**, so `.5.i`'s merge-by-name objection
  does not reproduce. GO on the lever, NO-GO on the naive predicate (four junk classes to exclude first).
- Next action: roll the `live-achievement-status` ledger first — refresh #51's entry took it to its 90% record
  threshold, so the next genuine product-status entry is refused until an owned rollover transaction runs
  (`COMMIT.md` §Rolling-ledger rollover; size the cut to include the record it must itself write). Then either
  `CORPUS-COVERAGE.2.52` — own refresh #52; the smallest-retained-source rule points at
  `opencapi_25gbps_phy_mechanical_spec_v10` (760 elements), re-measure before pinning — or
  `KG-ISF-COMPLETENESS.5.iv.a`, the deferred CODE slice, which is byte-changing on the AXI wire gold and wants
  a fresh focused session plus the full before/after WIRE-BASED-100 protocol.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
