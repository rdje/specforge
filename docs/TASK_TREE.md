# Repo-Local Task Tree Workflow

This document defines the repo-local task-tree workflow used by SpecForge.
Adapted from the FSMGen task-tree system.

## Purpose

Use a task tree when a top-level task is too broad to finish safely as one
signoff-level slice, or when a task is expected to discover subtasks and
sub-subtasks over time.

The goal is not to create a second roadmap. The roadmap states the high-level
workstream direction. A task tree owns the recursive breakdown, current
frontier, acceptance criteria, blockers, decisions, validation, and completion
evidence for one top-level task.

## Active Task Trees

| Tree | Status | Roadmap lane | Current frontier | File |
| --- | --- | --- | --- | --- |
| `PROVENANCE-HARDENING` | `done` | `R6` | — | [docs/tasks/PROVENANCE-HARDENING.md](docs/tasks/PROVENANCE-HARDENING.md) |
| `R6-FSM-ADAPTER` | `superseded` | `R6` | — (superseded by `ISF-ONLY-CONSOLIDATION`) | [docs/tasks/R6-FSM-ADAPTER.md](docs/tasks/R6-FSM-ADAPTER.md) |
| `R6-SOURCE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-SOURCE-HARDENING.md](docs/tasks/R6-SOURCE-HARDENING.md) |
| `R6-CONVERGE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-CONVERGE-HARDENING.md](docs/tasks/R6-CONVERGE-HARDENING.md) |
| `R6-EVIDENCE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-EVIDENCE-HARDENING.md](docs/tasks/R6-EVIDENCE-HARDENING.md) |
| `R6-INTENT-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-INTENT-HARDENING.md](docs/tasks/R6-INTENT-HARDENING.md) |
| `R6-SEMANTIC-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-SEMANTIC-HARDENING.md](docs/tasks/R6-SEMANTIC-HARDENING.md) |
| `R6-PRIOR-MEMORY-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-PRIOR-MEMORY-HARDENING.md](docs/tasks/R6-PRIOR-MEMORY-HARDENING.md) |
| `R15-GRAPH-DIRECTION-MIGRATION` | `done` | `R15` | — | [docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md](docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md) |
| `R7-VALIDATION` | `done` | `R7` | — (.5 deferred) | [docs/tasks/R7-VALIDATION.md](docs/tasks/R7-VALIDATION.md) |
| `SIGNOFF-REMEDIATION` | `done` | `R0` | — | [docs/tasks/SIGNOFF-REMEDIATION.md](docs/tasks/SIGNOFF-REMEDIATION.md) |
| `R6-ISF-ADAPTER` | `done` | `R6` | — | [docs/tasks/R6-ISF-ADAPTER.md](docs/tasks/R6-ISF-ADAPTER.md) |
| `ISF-ONLY-CONSOLIDATION` | `done` | `R6` | — | [docs/tasks/ISF-ONLY-CONSOLIDATION.md](docs/tasks/ISF-ONLY-CONSOLIDATION.md) |
| `ISF-TEMPORAL-LOWERING` | `done` | `R15b` | — | [docs/tasks/ISF-TEMPORAL-LOWERING.md](docs/tasks/ISF-TEMPORAL-LOWERING.md) |
| `ISF-ONLY-IR-PRUNE` | `done` | `R6` | — | [docs/tasks/ISF-ONLY-IR-PRUNE.md](docs/tasks/ISF-ONLY-IR-PRUNE.md) |
| `AUDIT-DOC-RECONCILE` | `done` | `R0` | — | [docs/tasks/AUDIT-DOC-RECONCILE.md](docs/tasks/AUDIT-DOC-RECONCILE.md) |
| `FSMGEN-ISSUE-REPORTING` | `done` | `R0` | — | [docs/tasks/FSMGEN-ISSUE-REPORTING.md](docs/tasks/FSMGEN-ISSUE-REPORTING.md) |
| `FSMGEN-SUBMODULE-BUMP` | `done` | `R0` | — | [docs/tasks/FSMGEN-SUBMODULE-BUMP.md](docs/tasks/FSMGEN-SUBMODULE-BUMP.md) |
| `ISF-HANDSHAKE-STAGE-LOWERING` | `superseded` | `R15b` | — (delivered via `R16-CONTRACT-IR.4`) | [docs/tasks/ISF-HANDSHAKE-STAGE-LOWERING.md](docs/tasks/ISF-HANDSHAKE-STAGE-LOWERING.md) |
| `R16-INTENT-CAPTURE` | `active` | `R16` | `R16-INTENT-CAPTURE.2` (governance live; #1+#2 done, #5/order-3 promoted active) | [docs/tasks/R16-INTENT-CAPTURE.md](docs/tasks/R16-INTENT-CAPTURE.md) |
| `R16-CONTRACT-IR` | `done` | `R16` | — (program point #1 delivered) | [docs/tasks/R16-CONTRACT-IR.md](docs/tasks/R16-CONTRACT-IR.md) |
| `R16-KG-PROTOCOL-ONTOLOGY` | `done` | `R16` | — (program point #2 delivered `2026-05-20`; typed `protocol_graph` + projection + `validate` count surface; kg-bench fixtures deferred to extraction trees) | [docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md](docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md) |
| `R16-CAPTURE-FIDELITY-GATES` | `active` | `R16` | `R16-CAPTURE-FIDELITY-GATES.2` (`.1` design done; implement typed `fidelity` module + per-gate evaluators + trace primitive) | [docs/tasks/R16-CAPTURE-FIDELITY-GATES.md](docs/tasks/R16-CAPTURE-FIDELITY-GATES.md) |
| `R16-MULTIMODAL-CONTRACT-FUSION` | `proposed` | `R16` | — (point #3, order 4; deps: 1,2,3) | [docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md](docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md) |
| `R16-WAVEFORM-CONTRACT-MINING` | `proposed` | `R16` | — (point #4, order 5 — crux; deps: 1,3) | [docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md](docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md) |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION` | `proposed` | `R16` | — (point #6, order 6 — crux; deps: 1,3) | [docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md](docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md) |
| `BOOK-METHOD-DOC` | `active` | `R0` | `BOOK-METHOD-DOC.2` (backfill existing trees; `.1` done) | [docs/tasks/BOOK-METHOD-DOC.md](docs/tasks/BOOK-METHOD-DOC.md) |

## Directory Layout

```text
docs/TASK_TREE.md
docs/tasks/
  TEMPLATE.md
  <TREE>.md
```

`docs/TASK_TREE.md` is the workflow and active-tree index.
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
2. Read the active task file named in the `Active Task Trees` table.
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
- `MEMORY.md`, `CHANGES.md`, `DEVELOPMENT_NOTES.md`,
  `LIVE_ACHIEVEMENT_STATUS.md`, and `ROADMAP.md` are updated when the
  leaf changes project state.
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
