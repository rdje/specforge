---
id: clarification-planner-structural-policy
title: Clarification planning groups exact governed needs and withholds autonomous work
answers:
  - "Where is the deterministic clarification planner implemented?"
  - "How does SpecForge deduplicate equivalent clarification questions?"
  - "How does the clarification planner avoid asking about autonomously recoverable information?"
  - "How are blocking and advisory clarification packets separated?"
  - "What determines clarification information-gain ordering?"
  - "What happens when clarification dependencies form a cycle?"
date: 2026-08-15
status: current
tags: [clarification, planner, residuals, autonomy, dependency-order]
evidence: crates/specforge/src/ir/clarification.rs; docs/tasks/SPEC-CLARIFICATION-LOOP.md; docs/book/src/pipeline/clarification-loop.md
reverify: cargo test -p specforge-core clarification --offline && cargo clippy -p specforge-core --all-targets --offline -- -D warnings
---

`SPEC-CLARIFICATION-LOOP.2` implements `plan_clarifications` in
`specforge_core::ir::clarification`. Governed producers normalize residual decisions, contradictions,
completeness/validation findings, adapter blocks, and external choices into `ClarificationNeed`. The planner
does not parse diagnostic prose or infer semantic equivalence: grouping requires the exact same producer-owned
equivalence key, question revision, missing-information reason, and answer-schema digest.

A need with a governed `AutonomousActionAvailable` is returned as executable work, not a question. Its transitive
dependents are withheld until the action executes and the caller replans. Eligible needs union evidence and
alternatives deterministically; impacts with the same stage/surface/stable-record identity merge and blocking is
the conservative OR.

Dependency-connected questions remain in one class. A component containing any blocking impact becomes part of
the blocking packet; other components become advisory. Each class emits at most one packet, topologically ordered
with cycles rejected. Ready questions rank by a structural 0–100 information-gain score derived from blocking
reach, distinct affected surfaces, known alternatives, and whole-pipeline blockage, with priority and opaque key
as stable tie-breakers. Planning grants no answer or proof authority.
