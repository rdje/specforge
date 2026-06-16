---
id: agent-trailing-fragment-consolidation
title: The .1b.i consolidation strips a TRAILING universal verb/discourse-adverb off a relation-subject ("Subordinate extends"->"Subordinate", "decoder also"->"decoder") inside normalize_relation_actor_name BEFORE the .1a reject, so stranded relations re-attribute onto the real agent (no name list, ADR 0006); conjunctions and "X interface" are deliberately excluded
answers:
  - "how does specforge consolidate a Class-B agent fragment like Subordinate extends onto Subordinate"
  - "what is the KG-ISF-COMPLETENESS.1b.i trailing-fragment consolidation"
  - "where is consolidate_trailing_fragment and how is it ordered against the .1a reject"
  - "why does decoder go from 0/0 to connected (decoder also consolidated)"
  - "what is NON_ACTOR_TRAILING_DISCOURSE_MARKERS and why is it a subset of the leading function-word list"
  - "why does .1b.i NOT strip a trailing conjunction (and/or) or X interface"
  - "how was the trailing-fragment consolidation proven not to regress real agents (WIRE-BASED-100)"
date: 2026-06-16
tags: [kg-isf-completeness, actors, agent-surface, actor-signal-relations, completeness, consolidation, adr-0006, measured, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (consolidate_trailing_fragment + NON_ACTOR_TRAILING_DISCOURSE_MARKERS, wired into normalize_relation_actor_name between the meaningful check and the .1a is_non_actor_phrase_fragment reject); docs/research/agent-surface-fidelity-measurement.md §7; docs/tasks/KG-ISF-COMPLETENESS.md (.1b.i)
reverify: "cargo test -p specforge --lib trailing_fragment_consolidation (and trailing_discourse_markers_are_known_leading_words). Live: build fresh AHB evidence->semantic->intent (Pattern, --dry-run into a temp evidence-root) and confirm Subordinate 25/23->27/26, decoder 0/0->4/2, and the fragment actors 'Subordinate extends'/'Subordinate then'/'decoder also' are GONE; AXI real agents byte-identical (Manager 169/Subordinate 168/interconnect 5). WIRE-BASED-100: eval-extraction seed_{axi,apb,ahb,swd}{,_temporal}.json --provider skip --evidence-root <fresh-Pattern root> stays 1.000 on constraints(AXI/APB/AHB)+relations(x4)+temporal(x3); kg-bench 156/156."
---

**Landed `2026-06-16` (`KG-ISF-COMPLETENESS.1b.i`).** The first completeness fix for the agent surface
(`[[agent-surface-defect-taxonomy]]` Class B — Fragment of a real agent), complementing the `.1a` precision
gate (`[[agent-identity-structural-gate]]`). It recovers the relations a real agent had stranded under a
fragment spelling of its name.

**The defect.** The prose subject extractor sometimes captures a real agent noun together with a dangling
trailing verb or discourse-adverb — `Subordinate extends`, `Subordinate then`, `decoder also`. Because the
LEADING token is a real noun, the `.1a` gate (which judges only the first token) rightly keeps it — but it
survives as a SEPARATE phantom actor, holding relations that belong to the real agent. AHB `decoder` read
0/0 only because its `drives HSELx` relations were stranded under `decoder also`/`address decoder`.

**The fix.** `consolidate_trailing_fragment(value)` in `ir/evidence.rs` strips trailing tokens that are a
`NON_ACTOR_LEADING_VERBS` token or a `NON_ACTOR_TRAILING_DISCOURSE_MARKERS` adverb (`then`/`also`/`next`/…),
keeping at least the leading content token. It is wired into the DRY seam `normalize_relation_actor_name`
**between the `is_meaningful_actor_term` check and the `.1a` `is_non_actor_phrase_fragment` reject** — the
ordering matters: consolidate first so `Subordinate extends`→`Subordinate` is KEPT, then `.1a` judges the
clean leading head. The relation now re-attributes onto the real agent, and `dedup_actor_signal_relations`
merges it; the fragment, left with no relations, is never minted as a Phase-1 actor.

**Deliberate exclusions (universal grammar, ADR 0006 — no name list).**
- **Conjunctions** (`and`/`or`) are NOT trailing-strip targets — a trailing `and` is a coordinated-subject
  remnant (`Subordinate and decoder`) for `.1b.iii` to SPLIT, not strip.
- **`"X interface"→"X"`** is NOT done here (`.1b.ii`): a named-interface block (GIC `CPU interface`, the
  GICC) is a distinct entity from generic `CPU`; that strip needs an "only when X is already a real connected
  agent in this doc" sub-gate, which the pure-string seam lacks.
- The trailing-marker const is a SUBSET of `NON_ACTOR_LEADING_FUNCTION_WORDS` (different structural role:
  trailing-strip vs leading-reject), pinned by the `trailing_discourse_markers_are_known_leading_words`
  drift-guard test so the two lexicons can't silently diverge.

**Safe by construction + proven.** Returns the input byte-identical when nothing strips (preserves the
byte-stability guarantee for fragment-free docs). Infrastructure can never surface (the full string already
passed `normalize_table_actor_name`'s infra reject; a leading-token prefix introduces no new substring).
Measured net effect over the persisted corpus (ordered consolidate→`.1a`): ZERO real agents (≥8 ports)
vanish; residual junk (`does not`→`does`, `It also`→`It`) consolidates to a function-word head the `.1a`
reject then removes. Live AHB rebuild: `Subordinate` 25/23→**27/26**, `decoder` 0/0→**4/2**, actors 25→19;
AXI real agents byte-identical, actors 21. WIRE-BASED-100 held at 1.000 (constraints ×3 / relations ×4 /
temporal ×3); `kg-bench` 156/156; `run_ci.sh` green (+2 tests, lib 1635).

See `[[agent-surface-defect-taxonomy]]` (the 3-class taxonomy this addresses Class B of),
`[[agent-identity-structural-gate]]` (the `.1a` precision gate it runs after), and
`[[feedback_avoid_denylists_prefer_structural]]`.
