---
id: trajectory-controller-engine
title: The trajectory engine keeps nine exact dimensions and can only propose task-tree-owned work
answers:
  - "where is the SpecForge trajectory controller engine"
  - "how does the trajectory controller classify converging diverging stalled mixed and unmeasurable"
  - "how does automatic task ranking keep hard failures ahead of breadth work"
  - "can the trajectory controller use one weighted score"
  - "can an estimated point claim a trend without paired uncertainty history"
  - "how does the controller prove a proposed task is task-tree owned"
  - "can the trajectory controller mutate canonical semantic artifacts"
date: 2026-08-12
status: current
tags: [spec-to-intent-alignment, trajectory, multi-metric, state-classification, task-ranking, review-authority]
evidence: crates/specforge/src/ir/trajectory.rs; doctrine/trajectory_controller_input_schema.json; docs/decisions/0034-trajectory-steering-is-a-reviewable-multimetric-control-loop.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.5a)
reverify: "cargo test -p specforge --lib ir::trajectory && cargo clippy -p specforge --all-targets -- -D warnings"
---

`SPEC-TO-INTENT-ALIGNMENT.5a` implements a strict version-1 input and deterministic report engine for all nine
ADR-0034 dimensions: source capture, semantic correctness, semantic completeness, stage conservation,
provenance/honesty, production participation, generalization/robustness, operational confidence, and executable
readiness. Every metric retains a bounded rational numerator/denominator, target operator, improvement
direction, material-change threshold, hard/required flags, oracle, population, uncertainty, and repository-
relative evidence. There is no authoritative blended score.

Current hard-gate or hard-metric violations classify `diverging` even without history. Otherwise exact paired
changes distinguish `converging`, `diverging`, and `mixed`; unchanged deficits become `stalled` only after the
declared comparable-snapshot window. A missing required oracle or history yields `unmeasurable`. An estimated
point cannot drive a trend while paired historical uncertainty is absent; the engine reports the missing
comparison instead of pretending it is exact.

Gaps are ranked lexicographically by hard-invariant/source-loss/semantic-regression/oracle/residual/breadth tier,
then affected population, causal confidence, reversible slice size, uncertainty, and stable id. Each gap must
name an existing task ID declaration in a repository-relative task tree before evaluation can emit the proposal.
The authority object is fixed to `report_only`, rejects canonical semantic mutation, and requires task-tree
review. [[trajectory-steering-is-a-reviewable-multimetric-control-loop]] remains the decision authority.

Sixteen focused tests cover all five states, seeded fabrication, dimension omission, missing history/oracle,
estimated-uncertainty refusal, hard-first ranking, owner reachability, absolute paths, unknown fields, stable
serialization, public-schema parity, and the semantic-mutation boundary. `.5a` deliberately contains no `.4c`
product values; `.5b` owns composition and the first real task proposal.
