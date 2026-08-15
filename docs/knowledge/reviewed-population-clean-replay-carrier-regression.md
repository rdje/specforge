---
id: reviewed-population-clean-replay-carrier-regression
title: Fresh reviewed replay blocks on qualified-header and register-access carrier gaps
answers:
  - "why is the latest reviewed population replay not published"
  - "did the current clean replay contradict the pinned b977 source to intent result"
  - "where are actual replay stage hashes stored"
  - "why did Arm Debug lose twelve reviewed register facts"
  - "why does GIC 400 emit fifteen registers with missing access"
  - "why did the AMD IOMMU packed layout false register disappear"
  - "what does SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.a repair"
  - "what are the blocked clean replay source to IntentIR counts"
date: 2026-08-15
status: current
tags: [spec-to-intent-alignment, replay, register-map, structural-classification, regression]
evidence: docs/research/reviewed-population-clean-replay-diagnostic.md; docs/tasks/spec-to-intent-alignment/behavioral-qualification.md; .project-data/tmp/spec-to-intent-f-iv-population-r1/replay_manifest.json
reverify: "python3 -B scripts/replay_source_to_intent_population.py --output-root .project-data/tmp/<fresh-root> --external-source-map .project-data/tmp/<runtime-map>.json --replay-id <portable-id> --owner SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.b --dataset-id <portable-id>"
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

`.f.iv.a` owns a specification-instance-neutral carrier repair: closed header roles may admit a parenthesized
qualifier, and register-map access must survive an explicit or unambiguous structural access column. `.f.iv.b`
then owns a new clean replay, portable publication, and exact cleanup. No diagnostic metric is promoted first.
