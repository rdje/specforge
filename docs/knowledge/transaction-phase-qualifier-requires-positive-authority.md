---
id: transaction-phase-qualifier-requires-positive-authority
title: Typed transaction-phase names require positive phrase authority
answers:
  - "why did transaction_phases contain called edge or positive"
  - "how does build_transaction_phases distinguish a named phase from phase error or phase tolerance"
  - "what is the transaction phase qualifier authority rule"
  - "how many retained typed transaction phases were false positives"
  - "why are there 82 retained phase records but 101 on a current rule replay"
  - "does transaction phase precision change phase membership or ISF output"
  - "are transaction phase names allowlisted by protocol"
date: 2026-08-10
status: current
tags: [semantic-ir, transaction-phases, phrase-authority, corpus-coverage, adr-0006]
evidence: docs/research/transaction-phase-qualifier-precision-measurement.md; docs/tasks/CORPUS-COVERAGE.md (.2.43a.ii); crates/specforge/src/ir/semantic.rs; docs/book/src/pipeline/semanticir.md
reverify: "cargo test -p specforge phase -- --nocapture; rebuild the 79 retained EvidenceIR inputs with target/release/specforge semantic --dry-run and expect 70 transaction_phases records"
---

`build_transaction_phases` no longer treats every surviving token immediately before `phase` / `phases` as a
name. A candidate must have at least one positive, local phrase-authority cue: phase heading/termination,
numbering, predicate, temporal use, naming/copular/possessive construction, or bounded busy-state reporting.
Comma adjacency, unmatched parenthetical predicates, interrogative/quantity grammar, and bare compounds such as
`phase error`, `phase tolerance`, or `phase tracking device` do not authorize a name.

The exact retained census is 82 records: 59 valid and 23 false. Because retained artifacts are mixed-vintage, a
current-rule replay over the same 79 EvidenceIR inputs has 101 candidates: eleven additional valid phases and
eight additional false candidates. The repaired producer emits 70 valid records across 27 documents, removing
all 31 false candidates. Every removed candidate has an empty declared-signal intersection, every retained
name's provenance is unchanged, and old-rule-populated versus repaired IntentIR variants are byte-identical.

The producer is two-pass: one authoritative occurrence admits a document-local name, then all grammatical
occurrences of that name remain provenance and signal-membership evidence. The rule contains no vendor,
protocol, document, signal, or exact phase-name allowlist/denylist.
