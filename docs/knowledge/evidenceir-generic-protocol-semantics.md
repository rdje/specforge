---
id: evidenceir-generic-protocol-semantics
title: EvidenceIR protocol structure is document-derived and schema-2 neutral
answers:
  - "How are frame fields extracted without hardcoding a protocol?"
  - "What replaced SwdOperation and SerialFramePhase?"
  - "What replaced swd_operations and swdio_direction?"
  - "How does EvidenceIR schema 2 handle old protocol-specific artifacts?"
  - "Does SemanticIR and IntentIR preserve generic protocol records?"
  - "Can a signal or response spelling select a production extractor?"
  - "Why can neutral extraction lose recall after removing shortcuts?"
date: 2026-08-12
status: current
tags: [genericity, evidence-ir, schema, protocol-structure, provenance]
evidence: crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/eval.rs; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6d.ii.c)
reverify: "cargo test -p specforge --lib evidence_ir_schema_ && cargo test -p specforge --lib swd_serial_extraction && bash scripts/check_chain_currency.sh --check"
---

EvidenceIR schema 2 exposes generic `SerialFrameField`, `ProtocolOperationRecord`,
`ParticipantDriveRecord`, and `ProtocolStateRecord` carriers. Frame phases, operation/branch labels,
participant names, response values, field names, and state names are opaque strings copied from the current
document. The production types contain no fixed protocol phase enum, response branch, signal spelling, or
host/target role mapping.

Admission is structural. A bit range or named bit must be explicitly bound to a frame/packet statement or a
source-named phase; composition lists establish field sequence from their own order. Participant direction
requires an explicit `from A to B` relation. Operations require an `operation` or `response` clause with an
explicit phase cardinality. State grammars require repeated inventories or explicit state-machine/transition
structure. Unsupported or weak text emits no record instead of selecting a named fallback.

SemanticIR, IntentIR, convergence snapshots, validation, evaluation, and adapter residual accounting carry
the generic records losslessly. EvidenceIR schema 1 is not reinterpreted: loading first clears its old frame,
operation, state, and extraction-manifest authority in untyped JSON, upgrades the in-memory schema to 2, and
preserves all unrelated evidence. Re-ingestion from SourceIR is required to repopulate the neutral surfaces;
future schemas fail closed.

The migration can intentionally reduce benchmark recall. That is evidence that the retired result depended on
a stored vocabulary, not evidence that neutral extraction is impossible. Recall must be rebuilt with universal
document grammars and grounded model proposals while alpha-renaming and negative controls keep the production
decision graph invariant.

The 2026-08-12 corpus reconciliation migrated all 78 persisted EvidenceIRs to schema 2, rebuilding the 24 whose
normalized bundles remain retained and neutralizing only the obsolete surfaces in the other 54. Evidence changed
in all 78 because of the schema firewall; content deltas affected 24 downstream chains. The old population held
22 fixed-phase frame records, four named operations, and 76 state records across 24 documents. The neutral replay
holds five generic operation records across two documents and 40 structurally admitted states across three; it
emits no frame record where the current sources do not satisfy the stricter phase-binding grammar. Chain currency
is exact at 24/24 measurable EvidenceIR and 78/78 SemanticIR, IntentIR, and adapters.
