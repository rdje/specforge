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
status: superseded
tags: [semantic-ir, intent-ir, phases, authority, corpus-coverage]
evidence: docs/research/semantic-phase-authority-measurement.md; docs/tasks/CORPUS-COVERAGE.md (.2.43a); crates/specforge/src/ir/semantic.rs; docs/book/src/pipeline/semanticir.md
reverify: "cargo test -p specforge generic_phase --lib --quiet && cargo test -p specforge generic_section_phase --lib --quiet"
---

This card records the intermediate `.2.43a` boundary that removed sentence-only authority. The complete title
audit subsequently found no defensible generic section-phase grammar and retired the producer and Intent
projection. Use [[legacy-generic-section-phases-are-audit-only]] for current behavior. The independent typed
`transaction_phases[]` recognizer remains active.
