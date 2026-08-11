---
id: timing-observation-to-verified-figure-contract
title: Production timing observations now reach verified figure contracts through an optional typed EvidenceIR FigureRegion
answers:
  - "does FigureRegion have a production producer or only synthetic tests"
  - "how does a VLM timing note become a FigureRegion"
  - "where is FigureRegion stored in EvidenceIR"
  - "how does a timing diagram become an ActorContract in SemanticIR and IntentIR"
  - "can a VLM-invented waveform lane become canonical intent"
  - "what gives timing observation samples tick authority"
  - "what happens when a timing observation has no explicit tick-addressed samples"
  - "are figure-mined contracts verified before fusion and lowering"
  - "what real PDF proves the FigureRegion vertical path"
  - "does the reviewed I2S fixture claim a live VLM run"
  - "why is a stable span after tick zero residual"
date: 2026-08-11
tags: [figure-region, evidence-ir, semantic-ir, intent-ir, vlm, waveform, grounding, verification]
evidence: crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/figure_region.rs; crates/specforge/src/ir/waveform.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/test_data/figure_region_i2s_reviewed/fixture.json; corpus/nxp/i2s/current/UM11732_v3_2022-02-17_I2S_Bus_Specification.pdf; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.3)
reverify: "cargo test -p specforge --lib reviewed_i2s_pdf_figure_reaches_verified_intent_contract && cargo test -p specforge --lib ir::figure_region && cargo test -p specforge --lib ir::waveform"
---

`SPEC-TO-INTENT-ALIGNMENT.3` activates the existing region-crop VLM seam rather than adding a full-page pass.
`enrich` writes bounded timing JSON to `SourceIR.visual_assets[*].note`. During EvidenceIR construction, a
timing note remains a raw `TimingDiagramExtraction` observation and, when usable, also becomes the matching
`VisualEvidenceItem.figure_region: Option<FigureRegion>`.

The projection is intentionally narrower than the raw observation. Each retained sample needs an explicit JSON
integer, decimal string, or `T<n>` `cycle`; response-array order never supplies time. Only `HIGH`/`LOW` become
concrete levels, conflicts at one tick become `Unknown`, and free-form annotations remain `Unknown` rather than
being guessed into typed bounds. Validation separately reports `typed_figure_regions` and
`typed_figure_regions_unavailable`; the latter warns when a timing observation exists but no typed region could
be produced.

SemanticIR grounds every region against signal names already known from the same document. Exact names survive,
a unique case-folded match canonicalizes, and model-only or ambiguous lanes disappear before contract mining.
The grounded region flows through `figure_region_to_partial_trace` and `verified_contracts_from_figure_region`.
A lowerable candidate survives only when the round-trip trace verifier returns `Pass`; `Fail` and
`NotEvaluated` become explicit residuals before fusion/fidelity. A stable span beginning after tick zero is also
residual because the current `Stable` shape has no trigger/phase anchor and promoting it would relocate the
observation. IntentIR carries the resulting contracts unchanged.

The reviewed fixture is tied to retained NXP UM11732 Figure 1 by repository-relative PDF path, PDF size/hash,
page, asset/caption/source reference, and reviewed crop size/hash/dimensions. Its vertical test exercises the
persisted PDF → SourceIR → EvidenceIR → SemanticIR → IntentIR boundaries, proves a grounded `WS` stable contract
survives, and proves an injected model-only lane does not. The fixture is labeled
`reviewed_fixture_no_live_vlm`: no Ollama or LM Studio endpoint was ready, and the retained corpus contains zero
persisted VLM notes. This proves production wiring and fail-closed semantics, not live-provider recall or
precision. Related: [[full-page-capture-gap]], [[page-image-disk-bounding]].
