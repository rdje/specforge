# Pipeline Model

This chapter explains the four canonical IR stages in increasing semantic depth.

## Stage model

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

The design goal is not to "parse English" directly. The design goal is to build a typed protocol-world model and feed it with grounded multimodal evidence.

The current program constraint is getting that evidence faithfully from PDFs into every required canonical
IR surface. ISF remains the executable adapter boundary, but adapter expansion is evidence-triggered: the
project first proves that a source-grounded semantic reached `IntentIR`, then extends ISF/FSMGen only if the
existing target cannot represent it. See [Trajectory And Automatic Steering](../quality/trajectory.md).

The chapters that follow should be read as stage boundaries, not just stage names.
Each stage has a different job, a different truthfulness contract, and a different kind of allowed decision-making.

## What changes from stage to stage

### `SourceIR`

Preserve document structure and normalization output.

### `EvidenceIR`

Lift grounded evidence out of the document.

### `SemanticIR`

Assemble typed domain meaning from the evidence and preserve ambiguity honestly.

### `IntentIR`

Canonicalize the semantically grounded result into the main backend-independent product surface.

## Why there are multiple stages

Because different kinds of truth belong in different places:

- source truth
- extracted evidence
- semantic interpretation
- canonical intent

Flattening those into one pass would make the tool less inspectable, less debuggable, and much more likely to hallucinate.

## What the pipeline is optimizing for

- grounded structure
- explicit conflicts
- explicit residual decisions
- canonical typed records
- backend independence

## The most important boundary

The pipeline is intentionally arranged so that later stages do not need to guess what earlier stages should have preserved.

That means:

- `SourceIR` should preserve document structure
- `EvidenceIR` should preserve grounded extracted evidence
- multimodal evidence should preserve figure, caption, and VLM observation grounding without bypassing the staged pipeline
- `SemanticIR` should preserve typed meaning plus honest conflict surfaces
- `IntentIR` should preserve the best stable canonical result without erasing uncertainty

That staged discipline is one of the core reasons `specforge` exists in this form at all.

The book chapters that follow explain each stage separately.
