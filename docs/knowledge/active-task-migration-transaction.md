---
id: active-task-migration-transaction
title: Active task migration is a byte-safe root-last transaction
answers:
  - "how is an active task evidence migration written atomically"
  - "why is the bounded active task root written last"
  - "how does the active task migration preserve non-ASCII legacy bytes"
  - "what does the active task migration roll back after a validation failure"
date: 2026-08-09
status: current
tags: [documentation, task-tree, containment, migration, provenance]
evidence: docs/tasks/ACTIVE-TASK-EVIDENCE-CONTAINMENT.md
reverify: perl scripts/check_active_task_evidence.pl --report
---

`scripts/check_active_task_evidence.pl --migrate ROOT_TEMPLATE` materializes the ADR 0019 active-task topology
from contract data. It first validates the committed `source_locked/complete` state, destination absence, the
same-volume root template, fixed bounds, required/forbidden literals, and its sole local index route. It renders
the exact source capsule, seven marker-delimited semantic parts, bounded index, manifest, and migrated contract,
then writes the reviewed current root last. The final migrated-state validation must pass before the transaction
is accepted.

Legacy payloads are raw source bytes, not decoded text. Rendered Unicode scaffolds are explicitly encoded to
UTF-8 before those raw slices are appended. This boundary matters: concatenating raw UTF-8 source bytes into an
already character-upgraded Perl scalar can reinterpret and double-encode non-ASCII history while leaving ASCII
fixtures green. The focused writer fixture therefore includes a non-ASCII legacy heading and compares every
extracted payload byte-for-byte.

If a write or final validation fails, the materializer restores the original root and contract and removes only
the two destination directories whose preflighted absence proves that it owns them. The real migration leaves a
106-line / 5,295-byte active root, a 79-line / 5,053-byte index, seven parts totaling 2,473 lines / 225,132 bytes,
and an exact 2,393-line / 222,616-byte capsule at SHA-256
`9284dce40ad896c3de3811e95c3fdd347132b1849083499e9c543fc9026a19d4`. Focused cases likewise remove their
PID-scoped repository workspace before rethrowing any unexpected failure, so the suite does not strand residue.
