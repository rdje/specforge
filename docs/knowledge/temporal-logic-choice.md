---
id: temporal-logic-choice
title: SpecForge captures temporal behavior in LTL/MTL, not CTL or TLA+
answers:
  - "does SpecForge use LTL CTL or TLA+"
  - "why not CTL for temporal behavior"
  - "why doesn't SpecForge use TLA+"
  - "what temporal logic backs temporal_rules"
  - "does SpecForge model-check temporal properties"
date: 2026-06-04
tags: [temporal, ltl, mtl, formalism, decision]
evidence: docs/decisions/0005-temporal-logic-ltl-mtl-not-ctl-tla.md; crates/specforge/src/ir/temporal_ltl.rs
reverify: grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs
---

**LTL / MTL — yes, in use.** SpecForge's `temporal_rules` *are* the linear-time
`G(antecedent → consequent)` template (Pnueli, FOCS 1977); the next-tick delay is LTL `X`, a
`cycle_window` is the Metric Temporal Logic bounded-eventually `F[min,max]`; rendered by
`ir/temporal_ltl.rs`. Grounded in Pnueli/GoldMine/Texada (all linear-time).

**CTL — no, deliberately.** CTL is branching-time (quantifies over all/some future paths,
`A`/`E`); a spec states a single intended *linear* behavior over the clock ticks, so
linear-time LTL is the fit, not branching CTL.

**TLA+ — no.** TLA+ (Lamport) is a full specification + model-checking *environment* for
authoring/verifying designs — orthogonal to mining intent *from* a document. SpecForge does
not author or check TLA+. It (or PSL/SVA) could be a future **export target** so mined
properties become checkable downstream, but that is not planned.

SpecForge **mines** temporal properties; it does **not** model-check them (that is the
downstream verifier's job — FSMGen/sim — and is out of scope). Full ADR:
`docs/decisions/0005`. See `[[temporal-rule-ltl-rendering]]`, `[[spec-mining-framing]]`.
