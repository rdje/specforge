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
- latest_commit_hash: `3a84112a326604ddfe5c0a3a56f624cadd95aa1d`
- latest_commit_brief_message: `test(kg): lock mixed direction gap findings`
- note: the latest committed baseline is batch slice 2/10; it locks mixed graph-backed lag plus unresolved compatibility-direction related-id separation

## Recent commit chain (last 6)
- `3a84112` test(kg): lock mixed direction gap findings
- `b487152` docs(workflow): document batch push policy
- `62d09a8` test(kg): lock unresolved direction gaps
- `4587325` chore(fsmgen): refresh submodule contract baseline
- `c151bdc` fix(validate): split intent direction gap findings
- `5fb059e` test(adapter): lock fsm graph undriven outputs

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 26` of `origin/main`
- files in flight for slice 3:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `README.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/validate.rs`
  - `crates/specforge/test_data/kg_quality/compat_direction_hints_graph_conflict_incomplete_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/compat_direction_hints_graph_conflict_incomplete_negative/source.md`
  - `crates/specforge/test_data/kg_quality/graph_direction_same_actor_conflict_negative/fixture.json`
  - `docs/book/src/quality/kg-bench.md`

## Active N-slice batch
- requested_count: `10`
- completed_count: `2`
- push_policy: defer push until all `10` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add a conflicted graph-direction KG fixture where same-actor actor ports disagree on `PREADY`
  - keep conflicted graph evidence excluded from resolved graph direction coverage while still reporting it as graph-coverage debt and unresolved compatibility-direction debt when the flat hint is missing
  - prove conflicted graph evidence does not produce graph-backed compatibility lag
- tracker effect:
  - add a Done row for conflicted graph-direction evidence being locked as unresolved direction coverage
- verification status:
  - focused `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench compat_direction_hints_graph_conflict_incomplete_negative` passed with `1/1` fixture
  - focused `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench graph_direction_same_actor_conflict_negative` passed with `1/1` fixture
  - focused `cargo test --manifest-path Cargo.toml -p specforge graph_direction` passed with `12` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - first `bash scripts/run_ci.sh` attempt failed because the older `graph_direction_same_actor_conflict_negative` fixture still excluded graph-coverage debt; the fixture has been updated and focused KG validation now passes
  - final `bash scripts/run_ci.sh` passed with `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 3 without pushing
