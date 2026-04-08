# Pipeline Model

This chapter explains the four canonical IR stages in increasing semantic depth.

## Stage model

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

The design goal is not to "parse English" directly. The design goal is to build a typed protocol-world model and feed it with grounded multimodal evidence.

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

The book chapters that follow explain each stage separately.

