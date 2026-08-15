---
id: reviewed-population-clean-replay-carrier-regression
title: Fresh reviewed replay carrier gaps are repaired and clean publication is complete
answers:
  - "why is the latest reviewed population replay not published"
  - "did the current clean replay contradict the pinned b977 source to intent result"
  - "where are actual replay stage hashes stored"
  - "why did Arm Debug lose twelve reviewed register facts"
  - "why does GIC 400 emit fifteen registers with missing access"
  - "why did the AMD IOMMU packed layout false register disappear"
  - "what does SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.a repair"
  - "did the qualified header and register access carrier repair pass"
  - "what remains before the repaired reviewed population can be published"
  - "was the repaired reviewed population published"
  - "what are the blocked clean replay source to IntentIR counts"
date: 2026-08-15
status: current
tags: [spec-to-intent-alignment, replay, register-map, structural-classification, regression]
evidence: docs/research/reviewed-population-clean-replay-diagnostic.md; docs/tasks/spec-to-intent-alignment/behavioral-qualification.md
reverify: "cargo test -p specforge-conformance --lib test_support::trajectory_snapshot"
---

The first clean `.f.iv` replay completes 12/12 hash-equal sources and 48/48 isolated stages at revision
`0d218116`, but it is diagnostic rather than publishable current truth. Its IntentIR aggregate is 12/15/28
TP/FP/FN, 30/30 provenance, 36/64 conservation, four of 24 actionable residual observations, 15 fabricated
facts, and 28 unexplained drops. Exactly three reviewed cells differ from the pinned `b977a51f` authority.

There is no evaluator contradiction. `build_fixture.py` deliberately puts frozen review-era hashes in the
bounded cell projection; actual fresh artifact hashes are in `replay_manifest.json` and differ across all target
chains. The pinned result remains valid evidence for its named revision, not the present production pipeline.

Fresh structural classification correctly makes the AMD 4×21 packed PTE layout `unknown`, removing its false
register. GIC-400 now classifies its `Offset | Name | Type | Reset | Description` table as a register map and
emits all 15 source names and offsets, but drops access because the register synthesizer does not treat the
structurally established `Type` column as access. Arm Debug remains unknown because exact role matching rejects
the qualified header `Address (A a, SELECT.DPBANKSEL)`, losing all 12 previously exact register/access facts.

`.f.iv.a` closes the specification-instance-neutral carrier repair. A closed header role may admit exactly one
parenthesized qualifier, and register-map access survives an explicit or uniquely unambiguous closed-literal
column. Direct real-PDF replay restores Arm's 12 and GIC-400's 15 reviewed facts while AMD stays empty. Exact
retained-chain reconciliation is 24/24 current and zero stale through the adapter; only Arm and one OpenCAPI
access carrier change outside proof/validation.

Publication is complete. `.f.iv.b` reinspection disproved the recorded driver mismatch: source, parent
revision, and the Rust example all show exactly four post-`--` values, while the diagnostic prose counted replay
root twice. A focused command-builder test pins that suffix and separates environment prefixes. The clean replay
publishes 39/0/1 IntentIR TP/FP/FN, 42/42 provenance, 117/118 conservation, zero fabrications, and one unexplained
APB drop. All controller hard gates pass and `.7` ranks first; `.6e` is superseded. The blocked 12/15/28 diagnostic
remains historical evidence rather than current product truth.

The `.f.iv.a` five-root scratch set was removed after the complete gate consumed it: 5,112 files / 3,614,756 KiB,
with every exact path absent on the residue census. The current retained chain remains 24/24 current at all four
replayed stages; the deleted rollback copy is intentionally no longer recoverable. `.f.iv.b` additionally removes
its three attempt roots (5,616 files / 2,038,580 KiB) and runtime map with no residue.
