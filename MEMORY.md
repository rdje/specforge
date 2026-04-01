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
  - `evidence <source-ir> --dry-run`
  - `evidence <source-ir>`
  - `semantic <evidence-ir> --dry-run`
  - `semantic <evidence-ir>`
- the currently implemented real stage artifacts are `SourceIR`, `EvidenceIR`, and `SemanticIR`
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters

## Latest committed baseline
- latest_commit_hash: `1612482bbffd9802443f7c0f6ea61c8695c6900c`
- latest_commit_brief_message: `Implement first real EvidenceIR extraction stage`
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
- the next slice has now implemented a real Docling-backed PDF normalization path inside `SourceIR`
- the current slice has now implemented the first real `EvidenceIR` extractor and CLI on top of ready `SourceIR` artifacts
- the current slice has now implemented the first real `SemanticIR` extractor and CLI on top of ready `EvidenceIR` artifacts

## In-flight work in this session
- refresh the live docs and continuity files for the new `SemanticIR` stage
- run the commit workflow immediately after this task is closed

## Current execution checkpoint
- `SemanticIR` now has a real build/materialization path
- execute-mode `specforge semantic` now writes:
  - `generated/semantic_ir/<document_key>/semantic_ir.json`
  - actors, interfaces, phases, invariants, contracts, gates, abstractions, and decomposition candidates
  - residual decisions for unresolved actor/interface/visual ambiguities
- the current validation set is:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
- the next implementation action is the first real `IntentIR` constructor

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `INTENTIR_SPEC.md`
4. inspect `LIVE_ACHIEVEMENT_STATUS.md`
5. inspect `ROADMAP.md`
6. inspect `RUST_CODEBASE_ANALYSIS.md`
7. continue with the next implementation slice unless the user redirects

## Recommended next implementation slice
- build the first real `IntentIR` constructor from grounded `SemanticIR` artifacts
- keep `IntentIR` as the canonical endpoint and keep adapters downstream of it

## Commit status
- the latest committed baseline is `1612482bbffd9802443f7c0f6ea61c8695c6900c`
- the current working tree contains the uncommitted SemanticIR builder, CLI wiring, live-doc refreshes, and validation-record updates for the new stage
- before the next commit, follow `COMMIT.md`
