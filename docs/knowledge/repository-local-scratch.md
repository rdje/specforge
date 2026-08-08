---
id: repository-local-scratch
title: Diagnostic scratch data is project data and must stay on the repository volume
answers:
  - "where may diagnostic scratch files and comparison lists be written"
  - "why must read-only census commands avoid /tmp and /private/tmp"
  - "what proved that analysis commands can violate project data locality"
date: 2026-08-08
status: current
tags: [data-locality, diagnostics, temporary-files, portability]
evidence: docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md
reverify: test ! -e /private/tmp/specforge-task-files.txt && test ! -e /private/tmp/specforge-task-links.txt
---

A read-only `.5b` membership comparison accidentally redirected two derived path lists to
`/private/tmp`. The source operation was non-mutating, but the comparison lists were project-owned
diagnostic data and therefore violated the same-volume policy. Both exact files were deleted
immediately and an absence check passed; no shared cache or ambiguous path was touched.

The durable rule is broader than production builds: diagnostic lists, diff inputs, test fixtures,
logs, and command scratch are project data too. Commands must use pipelines/process substitution when
no persistence is needed, or derive a path under repository-local `generated/` when scratch must be
materialized. `.6` owns mechanical enforcement and the wider residue census.
