---
id: isf-temporal-lowering-no-silent-drop
title: temporal_rules are never silently dropped in the IntentIR->.isf lowering (already guaranteed)
answers:
  - "are temporal rules silently dropped when lowering IntentIR to .isf"
  - "does every temporal_rule reach the .isf or a residual"
  - "is there a lowering-completeness check for temporal rules"
  - "where does .isf record dropped temporal obligations"
  - "should I build an isf lowering-completeness verifier for temporal rules"
date: 2026-06-04
tags: [isf, temporal, lowering, residual-honesty, no-build]
evidence: crates/specforge/src/ir/isf_ir.rs; crates/specforge/src/ir/adapters.rs
reverify: grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs
---

**Already guaranteed — no new verifier needed.** The `IntentIR → .isf` lowering classifies
**each** `temporal_rule` into exactly one disposition (a `(contract …)`, a `(rule temporal_…)`,
or a `ResidualDecisionPacket`) — `ISF-TEMPORAL-LOWERING.2.2/.2.3` in `ir/isf_ir.rs` (~L885).
Un-lowerable obligations become `temporal_residuals()`, which `adapters.rs` folds into the
artifact's `residual_decisions` (~L322). The end-to-end test
`isf_temporal_rules_reach_isf_end_to_end` (`adapters.rs` ~L611) asserts a produced
`temporal_rule` appears as a contract, a temporal rule, **or** a residual — i.e. it is never
silently dropped.

So a separate "`.isf` temporal lowering-completeness verifier" is a **NO-BUILD** (the invariant
and its test already exist; investigated 2026-06-04). A general no-silent-drop verifier for
*other* IntentIR constructs would be a separate, larger investigation (not yet done). Related:
`ISF-RULE-CONFLICT-RESIDUAL` (rule conflicts → residual), `[[temporal-rule-ltl-rendering]]`.
