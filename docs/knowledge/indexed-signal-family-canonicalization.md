---
id: indexed-signal-family-canonicalization
title: Indexed-family spelling canonicalization was historical and is no longer production authority
answers:
  - "why is the PSEL antecedent dropped in a temporal rule"
  - "how does specforge handle PSEL vs PSELx (or HSEL vs HSELx)"
  - "what is index-family signal canonicalization"
  - "what does resolve_indexed_signal_family do"
  - "why does a temporal antecedent use PSELX not PSEL"
  - "how are per-instance indexed signals (PSELx HSELx) referenced in prose handled"
  - "how did APB temporal reach 100% (WIRE-BASED-100.4)"
date: 2026-06-06
status: superseded
tags: [temporal, semantic, signal-identity, wire-based-100, adr-0006]
evidence: commit 1c28516b; commit f88d463d; docs/decisions/0037-identifiers-are-opaque-and-one-way-grounded.md; crates/specforge/src/ir/semantic.rs (find_known_signal_name and temporal_condition_does_not_infer_numeric_or_x_index_aliases)
reverify: "cargo test --offline -p specforge-core --lib temporal_condition_does_not_infer_numeric_or_x_index_aliases"
---

This card records a historical rule, not current production behavior. Commit `1c28516b` introduced
`resolve_indexed_signal_family`: a bare prose token such as `PSEL` or `HSEL` could resolve to a declaration named
by that token plus `X` or digits. It recovered temporal antecedents that otherwise disappeared.

Commit `f88d463d` deliberately removed that rule while implementing ADR 0037. Current-document identifiers are
opaque: case, prefix, suffix, and numeric shape carry no semantic or alias authority. Exact identity wins;
unique case-fold recovery is allowed; every spelling-family guess fails closed. The live unit control
`temporal_condition_does_not_infer_numeric_or_x_index_aliases` pins this behavior, and the historical helper no
longer exists.

For the current reviewed APB recovery, `PSEL` is not obtained from declared `PSELX`. The exact source span says
`The select signal, PSEL, is asserted`; `SPEC-TO-INTENT-ALIGNMENT.7a` freezes that bounded same-clause
appositive as local declaration authority and separately proves that a bare `PSEL` cannot alias `PSELX` by
shape. Follow [[inference-antecedent-state-loss]] for current truth. The June temporal score and PSELX gold
correction remain accurate history at their own revision, not authority for present extraction.

`WIRE-BASED-100.4a` later restored the APB antecedent on a footing that is *not* this rule: the document
writes the same appositive role phrase before both identifiers (`Select signal, PSELx` and
`Select signal, PSEL`), so the identity is read from what the document states rather than from the `x`.
A single appositive still links nothing. See [[document-stated-identifier-coreference]].
