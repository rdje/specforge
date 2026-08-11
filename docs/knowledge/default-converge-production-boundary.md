---
id: default-converge-production-boundary
title: Converge emits a guarded per-run ledger for every production command and every capability island
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
  - "how does converge report integrated scheduled and omitted production capabilities"
  - "will a new standalone extraction command fail if converge does not account for it"
date: 2026-08-11
status: current
tags: [converge, orchestration, extraction, intentir, nli, contracts, relations, register-bits]
evidence: crates/specforge/src/commands/converge.rs (production_capability_report and Clap-surface partition tests); crates/specforge/src/lib.rs; crates/specforge/src/commands/rescan_plan.rs; docs/book/src/commands/pipeline.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.2)
reverify: "cargo test -p specforge --lib commands::converge::tests::production_capability_registry_partitions_cli_and_accounts_for_every_producer && cargo test -p specforge --lib commands::converge::tests::provider_free_capability_report_names_every_current_capability_island"
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

Every successful run now emits a 17-row JSON ledger covering all 16 commands classified as production
capabilities. Each row separates stable participation (`integrated`, `scheduled`, or `omitted`) from the
per-run state (`executed`, `inspected_only`, or `not_executed`) and names the exact command/entrypoint plus a
reason. Provider-backed integrated stages therefore remain visible when disabled, while the four capability
islands above are explicit omissions rather than implied delivery. Standalone condition repair is reported
covered only when the integrated LLM-primary replacement actually ran.

A test derives all subcommand names from Clap, compares them with a complete 28-command role partition, and
requires all 16 production commands to occur in the ledger. Adding a subcommand without classification, or
classifying a production command without a ledger row, fails. `converge` remains the default orchestration path,
but its exact end-to-end boundary is now guarded and machine-readable (`SPEC-TO-INTENT-ALIGNMENT.2`).
