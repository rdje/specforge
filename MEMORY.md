# MEMORY
## Purpose
- maintain a compact but actionable continuity record for interrupted sessions
- preserve enough context to resume work quickly after session loss, tool restart, machine crash, or model handoff

## Current project identity
- repository name: `specforge`
- CLI/binary name: `specforge`
- implementation language: Rust
- canonical deliverable: `IntentIR`
- stage model: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## What the user has explicitly required
- live documents are critical infrastructure, not optional afterthoughts
- all live documents together must preserve full operational continuity after session loss, crashes, or tool restarts
- `RUST_CODEBASE_ANALYSIS.md` must remain a live deep-dive analysis of the Rust codebase
- `MEMORY.md` must be updated after completed tasks and also at meaningful checkpoints while long tasks are in progress
- `MEMORY.md` must contain the latest committed Git hash and the corresponding brief commit message, or explicitly state that no commit exists yet
- `MEMORY.md` must stay compact, precise, and operational rather than becoming a verbatim transcript
- `CHANGES.md` must contain the full detailed set of changes about to be committed
- `DEVELOPMENT_NOTES.md` must capture engineering decisions, rationale, and context
- `USER_GUIDE.md` must explain the tool from the user perspective
- `ROADMAP.md` must track the goals, remaining work, and advancement
- `README.md` must be the single entry point and its last line must be:
  - `Read SESSION_BOOTSTRAP.md and start from there.`

## Repository state at this checkpoint
- Git repository exists
- the live documentation surface is established
- the active workspace member is `crates/specforge`
- the runnable CLI command surface now includes:
  - `inspect <path>`
  - `ingest <source> --dry-run`
  - `ingest <source>`
- the currently implemented real stage artifact is `SourceIR`
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters

## Latest committed baseline
- latest_commit_hash: `006b6a5e573beb4d5f07365903cb5f25c79c7817`
- latest_commit_brief_message: `Bootstrap specforge workspace and intent-capture baseline`
- continuity_rule:
  - refresh this section whenever a new latest committed baseline exists at the time `MEMORY.md` is updated

## Important session history
- the repository bootstrap and first baseline commit were completed earlier in the session
- the user then requested a stronger architecture:
  - stop treating `.fsm` as the endpoint
  - make `IntentIR` the canonical deliverable
  - keep backends as adapters
  - rename the CLI direction from `spec2fsm` to `specforge`
  - redesign the pipeline explicitly as `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- the user then clarified that PDF images, figures, and diagrams must be treated as semantically meaningful inputs when they carry information content
- the user explicitly required a strict SOTA quality bar for every stage of the tool
- the repo docs were rewritten around that architecture
- `INTENTIR_SPEC.md` was added as the canonical long-form architecture/spec document
- the Rust crate and CLI identity were renamed to `specforge`
- the Rust types were refactored so the current ingest slice is now explicitly `SourceIR`
- typed scaffolding was added for `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter planning
- `SourceIR` was further extended to reserve parser-backend, page-artifact, and visual-asset schema surface
- `EvidenceIR` was further extended to reserve multimodal visual-evidence records and text-to-figure linkage

## In-flight work in this session
- complete the IntentIR pivot in docs and code
- rename the active CLI/crate direction to `specforge`
- refactor the current ingest-side types into `SourceIR`
- record the staged IR architecture clearly enough for future implementation work to follow it consistently

## Current execution checkpoint
- the IntentIR pivot is implemented in the working tree and validated with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- --help`
  - `cargo run -p specforge -- ingest README.md --dry-run`
- the remaining `spec2fsm` references in tracked docs are historical notes only
- `specforge ingest` now emits `SourceIR` rather than a backend-oriented ingest artifact
- `SourceIR` now reserves parser-backend, page-artifact, and visual-asset fields for structured PDF normalization
- `EvidenceIR` now reserves multimodal evidence, visual observations, and text-to-figure links
- `specforge ingest README.md --dry-run` now shows the new parser backend, page-artifact manifest, visual-asset manifest, and placeholder-binding fields in the emitted `SourceIR`
- the next implementation action is to close the remaining `SourceIR` structured-PDF-normalization gap and then build the first real multimodal `EvidenceIR` extractor

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `INTENTIR_SPEC.md`
4. inspect `LIVE_ACHIEVEMENT_STATUS.md`
5. inspect `ROADMAP.md`
6. inspect `RUST_CODEBASE_ANALYSIS.md`
7. continue with the next implementation slice unless the user redirects

## Recommended next implementation slice
- orchestrate structured PDF normalization inside `SourceIR`
- materialize the promoted markdown, page-artifact, metadata, and visual-asset layout recorded by `SourceIR`
- build the first real `EvidenceIR` extractor from normalized markdown, figures, captions, and page assets
- keep `IntentIR` as the canonical endpoint and keep adapters downstream of it

## Commit status
- the latest committed baseline is still `006b6a5e573beb4d5f07365903cb5f25c79c7817`
- the current working tree contains the uncommitted IntentIR pivot, CLI rename, staged IR refactor, multimodal-ingestion schema/doc updates, and validation-record updates
- before the next commit, follow `COMMIT.md`
