---
id: book-behaviour-currency-instrument
title: The book's stale behavioural claims are found by the present-tense assertion population, and a symbol scanner finds none of them
answers:
  - "how do I check the mdBook for stale behavioural claims"
  - "does any gate catch a book paragraph describing deleted behaviour"
  - "why not scan the book for deleted Rust symbols"
  - "what does check_book_current_truth.sh actually cover"
  - "how many book claims describe behaviour the code no longer has"
  - "why is the book-behaviour obligation not hard-gated"
  - "what must a slice that deletes a production rule say about the book"
  - "why was the task-tree count in live-docs.md wrong"
date: 2026-09-11
status: current
tags: [mdbook, doctrine-enforcement, live-documents, currency, derived-state]
evidence: docs/tasks/BOOK-BEHAVIOUR-CURRENCY.md (.0 census); TOOLBOX.md (the LOCKSTEP sub-clause); scripts/check_book_current_truth.sh; scripts/check_book_quantitative_claims.pl; docs/book/src/domain/temporal-semantics.md; docs/book/src/reference/live-docs.md
reverify: "grep -rnE 'SpecForge (now|currently) [a-z]+|(now|currently) (resolves|infers|uses|treats|derives|applies|promotes|accepts|recognizes|recognises|canonicalizes|canonicalises|expands|merges)' docs/book/src --include='*.md' | wc -l   # expect ~18; adjudicate each against a named seam"
---

The mdBook is the owner's only window into SpecForge, and the contract is that it reflects what the code
actually does. `WIRE-BASED-100.4a` found it did not: the temporal chapter said SpecForge *"now resolves an
un-indexed prose reference … using the universal `x`/digit index convention"* for about a month **after**
`f88d463d` deleted that resolver and pinned the opposite as a unit control. Every gate was green throughout.

## Why the gates did not see it

`check_book_current_truth.sh` is a **hand-curated list of about twenty literal pairings** — a code literal
must exist *and* the book must carry the matching sentence, plus `reject_literal` entries that fail if a
known stale phrasing returns. It works; it is simply **opt-in per claim**, so its reach is whatever past
authors thought to enrol. `check_book_quantitative_claims.pl` polices numbers, but only where the line
matches its closed unit vocabulary — `trees` is not in it, which is how `live-docs.md` kept claiming
`docs/TASK_TREE.md` derived a row for **121** real task trees when the authority printed **152**.

## Two instruments, and only one of them earns its keep

**By symbol — measured worthless on this corpus.** `git log --since=2026-06-22 -p` over
`crates/specforge/src` removes 193 distinct `fn` names, **167 genuinely gone**. Grepping all 167 against
`docs/book/src` returns **two hits and both are false positives**: `infer` and `overlaps` match as ordinary
English words. No deleted production function is named by symbol anywhere in the book, so a symbol scanner
would have **no true positives at all** — it is not merely blind to the `.4a` case.

**By assertion — the one that works.** The stale paragraph's signature was `SpecForge **now** …`. Grepping
present-tense behavioural assertions yields **about 18 claims across 10 chapters**: small enough to
adjudicate one by one against a named seam, and that is what found both the `.4a` paragraph and the 121/152
count. The `reverify` command above is that grep.

## Why the obligation is guidance, not a gate

`TOOLBOX.md`'s LOCKSTEP item now carries a sub-clause: a slice that **deletes or replaces a production
rule** says which book text described the old behaviour and what happened to it, or that none did. It is
deliberately **not** hard-gated. The measured exposure is one stale claim per quarter; the hard-gated boxes
(ROOT CAUSE, ADDRESSED, NO REGRESSION) are reserved for what a green build must never hide, and a required
tick on a rare judgement-bound check buys false positives rather than currency.

## The residual this leaves

One claim in the population is **not re-derived**: `pipeline/isf-adapter.md` states the pinned FSMGen now
accepts nested `(eventually s (within N))` and `(stage p (ready r)(valid v))`. That needs an
`fsmgen --strict --check` probe against `subs/fsmgen`, which the census did not run. It is named rather
than assumed.

Links: [[live-surface-edit-bookkeeping-chain]], [[document-stated-identifier-coreference]],
[[indexed-signal-family-canonicalization]].
