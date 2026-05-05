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
- active batch-run rule: for the new `N=40` batch, commit after every slice but defer push until all 40 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `0fe7f908860b8b7721ef2489ae5dfc8be5b9f53c`
- latest_commit_brief_message: `test(rescan): lock grade-only arbitration`
- note: new local `N=40` batch is active; push remains deferred until all 40 slices complete

## Recent commit chain (last 6)
- `0fe7f90` test(rescan): lock grade-only arbitration
- `fb2f5ba` test(rescan): lock execute document scoping
- `a2b9621` test(rescan): lock execute limit handling
- `697d9cb` test(rescan): lock execute report no-change
- `c6a3bfc` test(rescan): lock dry-run report counts
- `97bd187` test(rescan): lock replay-input rendering

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 18]`
- files in flight for new batch slice 19:
  - `crates/specforge/src/commands/rescan_plan.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`

## Active N-slice batch
- requested_count: `40`
- completed_count: `18`
- push_policy: defer push until all `40` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add rescan-plan execution-status coverage proving grade-only validation changes count as changed execution
  - refresh live docs for grade-only execution-status behavior
- tracker effect:
  - planned live-status row addition: `specforge rescan-plan` grade-only execution status is changed: `Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_execution_status_treats_grade_only_delta_as_changed` passed with `1` test
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `577` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `577` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 19 without pushing, then continue slice 20
