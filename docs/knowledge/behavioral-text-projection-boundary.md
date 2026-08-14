---
id: behavioral-text-projection-boundary
title: Behavioral genericity separates rich-PDF identity proof from normalized-text metamorphics
answers:
  - "can normalized Markdown prove PDF ingestion is generic"
  - "which behavioral genericity transforms cover rich PDF capture"
  - "how many current documents are behaviorally measurable"
  - "what is the behavioral genericity held-out population"
  - "is the reviewed source-to-intent dataset historically unseen"
  - "what makes a behavioral genericity run invalid unmeasurable or failed"
date: 2026-08-14
tags: [genericity, metamorphic-testing, source-authority, held-out, proof-ledger, residual-honesty]
evidence: doctrine/production_genericity/behavioral_qualification.json; doctrine/production_genericity/behavioral_population.tsv; scripts/check_behavioral_genericity_contract.py; crates/specforge-conformance/src/behavioral_genericity.rs; docs/research/behavioral-genericity-qualification-design.md
reverify: "python3 -B scripts/check_behavioral_genericity_contract.py && python3 -B scripts/check_behavioral_genericity_contract.py --self-test && cargo test --offline -p specforge-conformance behavioral_genericity --no-fail-fast"
---

Behavioral qualification has two honest input planes. Byte-identical replay and adversarial filename/document
identity perturbation operate on hash-verified PDFs and cover the full SourceIR capture: pages, visual assets,
structured tables, content elements, and sections. Symbol alpha-renaming, reviewed paraphrase/layout variants,
and semantic negatives operate on the retained normalized Markdown projection. Direct Markdown SourceIR omits
those rich captures, so a passing text relation cannot be reported as PDF-ingestion, table, visual, or geometry
qualification.

The frozen current denominator is 24 proof-current retained chains: 3 repository-owned and 21 authorized
external PDFs. All 24 support PDF relations when their authority and local provider are available. Provider-free
text replay is semantically non-vacuous on 23; one row produces zero SemanticIR and IntentIR semantic records
and is explicitly unmeasurable for text metamorphics.

Seven current documents overlap the historically exposed 12-document reviewed evaluator. They are calibration,
not a retrospectively relabeled holdout. The other 17 current documents are prospectively held out from
behavioral recipe calibration at commit `8307100e96a7ed6b9f04315949fd2f1c3c232bbf`; four are vendor-novel and
13 family-novel relative to calibration. The prospective set covers four categories, so absent prospective
`register-ip` and `cpu-isa` denominators remain unmeasurable even though the reviewed evaluator spans six.

Every relation compares all fields and proof claims through SourceIR, EvidenceIR, SemanticIR, IntentIR, and the
ISF adapter. Missing authority/provider or a vacuous baseline is `unmeasurable`; stale population, ambiguous
transform, or partial coverage is `invalid`; a missing expected delta or undeclared semantic/proof/provenance/
validation/lowering delta is `fail`. None of those states may be summarized as a pass.

The first executable calibration preserves this boundary. A normalized-Markdown alpha pair passes all five
stages but carries the explicit rich-capture exclusion; separate unchanged and byte-identical adversarial PDF
pairs pass all five stages through the repo-local Docling provider. The unchanged relation normalizes scratch-root
and scratch-root-derived proof identity; the adversarial pair additionally changes only declared path/display/
document identity. None is yet the governed population signoff.
