---
id: bounded-ingest-resource-risk-below-page-threshold
title: Bounded-ingest activation is sized from fixed host capacity and capped below the reproduced 400-page risk
answers:
  - "why was the Arm Debug replay killed at 400 pages"
  - "is a greater than 500 page threshold sufficient for bounded PDF ingestion"
  - "what does SPEC-TO-INTENT-ALIGNMENT.6b.iii repair"
  - "how was the 400 page Docling SIGKILL reproduced"
  - "did bounded ingestion change the Arm Debug SourceIR"
  - "how is the default SPECFORGE_INGEST_BATCH_THRESHOLD selected"
  - "why is the default ingest threshold 131 pages on a 24 GiB host"
  - "what happens when ingest cannot count PDF pages"
  - "what is AppError::IngestTerminatedBySignal"
date: 2026-08-12
status: current
tags: [ingestion, docling, memory, batching, resource-risk, replay]
evidence: crates/specforge/src/ir/source/docling_backend.rs (resource_sized_batch_threshold / resolve_batch_threshold / backend_exit_error / embedded helper); crates/specforge/src/error.rs (IngestTerminatedBySignal); docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6b.ii.b, .6b.iii)
reverify: "cargo test -p specforge --lib ir::source::docling_backend::tests && cargo test -p specforge --lib pdf_source_ir_materialization && bash scripts/check_project_data_locality.sh"
---

The `.6b.ii.b` population replay reproduced the same operational failure twice: current default Docling
single-pass ingestion was terminated by signal after loading model weights for the 400-page Arm Debug source.
The host retained about 85% free memory immediately afterward and no child process survived, so the failure was
not a persistent host-exhaustion condition or leaked worker. The previous policy assumption that only documents
above 500 pages require batching is therefore disproved.

The root cause crossed two seams. Rust adapted the size of an active batch from total physical RAM, but embedded
Python separately defaulted activation to 512 pages and treated an unreadable page count as permission for the
same single-pass path. A signal-killed child then surfaced as a generic command failure with no preserved process
status meaning.

`SPEC-TO-INTENT-ALIGNMENT.6b.iii` closes those seams generically. Rust reads total RAM once, budgets 40% against
the measured 64-page / roughly 4.8-GiB working set (a conservative 75 MB/page), clamps the default threshold to
1..399, and passes it with the existing adaptive batch size to Python. The 24-GiB host resolves threshold 131 and
batch size 64. Explicit nonnegative threshold overrides remain exact, including zero to force batching; absent,
empty, negative, or malformed values return to policy. The helper refuses conversion when it cannot count pages
and records threshold, batch size, and batched state in backend metadata.

Unix signal termination now returns `AppError::IngestTerminatedBySignal`. This reports a possible external
resource-enforcement event without falsely claiming OOM; the existing RAM guard alone returns the causally
specific `IngestAbortedForMemory`. Both discard staging and preserve the last good normalized bundle.

An override-free live Arm Debug replay selected 131/64 and completed all four stages. Its profile, tables,
elements, and sections are byte-identical to retained SourceIR; page and visual surfaces match after path
normalization. EvidenceIR, SemanticIR, and IntentIR also match after removing validation backannotations. The
exact 795-file / 184,164-KiB root was removed and is absent, as are all earlier failed/qualification roots and the
runtime source map. No document, vendor, protocol, filename, or reviewed-fixture branch exists in production.
