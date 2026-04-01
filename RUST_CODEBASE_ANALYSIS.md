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
- the implemented stage today is `SourceIR`
- `SourceIR` now reserves parser-backend, page-artifact, and visual-asset schema surface for multimodal source capture
- `EvidenceIR` now reserves typed multimodal evidence records instead of assuming text-only evidence
- the later stages currently exist as typed scaffolding, which is valuable for architectural clarity but still needs real builders
- the next slice should turn the architecture from declared stages into executed stages by closing the remaining `SourceIR` structured-PDF-normalization gap and constructing the first real visual-aware `EvidenceIR`

## Observed current state
### Repository contents directly observed
- `.git/`
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
- `crates/specforge/src/ir/mod.rs`
- `crates/specforge/src/ir/source.rs`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/intent.rs`
- `crates/specforge/src/ir/adapters.rs`

### Rust-specific contents still absent
- no dedicated `specforge-source` crate
- no dedicated `specforge-evidence` crate
- no dedicated `specforge-semantic` crate
- no dedicated `specforge-intent` crate
- no dedicated `specforge-adapters` crate
- no dedicated validation crate
- no integration-test harness beyond crate-local unit tests
- no real builders beyond the current `SourceIR` slice

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
- `src/ir/mod.rs`
  - stage identifiers for `source_ir`, `evidence_ir`, `semantic_ir`, and `intent_ir`
- `src/ir/source.rs`
  - concrete `SourceIR` implementation
- `src/ir/evidence.rs`
  - `EvidenceIR` scaffolding
- `src/ir/semantic.rs`
  - `SemanticIR` scaffolding
- `src/ir/intent.rs`
  - `IntentIR` scaffolding
- `src/ir/adapters.rs`
  - adapter targets and adapter-plan scaffolding

## Assessment of current structure
### What is good
- the crate/binary identity now matches the repo direction
- the code no longer hardcodes `.fsm` as the conceptual endpoint
- a real typed `SourceIR` artifact exists instead of a handwritten ingest plan
- the later stages have typed names and module homes, which reduces the risk of accidental backend-first growth
- adapter planning is separated from the canonical IR stages
- the IR surface now makes room for structured PDF assets and multimodal visual evidence before the real builder exists

### What is still insufficient
- only `SourceIR` has a real builder today
- structured PDF normalization is still missing inside `SourceIR`
- page-artifact and visual-asset manifests are still planned rather than materialized
- `EvidenceIR`, `SemanticIR`, and `IntentIR` remain type scaffolding rather than executable stage builders
- visual enrichment and figure-grounding are not implemented yet
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
- `SourceIR` structured PDF normalization orchestration
- first visual-aware `EvidenceIR` extractor
- initial statement classification into source facts / derived rules / local design decisions / explicit abstractions

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
  - `src/ir/evidence.rs`
- next real implementation target:
  - build a real extractor from normalized markdown, figures, captions, and page/asset manifests

### SemanticIR
- current declared ownership:
  - `src/ir/semantic.rs`
- dependency:
  - requires real `EvidenceIR`

### IntentIR
- current declared ownership:
  - `src/ir/intent.rs`
- dependency:
  - requires real `SemanticIR`

### Adapters
- current declared ownership:
  - `src/ir/adapters.rs`
- dependency:
  - requires stable `IntentIR`

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

### Risk: SourceIR overpromises on PDFs
- today `SourceIR` records planned promoted markdown, page-artifact, and visual-asset paths for PDFs, but real converter orchestration is still absent
- this is acceptable temporarily, but should be closed soon so the first stage is truly operational for PDFs

## Testing implications
- current tests cover:
  - source-kind detection
  - deterministic source key naming
  - `SourceIR` JSON materialization
  - directory residual decision emission
  - PDF normalization planning
- next tests should cover:
  - structured PDF normalization and failure handling
  - first `EvidenceIR` extraction on normalized markdown plus visual-asset fixtures
  - figure extraction and caption-linking fidelity
  - visual evidence grounding and confidence propagation
  - provenance retention across stage boundaries
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
- repo-wide stale-name sweep
  - remaining `spec2fsm` references are historical notes only, not active CLI or architecture surfaces

## Current recommendation
- keep the current single-crate workspace for one more slice
- next, close the remaining `SourceIR` gap by orchestrating structured PDF normalization
- immediately after that, build the first real visual-aware `EvidenceIR` extractor
- keep `IntentIR` canonical and resist any temptation to make `.fsm` the hidden endpoint again
