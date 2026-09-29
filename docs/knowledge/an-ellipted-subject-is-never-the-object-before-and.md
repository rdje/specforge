---
id: an-ellipted-subject-is-never-the-object-before-and
title: A relation verb after "and [then]" whose shared verb relates no signal keeps the plain subject only when a verb follows that phrase; a passive actor phrase ends at its punctuation
answers:
  - "why did ADIv6 publish system power drives CSYSPWRUPACK"
  - "what is the subject of a relation verb after and then"
  - "when does active_verb_subject refuse a coordinated relation verb"
  - "what is is_verb_shaped"
  - "why is the target drives SCL relation in I2C kept"
  - "why did an actor read debugger CDBGPWRUPREQ"
  - "does extract_actor_phrase stop at a comma"
date: 2026-09-29
status: current
tags: [evidence-ir, actor-signal-relations, relation-precision, grammar, coordination, adr-0006, measured]
evidence: crates/specforge/src/ir/evidence.rs (active_verb_subject, extract_subject_phrase's ellipted_subject, is_verb_shaped, extract_actor_phrase); docs/tasks/extraction-representation-gaps/census-leaves.md (.3.c.ii.a, .3.c.ii.b)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_coordinated_relation_verb_takes_no_subject_from_the_object_before_the_conjunction a_passive_actor_phrase_ends_at_its_punctuation; then python3 over generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json: no actor_signal_relations record has actor_name 'system power'"
---

A finite relation verb opened by *and* or *or* has an ellipted subject, shared with the verb it is coordinated with
(`KG-ISF-COMPLETENESS.1e`). When that verb is itself a relation verb, its subject is read. Otherwise,
`active_verb_subject` used to fall back to the plain subject reading, which takes the nearest determiner phrase before
the relation verb. After a transitive verb, that phrase is the verb's object. ADIv6's *the power controller powers
up the system power domain and then asserts CSYSPWRUPACK* published `system power drives CSYSPWRUPACK`, and the
relation declared the acknowledge an output and made a junk actor with an unconditional drive.

Since `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.a` (`2026-09-29`):

- *then* between the conjunction and the verb is looked through;
- in that fallback, `extract_subject_phrase(…, ellipted_subject: true)` keeps a phrase only when a verb-shaped word
  (`is_verb_shaped`: a closed-class copula, auxiliary or modal, or a lower-case `-s` word) follows it before the relation
  verb. The phrase's own swallowed second word counts (*the target is ready for another byte of data and releases
  clock line SCL* keeps `target`).

The `-s` test admits plural nouns (`inputs`), which can only keep a reading, so the rule removes nothing the old one
kept except an object. On the 31 current chains it removed exactly one relation, ADIv6's, with what hung off it (the
declaration, the unconditional temporal rule, three behaviours, `CSYSPWRUPACK`'s interface record and constraint),
all of which `.3.c.ii` restores with the power controller as driver.

`extract_actor_phrase` (the passive reader, *X is asserted by …*) stripped a word's trailing delimiter and kept
reading: *When CSYSPWRUPREQ is asserted by the debugger, CDBGPWRUPREQ must also be asserted* named the actor
`debugger CDBGPWRUPREQ`. Since `.3.c.ii.b` a word that ends in a delimiter ends the phrase. It moves no current
chain, since no persisted actor spans such a delimiter, and matters as soon as `.3.c.ii` makes `CSYSPWRUPREQ` known.
See also [[subject-phrase-that-swallowed-its-verb-is-refused]].
