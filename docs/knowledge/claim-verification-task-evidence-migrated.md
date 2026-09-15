---
id: claim-verification-task-evidence-migrated
title: Partitioning a task tree costs four surface records, and the registry that holds them has no warning band
answers:
  - "has the CLAIM-VERIFICATION-ADOPTION task evidence migration completed"
  - "where is the exact pre-migration claim-verification task source"
  - "what does it cost to register a partitioned task tree"
  - "how many surface records does one migrated task tree add"
  - "which task trees use the sharded route catalog"
  - "how do I partition an oversized task tree"
  - "how do I prove a task-evidence partition is lossless without trusting the writer"
  - "why is a task-evidence part allowed to own several non-adjacent source regions"
  - "what binds task_evidence bytes_each now"
date: 2026-09-15
status: current
tags: [documentation, task-tree, containment, migration, live-document-size, claim-verification]
evidence: docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md (.21, .22); doctrine/live_document_size/claim_verification_task_evidence.json; doctrine/live_document_size/surfaces.jsonl; scripts/check_active_task_evidence.pl; scripts/check_live_document_size.pl
reverify: perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/claim_verification_task_evidence.json --report
---

`LIVE-DOCUMENT-PRESSURE-HEADROOM.21` partitioned `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md` on
`2026-09-15`, when it stood at 278,514 of the 278,528-byte `task_evidence.bytes_each` ceiling — fourteen
bytes, on an axis whose health target and enforcement ceiling are the same value, so there was no warning
band left to spend either. The remedy was the accepted active-task-evidence contract (ADR 0039 topology,
ADR 0046 sharded route catalog) rather than a new containment shape, because that contract is the only
candidate that ships a writer: an atomic root-last materializer with rollback, derive-and-diff generation
of the index, route catalog and manifest, and a gate that refuses any byte the contract does not derive.

The migrated result is a 140-line / 8,157-byte bounded root, a 40-line index, a single 56-line route
catalog part, and eleven semantic parts totalling 2,896 lines / 282,162 bytes whose largest member is 384
lines / 58,277 bytes. Forty-six leaf routes carry a lifecycle each and **zero** are uncorroborated: every
one is cross-checked against its own node's `Status:` line in its primary part. The exact source capsule
is `docs/archive/tasks/claim-verification-adoption/source-through-2026-09-15.md`, 2,801 lines / 278,514
bytes at SHA-256 `938f909f88e71f3b335549232be4c5388a9d91a41846de5d1d15553b76c7674b`.

**Prove losslessness without trusting the writer that produced it.** Re-harvest only the
marker-delimited payloads out of the part *files*, ignoring the contract's own line spans, concatenate
them in declared source order, and compare with `git show HEAD:<path>`. That reproduces the committed
pre-migration blob byte-for-byte, and it is a different question from the checker's, which compares each
part against spans it also authored. Compare the `- ID:` node sets across source, parts, and the new
root's owner registry too; the contract requires the root registry and the route set to be equal, so a
lost node fails in two places.

**A part may own several non-adjacent regions, and that is what makes a concern-based cut possible.**
Regions must be contiguous and cover the whole source, but the region-to-part mapping is free. An
accreted task file interleaves leaves with the checklists of earlier leaves, so the one open leaf, the
frontier table, and the open questions sat in three separate places; all three route to the single
`active` part, and no byte had to move to make the cut land.

**The cost, which is the part a diff does not show.** One partitioned tree adds exactly four records to
`doctrine/live_document_size/surfaces.jsonl` — index, semantic parts, route catalog, archived capsule —
plus one line in `scripts/check_task_evidence_contracts.sh`, three current-claim-census dispositions and
a bumped `expected_current_surfaces`, and a `surface_disposition` in `published_assertions.jsonl` for
every new surface that owns a claim-annotated file. After `.21` the registry is 62 of its declared
`max_records: 64`, against a portable hard cap of 128. That bound is the one control in the registry with
**no milestone block**: `check_live_document_size.pl` raises a single unconditional error when it is
exceeded and warns at nothing below it, so the next tree needing containment would fail the build with no
prior notice. `LIVE-DOCUMENT-PRESSURE-HEADROOM.22` owns that.

**The stop relocated; it was not removed.** `task_evidence.bytes_each` now reports
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` at 258,302 of 278,528 as the surface maximum — 92.7%, an active
tree with open leaves. Partitioning bought room on one file; the axis still binds.

Links: [[alignment-task-evidence-migrated]], [[active-task-migration-transaction]],
[[active-task-legacy-route-aliases]], [[live-surface-edit-bookkeeping-chain]].
