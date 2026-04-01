# RUST_CODEBASE_ANALYSIS
## Purpose
- maintain a live, deep-dive analysis of the Rust codebase
- record the current architecture, risks, subsystem boundaries, and recommended implementation direction
- remain useful even while only the early IR stages are implemented

## Executive summary
- the repository now contains a renamed `specforge` crate and CLI rather than the older `spec2fsm` identity
- the canonical product boundary is now `IntentIR`, not `.fsm`
- the codebase has been reshaped around an explicit staged IR pipeline:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters
- the implemented executable stages today are `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
- `SourceIR` now has a real Docling-backed PDF materialization path that emits promoted markdown, page artifacts, visual assets, metadata JSON, and backend raw JSON
- `EvidenceIR` now has a real builder that emits typed multimodal evidence records instead of remaining text-only scaffolding
- `SemanticIR` now has a real builder that lifts grounded evidence into inspectable semantic records and residual decisions
- `IntentIR` now has a real builder that canonicalizes semantic records into inspectable backend-neutral intent artifacts
- the later stages currently exist as typed scaffolding, which is valuable for architectural clarity but still needs real builders
- the next slice should lower the now-materialized `IntentIR` outputs through the first real adapter

## Observed current state
### Repository contents directly observed
- `.git/`
- `.gitmodules`
- live documentation surface
- `INTENTIR_SPEC.md`
- `Cargo.toml`
- `Cargo.lock`
- `crates/specforge/Cargo.toml`
- `crates/specforge/src/main.rs`
- `crates/specforge/src/lib.rs`
- `crates/specforge/src/cli.rs`
- `crates/specforge/src/error.rs`
- `crates/specforge/src/commands/inspect.rs`
- `crates/specforge/src/commands/ingest.rs`
- `crates/specforge/src/commands/evidence.rs`
- `crates/specforge/src/commands/semantic.rs`
- `crates/specforge/src/commands/intent.rs`
- `crates/specforge/src/ir/mod.rs`
- `crates/specforge/src/ir/source.rs`
- `crates/specforge/src/ir/source/docling_backend.rs`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/intent.rs`
- `crates/specforge/src/ir/adapters.rs`
- `subs/fsmgen/`

### Rust-specific contents still absent
- no dedicated `specforge-source` crate
- no dedicated `specforge-evidence` crate
- no dedicated `specforge-semantic` crate
- no dedicated `specforge-intent` crate
- no dedicated `specforge-adapters` crate
- no dedicated validation crate
- no integration-test harness beyond crate-local unit tests
- no real builders beyond the current `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR` slices

### Immediate implication
- the codebase now has a clean top-level architectural story
- the current risk is no longer naming confusion; it is execution lag between the declared staged architecture and the still-limited implemented builders

## What the tool needs to do
- build `SourceIR` from raw specifications and normalized artifacts
- build `EvidenceIR` from normalized markdown, page assets, figures, captions, and evidence extraction
- build `SemanticIR` from actors, interfaces, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- build canonical `IntentIR` as a backend-independent intent model
- lower `IntentIR` through adapters such as `.fsm`, SystemVerilog, Verilog, and VHDL
- validate stage outputs and adapter outputs and back-annotate findings

## Current implemented architecture
### Root workspace
- `Cargo.toml`
  - workspace root
- `Cargo.lock`
  - dependency lockfile

### Active crate
- `crates/specforge`
  - single user-facing CLI crate and binary for the current slice

### Implemented module boundaries
- `src/main.rs`
  - binary entrypoint
- `src/lib.rs`
  - command dispatch and public module exports
- `src/cli.rs`
  - clap-based command model using the `specforge` binary name
- `src/error.rs`
  - typed error/result boundary
- `src/commands/inspect.rs`
  - deterministic source/path inspection command
- `src/commands/ingest.rs`
  - `SourceIR` preview/materialization command
- `src/commands/evidence.rs`
  - `EvidenceIR` preview/materialization command
- `src/commands/semantic.rs`
  - `SemanticIR` preview/materialization command
- `src/commands/intent.rs`
  - `IntentIR` preview/materialization command
- `src/ir/mod.rs`
  - stage identifiers for `source_ir`, `evidence_ir`, `semantic_ir`, and `intent_ir`
- `src/ir/source.rs`
  - concrete `SourceIR` implementation and PDF materialization lifecycle
- `src/ir/source/docling_backend.rs`
  - external backend discovery, Docling command orchestration, and the embedded Python helper for structured PDF normalization
- `src/ir/evidence.rs`
  - concrete `EvidenceIR` builder, markdown parsing, caption/reference linking, and extraction heuristics
- `src/ir/semantic.rs`
  - concrete `SemanticIR` builder, semantic lifting heuristics, and residual-decision generation
- `src/ir/intent.rs`
  - concrete `IntentIR` builder, canonicalization heuristics, and residual-decision preservation
- `src/ir/adapters.rs`
  - adapter targets and adapter-plan scaffolding

## Assessment of current structure
### What is good
- the crate/binary identity now matches the repo direction
- the code no longer hardcodes `.fsm` as the conceptual endpoint
- a real typed `SourceIR` artifact exists instead of a handwritten ingest plan
- a real structured PDF normalization path now exists inside `SourceIR`, so the first stage is operational for both Markdown and PDF inputs
- a real typed `EvidenceIR` artifact now exists, so the staged pipeline is operational beyond raw source normalization
- a real typed `SemanticIR` artifact now exists, so the staged pipeline now reaches a backend-neutral semantic layer before the final canonicalization stage
- a real typed `IntentIR` artifact now exists, so the end-to-end source-to-intent pipeline is operational before adapter lowering
- the later stages have typed names and module homes, which reduces the risk of accidental backend-first growth
- adapter planning is separated from the canonical IR stages
- the IR surface now carries page and visual manifests plus backend source references that later stages can ground against
- the repository now also contains a pinned local `fsmgen` checkout, which gives the next `.fsm` adapter slice a nearby reference implementation without changing the canonical `IntentIR` boundary

### What is still insufficient
- only `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR` have real builders today
- deeper visual enrichment beyond caption/reference grounding is not implemented yet
- there is no real adapter implementation yet
- validation/back-annotation is still absent

## Architectural recommendation
### Core architectural stance
- keep `IntentIR` as the canonical endpoint
- keep adapters downstream of `IntentIR`
- keep the internal system of record typed and stage-specific
- keep residual decisions explicit at every stage
- do not let convenience around one backend contaminate the stage-neutral model
- use structured parsing first and selective multimodal enrichment second, rather than collapsing the problem into markdown-only OCR or ungrounded VLM generation

### Recommended growth path from the current codebase
#### Keep in the current crate for one more slice
- first real adapter lowering from grounded `IntentIR`
- initial target-specific structure emission while preserving the canonical model boundary

#### Split into dedicated crates when pressure becomes real
- `specforge-source`
  - source registration, normalization, converter orchestration
- `specforge-evidence`
  - section anchors, evidence spans, statement extraction and provenance
- `specforge-semantic`
  - actor and semantic lifting
- `specforge-intent`
  - canonical intent model and versioned serialization
- `specforge-adapters`
  - target-specific lowerings
- `specforge-validate`
  - validation, diagnostics, back-annotation

## Mapping from staged architecture to the current modules
### SourceIR
- current primary ownership:
  - `src/commands/ingest.rs`
  - `src/ir/source.rs`

### EvidenceIR
- current declared ownership:
  - `src/commands/evidence.rs`
  - `src/ir/evidence.rs`
- current executable behavior:
  - builds `EvidenceIR` from ready `SourceIR`, promoted markdown, and visual-asset manifests

### SemanticIR
- current declared ownership:
  - `src/commands/semantic.rs`
  - `src/ir/semantic.rs`
- dependency:
  - requires real `EvidenceIR`
- current executable behavior:
  - builds `SemanticIR` from persisted `EvidenceIR`, deriving actors, interfaces, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions

### IntentIR
- current declared ownership:
  - `src/commands/intent.rs`
  - `src/ir/intent.rs`
- dependency:
  - requires real `SemanticIR`
- current executable behavior:
  - builds `IntentIR` from persisted `SemanticIR`, deriving intent identity, actor responsibilities, behaviors, constraints, assumptions, and residual decisions

### Adapters
- current declared ownership:
  - `src/ir/adapters.rs`
- dependency:
  - requires stable `IntentIR`
- nearby reference implementation:
  - `subs/fsmgen/`
- next real implementation target:
  - lower canonical `IntentIR` into the first concrete backend target without leaking adapter assumptions backward

## Major risks
### Risk: backend leakage into IntentIR
- if `.fsm` or RTL-specific assumptions creep back into the canonical model, the pivot fails even if the names remain correct

### Risk: stage scaffolding without stage execution
- if `EvidenceIR`, `SemanticIR`, and `IntentIR` remain only structs for too long, the architecture becomes performative rather than operational

### Risk: incomplete provenance in EvidenceIR
- if evidence ranges are not carried forward precisely, later semantic lifting and validation will be fragile

### Risk: markdown-only drift for PDFs
- if the real builder treats markdown as the only normalized representation, the system will silently lose figure, chart, and layout semantics before `EvidenceIR`

### Risk: ungrounded visual descriptions
- if multimodal descriptions are generated without stable links back to page regions, captions, and source references, later stages will be vulnerable to hallucinated evidence
### Risk: mixed Rust/Python backend seam
- the SourceIR PDF path now depends on a Rust-to-Python orchestration boundary and an external Docling runtime
- interpreter discovery, package installation, and first-run model downloads are operational concerns that must stay explicit in docs and tests
- this is acceptable temporarily, but should be closed soon so the first stage is truly operational for PDFs

## Testing implications
- current tests cover:
  - source-kind detection
  - deterministic source key naming
  - `SourceIR` JSON materialization
  - directory residual decision emission
  - PDF normalization planning
  - PDF materialization through a stubbed backend override that exercises the manifest-writing path
  - markdown-backed `EvidenceIR` construction
  - caption and figure-reference grounding into visual evidence
  - handshake-driven `SemanticIR` actor/interface/invariant extraction
  - ambiguous visual-grounding residual decisions in `SemanticIR`
  - handshake-driven `IntentIR` identity/behavior/constraint/assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
- next tests should cover:
  - structured PDF normalization failure handling against missing runtimes and malformed backend output
  - additional `EvidenceIR` extraction on richer visual-asset fixtures
  - figure extraction and caption-linking fidelity across ambiguous numbering cases
  - visual evidence grounding and confidence propagation beyond the current heuristic pass
  - provenance retention across stage boundaries
  - richer `SemanticIR` snapshots and residual-decision coverage on protocol-heavy fixtures
  - richer `IntentIR` snapshots and canonicalization coverage on protocol-heavy fixtures
  - first adapter snapshots and lowering coverage
  - adapter planning and eventual adapter lowering snapshots

## Validation completed in this session
- `cargo fmt --all --manifest-path Cargo.toml`
  - passed after the multimodal `SourceIR` and `EvidenceIR` schema update
- `cargo test`
  - passed for the renamed `specforge` crate and current staged IR modules
- `cargo run -p specforge -- --help`
  - passed and reports the staged `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter direction
- `cargo run -p specforge -- ingest README.md --dry-run`
  - passed and emits `SourceIR` JSON with parser backend, page-artifact manifests, visual-asset manifests, placeholder bindings, and downstream stage planning fields
- `cargo run -p specforge -- ingest README.md`
  - passed after the PDF materialization changes, confirming the markdown execute path still behaves correctly
- `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - passed and materialized a real PDF `SourceIR` with 9 page artifacts, 11 visual assets, promoted markdown, metadata JSON, backend raw JSON, and manifest files
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - passed and previewed a markdown-backed `EvidenceIR`
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - passed and materialized a markdown-backed `EvidenceIR` with 14 section anchors, 167 evidence spans, and 167 extracted statements
- `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - passed and materialized a PDF-backed `EvidenceIR` with 18 section anchors, 225 evidence spans, 11 visual evidence items, 19 evidence links, and 225 extracted statements
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - passed and previewed a markdown-backed `SemanticIR`
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - passed and materialized a markdown-backed `SemanticIR` with 2 actors, 4 phases, 3 invariants, 3 gates, 1 abstraction, and 12 decomposition candidates
- `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
  - passed and materialized a PDF-backed `SemanticIR` with 2 actors, 22 interfaces, 7 phases, 17 invariants, 2 contracts, 20 gates, 12 decomposition candidates, and 2 residual decisions
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - passed and previewed a markdown-backed `IntentIR`
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - passed and materialized a markdown-backed `IntentIR` with 2 actors, 7 behaviors, 3 constraints, 1 assumption, and 0 residual decisions
- `cargo run -p specforge -- intent generated/semantic_ir/specforge_docling_sample/semantic_ir.json`
  - passed and materialized a PDF-backed `IntentIR` with 2 actors, 28 behaviors, 24 constraints, 1 assumption, and 2 residual decisions
- repo-wide stale-name sweep
  - remaining `spec2fsm` references are historical notes only, not active CLI or architecture surfaces

## Current recommendation
- keep the current single-crate workspace for one more slice
- next, build the first real adapter lowering pass on top of the newly materialized `IntentIR` artifacts
- keep `IntentIR` canonical and resist any temptation to make `.fsm` the hidden endpoint again
