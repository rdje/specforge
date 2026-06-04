---
id: stable-obligation-phase-scoped-residual
title: SpecForge stability obligations stay .isf residuals — phase-scoped, not FSMGen's unconditional (stable s)
answers:
  - "why doesn't SpecForge lower stability obligations to (assert (stable sig))"
  - "can SpecForge use FSMGen's (stable …) sampled-value predicate"
  - "are SignalStable obligations representable in .isf"
  - "why are stability obligations residuals"
date: 2026-06-04
status: current
tags: [isf, temporal, stable, fidelity, residual, no-build, fsmgen]
evidence: crates/specforge/src/ir/contract.rs (contract_from_temporal_rule, Obligation::Stable); subs/fsmgen/docs/knowledge/isf-sampled-value-predicates.md
reverify: grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs
---

**Do NOT lower SpecForge stability obligations to `(assert (stable s))` — it would over-assert.**
Every mined `SignalStable` / `ActorMaintainsSignalStable` predicate carries `from_phase` +
`to_phase`, lowered to `Obligation::Stable { during: Window::Between { tick_phase_events } }`
(`contract.rs` ~L432) — i.e. "`s` is stable **during** the interval `[from_phase, to_phase]`".
There is no unconditional or level-guarded stability variant in the model.

FSMGen's `(stable s)` (shipped in `ISF-PROPERTY-SAMPLED-VALUE`, `6700fbb4`) is `$stable(s)` —
**unconditional per-tick** stability, asserted every clock edge. That is *stronger* than
"stable during a phase interval", so emitting `(assert (stable s))` for a phase-scoped obligation
would assert more than the spec says (a fidelity bug). The faithful form
`(assert (=> g (stable s)))` needs a boolean `g` meaning "currently inside `[from_phase,
to_phase]`" — but tick-phases are abstract markers, not `.isf` interface signals, so no such
guard exists. Hence the existing **residual** ("bare stability across tick phases has no
supported `.isf` construct") is the correct, honest disposition (verified 2026-06-04 under
`FSMGEN-ASSERT-LOWERING.2`; FSMGen's suggestion to lower these assumed a boolean antecedent
SpecForge's phase model does not provide).

What CAN faithfully lower with the new primitives is the **antecedent→consequent** family where
the antecedent is a representable boolean — `(assert (=> A B))` / `(=> A (within B MIN MAX))`
(`min > 1` via `ISF-PROPERTY-WINDOW-RANGE`, `92d7036b`) — pursued in `FSMGEN-ASSERT-LOWERING.3`.
A faithful stable lowering would require SpecForge to first reduce a phase interval to a boolean
guard (e.g. a level-delimited "stable while <signal>"), which the current extractor does not
produce. See `[[fsmgen-temporal-isf-form]]`, `[[isf-temporal-lowering-no-silent-drop]]`.
