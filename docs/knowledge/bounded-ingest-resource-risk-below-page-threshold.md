---
id: bounded-ingest-resource-risk-below-page-threshold
title: A 400-page PDF can require bounded ingestion below the old page threshold
answers:
  - "why was the Arm Debug replay killed at 400 pages"
  - "is a greater than 500 page threshold sufficient for bounded PDF ingestion"
  - "what does SPEC-TO-INTENT-ALIGNMENT.6b.iii repair"
  - "how was the 400 page Docling SIGKILL reproduced"
  - "did bounded ingestion change the Arm Debug SourceIR"
date: 2026-08-12
status: current
tags: [ingestion, docling, memory, batching, resource-risk, replay]
evidence: scripts/replay_source_to_intent_population.py; crates/specforge/src/commands/ingest.rs; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6b.ii.b, .6b.iii)
reverify: "cargo test -p specforge --lib commands::ingest && bash scripts/check_project_data_locality.sh"
---

The `.6b.ii.b` population replay reproduced the same operational failure twice: current default Docling
single-pass ingestion was terminated by signal after loading model weights for the 400-page Arm Debug source.
The host retained about 85% free memory immediately afterward and no child process survived, so the failure was
not a persistent host-exhaustion condition or leaked worker. The previous policy assumption that only documents
above 500 pages require batching is therefore disproved.

Forcing `SPECFORGE_INGEST_BATCH_THRESHOLD=256` completed all four stages. Its SourceIR matched the persisted
single-pass authority in document profile, tables, elements, sections, and path-normalized visual content. The
authoritative whole-population replay used a command-recorded threshold of 399, so only the 400-page member
entered the bounded path; the other eleven reviewed documents, all at or below 310 pages, retained default
behavior.

This evidence qualifies the semantic repair but does not close the operational defect. The failed 2,567-file /
765,728-KiB root, empty retry root, bounded diagnostic root, successful 3,913-file / 1,090,884-KiB population
root, and runtime source map were each removed exactly and are absent. `SPEC-TO-INTENT-ALIGNMENT.6b.iii` owns a
generic resource/shape policy, typed memory-abort reporting, and single-pass/bounded fidelity checks; it must not
special-case this document or silently hard-code the qualification override.
