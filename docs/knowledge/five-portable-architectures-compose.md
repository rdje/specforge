---
id: five-portable-architectures-compose
title: SpecForge's five portable architectures compose ownership, continuity, retrieval, enforcement, and claim evidence
answers:
  - "what are SpecForge's five portable architectures"
  - "how do task trees memory the Knowledge Map doctrine enforcement and claim verification fit together"
  - "which architecture owns work and which one verifies published claims"
  - "does claim verification replace task trees or doctrine enforcement"
  - "how does a future SpecForge session retrieve and re-run a current claim"
date: 2026-08-15
status: current
tags: [architecture, task-tree, memory, knowledge-map, doctrine-enforcement, claim-verification, continuity]
evidence: docs/decisions/0003-task-tree-and-commit-doctrine.md; MEMORY_ARCHITECTURE.md; knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md; docs/decisions/0006-doctrine-enforcement-architecture.md; CLAIM_VERIFICATION.md; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md
reverify: bash scripts/check_memory_architecture.sh && bash knowledge-map/scripts/check_knowledge_map.sh && bash scripts/check_doctrines.sh && perl scripts/check_claim_verification.pl --report
---

SpecForge has five complementary portable architectures. Task trees own the bounded work and its acceptance
frontier. Durable memory makes the current frontier resumable after session loss. The Knowledge Map routes a
question to the canonical decision, fact, task, or source without archaeology. Doctrine enforcement composes
repository rules into one mechanically gated driver. Claim verification decides whether a current actionable
assertion has source re-derivation, a dimensionally different known-bad falsification control, and tracked
stale-detecting durability before that driver preserves it.

No layer substitutes for another. A claim record cannot authorize unowned work; a task checklist cannot make a
measurement independently true; a Knowledge Map card cannot enforce its own currency; and a green doctrine gate
cannot rescue a claim whose competing hypothesis was never challenged. Their join is the continuity path: resume
from `MEMORY.md`, open the owning task, retrieve established facts through `KNOWLEDGE_MAP.md`, rerun the named
claim evidence, and let `scripts/check_doctrines.sh` reject drift across the composed repository state.
