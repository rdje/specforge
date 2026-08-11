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
- Active unit: none. `CORPUS-COVERAGE.2.52` is `done` and committed. Open trees, all tracking-only:
  `SEMANTIC-EMPTY-CATALOG-FILTER` at `.0`, `STATUS-LEDGER-ROLLOVER` at `.2`, `TASK-PART-SEAL-REACHABILITY` at `.0`.
- Current state: corpus coverage is **52/57 with five real documents remaining**, at 78 SourceIR / 24
  normalized / 78 EvidenceIR / 78 downstream chains, 44/44 emitted ISFs FSMGen-strict clean. `CORPUS-FRONTIER`
  (gate-tier, `bash scripts/check_corpus_frontier.sh`) re-derives 57 = 52 + 5 on every run, so **a refresh must
  move `doctrine/corpus_frontier/census.json` and the root prose in the same transaction or it fails closed**;
  a refresh must likewise add its key to `doctrine/chain_currency/retained_bundles.json` (now 24).
- Next action: select and own `CORPUS-COVERAGE.2.53`. The smallest of the five gated remaining documents is
  `opencapi_3_0_transaction_layer_28jan2020` at 774 elements; re-measure all five from their own persisted
  SourceIR `document_profile` before committing to that pick. Append to `refreshes-51-56` (440/640 lines, 68.8%
  — a third refresh at the largest observed 295-line cost would overshoot the 90% rollover, so measure before
  writing and split at the boundary if it would).
- In-flight uncommitted: none. Rollback/run snapshots and logs live under `.project-data/tmp/corpus-coverage-2-52-*`
  and are disposable — delete them freely (keep `*.log` out of the top level of `.project-data/tmp`, which
  `PROJECT-DATA-LOCALITY` fails closed on). No background job is running.
- Blockers: none. `SEMANTIC-EMPTY-CATALOG-FILTER` is a measured, pre-existing defect on 29 documents that reaches
  no emitted target; it blocks no refresh. `CHANGES.md` is near 87% of its 1800-line health target — roughly
  four commits before its 90% rollover becomes mandatory. The user-owned `.claude/settings.json` is untouched.
