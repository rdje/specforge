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
- active batch-run rule: for the new `N=100` batch, commit after every slice but defer push until all 100 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `92022ba6befd4554250b313e9cf09cf989442146`
- latest_commit_brief_message: `test(adapter): lock mixed child top support`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `92022ba` test(adapter): lock mixed child top support
- `f18a8eb` test(adapter): lock reused FSM child top support
- `1d3e2bc` test(adapter): lock reused child top support
- `87ff7ed` test(adapter): lock baseline top composition support
- `73b0c96` test(adapter): lock child top-root confidence support
- `57e29a6` test(adapter): lock recovered top-root confidence support

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 75]`
- files in flight for new batch slice 76:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `75`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - regression-lock single FSM-child top-composition child provenance
  - prove selected top candidates preserve the single FSM child support IDs while FSM root kind and top-first direct-root order stay stable
  - keep child declaration evidence auditable on the simplest explicit top plus FSM-child document path
- tracker effect:
  - live-status tracker gains `.fsm single FSM-child top lowering now regression-locks child support IDs: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge renderable_top_document_preserves_fsm_child_root_kind` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `80` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed before the final live-doc refresh
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` tracked fixtures
  - `bash scripts/run_ci.sh` passed with formatting, warning-deny Clippy, `601` Rust tests, warning-deny Rust docs, and mdBook build
- current known local CI baseline:
  - `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation expected after this slice
  - `148/148` tracked KG fixtures

## Next exact steps
- run final docs CI and guards, commit slice 76 without pushing, and continue slice 77
