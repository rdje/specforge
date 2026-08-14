---
id: alignment-task-evidence-migrated
title: Alignment task evidence is a bounded active root over exact semantic and provenance authorities
answers:
  - "has the SPEC-TO-INTENT-ALIGNMENT task evidence migration completed"
  - "what are the exact migrated alignment task evidence metrics"
  - "where is the exact pre-migration alignment task source"
  - "how many alignment task owners and routes survive migration"
  - "why does aggregate_composition support different health and ceiling counts"
  - "how are different health and ceiling file counts declared"
date: 2026-08-14
status: current
tags: [documentation, task-tree, containment, migration, live-document-size]
evidence: docs/tasks/SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.md; docs/decisions/0039-bounded-spec-to-intent-task-evidence.md; doctrine/live_document_size/spec_to_intent_task_evidence.json; doctrine/live_document_size/surfaces.jsonl; scripts/check_active_task_evidence.pl; scripts/check_live_document_size.pl
reverify: perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/spec_to_intent_task_evidence.json --report
---

Containment `.3` migrated the clean `38b79395` boundary through the accepted same-volume root-last writer. The
stable alignment path is a 127-line / 5,725-byte active root containing normalized current state, no eligible
product frontier, one detail link, and all 58 exact executable owner declarations. Its 86-line / 5,925-byte
index routes 58 primary owners over seven immutable legacy parts and one reserved active behavioral part.

The eight parts contain 2,144 lines / 282,030 bytes; their largest member is 502 lines / 65,125 bytes with a
1,605-byte maximum line. All 21 marked regions reproduce the locked source byte-for-byte. The independent
capsule at `docs/archive/tasks/spec-to-intent-alignment/source-through-2026-08-14.md` is exactly 2,049 lines /
278,178 bytes at SHA-256 `e70892a5c6acbe794bea6a9dd4484a90481e31d0aba64d8b8e8dcaaeae90a26c`.

The accepted part profile has 16 health files but 24 ceiling files, with tighter heterogeneous aggregate totals.
The live-size registry therefore permits an `aggregate_composition` role count to be either one positive scalar
for equal-band cardinalities or a closed `{health, ceiling}` object. It sums the selected count independently for
each band and still requires exact file, total-line, total-byte, and largest-member equality. This represents the
accepted limits without widening them; missing or unknown count bands fail closed.
