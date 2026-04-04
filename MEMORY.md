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
- latest_commit_hash: `4ad132b`
- latest_commit_brief_message: `feat(semantic): surface provisional role residuals`
- note: current uncommitted work hardens handshake-role consumers so fallback-only provisional semantic roles no longer drive typed handshake recovery and handshake-shaped provisional-role signals now block raw name fallback too

## Recent commit chain (last 5)
- `4ad132b` feat(semantic): surface provisional role residuals
- `29107c8` feat(semantic): surface blocked handshake fallback
- `99c93bb` feat(semantic): block handshake fallback on contested roles
- `cddd9b3` feat(semantic): surface role arbitration state
- `98083b8` feat(semantic): preserve role candidates

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports

## Completed technical work in this session
- `crates/specforge/src/ir/semantic.rs` now requires observation-backed `semantic_consensus` before a signal can populate the canonical handshake-role context
- handshake-shaped signals with fallback-only provisional roles now block literal name fallback too, so spelling no longer silently re-promotes weaker role meaning into typed handshake semantics
- `crates/specforge/src/commands/validate.rs` now reports blocked handshake fallback for those provisional-role cases under the existing handshake-fallback surface
- added regression coverage proving that provisional roles do not drive handshake-role context and that validation reports the blocked fallback case
- synced `README.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `DEVELOPMENT_NOTES.md`, and `CHANGES.md` so the stricter handshake-consumer rule is visible in continuity docs
- validation ran for this task:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `175/175`

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/src/commands/validate.rs`
  - `README.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. commit the stricter handshake-consumer slice
2. continue the semantic-truthfulness program by carrying the same “consensus-backed meaning outranks weaker fallback meaning” rule into more downstream semantic consumers and arbitration surfaces
3. keep adapter work de-prioritized until the graph, temporal, arbitration, and evaluation surfaces are materially stronger

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, multi-producer ambiguity, modality-aware semantic-role grounding, consensus summaries, semantic candidates, and semantic arbitration summaries (`R15d`)
- KG-quality evaluation and benchmark hardening (`R15e`)
- Tier 3 relation extraction after the graph/temporal/eval surfaces are ready (`R14`)
- some downstream compatibility and consumer paths still rely on flat `direction_hint` instead of the graph-native surface
- meaning-based role inference now covers tables, prose, alias-grounded prose, visual captions, and VLM timing annotations, and canonical layers now preserve that provenance, but broader downstream use of preserved arbitration state is still early
- semantic-role disagreement is now surfaced explicitly across `EvidenceIR`, `SemanticIR`, and `IntentIR`, but richer multimodal role grounding and broader arbitration still need to grow
- `SourceIR` / ingest are strong enough to remain the foundation, but not strong enough to be assumed universal; future Tier 1 work should stay focused on robustness and honest failure handling
- adapter expansion and adapter validation are now intentionally horizon work
- the workspace still emits compile warnings in `ir/adapters.rs` and `ir/semantic.rs`

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `LIVE_ACHIEVEMENT_STATUS.md`
4. read `ROADMAP.md`
5. read `RUST_CODEBASE_ANALYSIS.md`
6. inspect `git --no-pager status --short`
7. continue with the next evidence-loop hardening slice unless the user redirects
