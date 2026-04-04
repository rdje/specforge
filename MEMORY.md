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

## User-required continuity rules
- live documents are critical infrastructure, not optional afterthoughts
- update `MEMORY.md` after completed tasks and at meaningful checkpoints during long-running work
- `MEMORY.md` must record the latest already-committed Git hash/message known at the time of update
- `CHANGES.md` must capture the detailed changes about to be committed
- `DEVELOPMENT_NOTES.md` must capture engineering decisions, rationale, and validation context
- `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and `RUST_CODEBASE_ANALYSIS.md` must be reviewed before each commit when the task changes them
- `README.md` remains the single entry point and its last line must be `Read SESSION_BOOTSTRAP.md and start from there.`
- `subs/fsmgen` is contextual-only and must stay read-only from `specforge`
- if `fsmgen` behavior looks wrong, file a local tracked bug report using `FSMGEN-BUG-####` instead of patching the submodule

## Latest committed baseline
- latest_commit_hash: `fc66933`
- latest_commit_brief_message: `feat(ir): preserve kg and backannotate validation`
- note: current uncommitted work adds deterministic live-doc projection for persisted validation reports via `specforge project-validation`

## Recent commit chain (last 5)
- `fc66933` feat(ir): preserve kg and backannotate validation
- `e8aae43` feat(core): add convergent IR pipeline loop
- `0ab3b02` fix(nlp): filter markdown alias markers
- `2c9bd30` feat(evidence): converge extraction and refresh AMBA artifacts
- `1a5f7bc` fix(ingest): broader timing_diagram classification for figure captions

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, and `nlp-enrich`
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports

## Completed technical work in this session
- landed the next `R7` slice:
  - added `specforge project-validation <artifact>...`
  - the new command validates the passed artifacts, reloads their persisted `validation_reports`, writes tracked `VALIDATION_SNAPSHOT.md`, and refreshes the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md`
  - staged validation continuity no longer depends on hand-editing score snapshots after local runs
- refreshed the tracked validation snapshot from the current AMBA `IntentIR` artifacts:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 89/100 GOOD
- live docs were refreshed so roadmap/status/analysis reflect:
  - live-doc projection of staged validation findings is now implemented
  - adapter validation is the remaining `R7` gap
  - actor-relative direction modeling remains `In Progress`
  - test suite = `107` passing
- validation completed:
  - `cargo fmt --all` → passed
  - `cargo test --manifest-path Cargo.toml` → `107` passed
  - `cargo run -p specforge -- project-validation generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/cli.rs`
  - `crates/specforge/src/lib.rs`
  - `crates/specforge/src/commands/mod.rs`
  - `crates/specforge/src/commands/project_validation.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `README.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `USER_GUIDE.md`
  - `VALIDATION_SNAPSHOT.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. commit the live-doc projection slice with the new CLI command, `VALIDATION_SNAPSHOT.md`, and the refreshed roadmap/status/memory docs; do not stage `generated/`
2. continue the remaining `R15` slice by making the actor-relative graph, not compatibility `direction_hint`, the primary downstream direction model
3. extend validation/reporting from staged IR artifacts into downstream adapter artifacts after the graph-first direction work is further along

## Remaining engineering gaps after this commit
- adapter validation beyond the staged IR surface (`R7`)
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- downstream scoring and compatibility paths still rely on flat `direction_hint` more than the new graph-native surface
- the workspace still emits compile warnings in `ir/adapters.rs` and `ir/semantic.rs`

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `LIVE_ACHIEVEMENT_STATUS.md`
4. read `ROADMAP.md`
5. read `RUST_CODEBASE_ANALYSIS.md`
6. inspect `git --no-pager status --short`
7. continue with the remaining graph-first `R15` direction-model work unless the user redirects
