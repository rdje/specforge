---
id: a-signal-from-one-actor-to-another-names-both-ends
title: "X is the [name] signal from A to B" makes A drive X and B sample X, and names X itself, so X need not be a known signal yet
answers:
  - "how does specforge read CDBGPWRUPREQ is the signal from the debug interface to the power controller"
  - "why were ADIv6's power-up handshake wires declared nowhere"
  - "what is from_to_signal_relations"
  - "why is TCK / SWCLK is the Test Clock signal from ... refused"
  - "why did the from/to reader land after .39, .3.c.ii.a, .3.c.ii.b, .42 and KG-ISF-COMPLETENESS.1g"
  - "can a relation be read for a signal that is not yet known"
date: 2026-09-29
status: current
tags: [evidence-ir, actor-signal-relations, declarations, adiv6, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (from_to_signal_relations, from_to_actor, extract_actor_signal_relations, synthesize_directions_from_relations); docs/tasks/extraction-representation-gaps/census-leaves.md (.3.c.ii); docs/book/src/domain/actor-connectivity.md (Where graph facts come from)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_signal_from_one_actor_to_another_is_driven_by_the_first_and_sampled_by_the_second; then python3 over generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json: extracted_statements hold 'Signal CDBGPWRUPREQ is output.' and actor_signal_relations hold power controller drives CSYSPWRUPACK"
---

ADIv6 states its power-up handshake wires (`CDBGPWRUPREQ`, `CDBGPWRUPACK`, `CSYSPWRUPREQ`, `CSYSPWRUPACK`) twice. Its
programmers'-model table gives them no direction or width, so the declaration reader drops each row. Its prose names
each wire's two ends: *CDBGPWRUPREQ is the signal from the debug interface to the power controller*. The relation
reader read relations only for *known* signals, so these sentences yielded nothing.

`from_to_signal_relations` (`EXTRACTION-REPRESENTATION-GAPS.3.c.ii`, `2026-09-29`) reads *X is the [long name] signal
from A to B* as A drives X and B samples X. The sentence types X, so X need not be known. X must be the sentence's
whole subject and signal-shaped: one identifier, not an ordinary word, not all lower case. Each end is at most four
words with no *and*/*or*, and passes `normalize_relation_actor_name`. The drive then declares X through
`synthesize_directions_from_relations`. Across the 78 persisted chains the form occurs 14 times, all correct (ADIv6 8,
CoreSight Architecture 6); `TCK / SWCLK is the Test Clock signal …` is refused as an alias pair.

The order mattered more than the reader. Making four wires known let other readers fire on them, and each exposed a
defect that had to land first:
- a *When* opening a later sentence split a rule wrongly ([[a-condition-marker-after-a-sentence-end-is-fronted]]);
- *and then asserts* took an object as its actor, and a passive actor ran past a comma
  ([[an-ellipted-subject-is-never-the-object-before-and]]);
- `bit` became an actor that absorbed 32 behaviours ([[a-storage-or-value-noun-heads-no-agent]]);
- removing junk actors exposed a condition cut at `&gt;` ([[a-character-reference-semicolon-ends-no-clause]]).
