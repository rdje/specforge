---
id: indexed-signal-family-canonicalization
title: Un-indexed prose signal refs resolve to the declared indexed family member (PSEL → PSELx)
answers:
  - "why is the PSEL antecedent dropped in a temporal rule"
  - "how does specforge handle PSEL vs PSELx (or HSEL vs HSELx)"
  - "what is index-family signal canonicalization"
  - "what does resolve_indexed_signal_family do"
  - "why does a temporal antecedent use PSELX not PSEL"
  - "how are per-instance indexed signals (PSELx HSELx) referenced in prose handled"
  - "how did APB temporal reach 100% (WIRE-BASED-100.4)"
date: 2026-06-06
tags: [temporal, semantic, signal-identity, wire-based-100, adr-0006]
evidence: docs/tasks/WIRE-BASED-100.md; crates/specforge/src/ir/semantic.rs (resolve_indexed_signal_family, temporal_clause_value, parse_temporal_condition_predicates)
reverify: ./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'
---

Specs declare per-instance signals with an index (`PSELx`, `HSELx`, or numeric `FOO0`/`FOO1`) but
prose refers to them bare (`PSEL`, `HSEL`). The bare name is not a declared signal, so a temporal
**antecedent** that mentions only the bare form was dropped — on APB this lost the `PSEL` member of
`when PSEL, PENABLE, and PREADY are asserted` (PENABLE/PREADY are declared, so they survived) and the
single `when PSEL is asserted` antecedent entirely.

Fix (`WIRE-BASED-100.4`, owner-delegated **signoff = canonicalize**, an IR uses ONE signal identity):
`resolve_indexed_signal_family(text, known_signals)` in `ir/semantic.rs` resolves an un-indexed prose
token to its declared indexed family member (declared == token + `X` or token + digits) and returns the
**declared** name, so temporal antecedents use the same canonical identity as the catalog/graph. It is
grammar (the index convention), NOT a hardcoded name (ADR 0006), and **purely additive** — it fires only
when the bare token is not itself declared, so it can never override a real declaration or invent a
signal with no declared family. Wired in `parse_temporal_condition_predicates` (after
`find_known_signal_name` fails).

Follow-on: a token that is itself a signal must never be emitted as a *value* — `temporal_clause_value`
now takes `known_signals` and rejects a value candidate that is a signal (declared or index-family), so a
bare list member like `PSEL` stays value-less and the shared `ASSERTED` distributes (was leaking
`sv|PSELX|PSEL` and dropping PENABLE). Result: APB temporal `P=R=F1=1.000` (was 0.333); the temporal gold
antecedent was corrected `PSEL`→`PSELX` to the canonical identity (fact unchanged). Reused by the
cross-spec roll (`.5` AHB `HSELx`, etc.). See `[[apb-signal-catalog-fully-extracted]]`.
