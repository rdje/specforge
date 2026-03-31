# DEVELOPMENT_NOTES
## Current project direction
- project name: `specforge`
- working CLI/binary name: `spec2fsm`
- implementation language: Rust
- product shape: staged toolchain, not one-shot conversion
- project mission: staged specification intent capture pushed toward fully automated `.fsm` synthesis

## Foundational engineering choices
### Typed IR first
- the internal system of record should be typed Rust data, not ad hoc markdown text or string templates
- markdown artifacts, prompts, and emitted `.fsm` files should be generated from typed internal structures

### Automation-first intent capture
- the tool should optimize for automated extraction of implementation-relevant intent rather than merely producing intermediate paperwork
- full automation is the target state whenever the evidence and validation support it
- the correct question for each stage is not only “what artifact do we emit?” but “what ambiguity have we eliminated?”

### Actor-first extraction
- the tool should preserve actor boundaries and protocol decomposition rather than flattening everything into one monolithic model

### Residual decision packets instead of ad hoc manual gaps
- when automation cannot safely choose a single interpretation, the system should emit a structured residual decision packet
- each packet should capture the unresolved question, supporting/conflicting evidence, candidate interpretations, downstream impact, and the minimum user decision required to continue
- unresolved ambiguity should stay machine-tracked and resumable rather than leaking into freeform notes or tribal knowledge

### Deterministic versus assisted stages
- deterministic stages should own ingest, manifests, normalization, and validation
- interpretation-heavy stages such as actor discovery and semantic extraction can later use assisted reasoning, but must still produce typed artifacts with provenance

### Continuity as infrastructure
- live documentation is not optional process overhead
- `MEMORY.md`, `RUST_CODEBASE_ANALYSIS.md`, `CHANGES.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and `README.md` are part of the engineering system
- they must be updated when work completes and at meaningful intermediate checkpoints during long-running tasks
- `MEMORY.md` is an operational continuity view, not a transcript
- `MEMORY.md` must carry the latest committed baseline hash/message known at the time it is refreshed, and explicitly say `none yet` before the first commit exists

## Current repository observations
- the repository started essentially empty apart from `COMMIT.md` and `.git/`
- the documentation surface was established before Rust code was added
- the repository now contains the first Rust workspace and CLI bootstrap

## Initial architecture recommendation
- start with a small Rust workspace and grow only when real pressure appears
- preserve these conceptual subsystem boundaries from the start:
  - CLI/orchestration
  - ingest/normalization
  - typed extraction IR
  - validation integration

## Documentation surface established in this session
- `README.md`
- `SESSION_BOOTSTRAP.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `USER_GUIDE.md`
- `CHANGES.md`
- `MEMORY.md`
- `.gitignore`

## Rust implementation established in this session
### Workspace shape
- root workspace manifest: `Cargo.toml`
- initial CLI crate: `crates/spec2fsm`

### Current code boundaries
- `src/main.rs`
  - binary entrypoint
- `src/lib.rs`
  - command dispatch
- `src/cli.rs`
  - clap CLI model
- `src/error.rs`
  - typed error/result boundary
- `src/source.rs`
  - source-kind classification helpers
- `src/commands/inspect.rs`
  - deterministic inspection command
- `src/commands/ingest.rs`
  - ingest planning command, currently dry-run only

### Engineering choices in the first code slice
- keep the first executable implementation in one crate to avoid premature crate sprawl
- still preserve conceptual boundaries through modules so later extraction into dedicated crates stays straightforward
- implement two deterministic commands first:
  - one for path/source inspection
  - one for ingest planning
- keep real ingest side effects out of the first slice; dry-run first is safer and clarifies the intended staged boundary

### Validation run
- `cargo test`
- `cargo run -p spec2fsm -- --help`
- `cargo run -p spec2fsm -- inspect README.md`
- `cargo run -p spec2fsm -- ingest README.md --dry-run`
- all passed

## Continuity workflow refinement
- `COMMIT.md` now explicitly requires `MEMORY.md` to track the latest committed baseline hash/message
- the documented workflow also now explains the timing nuance:
  - a just-created commit hash can only be known after commit creation
  - therefore `MEMORY.md` records the latest already-known committed baseline at update time
  - then gets refreshed at the next documentation checkpoint so continuity catches up to the newly-created commit

## Recovered methodological precedent from the AXI workspace
- external reference workspace:
  - `/Users/richarddje/Documents/livework/protocols/arm/axi`
- re-read reference artifacts:
  - `PROTOCOL_EXTRACTION_METHOD.md`
  - `PROTOCOL_EXTRACTION_PROMPT.md`
  - `PROTOCOL_EXTRACTION_WORKSHEET.md`
  - `AXI_CORE_EXTRACTION_WORKSHEET.md`
  - `AXI_PROTOCOL_DOSSIER.md`
  - `AXI_ACTOR_CATALOG.md`
  - `AXI_FSM_DECOMPOSITION.md`
- durable method to carry forward:
  - convert and promote the source into searchable Markdown
  - build a section map before deeper synthesis
  - classify extracted content as source facts, derived machine rules, local design decisions, and explicit abstractions
  - discover actors before inventing FSM states
  - define interfaces and phases before state decomposition
  - capture invariants, contracts, gates, assertions, and abstractions explicitly
  - separate `.fsm` decomposition from `.fsm` emission
  - validate and back-annotate findings into the analysis artifacts
- refinements for `specforge`:
  - markdown worksheet/catalog/decomposition artifacts are outputs, not the primary internal representation
  - typed Rust data with provenance should be the primary internal representation
  - the method must generalize beyond AXI and bus protocols to arbitrary protocol and RTL-module specifications
  - deterministic ingest/normalization should remain separate from later assisted semantic extraction
  - the extracted model should represent implementation-relevant intent, not just document structure
- implication for the next code slice:
  - implement ingest structures that can eventually feed dossier, section-map, and evidence artifacts
  - define provenance/source-reference types early so later actor, invariant, contract, gate, and assertion IR can attach to them cleanly
  - introduce a residual-decision type so incomplete automation has a structured representation from the beginning

## Immediate next engineering target
- implement a real ingest manifest, normalized source model, and residual-decision scaffolding behind `spec2fsm ingest`
