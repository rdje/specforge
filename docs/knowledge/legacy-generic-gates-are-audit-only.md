---
id: legacy-generic-gates-are-audit-only
title: Legacy generic whole-statement gates are schema-compatible audit data only
answers:
  - "why does a new SemanticIR have an empty gates array"
  - "do old SemanticIR gates create IntentIR behaviors or constraints"
  - "does GateRecord remain schema compatible"
  - "why is a sentence containing when not necessarily a semantic gate"
  - "how many synthetic behaviors came from generic gates"
  - "does retiring gates remove conditional rules or temporal rules"
  - "where was build_gates removed"
  - "does the ISF adapter lower generic gate behaviors"
date: 2026-08-10
status: current
tags: [semantic-ir, intent-ir, gates, compatibility, conditional-rules, corpus-coverage]
evidence: docs/research/generic-gate-authority-retirement-measurement.md; docs/tasks/CORPUS-COVERAGE.md (.2.43b); crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; docs/book/src/pipeline/semanticir.md; docs/book/src/pipeline/intentir.md
reverify: "cargo test -p specforge legacy_generic_gates_load_but_do_not_authorize_intent --lib --quiet && cargo test -p specforge builds_semantic_ir_from_handshake_evidence --lib --quiet"
---

Current SemanticIR producers serialize the legacy `gates[]` field empty, and current IntentIR builders ignore
populated records from old artifacts. `GateRecord` remains loadable and round-trippable so historical provenance
is auditable; it no longer authorizes behavior or interface-coupled constraints.

The retired producer copied one complete source sentence whenever it contained `if`, `when`, `unless`, `while`,
`after`, `before`, or `until`. The record had no parsed antecedent, consequent, effect, actor role, signal/value
operands, or executable operation. Across 79 retained artifacts, that path created 21,206 behaviors / 642,401
actor assignments and 5,974 constraints / 366,087 interface assignments with zero rendered-ISF, renderability,
status, or executable-count value.

Conditional meaning remains in the source evidence, stronger invariant/contract/constraint surfaces, and
independently typed `conditional_rules[]` / `temporal_rules[]`. Those record families are not aliases for the
retired whole-sentence collection and remain subject to their own precision contracts.
