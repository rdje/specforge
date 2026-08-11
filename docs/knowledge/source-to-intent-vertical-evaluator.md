---
id: source-to-intent-vertical-evaluator
title: Vertical evaluation is a strict data-defined oracle over pinned four-stage snapshots
answers:
  - "how does SpecForge measure source-to-IntentIR stage loss"
  - "where is the source-to-Intent vertical evaluation schema"
  - "how does the held-out evaluator detect omission fabrication provenance loss and silent drops"
  - "how are incomplete gold and product failure distinguished"
  - "how are external held-out PDFs identified without absolute host paths"
  - "what makes a source-to-Intent residual actionable"
  - "does the vertical evaluator make a category support claim yet"
date: 2026-08-11
tags: [spec-to-intent-alignment, evaluation, stage-loss, provenance, residuals, portability, mutation-testing]
evidence: crates/specforge/src/ir/source_to_intent_eval.rs; doctrine/spec_to_intent_vertical_eval_schema.json; docs/book/src/source-to-intent-contract.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.4a)
reverify: "cargo test -p specforge --lib ir::source_to_intent_eval && python3 -m json.tool doctrine/spec_to_intent_vertical_eval_schema.json >/dev/null"
---

`SPEC-TO-INTENT-ALIGNMENT.4a` defines a strict version-1 dataset and a generic Rust evaluator over bounded,
pinned SourceIR, EvidenceIR, SemanticIR, and IntentIR JSON snapshots. Each review cell supplies JSON-pointer
queries, predicates, exhaustive keys, provenance ids, semantic family, modality, oracle, review scope, and
canonical/residual disposition as data. Runtime code contains no document, vendor, protocol, signal, or layout
exceptions.

Exact per-stage TP/FP/FN scores feed first-failing-stage, source-disposition, required-modality, provenance,
conservation-or-residual, residual-actionability, fabrication, and unexplained-drop results. Duplicate actual
keys remain false positives. An actionable residual must be exact at both promoted stages and populate every
review-required actionability field. Mutation tests kill omission, fabrication, lost provenance, silent stage
loss, missing modality, and inactionable-residual controls.

The selection boundary is a Git id; every original source and stage is hash-pinned. Repository sources use
root-relative paths. Necessary external read-only PDFs retain only a portable id, digest, and necessity, never a
host path. Too few documents or incomplete gold makes a category `unmeasurable` even if provisional diagnostics
already expose faults. The evaluator therefore cannot turn missing review authority into a pass. `.4a` ships the
oracle only; [[spec-to-intent-category-contract]] remains the acceptance authority,
[[source-to-intent-reviewed-population]] owns the frozen gold, and
[[source-to-intent-first-reviewed-result]] records the first actual category conclusions.
