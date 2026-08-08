---
id: corpus-kb-bounded-projection-shape
title: Corpus-KB fixture projections grow by bounded rows and evidence bullets
answers:
  - "why did the corpus KB size warning appear and how was it removed"
  - "how does the corpus KB aggregate fixture page stay bounded as fixtures grow"
  - "why are prior candidate fixture names not joined into one long Markdown line"
  - "what are the current corpus KB live-document size metrics"
date: 2026-08-08
status: current
tags: [corpus-kb, kg-bench, generated-output, live-document-size]
evidence: crates/specforge/src/commands/corpus_kb.rs; corpus_kb/benchmarks/kg-fixtures.md; corpus_kb/prior_candidates/kg-fixture-candidates.md; doctrine/live_document_size/corpus_kb.json
reverify: perl scripts/check_corpus_kb_currentness.pl --report
---

The former corpus-KB warnings were local producer-shape problems, not aggregate collection pressure.
The aggregate benchmark page emitted six presentation lines for every passing fixture, and the prior-
candidate table joined every positive and guard fixture name into two unbounded cells.

The benchmark producer now renders one table row per fixture and emits separate failure details only
for fixtures that fail. The prior-candidate table carries counts, while `Fixture Evidence` lists every
positive and guard fixture on its own bullet. The paired JSON manifest remains the complete machine-
readable candidate evidence plane. Human synthesis, fixture outcomes, canonical IR, and typed prior
memory are unchanged.

With 156/156 fixtures passing, the 24 Markdown files now total 1,172 lines / 124,679 bytes. The largest
file is 201 lines / 25,470 bytes and the longest content line is 305 bytes, all below the unchanged
1,200-line / 49,152-byte / 1,280-line-byte per-file ceilings and their 80% warning milestones.
