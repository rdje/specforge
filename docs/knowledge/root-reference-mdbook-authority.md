---
id: root-reference-mdbook-authority
title: Root user and architecture documents are bounded pointers to mdBook product contracts
answers:
  - "where is the canonical SpecForge user guide"
  - "where is the extraction architecture contract"
  - "where is the normative IntentIR product contract"
  - "why are USER_GUIDE.md and the root architecture documents only pointers"
  - "what prevents root documentation from regrowing into a second manual"
  - "where did the root architecture catalog go"
date: 2026-08-08
status: current
tags: [documentation, mdbook, architecture, pointer, current-truth]
evidence: docs/decisions/0015-mdbook-owns-public-product-contracts.md
reverify: bash scripts/check_book_current_truth.sh
---

The mdBook is SpecForge's one maintained public product-documentation plane. Unique durable
extraction requirements live in `docs/book/src/reference/extraction-architecture.md`; the normative
canonical-output boundary lives in `docs/book/src/reference/intentir-contract.md`; current behavior
stays in the existing topical chapters.

The stable root paths `USER_GUIDE.md`, `EXTRACTION_ARCHITECTURE.md`,
`KNOWLEDGE_GRAPH_ARCHITECTURE.md`, and `INTENTIR_SPEC.md` are compatibility pointers only. Each is a
separately bounded snapshot, and `scripts/check_book_current_truth.sh` requires its direct routes,
binds the product claims to live Rust seams, and rejects the stale implementation/status headings
that previously caused drift. Because those files no longer contain partitioned canonical prose,
their generated root-architecture collection catalog was retired.
