---
id: a-storage-or-value-noun-heads-no-agent
title: A relation actor whose head noun is bit, field, register, number, value or signal names no agent; the subject reader must refuse it without moving on to an earlier phrase
answers:
  - "why did ADIv6 publish IR register drives DBGTDI or number reads TDI"
  - "why is HPROT bit not an actor"
  - "what is is_storage_or_value_phrase"
  - "why does a refused subject end the determiner search"
  - "why did the first replay publish meaning drives HPROT"
  - "where does a register field bound to a wire go"
  - "why did TMC's isf gain FULL and SYNCREQS outputs"
date: 2026-09-29
status: current
tags: [kg-isf-completeness, actor-signal-relations, agent-identity, relation-precision, adr-0006, measured]
evidence: crates/specforge/src/ir/evidence.rs (is_storage_or_value_phrase, normalize_relation_actor_name, extract_subject_phrase's governed closure); docs/tasks/kg-isf-completeness/actor-surface-leaves.md (.1g); docs/book/src/domain/actor-connectivity.md (What gets filtered)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_phrase_headed_by_a_storage_or_value_noun_is_not_an_actor a_refused_storage_subject_is_not_replaced_by_an_earlier_phrase; then no actor_signal_relations record in generated/evidence_ir/*/evidence_ir.json of the 31 retained chains has an actor_name whose last word is bit, field, register, number, value or signal (or a plural)"
---

The `.1a` gate judges a relation actor by its first token, so a phrase that opens with a real-looking noun and ends
in a storage or value noun passed: `IR register`, `HPROT bit`, `read-only field`, `maximum number`, `DAT_I signal`.
Over the 78 persisted chains, 105 relations had such an actor, and every one read the head noun's referent as the
agent. Since `KG-ISF-COMPLETENESS.1g` (`2026-09-29`), `is_storage_or_value_phrase` refuses a phrase whose last
content token is one of twelve closed nouns (`bit`, `field`, `register`, `number`, `value`, `signal` and their
plurals). That is the head, so `register slice` and `debug interface` stay agents.

The gate alone is not enough. `extract_subject_phrase` scans back for a determiner, and when a phrase fails to
normalize it tries the next one, which re-attributed the relation: *the meaning of each HPROT bit* gave `meaning drives
HPROT`, and *The instruction in the IR register* gave `instruction drives DBGTDI`. So the subject reader returns
`Err` for such a phrase, the same "refused, never re-attributed" rule as
[[subject-phrase-that-swallowed-its-verb-is-refused]].

Removing these actors can move a wire onto the emitted actor. TMC's `FULL` and `SYNCREQS` had been held by the junk
`DEVID register` and `signal`. Now they are declared on the emitted actor, as the signal table states. That exposed
an unconditional *FULL is HIGH* whose condition a clause splitter had cut at `&gt;`
([[a-character-reference-semicolon-ends-no-clause]]), which is why `.42` landed first.
A sentence like *This field drives the nSRSTOUT signal* states a register-to-wire binding, owned by
`EXTRACTION-REPRESENTATION-GAPS.41`. See also [[agent-surface-defect-taxonomy]].
