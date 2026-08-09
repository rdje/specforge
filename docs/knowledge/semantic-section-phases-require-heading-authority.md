---
id: semantic-section-phases-require-heading-authority
title: Generic SemanticIR section phases require heading authority
answers:
  - "can the word when in a statement make its whole section a semantic phase"
  - "can before after once or while authorize SemanticIR phases"
  - "how are generic SemanticIR phases different from transaction_phases"
  - "why did an OpenCAPI functional test become an IntentIR behavior"
  - "where are section-derived semantic phases built"
  - "how many retained phases depended only on sequencing words"
  - "does removing sentence fallback remove address phase recognition"
  - "are Reset value headings valid semantic phases"
date: 2026-08-10
status: current
tags: [semantic-ir, intent-ir, phases, authority, corpus-coverage]
evidence: docs/research/semantic-phase-authority-measurement.md; docs/tasks/CORPUS-COVERAGE.md (.2.43a); crates/specforge/src/ir/semantic.rs; docs/book/src/pipeline/semanticir.md
reverify: "cargo test -p specforge build_phases --lib --quiet && cargo test -p specforge typed_transaction_phase_does_not_depend_on_generic_section_phase --lib --quiet && jq '{statements:(.extracted_statements|length)}' generated/evidence_ir/opencapi_3_0_ready_definition_v1_1/evidence_ir.json && jq '{phases:(.phases|length),transaction_phases:(.transaction_phases|length)}' generated/semantic_ir/opencapi_3_0_ready_definition_v1_1/semantic_ir.json"
---

`SemanticIR.phases[]` is a broad section-derived surface. A whole section now needs a non-empty, phase-like
heading to enter it. Sentence-level sequencing words such as `before`, `after`, `once`, `when`, and `while` may
occur in requirements, notes, register descriptions, qualification attestations, and test procedures; they do
not establish section-wide phase authority.

This boundary is independent of `SemanticIR.transaction_phases[]`. The typed transaction recognizer uses a
bounded `<qualifier> phase` grammar, retains exact statement provenance, and intersects referenced signals with
the declared inventory. An explicit `address phase` therefore remains recognizable even when its containing
section does not become a generic phase.

The exact retained-artifact census classified 11,286 historical generic phase records: 3,180 had a title cue and
8,106 across 77 documents depended only on the removed sentence fallback. Rebuilding OpenCAPI 3.0 Ready
Definition from unchanged 173-statement EvidenceIR removes its three false phases and three derived Intent
behaviors while every other measured semantic count holds.

The 3,180 title-authorized records are not all certified as correct by this fact. In particular, 720 are headed
`Reset value`. Pending child `CORPUS-COVERAGE.2.43a.i` owns that distinct title-grammar audit; the current fact
establishes only that sentence prose cannot promote its entire section.
