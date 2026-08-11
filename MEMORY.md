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
- Active unit: none — `SEMANTIC-EMPTY-CATALOG-FILTER` is `done` and closed at `.2`. Open, tracking-only:
  `STATUS-LEDGER-ROLLOVER` at `.2`, `TASK-PART-SEAL-REACHABILITY` at `.0`.
- Current state: the closed tree deleted the `declared_signal_names.is_empty()` special case in
  `crates/specforge/src/ir/semantic.rs`, so **one grounding predicate now governs every document** and a rejected
  record is demoted to a `semantic_ungrounded_records_not_promoted` residual packet instead of dropped. All 78
  downstream chains were rebuilt from unchanged EvidenceIR and the 179-artifact validation population re-validated,
  so `CHAIN-CURRENCY` is green at 24/78/78/78. Corpus coverage is unchanged at **52/57 with five real documents
  remaining**; all 44 emitted `.isf` are **byte-identical** and FSMGen-strict clean at 0 diagnostics.
- Next action: own `CORPUS-COVERAGE.2.53` — select refresh #53 from the five gated remaining. `CORPUS-FRONTIER`
  (gate-tier, `bash scripts/check_corpus_frontier.sh`) re-derives 57 = 52 + 5 on every run, so **a refresh must
  move `doctrine/corpus_frontier/census.json` and the root prose in the same transaction or it fails closed**, and
  must add its key to `doctrine/chain_currency/retained_bundles.json` (now 24).
- Details for that pick: smallest of the five gated remaining is
  `opencapi_3_0_transaction_layer_28jan2020` at 774 elements; re-measure all five from their own persisted
  SourceIR `document_profile` before committing to that pick. Append to `refreshes-51-56` (440/640 lines, 68.8%
  — a third refresh at the largest observed 295-line cost would overshoot the 90% rollover, so measure before
  writing and split at the boundary if it would).
- In-flight uncommitted: none. Replay/rebuild/snapshot evidence lives under
  `.project-data/tmp/semantic-empty-catalog-filter-1/` and is disposable — delete it freely (keep `*.log` out of
  the top level of `.project-data/tmp`, which `PROJECT-DATA-LOCALITY` fails closed on). No background job runs.
- Blockers: none. Two ledgers are tight: `CHANGES.md` at 1,598 of its 1,800-line health target (89%), and
  `LIVE_ACHIEVEMENT_STATUS.md` at 69 of 80 records (86%) — its 90% rollover lands at 72, so **three more product
  records before `STATUS-LEDGER-ROLLOVER` becomes mandatory**. The closed tree left one tracked open question: the
  documents whose signal catalogs were never captured (Wishbone `CLK`/`CYC`/`STB`, DTI `TDATA`/`TKEEP`/`TLAST`,
  USB `ACK`/`ERDY`/`NRDY` are demoted as ungrounded because nothing declared them) — extraction-breadth lane, not
  a grounding defect. The user-owned `.claude/settings.json` is untouched.
