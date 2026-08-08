---
id: roadmap-current-history-boundary
title: The roadmap has a pinned current/history migration boundary
answers:
  - "how will the oversized SpecForge roadmap be made bounded without losing its history"
  - "what is the exact pre-containment ROADMAP source identity"
  - "why can the roadmap not be split safely at Markdown headings"
  - "which stale roadmap statuses were found before migration"
date: 2026-08-08
status: current
tags: [roadmap, documentation, archive, containment, current-truth]
evidence: doctrine/live_document_size/roadmap_projection.json; docs/decisions/0010-bounded-current-roadmap-and-exact-history.md
reverify: perl scripts/check_roadmap_projection_contract.pl --report
---

The pre-containment `ROADMAP.md` is exactly 1,487 lines / 183,445 bytes with SHA-256
`20a71e88c15133398879fb11620dbb4c3ed20a6b689f2bcea733ef82b2a47d88`. Its 23 workstream records
mix current goals and remaining scope with 722 lines / 129,242 bytes of `done:` chronology, so a
heading-only partition would either discard current meaning or keep the delivery ledger live.

ADR 0010 therefore requires an exact immutable source capsule before the root is rewritten as a
bounded current-direction snapshot. The executable contract pins five exhaustive source regions, all
23 workstream ids and owning task routes, known readers/writer, archive/index/manifest topology, and
independent 384-line / 49,152-byte / 512-byte hard limits for the future root.

The audit also pins four current-truth defects for correction in the migration: stale `In Progress`
labels for R6 and R7, R14 still marked `Not Started` after its command tree closed, and a recommended-
order item asking for the already-complete R15 graph migration. The exact capsule preserves those
statements as history; the bounded root must not reproduce them as current direction.
