---
id: behavioral-reviewed-recipe-boundary
title: Reviewed text equivalence is digest-pinned and parser-boundary demonstrated
answers:
  - "where are behavioral paraphrase and harmless layout recipes registered"
  - "how does SpecForge prove a reviewed paraphrase is parser equivalent"
  - "why was the at least timing paraphrase rejected"
  - "why can an extra blank line fail harmless layout comparison"
  - "which fields may reviewed text normalization change"
  - "how are source-derived ids normalized for a reviewed paraphrase"
  - "what reviewed paraphrase and layout calibrations currently pass"
date: 2026-08-14
status: current
tags: [genericity, metamorphic-testing, paraphrase, layout, conformance, proof-ledger]
evidence: doctrine/production_genericity/reviewed_recipe_manifest.json; doctrine/production_genericity/reviewed_recipes/um11732_structure_preserving_paraphrase.json; doctrine/production_genericity/reviewed_recipes/um11732_harmless_layout.json; crates/specforge-conformance/src/behavioral_genericity.rs; docs/research/behavioral-genericity-qualification-design.md
reverify: "python3 -B scripts/check_behavioral_genericity_contract.py && cargo test --offline -p specforge-conformance behavioral_genericity::tests -- --nocapture"
---

Reviewed normalized-text equivalence is conformance authority, never a production inference. The manifest binds
each approved recipe to a reviewed-calibration source path, source SHA-256, recipe path, recipe SHA-256, relation,
and owner. A reviewed run rejects missing or mismatched authority, unsafe paths/fields, duplicate ids, stale or
ambiguous occurrences, overlapping or irreversible spans, incomplete change-kind coverage, stale conclusion
pointers/values, and any open unaffected complement.

The released paraphrase replaces one UM11732 timing sentence's “must not be less than” wording with “must not be
lower than.” Its recipe pins the one source span and all eight propagated evidence, semantic, and intent text
conclusions. The layout recipe pins four same-line-count variants: trailing heading whitespace, table-delimiter
alignment, horizontal whitespace on a blank line, and emphasis around “User manual.” It also pins two unaffected
semantic/intent timing conclusions. Both relations pass SourceIR, EvidenceIR, SemanticIR, IntentIR, and ISF-
adapter comparison and retain explicit page/visual/table/content-element/section exclusions.

Inverse text mapping is per changed span and per declared field; permissions are not unioned across spans. An
approved exact alphanumeric projection maps only source-derived ids containing that span. All unlisted leaves,
collection topology, proof addresses/rules/premises/confidence, residuals, validation, and lowering stay exact.

Human semantic equivalence alone is insufficient. Calibration rejected “at least” because it entered a different
timing-rule path and changed downstream proof topology. It rejected an added blank line because every subsequent
evidence coordinate shifted, and rejected a reworded numbered heading because section-derived records changed.
The harness fails those candidates instead of broadening normalization.
