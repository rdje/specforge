# Corpus KB Schema And Policy

## Boundary

The corpus knowledge base is an inspectable synthesis plane.
It is not a canonical IR artifact, not a prior-memory store, and not an approval artifact.

Allowed uses:

- summarize recurring patterns across validation reports, KG fixtures, and reviewed implementation notes
- record source-grounded failure archetypes and negative knowledge
- propose future benchmark fixtures or typed prior candidates
- support debugging and session handoff

Disallowed uses:

- directly mutate `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, or adapters
- smuggle facts from one document into another document's canonical truth
- treat freeform prose as a typed prior without a separate validation-gated promotion path
- replace KG-bench fixtures, validation reports, or typed prior-memory records

## Page Requirements

Every page should state:

- what structural evidence capability it summarizes
- whether any section is auto-refreshed
- which paths, validation reports, fixtures, or implementation notes ground the claims
- what a future extractor may use the page for
- what the page is not allowed to promote by itself

## Managed Blocks

Auto-refreshable sections must use explicit managed markers.
Human-authored synthesis should live outside managed blocks so refreshes do not overwrite review notes.

The current managed block families are:

```text
<!-- corpus_kb_validation_findings:start -->
<!-- corpus_kb_validation_findings:end -->
```

```text
<!-- corpus_kb_kg_fixtures:start -->
<!-- corpus_kb_kg_fixtures:end -->
```

```text
<!-- corpus_kb_kg_fixture_family:start -->
<!-- corpus_kb_kg_fixture_family:end -->
```

```text
<!-- corpus_kb_prior_candidates:start -->
<!-- corpus_kb_prior_candidates:end -->
```

Prior-candidate managed rows must remain non-promoting.
They can name a target `CorpusMemory` schema and required gates, but they must also preserve explicit non-mutation state for canonical IR and corpus memory until a separate validated promotion workflow exists.
If a prior-candidate block includes a gate matrix, the matrix is a capability-level review checklist only.
It may name visible schema, fixture, harvest, and consumer surfaces, but it must still preserve
`review_scope: capability_surface_not_individual_prior` and a non-mutation promotion boundary.
The sibling `prior_candidates/kg-fixture-candidates.json` manifest is the machine-readable form of that review index.
It may expose fixture counts, readiness labels, gate identifiers, and fixture names for future tooling, but it must keep `canonical_mutation_allowed: false`, `corpus_memory_mutation_allowed: false`, and `promotion_boundary: review_only_no_corpus_memory_or_canonical_ir_mutation`.
Readiness labels such as `prior_and_control_surfaces_present` or `caution_surface_review_ready` mean the typed
fixture surface is reviewable; they do not approve any concrete prior record. Schema 2 names
`prior_present_fixtures` and `control_fixtures` explicitly. It does not infer “gold,” “negative,” or any other
semantic class from a fixture name.

Typed prior-memory capability pages are also non-promoting.
They can summarize KG fixtures that stage or exercise local `CorpusMemory`, but they are not `generated/prior_memory/corpus_memory.json` and cannot add, edit, approve, or delete machine-usable priors.

## Promotion Policy

Corpus KB observations can become machine-usable only by moving through a stricter surface:

- a typed `CorpusMemory` prior family with bounded consumers
- a KG-bench fixture proving safe behavior
- a validation/rescan hint that still requires local current-document evidence
- an explicitly designed future approval artifact for canonical mutation, if such a workflow is ever introduced

Until then, corpus KB content is guidance and synthesis, not truth promotion.
