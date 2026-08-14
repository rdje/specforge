---
id: behavioral-identity-alpha-harness
title: Conformance owns closed five-stage identity and alpha qualification
answers:
  - "where is the behavioral alpha renaming harness implemented"
  - "how does behavioral genericity normalize renamed stable ids safely"
  - "how does the adversarial PDF identity test preserve source bytes"
  - "why do unchanged PDF proof digests differ between isolated scratch roots"
  - "which collection order changes may the behavioral comparator normalize"
  - "does the alpha harness expose transform recipes to production core"
date: 2026-08-14
status: current
tags: [genericity, metamorphic-testing, conformance, alpha-renaming, document-identity, proof-ledger]
evidence: crates/specforge-conformance/src/behavioral_genericity.rs; docs/research/behavioral-genericity-qualification-design.md; docs/tasks/spec-to-intent-alignment/behavioral-qualification.md; doctrine/production_genericity/behavioral_qualification.json
reverify: "cargo test --offline -p specforge-conformance behavioral_genericity --no-fail-fast && cargo test --offline -p specforge-conformance behavioral_genericity::tests::full_pdf_adversarial_identity_run_emits_five_stage_evidence -- --ignored --exact"
---

`specforge-conformance::behavioral_genericity` owns transforms, comparison, and evidence; `specforge-core`
receives only an ordinary current input. The harness copies a hash-pinned source into repository-local scratch,
runs isolated SourceIR, EvidenceIR, SemanticIR, IntentIR, and ISF-adapter pipelines, reloads every persisted
artifact through canonical verification, and emits `behavioral_evidence.json` with contract/source/recipe/prior/
tool identities plus complete field and proof-claim coverage.

Symbol alpha derives only typed source-bound identifiers that occur as complete source tokens. Its deterministic
bijection replaces every occurrence, uses seed-qualified familiar engineering aliases, and reverses the original
lexical order. Case-fold ambiguity, collision, missed occurrence, empty eligible catalog, or non-bijection
rejects. The normalized-text calibration passes all five stages without claiming PDF page, visual, table, or
geometry coverage.

Inverse comparison substitutes only declared symbols, document/source/scratch identity, and relation-bound
content/proof digests. A changed derived id maps only when its enclosing normalized record is otherwise
equivalent and both directions remain bijective. Keyed record collections, explicit id/name reference sets,
grouped-interface symbol sets, and emitted ISF interface declarations may realign after lexical perturbation;
unkeyed ordered arrays remain exact. A semantic-role mutation therefore remains visible and fails.

The adversarial identity relation copies a valid PDF byte-for-byte to a deterministic misleading filename on the
repository filesystem. Its path/display identity and document key change; full rich capture and every normalized
semantic/proof decision remain invariant. An explicit test exercises the actual repository-local Docling
provider across all five stages.

The unchanged-PDF control also passes. Isolated scratch paths are inputs to canonical proof contexts, so verified
scope/conclusion/input digests differ even when normalized semantic content is exact. The declared relation
normalizes those scratch-root cryptographic consequences while still requiring exact ruleset, prior, validation,
proof topology, and every non-digest value.
