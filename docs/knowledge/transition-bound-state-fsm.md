---
id: transition-bound-state-fsm
title: A third agnostic FSM grammar — single ALL-CAPS `<NAME> state` bound by a transition/locative verb (SWP ACTIVATED/DEACTIVATED)
answers:
  - "how does specforge extract a single-word ALL-CAPS state machine (SWP ACTIVATED / DEACTIVATED / SUSPENDED)"
  - "what is extract_transition_bound_states / transition_bound_state_names_in / is_bare_state_name / named_state_ ids"
  - "how are the three FSM grammars (SWD hyphen, quoted-mode, single-word) distinguished"
  - "how is the single-word `<NAME> state` grammar kept false-positive-free without a keyword doc-gate (ADR 0006)"
  - "why does SWP yield 0 from extract_protocol_states and extract_quoted_mode_states but 4 from the transition-bound path"
date: 2026-06-09
tags: [swp, fsm, state-machine, serial, extraction, evidence-ir, pdf-variant-digestion, adr-0006]
evidence: docs/tasks/PDF-VARIANT-DIGESTION.md (.9.7); crates/specforge/src/ir/evidence.rs (extract_transition_bound_states, transition_bound_state_names_in, is_bare_state_name)
reverify: python3 -c "import json; e=json.load(open('generated/evidence_ir/etsi_ts_102613_v16_0_0_2021_10_smart_cards_uicc_clf_single_wire_protocol_swp/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])"
---

Protocols name their FSM states **three** structurally different ways, and SpecForge now has an extractor
for each (all grammar-only, ADR 0006 — never the literal names):

1. **SWD/JTAG shape** — an unquoted Capitalized-Hyphen identifier + the word *state* (`Shift-DR state`),
   behind a TAP/scan-chain doc-gate → `find_states_with_actions` / `extract_protocol_states`
   (`[[swd-protocol-fsm-surface]]`).
2. **Quoted-mode shape** — a single-quoted operational mode bound to a generic actor-noun (`a node is
   'error passive'`) → `extract_quoted_mode_states` (`[[agnostic-quoted-mode-fsm]]`).
3. **Single ALL-CAPS word shape** — `the DEACTIVATED state`, `into the ACTIVATED state`, `in the SUSPENDED
   state`. SWP (ETSI Single Wire Protocol) writes its interface FSM this way and never says "state machine"
   / "FSM" / draws it, so shapes #1 (needs a hyphen + TAP gate) and #2 (needs quotes + actor) both yield
   **0**. `.9.7` adds `extract_transition_bound_states` for this shape.

The cue that makes single-word matching safe — where "the security state" / "the cache state" / "the
current state" would otherwise flood it — is a **transition/locative binding**: a real state is one you
ENTER, LEAVE, or are IN. A candidate is taken only inside `<TRIGGER> [the|a|an] <NAME> state`, TRIGGER ∈
{enter(s)/into/leave(s)/exit(s)/to/in/from/reach(es)/remain(s)/stay(s)/move(s)/transition(s)/return(s)/
put(s)/place(s)}. `<NAME>` must be **ALL-CAPS** (≥2 chars, ≥1 letter, hyphens allowed; `is_bare_state_name`),
which excludes lowercase descriptions. Three guards finish it: an after-token guard drops `<X> state
machine|diagram` (names the machine, not a state); a denylist removes logic levels (`HIGH`/`LOW`/…) and the
universal architectural pseudo-values `UNKNOWN`/`UNPREDICTABLE`; and — exactly as `.9.3a` — each name must
**recur in ≥2 statements** and a doc must yield **≥2 distinct** states. That structural self-gate replaces a
keyword doc-gate, which was rejected precisely because SWP never uses the phrase "state machine". Records use
a distinct `named_state_NNNN` id so they never collide with `protocol_state_` / `swd_line_state_` /
`mode_state_`; the path is additive and deduped by name after the other three (the existing extractors are
byte-for-byte untouched → zero regression by construction).

Result, empirically locked by probing the grammar over all 80 persisted evidence docs **before** coding:
**SWP → 4 states** (`DEACTIVATED`/`ACTIVATED`/`SUSPENDED`/`HALT`, was 0). **SWD/ADI gains 0** `named_state_*`
(its only ≥2-support all-caps word is `UNKNOWN`, now denylisted, and 1 distinct < 2 anyway) → its 13-state
FSM eval stays `P=R=F1=1.000`. **CAN gains 0** here (keeps its 3 `.9.3a` quoted states). Genuine breadth win:
18 docs total gain a real FSM surface — including the parallel buses' own operating machines (APB
`SETUP`/`ACCESS`, AXI low-power `RUN`/`STOP`/`ACTIVATE`/`DEACTIVATE`), an honest correctness gain that
touches none of the scored constraint/relation/temporal surfaces. kg-bench 151/151; no `protocol_states`
fixture exists, so no benchmark lock to break.
