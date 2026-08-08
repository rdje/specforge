# IntentIR Product Contract

`IntentIR` is the canonical product boundary of SpecForge. The tool exists to recover
implementation-relevant intent from specifications; `.isf` is a downstream adapter representation,
not a replacement for the canonical intent model.

## Required properties

`IntentIR` must remain:

- backend-independent;
- serializable and explicitly schema-versioned;
- precise enough for adapters to lower without reconstructing source meaning;
- traceable to grounded upstream evidence;
- explicit about assumptions, abstractions, conflicts, and unresolved decisions; and
- extensible as new evidence modalities and typed intent surfaces are recovered.

It represents more than syntax. Its domain includes actors and responsibilities, interface and
connectivity structure, system contracts, state and behavior, timing and ordering constraints,
assumptions, abstractions, and residual decisions. The current concrete surface is described in the
[IntentIR chapter](../pipeline/intentir.md); this chapter defines the stable boundary that surface
must respect.

## Stage boundary

The canonical order is:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

Each stage has a separate truth contract. `IntentIR` consumes promoted semantic meaning; it must not
reach around `SemanticIR` to reinterpret raw source text or allow an adapter to inject semantics
backward into the canonical model. Incomplete grounded intent is acceptable. Fabricated completeness
is not.

## Residual decisions

Residual decisions are first-class product data. Each unresolved packet must preserve enough
structure for review and replay, including:

- the unresolved question;
- why automation could not decide safely;
- candidate interpretations when known;
- downstream impact;
- automation confidence; and
- provenance or related record identifiers needed to revisit it.

Residuals make automation honest: they allow the pipeline to proceed without hiding ambiguity in
free-form notes or silently choosing a convenient backend interpretation.

## Serialization and identity

- JSON is the primary interchange representation for staged IR artifacts.
- Every artifact carries a schema version and stage identity.
- Stable machine identifiers matter more than display wording.
- Repository-owned persisted paths are repository-root-relative; absolute host paths are not part of
  the portable product contract.
- Source, evidence, and decision provenance must survive canonicalization where it is needed to
  explain or revalidate a promoted fact.
- Visually rich sources retain their structured page and asset evidence even when normalized
  Markdown is also available.

The [Generated Artifacts](generated-artifacts.md) chapter documents current layouts and inspection
paths. It is the operational authority; examples in this contract are intentionally avoided so a
conceptual payload cannot drift into a second schema definition.

## Adapter boundary

SpecForge has one adapter target: `.isf`. The typed lowering is
`IntentIR -> IsfIr::from_intent_ir() -> render() -> .isf`.

FSMGen consumes `.isf` and owns scheduling, `.fsm`, and HDL downstream. SpecForge does not move those
responsibilities into `IntentIR`, and adapter convenience must not collapse the canonical model into
ISF-specific syntax. See the [ISF Adapter](../pipeline/isf-adapter.md) chapter for current lowering
behavior and honest residuals.

## Guardrails

- Do not let `IntentIR` collapse into `.isf`, `.fsm`, HDL, or any other target's assumptions.
- Do not let adapter concerns leak backward into evidence or semantic truth.
- Do not treat Markdown reports as the canonical IR system of record.
- Do not reduce visually meaningful sources to text-only evidence.
- Do not erase contradictions, abstractions, assumptions, or unresolved choices for a cleaner score.
- Do not claim complete automation while material residual decisions remain.
- Do not let learned cross-document priors author facts that the current source does not support.

These rules define the product even as the concrete typed schema evolves.
