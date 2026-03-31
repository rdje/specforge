# USER_GUIDE
## Purpose
- explain what `specforge` is intended to do from an end-user perspective
- document the expected command-line workflow as the tool is built

## What SpecForge is intended to become
- a staged Rust tool that helps transform protocol specifications and RTL-module specifications into:
  - normalized source artifacts
  - structured extraction documents
  - actor-oriented planning artifacts
  - `.fsm` scaffolds and later fuller `.fsm` implementations

## Current state
- the repository contains workflow and continuity documentation plus the first runnable Rust CLI bootstrap
- the current CLI already supports:
  - `spec2fsm inspect <path>`
  - `spec2fsm ingest <source> --dry-run`

## Available commands today
### Inspect a path
```bash
cargo run -p spec2fsm -- inspect README.md
```
- reports:
  - canonical path
  - path kind
  - detected source kind
  - extension
  - file size for regular files

### Plan an ingest run
```bash
cargo run -p spec2fsm -- ingest README.md --dry-run
```
- reports the planned ingest steps without modifying anything
- useful for checking how the staged workflow will classify a source

## Planned user workflow
1. provide a source specification
2. normalize it into a stable working artifact set
3. inspect the extracted structure and actor planning artifacts
4. emit `.fsm` scaffolds
5. validate the emitted `.fsm` outputs
6. iterate until the extracted model is precise enough

## Planned command shape
- `spec2fsm ingest <source>`
- `spec2fsm inspect <artifact>`
- `spec2fsm extract <source>`
- `spec2fsm worksheet <source>`
- `spec2fsm emit <actor-or-plan>`
- `spec2fsm validate <fsm-file>`

## Expected user-visible principles
- the tool should be staged and inspectable
- intermediate artifacts should be preserved, not hidden
- evidence and provenance should be visible
- deferred features and abstractions should be explicit

## Current limitation
- this is still a bootstrap guide
- real ingest, extraction IR, worksheet emission, and validation integration are not implemented yet

## Where to look next
- `README.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
