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

- what evidence family it summarizes
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

## Promotion Policy

Corpus KB observations can become machine-usable only by moving through a stricter surface:

- a typed `CorpusMemory` prior family with bounded consumers
- a KG-bench fixture proving safe behavior
- a validation/rescan hint that still requires local current-document evidence
- an explicitly designed future approval artifact for canonical mutation, if such a workflow is ever introduced

Until then, corpus KB content is guidance and synthesis, not truth promotion.
