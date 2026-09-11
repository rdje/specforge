---
id: document-stated-identifier-coreference
title: A repeated appositive role phrase is the document stating two spellings are one wire, and it is the only alias ADR 0037 leaves open
answers:
  - "how can a prose signal spelling bind to a declared one without a suffix inference"
  - "why is the PSEL antecedent recovered again after ADR 0037 removed the index-family resolver"
  - "what is document_signal_coreferences"
  - "how does SpecForge resolve PSEL to PSELx in a temporal condition"
  - "what is an appositive role phrase co-reference"
  - "why does one appositive never establish a signal alias"
  - "why must a co-reference key be rejected as a clause value"
  - "how many identifier co-references exist corpus-wide"
  - "does the co-reference rule resurrect resolve_indexed_signal_family"
  - "should a co-referenced antecedent be marked alias_dependent"
date: 2026-09-11
status: current
tags: [semantic-ir, temporal, signal-identity, adr-0006, adr-0037, wire-based-100, apb]
evidence: crates/specforge/src/ir/semantic.rs (document_signal_coreferences; signal_role_appositives; find_known_signal_name; temporal_clause_value; a_repeated_appositive_role_phrase_co_references_two_identifiers; one_appositive_alone_never_co_references_by_spelling; a_co_referenced_alias_is_a_signal_name_never_a_value); docs/tasks/WIRE-BASED-100.md (.4a); docs/decisions/0037-identifiers-are-opaque-and-one-way-grounded.md
reverify: "cargo test --offline -p specforge-core --lib co_reference && cargo test --offline -p specforge-core --lib temporal_condition_does_not_infer_numeric_or_x_index_aliases && ./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip"
---

ADR 0037 made current-document identifiers opaque: case, prefix, suffix and numeric shape carry no alias
authority, and `resolve_indexed_signal_family` was deleted. That is right, and it cost something real. AMBA
APB declares `PSELx` and writes ordinary obligations about `PSEL` — *"PNSE must be valid when PSEL is
asserted"* — so those conditions resolved to nothing and the antecedent was dropped **silently**. Four APB
temporal rules lost a select antecedent this way, and only a gold that scored one made it visible.

## The document says it, so nothing has to be inferred

APB writes the **same appositive role phrase** before both identifiers, in two different lists:
`Select signal, PSELx` and `Select signal, PSEL`. That is the document asserting the identity in its own
words. `document_signal_coreferences` reads exactly that and nothing else:

- the grammar is `SPEC-TO-INTENT-ALIGNMENT.7a`'s appositive punctuation — `<role phrase>, IDENTIFIER`
  closed by a comma, a period, or the end of the statement;
- the role phrase is the at-most-four words before that comma whose last word is `signal`, with a leading
  determiner dropped so `Select signal` and `The select signal` are one key;
- a phrase links only when it names **exactly two** identifiers of which **exactly one** is declared.

The phrase is matched only against itself; its words are never interpreted, and both identifiers may be
alpha-renamed without moving the result.

## Why this is not the deleted rule

`resolve_indexed_signal_family` bound `PSEL` to `PSELX` from the spelling alone. This rule refuses that case
outright: a single appositive has nothing to co-refer with. Both pinned controls —
`temporal_condition_does_not_infer_numeric_or_x_index_aliases` and
`temporal_condition_does_not_alias_an_undeclared_name_from_suffix_spelling` — still pass unchanged. Two
*declared* identifiers under one phrase are two real wires and stay distinct; three are ambiguous and refuse.

**Corpus-wide this selects two role phrases and links exactly one** — the APB fact above. It was measured
before any code was written, which is how the "exactly one declared" and closed-punctuation guards were
chosen: without them the census also admits sentence continuations such as `, Table` and `, although`.

## Two things that are easy to get wrong

**A co-reference key is a signal name, so it is never a value.** The clause `PSEL` inside
`PSEL, PENABLE, and PREADY are asserted` resolves its signal to `PSELx` and then, unless the key is rejected
the way a declared name already is, reads its own alias text back as the VALUE. That does not merely produce
`PSELx = "PSEL"`: it puts two distinct values in the list, which disables the shared-value distribution and
silently drops `PENABLE` as well.

**A co-referenced antecedent is not `alias_dependent`.** That flag exists for the LLM-learned prose alias
(`signal_alias_map`, "address bus" → `HADDR`), which is an inference about what a description means. A
repeated appositive is direct source evidence of the same class `.7a` accepted as local declaration
authority, so marking it provisional would misreport it. The honest weakening is refusal, and the rule
refuses whenever the document does not say it twice.

## What it did and did not move

`seed_apb_temporal` went `0.333 → 1.000` with the gold file untouched, and the other eight wire numbers plus
SWD's 5/29 re-derive unchanged. Across the 27 proof-carrying chains nothing else moved — ports, actors,
declared inventory, provenance, constraints and emitted ISF counts are identical, and the other 26 chains'
temporal signatures are byte-identical. The emitted APB `.isf` is also unchanged: those rules are already
residualized at lowering because `VALID` is not an ISF literal, so this recovery improves canonical IntentIR
and the score without yet reaching the product.

Links: [[indexed-signal-family-canonicalization]], [[inference-antecedent-state-loss]],
[[base-name-template-table-is-not-a-catalogue]], [[alpha-variant-placeholder-is-not-a-wire]].
