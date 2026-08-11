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
- Active unit: `SIGNAL-CATALOG-CAPTURE-GAP.2` — `.0` (ownership + census) and `.1` (classification) done, no
  code touched yet. Tracking-only: `STATUS-LEDGER-ROLLOVER` `.2`, `TASK-PART-SEAL-REACHABILITY` `.0`.
- Current state: `.1` classified all 33 empty-catalog documents — **32 honest absence, 1 capture miss**. No
  empty-catalog document has an identity-header `signal_description` table, so the table gate is not the cause;
  `wbspec_b4_wishbone_b4_specification` alone is a real miss and declares its wires as 32 section *headings*.
  Two corpus facts fell out: markdown escaping (`\_`) in the normalized text EvidenceIR reads truncates
  underscore-bearing identifiers on 67 of 78 documents (`.4`), and 14 of 78 evidence artifacts carry no
  validation report, hence no `document_class` (`.5`). Ingest is complete on all 77 available sources.
- Next action: **roll the `changes` ledger first** — it is now mandatory (below), and `.1`'s ledger entry is
  deferred into that slice; its content is fully recoverable from the `.1` task-tree block and the research
  record. Then `SIGNAL-CATALOG-CAPTURE-GAP.2`: design the heading-as-declaration rule and measure what it would
  admit across all 78 before writing code — a title shape alone is measured unsafe, so the rule must be
  corroborative (title names an identifier **and** body states a direction). `.4` is a valid smaller slice.
- In-flight uncommitted: none. Census/probe scripts under `.project-data/tmp/signal-catalog-capture-gap-1/`
  are disposable — delete freely (keep `*.log` out of the top level of `.project-data/tmp`, which
  `PROJECT-DATA-LOCALITY` fails closed on). No background job runs.
- Blockers: **`CHANGES.md` rollover is mandatory before its next append.** At 1,598 lines it is 88.8% of its
  1,800-line health target; the 90% trigger lands at 1,620, so any entry of ~22 lines or more fails
  `LIVE-DOC-SIZE` (measured: a 31-line entry gave 1,629 = 90.5%). Follow the `COMMIT.md` rolling-ledger
  protocol. `LIVE_ACHIEVEMENT_STATUS.md` is second at 69 of 80 records, rollover at 72
  (`STATUS-LEDGER-ROLLOVER`). The user-owned `.claude/settings.json` is untouched.
