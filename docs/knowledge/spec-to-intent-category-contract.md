---
id: spec-to-intent-category-contract
title: The source-to-IntentIR completeness contract is category-aware, per-cell exact, and independent of ISF emission
answers:
  - "what is the source-to-IntentIR completeness contract"
  - "what precision and recall floors must a supported document category meet"
  - "can strict-valid ISF prove that a PDF was understood (no)"
  - "when may a semantic family or source modality be marked non-applicable"
  - "what happens when document intent category is unresolved"
  - "which source modalities and IntentIR surfaces are required per chip-spec category"
  - "why do message fields prevent a wire or register category completeness claim"
  - "what global provenance and stage-conservation gates apply before executable lowering"
date: 2026-08-11
tags: [spec-to-intent-alignment, completeness, intentir, category, source-modalities, provenance, conservation, recall, precision, residuals]
evidence: doctrine/spec_to_intent_category_contract.json; docs/book/src/source-to-intent-contract.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.1); crates/specforge/src/ir/evidence.rs (EvidenceIr typed surfaces); crates/specforge/src/ir/semantic.rs (SemanticIr carriers); crates/specforge/src/ir/intent.rs (IntentIr canonical surfaces)
reverify: "python3 -m json.tool doctrine/spec_to_intent_category_contract.json >/dev/null && rg -n 'reviewed_gold_(precision|recall)|stage_conservation_or_residual|message_field_records|unresolved_category_policy' doctrine/spec_to_intent_category_contract.json"
---

`SPEC-TO-INTENT-ALIGNMENT.1` fixes a reviewed, versioned contract for the
`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` boundary. It applies separately to each document, semantic
family, and present source modality; a macro average cannot hide a failing cell. A support claim needs at least
two held-out category examples plus exhaustive reviewed gold for each applicable family/modality cell. Initial
gold precision and recall floors are both `1.000`.

The hard honesty floors are exact: source-region disposition, modality accounting, canonical provenance,
stage conservation-or-residual, and residual actionability are each `1.000`; fabricated canonical facts,
unexplained stage drops, silently removed conflicts, and forced category errors are each zero. ISF emission is
measured only after this upstream boundary passes and is never an upstream-completeness proxy.

The contract names the six dominant-purpose categories separately, even though the automatic recognizer
honestly combines register/platform and may return `unresolved`. An unresolved result cannot pass: it keeps the
category residual and applies the union of plausible modality checks until review decides. Non-applicable is
also evidenced, never inferred from zero output: the record must name category, family/modality, source scope,
reason, reviewer/oracle, and source ids.

The contract records current carrier gaps without treating honest residuals as completion. In particular,
`EvidenceIR.message_field_records` and `message_field_constraints` have no canonical `IntentIR` carrier. Packet,
flit, descriptor, queue, page-table, context, and other packed-layout facts therefore require source-linked
residuals and keep their category/family incomplete until a typed carrier exists.
