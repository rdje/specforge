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
- latest_commit_hash: `8bbfe1f`
- latest_commit_brief_message: `feat(semantic): add typed temporal rules`
- note: current uncommitted work adds cycle-window recovery to the typed temporal-rule layer, updates validation to count bounded temporal rules, and syncs the continuity docs to that state

## Recent commit chain (last 5)
- `8bbfe1f` feat(semantic): add typed temporal rules
- `fbd0310` feat(validation): score direction from graph first
- `ddd2cac` docs: capture multimodal extraction steering
- `1064bec` feat(converge): default to full ollama loop
- `894c538` feat(validation): project reports into live docs
- `fc66933` feat(ir): preserve kg and backannotate validation
- `e8aae43` feat(core): add convergent IR pipeline loop

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports

## Completed technical work in this session
- the typed temporal-rule layer now also recovers bounded `cycle_window` latency from prose such as `within 2 cycles`, `for 2 cycles`, `at least N cycles`, and from timing rows whose unit is already `cycles`
- `specforge validate` now reports `temporal_rules_with_cycle_window` and flags temporal-rule sets that still have no bounded latency at all
- added regression tests for:
  - cycle-window derivation from cycle-bounded signal constraints
  - cycle-window carry-through into `IntentIR`
  - validation metrics for bounded temporal rules
- synced the roadmap/status/analysis/README/change docs so future sessions know the temporal layer now carries bounded latency, not only edge predicates
- validation ran for this task:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with 115 tests

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/src/ir/intent.rs`
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
1. commit the cycle-window temporal slice; do not stage `generated/`
2. continue the remaining `R15` slice by moving direct downstream `direction_hint` consumers onto actor-relative graph semantics
3. deepen `R15b` with actor-relative drive events, richer temporal composition, and contradiction detection before moving on to `R15c`

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- evidence arbitration / conflict handling (`R15d`)
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
7. continue with the remaining graph-first `R15` consumer migration and the next temporal-deepening slice unless the user redirects
