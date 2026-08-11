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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.1`; `.0` is complete. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: the specification-to-executable-intent endpoint is unchanged, but the current program
  constraint is faithful PDF content population through SourceIR, EvidenceIR, SemanticIR, and IntentIR — not
  speculative ISF expansion (ADR 0033). The architecture is directionally convergent but the product is not
  semantically complete: all four persisted IR stages have 78 artifacts and only 44 emit `.isf`, while the
  default `converge` path does not directly run every production extractor and the typed `FigureRegion`
  surface still has no real producer. ADR 0034 and `docs/research/specforge-trajectory-control.md` specify a
  reviewable, non-scalar convergence controller; no runtime controller ships yet.
- Next action: define `.1`'s machine-readable, per-document-category EvidenceIR-to-IntentIR content contract,
  including required modalities, typed surfaces, provenance/residual outcomes, and measurable recall and
  precision floors. Use it to rank the already-owned Wishbone heading capture (`SIGNAL-CATALOG-CAPTURE-GAP.2`)
  and normalized Markdown identifier loss (`.4`) before composing the canonical workflow in `.2`.
- In-flight uncommitted: none after the `.0` commit. No background job runs.
- Blockers: **`CHANGES.md` rollover is mandatory before its next append.** At 1,598 lines it is 88.8% of its
  1,800-line health target; the 90% trigger lands at 1,620, so any entry of ~22 lines or more fails
  `LIVE-DOC-SIZE` (measured: a 31-line entry gave 1,629 = 90.5%). Follow the `COMMIT.md` rolling-ledger
  protocol. `LIVE_ACHIEVEMENT_STATUS.md` is second at 70 of 80 records, rollover at 72
  (`STATUS-LEDGER-ROLLOVER`). Avoid further append-only growth in `RUST_CODEBASE_ANALYSIS.md`, now near its
  90% health trigger. The user-owned `.claude/settings.json` is untouched.
