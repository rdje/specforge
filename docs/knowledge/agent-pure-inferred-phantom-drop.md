---
id: agent-pure-inferred-phantom-drop
title: The .1b.iv Class-C PURE-INFERRED phantom drop removes a generic role-term actor (agent/controller/producer/…) the SemanticIR Phase-2 scan minted only because a statement MENTIONED the word — when its IntentIR responsibilities reduce to exactly the term-scan marker "semantic role inferred around `X` evidence" (0 ports, 0 rels, no phase/contract grounding); a guard in build_intent_actors, marker-SHAPE match not a name list (ADR 0006), grounded 0/0 agents kept
answers:
  - "how does specforge drop a zero-evidence phantom actor like controller or agent"
  - "what is the KG-ISF-COMPLETENESS.1b.iv pure-inferred phantom drop"
  - "why is AXI transmitter / SWD host / GIC arbiter kept but APB controller / AHB agent dropped"
  - "where is is_pure_inferred_phantom_role and the build_intent_actors phantom guard"
  - "what distinguishes a PURE-INFERRED phantom from a PROSE-GROUNDED or SECTION+INFERRED 0/0 actor"
  - "why does dropping phantom actors leave the .isf and WIRE-BASED-100 unchanged"
  - "how does the SemanticIR Phase-2 role-term scan (build_actors ACTOR_TERMS) mint Class-C actors"
date: 2026-06-17
tags: [kg-isf-completeness, actors, agent-surface, completeness, precision, phantom, role-term, adr-0006, measured, wire-based-100, intent-ir]
evidence: crates/specforge/src/ir/intent.rs (is_pure_inferred_phantom_role + the guard in build_intent_actors that `continue`-skips a pure-inferred phantom); crates/specforge/src/ir/semantic.rs:3177 (build_actors ACTOR_TERMS scan, marker template :3190); docs/research/agent-surface-fidelity-measurement.md §8; docs/tasks/KG-ISF-COMPLETENESS.md (.1b.iv)
reverify: "cargo test -p specforge --lib pure_inferred (pure_inferred_phantom_marker_matches_producer_template drift-guard + build_intent_actors_drops_pure_inferred_phantom_but_keeps_grounded). Live: rebuild the 4 wire docs evidence->semantic->intent into a TEMP evidence-root (symlink canonical generated/source_ir in, run the release binary from the temp CWD so the relative output root redirects there), then confirm the intent actors drop APB `controller` + AHB `agent` only, AXI/SWD unchanged, and AXI `transmitter`/SWD `host` KEPT. Stage-diff vs an old-binary temp build: evidence+semantic byte-identical (modulo embedded input-path), actor_signal_relations/signal_constraints/temporal_rules byte-identical. WIRE-BASED-100: eval-extraction seed_{apb,ahb,axi}{,_temporal}.json --provider skip --evidence-root <fresh root> stays source-tolerant 1.000; kg-bench 156/156."
---

**Landed `2026-06-17` (`KG-ISF-COMPLETENESS.1b.iv`).** The Class-C honesty slice of the agent-surface
taxonomy (`[[agent-surface-defect-taxonomy]]` Class C), after `.1a` precision
(`[[agent-identity-structural-gate]]`), `.1b.i` trailing consolidation
(`[[agent-trailing-fragment-consolidation]]`), and `.1b.iii` coordinated-subject split
(`[[agent-coordinated-subject-split]]`). It removes the unambiguous generic-vocabulary phantom actors
so the canonical `IntentIR.actors[]` reflects only real protocol agents (north-star bar #1).

**How the phantom is minted.** `build_actors` (semantic.rs:3177) scans every statement for one of 24
generic `ACTOR_TERMS` (`agent`/`controller`/`producer`/`consumer`/`requester`/`encoder`/`host`/…) and
mints an actor the moment the word appears, with `role_summary` =
``"semantic role inferred around `<term>` evidence"`` (semantic.rs:3190). A term-scan-only actor has no
relations (the term scan adds only supporting-statement ids), hence no ports — it is 0/0 by
construction. If a relation actor of the same name already exists, the relation accumulator wins the
slot (its summary is the distinct ``"… from actor-signal relation evidence around `X`"``), so the
phantom marker uniquely tags a term-scan-ONLY actor.

**The discriminator (research §8, the recommended "drop only PURE-INFERRED").** At `build_intent_actors`
(intent.rs) — the single place an actor's `responsibilities` set is assembled (role_summary + contract
sentences + `participate in <phase>` + channel note) — an actor is dropped iff its responsibilities is
**exactly the single term-scan marker** (`responsibilities.len() == 1 && is_pure_inferred_phantom_role`).
`is_pure_inferred_phantom_role` matches the marker SHAPE
(``starts_with("semantic role inferred around `") && ends_with("` evidence")``), never a chip-name list
(ADR 0006). A drift-guard test pins the detector to the producer template.

**Why it is conservative and safe (measured invariant).** A grounded 0/0 actor keeps a phase or contract
responsibility (length > 1) and a connected actor carries the relation-evidence summary — so neither
matches. Proven over the fresh wire IR AND the persisted 36-doc corpus: drops exactly **21 phantoms / 16
docs** (wire: APB `controller`, AHB `agent`; AXI/SWD 0) with **ZERO connected or grounded actors
touched**. Genuinely-discussed-but-unwired agents (AXI `transmitter`, SWD `host`, GIC-class `arbiter`)
are deliberately KEPT — completeness over aggressive pruning, honouring the owner's north star.

**Zero blast radius downstream.** `actor_ids` (→ behaviors) and `build_assumptions` both derive from the
returned `actors`, so the drop propagates with no dangling reference (a 0/0 phantom is referenced by
nothing else). Stage-diff old-vs-new binary: evidence + semantic byte-identical, intent differs only by
the removed phantoms (+ the phantom id leaving the global behaviors' `actor_ids`);
`actor_signal_relations`/`signal_constraints`/`temporal_rules` byte-identical; the emitted `.isf` is
byte-identical (the ISF emitter lowers signals/behaviors from `actor_ports`, never the raw `actors[]`).
WIRE-BASED-100 held 1.000 (constraints + relations + temporal, APB/AHB/AXI, fresh eval); `kg-bench`
156/156; `run_ci.sh` green (lib 1654, +2 tests).

The Class-C taxonomy is now closed for the unambiguous phantom subset; the broader PROSE-GROUNDED 0/0 set
stays honestly kept (no clean genuinely-declared discriminator — `[[agent-surface-defect-taxonomy]]`,
report §7.3). See `[[project_kg_isf_completeness]]`.
