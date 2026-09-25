---
id: relation-verb-after-an-article-is-a-noun
title: A relation verb directly after an article or determiner is a noun and licenses no actor-signal relation — 124 relations left the corpus, all wrong, golds unmoved
answers:
  - "why did specforge publish value reads APB from the read pointer"
  - "why are there relations like Any reads CACHE or details reads TDO"
  - "what is is_nominal_verb_occurrence"
  - "which determiners stop a relation verb from licensing a relation"
  - "why is that not in the nominal determiner list (the Manager that drives HTRANS)"
  - "why is its not a determiner for relation verbs (GIC ITS agent)"
  - "why does a capital A not count as an article (Master A asserts address)"
  - "where do phrase-shaped relation actors come from"
  - "how many relations did KG-ISF-COMPLETENESS.1d.i remove"
  - "does a noun use of read or access create a relation"
  - "why did specforge publish Manager reads HTRANS from this next access (a determiner then a sequence modifier makes access a noun)"
  - "where does an active relation verb's object end (at but, and at an infinitive with its own subject: for the Manager to drive)"
date: 2026-09-23
status: current
tags: [kg-isf-completeness, actor-signal-relations, relation-precision, grammar, adr-0006, measured]
evidence: crates/specforge/src/ir/evidence.rs (is_nominal_verb_occurrence, extract_actor_signal_relations, mod kg_isf_completeness_1d_i); crates/specforge/src/ir/normative_vocab.rs (ACTIVE_READS_VERBS, ACTIVE_DRIVES_VERBS); docs/tasks/kg-isf-completeness/relation-precision.md (.1d, .1d.i)
reverify: "cargo test --workspace --lib --exclude specforge-production-graph kg_isf_completeness_1d_i — expect 5 passed. Replay: append a throwaway #[ignore] probe to ir/evidence.rs that runs extract_actor_signal_relations over every generated/evidence_ir/*/evidence_ir.json's extracted_statements (method in docs/tasks/kg-isf-completeness/relation-precision.md, .1d.i), then remove it and cmp the file."
---

`extract_actor_signal_relations` licenses an active relation at **any spelling** of a vocabulary verb whose
object holds the signal, and the vocabulary carries forms English uses as nouns and modifiers — `read`,
`access`, `writes` among them. So *"the value of the read pointer … over the APB interface"* minted
`value reads APB`, and the actor was whatever phrase stood before the noun. That is where a large share of
the phrase-shaped actors came from: they were never subjects, because there was no clause.

**The rule (`KG-ISF-COMPLETENESS.1d.i`).** `is_nominal_verb_occurrence` refuses an occurrence whose
preceding word is `the`, `an`, lowercase `a`, `this`, `these`, `those`, `each`, `every`, `any`, `no` or
`another`: a word an article governs heads or modifies a noun phrase and cannot be a clause's finite verb.
It is checked at all four active licensing sites. Grammar, not names (ADR 0006).

**Three measured exclusions — do not add them back:**
- `that` — also the relative pronoun of *"the Manager that drives HTRANS"*;
- `its` — collides with the GIC `ITS` agent: *"the ITS forwards any operations"* (the `.1a` collision);
- capital `A` — a label: *"1. Master A asserts address"* is `Master drives ADDRESS`. Only lowercase `a` is
  the article; a case-blind check fails the seam test that pins this.

**Measured `2026-09-23`.** Replaying the current extractor over all 78 chains' persisted statements: 1,342
→ 1,218 relations, 124 removed across 16 documents, 0 added, every removal read and wrong (`read` 84,
`access` 27, `writes` 11). Rebuilding the 27 replayable chains with both binaries: 25 byte-identical, AHB and
ADIv6 lose 17 relations, WIRE-BASED-100 scores identical on all seven seeds. Two of the 17 were true but
never evidenced — AHB `Subordinate reads HREADY` came from *"drives the read data bus"*, and its check
signal inherited it — and are owned as `.1d.i.a` rather than kept on a misread sentence.

What it does not touch: a subject cut that swallows its own verb (`HBM responds`, `.1d.ii`), clauses with no
agent (copula, imperative), and pronoun subjects (`.1f`).

**One word further, and two clause boundaries (`2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS.27.f`).** A determiner
may stand one sequence modifier away (`next`, `first`, `last`, `previous`, `following`, `subsequent`, `same`, `single`,
`current`): AHB's *cancel this next access and drive HTRANS* had published `Manager reads HTRANS`. And an active verb's
object now ends where another agent's clause begins: at `but` (*The Manager controls the move …, but the Subordinate
can use the ASKSTOP signal* had published `Manager drives ASKSTOP`) and at `for <noun phrase> to` (*The two-cycle
response provides sufficient time for the Manager to … drive HTRANS* had published `two-cycle response drives HTRANS`).
Over the 27 live chains 8 relations left, all false, and none arrived.
