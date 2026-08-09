---
id: corpus-refresh-completion-vs-normalized-retention
title: Corpus current-binary refresh completion is independent of normalized-cache retention
answers:
  - "does cleaning normalized bundles undo a completed corpus re-ingest"
  - "why does CORPUS-COVERAGE still say 32 refreshes when only one normalized bundle exists"
  - "what is the difference between a refreshed EvidenceIR and a retained normalized bundle"
  - "how many CORPUS-COVERAGE re-ingests remain after normalized cleanup"
  - "should corpus refresh progress be counted from normalized directories"
date: 2026-08-09
status: current
tags: [corpus-coverage, re-ingest, normalized-bundle, cleanup, lifecycle, currentness]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2-.3); git commit 70534fe0; generated/{source_ir,evidence_ir,semantic_ir,intent_ir,adapters/isf}
reverify: "git show -s --format=%B 70534fe0; find generated/source_ir -type d -name normalized | wc -l; find generated/{source_ir,evidence_ir,semantic_ir,intent_ir,adapters/isf} -type f | wc -l"
---

`CORPUS-COVERAGE.2` completion records a **current-binary stage refresh**, not permanent retention of a
Docling cache. Each successful slice re-ingested its PDF, rebuilt EvidenceIR through adapter output, verified the
result, and recorded the measured delta in the task table. `specforge clean --scope source-normalized --execute`
may later reclaim the heavyweight normalized Markdown/assets without deleting the retained stage artifacts.

That lifecycle happened after refresh #32: the documented `2026-07-05` cleanup reclaimed every normalized
bundle. A `2026-08-09` census found 80 SourceIR documents, one normalized bundle (the newly promoted SWD
canonical), 79 EvidenceIR, and 78 SemanticIR/IntentIR/adapter artifacts. The 32 exact `.2` document keys still
have all five expected stage files: **160/160 present**. Cleanup therefore did not undo those completed refreshes.

Progress must come from the durable per-document refresh log, not a live `normalized/` directory count. The
correct frontier remains **32 completed refreshes and 24 real chip-spec documents not yet refreshed by `.2`**.
Normalized retention is an ephemeral rebuildability/cache census and must be reported separately whenever it is
operationally relevant. Historical task rows saying `RESTORED` describe their slice-completion postcondition;
they do not promise permanent cache retention.
