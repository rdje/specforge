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
- latest_commit_hash: `1064bec`
- latest_commit_brief_message: `feat(converge): default to full ollama loop`
- note: current uncommitted work is a docs-only steering update that logs multimodal semantic recovery and KG-guided rescans as the preferred implementation direction

## Recent commit chain (last 5)
- `1064bec` feat(converge): default to full ollama loop
- `894c538` feat(validation): project reports into live docs
- `fc66933` feat(ir): preserve kg and backannotate validation
- `e8aae43` feat(core): add convergent IR pipeline loop
- `0ab3b02` fix(nlp): filter markdown alias markers

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports

## Completed technical work in this session
- logged a new steering principle in `DEVELOPMENT_NOTES.md`:
  - the project goal is grounded implementation-intent recovery from chip-design PDFs, not PDF parsing for its own sake
  - tables, figures, and prose are all first-class evidence surfaces
  - the preferred future tactic is KG-guided staged rescanning until the backannotated knowledge stabilizes
- updated continuity docs so the steering note is recoverable in future sessions
- validation not run for this task because the change is documentation-only

## Current working tree before commit
- modified tracked files currently include:
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. commit the steering-note doc update; do not stage `generated/`
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
