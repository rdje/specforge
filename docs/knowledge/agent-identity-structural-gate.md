---
id: agent-identity-structural-gate
title: The .1a structural agent-identity gate rejects relation subjects whose first content token is a universal function word or leading verb (no name list, ADR 0006); determiners and the colliding pronouns i/its are deliberately excluded
answers:
  - "how does specforge reject prose-fragment non-agents like For / Then it / is recommended / ensures"
  - "what is the KG-ISF-COMPLETENESS.1a agent-identity / actor precision gate"
  - "where is the structural gate that drops function-word-led and verb-led actor candidates"
  - "why are determiners (All Managers) NOT rejected by the .1a agent gate"
  - "why do i / its stay out of the non-actor function-word list (GIC ITS, the letter I)"
  - "what are NON_ACTOR_LEADING_FUNCTION_WORDS and NON_ACTOR_LEADING_VERBS for"
  - "does the agent-identity gate keep Class-B fragments like Subordinate extends"
  - "how was the agent-identity gate proven not to drop real agents (>=8-port proxy, WIRE-BASED-100)"
date: 2026-06-16
tags: [kg-isf-completeness, actors, agent-surface, actor-signal-relations, precision, structural-gate, adr-0006, measured, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (is_non_actor_phrase_fragment + NON_ACTOR_LEADING_FUNCTION_WORDS + NON_ACTOR_LEADING_VERBS + first_content_token_lower, wired into normalize_relation_actor_name — the DRY seam for both prose paths + the table relation path); docs/research/agent-surface-fidelity-measurement.md; docs/tasks/KG-ISF-COMPLETENESS.md (.1a)
reverify: "cargo test -p specforge --lib actor_identity_gate (3 tests). Live: build fresh AXI evidence->semantic->intent (Pattern, --dry-run) and confirm For/with write/Then it/is permitted are gone while Manager(169 ports)/Subordinate(168)/interconnect(5) are unchanged. WIRE-BASED-100: eval-extraction on seed_{axi,apb,ahb,swd}{,_temporal}.json --provider skip --evidence-root <fresh-Pattern root> stays 1.000 on constraints(APB/AHB/AXI)+relations(x4)+temporal(x3); kg-bench 156/156."
---

**Landed `2026-06-16` (`KG-ISF-COMPLETENESS.1a`).** The first precision fix for the agent surface
(`[[agent-surface-defect-taxonomy]]` Class A — Junk). It rejects a relation-subject candidate that is a
phrase FRAGMENT rather than a protocol agent, by a purely **structural** rule on the FIRST content token
— universal English grammar, never a chip-spec name list (ADR 0006; `[[feedback_avoid_denylists_prefer_structural]]`).

**Where.** `is_non_actor_phrase_fragment(value)` in `ir/evidence.rs`, called inside
`normalize_relation_actor_name` (the ONE DRY seam both prose paths — `extract_subject_phrase` /
`extract_actor_phrase` — and the table relation path funnel through). One edit gates all relation paths.
The first content token = first maximal run of ASCII letters, lowercased (case is a soft cue —
`[[feedback_case_is_soft_not_critical]]`).

**The rule.** Reject iff the first token is in `NON_ACTOR_LEADING_FUNCTION_WORDS` (prepositions /
conjunctions / auxiliaries-copulas-modals / non-colliding pronouns / leading adverbs) OR in
`NON_ACTOR_LEADING_VERBS` (universal verbs that, leading, signal a verb-phrase fragment —
`ensures`/`exit`/`extends`/…). **Verb-LED only, NOT "contains a verb"** — so a Class-B fragment like
`Subordinate extends` keeps its leading NOUN and survives for `.1b` consolidation.

**Two measurement-driven exclusions (do NOT re-add them):**
- **Articles / determiners / demonstratives** (`the`/`a`/`an`/`this`/`that`/`these`/`those`/`each`/
  `every`/`any`/`all`/`both`/`some`/`no`/…) are NOT rejected: a determiner can precede a REAL agent
  (`All Managers`, `Any Manager`) → that is a `.1b` determiner-strip + consolidate, never a `.1a` drop.
- **`i` and `its`** are NOT in the list: lowercased they collide with the letter/Roman-numeral `'I'`
  (a 16-port mis-extraction) and the **GIC `ITS` agent** (Interrupt Translation Service, an 8-port
  real-ish entity). Case can't disambiguate them, so they are dropped from the reject set.

**Proven (measurement-first, before and after code).** Across all 36 persisted IntentIR docs the refined
gate rejects **ZERO** ≥8-port actors (the real-agent proxy; 23 high-port actors preserved), catches all
9 designed targets, and the 63 rejects are all non-agents. Live AXI rebuild: actors 24→21, junk gone,
real agents preserved at identical port counts, descriptor-noun precision (`monitor`/`Note`/`exclusive`/
`instruction`/`Shareable` — noun/adjective-led, neither function word nor leading verb) **deferred** to a
later leaf. WIRE-BASED-100 held at 1.000 (constraints APB/AHB/AXI, relations ×4 docs, temporal ×3);
`kg-bench` 156/156; `run_ci.sh` green.

**What it does NOT do (future leaves).** It does not consolidate Class-B fragments or strip leading
determiners (`.1b`), and it does not touch the descriptor-noun precision class (`HPROT bit`/`TREADY
input`/`section`/`Note`), which needs a contains-declared-signal sub-gate + furniture-noun handling.
