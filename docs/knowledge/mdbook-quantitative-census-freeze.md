---
id: mdbook-quantitative-census-freeze
title: The mdBook quantitative census verifies the mapping, not the totals, which the manual itself moves
answers:
  - "what did the mdBook quantitative claim census find"
  - "how many quantitative prose candidates are in the SpecForge book"
  - "which mdBook quantitative census totals may be published and which must be read from the report"
  - "which mdBook quantitative assertions still lack claim authority"
  - "how are authored examples identity literals and dated book measurements excluded"
  - "why did the book quantitative checker separate record and array bounds"
  - "how do I reverify the frozen mdBook quantitative census"
date: 2026-08-30
status: current
tags: [claim-verification, mdbook, quantitative, census, authority, currentness]
evidence: doctrine/claim_verification/book_quantitative_claims.jsonl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.3b.3.3); doctrine/claim_verification/claims.jsonl (mdbook-quantitative-census-frozen)
reverify: perl scripts/check_book_quantitative_claims.pl --self-test && perl scripts/check_book_quantitative_claims.pl --check && perl scripts/check_book_quantitative_claims.pl --report
---

`CLAIM-VERIFICATION-ADOPTION.3b.3.3` freezes the manual-wide quantitative review denominator and its exact
semantic outcomes. The durable property is the **mapping**: every prose candidate line in the governed book has
exactly one non-overlapping exact line/SHA region, each region carries one closed outcome, and `--check` fails
if any candidate is unmapped or any region's source bytes move. `registered` regions all join
`workflow-standard-capacity-profile`; every `incomplete` region explicitly names re-derivation, independent
falsification, and durability as missing, and the freeze verifies that the uncertainty is represented, not that
the underlying assertions are true. Exclusions stay separated by exact scope reason — authored threshold or
choice, example or command literal, schema/version/date/path/digest identity, and dated boundary observation.

**None of the totals is a constant, and this card publishes none of them.** `regions`, `registered`,
`incomplete`, `excluded`, and the exclusion split all move whenever the manual changes — including a commit
that only edits a chapter. Read them from `perl scripts/check_book_quantitative_claims.pl --report`, and read
the frozen expectations from `expected_book_files` / `expected_candidate_files` / `expected_candidate_lines` in
`doctrine/claim_verification/book_quantitative_claims.jsonl`. The dated `.3b.3.3` boundary vector was
307 regions = 8 registered + 78 incomplete + 221 excluded, and it was correct only until `4dac5642`
(`2026-08-27`). Carrying it here past that point is what `CLAIM-VERIFICATION-ADOPTION.11` found and withdrew:
`.6b` repaired the identical vector on `docs/book/src/reference/doctrine-enforcement.md` while quoting these
numbers off it, and never looked for the second copy, because nothing enumerated the surfaces that publish
these producers' fields. See [[current-claim-census-freeze]] for the same rule on the sibling census.

The first full-region replay exposed a dimensional checker defect: the JSONL record sequence had been subjected
to the nested-array bound. Record count now belongs only to `max_records`, while `max_array_items` applies inside
each record. The controlled suite includes the positive cross-dimension case plus missing, overlap, stale,
untracked, authority, source, schema, and portable-bound failures.
