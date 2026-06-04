---
id: temporal-rule-ltl-rendering
title: Temporal rules render to standard LTL/MTL via ir/temporal_ltl.rs
answers:
  - "how are temporal rules expressed as LTL or MTL"
  - "where is the LTL renderer for temporal rules"
  - "what is the LTL form of a temporal_rule"
  - "is there a PSL or SVA export of temporal rules"
  - "temporal rule predicate atom vocabulary"
date: 2026-06-04
tags: [temporal, ltl, grounding, verification]
evidence: crates/specforge/src/ir/temporal_ltl.rs; docs/book/src/domain/temporal-semantics.md
reverify: grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs
---

`crate::ir::temporal_ltl::temporal_rule_to_ltl(&TemporalRuleRecord) -> String` renders a mined
temporal rule in standard LTL/MTL notation: `G( ante -> X cons )` (no cycle window),
`G( ante -> F[min,max] cons )` (cycle window), or `G( cons )` (empty-antecedent invariant).
Atom vocabulary: `sig==VAL`, `drive(actor,sig)`, `stable([actor,]sig)`, `sample([actor,]sig)`,
`handshake(valid,ready)`. It is **pure + derived** (never persisted to the IR → no `kg-bench`
fixture churn) and grounds `temporal_rules` in the spec-mining `G(antecedent → consequent)`
template (Pnueli / GoldMine / Texada).

There is **no PSL/SVA export yet** — that touches the FSMGen handoff contract and is an
explicit separate downstream tree; this renderer is the LTL foundation it will consume.
Canonical homes: the renderer module + the "Standard LTL/MTL notation" section in
`docs/book/src/domain/temporal-semantics.md`. See `docs/tasks/TEMPORAL-RULE-LTL-RENDER.md`.
