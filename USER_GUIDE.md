# USER_GUIDE
## Purpose
- explain what `specforge` is intended to do from an end-user perspective
- document the expected staged workflow as the tool evolves toward canonical `IntentIR`

## What SpecForge is intended to become
- a staged Rust tool that turns protocol, component, and system specifications into backend-independent `IntentIR`
- a system where backend formats are adapters, not endpoints
- an automation-first workflow that keeps ambiguity explicit through residual decision packets

## Current state
- the repository contains workflow and continuity documentation plus a runnable Rust CLI named `specforge`
- the current CLI already supports:
  - `specforge inspect <path>`
  - `specforge ingest <source> --dry-run`
  - `specforge ingest <source>`
- the currently implemented stage is `SourceIR`
- `SourceIR` now reserves parser backend, page-artifact, and visual-asset fields so PDF ingestion can remain multimodal rather than markdown-only
- `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter planning now have typed scaffolding but not real builders yet

## Available commands today
### Inspect a path
```bash
cargo run -p specforge -- inspect README.md
```
- reports:
  - canonical path
  - path kind
  - detected source kind
  - extension
  - file size for regular files

### Preview a SourceIR artifact
```bash
cargo run -p specforge -- ingest README.md --dry-run
```
- prints computed `SourceIR` JSON without writing artifacts
- useful for checking:
  - source identity
  - normalization planning
  - downstream staged IR intent
  - residual decision packets for ambiguous sources
  - planned adapter targets

### Materialize a SourceIR artifact
```bash
cargo run -p specforge -- ingest README.md
```
- writes `generated/source_ir/<document_key>/source_ir.json`
- this is the first real IR artifact emitted by the tool
- for Markdown inputs, the normalization plan points at the existing Markdown source
- for PDF inputs, the artifact records the planned parser backend plus promoted markdown, page-artifact, metadata, and visual-asset layout even though converter execution is still a later step

## Planned user workflow
1. provide a source specification
2. build `SourceIR`
3. build `EvidenceIR` from text, figures, captions, charts, and other grounded evidence
4. build `SemanticIR`
5. build canonical `IntentIR`
6. lower `IntentIR` through an adapter such as `.fsm` or RTL
7. validate the result and back-annotate findings

## Planned command shape
- `specforge ingest <source>`
- `specforge inspect <artifact-or-path>`
- `specforge evidence <source-ir>`
- `specforge semantic <evidence-ir>`
- `specforge intent <semantic-ir>`
- `specforge adapt <intent-ir> --target <fsm|systemverilog|verilog|vhdl>`
- `specforge validate <artifact>`

## Expected user-visible principles
- the tool should be staged and inspectable
- intermediate IR artifacts should be preserved, not hidden
- evidence and provenance should be visible
- images, figures, captions, and charts should stay available as first-class evidence when they carry semantic value
- unresolved ambiguity should be emitted as residual decision packets instead of being hidden in ad hoc notes
- `IntentIR` should remain backend-independent
- `.fsm` is only one adapter target among several

## Current limitation
- `SourceIR` is implemented
- structured PDF normalization and multimodal asset extraction are not implemented yet
- `EvidenceIR`, `SemanticIR`, and `IntentIR` builders are not implemented yet
- adapters are planned but not implemented
- validation/back-annotation is not implemented yet

## Where to look next
- `README.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
