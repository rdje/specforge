---
id: spec-clarification-loop-direction
title: SpecForge is autonomous first and asks typed questions at the evidence boundary
answers:
  - "How will SpecForge proceed when a PDF lacks information needed for complete ISF?"
  - "What must a SpecForge clarification packet contain?"
  - "Can a user answer directly authorize canonical intent?"
  - "How will SpecForge resume after the user answers a clarification?"
  - "How does SpecForge minimize clarification round trips?"
  - "Is the autonomous-first clarification loop implemented yet?"
date: 2026-08-15
status: current
tags: [clarification, residuals, user-assistance, proof, isf]
evidence: docs/tasks/SPEC-CLARIFICATION-LOOP.md; ROADMAP.md; docs/book/src/architecture-rationale.md
reverify: perl scripts/check_task_tree_catalog.pl && bash knowledge-map/scripts/check_knowledge_map.sh
---

`SPEC-CLARIFICATION-LOOP` owns the planned boundary. SpecForge should finish every safely decidable pipeline
branch autonomously. When required information is absent, contradictory, ambiguous, extraction-limited, or an
external design choice, it should emit a typed clarification packet rather than fabricate a default.

The packet names the current source/artifact/revision/stage and exact source/proof context; the unresolved
engineering proposition, alternatives, and reason automation stopped; blocked downstream facts, validation,
adapter, or ISF decisions; priority, expected information gain, and dependencies; accepted answer schema,
units/constraints and validation; lifecycle/currentness; and a deterministic resume plan. Equivalent questions
are grouped and dependency-ordered, while unaffected work continues autonomously.

A user answer is provenance-bearing evidence, not automatic canonical truth. It must pass typed parsing,
authority, grounding, currentness, consistency, and conflict checks. Accepted answers replay only the affected
proof dependency closure; stale, incomplete, contradictory, unknown, unavailable, not-applicable, or deferred
answers remain explicit and resumable. The task tree is active, but the current runtime does not yet implement
this interaction loop; `.1` begins with the versioned IR and proof-authority architecture.
