---
id: subject-phrase-that-swallowed-its-verb-is-refused
title: A subject phrase that swallowed its own verb governs a later relation verb only by coordination or a to-infinitive — otherwise the relation is refused, never trimmed
answers:
  - "why did specforge publish Arm recommends drives SPI or Manager sees drives LRCREDIT"
  - "why not trim a two-word actor like HBM responds back to its head"
  - "what is swallowed_verb_governance"
  - "why is a two-word relation actor ending in an -s word refused"
  - "how does specforge tell a finite verb from a plural noun in a relation subject"
  - "why does coordination need a verb marker (Distributor registers and provides)"
  - "which words mark a swallowed word as a verb (has does was, complementizer that)"
  - "why is a refused subject phrase not re-attributed to an earlier determiner"
  - "how many relations did KG-ISF-COMPLETENESS.1d.ii remove"
  - "does a to-infinitive keep the subject (the host tries to access, the Manager needs to drive)"
date: 2026-09-23
status: current
tags: [kg-isf-completeness, actor-signal-relations, relation-precision, grammar, adr-0006, measured]
evidence: crates/specforge/src/ir/evidence.rs (swallowed_verb_governance, extract_subject_phrase, active_verb_subject, mod kg_isf_completeness_1d_ii); docs/tasks/kg-isf-completeness/relation-precision.md (.1d.ii)
reverify: "cargo test --workspace --lib --exclude specforge-production-graph kg_isf_completeness_1d_ii — expect 8 passed. Replay: append a throwaway #[ignore] probe to ir/evidence.rs that runs extract_actor_signal_relations over every generated/evidence_ir/*/evidence_ir.json's extracted_statements, with known signals = table_signal_declaration_provenance names plus collect_known_signal_names (method in docs/tasks/kg-isf-completeness/relation-precision.md, .1d.i/.1d.ii), then remove it and cmp the file."
---

`extract_subject_phrase` takes up to two words as a relation's subject, and when the second word is the
subject's own finite verb the phrase swallows it: *"For efficient operation, Arm recommends that sources are
programmed to write SPI IDs"* published `Arm recommends drives SPI`.

**Do not fix this by trimming to the head.** The relation verb usually sits inside the swallowed verb's
complement, where the subject never governed it. Trimming turns `Arm recommends drives SPI` into the
plausible, wrong `Arm drives SPI`. The `.1d.ii` measurement found trimming right in only 2 of 21 cases.

**The rule (`KG-ISF-COMPLETENESS.1d.ii`, `swallowed_verb_governance`).** It applies when a two-word phrase ends
in a finite-verb-shaped word (lowercase, `-s`, not `-ss`/`-us`/`-is`) and clause material stands between
that word and the relation verb. Modals and short adverbs (`must`, `then`, `only`, …) do not count as clause
material. The gap is read from the clause **before** the relative-marker cut, because that cut is exactly
what hid *"that sources are programmed to"*. The outcome is one of three:
- **to-infinitive** (the gap is exactly `to`) — the head is the subject: *"the Manager needs to drive
  HTRANS"*;
- **coordination** (`and`/`or`, then nothing or a comma-closed adjunct, before a FINITE relation verb) —
  the head is the subject, but only if the word is **marked as a verb**;
- otherwise **refused**, and the search stops. Continuing to an earlier determiner re-attributes: *"…
  defined in this chapter dictates"* would publish `TL-C standard reads DATA`.

**Why coordination needs a marker (measured, do not remove).** `-s` cannot tell `sees` from the noun
`registers`. Without a marker the rule added 4 wrong relations for 1 right one, for example *"the
Distributor registers in base offset order and provides a reference"* gave `Distributor drives GIC`. Two
markers count:
- a closed-class auxiliary `has`/`does`/`was`. Without it, P-Channel's `device drives PACCEPT` (*"has
  sampled … and sets PACCEPT"*) is lost;
- a complementizer `that` followed by the new clause's own subject (*"sees that LMOPENACK is"*). A `that`
  followed directly by a verb is a relative pronoun (*"encodings that are"*) and marks nothing.

**Measured `2026-09-23`.** Replay over all 78 chains' persisted statements went from 1,218 to 1,169
relations. 50 were removed, every one read: 41 refused wrong relations and 9 collapsed onto keys already
held from other sentences. 1 was added, LTI `Manager drives LRCREDIT`, which is correct. A rebuild of the
27 replayable chains with both binaries left 25 byte-identical; ADIv6 and I2C lose only wrong relations.

**Known boundary (pinned by a test):** a two-word plural subject that a relative clause separates from its
verb (*"the Subordinate interfaces, which are optional, drive HREADYOUT"*) is refused too.

Not covered: agentless clauses (copula, imperative, fused leading PP) stay with `.1d`, and pronoun
subjects with `.1f`.
