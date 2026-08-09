---
id: active-task-legacy-route-aliases
title: Active task legacy routes bind canonical IDs to exact source spellings
answers:
  - "why can an active task legacy route use a shorthand source literal"
  - "why does a PDF-VARIANT-DIGESTION commit subject id not appear fully qualified in the task source"
  - "what does source_literal mean in the active task evidence contract"
  - "which PDF task history ids exist only as tree-relative shorthand"
date: 2026-08-09
status: current
tags: [documentation, task-tree, containment, migration, provenance]
evidence: docs/tasks/ACTIVE-TASK-EVIDENCE-CONTAINMENT.md
reverify: perl scripts/check_active_task_evidence.pl --report
---

At boundary commit `f04db37ac0a4d9935f373b2d2887e647ba6f4c4d`, Git path subjects contribute 52 canonical
`PDF-VARIANT-DIGESTION` IDs. The final 2,393-line source uses mixed historical spelling. Six IDs—`.10p`,
`.12a`, `.12b`, `.13a`, `.13c`, and `.13d`—occur only in tree-relative form; another four use shorthand in
their primary semantic payload even though a fully qualified mention exists elsewhere.

A legacy route therefore records a fully qualified `leaf_id` for stable navigation and an exact
`source_literal` for provenance matching. The checker permits only the full ID or the same ID with the declared
tree prefix removed, and token-matches that literal inside the route's primary source-derived payload. Arbitrary
aliases, missing literals, mismatched payloads, omitted commit-history IDs, and post-migration use of
`source_literal` fail closed.

This distinction is required for lossless migration: normalizing the historical payload would change evidence,
while requiring only fully qualified source text would make the legitimate committed source impossible to
route. The bounded index and manifest still expose canonical fully qualified IDs.
