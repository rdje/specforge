---
id: mdbook-quantitative-census-freeze
title: The mdBook quantitative census freezes 307 exact regions and 78 honest incomplete assertions
answers:
  - "what did the mdBook quantitative claim census find"
  - "how many quantitative prose candidates are in the SpecForge book"
  - "which mdBook quantitative assertions still lack claim authority"
  - "how are authored examples identity literals and dated book measurements excluded"
  - "why did the book quantitative checker separate record and array bounds"
  - "how do I reverify the frozen mdBook quantitative census"
date: 2026-08-15
status: current
tags: [claim-verification, mdbook, quantitative, census, authority, currentness]
evidence: doctrine/claim_verification/book_quantitative_claims.jsonl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.3b.3.3); doctrine/claim_verification/claims.jsonl (mdbook-quantitative-census-frozen)
reverify: perl scripts/check_book_quantitative_claims.pl --self-test && perl scripts/check_book_quantitative_claims.pl --check && perl scripts/check_book_quantitative_claims.pl --report
---

`CLAIM-VERIFICATION-ADOPTION.3b.3.3` freezes the manual-wide quantitative review denominator and its exact
semantic outcomes. The governed book has 39 Markdown files; 307 prose candidate lines occur across 21 files,
and each has one non-overlapping exact line/SHA region.

The frozen vector is eight `registered`, 78 `incomplete`, and 221 `excluded`. The registered regions all join
`workflow-standard-capacity-profile`. Every incomplete region explicitly names re-derivation, independent
falsification, and durability as missing; the freeze verifies that uncertainty is represented, not the truth of
those 78 underlying assertions. Exclusions remain exact: 26 authored thresholds or choices, eight examples, one
schema/version/date/path/digest identity, and 186 dated boundary observations.

The first full-region replay exposed a dimensional checker defect: the JSONL record sequence had been subjected
to the nested-array bound. Record count now belongs only to `max_records`, while `max_array_items` applies inside
each record. The controlled suite includes the positive cross-dimension case plus missing, overlap, stale,
untracked, authority, source, schema, and portable-bound failures.
