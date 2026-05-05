# MEMORY
## Purpose
- maintain a compact, actionable continuity record for interrupted sessions
- preserve enough context to resume cleanly after session loss, tool restart, machine crash, or model handoff

## Current project identity
- repository name: `specforge`
- CLI/binary name: `specforge`
- implementation language: Rust
- canonical deliverable: `IntentIR`
- stage model: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## User-required continuity rules
- follow `COMMIT.md` strictly after every completed slice
- sync all relevant tracked live docs before commit, not as an afterthought
- `git_message_brief.txt` must stay untracked and be truncated to `0` bytes after each commit
- every completion message must report the commit id, exact commit message, full tracked-file list, current live-status snapshot, and whether that snapshot changed
- outside explicit batch runs, do not push unless the user asks or the branch reaches `25` local commits since the last push
- active batch-run rule: for this `N=10` batch, commit after every slice but defer push until all 10 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `7beb26588c91c6bda1497838a0e96877aee9c0ad`
- latest_commit_brief_message: `test(validation): preserve graph replay target split`
- note: the latest committed baseline is batch slice 5/10; it locks concurrent SemanticIR graph-direction coverage and conflict replay recommendations

## Recent commit chain (last 6)
- `7beb265` test(validation): preserve graph replay target split
- `741267d` test(kg): lock flat-hint graph conflict boundary
- `d6751ed` fix(validate): treat graph conflicts as unresolved direction
- `3a84112` test(kg): lock mixed direction gap findings
- `b487152` docs(workflow): document batch push policy
- `62d09a8` test(kg): lock unresolved direction gaps

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 29` of `origin/main`
- files in flight for slice 6:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/project_validation.rs`
  - `docs/book/src/quality/validation.md`

## Active N-slice batch
- requested_count: `10`
- completed_count: `5`
- push_policy: defer push until all `10` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add the IntentIR-stage sibling regression for one report carrying both graph-direction coverage and graph-direction conflict replay guidance
  - prove both replay recommendations survive with separate related ids and the expected EvidenceIR plus SemanticIR replay inputs
- tracker effect:
  - add a Done row for concurrent IntentIR graph-direction conflict and coverage replay targets staying preserved
- verification status:
  - focused `cargo test --manifest-path Cargo.toml -p specforge project_validation_keeps_graph_direction_conflict_and_coverage_replays_for_intent_stage` passed with `1` test
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 6 without pushing
