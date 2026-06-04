# 0005 — Temporal behavior is captured in LTL/MTL, not CTL or TLA+

- Date: 2026-06-04
- Status: accepted
- Tags: temporal, ltl, mtl, formalism, scope

## Context

SpecForge captures timing/protocol obligations as typed `temporal_rules`. A natural question
(raised 2026-06-04): which temporal formalism backs that — LTL, CTL, or TLA+? This records the
decision so it is not re-litigated.

## Decision

1. **LTL / MTL — yes, in use.** SpecForge's `temporal_rules` *are* the linear-time-logic
   `G(antecedent → consequent)` template (Pnueli, *The Temporal Logic of Programs*, FOCS 1977),
   the same shape the specification-mining literature (GoldMine, Texada) uses. The "next tick"
   delay is LTL's `X`; a bounded `cycle_window` is the **Metric Temporal Logic** bounded-
   eventually `F[min,max]`. `crate::ir::temporal_ltl::temporal_rule_to_ltl` renders exactly
   this. So LTL/MTL is the backing formalism today.
2. **CTL — no, deliberately.** CTL is *branching*-time: it quantifies over *all* (`A`) or
   *some* (`E`) future paths. A specification states a single intended *linear* behavior over
   the clock-tick sequence ("when X holds, on the next tick Y must hold"), which is a
   linear-time property; branching path-quantifiers do not match what spec prose asserts, and
   the literature SpecForge grounds on is all linear-time. No plan to adopt CTL.
3. **TLA+ — no.** TLA+ (Lamport) is a *full specification + model-checking language and
   environment* for authoring and verifying system/algorithm designs. That is orthogonal to
   SpecForge's job, which is to **mine** intent *from* a human-authored document. SpecForge
   does not author or model-check TLA+.

## Scope boundary

- SpecForge **mines** temporal properties; it does **not** model-check or prove them.
  Verification is the downstream consumer's job (FSMGen / simulation). Full LTL **model
  checking** is explicitly out of scope (see `docs/research/grounding/adopt-defer-ledger.md`,
  the Pnueli entry: take the operators + the `G(ante→cons)` shape, leave out model checking).
- TLA+ (or PSL/SVA) could *conceivably* become a future **export target** — so mined
  properties become checkable downstream — analogous to the flagged `.isf`→PSL/SVA export.
  That is not planned; the LTL renderer (`ir/temporal_ltl.rs`) is the foundation any such
  export would build on.

## Consequences

- The temporal model stays **linear-time**, restricted to the
  `G(antecedent → X / F[min,max] consequent)` template — tractable, grounded in what the spec
  states, renderable to LTL/MTL today and (future, deferred) to PSL/SVA for downstream checking.
- A future "export to a checkable formalism (PSL/SVA, possibly TLA+)" is a separate,
  downstream tree, gated on the FSMGen handoff contract.

## Links

- Task-trees: `TEMPORAL-RULE-LTL-RENDER`, `SPEC-MINING-PROVENANCE`, `TEMPORAL-RULE-EVAL`.
- KM: `temporal-logic-choice`, `temporal-rule-ltl-rendering`, `spec-mining-framing`.
- Grounding: `docs/research/grounding/protocol-temporal-semantics.md`, `adopt-defer-ledger.md`.
