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
- latest_commit_hash: `e8aae43`
- latest_commit_brief_message: `feat(core): add convergent IR pipeline loop`
- note: current uncommitted work is the first `R15` carry-through slice that preserves the actor-relative KG in `SemanticIR` / `IntentIR`

## Recent commit chain (last 5)
- `e8aae43` feat(core): add convergent IR pipeline loop
- `0ab3b02` fix(nlp): filter markdown alias markers
- `2c9bd30` feat(evidence): converge extraction and refresh AMBA artifacts
- `1a5f7bc` fix(ingest): broader timing_diagram classification for figure captions
- `a264144` fix(ingest): caption-gated signal_description classification

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, and `nlp-enrich`
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior

## Completed technical work in this session
- landed the first `R15` slice:
  - `SemanticIR` now preserves `actor_signal_relations`, `actor_ports`, and `signal_connectivity`
  - `IntentIR` now preserves the same actor-relative KG surface as canonical output
  - `ActorRecord` / `IntentActor` now preserve grounded actor names when relation evidence exists
  - `specforge validate` now reports KG-native counts for `SemanticIR` / `IntentIR`
- landed the next `R7` slice:
  - `specforge validate` now writes a deterministic `validation_report.json` sidecar for each IR-stage artifact
  - the validated artifact now stores the latest report in `validation_reports`
  - validation findings are now graph-aware for the actor-relative KG surface
- live docs were refreshed so roadmap/status/analysis reflect:
  - latest local APB/AHB/AXI snapshot = `95 / 95 / 89`
  - actor-relative KG carry-through is now `In Progress`, not `Not Started`
  - IR-stage validation backannotation is now implemented
  - test suite = `106` passing
- validation completed:
  - `cargo fmt --all` → passed
  - `cargo test --manifest-path Cargo.toml` → `106` passed

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/commands/validate.rs`
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/src/ir/intent.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `README.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. if committing this slice, stage the IR/validator/doc updates for actor-relative KG carry-through; do not stage `generated/`
2. continue `R7` by projecting the persisted validation findings into the live docs instead of keeping that step manual
3. continue the remaining `R15` slice by making the actor-relative graph, not compatibility `direction_hint`, the primary downstream direction model

## Remaining engineering gaps after this commit
- live-doc projection of validation findings and adapter validation (`R7`)
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
7. continue with live-doc projection for persisted validation findings unless the user redirects
