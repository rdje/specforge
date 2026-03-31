# RUST_CODEBASE_ANALYSIS
## Purpose
- maintain a live, deep-dive analysis of the Rust codebase
- record the current architecture, risks, subsystem boundaries, and recommended implementation direction
- remain useful even when the Rust codebase is still small or only partially built

## Executive summary
- the repository now contains the first Rust workspace and a minimal but real `spec2fsm` CLI
- the current codebase is intentionally small and architecture-light, but it already establishes:
  - a build/test loop
  - a user-visible command surface
  - typed error handling
  - a first deterministic source-classification helper
- the main architectural risk has shifted from “starting with the wrong first abstraction” to “letting the initial single-crate bootstrap harden into a monolith once ingest and IR logic become real”
- the next slice should add real ingest manifest and normalization support while preserving provenance and staged boundaries

## Observed current state
### Repository contents directly observed
- `.git/`
- live project documentation surface
- `Cargo.toml`
- `Cargo.lock`
- `crates/spec2fsm/Cargo.toml`
- `crates/spec2fsm/src/main.rs`
- `crates/spec2fsm/src/lib.rs`
- `crates/spec2fsm/src/cli.rs`
- `crates/spec2fsm/src/error.rs`
- `crates/spec2fsm/src/source.rs`
- `crates/spec2fsm/src/commands/inspect.rs`
- `crates/spec2fsm/src/commands/ingest.rs`

### Rust-specific contents still absent
- no dedicated `specforge-core` crate
- no dedicated `specforge-ingest` crate
- no dedicated `specforge-ir` crate
- no dedicated `specforge-validate` crate
- no `fixtures/`
- no `examples/`
- no integration-test harness beyond the crate-local unit tests

### Immediate implication
- there is now a real Rust codebase to preserve and evolve
- the current code is still bootstrap-scale, so a single crate is acceptable
- future growth pressure should be handled by splitting real boundaries out of the crate only when the code actually justifies that move

## What the tool needs to do
- orchestrate source ingest from PDF or already-normalized Markdown
- maintain source provenance and evidence ranges
- support actor-oriented extraction rather than monolithic protocol flattening
- preserve the distinction between:
  - source facts
  - derived rules
  - local design decisions
  - explicit abstractions
- emit structured markdown artifacts and later `.fsm` outputs
- validate emitted `.fsm` files and back-annotate the result

## Current implemented architecture
### Root workspace
- `Cargo.toml`
  - workspace root
- `Cargo.lock`
  - dependency lockfile

### Active crate
- `crates/spec2fsm`
  - single user-facing CLI crate and binary for the first slice

### Implemented module boundaries
- `src/main.rs`
  - binary entrypoint
- `src/lib.rs`
  - command dispatch
- `src/cli.rs`
  - clap-based command model
- `src/error.rs`
  - typed error/result boundary
- `src/source.rs`
  - source-kind classification helper
- `src/commands/inspect.rs`
  - deterministic inspection command
- `src/commands/ingest.rs`
  - ingest planning command, currently dry-run only

### Assessment of current structure
- good:
  - not a single-file prototype
  - clear enough command/module split for the first slice
  - low friction for rapid progress
- not yet sufficient for the long-term architecture:
  - ingest logic is not yet rich enough to justify a dedicated ingest crate
  - there is no typed extraction IR yet
  - there is no validation crate boundary yet

## Architectural recommendation
### Core architectural stance
- use typed Rust data structures as the system of record
- make the typed model represent implementation-relevant intent, not merely document structure
- do not let markdown text or prompt text become the primary internal representation
- treat markdown emission, prompts, and `.fsm` emission as views over typed IR rather than as the IR itself
- represent irreducible ambiguity explicitly as residual decision packets rather than freeform notes

### Recommended growth path from the current codebase
#### Keep in the current crate for one more slice
- path/source inspection
- ingest planning
- early manifest types
- basic source normalization

#### Split into a dedicated crate when pressure becomes real
- ingest manifest and artifact promotion logic
- reusable typed IR
- validation integration

### Recommended future subsystem split
- `specforge-cli`
  - CLI parsing
  - command dispatch
  - top-level workflow orchestration
- `specforge-core`
  - common error types
  - identifiers
  - filesystem/path helpers
  - manifest metadata
- `specforge-ingest`
  - PDF/Markdown source registration
  - converter orchestration
  - artifact promotion and normalization
- `specforge-ir`
  - typed models for:
    - section maps
    - evidence items
    - signals
    - actor catalogs
    - actor sheets
    - invariants
    - contracts
    - gates
    - assertion seeds
    - abstractions
    - decomposition plans
- `specforge-validate`
  - FSM validation boundary
  - validation report capture
  - generated-output metadata

## Mapping from staged method to Rust subsystems
### Phase 0: PDF to Markdown conversion
- future primary ownership:
  - `specforge-ingest`
- immediate precursor in current code:
  - `ingest --dry-run`

### Phase 1: Section mapping
- future ownership:
  - `specforge-ingest`
  - later `specforge-ir`

### Phase 2 through Phase 11: Extraction and normalization
- future ownership:
  - `specforge-ir`

### Phase 12 and Phase 13: Decomposition and `.fsm` emission
- future ownership:
  - `specforge-ir`
  - later an emitter layer

### Phase 14: Validation and back-annotation
- future ownership:
  - `specforge-validate`

## Recovered precedent from the AXI extraction workspace
- the workspace at `/Users/richarddje/Documents/livework/protocols/arm/axi` already exercised the intended extraction method at the document/artifact level
- reusable artifact families observed there:
  - protocol dossier
  - section map / key section anchors
  - channel catalog
  - actor catalog
  - actor sheets
  - invariant, contract, gate, and assertion ledgers
  - abstraction log
  - `.fsm` decomposition sheet
  - validation log
- architectural conclusion:
  - these should become typed models, not ad hoc markdown-only artifacts
- likely Rust type families implied by that precedent:
  - `IngestManifest`
  - `NormalizedSourceDocument`
  - `DocumentIdentity`
  - `SourceReference`
  - `SectionRef`
  - `EvidenceItem`
  - `StatementClass`
  - `AutomationConfidence`
  - `ResidualDecisionPacket`
  - `CandidateInterpretation`
  - `ProtocolDossier`
  - `ActorCatalog`
  - `ActorSheet`
  - `InvariantRecord`
  - `ContractRecord`
  - `GateRecord`
  - `AssertionSeed`
  - `AbstractionRecord`
  - `FsmDecompositionPlan`
  - `ValidationLog`
- immediate impact on the next slice:
  - `crates/spec2fsm/src/commands/ingest.rs` already sketches the right staged flow, but still only prints the plan
  - R2 should materialize a real manifest plus normalized-source metadata/provenance, not stop at path classification
  - R2 should leave clear hooks for later section-map and evidence extraction even if those are not fully implemented yet
  - R2 should introduce the first residual-decision scaffolding so incomplete automation has a typed representation from the start
  - markdown dossier/worksheet emission should come after the typed forms exist

## Data-model requirements
### Provenance is mandatory
- every extracted fact should be able to reference:
  - source file
  - section/range
  - optional figure/table reference
  - extraction confidence

### IR stability matters more than prompt stability
- prompts will evolve
- markdown templates will evolve
- the typed IR should be comparatively stable and explicit

### Abstractions must be first-class
- deferred capability cannot be hidden in prose
- the IR must carry explicit abstraction records so downstream `.fsm` emission and validation know what was intentionally omitted

## First coding slice assessment
### Intended goals
- create the smallest executable Rust workspace that still reflects the staged architecture

### What was actually delivered
- root workspace manifest
- one user-facing CLI crate and binary
- clap-based command parsing
- typed error handling
- first deterministic command pair:
  - `inspect`
  - `ingest --dry-run`
- unit tests for source-kind detection and ingest stem logic

### Why this slice is good enough
- it establishes the repo as a real Rust project
- it keeps the first code slice deterministic and inspectable
- it avoids pretending ingest already exists when only the command surface is ready

### What is still missing
- real ingest manifest type
- normalized source model
- converter orchestration
- typed extraction IR
- worksheet/artifact emission
- validation integration

## Major risks
### Risk: premature monolith
- if ingest, extraction, emission, and validation all accumulate in `crates/spec2fsm` without boundary management, the staged-tool design will erode quickly
- current status:
  - acceptable today
  - must be watched closely in the next one or two slices

### Risk: prompt-driven architecture
- if LLM prompts become the de facto internal format, deterministic replay and continuity will suffer
- current status:
  - not active yet because no LLM boundary exists in code

### Risk: insufficient provenance
- if extracted artifacts lose source ranges early, later validation and auditing will become fragile
- current status:
  - not yet a live implementation defect
  - must be addressed in the next real ingest/IR slice

### Risk: emitter-first implementation
- if `.fsm` emission is implemented before the IR is stabilized, the project will accumulate string-based technical debt
- current status:
  - low for now
  - avoid rushing into `.fsm` output from the current CLI scaffold

## Testing implications
- current tests cover:
  - source-kind detection
  - deterministic ingest stem naming
- next tests should cover:
  - ingest manifest/path normalization
  - command behavior around missing/unsupported inputs
  - deterministic artifact naming
- protocol-semantic tests should come later once the IR exists

## Validation completed in this session
- `cargo test`
  - passed
- `cargo run -p spec2fsm -- --help`
  - passed
- `cargo run -p spec2fsm -- inspect README.md`
  - passed
- `cargo run -p spec2fsm -- ingest README.md --dry-run`
  - passed

## Recommended update triggers for this document
- creation of a real ingest manifest
- addition of new crates or major module boundaries
- introduction of public integration seams
- introduction of an LLM boundary
- changes in the recommended architecture or risk picture

## Current recommendation
- keep the current single-crate workspace for one more slice
- next, implement a real ingest manifest and normalized source model behind `spec2fsm ingest`
- after that, reassess whether `specforge-ingest` should become its own crate
