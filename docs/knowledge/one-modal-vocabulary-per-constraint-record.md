---
id: one-modal-vocabulary-per-constraint-record
title: Every part of a constraint record reads one clause for its modal, so they must all read the same modal set — and the negated VALUE binding the vocabulary has a slot for has no corpus population at all
answers:
  - "does SpecForge type an obligation stated with cannot or will not"
  - "why did a must not be changed obligation type while cannot be changed did not"
  - "which functions decide whether a clause states an obligation"
  - "what modal words does obligation_is_negated accept"
  - "why does constraint_bearing_sentence need the same modals as the kind table"
  - "how many corpus constraints need a negated form of extract_protocol_state_value"
  - "does the corpus contain a negated value binding the extractor cannot read"
  - "why did teaching the classifier a new modal publish a NOTE constraint"
  - "what happens when an obligation sentence cannot be located"
  - "is MustBeValue plus negated ever produced by a deterministic path"
date: 2026-09-12
status: current
tags: [evidence-ir, constraint-extraction, genericity, extraction-quality-gauge, claim-verification]
evidence: crates/specforge/src/ir/evidence.rs (normalize_obligation_modal, sentence_states_an_obligation, obligation_is_negated, classify_signal_constraint_kind, constraint_bearing_sentence, mod extraction_quality_gauge_3k_2d); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3i, .3k.2a-.3k.2d)
reverify: "cargo test -p specforge-core --lib extraction_quality_gauge_3k_2d — five controls, including every_negating_modal_can_locate_its_own_obligation_sentence, which asserts the property rather than a list of examples"
---

A `SignalConstraintRecord` is assembled by several functions, and three of them ask the same question
of the same clause — **does this state an obligation, and is it negated?** Each used to answer with
its own list of modal words:

| the reader | what it decides | the modals it read |
| --- | --- | --- |
| `obligation_is_negated` | is the record negated | `must not`, `shall not`, `must never`, `shall never`, `cannot`, `will not` |
| `constraint_bearing_sentence` | which sentence IS the obligation | `must`, `shall` |
| `classify_signal_constraint_kind` | which kind it states | every phrase is spelled `must`/`shall` |

So an obligation a document writes with `cannot` was flagged negated by the first, given no sentence
of its own by the second, and typed as nothing by the third. `EXTRACTION-QUALITY-GAUGE.3i` found the
same defect one level down — every phrase in the table was AFFIRMATIVE — and answered it with three
more literals (`must not be changed`, `must not be asserted`, `must not be active`). The general form
is that a literal phrase table states its vocabulary twice, once in the words and once in the modal,
and only the second can be normalized away: `normalize_obligation_modal` reduces the equivalent
negative modals to the `must not` form the table is written in, and `sentence_states_an_obligation`
gives the sentence scan that same set.

**Both halves are load-bearing, and the failure mode of shipping one is specific.** A modal the
classifier knows and the sentence scan does not leaves the record's span falling back to the whole
statement — which, for a serialized table row, is the row's other cells. Measured over the corpus,
the classifier half alone published two records whose subject was a row's `NOTE` marker, taken from
`| NOTE 1 | … A Device … will not change its state to the rcv state. … |`. With both readers taught,
the obligation has a sentence of its own, the marker is not in it, and the corpus delta is zero.

**The measured population, and the thing worth not re-deriving.** Enumerated over all 78 persisted
artifacts as the full cross product of those six modals with the binder verbs
`extract_protocol_state_value` reads: 31 `signal_value_constraint` statements carry a negated binder,
of which 12 reach the untyped arm. Read against source, **not one of them needs a negated form of the
value binder** — nine are *"cannot/will not be changed"* (a no-change obligation, modal-blocked), two
are reference magnitudes `EXTRACTION-QUALITY-GAUGE.3k.1` correctly refuses, and one is *"QREQn cannot
be driven HIGH"* (a level obligation the table already owns, modal-blocked). In the table-row path,
of 50 admitted obligation clauses, **zero** carry one of these modals.

So the vocabulary slot `MustBeValue` + `negated` — which `.3k.2b` named and `.3k.2d` was opened to
fill — is reached by **no corpus clause in either caller of the kind classifier**. The gap is real as
a capability and empty as a population. The one genuine negated value binding in the corpus,
AMBA LTI *"When LAMMUV is 1 and LAPM is 1, LAFLOW must not be Stall"*, is unreachable for an unrelated
reason: it sits in a table CONTINUATION row whose name cell is empty, so
`resolve_declared_signal_identifier` fails and every obligation in that row is dropped
(`EXTRACTION-QUALITY-GAUGE.3k.2h`).

**How the leaf's own premise was wrong, because the shape recurs.** `.3k.2d` opened citing *"The DV
operand must not be 1 for IODIR"* as the defect's instance. That is a TEST STRING. The document's own
statement is RISC-V IOMMU `statement_1033`, class `conditional_rule`, and its records are
`dyn_sigcon_0008`/`0009` — the DYNAMIC path, which never calls this classifier. A rule written from a
description of a producer rather than from the producer is green precisely where it is blind
(`CLAIM_VERIFICATION.md` §3 Leg 2); here it was not even pointed at the right producer.

Related: [[constraint-record-producer-strata]] (which producer a record came from decides whose
population it is), [[persisted-census-measures-published-not-current]] (size against what the
extractor mints today, not against what the corpus published).
