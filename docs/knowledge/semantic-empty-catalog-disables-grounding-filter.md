---
id: semantic-empty-catalog-disables-grounding-filter
title: SemanticIR skips its grounding filter exactly when a document declares no signals
answers:
  - "why did removing false signals ADD conditional rules to SemanticIR"
  - "why does a document with no declared signals carry more ungrounded rules than one with signals"
  - "what does the declared_signal_names.is_empty() branch in semantic.rs do"
  - "why is consequent_signal NOTICE or PDF or IMPLEMENTATION or MUST in SemanticIR"
  - "how many documents carry unfiltered conditional rules and signal constraints"
  - "do unfiltered SemanticIR conditional rules reach an emitted .isf"
  - "why were PWR and OPEN promoted as conditional-rule consequent signals"
  - "which tree owns the empty-catalog grounding filter defect"
date: 2026-08-11
status: current
tags: [semantic-ir, grounding, signal-authority, conditional-rules, signal-constraints, corpus-coverage, adr-0006]
evidence: crates/specforge/src/ir/semantic.rs:276-308; docs/tasks/SEMANTIC-EMPTY-CATALOG-FILTER.md; docs/tasks/corpus-coverage/refreshes-51-56.md (`CORPUS-COVERAGE.2.52`); generated/semantic_ir/opencapi_25gbps_phy_mechanical_spec_v10
reverify: "Replay the semantic stage from a stale pre-refresh EvidenceIR that still declares signals and from the refreshed one that does not; the declaring input yields zero conditional rules and the non-declaring input yields two. Then census every generated/semantic_ir/*/semantic_ir.json for an empty non-low-confidence declared-signal set carrying conditional_rules or signal_constraints; expect 33 empty-catalog documents of 78, 29 of them carrying 1,423 rules and 100 constraints, and zero of those documents holding an emitted .isf."
---

**Established `2026-08-11` (`CORPUS-COVERAGE.2.52`).** `SemanticIr::build` filters EvidenceIR's
`signal_constraints` and `conditional_rules` to records whose subject/consequent is a **declared signal** — the
rule that stops prose-derived records becoming canonical hardware authority. Both filters sit behind the same
guard (`crates/specforge/src/ir/semantic.rs:283` and `:293`):

```rust
let conditional_rules = if declared_signal_names.is_empty() {
    evidence_ir.conditional_rules.clone()      // no filter at all
} else {
    /* keep only rules whose consequent_signal is declared, or absent */
};
```

So the filter is **strongest on documents that have signal authority and absent on documents that have none**.
This is a discontinuity, not a gradient: one declared signal filters everything against a one-element catalog,
zero declared signals filters nothing. `declared_signal_names` is built at `:276-281` from interface
`signal_records`, excluding `AutomationConfidence::Low`, so low-confidence declarations push a document *into*
the unfiltered branch.

The counter-intuitive consequence, and how it was found: refreshing
`opencapi_25gbps_phy_mechanical_spec_v10` retired its two false acronym signals (`IS`, `OD`), which emptied the
catalog — and its conditional rules went **0 → 2**. Removing false signals *added* ungrounded records. The
movement is input-driven, not a code delta: replaying the semantic stage from the **stale** EvidenceIR with the
same binary reproduces the stale result exactly (0 rules / 2 interfaces / 7 actors). The two promoted rules name
`PWR` and `OPEN` — prefixes cut at the underscore of `PWR_GOOD` and `OPEN_CAPI` — with actions
`(see source_text)` and `must be taken` (the latter captured from the idiom "Care must be taken that…").

**Measured blast radius** over all 78 persisted SemanticIR artifacts: 33 documents have an empty declared-signal
set and 29 of those ride the unfiltered branch, carrying **1,423 conditional rules** and **100 signal
constraints** with no grounding check. Largest: `usb_3_2_revision_1_0_2017_09` (542),
`nvme_base_specification_2_0a_2021_07_26` (250), `ihi0088_g_2024_06_amba_dti_protocol_specification` (116).
Observed consequents include `NOTICE`, `PDF`, `IMPLEMENTATION`, `UNPREDICTABLE`, `DATASHEET`, `MUST`, `RISC`,
`IBM`, `FFFF`, `YYY`, `QRDDL` — metadata, boilerplate, English modals, and table noise.

**It reaches no emitted target today.** All 44 emitted `.isf` come from populated-catalog documents, and an empty
catalog blocks the adapter on `no signals declared in interface` before any rule renders. The defect pollutes
canonical `SemanticIR`/`IntentIR` and the surfaces reading them (validation, priors, corpus KB), not the product
boundary — but a document declaring one real signal alongside this prose noise takes the populated branch and
filters correctly, so the exposure is one grounded signal wide.

`SEMANTIC-EMPTY-CATALOG-FILTER` owns the repair with three candidate rules (symmetric filter, grounded-token
test, or demote-to-residual). It was deliberately not fixed inside the refresh that found it: the change moves 29
documents and needs its own corpus-wide old-versus-new replay and before/after evals. Related:
[[parenthetical-data-head-requires-wire-qualifier]] (the gate that retired `OD` and emptied this document's
catalog in the first place).
