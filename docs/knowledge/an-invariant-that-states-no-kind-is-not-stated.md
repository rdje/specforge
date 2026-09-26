---
id: an-invariant-that-states-no-kind-is-not-stated
title: An IntentIR temporal invariant lifted from a statement carries the kind its text states, or not_stated (TemporalInvariantKind::NotStated); before 2026-09-26 classify_invariant_text labelled text stating no kind must_remain_stable, 5,089 of 5,162 on the live chains (EXTRACTION-REPRESENTATION-GAPS.11.a)
answers:
  - "why do most tinv_invariant records say not_stated"
  - "what is TemporalInvariantKind::NotStated"
  - "what does classify_invariant_text return for prose that states no stability"
  - "may .11 lower a statement invariant as a stability assertion"
date: 2026-09-26
tags: [intent, invariants, classification, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/intent.rs (TemporalInvariantKind, classify_invariant_text, synthesize_temporal_invariants); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.11, .11.a)
reverify: "cargo test --offline -p specforge-core --lib -- temporal_invariant_kind_classification. jq '[.temporal_invariants[] | select(.invariant_kind == \"not_stated\")] | length' generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json -> 845 on 2026-09-26 (AXI's share of the 5,089)."
---

**Measured and landed `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.11.a`, before `.11` lowers stability as
assertions).** `synthesize_temporal_invariants` lifts SemanticIR's statement invariants into IntentIR, using
`classify_invariant_text` for the kind. Six phrases name a kind: *must not change*, *remain stable*/*must be
stable*, *must be asserted*, *must match*, *must not exceed*, *only valid when*. Any other text fell to a default of
`MustRemainStable` ("default for chip invariants").

On the 27 live chains that default carried 5,089 of 5,162 such invariants. Examples: *The PE cannot service
interrupts in Debug state.*, *Each access port can drive a different memory space.* None has a subject signal, so
none reached an `.isf`. Each was nonetheless a stability claim the document never made, and lowering stability as
an assertion (`.11`) would have emitted it as one.

`TemporalInvariantKind::NotStated` is now the default. Exactly 5,089 invariants flip `must_remain_stable` →
`not_stated`; the 73 that name a kind are unchanged, and no semantic record, adapter or `.isf` moves. `.11` lowers
only a stated kind.
`[[an-invariant-holds-at-every-cycle]]` · `[[an-uncited-obligation-is-a-candidate-miss]]`.
