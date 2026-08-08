---
id: task-tree-catalog
title: The task-tree catalog is a bounded derived navigation layer
answers:
  - "how is docs TASK_TREE kept complete without mirroring task history"
  - "why is TEMPLATE excluded from the task catalog"
  - "how do I verify every task tree is linked exactly once"
date: 2026-08-08
status: current
tags: [task-tree, navigation, generated-index, continuity]
evidence: scripts/check_task_tree_catalog.pl
reverify: perl scripts/check_task_tree_catalog.pl --check
---

`docs/TASK_TREE.md` contains a derived catalog with one concise row for every real immediate
`docs/tasks/*.md` file. Each row is generated from the safe filename, matching H1 tree id, first
metadata status, and H1 title. `TEMPLATE.md` is an authoring resource, so it has a separate link and
is never represented as active work.

The catalog does not copy frontiers, decisions, verification logs, or commit history. `MEMORY.md`
holds the single resume pointer; the selected task file remains canonical for execution detail. The
checker enforces derive-and-diff, closed statuses, exact membership, ordering, row/section/title
bounds, and runs through `LIVE-DOC-SIZE` on every doctrine/CI gate.
