---
id: isf-lowering-fidelity-gauge
title: Which IntentIR surfaces reach the .isf vs are silently dropped (KG-ISF-COMPLETENESS.2 measurement) — the bar-#6 silent-drop is already honest for grounded wire intent; the 17k raw "drops" are ungroundable free-text / non-wire register fields (residualizing them = noise); the largest TRUE infidelity is signal direction/width (~98% defaulted output/width-1), which is design-gated (relationship-relative + R6-ISF-ADAPTER.4 deliberate default + owner steer)
answers:
  - "which IntentIR surfaces are lowered to the .isf vs silently dropped"
  - "is bar #6 (ISF round-trip, no silent drop) a large faithful-lowering gap"
  - "why are behaviors and constraints (22k each) not lowered to .isf — is that a gap"
  - "are temporal_invariants / conditional_rules / signal_constraints silently dropped at ISF lowering"
  - "why does the emitted .isf default ~98% of signals to output and width 1"
  - "where does the .isf signal direction/width come from (direction_hint/width_hint, not the actor graph)"
  - "why is signal direction hard to lower faithfully to a single .isf module"
  - "what is the largest faithful-lowering gap in the IntentIR -> .isf round trip"
  - "should specforge record a residual for every typed rule that does not lower to .isf"
date: 2026-06-17
tags: [kg-isf-completeness, isf, isf-adapter, lowering, fidelity, round-trip, signals, direction, width, measured, wire-based-100, adr-0006, honest-residual]
evidence: docs/research/isf-lowering-fidelity-measurement.md (full per-dimension gauge); crates/specforge/src/ir/isf_ir.rs (from_intent_ir:613 — signals from interfaces[].signal_records direction_hint:688/width_hint:692 defaulting output/1; typed-rule filters conditional_rules:1038 / signal_constraints:1058 / temporal_invariants:1075 each `continue`-skip with no residual); crates/specforge/src/ir/adapters.rs (derive_isf_actor_name:178 single flat module; R6-ISF-ADAPTER.4 deliberate direction/width default policy:202-217); docs/tasks/KG-ISF-COMPLETENESS.md (.2/.2a/.2b)
reverify: "Read-only python over generated/intent_ir/*/intent_ir.json: replicate signal_names (interfaces[].signal_records minus clock/reset) + the from_intent_ir filters → temporal_invariants 16463 total / 257 lower / 16179 empty-subject + 27 undeclared; conditional_rules 1634 / 498 lower / 1095 no-consequent + 41 undeclared; signal_constraints 444 / 361 lower / 83 undeclared (0 undeclared on all 4 wire docs). Direction: grep -cE '\\(output ' vs '\\(input ' generated/adapters/isf/ihi0022_l_*/agent.isf → 283 output / 4 input, all (width 1); signal_records direction_hint=None for 282/289 AXI signals while actor_ports carry both input+output per signal. Bar #6 silent-drop of grounded wire intent ≈ 0; the largest true infidelity is the deliberate direction/width default (R6-ISF-ADAPTER.4)."
---

**Measured `2026-06-17` (`KG-ISF-COMPLETENESS.2`, read-only, docs-only).** The measurement-first gauge of
which ISF-fidelity bar dimension carries the largest faithful-lowering gap, before any emitter change.
Report: `docs/research/isf-lowering-fidelity-measurement.md`. Serves the owner's `2026-06-16` north star
(`[[project_kg_isf_completeness]]`) — complete IntentIR → faithful ISF.

**What lowers.** `from_intent_ir` reads `interfaces` (→ signals), `symbol_definitions` (→
constants/types/enums), `register_records` (→ `(storage (var … reset))`), `transactions`,
`temporal_rules`/`actor_contracts` (→ asserts/stages/rules **or** `temporal_residuals`), `conditional_rules`
/ `signal_constraints` / `temporal_invariants` (→ `(rule)`), plus clock/reset. Temporal rules and register
resets already carry honest residuals (`[[isf-temporal-lowering-no-silent-drop]]`,
`[[register-reset-isf-emit]]`).

**The 22k `behaviors` / 22k `constraints` are NOT a gap.** They are free-text (`{id, statement: String}`)
legacy surfaces whose semantic content is already lowered via the typed twins (`signal_constraints` /
`temporal_rules` / `conditional_rules`). Re-lowering them would be the redundant second rendering
`KG-ISF-TRANSACTIONS.2b` correctly removed.

**The silent-drop of typed rules is real but not a clean gap.** The lowered surfaces are `continue`-skipped
per-element when the subject signal is empty / not declared, with no residual: temporal_invariants 16179
empty-subject (e.g. a table-of-contents heading classified as an invariant), conditional_rules 1095
no-consequent (legal boilerplate, vague "must"/"shall"), and 151 "undeclared-signal" cases that are
dominated by register/struct-field paths (`process_id[19:17]`, `DC.tc.SXL`, `DID`, `Reserved`) belonging to
the register / message-field surface, not wire rules. **0 undeclared drops on all four wire docs.**
Mass-residualizing the 17k would be dishonest noise (the "absence is not an event" rule) — so bar #6 is
**already honest** for grounded wire-relevant intent.

**The largest TRUE infidelity is signal direction/width.** The emitter reads only the legacy flat
`direction_hint`/`width_hint` (`None` for 85–98% of signals) and defaults to **output / width-1** — the AXI
`.isf` emits 283 `(output)` vs 4 `(input)`, all width 1. The canonical direction lives in the actor-relative
graph (`actor_ports`), not the flat hint, but it is **relationship-relative** (every signal is both an input
and an output across actors), the `.isf` is a **single flat module**, and the default is a **deliberate,
documented** choice (`R6-ISF-ADAPTER.4`: FSMGen schedules, so blocking on direction/width would over-restrict).
Overriding it is an **owner decision** in tension with the north star → spun `.2a` (deferred-with-trigger,
needs an FSMGen-contract check + reference-boundary design + owner steer). The buildable candidate is `.2b`
(an additive lowering-coverage visibility gauge mirroring `storage_reset_residual_packet`, byte-identical
`.isf` → WIRE-BASED-100 trivially held).

See `[[project_kg_isf_completeness]]`, `[[isf-temporal-lowering-no-silent-drop]]`,
`[[register-reset-isf-emit]]`, `[[feedback_isf_no_hacks]]`, `[[feedback_scoring_rigor]]`.
