# SpecForge
This file is the single entry point for the project.
Use it first for the project objective, document navigation, and the current implementation map.

## Project objective
- build a staged Rust toolchain for extracting implementation-relevant intent from protocol, component, and system specifications and converging that intent toward actor-oriented `.fsm` implementations
- push automation as far as safely possible; if any step cannot yet be fully automated, emit a structured residual decision packet instead of leaving an ad hoc manual gap
- keep deterministic extraction, typed intent representation, LLM-assisted reasoning, `.fsm` emission, and validation as distinct stages instead of collapsing everything into one opaque step
- preserve crash-safe continuity through live documentation so a new AI or LLM session can resume work quickly and correctly

## Current repository state
- the initial live-document surface has been established
- the initial Rust workspace has been scaffolded
- the first `spec2fsm` CLI surface exists and currently supports:
  - `inspect <path>`
  - `ingest <source> --dry-run`
- the next implementation milestone is real ingest manifest, normalized source-model, and residual-decision scaffolding support

## Working naming
- repository / project name: `specforge`
- working CLI / binary name: `spec2fsm`

## Fast ramp-up order
1. `README.md`
2. `SESSION_BOOTSTRAP.md`
3. `ROADMAP.md`
4. `LIVE_ACHIEVEMENT_STATUS.md`
5. `RUST_CODEBASE_ANALYSIS.md`
6. `USER_GUIDE.md`
7. `DEVELOPMENT_NOTES.md`
8. `CHANGES.md`
9. `MEMORY.md`
10. `COMMIT.md`

## Documentation index
- `README.md`
  - single project entry point and navigation hub
- `SESSION_BOOTSTRAP.md`
  - exact fresh-session instruction for a new AI or LLM instance
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
  - dependency lockfile created by the first build
- `crates/spec2fsm/Cargo.toml`
  - initial CLI crate manifest
- `crates/spec2fsm/src/main.rs`
  - binary entrypoint
- `crates/spec2fsm/src/lib.rs`
  - top-level command dispatch
- `crates/spec2fsm/src/cli.rs`
  - clap-based CLI model
- `crates/spec2fsm/src/error.rs`
  - current typed error boundary
- `crates/spec2fsm/src/source.rs`
  - source-kind classification helpers
- `crates/spec2fsm/src/commands/inspect.rs`
  - first path/source inspection command
- `crates/spec2fsm/src/commands/ingest.rs`
  - ingest planning command, currently dry-run only

### Planned future implementation paths
- `fixtures/`
  - planned sample protocols, specs, and test inputs
- `examples/`
  - planned example invocations and sample projects
- `generated/`
  - planned generated artifacts only when intentionally versioned

## Quick start
```bash
cargo test
cargo run -p spec2fsm -- --help
cargo run -p spec2fsm -- inspect README.md
cargo run -p spec2fsm -- ingest README.md --dry-run
```

## Planned product shape
- stage 0: ingest and normalize source documents
- stage 1: build section maps and evidence records
- stage 2: build typed intent records for source facts, derived rules, local design decisions, abstractions, and actor-oriented behavior
- stage 3: produce dossiers, worksheets, actor/decomposition artifacts, and residual decision packets
- stage 4: synthesize `.fsm` scaffolds and later fuller `.fsm` implementations from typed intent
- stage 5: validate with FSMGen and back-annotate the findings

## Key operating principles
- staged tool, not one-shot conversion
- intent capture rather than literal text conversion
- automation-first, manual-last
- actor-first extraction
- typed IR over string-based generation
- deterministic steps where possible
- LLM assistance where interpretation is required
- residual decision packets for irreducible ambiguity
- live documentation as critical continuity infrastructure

Read SESSION_BOOTSTRAP.md and start from there.
