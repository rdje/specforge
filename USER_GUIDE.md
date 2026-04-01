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
  - `specforge evidence <source-ir> --dry-run`
  - `specforge evidence <source-ir>`
  - `specforge semantic <evidence-ir> --dry-run`
  - `specforge semantic <evidence-ir>`
- the currently implemented executable stages are `SourceIR`, `EvidenceIR`, and `SemanticIR`
- `SourceIR` now handles existing Markdown directly and performs Docling-backed structured PDF normalization for PDF inputs
- `EvidenceIR` now consumes ready `SourceIR` artifacts and extracts section anchors, evidence spans, visual evidence, figure/caption links, and heuristic statement classes
- `SemanticIR` now consumes ready `EvidenceIR` artifacts and lifts heuristic actors, interfaces, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions
- `IntentIR` and adapter lowering remain planned

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
- this is the first stage artifact emitted by the tool
- for Markdown inputs, the normalization plan points at the existing Markdown source
- for PDF inputs, execute mode now materializes:
  - promoted markdown
  - page images and page metadata sidecars
  - visual asset crops for pictures and tables
  - metadata JSON and backend raw JSON
  - `page_artifacts.json` and `visual_assets.json`
- PDF execute mode expects `docling` to be importable from `python3` or `python`; when needed, point `SPECFORGE_DOCLING_PYTHON` at the correct interpreter

### Preview an EvidenceIR artifact
```bash
cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run
```
- prints computed `EvidenceIR` JSON without writing artifacts
- requires a `SourceIR` whose normalization status is `ready`
- useful for checking:
  - section anchors and line provenance
  - evidence spans and extracted statements
  - figure/caption linkage
  - visual evidence counts before materialization

### Materialize an EvidenceIR artifact
```bash
cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json
```
- writes `generated/evidence_ir/<document_key>/evidence_ir.json`
- builds:
  - section anchors from promoted markdown headings
  - block-level evidence spans with line provenance
  - visual evidence items from `SourceIR` visual assets
  - explicit `describes` and `cites` links for caption and figure/table references
  - extracted statements classified into source facts, derived rules, local design decisions, or explicit abstractions

### Preview a SemanticIR artifact
```bash
cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run
```
- prints computed `SemanticIR` JSON without writing artifacts
- requires an `EvidenceIR` JSON artifact
- useful for checking:
  - actor and interface discovery
  - phase, invariant, and gate extraction
  - abstraction and decomposition candidate creation
  - semantic residual decisions before materialization

### Materialize a SemanticIR artifact
```bash
cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json
```
- writes `generated/semantic_ir/<document_key>/semantic_ir.json`
- builds:
  - actors from role-like evidence terms or inferred channel groupings
  - interfaces from recurring grouped signal names
  - phases from section structure and sequencing language
  - invariants, contracts, and gates from heuristic semantic lifting
  - abstractions and decomposition candidates with supporting statement ids
  - residual decisions when actor boundaries, interface grouping, or visual semantics remain ambiguous

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
- `SourceIR`, the first real `EvidenceIR` pass, and the first real `SemanticIR` pass are implemented
- the current `EvidenceIR` extraction logic is still heuristic and does not yet perform deeper OCR, chart extraction, or semantic lifting from visual regions
- the current `SemanticIR` extraction logic is still heuristic and conservative, so later `IntentIR` work will need refinement rather than semantic invention
- `IntentIR` builder is still not implemented
- adapters are planned but not implemented
- validation/back-annotation is not implemented yet

## Where to look next
- `README.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
