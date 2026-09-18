---
id: book-quantitative-candidate-vocabulary
title: The book's quantitative-claim gate could not see the product's own output nouns — ten admitted on measured precision, six rejected, denominator 483 to 539
answers:
  - "which nouns does the book quantitative-claim candidate grammar recognise (the .9 set — files, lines, bytes, records, members, facts, questions, shards, cases, tests, checks, surfaces, claims, fields, families, documents, pages, fixtures, diagnostics, commands, doctrines, signals, registers, artifacts, rules, items, units, tables, cells — plus the .18 set: statements, behaviors, rows, constraints, actors, declarations, invariants, subjects, spans, leaves)"
  - "why did the book quantitative-claim census report full coverage while published counts went ungated (its unit vocabulary is a closed alternation assembled from nouns that had already produced a loud miss, and SpecForge's own output nouns had never produced one — a miss there is a silent zero rather than an error, so nothing looked)"
  - "what did admitting the product's output nouns to the candidate grammar cost (56 new candidate lines, 483 to 539, with candidate files unmoved at 26; individually statements +13, behaviors +12, rows +12, constraints +11, actors +5, declarations +5, invariants +5, subjects +2, spans +1, leaves +1, which sum to 67 because the book counts several on one line)"
  - "which nouns were rejected from the candidate unit vocabulary and why (edges 3 real of 8 — four misses are the temporal chapter's own grammar examples like within 2 clock edges; names 2 of 3; relations 1 of 3, both misses section titles; contracts 0 of 3; identifiers 0 of 1, quoted source text; sentences 1 of 1 but only by an incidental prefix of sentence-start phrases; assertions, obligations, columns, entries and nodes cost zero because every line carrying one already carries an admitted noun)"
  - "what is the admission bar for a new candidate unit noun (measured precision: a noun is admitted only when EVERY line it newly admits is a real published quantity, read by hand, with the marginal count taken over the already-admitted set rather than the old baseline because overlap flatters a noun)"
  - "how do I measure what a candidate-grammar change would cost (drive the checker's own discover_candidates over its own derive_book_members — the baseline must reproduce --check exactly, 483 candidates across 26 files with 0 errors — then diff the candidate list with the noun added alone; --produce cannot be used because validate_contract runs first and aborts on the changed denominator)"
  - "how are new book quantitative regions adjudicated (excluded with scope_reason dated_boundary_evidence when the sentence or its lead-in anchors the figure to a completed measurement or a superseded state — measured, found, removed, restored, the old IntentIR, a before-and-after; incomplete with all three legs otherwise, which is the conservative default because it declares the legs are missing rather than excusing the figure)"
date: 2026-09-18
status: current
tags: [claim-verification, book, gates, vocabulary, measurement, method]
evidence: docs/tasks/claim-verification-adoption/current-and-open-work.md (.18, .9); scripts/check_book_quantitative_claims.pl (is_candidate, the grammar control); doctrine/claim_verification/book_quantitative_claims.jsonl
reverify: "perl scripts/check_book_quantitative_claims.pl --check — expect 42 book files, 539 prose candidate lines across 26 files, 539 adjudicated regions; and --self-test 20/20"
---

A gate whose coverage depends on an author choosing a synonym it happens to know is not a gate. The book's
quantitative-claim census reported **full coverage** — 483 candidates, 483 adjudicated — while 40 lines
across 11 files published a count it could not see.

## Why the blind spot survived

`CLAIM-VERIFICATION-ADOPTION.9` built the unit vocabulary as a **closed** list, which is right, and
populated it from nouns that had already produced a **recorded miss**. That method cannot reach a noun
nobody has looked for, and the nouns nobody had looked for were SpecForge's own output vocabulary:
`statements`, `behaviors`, `rows`, `constraints`, `actors`, `declarations`, `invariants`, `subjects`.
A miss there is a **silent zero**, not an error, so it never announced itself.

It was found by an author who declined to use it: `EXTRACTION-QUALITY-GAUGE.3j.2.c.i` first wrote
*"109 spans"*, *"7 obligations"* and *"5 rows"* into the book, watched the census stay green at 476/476,
and rewrote the lines to nouns the detector holds rather than keeping the free pass.

## The admission bar, and the measurement that applies it

**A noun is admitted only when every line it newly admits is a real published quantity**, read by hand.
The measurement drives the checker's *own* `discover_candidates` over its *own* `derive_book_members`, so
the baseline reproduces `--check` exactly and each delta belongs to that grammar rather than to a scanner.
`--produce` cannot serve, because `validate_contract` runs first and aborts the moment the denominator
moves.

Marginal counts must be taken **over the already-admitted set**, not over the old baseline: overlap
flattered three nouns, and one — `leaves` — turned out to earn admission only after `subjects` had taken
its other line.

Admitted (10): `statements`, `behaviors`, `rows`, `constraints`, `actors`, `declarations`, `invariants`,
`subjects`, `spans`, `leaves`. Cost 56 lines, 483 → 539, files unmoved at 26.

Rejected, with the marginal count: `edges` 3/8 — four of the five misses are the temporal chapter's own
grammar examples (*"within 2 clock edges"*), which are language and not counts; `names` 2/3; `relations`
1/3, both misses section titles; `contracts` 0/3; `identifiers` 0/1, quoted source text. `sentences`
scores 1/1 but only through an incidental prefix of *"sentence-start phrases"*, so it has no demonstrated
population of its own. `assertions`, `obligations`, `columns`, `entries` and `nodes` cost **zero**.

The residual is **asserted**, not remembered: the grammar control's `must_not` half now carries one real
line of each rejected shape, so a later author who admits `edges` takes the control down.
