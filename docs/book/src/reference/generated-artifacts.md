# Generated Artifacts

This chapter is the public map of what `specforge` writes and where those files fit in the project.

The most important rule is simple:

`generated/` is local execution state, not source code.

That rule is intentional. Real chip PDFs can produce large intermediate artifacts, and those artifacts can change whenever extraction, enrichment, validation, or learning logic improves.

## Generated artifact roots

- `generated/source_ir/<document_key>/source_ir.json`
- `generated/evidence_ir/<document_key>/evidence_ir.json`
- `generated/semantic_ir/<document_key>/semantic_ir.json`
- `generated/intent_ir/<document_key>/intent_ir.json`
- `generated/adapters/fsm/<document_key>/adapter.json`
- `generated/prior_memory/corpus_memory.json`

## Stage artifacts

The four main artifact roots mirror the four pipeline stages:

- `generated/source_ir/<document_key>/source_ir.json`
- `generated/evidence_ir/<document_key>/evidence_ir.json`
- `generated/semantic_ir/<document_key>/semantic_ir.json`
- `generated/intent_ir/<document_key>/intent_ir.json`

The `document_key` is the stable local key `specforge` derives for the input document.
For a PDF run, that key usually comes from the normalized file stem.

These artifacts are useful for different kinds of inspection:

- inspect `SourceIR` when table, section, page, figure, or backend normalization looks wrong
- inspect `EvidenceIR` when signal declarations, extracted statements, visual evidence, semantic hints, polarity evidence, or early KG relations look wrong
- inspect `SemanticIR` when actors, ports, connectivity, temporal rules, arbitration, conflicts, or residuals look wrong
- inspect `IntentIR` when you want the canonical product surface that validators, adapters, and prior learning consume

## Source-side sidecars

PDF ingest may also materialize source-side sidecars under the same document root.
Depending on the input and backend path, these can include:

- promoted markdown
- page metadata
- page images
- visual asset crops
- normalized backend dumps
- backend diagnostics

Those files are part of the local evidence trail.
They are useful for debugging and visual grounding, but they should not be treated as hand-authored project assets.

## Validation reports

`validate` writes deterministic validation reports next to the artifact being validated.
For example, validating an `IntentIR` artifact creates or refreshes the corresponding validation report for that stage.

The reports are useful because they separate:

- artifact content
- deterministic quality assessment
- tracked project-level projection

That separation matters.
An artifact can improve without the project snapshot being refreshed until `project-validation` is run.

## Adapter artifacts

Adapter output lives under:

- `generated/adapters/fsm/<document_key>/adapter.json`

Adapter artifacts are downstream products of `IntentIR`.
They should not define what the canonical document meaning is.
If an adapter needs meaning that is not present in `IntentIR`, the fix should usually be upstream in the IR pipeline, not hidden inside the adapter.

## Prior memory

The cross-document learning plane writes:

- `generated/prior_memory/corpus_memory.json`

That file is the symbolic memory that grows over time as validated artifacts teach `specforge` reusable extraction priors.
It can contain actor-taxonomy, semantic-phrase, semantic-modality-reliability, temporal-phrase, table-shape, visual-motif, and negative-knowledge priors.

It is still local generated state.
The code defines how learning works; `corpus_memory.json` stores what the local workspace has learned so far.

## Why generated artifacts are not tracked

`generated/` stays local and untracked.

That rule matters because:

- generated artifacts can be large
- they change often
- they can include backend dumps that exceed hosted Git limits
- they are reproducible enough to regenerate when the runtime is available
- continuity should live in stable tracked docs, not in versioned generated blobs

There are exceptions only when the project deliberately creates small tracked fixtures under test data.
Those fixtures are curated regression inputs and expected outputs, not arbitrary live pipeline output.

## Important tracked docs

Tracked docs are not generated artifacts.
They are continuity and steering surfaces.

The most important user-visible ones are:

- `README.md`
- `VALIDATION_SNAPSHOT.md`
- `LIVE_ACHIEVEMENT_STATUS.md`

The more development-centric ones are described in the next chapter.

## How to inspect an artifact

Most artifacts are JSON.
The easiest path is to open the relevant stage file directly and compare it with the validation report for the same document.

When debugging, prefer following the pipeline backward:

- an `IntentIR` issue often comes from `SemanticIR`
- a `SemanticIR` issue often comes from `EvidenceIR`
- an `EvidenceIR` issue often comes from `SourceIR`
- a `SourceIR` issue often comes from ingest/backend normalization

That staged backward trace is one of the main reasons the project preserves intermediate artifacts at all.
