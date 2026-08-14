---
id: memory-resume-pointer-byte-cap
title: MEMORY.md has an exact 32,768-byte one-read ceiling
answers:
  - "What is the maximum file size of MEMORY.md?"
  - "Why is the MEMORY.md byte cap 32,768 bytes?"
  - "Did raising the MEMORY.md byte cap relax its line or overwrite-only rules?"
date: 2026-08-15
status: current
tags: [memory, continuity, live-document-containment, doctrine]
evidence: scripts/check_memory_architecture.sh; doctrine/live_document_size/surfaces.jsonl; MEMORY_ARCHITECTURE.md; LIVE_DOCUMENT_SIZE_CONTAINMENT.md; docs/tasks/MEMORY-RESUME-POINTER-BYTE-CAP.md
reverify: "scripts/check_memory_architecture.sh && perl scripts/check_live_document_size.pl --no-history --report"
---

`MEMORY.md` has an exact maximum file size of 32,768 bytes (`32 * 1024 B`). The memory-architecture checker and
the `active_resume` live-document surface enforce the same value, keeping the complete resume pointer within one
bounded read.

The byte increase does not turn the pointer into a log. Independent limits remain 50 lines and 160 bytes per
content line, and the file remains overwrite-only current state. Chronology belongs in Git and task-tree logs;
durable facts and decisions belong in their canonical repository layers.
