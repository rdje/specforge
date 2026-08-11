---
id: default-converge-production-boundary
title: The ordinary converge path is not the union of all production extraction commands
answers:
  - "which extraction commands does converge run directly"
  - "does converge automatically run extract-contracts"
  - "does converge automatically run signal-resolve"
  - "does converge automatically run recover-register-bits"
  - "does converge apply the IntentIR NLI demotion gate"
  - "is converge the complete production capability path"
  - "why can a shipped standalone extractor fail to improve the default end-to-end result"
  - "what does converge do after stabilization"
  - "does the converge NLI pass measure quality or demote unsupported intent"
date: 2026-08-11
status: current
tags: [converge, orchestration, extraction, intentir, nli, contracts, relations, register-bits]
evidence: crates/specforge/src/commands/converge.rs; crates/specforge/src/lib.rs; crates/specforge/src/commands/rescan_plan.rs; docs/book/src/commands/pipeline.md; docs/book/src/commands/quality-and-learning.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md
reverify: "rg -n 'use crate::commands|enrich::run|nlp_enrich::run|promote_constraints|measure_and_persist_gauge|extract_contracts|signal_resolve|recover_register_bits' crates/specforge/src/commands/converge.rs; then compare command dispatch in crates/specforge/src/lib.rs and the nlp-enrich-only rescan allowlist in crates/specforge/src/commands/rescan_plan.rs"
---

The ordinary `converge` pass directly runs visual enrichment when enabled, builds EvidenceIR, runs
`nlp-enrich` when enabled, builds SemanticIR and IntentIR, lowers the adapter, and repeats until its exact
knowledge snapshot stabilizes. After stabilization, a live-NLP run promotes the LLM-primary constraint
surface, rebuilds downstream stages once, and records an NLI extraction-quality **measurement**. An optional
rescan plan can replay its explicitly allowlisted `nlp-enrich` hints.

The direct path does **not** invoke the standalone `extract-contracts`, `signal-resolve`, or
`recover-register-bits` commands. It also does not call the `intent --nli-verify` construction path that
demotes not-entailed IntentIR values; its NLI call records a gauge on EvidenceIR. Those capabilities are real
and command-dispatched in `lib.rs`, but ordinary convergence does not compose them.

Therefore `converge` is the default orchestration path, not evidence that every shipped extraction capability
participates in the default result. A roadmap capability must be classified as integrated, deliberately
scheduled, or explicitly omitted before it can count as end-to-end product capability. This distinction is
owned by `SPEC-TO-INTENT-ALIGNMENT.2`.
