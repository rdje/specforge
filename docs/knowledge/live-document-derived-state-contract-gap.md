---
id: live-document-derived-state-contract-gap
title: SpecForge has surface currentness but lacks the newer field-level derived-state contract
answers:
  - "did SpecForge adopt the 2026 08 09 FSMGen derived state containment revision"
  - "which exact current state copies are not yet independently verified in SpecForge"
  - "how should Rust version copies be verified across Cargo README book and CI"
  - "how should the current FSMGen gitlink in documentation be verified"
  - "why should corpus counts leave MEMORY md"
  - "what does LIVE DOCUMENT SIZE CONTAINMENT ADOPTION 8 implement"
date: 2026-08-09
status: current
tags: [documentation, live-document-containment, derived-state, currentness, memory, gitlink, rust-toolchain]
evidence: docs/research/live-document-derived-state-adoption-delta.md; docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md (.8); LIVE_DOCUMENT_SIZE_CONTAINMENT.md; doctrine/live_document_size/surfaces.jsonl; MEMORY.md; README.md; docs/FSMGEN_FEEDBACK.md
reverify: jq -s '[.[] | select(.surface_id)] | length' doctrine/live_document_size/surfaces.jsonl
---

SpecForge's `2026-08-08` containment adoption already has 41 governed surfaces, eight enforced currency contracts,
15 non-budget lifecycle verifiers, bounded generated projections, lossless rolling ledgers, and same-volume
locality. The donor's `2026-08-09` portable revision nevertheless adds a distinct field-level contract that the
local doctrine, registries, and neutral checker do not yet represent: exact current state must be derived on read
or retained only as an authority-verified copy, while authored intent and revision-bound evidence remain separate.

Three concrete gaps prove this is not wording-only. Before the `.8a` pointer overwrite, `MEMORY.md` carried
ungated corpus/cache counts as claims about now; the Rust `1.95.0` prerequisite is copied across README/book/CI without comparison to workspace
`rust-version`; and the feedback root/contract agree on FSMGen gitlink
`d327129b718ab29fc889db026c19257b0f7fcc49` without checking the mode-`160000` Git-index authority. The current
values agree, so this is missing enforcement rather than observed drift.

`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8a` selects a bounded local adoption. `.8b` adds explicit field markers
and closed classifications, a neutral bounded registry/checker plus project-specific authority adapter, and the
two deterministic comparisons; the `.8a` pointer overwrite has already removed the misplaced resume counts. Existing surface generators/currentness
oracles remain authoritative rather than being duplicated. `.8c` independently audits the result. Donor paths,
thresholds, measurements, and conclusions are not copied, and no ceiling is widened.

The card/index plane is an explicit implementation constraint: this card's catalog row leaves
`docs/knowledge/INDEX.md` at 32,664 bytes, 514 bytes below its first failing 90% rollover byte. `.8b` updates this
card rather than adding another; any crossing requires an owned partition before the append, not a wider ceiling.
