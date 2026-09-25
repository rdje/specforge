---
id: temporal-condition-read-whole
title: A temporal rule's condition is read whole or not at all — a list member takes the value that closes its list, an unless/except clause is a typed exception (never an antecedent, never dropped), and a partly read condition yields no guard
answers:
  - "how does SpecForge read a temporal condition with an unless or except clause"
  - "what is TemporalRuleRecord exceptions"
  - "what is Condition::Ne"
  - "why did PRUSER's condition lose PSEL and PENABLE"
  - "why does a list condition with two values drop its first members"
  - "is an unless clause an antecedent (no — an exception, kept apart and lowered negated)"
  - "what happens to a temporal condition that is only partly read"
  - "why is a conditional rule with an unread condition listed instead of emitted"
  - "how does the isf adapter lower an exception"
  - "does the WIRE-BASED-100 temporal key score exceptions"
  - "is until an exception (no — it releases a persistence)"
date: 2026-09-25
status: current
tags: [semantic-ir, temporal, conditions, isf-adapter, wire-based-100, extraction-representation-gaps]
evidence: crates/specforge/src/ir/semantic.rs (parse_temporal_condition; read_condition_clauses; a_list_member_takes_the_value_of_the_clause_that_closes_its_list; an_exception_is_kept_apart_from_the_condition_and_an_until_is_not_one; a_condition_read_only_in_part_yields_nothing); crates/specforge/src/ir/contract.rs (Condition::Ne; guard_candidates_from_antecedents); crates/specforge/src/ir/isf_ir.rs (guard_from_candidates; unread_stated_condition); crates/specforge/src/eval.rs (temporal_rule_key); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.26)
reverify: "cargo test --offline --workspace --lib --exclude specforge-production-graph -- a_list_member_takes a_condition_read_only an_exception_is_kept a_rule_keeps_its_exception a_rules_exceptions_join contracts_that_differ_only_in_an_exception a_temporal_key_carries an_exception_renders"
---

A rule's antecedents are the condition its sentence states. A guard built from part of it applies the
obligation in cycles the document excludes, so `parse_temporal_condition` (`ir/semantic.rs`) returns the whole
condition or nothing.

## The three rules

- **A list states its value once, after its last member.** *PSEL, PENABLE, and PREADY are asserted, and PWRITE is
  deasserted*: a member with no value of its own takes the value of the next clause that has one. Before
  `EXTRACTION-REPRESENTATION-GAPS.26` a value was distributed only when the whole condition stated exactly one, so
  the first list's members were dropped (APB `sigcon_0015` kept `PREADY` and `PWRITE`). A trailing member that no
  clause follows still takes the condition's single stated value.
- **`unless`/`except` introduce an exception, kept in `TemporalRuleRecord::exceptions`.** WIRE-BASED-100.5h was
  right that *unless HRESP is ERROR* is not an antecedent; cutting it was wrong, because the rule then applied when
  HRESP is ERROR. `until` is different: it releases a persistence and is still cut (the obligation it belongs to is
  `.25.c`'s).
- **Anything left unread empties the condition**: a named signal with no value, an exception naming no signal, or
  a head no predicate carries alongside an exception. The rule keeps its source text and no antecedent, and the
  adapter lists it (`unread_stated_condition`, which now covers conditional rules as well as constraints).

## Downstream

The contract carries an exception as `Condition::Ne` in `guard_candidates`, so fusion keeps apart two contracts
that differ only in one. The adapter lowers it as `(! (== S L))` inside the `(& …)` guard; an exception at the
level the guard requires is a contradiction, and one whose value is no level (`ERROR`, an enum member until `.9`)
refuses the rule. FSMGen strict accepts the negated form at pin `54ac1c22`; the lowered HDL was not inspected,
because FSMGen writes HDL only inside its own repository. The eval key gains an `|X:` segment only for a rule with
an exception, so every other key is byte-identical; the two AHB golds that state one were labelled with it.

## Measured `2026-09-25`

Of the 16 temporal rules on the live chains whose stated condition yields a guard, 5 had kept part of it (APB's
list, AHB's four *unless HRESP signal is ERROR*). None was emitted. The fix changed only APB-E's and AHB's
SemanticIR (the other 25 chains identical at SemanticIR, IntentIR and adapter), and their adapters and AHB's
`manager.isf` stayed identical, since those obligations are refused for their own reasons (`VALID`, stability).

Links: [[document-stated-identifier-coreference]], [[isf-emission-blocked-by-clock-reset-contract]].
