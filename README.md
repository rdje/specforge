# SpecForge
This file is the single entry point for the project.
Use it first for the project objective, document navigation, and the current implementation map.

## Project objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from protocol, component, and system specifications
- make the canonical deliverable a backend-independent `IntentIR`, serialized as JSON or a future equivalent interchange format
- treat `.fsm`, SystemVerilog, Verilog, and VHDL as adapter targets downstream of `IntentIR`, not as the core product boundary
- push automation as far as safely possible, while representing unresolved ambiguity as structured residual decision packets instead of ad hoc manual gaps
- preserve crash-safe continuity through live documentation so a new AI or LLM session can resume work quickly and correctly

## Current repository state
- the live-document surface has been pivoted around `IntentIR` as the canonical endpoint
- the Rust workspace and active CLI/crate identity are now `specforge`
- the current `specforge` CLI surface supports:
  - `inspect <path>`
  - `ingest <source> --dry-run`
  - `ingest <source>`
- `specforge ingest` now computes and materializes `SourceIR` at `generated/source_ir/<document_key>/source_ir.json`
- the `SourceIR` schema now reserves:
  - parser-backend identity
  - page-artifact manifests
  - visual-asset manifests
  - placeholder bindings for normalized sources
- PDF normalization is now explicitly treated as structured source capture with page images, figure/table assets, captions, and markdown as a convenience view rather than the sole system of record
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapter planning
- the next implementation milestone is to close the remaining `SourceIR` gap for structured PDF normalization and then build the first real multimodal `EvidenceIR` extraction pass

## Working naming
- repository / project / CLI / crate name: `specforge`
- canonical output: `IntentIR`
- adapter targets:
  - `.fsm`
  - SystemVerilog
  - Verilog
  - VHDL

## Fast ramp-up order
1. `README.md`
2. `SESSION_BOOTSTRAP.md`
3. `INTENTIR_SPEC.md`
4. `ROADMAP.md`
5. `LIVE_ACHIEVEMENT_STATUS.md`
6. `RUST_CODEBASE_ANALYSIS.md`
7. `USER_GUIDE.md`
8. `DEVELOPMENT_NOTES.md`
9. `CHANGES.md`
10. `MEMORY.md`
11. `COMMIT.md`

## Documentation index
- `README.md`
  - single project entry point and navigation hub
- `SESSION_BOOTSTRAP.md`
  - exact fresh-session instruction for a new AI or LLM instance
- `INTENTIR_SPEC.md`
  - canonical product and stage specification for `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- `ROADMAP.md`
  - live roadmap for project objectives, sequencing, and remaining work
- `LIVE_ACHIEVEMENT_STATUS.md`
  - authoritative live progress snapshot using the project status vocabulary
- `RUST_CODEBASE_ANALYSIS.md`
  - live deep-dive analysis of the Rust codebase and its architecture
- `USER_GUIDE.md`
  - end-user oriented guide to how the tool is expected to work
- `DEVELOPMENT_NOTES.md`
  - engineering rationale, design choices, and implementation context
- `CHANGES.md`
  - full detailed summary of the current set of changes
- `MEMORY.md`
  - compact but actionable continuity record for crash/session-loss recovery
- `COMMIT.md`
  - exact commit workflow and commit-time reporting requirements

## Project file and directory map
### Current workflow and documentation paths
- `README.md`
- `SESSION_BOOTSTRAP.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `USER_GUIDE.md`
- `DEVELOPMENT_NOTES.md`
- `CHANGES.md`
- `MEMORY.md`
- `COMMIT.md`
- `.gitignore`

### Local-only workflow paths
- `git_message_brief.txt`
  - short commit-message file used by the commit workflow
  - must remain untracked
- `questions_keep_untracked.txt`
  - local user backlog/questions scratch file
  - must remain untracked
- `trace.log`
  - optional trace/log output
  - must remain untracked
- `target/`
  - Rust build output
  - must remain untracked

### Current Rust implementation paths
- `Cargo.toml`
  - root Rust workspace manifest
- `Cargo.lock`
  - dependency lockfile
- `crates/specforge/Cargo.toml`
  - active CLI crate manifest
- `crates/specforge/src/main.rs`
  - binary entrypoint
- `crates/specforge/src/lib.rs`
  - top-level command dispatch
- `crates/specforge/src/cli.rs`
  - clap-based CLI model
- `crates/specforge/src/error.rs`
  - typed error boundary
- `crates/specforge/src/commands/inspect.rs`
  - source/path inspection command
- `crates/specforge/src/commands/ingest.rs`
  - `SourceIR` preview/materialization command
- `crates/specforge/src/ir/mod.rs`
  - staged IR namespace and stage identifiers
- `crates/specforge/src/ir/source.rs`
  - `SourceIR` types, parser-backend selection, page/visual artifact planning, and ingest-side residual decisions
- `crates/specforge/src/ir/evidence.rs`
  - multimodal `EvidenceIR` scaffolding for text, figures, captions, and visual evidence
- `crates/specforge/src/ir/semantic.rs`
  - `SemanticIR` scaffolding
- `crates/specforge/src/ir/intent.rs`
  - `IntentIR` scaffolding
- `crates/specforge/src/ir/adapters.rs`
  - adapter targets and planning scaffolding

### Planned future implementation paths
- `fixtures/`
  - sample specifications, PDFs, markdown conversions, and expected IR snapshots
- `examples/`
  - example invocations and example stage outputs
- `generated/`
  - generated IR artifacts only when intentionally versioned

## Quick start
```bash
cargo test
cargo run -p specforge -- --help
cargo run -p specforge -- inspect README.md
cargo run -p specforge -- ingest README.md --dry-run
```
- `specforge ingest <source> --dry-run` prints computed `SourceIR` JSON without writing artifacts
- `specforge ingest <source>` materializes `generated/source_ir/<document_key>/source_ir.json`

## Planned product shape
- stage 0: build `SourceIR` from raw sources, normalized text views, structured page artifacts, and extracted visual assets
- stage 1: build `EvidenceIR` from normalized text, section anchors, evidence spans, figure/caption links, visual evidence, and statement extraction
- stage 2: build `SemanticIR` from actors, interfaces, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- stage 3: build `IntentIR` as the canonical backend-independent intent model
- stage 4: lower `IntentIR` through adapters such as `.fsm`, SystemVerilog, Verilog, and VHDL
- stage 5: validate adapters and back-annotate findings into the IR/documentation surface

## Key operating principles
- `IntentIR` is the canonical endpoint
- `.fsm` is an adapter target, not the core endpoint
- the pipeline is explicit: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- automation-first, manual-last
- actor-first extraction
- typed IR over string-based generation
- deterministic steps where possible
- structured document parsing first, selective multimodal enrichment second
- LLM assistance where interpretation is required
- markdown is a lossy convenience view for PDFs, not the only normalized representation
- residual decision packets for irreducible ambiguity
- live documentation as critical continuity infrastructure

Read SESSION_BOOTSTRAP.md and start from there.
