---
id: proof-bearing-chain-validation-order
title: Proof-bearing chains must validate each stage before building its consumer
answers:
  - "In what order must a current proof-bearing chain be rebuilt and validated?"
  - "Why does validating EvidenceIR after building SemanticIR make SemanticIR stale?"
  - "Is validation backannotation part of the cumulative proof prefix?"
date: 2026-08-14
status: current
tags: [proof, validation, chain-currency, pipeline]
evidence: crates/specforge/src/commands/validate.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/ir/adapters.rs; scripts/check_chain_currency.sh; docs/decisions/0025-persisted-chain-currency-is-measured-not-assumed.md; docs/tasks/spec-to-intent-alignment/behavioral-qualification.md
reverify: "scripts/check_chain_currency.sh"
---

A current proof-bearing chain is reconstructed causally: build and validate a stage before constructing its
downstream consumer. Validation backannotation is verified artifact state, not a detachable report. Its claims
become part of the exact cumulative upstream proof prefix retained by the next stage.

Consequently, building SemanticIR from a newly rebuilt EvidenceIR and only then validating that EvidenceIR
correctly makes SemanticIR stale: the upstream ledger gained its authorized validation state after the consumer
copied the prefix. The same rule applies at every boundary through the adapter. The remedy is ordered rebuild and
validation, never a proof bypass or a relaxed prefix comparison.

`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a` exercised this boundary while reconciling two alpha-ordering changes.
After ordered rebuild, the exhaustive gate reported 24/24 current and zero stale at EvidenceIR, SemanticIR,
IntentIR, and adapter; all record and proof-claim counts remained stable.
