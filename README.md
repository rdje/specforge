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
  - `evidence <source-ir> --dry-run`
  - `evidence <source-ir>`
  - `semantic <evidence-ir> --dry-run`
  - `semantic <evidence-ir>`
  - `intent <semantic-ir> --dry-run`
  - `intent <semantic-ir>`
  - `adapt <intent-ir> --target fsm --dry-run`
  - `adapt <intent-ir> --target fsm`
- `specforge ingest` now computes and materializes `SourceIR` at `generated/source_ir/<document_key>/source_ir.json`
- `specforge evidence` now computes and materializes `EvidenceIR` at `generated/evidence_ir/<document_key>/evidence_ir.json`
- `specforge semantic` now computes and materializes `SemanticIR` at `generated/semantic_ir/<document_key>/semantic_ir.json`
- `specforge intent` now computes and materializes `IntentIR` at `generated/intent_ir/<document_key>/intent_ir.json`
- `specforge adapt --target fsm` now computes and materializes typed adapter artifacts at `generated/adapters/fsm/<document_key>/adapter.json`
- a pinned `subs/fsmgen` git submodule now exists as a local `.fsm` reference implementation for upcoming adapter work
- the `SourceIR` schema now reserves:
  - parser-backend identity
  - page-artifact manifests
  - visual-asset manifests
  - placeholder bindings for normalized sources
- PDF normalization is now explicitly treated as structured source capture with page images, figure/table assets, captions, and markdown as a convenience view rather than the sole system of record
- `specforge ingest <pdf>` now performs real Docling-backed structured normalization and materializes promoted markdown, page images, page metadata sidecars, visual assets, metadata JSON, backend raw JSON, and manifest files under `generated/source_ir/<document_key>/normalized`
- the first real `EvidenceIR` extraction pass now builds section anchors, evidence spans, visual evidence items, figure/caption links, and heuristic extracted statements from ready `SourceIR` artifacts
- the first real `SemanticIR` lifting pass now builds actors, interfaces, backend-neutral system/init records, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions from persisted `EvidenceIR` artifacts
- the first real `IntentIR` canonicalization pass now builds intent identity, actor responsibilities, carried interface/control/system/init surface, behaviors, constraints, assumptions, and residual decisions from persisted `SemanticIR` artifacts
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - typed adapter lowering
- the first real `.fsm` adapter slice now materializes a DT-centric adapter artifact, selects a conservative `?dt:name` root, emits explicit standalone combinational and sequential DT text when the canonical facts are fully explicit, and blocks unsafe `.fsm` text emission with explicit residual decisions instead of fabricating target syntax
- the next implementation milestone is to promote explicit state/transition facts for honest `?fsm:name` lowering while keeping composition deferred

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
- `.gitmodules`
  - git submodule manifest for pinned local reference dependencies

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
- `.gitmodules`

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

### Reference adapter/tooling paths
- `subs/fsmgen/`
  - pinned local checkout of `fsmgen`, used as read-only contextual reference while building the first `.fsm` adapter
  - do not modify it from this repository; if upstream misbehavior is discovered, track it locally as a bug report instead

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
- `crates/specforge/src/commands/evidence.rs`
  - `EvidenceIR` preview/materialization command
- `crates/specforge/src/commands/semantic.rs`
  - `SemanticIR` preview/materialization command
- `crates/specforge/src/commands/intent.rs`
  - `IntentIR` preview/materialization command
- `crates/specforge/src/commands/adapt.rs`
  - `.fsm` adapter preview/materialization command for the first target-specific lowering slice
- `crates/specforge/src/ir/mod.rs`
  - staged IR namespace and stage identifiers
- `crates/specforge/src/ir/source.rs`
  - `SourceIR` types, parser-backend selection, page/visual artifact manifests, and ingest-side residual decisions
- `crates/specforge/src/ir/source/docling_backend.rs`
  - Docling backend orchestration and the embedded Python helper that materializes structured PDF artifacts for `SourceIR`
- `crates/specforge/src/ir/evidence.rs`
  - first real multimodal `EvidenceIR` builder for text spans, captions, figure/table references, visual evidence, and extracted statements
- `crates/specforge/src/ir/semantic.rs`
  - first real `SemanticIR` builder for actors, interfaces, typed signal records, backend-neutral control fragments, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions
- `crates/specforge/src/ir/intent.rs`
  - first real `IntentIR` builder for canonical intent identity, actor responsibilities, carried interface inventory, carried backend-neutral control fragments, behaviors, constraints, assumptions, and residual decisions
- `crates/specforge/src/ir/adapters.rs`
  - typed adapter artifacts, DT-centric `.fsm` lowering logic, safe standalone renderability analysis, and adapter-side residual decisions

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
cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run
cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run
cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run
cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run
```
- `specforge ingest <source> --dry-run` prints computed `SourceIR` JSON without writing artifacts
- `specforge ingest <source>` materializes `generated/source_ir/<document_key>/source_ir.json`
- `specforge evidence <source-ir> --dry-run` prints computed `EvidenceIR` JSON without writing artifacts
- `specforge evidence <source-ir>` materializes `generated/evidence_ir/<document_key>/evidence_ir.json`
- `specforge semantic <evidence-ir> --dry-run` prints computed `SemanticIR` JSON without writing artifacts
- `specforge semantic <evidence-ir>` materializes `generated/semantic_ir/<document_key>/semantic_ir.json`
- `specforge intent <semantic-ir> --dry-run` prints computed `IntentIR` JSON without writing artifacts
- `specforge intent <semantic-ir>` materializes `generated/intent_ir/<document_key>/intent_ir.json`
- `specforge adapt <intent-ir> --target fsm --dry-run` prints computed adapter JSON without writing artifacts
- `specforge adapt <intent-ir> --target fsm` materializes `generated/adapters/fsm/<document_key>/adapter.json` and writes an emitted standalone `.fsm` file when the canonical interface/control/system/init surface is explicit enough to lower honestly
- PDF execute-mode ingest expects `docling` to be importable from `python3` or `python`; when it lives elsewhere, set `SPECFORGE_DOCLING_PYTHON=/path/to/python`

## Planned product shape
- stage 0: build `SourceIR` from raw sources, normalized text views, structured page artifacts, and extracted visual assets
- stage 1: build `EvidenceIR` from normalized text, section anchors, evidence spans, figure/caption links, visual evidence, and statement extraction
- stage 2: build `SemanticIR` from actors, interfaces, backend-neutral system/init records, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- stage 3: build `IntentIR` as the canonical backend-independent intent model, including explicit interface inventory, backend-neutral guarded/action fragments, and backend-neutral system/init records when supported by the evidence
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
