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
- latest_commit_hash: `a57a564`
- latest_commit_brief_message: `feat(semantic): surface interface signal conflicts`
- note: current uncommitted work expands `R15b` so idiomatic one-cycle phrases like `next cycle`, `next tick`, and `next rising edge` become bounded `cycle_window` semantics instead of staying unbounded prose

## Recent commit chain (last 5)
- `a57a564` feat(semantic): surface interface signal conflicts
- `ee20e34` feat(semantic): surface structural kg conflicts
- `b68be48` feat(evidence): surface polarity conflicts explicitly
- `8c97c77` feat(evidence): mine polarity from signal tables
- `93d376c` feat(semantic): surface temporal conflicts

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports

## Completed technical work in this session
- `SemanticIR` now recognizes idiomatic one-cycle temporal phrases like `next cycle`, `next clock cycle`, `next tick`, `next rising edge`, and `following` / `subsequent` variants
- those phrases now lift into `CycleWindowRecord { min_cycles: Some(1), max_cycles: Some(1) }` so they join the same canonical temporal-rule surface as numeric phrases like `within 2 cycles`
- added regression tests for:
  - direct parser recovery of a single-cycle window from idiomatic one-cycle phrases
  - end-to-end temporal-rule derivation from a `next tick` signal constraint
- synced the roadmap/status/analysis/README/change docs so future sessions know the temporal layer now understands idiomatic one-cycle protocol language
- validation ran for this task:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with 138 tests

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/ir/semantic.rs`
  - `README.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. commit the idiomatic one-cycle temporal-language slice; do not stage `generated/`
2. continue `R15b` by broadening explicit temporal semantics beyond simple one-cycle idioms into richer tick-relative phrasing and arbitration
3. move on to `R15c` KG-guided multimodal rescans once the next temporal slice is landed cleanly

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, and multi-producer ambiguity (`R15d`)
- KG-quality evaluation and benchmark hardening (`R15e`)
- Tier 3 relation extraction after the graph/temporal/eval surfaces are ready (`R14`)
- some downstream compatibility and consumer paths still rely on flat `direction_hint` instead of the graph-native surface
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
