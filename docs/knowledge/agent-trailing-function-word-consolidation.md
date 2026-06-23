---
id: agent-trailing-function-word-consolidation
title: The .1c.i consolidation extends the .1b.i trailing strip to a closed class of trailing PREPOSITIONS + AUXILIARIES (NON_ACTOR_TRAILING_FUNCTION_WORDS) so a dense-prose relation subject like "host has"/"host to"/"cache in" folds onto its leading agent ("host"/"cache"); conjunctions stay excluded (that is .1b.iii); corpus-safe (0 >=8-port actors are X<aux/prep> across all 78 docs)
answers:
  - "how does specforge consolidate a dense-prose agent fragment like host has or host to onto host"
  - "what is the KG-ISF-COMPLETENESS.1c.i trailing preposition/auxiliary strip"
  - "what is NON_ACTOR_TRAILING_FUNCTION_WORDS and why does it exclude conjunctions"
  - "why did .1b.i NOT strip trailing prepositions and what changed in .1c.i"
  - "how was the trailing aux/prep strip proven safe for WIRE-BASED-100 and real agents"
  - "why does the eMMC actor count drop 153 to 138 after .1c.i"
  - "where is the trailing function-word strip in consolidate_trailing_fragment"
date: 2026-06-23
tags: [kg-isf-completeness, actors, agent-surface, actor-signal-relations, consolidation, dense-prose, adr-0006, measured, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (NON_ACTOR_TRAILING_FUNCTION_WORDS + the third OR-clause in consolidate_trailing_fragment, inside normalize_relation_actor_name BEFORE the .1a reject); tests trailing_function_words_are_known_leading_non_conjunctions + trailing_function_word_consolidation_strips_aux_and_prep_to_leading_agent; docs/research/agent-identity-prose-class-measurement.md; docs/tasks/KG-ISF-COMPLETENESS.md (.1c.i). Builds on [[agent-trailing-fragment-consolidation]] (.1b.i) and the probe [[agent-identity-prose-class-measurement]] (.1c).
reverify: "cargo test -p specforge --lib trailing (the two .1c.i tests + the .1b.i tests + the two drift guards pass). Live: new-binary evidence->semantic->intent --dry-run cascade on jesd84_b50_2013_09_emmc_5_0 -> intent actors 153 -> 138, the host* variants collapse from 11 to 7 ('host has'/'host is'/'host to'/'host with' fold onto 'host'), 29 phantom names removed; host.isf renders + fsmgen --strict --check --json success/0 diagnostics. WIRE-BASED-100: rebuild the 4 wire gold docs' evidence with the new binary into a temp evidence-root, eval-extraction seed_{apb,ahb,axi,swd}{,_temporal}.json --provider skip --evidence-root <temp> = filtered F1 1.000 (constraints 6/6·6/6·3/3, relations 5/5·6/6·6/6·1/1, temporal 3/3·4/4·3/3; SWD lone constraint 0/1 documented promotion-only). kg-bench 156/156; run_ci.sh GREEN (lib 1704)."
---

**Landed `2026-06-23` (`KG-ISF-COMPLETENESS.1c.i`).** The dense-prose extension of the `.1b.i` trailing
strip (`[[agent-trailing-fragment-consolidation]]`), scoped by the `.1c` probe
(`[[agent-identity-prose-class-measurement]]`). It folds the trailing-preposition/auxiliary residue that
dense descriptive prose leaves on a real agent noun.

**The defect.** `.1b.i` strips a trailing universal verb or discourse-adverb (`Subordinate extends`→
`Subordinate`) but deliberately excluded prepositions — the stated reason being the conjunction/`.1b.iii`
coordination case. On dense descriptive prose (a JEDEC eMMC/DRAM datasheet, the long combined AMBA AXI+ACE
manual) the prose subject extractor leaves a real agent noun with a dangling trailing PREPOSITION or
AUXILIARY/modal — `host has`, `host to`, `host is`, `host with`, `cache in`, `device to`. The leading token
is a noun, so the `.1a` gate keeps it; it carries a relation, so `.1b.iv` (0/0-only) can't touch it — and it
survives as a separate phantom actor.

**The fix.** A new const `NON_ACTOR_TRAILING_FUNCTION_WORDS` (a closed class of prepositions +
auxiliaries/modals, a deliberate SUBSET of `NON_ACTOR_LEADING_FUNCTION_WORDS`, EXCLUDING conjunctions) is
added as a third strip class in `consolidate_trailing_fragment` (`ir/evidence.rs`), inside
`normalize_relation_actor_name` BEFORE the `.1a` reject — same seam/ordering as `.1b.i`. So `host has`/
`host to`/`host is`/`host with` re-attribute onto `host` and merge by dedup. Conjunctions stay excluded
(a trailing `and`/`or` is a coordinated-subject remnant `.1b.iii` splits, never strips); `before`/`after`/
`until` are already discourse markers, not duplicated. Two drift guards lock it:
`trailing_function_words_are_known_leading_non_conjunctions` (subset-of-leading + no conjunction) mirrors the
`.1b.i` guard.

**Why it is safe (genericity guardrail, ADR 0006).** Universal English grammar, not a name list. Corpus-wide
measurement: across all 78 persisted IntentIR docs ZERO actors with ≥8 ports are `X <aux/prep>` shaped, so
the strip never renames a real high-participation agent. The 4 WIRE-BASED-100 gold docs carry no such actor
(their only trailing strips are the pre-existing `.1b.i` verb/adverb cases), so the gold relation/constraint/
temporal surfaces are byte-identical — WIRE-BASED-100 held 1.000, `kg-bench` 156/156, `run_ci.sh` GREEN.

**Scope boundary.** This folds only the *trailing-function-word* class. The bulk of the dense-prose explosion
— single-relation noun-phrase phantoms with a leading noun (`basic bus`, `actual sector`) — is a
participation/grounding precision problem (a name-shape drop is disproven unsafe: AMBA's real `agent`/
`controller`/`decoder` share the shape) and is the deferred `.1c.ii`.
