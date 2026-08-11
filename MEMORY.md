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
- Active unit: `SEMANTIC-EMPTY-CATALOG-FILTER.2` — the closing leaf. `.1` is `done` and committed (the code
  change landed). Also open, tracking-only: `STATUS-LEDGER-ROLLOVER` at `.2`, `TASK-PART-SEAL-REACHABILITY` at `.0`.
- Current state: `.1` deleted the `declared_signal_names.is_empty()` special case in
  `crates/specforge/src/ir/semantic.rs`, so **one grounding predicate now governs every document** and a rejected
  record is demoted to a `semantic_ungrounded_records_not_promoted` residual packet instead of dropped. All 78
  downstream chains were rebuilt from unchanged EvidenceIR and the 179-artifact validation population re-validated,
  so `CHAIN-CURRENCY` is green at 24/78/78/78. Corpus coverage is unchanged at **52/57 with five real documents
  remaining**; all 44 emitted `.isf` are **byte-identical** and FSMGen-strict clean at 0 diagnostics.
- Next action: land `SEMANTIC-EMPTY-CATALOG-FILTER.2` — the closing leaf. The corpus-wide evidence is already
  measured and recorded in the tree (11 identical / 41 residual-packet-only / 26 content-moved, all 26
  empty-catalog; `kg-bench` 156/156; nine provider-free evals at baseline; `run_ci.sh` green). What remains is the
  write-up: the `BOOK-METHOD-DOC` subsection in `docs/book/src/pipeline/semanticir.md`, refreshing the fact card
  `docs/knowledge/semantic-empty-catalog-disables-grounding-filter.md` to `.1`'s landed rule, one `CHANGES.md`
  entry for the whole tree, `LIVE_ACHIEVEMENT_STATUS.md`, and closing the tree.
- Deferred behind it: `CORPUS-COVERAGE.2.53`. Smallest of the five gated remaining is
  `opencapi_3_0_transaction_layer_28jan2020` at 774 elements; re-measure all five from their own persisted
  SourceIR `document_profile` before committing to that pick. Append to `refreshes-51-56` (440/640 lines, 68.8%
  — a third refresh at the largest observed 295-line cost would overshoot the 90% rollover, so measure before
  writing and split at the boundary if it would).
- In-flight uncommitted: none. Replay/rebuild/snapshot evidence lives under
  `.project-data/tmp/semantic-empty-catalog-filter-1/` and is disposable — delete it freely (keep `*.log` out of
  the top level of `.project-data/tmp`, which `PROJECT-DATA-LOCALITY` fails closed on). No background job runs.
- Blockers: none. `CHANGES.md` is at 1,566 of its 1,800-line health target (87%) — write **one** entry for this
  whole tree at `.2`, not one per leaf, to stay under the 1,620-line mandatory rollover. `.1` opened a new tracked
  question (the missing signal catalogs its demotion made visible); it belongs to the extraction-breadth lane, not
  this tree. The user-owned `.claude/settings.json` is untouched.
