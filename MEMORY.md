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
- Active unit: `CORPUS-COVERAGE.4` (frontier census integrity) — `.4.0` done, `.4.1` pending.
  `STATUS-LEDGER-ROLLOVER` is open at `.2` and `TASK-PART-SEAL-REACHABILITY` at `.0` — both tracking only.
- Current state: corpus coverage is **51/57 with six real documents remaining**, at 78 SourceIR / 23
  normalized / 78 EvidenceIR / 78 downstream chains, 44/44 emitted ISFs FSMGen-strict clean. `.4.0` derived
  that census instead of reading it and found the tracked count short by one: it had been hand-decremented
  once per refresh since `.2.29`, so a stale denominator adjustment dropped
  `nvme_base_specification_2_0a_2021_07_26` off the queue. Re-derive with the `reverify` command in
  `docs/knowledge/corpus-refresh-frontier-derivation.md`; never trust a transcribed refresh count.
- Next action: pick one. `CORPUS-COVERAGE.4.1` — the mechanical census gate that makes
  `cohort = refreshed + remaining` enforced rather than carried; until it lands the corrected count is still
  only carried. Or `CORPUS-COVERAGE.2.52` — own refresh #52; smallest-retained-source points at
  `opencapi_25gbps_phy_mechanical_spec_v10` (760 elements). Or `KG-ISF-COMPLETENESS.5.iv.a`, the deferred
  CODE slice, byte-changing on the AXI wire gold, wanting a fresh session plus the full WIRE-BASED-100 protocol.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. `CHANGES.md` sits near 84% of its line health target — roughly eight commits of runway before
  its own rollover becomes mandatory. The user-owned `.claude/settings.json` remains untouched.
