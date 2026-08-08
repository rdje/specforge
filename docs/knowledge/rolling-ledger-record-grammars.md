---
id: rolling-ledger-record-grammars
title: The four root rolling ledgers use three explicit whole-record grammars
answers:
  - "how are CHANGES DEVELOPMENT_NOTES LIVE_ACHIEVEMENT_STATUS and RUST_CODEBASE_ANALYSIS split into records"
  - "what is the rolling ledger archive protocol"
  - "why are two containment records at the bottom of CHANGES"
  - "what protects the validation projection when LIVE_ACHIEVEMENT_STATUS rolls over"
date: 2026-08-08
status: current
tags: [documentation, rolling-ledger, archive, continuity, validation]
evidence: doctrine/live_document_size/rolling_ledgers.jsonl
reverify: perl scripts/check_rolling_ledger_protocol.pl --report
---

`CHANGES.md` is a mixed grammar: leading modern `###` records, then legacy `##` records, plus the
exact `.0` and `.1` containment headings detached below the 2026-03-31 tail. `DEVELOPMENT_NOTES.md`
uses every H2 after its H1 as a record. `RUST_CODEBASE_ANALYSIS.md` keeps `## Purpose` in its
prologue and treats later H2 sections as records. `LIVE_ACHIEVEMENT_STATUS.md` treats each top-level
bullet between `## Current snapshot` and `## Highest-priority remaining gap` as one record; its
trailer includes the generated validation projection.

The bounded registry pins each grammar, source identity, planned newest-first whole-record window,
local limits, reader/writer paths, and repository-relative archive route. The checker reconstructs
every source byte-for-byte and verifies the planned live view. After migration it switches identity
authority to an immutable exact source capsule, verifies its manifest/index, enforces the bounded
root, and requires the retained migration records to remain an exact suffix behind any newly
prepended records. The validation writer's root path and managed markers are required literals.

`CHANGES.md` completed the first migration in `.4b`. Its immutable 1,798-record capsule is
`docs/archive/rolling-ledgers/changes/source-through-2026-08-08.md` with SHA-256
`d89809322857aab3c506dde1cc6caaf57e22d0349655b37ddaab1f7bf22ba994`; the root is the bounded current
view and `docs/archive/rolling-ledgers/INDEX.md` is the retrieval route. The other three ledgers remain
in measured `planned` state until their independent migration leaves.

`DEVELOPMENT_NOTES.md` completed the second migration in `.4c`. Its immutable 1,601-record capsule is
`docs/archive/rolling-ledgers/development-notes/source-through-2026-08-08.md` with SHA-256
`76b51a3f450cdb1e764922dc366cf6ff55529cb95f7a6f410bfba1f1f378fedc`; its root keeps the H1 prologue,
60 retained H2 records, and later prepends. `LIVE_ACHIEVEMENT_STATUS.md` and
`RUST_CODEBASE_ANALYSIS.md` remain measured `planned` ledgers for `.4d` and `.4e`.

For an H2 live cut, one blank line immediately before the removed successor is boundary structure,
not retained record content. The capsule preserves it exactly; the root renderer omits exactly that
one separator at EOF, and the suffix verifier permits no other difference. This prevents a
blank-at-EOF artifact without weakening content identity or normalizing archive bytes.
