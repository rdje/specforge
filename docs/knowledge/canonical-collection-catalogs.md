---
id: canonical-collection-catalogs
title: Query-only canonical collections have complete bounded membership catalogs
answers:
  - "where is the complete index for SpecForge research workflow architecture corpus KB or KG fixtures"
  - "how can a collection use a membership index outside its own surface"
  - "which canonical Markdown collections still rely only on git query"
date: 2026-08-08
status: current
tags: [documentation, navigation, generated-index, doctrine]
evidence: docs/catalogs/INDEX.md; doctrine/live_document_size/canonical_catalogs.jsonl
reverify: perl scripts/check_canonical_collection_catalogs.pl --check
---

No `partitioned_canonical` Markdown collection relies only on `git:query`. Five generated member
catalogs under `docs/catalogs/` directly link 237 workflow, FSMGen-issue, research, corpus-KB, and
KG-fixture members; their bounded landing links all five indexes. The four former root-architecture
members are now independently bounded compatibility pointers into the mdBook, so their obsolete
collection catalog was retired. `docs/TASK_TREE.md` remains the exact external index for 122 task
files, and `knowledge-map/README.md` directly links all four files in its portable bundle.

The live-document doctrine's `external_membership` contract permits a collection index to live in a
separately classified bounded Markdown surface. It requires one safe repository-relative `.md` path
outside the member surface, requires that index to be classified elsewhere, and applies the same
direct-link completeness proof used by an internal membership index. This avoids duplicating a
catalog inside canonical inputs or forcing a large browse index under fixture-sized file limits.
