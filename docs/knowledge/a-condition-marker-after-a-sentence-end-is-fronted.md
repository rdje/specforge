---
id: a-condition-marker-after-a-sentence-end-is-fronted
title: A When/While/During/Unless that follows a sentence end opens its own sentence's fronted condition, so the sentence before it is no consequent
answers:
  - "why did a conditional rule take its consequent signal from the previous sentence"
  - "why was ADIv6's CSYSPWRUPREQ power-up rule's antecedent the whole When sentence"
  - "how does split_conditional_sentence read a statement of several sentences"
  - "why did AXI's BROADCASTATOMIC rule have BROADCASTATOMIC as its consequent signal"
  - "why were TMC's When asserted: rules removed"
  - "does a conditional rule's consequent end at its sentence (yes, since .39.a)"
  - "what is sentence_end_in"
  - "which signal is a conditional rule's consequent signal"
date: 2026-09-29
status: current
tags: [evidence, conditions, conditional-rules, sentence-boundaries, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (split_conditional_sentence, extract_conditional_rules); docs/book/src/pipeline/obligation-reading.md (A condition can come first); docs/tasks/extraction-representation-gaps/recent-leaves.md (.39, .39.a)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_condition_marker_after_a_sentence_end_opens_its_own_sentence; then python3 over generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json: the conditional rule whose source_text starts 'CSYSPWRUPREQ is the signal from' has antecedent_text 'CSYSPWRUPREQ is asserted'"
---

`extract_conditional_rules` classifies a whole statement, which may hold several sentences, and
`split_conditional_sentence` finds its condition two ways: a fronted marker at the very start of the statement
(*When X, Y*), or else an embedded one (*Y when X*) found by searching for ` when `, ` while `, ` during `,
` unless `. Until `EXTRACTION-REPRESENTATION-GAPS.39` (`2026-09-29`) that search also found a *When* opening a later
sentence. ADIv6's *CSYSPWRUPREQ is the signal from … . When CSYSPWRUPREQ is asserted, the power controller … asserts
CSYSPWRUPACK* then gave the earlier sentences as the consequent and the whole *When* sentence as the antecedent.
The consequent signal, the first known signal in the consequent, came from the wrong sentence. It was right only by
chance, for as long as the one known signal there happened to be the acknowledge.

Now a marker whose preceding text ends in `.`, `:` or `;` opens a sentence, and that sentence is split as a
statement-initial one would be. A sentence the fronted path cannot split (no comma, no *then*) is refused, as it is
at the start of a statement. On the 31 current chains, 105 of 1,130 rules moved, in 21 documents, and only in
`conditional_rules`. 93 narrowed their antecedent to a strict prefix; 8 changed consequent signal; 1 changed
action; 6 were refused. No `.isf` text moved.

Since `.39.a` the fronted path reads one sentence (`sentence_end_in`, where a period after a word holding `_` is a
register's field separator, not an end), so ADIv6's *When nSRST is asserted, the entire system must be reset …* no
longer takes `nSRST` from its third sentence. That moved 15 rules on the 31 chains, and no `.isf`. Since `.43` an
obligation's consequent signal is its subject's first declared signal, or with an agent subject the first after the
modal (*…, so RCHUNKNUM and RCHUNKSTRB must be …* gives `RCHUNKNUM`). A consequent without a modal still takes its
first signal, so I2C's *transistor TR2 opens to separate the SCLH and SCL lines* gives `SCLH` (`.43.a`).
See also [[a-fronted-condition-ends-after-its-list]] and [[a-condition-never-contains-its-obligation]].
