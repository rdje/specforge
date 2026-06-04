---
id: dempster-fusion
title: SpecForge fuses agreeing-source confidence via Dempster corroboration (not min)
answers:
  - "how does SpecForge combine confidence across modalities or sources"
  - "does agreement between sources boost confidence"
  - "why doesn't fusion use the minimum confidence"
  - "what is the Dempster combiner in fusion"
  - "how is a fused contract's automation_confidence computed"
date: 2026-06-04
tags: [fusion, confidence, multimodal, dempster]
evidence: crates/specforge/src/ir/fusion.rs
reverify: grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs
---

`ir/fusion.rs::merge_cluster` fuses contracts sharing a `FusionKey`. When they **agree**, the
merged `automation_confidence` is the **Dempster corroboration** of the sources, not `min`:
each ordinal confidence → belief mass (High `0.9` / Medium `0.7` / Low `0.5`), combined
`m = 1 − ∏(1 − mᵢ)`, mapped back (`≥0.9 → High`, `≥0.7 → Medium`). So independent agreement
**raises** confidence (Medium+Medium → High; Low+Low → Medium; High caps; a single source is
unchanged) instead of capping at the weakest.

On **disagreement** (different kind/obligation/guard) the merge routes to a `Residual` and keeps
the conservative `min` — conflict is never corroborated. The conflict mass `K` = 0 on this path
(disagreement is pre-split before any confidence combination), so the Zadeh high-conflict guard
is a documented future extension. Grounded in Dempster (1967). See
`docs/tasks/DEMPSTER-FUSION-COMBINER.md`, `multimodal-fusion.md`.
