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
- latest_commit_hash: `7be5253702d99945bbfd693cde148121b6b8a7ed`
- latest_commit_brief_message: `Pivot specforge to IntentIR and multimodal IR scaffolding`
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

## In-flight work in this session
- implement real structured PDF normalization inside `SourceIR`
- refresh the live docs and continuity files for the new backend
- run the commit workflow immediately after this task is closed

## Current execution checkpoint
- `SourceIR` now has a real Docling-backed PDF materialization path
- execute-mode PDF ingest now writes:
  - promoted markdown
  - page images and page metadata sidecars
  - visual asset crops for pictures and tables
  - metadata JSON and backend raw JSON
  - `page_artifacts.json` and `visual_assets.json`
- visual assets now carry backend `source_ref` values for later grounding
- runtime discovery supports:
  - `python3` / `python` with `docling` importable
  - optional override via `SPECFORGE_DOCLING_PYTHON`
  - test/advanced override via `SPECFORGE_DOCLING_HELPER`
- the backend implementation is validated with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- ingest README.md`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
- the next implementation action is the first real multimodal `EvidenceIR` extractor

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `INTENTIR_SPEC.md`
4. inspect `LIVE_ACHIEVEMENT_STATUS.md`
5. inspect `ROADMAP.md`
6. inspect `RUST_CODEBASE_ANALYSIS.md`
7. continue with the next implementation slice unless the user redirects

## Recommended next implementation slice
- build the first real `EvidenceIR` extractor from normalized markdown, figures, captions, and page assets
- keep `IntentIR` as the canonical endpoint and keep adapters downstream of it

## Commit status
- the latest committed baseline is `7be5253702d99945bbfd693cde148121b6b8a7ed`
- the current working tree contains the uncommitted SourceIR PDF backend, live-doc refreshes, and validation-record updates for the new backend
- before the next commit, follow `COMMIT.md`
