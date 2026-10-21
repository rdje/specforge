---
id: an-obligation-stated-with-assume-keeps-its-class
title: A statement whose assume words are the obliged subject's (shall/must not assume), or that states an obligation in a sentence of its own, is neither apparatus nor an assumption
answers:
  - "why was 'A device shall not assume that transactions occur at the same location' apparatus"
  - "what is states_an_obligation_beside_its_assumptions"
  - "why does 'You must not assume any timing information that is not explicit in the diagrams' stay apparatus"
  - "why is 'an extra cycle is required to forward results' not an obligation (is required to states a need)"
  - "does a passive 'must be assumed to be approximately' keep its class (no: a premise)"
  - "why do some treated-as paragraphs now classify normative (an obligation sentence beside the equivalence)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.d.iii.c and .3.d.iii.e"
date: 2026-10-20
status: current
tags: [evidence-ir, classifier, assume, apparatus, obligation, statement-class, measured, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (abstraction_class; states_an_obligation_beside_its_assumptions; OBLIGATION_PHRASES; non_contract_assumption_tests); docs/archive/tasks/extraction-representation-gaps/default-and-assumption-leaves.md (.3.d.iii.c, .3.d.iii.e)
reverify: "cargo test --offline -p specforge-core --lib non_contract_assumption_tests && python3 scripts/measure_unread_intent_records.py | rg 'unread records|modalities'   # 2026-10-20: 140 unread; obligation 3"
---

`.3.d.ii.b.i`'s premise phrases (*assume that*, *assumes the*) filed a statement as apparatus whatever else it said,
and `every_assume_word_states_no_abstraction`, which already knew *shall not assume*, was only consulted for a
statement that had escaped the premise test. So *A device shall not assume that transactions occur at the same
location within each service interval* and *the debugger must not assume that the TAR wraps* were apparatus, and
paragraphs that pair a requirement with a premise (*A channel must meet the common-mode requirements … the limits
defined below assume a budgeting process*) were too.

Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.c` (`2026-10-20`) `abstraction_class` first asks
`states_an_obligation_beside_its_assumptions`; when it holds the statement takes the class it would take without the
word. It holds when **every** assume word is active and its clause holds *shall* or *must* (negated or not), or when
**another sentence** of the statement holds *shall*, *must*, *is not allowed to* or *is not permitted to*, no
abstraction word, and no document noun after *this*. Excluded on purpose, each measured on a first draft: a clause
whose subject is *you* (*You must not assume any timing information that is not explicit in the diagrams*: three
diagram keys), *is required to* (*an extra cycle is required to forward results* states a need), a disclaimer whose
*shall* sentence names *this specification*, and a passive *must be assumed* (a premise the reader is told to take).
A *treated as* paragraph with an obligation sentence beside it now classifies normative; its equivalence record is
unchanged, because `equivalence_rule_surface` reads every non-apparatus statement.

**Measured `2026-10-20`.** 24 statements in 8 chains change class (13 from apparatus, 11 from explicit abstraction),
the readers add 7 conditional rules and 3 permissions and remove nothing, SemanticIR abstractions fall by 11 and
invariants rise by 23, `.isf` texts byte-identical; unread IntentIR records 151 -> 140, obligations 8 -> 3.
Environment assumptions stated with *assumes that* (Wishbone's low-skew clock, AXI's MEC implementation, SMBus)
remained apparatus; since `.3.d.iii.e` they are assumptions ([[an-assume-sentence-about-the-document-is-apparatus]]).

Links: [[an-assume-sentence-about-the-document-is-apparatus]], [[assume-sentences-fall-into-six-shapes]],
[[a-stated-permission-is-a-typed-record]].
