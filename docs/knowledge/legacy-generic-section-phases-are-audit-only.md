---
id: legacy-generic-section-phases-are-audit-only
title: Legacy generic section phases are schema-compatible audit data only
answers:
  - "why does a new SemanticIR have an empty phases array"
  - "do old SemanticIR phases create IntentIR behaviors"
  - "does PhaseRecord remain schema compatible"
  - "why is Reset value not a semantic phase"
  - "why did retiring generic phases remove pure inferred actors"
  - "how many synthetic behaviors came from generic section phases"
  - "what is the difference between phases and transaction_phases"
  - "where was build_phases removed"
date: 2026-08-10
status: current
supersedes: semantic-section-phases-require-heading-authority
tags: [semantic-ir, intent-ir, phases, compatibility, corpus-coverage]
evidence: docs/research/generic-section-phase-retirement-measurement.md; docs/tasks/CORPUS-COVERAGE.md (.2.43a.i); crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; docs/book/src/pipeline/semanticir.md
reverify: "cargo test -p specforge generic_phase --lib --quiet && cargo test -p specforge generic_section_phase --lib --quiet"
---

Current SemanticIR producers serialize the legacy `phases[]` field empty, and current IntentIR builders ignore
populated records from old artifacts. `PhaseRecord` remains loadable and round-trippable so historical provenance
is auditable; it no longer authorizes behavior or actor responsibilities.

The retired producer mapped one section topic to one tautological summary without a typed name, order, guard,
signal set, actor role, or effect. Across 79 retained artifacts, its surviving title path created 2,238 synthetic
behaviors and preserved 48 pure-inferred actors with no adapter renderability, status, or executable-count value.
Typed protocol phase recognition remains in `transaction_phases[]`; see
[[transaction-capture-census]] and the canonical mdBook chapters linked by the evidence field.
