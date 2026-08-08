---
id: corpus-kb-managed-currentness
title: Tracked fixtures and the reviewed snapshot define corpus-KB managed currentness
answers:
  - "what inputs define corpus KB managed block currentness"
  - "how is corpus_kb currentness checked"
  - "can ambient validation reports define the tracked corpus KB validation page"
  - "does corpus KB refresh mutate canonical IR or CorpusMemory"
  - "how are human corpus KB notes preserved during refresh"
date: 2026-08-08
status: current
tags: [corpus-kb, currentness, validation, kg-bench, managed-blocks]
evidence: doctrine/live_document_size/corpus_kb.json; docs/decisions/0014-corpus-kb-managed-currentness-authority.md; corpus_kb/failures/validation-findings.md; corpus_kb/benchmarks/kg-fixtures.md
reverify: perl scripts/check_corpus_kb_currentness.pl --report
---

The tracked validation projection below `corpus_kb/` is derived from the last-reviewed
`VALIDATION_SNAPSHOT.md`, not from ambient ignored validation reports. The explicit
`corpus-kb --validation-snapshot VALIDATION_SNAPSHOT.md` mode parses that tracked projection and is
mutually exclusive with positional report inputs.

The KG denominator is every Git-indexed file below `crates/specforge/test_data/kg_quality/`: currently
312 files forming 156 fixtures. A complete refresh projects 156/156 passing results into one aggregate,
eight family pages, a review-only prior-candidate page, and its paired JSON manifest. The separate
`kg-bench` command remains the executable truthfulness gate.

`scripts/check_corpus_kb_currentness.pl` binds those inputs, the eleven Markdown managed regions, the
JSON output, and seven Rust producer regions. It separately hashes the human prefix and suffix around
every managed block, so currentness cannot be repaired by overwriting human synthesis. Neither the
refresh nor the checker may mutate canonical IR or typed prior memory.
