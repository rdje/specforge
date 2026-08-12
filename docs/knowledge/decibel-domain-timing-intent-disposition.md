---
id: decibel-domain-timing-intent-disposition
title: Decibel-domain timing rows remain source evidence but are non-applicable to executable digital intent
answers:
  - "how does SpecForge prevent analog dB limits from becoming digital timing intent"
  - "what is TimingIntentDisposition"
  - "does SpecForge delete non-applicable physical timing records"
  - "which units mark a timing record as decibel domain"
  - "why are some retained CCIX decibel timing records still canonical"
  - "what retained chains changed in SPEC-TO-INTENT-ALIGNMENT.6d.i"
date: 2026-08-12
status: current
tags: [timing, physical-link, non-applicable, evidence-ir, semantic-ir, intent-ir]
evidence: crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6d.i)
reverify: "cargo test -p specforge --lib decibel_domain_rows_are_non_applicable_without_suppressing_digital_timing && cargo test -p specforge --lib non_applicable_timing_observation_is_carried_but_not_executed && bash scripts/check_chain_currency.sh"
---

`TimingConstraintRecord.intent_disposition` is a backward-compatible, schema-closed carrier. Canonical is the
omitted/default value. A non-applicable record names a typed quantity domain, reason, first promotion boundary,
and replay route while retaining the original scalar values and direct source-table authority.

EvidenceIR currently assigns `decibel` only when the first alphanumeric token of the explicit unit is `dB` or
`dBc`, case-insensitively. That includes `dB_RMS`, `dB RMS`, and `dBc/Hz`. A parameter name never classifies the
row: `IL(settle)` with `ns` remains canonical. SemanticIR carries every record but derives temporal rules only
from canonical records; IntentIR carries the complete record unchanged. Validation uses the same eligibility
boundary, so a physical-only document is not told that an executable temporal rule is missing.

ADR 0025 measured exactly two replayable stale EvidenceIR chains. The two named conformance cascades retain all 115 timing
records; 26 become non-applicable, all six promoted artifacts are otherwise disposition-neutral, and both
adapters are byte-identical. That slice restored the then-current 24/24 EvidenceIR and 78/78 downstream census.
The schema-3 proof migration now reports only the 24 verifiable chains as current and all legacy/proofless
downstream replay as unmeasurable. Three older named conformance chains contain 48 matching legacy records but
have no normalized bundle, so current code cannot recompute their EvidenceIR. They remain an explicit
refresh-owned replayability boundary, not evidence that classification is document-specific.
