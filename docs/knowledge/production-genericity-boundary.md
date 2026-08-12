---
id: production-genericity-boundary
title: Production genericity is structural and behaviorally invariant
answers:
  - "Can SpecForge theoretically be specification and PDF neutral?"
  - "What does spec-neutral extraction mean?"
  - "Why is a forbidden vocabulary list insufficient?"
  - "Where is the whole extraction-pipeline genericity audit?"
  - "Why was PDF-AGNOSTIC-EXTRACTION closure superseded?"
  - "How does SourceIR classification remain specification neutral?"
  - "What happens when schema-1 SourceIR is loaded?"
  - "Why were legacy SourceIR classifications neutralized?"
  - "Were downstream artifacts reconciled after SourceIR schema 2?"
  - "How does EvidenceIR schema 2 remove protocol-specific extraction authority?"
date: 2026-08-12
status: current
tags: [genericity, extraction, architecture, doctrine]
evidence: docs/research/production-genericity-pipeline-audit.md; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/source/docling_backend.rs; crates/specforge/src/ir/evidence.rs
reverify: rg -n "SOURCE_IR_SCHEMA_VERSION|neutralize_legacy_source_classifications|EVIDENCE_IR_SCHEMA_VERSION|neutralize_legacy_protocol_json|embedded_source_classifiers_are_structural" crates/specforge/src/ir/source.rs crates/specforge/src/ir/source/docling_backend.rs crates/specforge/src/ir/evidence.rs
---

A neutral SpecForge is feasible when universal digital-intent semantics are separated from opaque,
document-derived symbols and provenance-bearing proposals. It cannot promise perfect recovery from
missing or ambiguous source information; it must residualize that uncertainty. Genericity is proved
by production module/type boundaries, registered grammar access, and renaming/identity/paraphrase
invariance—not by a finite token denylist. The canonical audit and enforcement design are in
`docs/research/production-genericity-pipeline-audit.md`; ADR 0006 owns the invariant.

SourceIR is the first remediated production boundary. Schema 2 classifies only explicit generic
visual/section forms and typed table roles; operation, participant, filename, protocol, and symbol
spellings cannot grant a kind. Unsupported shapes remain `unknown`. Loading a schema-1 SourceIR
preserves its captured source structure and provenance but neutralizes old diagram/table/section
semantic labels until re-ingest, because those labels were produced by the retired corpus-calibrated
policy. Future schemas fail closed. A read-only 78-document replay measured the intentional re-ingest
delta as 2,293 diagram, 2,123 section, and 3,484 table label changes without mutating an artifact.

ADR 0025 then identified and rebuilt the exact ten affected replayable cascades. Current-binary equality is
restored for 24/24 measurable EvidenceIR and all 78 SemanticIR, IntentIR, and adapter chains; both renderable
ISFs are FSMGen-strict clean. The reconciliation deliberately accepts 194 removed table-derived timing guesses
and other weak label-driven facts. Those are inputs to later generic recovery work, never justification for a
document-specific exception.

EvidenceIR is the second remediated boundary. Schema 2 replaces fixed protocol operations, phase enums, and
participant-direction roles with input-derived generic records. Frame, operation, state, and direction
extractors admit only explicit structural grammar and preserve document terms opaquely. Schema-1 loads clear
the old vocabulary-bound protocol surfaces before typed deserialization, preventing a retained artifact from
bypassing the new producer. The carriers remain lossless through SemanticIR, IntentIR, convergence,
validation, evaluation, and adapter residual accounting.
