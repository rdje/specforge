---
id: agent-identity-prose-class-measurement
title: The dense-prose agent explosion (eMMC 153 actors) is a relation-subject extraction-precision problem (148/153 connected phantoms from single REL-INFERRED relations, leading-noun so .1a passes), NOT actors[] prose-mint; a name-SHAPE-only drop is disproven unsafe (AMBA real agents share the shape); the clean first gate is extending the .1b.i trailing-strip to prepositions+auxiliaries — measured corpus-safe (ZERO >=8-port actors are X<aux/prep> across all 78 docs)
answers:
  - "why does the eMMC (JEDEC) IntentIR explode to 153 actors while HBM2 consolidates to 38"
  - "is the dense-prose phantom-actor explosion a relation-subject seam or an actors[] prose-mint seam problem"
  - "why can't specforge just drop single-noun or multiword actors to fix the prose phantom explosion"
  - "is it safe to extend the .1b.i trailing-fragment strip to trailing prepositions and auxiliaries (host has -> host, advantage of -> advantage)"
  - "what is Lever E / KG-ISF-COMPLETENESS.1c agent-identity precision for the dense-prose doc class"
  - "which docs exhibit the dense-prose actor explosion (is it AMBA or non-AMBA)"
  - "why do .1a and .1b.iv not catch the eMMC phantom actors like advantage of / basic bus / actual sector"
date: 2026-06-23
tags: [kg-isf-completeness, actors, agent-surface, actor-signal-relations, precision, dense-prose, corpus-coverage, adr-0006, measured, wire-based-100, probe]
evidence: docs/research/agent-identity-prose-class-measurement.md; docs/tasks/KG-ISF-COMPLETENESS.md (.1c PROBE); read-only over generated/intent_ir/<key>/intent_ir.json (78 docs). Relation-subject seam = normalize_relation_actor_name / actor_signal_relation_surface in crates/specforge/src/ir/evidence.rs (where .1a/.1b.i/.1b.iii live); actors[] prose-mint seam = build_actors Phase-2 role-term scan + .1b.iv Class-C drop.
reverify: "Read-only, no binary needed. python over generated/intent_ir: eMMC jesd84_b50_2013_09_emmc_5_0 has 153 actors / 349 relations; join actor_ports by actor_id + count actor_signal_relations by actor_name -> 148 CONNECTED (>=1 port AND >=1 rel), 5 pure-unconnected (channel/controller/source/target/transmitter, all grounded PROSE-PARA+SECTION-PHASE, 0 with ONLY a pure-inferred marker). Corpus safety: across all generated/intent_ir/*/intent_ir.json, NO actor with >=8 ports has a name whose last whitespace token is a closed-class preposition/auxiliary (of/to/in/on/is/has/...) -> trailing aux/prep strip renames zero real high-participation agents. Reach: 138 actor names corpus-wide end in a closed-class aux/prep across ~17 docs; the dense AXI+ACE ihi0022_h_c (189 actors) and CHI ihi0050_g (87) explode, the terse WIRE-BASED-100 AXI gold ihi0022_l (21) is clean."
---

**Measured `2026-06-23` (`KG-ISF-COMPLETENESS.1c` PROBE, read-only).** Spun out of `CORPUS-COVERAGE.2`
re-ingest #27 (JEDEC eMMC 5.0): the IntentIR exploded to 153 actors / 349 relations, while the structured
DRAM spec #28 (HBM2) consolidated 52→38 like the AMBA/CoreSight class. This card records WHY the `.1a`/`.1b.*`
agent-identity gates — clean on the AMBA/structured class — under-perform on dense descriptive prose, and what
the safe fix is. Companion to `[[agent-trailing-fragment-consolidation]]` (`.1b.i`),
`[[agent-identity-structural-gate]]` (`.1a`), and the AMBA-class study
`docs/research/agent-surface-fidelity-measurement.md`.

**It is a relation-subject seam problem.** Of eMMC's 153 actors, **148 are CONNECTED** (≥1 port AND ≥1
relation) — each minted from a single `REL-INFERRED` relation subject (`"advantage of" reads BACKGROUND`,
`"basic bus"`, `"B write"`, `"actual sector"`) at the `normalize_relation_actor_name` /
`actor_signal_relation_surface` seam where `.1a`/`.1b.i`/`.1b.iii` operate. The leading token is a **noun**,
so `.1a` (first-token part-of-speech) rightly keeps it; it carries a relation (hence a port), so the
`.1b.iv` Class-C drop (0/0 pure-inferred only) cannot touch it. Only **5** actors are pure-unconnected, and
those are the grounded-keep generic role terms (`channel`/`controller`/`source`/`target`/`transmitter`) the
completeness north star deliberately preserves — `.1b.iv` correctly drops 0 here. So the actors[] prose-mint
seam is already behaving; the explosion is relation-subject over-capture from descriptive prose.

**A name-SHAPE-only drop is disproven unsafe.** AMBA's REAL agents occupy the SAME structural shape classes
as eMMC's phantoms: `agent`/`controller`/`decoder`/`device` are real single-lowercase-noun agents;
`address decoder`/`Exclusive Access Monitor` are real multiword agents. Dropping an actor on shape alone
would destroy real AMBA agents and break WIRE-BASED-100. The differentiator must be grammatical
**normalization** (rewrite, not drop) or **participation/grounding**, never shape (the genericity guardrail,
`[[feedback_avoid_denylists_prefer_structural]]`).

**The clean, corpus-safe first gate (`.1c.i`).** Extend the proven `.1b.i` trailing-strip from
verbs/discourse-adverbs to a closed class of trailing **prepositions + auxiliaries** (`advantage of`→
`advantage`, `host has`→`host`, `cache in`→`cache`) — a rewrite that re-attributes the relation onto the head
noun. Safety is measured CLEAN: across all 78 persisted IntentIR docs, **zero** ≥8-port actors are
`X <aux/prep>` shaped, so the strip renames no real high-participation agent (the `.1a`/`.1b.i` bar); the 4
wire golds carry no such actor (structurally untouched). Reach: 138 names / ~17 docs — and the doc class is
**dense-prose specs** (AXI+ACE `ihi0022_h_c` 189 actors and CHI `ihi0050_g` 87 explode too), not "non-AMBA".
It is necessary-not-sufficient: the bulk (≈120) are single-relation noun-phrase phantoms with a leading noun.
`.1c.ii` MEASURED this class (`2026-06-23`) and found **no clean within-document structural gate**: a
`.1b.ii`-style connectivity fold mishandles the real cases on AXI+ACE `ihi0022_h_c` (`caching Manager`→
`Manager` correct, but `Manager component`→`component` WRONG — the agent is the modifier, not the noun-phrase
head; the agent token's position varies), and within one document a real descriptive reference
(`caching Manager`) and a phantom (`basic bus`) are structurally indistinguishable (same single relation, same
`SECTION-PHASE`/`PROSE-PARA` provenance, same shape; eMMC has no `ProtocolActorRecord` grounding surface). A
drop is forbidden by the genericity guardrail; a participation threshold by completeness. So `.1c.ii` is a
**bounded honest residual** — the genuine fix is upstream relation-subject extraction precision on descriptive
prose (`[[project_nlp_shallow_parse_direction]]`), not a downstream actor-surface rule; the phantoms never
reach the emitted `.isf`.
