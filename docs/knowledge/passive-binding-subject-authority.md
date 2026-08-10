---
id: passive-binding-subject-authority
title: A passive obligation's constraint subject must be named before its must/shall be|remain lead
answers:
  - "why did OpenCAPI produce constraints on CAPI OCDE and DLX that the document never constrains"
  - "why can an uppercase token inside a longer word like OpenCAPI become a signal constraint subject"
  - "what stops a later sentence or trailing agent phrase from supplying a passive constraint subject"
  - "why is must have its WSTRB input tied HIGH still extracted after the pre-bind subject repair"
  - "why are table-row sources exempt from the pre-bind constraint subject rule"
  - "what is the corpus pre-bind subject measurement (26 false records across nine documents)"
date: 2026-08-10
tags: [evidence-ir, constraints, subject-authority, grammar, corpus-coverage, adr-0006]
evidence: docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.50a); crates/specforge/src/ir/evidence.rs
reverify: "cargo test -p specforge corpus_coverage_2_50a --lib && target/debug/specforge kg-bench"
---

English binds a passive obligation — `must`/`shall`, optionally negated, plus the copula `be`/`remain` — to a
subject that **precedes** the modal. Both deterministic constraint extractors used to ignore that ordering. The
pattern path falls back to scanning the whole statement when its bounded subject clause yields no candidate, and
the dynamic value path scans the whole statement by construction, so any uppercase token anywhere in the sentence
could become the subject. Because candidate tokenization splits on case, a token need not even be a word: `CAPI`
is only the uppercase tail of `OpenCAPI`.

The OpenCAPI Data Link Layer refresh made this concrete. *"The endpoint shall be held in reset by an out-of-band
OpenCAPI Device Enable (OCDE) signal"* constrains the endpoint, yet produced `CAPI` and `OCDE` bound to `RESET`;
*"Lane reversal … shall be compatible with all supported lane widths"* produced `CAPI` and `DLX` bound to
`COMPATIBLE` from a sentence two positions later. Those four records created four ungrounded temporal rules and a
false `RESET` versus `COMPATIBLE` temporal conflict. The existing dotted-reference, descriptive-field-cell,
value-position, and condition-subject gates all pass such a token, because none of them expresses word order.

The repair is one shared predicate over the constraint-bearing sentence: keep a candidate only when it occurs at
identifier boundaries before that sentence's first passive binding lead. Three exemptions are deliberate and each
is measured, not assumed:

1. **Active obligations are untouched.** `must drive PSTRB LOW` and `must have its WSTRB input tied HIGH` state
   their object after the verb, so they carry no `be`/`remain` lead. Admitting `have` as a lead would have deleted
   real AXI `WSTRB` and Arm low-power `PREQ` constraints.
2. **Table rows are out of scope.** A row's other cells legitimately name the subject that its obligation cell
   then constrains, so a candidate absent from the obligation's own sentence can still be real inside a row.
3. **Ordinary passive constraints survive**, single- or multi-signal, because every one of their subjects precedes
   the lead.

Measured over the 80-document persisted corpus: 26 of 256 deterministic records are false pre-bind subjects — 17
`sigcon_*` across eight documents and nine `dyn_sigcon_*` across two — covering later conditions and scopes, a
protocol or device name, the modal word `MUST` itself, and non-subject fields. An isolated old-versus-new replay
over the 21 rebuildable documents removes exactly eight records, adds none, and leaves the other 18 documents
byte-identical; one emitted target improves, the I2C `wiring_patterns.isf` losing a false `SCL = 1` rule. The sibling
value-position defect — a hex literal in `shall have … set to FFFFh` reaching the subject slot — is a different
error class and was closed separately by `EXTRACTION-QUALITY-GAUGE.3h`. See also
[[timing-table-structural-authority]].
