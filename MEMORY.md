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
- `subs/fsmgen` is contextual only and must be treated as read-only from `specforge`
- if `fsmgen` misbehavior is identified, file a thorough local tracked bug report instead of patching the submodule in place
- use the local bug-report ID format `FSMGEN-BUG-####` for such upstream reports

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
  - `intent <semantic-ir> --dry-run`
  - `intent <semantic-ir>`
  - `adapt <intent-ir> --target fsm --dry-run`
  - `adapt <intent-ir> --target fsm`
- the currently implemented real stage artifacts are `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, and the first renderable standalone `.fsm` adapter slice
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters

## Latest committed baseline
- latest_commit_hash: `6aba2ef904678bd524a4f1fd55b78b592ad02b96`
- latest_commit_brief_message: `Implement first DT-centric fsm adapter slice`
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
- the current slice has now implemented the first real `IntentIR` constructor and CLI on top of ready `SemanticIR` artifacts
- the current slice has now added `subs/fsmgen` as a pinned local git submodule for `.fsm` adapter reference work
- the user has now clarified that `subs/fsmgen` is contextual-only, read-only, and any observed upstream misbehavior must be tracked locally under `FSMGEN-BUG-####`
- the current slice has now implemented the first DT-centric `.fsm` adapter artifact and `specforge adapt` command on top of persisted `IntentIR`
- the current slice has now enriched `SemanticIR` and `IntentIR` with backend-neutral interface/control records just far enough to support honest standalone renderable `.fsm` emission
- the current slice has now widened the `.fsm` adapter so it can emit a real standalone `?dt:name` file when the canonical facts are explicit and keep broader cases blocked otherwise

## In-flight work in this session
- refresh the live docs and continuity files for the canonical interface/control enrichment and renderable standalone `.fsm` slice
- run the commit workflow for the completed canonical/renderable slice

## Current execution checkpoint
- `IntentIR` now has a real build/materialization path
- the repository now includes:
  - `.gitmodules`
  - `subs/fsmgen` pinned at `57f00e581b4fc9a2aa02318846d1eb8a726c8960`
- `subs/fsmgen` is now explicitly treated as read-only contextual input, with future upstream bug reports to be tracked locally as `FSMGEN-BUG-####`
- execute-mode `specforge intent` now writes:
  - `generated/intent_ir/<document_key>/intent_ir.json`
  - intent identity, actor responsibilities, behaviors, constraints, assumptions, and residual decisions
- execute-mode `specforge adapt --target fsm` now writes:
  - `generated/adapters/fsm/<document_key>/adapter.json`
  - DT-centric root-kind choice, canonical signal inventory, canonical control-block candidates, renderability status, and adapter residual decisions
  - a real emitted standalone `.fsm` file when every referenced signal has explicit width/direction and every control block is fully typed
- `SemanticIR` now preserves:
  - typed signal records when explicit declarations are present
  - backend-neutral guarded/action control fragments from explicit `Block ...` statements
- `IntentIR` now carries:
  - canonical interface inventory
  - backend-neutral guarded/action control fragments
- the current validation set is:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/handshake.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/handshake/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/handshake/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/handshake/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/handshake/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/comb_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/comb_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/comb_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/comb_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/comb_dt/intent_ir.json --target fsm`
- the next implementation action is to broaden the canonical control surface beyond the first standalone renderable `.fsm` slice, starting with sequential/system-contract facts while keeping composition roots deferred

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `INTENTIR_SPEC.md`
4. inspect `LIVE_ACHIEVEMENT_STATUS.md`
5. inspect `ROADMAP.md`
6. inspect `RUST_CODEBASE_ANALYSIS.md`
7. continue with the next implementation slice unless the user redirects

## Recommended next implementation slice
- broaden the standalone renderable `.fsm` slice toward sequential/system-contract support
- keep `IntentIR` as the canonical endpoint and keep adapters downstream of it

## Commit status
- the latest committed baseline is `6aba2ef904678bd524a4f1fd55b78b592ad02b96`
- the current working tree contains the uncommitted canonical interface/control enrichment, renderable standalone `.fsm` slice, and the matching live-doc refreshes
- before the next commit, follow `COMMIT.md`
