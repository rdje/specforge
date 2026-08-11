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
- Active unit: `SIGNAL-CATALOG-CAPTURE-GAP.1` — new tree; `.0` (ownership + census) done, no code touched.
  `SEMANTIC-EMPTY-CATALOG-FILTER` is `done`/closed. Tracking-only: `STATUS-LEDGER-ROLLOVER` `.2`,
  `TASK-PART-SEAL-REACHABILITY` `.0`.
- Current state: the closed tree deleted the `declared_signal_names.is_empty()` special case in
  `crates/specforge/src/ir/semantic.rs`, so one grounding predicate governs every document and a rejected record
  is demoted to a `semantic_ungrounded_records_not_promoted` packet. All 78 chains were rebuilt from unchanged
  EvidenceIR and the 179-artifact validation population re-validated: `CHAIN-CURRENCY` green at 24/78/78/78, all
  44 emitted `.isf` byte-identical and FSMGen-strict clean. Corpus coverage unchanged at 52/57, five remaining.
- Next action: `SIGNAL-CATALOG-CAPTURE-GAP.1` — classify the 33 empty-catalog documents as honest absence vs
  capture miss, starting with the five `protocol`-classed ones; that tree's frontier holds the census and the
  honesty guardrail. Alternative if a refresh is preferred: `CORPUS-COVERAGE.2.53` (its tree holds the pick
  criteria and the frontier/retention declarations a refresh must move in the same transaction).
- In-flight uncommitted: none. Replay/rebuild/snapshot evidence under
  `.project-data/tmp/semantic-empty-catalog-filter-1/` is disposable — delete freely (keep `*.log` out of the top
  level of `.project-data/tmp`, which `PROJECT-DATA-LOCALITY` fails closed on). No background job runs.
- Blockers: none. Two ledgers are tight: `CHANGES.md` at 1,598 of its 1,800-line health target, and
  `LIVE_ACHIEVEMENT_STATUS.md` at 69 of 80 records — its 90% rollover lands at 72, so three more product records
  before `STATUS-LEDGER-ROLLOVER` becomes mandatory. The user-owned `.claude/settings.json` is untouched.
