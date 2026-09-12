# Repo-Local Task Tree Workflow

This document defines the repo-local task-tree workflow used by SpecForge.
Adapted from the FSMGen task-tree system.

See [`docs/TASK_TREE_README.md`](TASK_TREE_README.md) for the system overview
and the governing doctrine (no code change without an owning tree; every
activity tracked; past work audited into trees; ROADMAP ↔ codebase ↔ mdBook
locked with zero drift). Whole-roadmap task-tree coverage + the past-work audit
is driven by [`docs/tasks/ROADMAP-TASKTREE-COVERAGE.md`](tasks/ROADMAP-TASKTREE-COVERAGE.md).

## Purpose

Use a task tree when a top-level task is too broad to finish safely as one
signoff-level slice, or when a task is expected to discover subtasks and
sub-subtasks over time.

The goal is not to create a second roadmap. The roadmap states the high-level
workstream direction. A task tree owns the recursive breakdown, current
frontier, acceptance criteria, blockers, decisions, validation, and completion
evidence for one top-level task.

<!-- task_catalog:start -->
## Task Tree Catalog

This derived catalog is navigation, not execution history. Read `MEMORY.md` for the single
resume pointer, then open the owning tree for its frontier, decisions, evidence, and commits.
The author template is linked separately and is never classified as active work.

### Open trees

Every tree that is not `done` or `superseded` — the set a session can still act on. There is no
limit on how many task-trees exist; a finished tree stays as project history in the parts below
(ADR 0045).

| Tree | Status | Purpose | File |
| --- | --- | --- | --- |
| `ACTOR-NOUN-RELATION-DECLARATION` | `active` | an inferred declaration mints an ordinary word as a wire | [open](tasks/ACTOR-NOUN-RELATION-DECLARATION.md) |
| `CHANGES-LEDGER-ROLLOVER` | `active` | roll the change ledger before its next append is refused | [open](tasks/CHANGES-LEDGER-ROLLOVER.md) |
| `CLAIM-VERIFICATION-ADOPTION` | `active` | adopt three-leg verification for published claims | [open](tasks/CLAIM-VERIFICATION-ADOPTION.md) |
| `CORPUS-COVERAGE` | `active` | build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale | [open](tasks/CORPUS-COVERAGE.md) |
| `CORPUS-HARDENING` | `active` | harden SpecForge against the real chip-doc corpus (AMBA core first) | [open](tasks/CORPUS-HARDENING.md) |
| `CORPUS-PATTERN-REUSE` | `active` | reuse extraction patterns across PDFs, clustered by derived vendor/layout fingerprint | [open](tasks/CORPUS-PATTERN-REUSE.md) |
| `DOC-INTENT-TAXONOMY` | `active` | chip-spec document intent taxonomy → per-category complete ISF synthesis | [open](tasks/DOC-INTENT-TAXONOMY.md) |
| `EXTRACTION-GAP-FIX` | `active` | close the extraction gaps PDF-VARIANT-DIGESTION.4 quantified | [open](tasks/EXTRACTION-GAP-FIX.md) |
| `EXTRACTION-QUALITY-GAUGE` | `active` | measure the extraction-quality gap — and CHI's is large | [open](tasks/EXTRACTION-QUALITY-GAUGE.md) |
| `INVARIANT-SHAPE-ADMISSION` | `active` | 739 captions are a precision defect; the table rows are an extraction gap, and they are not the same problem | [open](tasks/INVARIANT-SHAPE-ADMISSION.md) |
| `KG-ISF-COMPLETENESS` | `active` | the KG/IntentIR must be COMPLETE enough to lower faithfully to ISF | [open](tasks/KG-ISF-COMPLETENESS.md) |
| `KG-ISF-TRANSACTIONS` | `active` | every supported protocol transaction + its signals, fully captured & ISF-ready | [open](tasks/KG-ISF-TRANSACTIONS.md) |
| `LIVE-DOCUMENT-PRESSURE-HEADROOM` | `active` | keep current-facing canonical surfaces writable | [open](tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md) |
| `MEMORY-BOUNDED-INGEST` | `active` | bounded-memory ingestion of very large PDFs | [open](tasks/MEMORY-BOUNDED-INGEST.md) |
| `NLP-SHALLOW-PARSE` | `active` | a deterministic in-Rust shallow-parse tier (subject–verb–object understanding) | [open](tasks/NLP-SHALLOW-PARSE.md) |
| `PDF-VARIANT-DIGESTION` | `active` | make SpecForge digest as many chip-spec PDF variants as possible | [open](tasks/PDF-VARIANT-DIGESTION.md) |
| `PRODUCTION-GRAPH-CENSUS-PIN` | `active` | four repository-wide census numbers drifted under a fully green gate | [open](tasks/PRODUCTION-GRAPH-CENSUS-PIN.md) |
| `PROSE-NAME-CELL-DECLARATION` | `active` | a row whose name cell is a phrase declares its first word as a signal | [open](tasks/PROSE-NAME-CELL-DECLARATION.md) |
| `PROVIDER-MODEL-STORE-LOCALITY` | `active` | decide and gate where the VLM/NLP model store lives | [open](tasks/PROVIDER-MODEL-STORE-LOCALITY.md) |
| `PURE-NLP-INTENT-EXTRACTION` | `active` | model-based intent extraction (ACTIVE — first increment) | [open](tasks/PURE-NLP-INTENT-EXTRACTION.md) |
| `R15C-R15G-LEARNING-PLANE-BACKFILL` | `active` | own + audit the learning / eval / corpus lanes (in-progress) | [open](tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| `RETAINED-BUNDLE-POPULATION-FROZEN` | `active` | the retained normalized-bundle set can neither grow nor shrink | [open](tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md) |
| `SCRATCH-RESIDUE-CONTAINMENT` | `active` | reclaim repository scratch that nothing can reach | [open](tasks/SCRATCH-RESIDUE-CONTAINMENT.md) |
| `SIGNAL-CATALOG-CAPTURE-GAP` | `active` | protocol specifications that declare no signals at all | [open](tasks/SIGNAL-CATALOG-CAPTURE-GAP.md) |
| `SIGNAL-DECLARATION-ROW-DROP` | `active` | the authoritative declaration reader silently discards 18.3% of the rows it was given | [open](tasks/SIGNAL-DECLARATION-ROW-DROP.md) |
| `SOURCE-IR-REPRODUCIBILITY` | `active` | make SourceIR ingest reproducible, and gate it | [open](tasks/SOURCE-IR-REPRODUCIBILITY.md) |
| `SPEC-CLARIFICATION-LOOP` | `active` | autonomous-first, user-assisted specification completion | [open](tasks/SPEC-CLARIFICATION-LOOP.md) |
| `SPEC-TO-INTENT-ALIGNMENT` | `active` | steer PDF evidence into complete executable-intent IR | [open](tasks/SPEC-TO-INTENT-ALIGNMENT.md) |
| `STATUS-LEDGER-ROLLOVER` | `active` | roll the status ledger before its next product record is refused | [open](tasks/STATUS-LEDGER-ROLLOVER.md) |
| `TASK-PART-SEAL-REACHABILITY` | `active` | give a completed task-evidence part a reachable closed state | [open](tasks/TASK-PART-SEAL-REACHABILITY.md) |
| `WIRE-BASED-100` | `active` | max every score (100%) on wire-based interface specs (APB/AHB/AXI/SWD/…) | [open](tasks/WIRE-BASED-100.md) |

### Complete catalog

All 158 trees route through 3 derived part(s); open a range to find an id.

| Part | Trees | First id | Last id |
| --- | ---: | --- | --- |
| [0001](task-catalog/catalog-0001.md) | 56 | `ACTIVE-TASK-EVIDENCE-CONTAINMENT` | `FSMGEN-REFRESH-INTEGRATE-8` |
| [0002](task-catalog/catalog-0002.md) | 56 | `FSMGEN-REFRESH-INTEGRATE` | `R16-CONTRACT-IR` |
| [0003](task-catalog/catalog-0003.md) | 46 | `R16-INTENT-CAPTURE` | `WIRE-BASED-100` |

Authoring template: [`docs/tasks/TEMPLATE.md`](tasks/TEMPLATE.md).

<!-- task_catalog:end -->

## Directory Layout

```text
docs/TASK_TREE.md
docs/task-catalog/
  catalog-0001.md
  <catalog-NNNN.md>
docs/tasks/
  TEMPLATE.md
  <TREE>.md
```

`docs/TASK_TREE.md` is the workflow spec plus a bounded landing: it lists every **open** tree and
routes to the derived catalog parts for the rest, so it grows with work in flight rather than with
project lifetime. `docs/task-catalog/` holds those parts; both the landing section and the parts are
generated by `perl scripts/check_task_tree_catalog.pl --write` and verified by derive-and-diff — never
hand-edited. There is no limit on the number of task-trees (ADR 0045).
Each top-level task owns one file in `docs/tasks/`.
`docs/tasks/TEMPLATE.md` is copied when creating a new top-level tree.

## Definitions

- Task tree: the recursive decomposition of one top-level task.
- Node: one item in that tree.
- Container node: a node with children. It is not directly executable.
- Leaf node: a node with no children. It is the only unit PNT may implement.
- Current frontier: the ordered set of leaf nodes that are eligible to be
  picked next.
- Slice: one completed leaf task plus its tests, docs, live-doc updates, and
  commit workflow.
- Evidence: the validation output, changed-doc summary, and git commit subject
  that prove a leaf was completed.

## ID Rules

Each task tree has a stable top-level ID.

```text
<TREE>
<TREE>.1
<TREE>.1.1
<TREE>.1.1.1
```

Rules:

- `<TREE>` uses uppercase letters, digits, and hyphens.
- Child IDs append dot-separated positive integers.
- IDs are permanent once published.
- Never renumber closed nodes.
- If a new ordering is needed, add new IDs and mark old nodes `superseded` or
  `deferred` with a reason.
- A commit that completes a task-tree leaf must identify the leaf ID in the
  commit subject or in the first body line.

## Status Vocabulary

Use only these statuses.

| Status | Meaning |
| --- | --- |
| `proposed` | Captured but not yet accepted into the active tree. |
| `active` | The top-level tree is open, or a container has unfinished children. |
| `pending` | Ready to be selected once it reaches the current frontier. |
| `in_progress` | Currently being implemented in the worktree. |
| `blocked` | Cannot proceed without a named blocker and unblock condition. |
| `done` | Completed, validated, documented, and committed. |
| `deferred` | Deliberately postponed with an explicit consequence. |
| `superseded` | Replaced by another node, with the replacement ID named. |

## Required Task File Sections

Every top-level task file must contain:

- Metadata: tree ID, status, roadmap lane, created date, last updated date.
- Goal: the user-visible or project-visible outcome.
- Non-goals: what this tree deliberately does not try to solve.
- Acceptance criteria: concrete conditions that close the top-level task.
- Task tree: all known nodes, with status and short result intent.
- Current frontier: ordered leaf nodes that PNT may select next.
- Decisions: accepted technical decisions and their rationale.
- Open questions: unresolved questions that do not block the whole tree yet.
- Blockers: blockers with unblock conditions.
- Verification log: checks run for completed leaves.
- Commit log: leaf IDs mapped to completion commit subjects.
- Changelog: dated edits to the tree itself.

## Node Rules

Every node must be one of these two shapes.

Container node:

```text
- ID: <TREE>.<n>
  Status: active
  Goal: ...
  Children: <TREE>.<n>.1, <TREE>.<n>.2
```

Leaf node:

```text
- ID: <TREE>.<n>
  Status: pending
  Goal: ...
  Acceptance: ...
  Verification: pending
  Commit: pending
```

A node with children must not be marked `done` until every child is `done`,
`deferred`, or `superseded`, and every non-`done` child has a recorded reason.

## Current Frontier Rules

The current frontier is the only list PNT uses when selecting work from a task
tree.

Rules:

- The frontier contains only leaf nodes.
- The frontier is ordered by intended priority.
- A container never appears in the frontier.
- A blocked node stays out of the frontier until unblocked.
- When a leaf is split, remove that leaf from the frontier, mark it `active`,
  add children, and place the first executable child or children in the
  frontier.
- When a leaf completes, remove it from the frontier and add the next eligible
  leaf or leaves.

## PNT Selection Rules

When PNT is asked to continue and at least one active task tree exists:

1. Read `docs/TASK_TREE.md`.
2. Read the active task file named in the `Open trees` table.
3. Pick the first eligible leaf in that file's `Current Frontier`.
4. Implement only that leaf.
5. If the leaf is too broad, split it before implementation and commit the
   tree update as the leaf's honest outcome.
6. Run the required validation for the leaf.
7. Update the task file, live docs, and roadmap if status changed.
8. Run the full commit workflow before selecting another leaf.

If several active trees exist, choose the first active tree in the table unless
the user names another tree or the roadmap status names a different immediate
lane.

## Splitting Rules

Split a node when any of these are true:

- It cannot be completed to signoff quality in one slice.
- It mixes design, implementation, diagnostics, tests, and docs in ways that
  can be reviewed independently.
- It hides an unresolved policy choice behind implementation wording.
- It would require touching unrelated ownership areas in one commit.
- It discovers a lower-level dependency that should be solved first.

Do not split merely to create vague placeholders. Every child must have a
clear goal and a way to verify completion.

## Completion Rules

A leaf is complete only when all of the following are true:

- Implementation or documentation work for that leaf is finished.
- Focused checks passed, and broader checks ran when warranted.
- The owning task file records the result, validation, and commit subject.
- The canonical documentation surfaces named by `COMMIT.md` are reviewed. Update each only when
  the truth it owns changed; no-op or ceremonial co-staging is not completion evidence.
- **Book method-doc close-rule** (`BOOK-METHOD-DOC`): a tree's
  closing leaf (`.N`) MUST add or refresh that tree's
  implementation+verification subsection in the topically-correct
  mdBook chapter (per the placement map in
  `docs/tasks/BOOK-METHOD-DOC.md`). The subsection is the
  human-facing "how + why + how-verified"; the task-tree file
  remains the machine-tracked authority. A close leaf whose book
  section is missing or stale is incomplete — the book / ROADMAP
  language always describes what the code does (the
  `AUDIT-DOC-RECONCILE` doctrine).
- The commit workflow in `COMMIT.md` has completed.
- `git_message_brief.txt` has been cleared after commit.

Commit hashes are intentionally not required inside the same task-file update:
the final hash cannot be known until after the commit exists. The stable
join key is the leaf ID in the commit subject or first body line. Later status
refreshes may backfill hashes if useful.

## Blocker Rules

A blocked node must record:

- the exact blocker,
- why it blocks the node,
- the unblock condition,
- and the next task that should run instead, if any.

Do not leave a node as `blocked` only because it is large or unclear. Large or
unclear work should be split until a real blocker is visible.

## Relationship To Live Docs

The task tree is the detailed execution ledger.

- `ROADMAP.md` remains the canonical high-level workstream status.
- `MEMORY.md` remains the recovery/handoff continuity log.
- `CHANGES.md` remains the chronological technical history.
- `DEVELOPMENT_NOTES.md` remains design rationale.
- `LIVE_ACHIEVEMENT_STATUS.md` remains the latest completed slice summary.
- The mdBook remains user-facing product/language documentation.

Do not duplicate the whole task tree into those files. Link to the task tree
and summarize only the part that changes live project state.

## SpecForge-Specific Defaults

- All R6 `.fsm` adapter hardening and provenance-field hardening work is
  task-tree-managed by default.
- Active PNT cycles select from the first active tree's current frontier
  unless the user names a different lane.
- The canonical validation command is `cargo test -p specforge --lib`.
- Commit messages for task-tree-managed leaves must include the leaf ID
  (e.g. `PROV-HARDEN.4`) in the commit subject or first body line.
