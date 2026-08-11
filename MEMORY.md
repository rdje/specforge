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
- Current state: `.1` classified all 33 empty-catalog documents — **32 honest absence, 1 capture miss**; the
  table gate is not the cause (no such document has an identity-header table). Wishbone alone is a real miss
  and declares its wires as 32 section *headings*. Two corpus facts fell out: the `\_` escape in the
  normalized text EvidenceIR reads truncates identifiers on 67 of 78 documents (`.4`), and 14 of 78 evidence
  artifacts carry no validation report, hence no `document_class` (`.5`).
- Real open owner decision (pre-existing, not new): `KG-ISF-COMPLETENESS.2a` is `deferred` **with-trigger** —
  `.isf` direction/width fidelity needs an FSMGen-contract check on `(width PARAM)`, a reference-boundary
  design, and an **owner decision**, because carrying direction/width is in tension with the `2026-06-16`
  north star. `SIGNOFF-BURNDOWN` and `MEASUREMENT-PLANE-CONVERGENCE-RISK` were both withdrawn `2026-08-11`
  as archaeology over that leaf; ignore them except as a cautionary record.
- Next action: **roll the `changes` ledger** — mandatory (below); `SIGNAL-CATALOG-CAPTURE-GAP.1`'s ledger
  entry is deferred into it. Then `SIGNAL-CATALOG-CAPTURE-GAP.2` (corroborative heading-as-declaration rule)
  or `.4` (the markdown escape). **Grep `KNOWLEDGE_MAP.md` before forming any diagnosis, not only before
  touching code** — that omission produced two withdrawn trees in one session.
- In-flight uncommitted: none. Scripts under `.project-data/tmp/signal-catalog-capture-gap-1/` are disposable
  (keep `*.log` out of `.project-data/tmp`'s top level). No background job runs.
- Blockers: **`CHANGES.md` rollover is mandatory before its next append.** At 1,598 lines it is 88.8% of its
  1,800-line health target; the 90% trigger lands at 1,620, so any entry of ~22 lines or more fails
  `LIVE-DOC-SIZE` (measured: a 31-line entry gave 1,629 = 90.5%). Follow the `COMMIT.md` rolling-ledger
  protocol. `LIVE_ACHIEVEMENT_STATUS.md` is second at 69 of 80 records, rollover at 72
  (`STATUS-LEDGER-ROLLOVER`). The user-owned `.claude/settings.json` is untouched.
