# Task-Tree Tracking System — README

This project tracks **all** work — every coding *and* non-coding activity,
task, slice, phase, and lane — as **task-trees**. This README states the
doctrine and points at the mechanics; the full workflow spec, active-tree
index, and PNT selection rules live in [`docs/TASK_TREE.md`](TASK_TREE.md).

## Why

Task-trees exist so the project **cleanly survives session loss, crashes, and
model/AI handoffs**, and so it holds a **signoff-level** bar. Every unit of
work leaves a durable, machine-tracked record of what was done, how it was
verified, and the commit that delivered it — so a fresh session can resume
correctly without re-reading chat history.

## The doctrine (non-negotiable)

1. **No code change without an owning task-tree — first.** Before *any* code
   change, however small, a task-tree leaf must own it. No compromise, no
   exception. Breaching this jeopardizes the project's signoff level.
2. **Every activity is tracked.** Coding and non-coding tasks, slices, lanes,
   and roadmap phases are all task-tree-owned. This is what preserves and
   enforces continuity between sessions.
3. **The whole roadmap is task-tree-tracked.** Every milestone/phase in
   [`ROADMAP.md`](../ROADMAP.md) maps to an owning task-tree.
4. **Past work is audited into trees.** Code changes made *before* the
   task-tree system existed are reconciled by creating their owning trees and
   recording a thorough, accurate, meticulous **audit** of what the codebase
   actually delivered — annotated back into the associated tree's Verification
   Log. (See [`docs/tasks/ROADMAP-TASKTREE-COVERAGE.md`](tasks/ROADMAP-TASKTREE-COVERAGE.md),
   the umbrella that drives this backfill.)
5. **ROADMAP ↔ codebase ↔ mdBook are locked — zero drift.** The three must
   always agree. The **mdBook** (`docs/book/`) is the user-facing surface into
   the project's features and capabilities and **must, at all times, reflect
   what the codebase actually does**. Drift is treated as a bug
   (the `AUDIT-DOC-RECONCILE` doctrine: the code is the truth; the text is the
   suspect and gets fixed).

## How it works (mechanics — see `docs/TASK_TREE.md` for the full spec)

- **One file per top-level tree** under [`docs/tasks/`](tasks/), copied from
  [`docs/tasks/TEMPLATE.md`](tasks/TEMPLATE.md).
- **`docs/TASK_TREE.md`** is the workflow spec + the derived bounded landing: every open tree,
  plus a route to the derived `docs/task-catalog/` parts that hold the complete catalog
  + the **PNT** (Pick-the-Next-Task) selection rules. The catalog is navigation only;
  `MEMORY.md` and each owning tree carry current execution state.
- **Nodes** are containers (have children) or **leaves** (the only unit PNT may
  implement). The **current frontier** is the ordered set of pickable leaves.
- **A commit that completes a leaf names the leaf ID** in its subject or first
  body line. The exact commit workflow + commit-time reporting is in
  [`COMMIT.md`](../COMMIT.md).
- **Completion Rules** (in `docs/TASK_TREE.md`) require every leaf to review the canonical
  documentation surfaces and update only those whose owned truth changed; ceremonial co-staging is
  forbidden. A tree's closing leaf must also satisfy the **`BOOK-METHOD-DOC` close-rule** by
  adding or refreshing the implementation+verification subsection in the topically-correct mdBook
  chapter. A close leaf whose book section is missing or stale is **incomplete**.

## Status vocabulary

`proposed` · `active` · `pending` · `in_progress` · `blocked` · `done` ·
`deferred` · `superseded` (defined in `docs/TASK_TREE.md`).

## Where to start

1. [`docs/TASK_TREE.md`](TASK_TREE.md) — workflow + complete catalog + PNT rules.
2. [`docs/tasks/ROADMAP-TASKTREE-COVERAGE.md`](tasks/ROADMAP-TASKTREE-COVERAGE.md)
   — the umbrella ensuring every roadmap milestone is owned + audited + locked
   to the code and the mdBook.
3. [`ROADMAP.md`](../ROADMAP.md) — the milestones each tree maps to.
