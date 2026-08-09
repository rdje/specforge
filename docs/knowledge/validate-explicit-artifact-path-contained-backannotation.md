---
id: validate-explicit-artifact-path-contained-backannotation
title: Validate backannotation is contained to the explicit artifact path
answers:
  - "does specforge validate modify the artifact passed on the command line"
  - "where does specforge validate write validation_report.json"
  - "can validating a copied artifact modify the canonical generated artifact"
  - "why did validating a copied rollback backannotate the canonical chain"
  - "does validate follow the embedded artifact layout"
  - "does validate materialize SourceIR normalized manifests"
  - "what is the difference between validate persistence and stage write_to_disk"
  - "are extracted evidence and semantic records rewritten by validate"
  - "how is copied artifact validation path containment tested"
  - "what happened to the original Introducing CoreSight rollback"
date: 2026-08-09
status: current
tags: [validate, path-containment, backannotation, artifact-layout, source-ir, evidence-ir, semantic-ir, intent-ir, adapter]
evidence: crates/specforge/src/commands/validate.rs (write_backannotated_artifact; copied_artifact_validation_never_follows_embedded_canonical_layout); docs/tasks/CORPUS-COVERAGE.md (.2.38a)
reverify: "Run cargo test -p specforge copied_artifact_validation_never_follows_embedded_canonical_layout plus the canonical SourceIR and IntentIR backannotation tests, then cargo clippy -p specforge --all-targets -- -D warnings and scripts/run_ci.sh. For a real artifact copy, hash the complete embedded canonical tree, run target/release/specforge validate on the copy, require the copy and adjacent validation_report.json to change while the canonical tree stays byte-identical."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.38a`).** `specforge validate <artifact>` loads the explicit
repository-owned path, computes a deterministic report, backannotates validation metadata into that same JSON,
and writes `validation_report.json` beside it. The explicit command path is the mutation authority. A copied
artifact can retain embedded `artifact_layout` paths that point at the canonical generated tree; validation does
not follow them.

Before this repair, all five `persist_*_validation` helpers called the artifact type's normal `write_to_disk()`.
That is correct for stage producers because it follows the embedded canonical layout and, for a ready PDF
SourceIR, may materialize normalized summary manifests. It is incorrect for validating a copy: report placement
and console output followed the copy, while the IR write followed the embedded canonical destination. A #38
rollback probe reproduced the split on real SourceIR, EvidenceIR, SemanticIR, and IntentIR artifacts and created
canonical `normalized/page_artifacts.json` plus `normalized/visual_assets.json`.

The fix adds one stage-neutral `write_backannotated_artifact` helper. It resolves the explicit artifact path and
writes the artifact's persisted `to_pretty_json()` representation there; the existing sidecar helper writes the
adjacent report. It intentionally does not call a stage producer, rewrite the embedded layout, emit an adapter
target, or create SourceIR normalized siblings. Validation metadata changes, but extracted evidence and semantic
records do not.

The regression builds canonical SourceIR→EvidenceIR→SemanticIR→IntentIR→ISF-adapter artifacts, copies each JSON,
validates the copy, and requires one embedded validation report plus the adjacent sidecar. After every stage, a
byte snapshot proves the complete canonical generated tree—including all siblings—is unchanged. Existing
canonical SourceIR and IntentIR tests keep the ordinary same-path case green. A release-binary replay over all
five real #38 copies independently reproduces the containment result.

The incident's data disposition is explicit: the six original #38 artifact hashes and typed counts were recorded
before validation, but both canonical and copied JSON were backannotated before the side effect was detected.
Those original bytes are not claimed as recoverable. The authenticated same-SSD PDF
`08a37c3535e2c62b98cc5feb10c23f153972fe204a839daba292ca35349df162` is the parent refresh's regeneration
authority, and the complete validator-side-effect state remains task evidence until that refresh closes.
