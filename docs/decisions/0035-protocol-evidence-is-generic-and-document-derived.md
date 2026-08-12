---
id: protocol-evidence-is-generic-and-document-derived
title: Protocol evidence is generic, document-derived, and fail-closed across schema migration
date: 2026-08-12
status: accepted
scope: genericity, evidence-ir, schema, extraction, projection, residual-honesty
evidence: crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; docs/research/production-genericity-pipeline-audit.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md
answers:
  - "can EvidenceIR represent protocol structure without hardcoding a specification"
  - "what replaced the SWD-specific EvidenceIR carrier"
  - "why does EvidenceIR schema 2 clear old protocol records"
  - "is lower recall acceptable when removing protocol-specific shortcuts"
  - "does generic protocol evidence still project losslessly to IntentIR"
---

# ADR 0035: Protocol evidence is generic, document-derived, and fail-closed across schema migration

## Context

ADR 0016 correctly required lossless EvidenceIR→SemanticIR→IntentIR projection and honest adapter residuals,
but it made a named protocol's phase enum, operation branches, signal direction, and response vocabulary part of
the canonical production schema. That violates ADR 0006: a generic extractor may know universal document and
digital-intent structure, but it may not remember the named specification that demonstrated it.

A finite forbidden-token list cannot establish this boundary. New protocols, aliases, fragments, identifiers,
and non-textual identity coupling are unbounded. The carrier and admission grammar themselves must make stored
names input-derived and named conformance knowledge incapable of steering production.

## Decision

1. EvidenceIR schema 2 carries generic frame fields, protocol operations, participant-drive relations, protocol
   states, and interface-edge timing. Phase, field, branch, operation, participant, response, and state names are
   opaque strings derived from the current document; the schema has no protocol-specific enum or role map.
2. Production admission is structural and fail-closed. Frame fields require explicit frame/packet and phase
   binding; direction requires an explicit source-to-destination relation; operations require explicit phase
   cardinality and a source label; states require explicit state-machine/naming grammar and repeated support.
3. Historical schema-1 protocol records are not reinterpreted as generic truth. Loading schema 1 clears their
   frame/state/operation and extraction-manifest authority before typed deserialization, preserves unrelated
   evidence, and upgrades the in-memory artifact to schema 2. A re-ingest is required to repopulate these
   surfaces. Unknown future schemas reject.
4. SemanticIR, IntentIR, convergence, validation, evaluation, and adapter accounting preserve the generic records
   exactly. Unsupported executable bindings remain explicit residuals; projection never guesses them.
5. Lower recall caused by removing a named shortcut is accepted as an honest deficit. Recall may be restored only
   with universal grammar or source-grounded proposals that pass identity/alpha-renaming and negative controls.
6. Named protocol examples and expected values belong only to conformance tests, fixtures, and research inputs.
   This decision does not declare the whole production core neutral; the remaining identity/spelling paths and
   the structural doctrine/invariance gates remain owned by `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d`–`.f`.

## Consequences

- The same carrier can describe a serial packet, a parallel transaction, or a previously unseen protocol without
  adding a public type for that specification.
- A schema migration cannot launder old vocabulary-bound output into current neutral authority.
- Persisted artifacts can become sparser after the migration. That is a visible recovery frontier, not a reason
  to reinstate names.
- ADR 0016 is superseded where it prescribes the named carrier. Its lossless-projection and honest-residual
  requirements remain in force through this generic replacement.
- This establishes feasibility, not omniscience: absent, contradictory, external, unreadable, or ambiguous source
  intent must remain residual rather than be fabricated.

## Links

- Genericity invariant: [`0006-no-hardcoded-chip-spec-vocabulary.md`](0006-no-hardcoded-chip-spec-vocabulary.md)
- Superseded carrier decision: [`0016-swd-protocol-projection-and-honest-isf-boundary.md`](0016-swd-protocol-projection-and-honest-isf-boundary.md)
- Task tree: [`SPEC-TO-INTENT-ALIGNMENT.md`](../tasks/SPEC-TO-INTENT-ALIGNMENT.md)
- Pipeline audit: [`production-genericity-pipeline-audit.md`](../research/production-genericity-pipeline-audit.md)
