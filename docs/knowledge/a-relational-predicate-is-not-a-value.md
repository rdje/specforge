---
id: a-relational-predicate-is-not-a-value
title: The value slot admits a state, never a relation — a predicate followed by a preposition naming an operand or a scope is refused, and the control that used to pin `must_be_value GREATER` was pinning a fabrication
answers:
  - "why does shall be unique within the subsystem publish no constraint"
  - "what does SpecForge do with must be compatible with all supported lane widths"
  - "why is must be greater than 0 not a value binding"
  - "does must_be_value GREATER exist in the corpus"
  - "which prepositions make a value slot a relation"
  - "why is by excluded from the relation complement markers"
  - "how does the value slot tell a state from a relation without an adjective list"
  - "what is the difference between is_reference_magnitude_constraint and value_slot_states_a_relation"
  - "why does a magnitude against a literal still yield no record"
  - "which admissibility routes run before the relational test"
  - "why must the value and its following word come from one scan"
date: 2026-09-13
status: current
tags: [evidence-ir, constraint-extraction, genericity, extraction-quality-gauge, claim-verification]
evidence: crates/specforge/src/ir/evidence.rs (value_slot_states_a_relation, protocol_state_value_and_complement, is_admissible_state_value, mod extraction_quality_gauge_3k_2k, mod extraction_quality_gauge_3k_1); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k.1, .3k.2b, .3k.2k); docs/book/src/pipeline/obligation-reading.md
reverify: "cargo test -p specforge-core --lib extraction_quality_gauge_3k_2k — six controls, including the_declared_level_and_numeric_routes_win_first and the_complement_comes_from_the_binder_the_value_came_from"
---

`extract_protocol_state_value` lifts the first non-filler word after `must be ` / `shall be ` and says
nothing about what it is. `EXTRACTION-QUALITY-GAUGE.3k.2b` asked one question about that word — value
or the obligation's VERB — and refused the passive participle. **There is a second thing that is not a
value and it wears no participle ending**: a predicate whose truth is not about the subject alone.

| the document writes | the slot holds | what it is |
| --- | --- | --- |
| `AWTAGOP must be Invalid` | `Invalid` | a state, complete at the word |
| `... shall be unique within the NVM subsystem` | `unique` | a relation, plus the scope it holds over |
| `... shall be compatible with all supported lane widths` | `compatible` | a relation, plus its other operand |
| `The value of PRANGE must be greater than 0` | `greater` | a relation, plus the bound it is measured against |

The discriminator is **positional, not lexical** (ADR 0006): a state is complete at the predicate,
while a relation must name its second operand or its scope and English marks that with a preposition
immediately after the predicate. No adjective list, no document vocabulary. `by` is excluded on
purpose — it marks an AGENT rather than an operand (*"must be invalidated by issuing commands"*), and
the participles it follows are already `.3k.2b`'s.

**The relational test only ever fires where `.3k.2b` left "everything else is admissible".** A value
the document declares, a logic level and a numeric literal are decided first and are untouched, so
`ZETASTATE must be Shared with the ZETAPEER` and `ZETAFMT must be 0 in every cycle` both bind however
the sentence continues.

**The value and its complement come from ONE scan.** `protocol_state_value_and_complement` returns
both, and `extract_protocol_state_value` delegates to it. A second, independent search for a relation
marker could land on a different binder occurrence than the value came from — the
two-readers-of-one-clause failure this family keeps paying for
([[one-modal-vocabulary-per-constraint-record]]).

## The clause-level refusal and the slot-level refusal are different rules

`is_reference_magnitude_constraint` (`.3k.1`) refuses a whole statement whose comparison is made
against another operand; `value_slot_states_a_relation` (`.3k.2k`) refuses a word in the value slot.
They are not the same test and they disagree on exactly one shape, which is worth holding on to:

* `must not be greater than the size indicated by the OAS field` — refused by **both**.
* `The value of PRANGE must be greater than 0` — **not** a reference magnitude (the bound is a
  literal), and still refused, because `greater` is not a state either.

**`.3k.1` shipped a control asserting the second sentence "still yields its constraint", describing it
as a value binding.** It was not one: the record it pinned was `PRANGE must_be_value GREATER`, with
the literal `0` the sentence actually names nowhere in it. The constraint vocabulary has no `at least`
kind, so a magnitude against a literal has no more of a slot than a magnitude against a reference —
the difference `.3k.1` measured is real, but it is a difference between two REFUSALS, not between a
refusal and a capture. The control now pins `.3k.1`'s own gate verdict on the literal operand, which
is what it was reaching for. **A control that asserts a record exists pins whatever that record says**,
including a fabrication, and that is how this one survived a leaf written to remove fabrications.

## The population is zero today and the class is live

Over all 78 persisted artifacts, no judged `must_be_value` record carries a relational value: the
published deterministic values are `0`, `1`, `5`, `12`, `0B01`, `0B11`, `VALID`, `LOW`, `NO`, `SET`,
`PACKED`, `INVALID`, `INVALIDATED`, `UPDATED`. `replay-constraints` over the whole corpus is
byte-identical across this change — 263 replayed, 137 reproduced, before and after.

The class is nevertheless reachable by the real producer on real corpus sentences (NVMe
`statement_7397`, OpenCAPI `statement_0613`, I2C `statement_0620`), and it becomes a published
population the moment the kind's span narrows to its own clause: `EXTRACTION-QUALITY-GAUGE.3k.3`'s
addition measurement ran the narrowed producer and NVMe gained exactly the two `must_be_value` records
the container predicted. That is why this leaf lands first — the container's ordering rationale says
the arm is fixed before the span is, or one fabricated fact is traded for another.

Related: [[constraint-record-producer-strata]], [[persisted-census-measures-published-not-current]].
