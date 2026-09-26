---
id: missing-pieces-are-indexed-in-docs-gaps
title: Every missing piece from PDF to .isf is indexed in docs/gaps/ — SpecForge's items by document category and pipeline stage (specforge.md), FSMGen's incremental 28-rung capability ladder classified against its published book (fsmgen-ladder.md), and one index whose owner column is an enforced ownership region (INDEX.md); since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.36)
answers:
  - "where is the list of everything SpecForge still cannot do from PDF to ISF"
  - "what does SpecForge need FSMGen to support"
  - "which FSMGen capabilities are supported, requested, anticipated or fixed upstream"
  - "which leaf owns a missing piece"
  - "how is the gap catalog kept current and owned"
  - "what is the gap_catalog surface / gap_catalog_owners region"
date: 2026-09-26
tags: [gaps, fsmgen, catalog, ownership, extraction-representation-gaps, routing]
evidence: docs/gaps/INDEX.md; docs/gaps/specforge.md; docs/gaps/fsmgen-ladder.md; doctrine/ownership_citations/citations.jsonl (gap_catalog_owners); doctrine/live_document_size/surfaces.jsonl (gap_catalog); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.36, .36.a, .37)
reverify: "bash scripts/check_doctrines.sh --only OWNERSHIP-CITATIONS -> PASS (every owner in docs/gaps/INDEX.md is classified and open). grep -c '^| S' docs/gaps/specforge.md -> 25 and grep -c '^| F' docs/gaps/fsmgen-ladder.md -> 28 on 2026-09-26."
---

**Built `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.36`, at the owner's request).** Knowledge of what is missing was
spread across this tree's leaves, the book's ledger, `docs/FSMGEN_FEEDBACK.md`, 13 packets under
`docs/upstream/fsmgen/`, other trees' leaves, and the category contract's families with no IntentIR carrier.
`docs/gaps/` indexes it all without restating it:

- `specforge.md`: 25 items (S1–S25), each giving the road, the stage, the evidence and the owning leaf.
- `fsmgen-ladder.md`: 28 rungs (F1–F28), ordered along the PDF-to-`.isf` path. They run from actor and boundary
  through level obligations, time, transactions, storage and structures, to systems of actors and tooling. Each is
  classified from FSMGen's book at pin `54ac1c22` (§13k matrix and *Explicit Non-Claims*):
  - 15 supported;
  - 1 fixed upstream (reset polarity);
  - 9 requested, 3 of them acknowledged;
  - 1 question;
  - 2 anticipated: temporal properties beyond *eventually* (`.37`) and structured records (after
    `DOC-INTENT-TAXONOMY.4b`). F26's multi-bit clock crossings are anticipated too.
- `INDEX.md`: every open item, meaning the 25 plus the 14 rungs FSMGen does not yet provide. Its table sits inside
  the `gap_catalog_owners` markers, so `OWNERSHIP-CITATIONS` refuses a missing, unclassified or closed owner.

Rules: a new missing piece gets a leaf first, then a row. A rung becomes *requested* only through `PROCESS.md`'s
evidence bar; *anticipated* is planning, not a request (ADR 0033). The ladder is re-checked on every pin refresh.
`.36.a` triages the remaining 244 non-closed leaves in 64 trees into catalog items, process work, or closure.
`[[a-polarity-rule-reaches-its-stated-scope]]` · `[[an-uncited-obligation-is-a-candidate-miss]]`.
