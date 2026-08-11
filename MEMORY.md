# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey. History lives in
> `git log`; work state lives in the task-trees (`docs/tasks/`); durable facts/decisions live in
> `docs/decisions/`. Do **not** append session narration — overwrite the "Current state" block.

## How to resume (any AI, any harness)
- Derive the current revision on read with `git rev-parse HEAD`; never store a latest-commit shadow.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (doctrines are mechanically
  gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`); follow `COMMIT.md`
  after every slice (unit id in the commit subject).
- Non-negotiable doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md` (no code change without
  an owning task-tree first; signoff quality; zero ROADMAP↔code↔mdBook drift; push ~every 200 commits;
  artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh`; hooks + CI run it too. Retrieval starts at bounded
  `KNOWLEDGE_MAP.md`, then its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: none. `STATUS-LEDGER-ROLLOVER` is open at `.2` and `TASK-PART-SEAL-REACHABILITY` at `.0`
  — both pending, tracking only; neither blocks anything.
- Current state: corpus coverage is **51/56 with five real documents remaining**, at 78 SourceIR / 23
  normalized / 78 EvidenceIR / 78 downstream chains, 44/44 emitted ISFs FSMGen-strict clean. The status
  ledger is rolled: `segment-0007` seals 21 records and leaves the root at 50 records / 80,586 bytes
  (62.5% of records, 70.1% of bytes), so product-status entries are unblocked for roughly twelve slices
  instead of one. The mdBook no longer stores any current live-root size — four such sentences were false,
  and a value every commit invalidates is now derived on read.
- Next action: pick one. `CORPUS-COVERAGE.2.52` — own refresh #52; the smallest-retained-source rule points
  at `opencapi_25gbps_phy_mechanical_spec_v10` (760 elements), re-measure before pinning. Or
  `KG-ISF-COMPLETENESS.5.iv.a`, the deferred CODE slice, which is byte-changing on the AXI wire gold and
  wants a fresh focused session plus the full before/after WIRE-BASED-100 protocol.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. `CHANGES.md` sits at 82.2% of its line health target — about ten commits of runway before
  its own rollover becomes mandatory. The user-owned `.claude/settings.json` remains untouched.
