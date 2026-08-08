---
id: live-document-containment-fixture-gate
title: Live-document lifecycle and control-plane proofs run on the repository volume
answers:
  - "how is the live-document containment checker tested"
  - "where do live-document checker test fixtures create temporary files"
  - "does the live-document registry reject unknown fields oversized arrays or oversized scalars"
  - "how are live-document ceiling increases and immutable debt baselines tested"
date: 2026-08-08
status: current
tags: [documentation, containment, doctrine, testing, locality]
evidence: scripts/test_live_document_size.pl
reverify: perl scripts/test_live_document_size.pl --quiet
---

`scripts/test_live_document_size.pl` is the 48-case executable proof suite for the local containment
contract. It covers all seven governed Markdown lifecycle classes, membership and query indexes,
executed freshness and currency controls, routes, frozen and archive controls, maintained-reference
authority, transition debt, exact coverage, path locality, all six independent size dimensions, and
Git-history ceiling/baseline rules. The `LIVE-DOC-SIZE` adapter runs the suite on every doctrine gate.

Every fixture workspace is created below repository-local `generated/` and removed automatically.
The JSONL registries declare and enforce schema version, record count, file bytes, raw-record bytes,
array cardinality, and scalar byte limits; unknown fields and identifiers outside their closed domain
fail closed.
