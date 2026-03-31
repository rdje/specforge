# ROADMAP
## Objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from protocol, component, and system specifications and converting that intent into actor-oriented `.fsm` outputs
- push automation as far as safely possible; any remaining manual work must be reduced to structured decision packets with explicit evidence and downstream impact
- preserve deterministic provenance and typed intermediate data across all stages
- make the workflow resumable and understandable through live project documentation

## Major workstreams
### R0 Repository, workflow, and continuity bootstrap
- status: Done
- goals:
  - establish the live documentation surface
  - define the project objective and staged direction
  - define the commit workflow and continuity expectations
  - create a usable session bootstrap path for future AI/LLM sessions
- completion criteria:
  - core live documents exist
  - README is the single entry point
  - SESSION_BOOTSTRAP is in place
  - roadmap and live-status files are established

### R1 Rust workspace and CLI bootstrap
- status: Done
- goals:
  - create the Rust workspace manifest
  - create the initial `spec2fsm` CLI entrypoint
  - establish shared error handling, config loading, and command dispatch
- completion criteria:
  - `cargo` workspace builds
  - `spec2fsm --help` works
  - at least one no-op or inspection command exists

### R2 Source ingest and normalization
- status: Not Started
- goals:
  - orchestrate PDF to Markdown conversion
  - normalize file layout, metadata, and artifact manifests
  - preserve figures, tables, and promotion paths
  - record source identity, conversion quality, and promotion provenance
- completion criteria:
  - deterministic ingest manifest exists
  - source identity, conversion quality, and stable promoted markdown paths can be recorded

### R3 Evidence extraction and typed IR
- status: Not Started
- goals:
  - extract section maps
  - capture signals, channel candidates, and evidence spans
  - classify source facts, derived machine rules, local design decisions, and explicit abstractions
  - define typed intent IR for actors, invariants, contracts, gates, assertions, abstractions, decomposition, and automation confidence
- completion criteria:
  - typed intent IR exists
  - evidence items retain provenance to source ranges
  - residual decision artifacts can be represented in typed form

### R4 Actor planning and worksheet generation
- status: Not Started
- goals:
  - generate actor catalogs
  - generate structured extraction worksheets
  - record deferred features and abstractions explicitly
  - emit structured residual decision packets for unresolved ambiguities
- completion criteria:
  - worksheet, actor planning, and residual decision artifacts can be emitted from IR

### R5 `.fsm` scaffold emission
- status: Not Started
- goals:
  - emit initial `.fsm` scaffolds from typed IR
  - preserve actor boundaries and decomposition choices
  - avoid opaque string-only generation
- completion criteria:
  - at least one emitted `.fsm` validates structurally

### R6 Validation and back-annotation
- status: Not Started
- goals:
  - integrate FSMGen validation
  - collect diagnostics and generated-output summaries
  - back-annotate validation findings into live artifacts
- completion criteria:
  - validation reports are reproducible and tied to emitted artifacts

### R7 Assisted reasoning layer
- status: Not Started
- goals:
  - define the LLM-assisted boundary for actor discovery and semantic extraction
  - keep deterministic stages separate from interpretation-heavy stages
  - preserve prompts, evidence, and outputs in replayable form
  - automate by default and escalate only when confidence or validation says a user decision is necessary
- completion criteria:
  - assisted extraction can be audited and resumed with provenance

## Recommended implementation order
1. finish repository/bootstrap documentation
2. scaffold the Rust workspace and `spec2fsm` CLI
3. implement ingest manifest, normalized source model, and provenance capture
4. implement section-map and evidence extraction
5. define the actor-oriented typed intent IR and residual decision types
6. emit structured markdown artifacts and residual decision packets from the IR
7. emit `.fsm` scaffolds
8. integrate validation and back-annotation

## Immediate next milestone
- implement the first real ingest manifest, normalized source model, and residual-decision scaffolding behind `spec2fsm ingest`
