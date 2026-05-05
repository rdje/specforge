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
- latest_commit_hash: `baf51dae3fc0f834432d7dccfdc04224d46aa5ac`
- latest_commit_brief_message: `test(rescan): lock removed-finding arbitration`
- note: new local `N=40` batch is active; push remains deferred until all 40 slices complete

## Recent commit chain (last 6)
- `baf51da` test(rescan): lock removed-finding arbitration
- `fb8d416` test(rescan): lock command hint render order
- `08a81d7` test(rescan): lock unscoped pending limits
- `9a77729` docs(memory): start N=40 batch
- `0e108e7` test(rescan): lock scoped pending limits
- `bf7d825` test(rescan): lock no-change promotion gate

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 4]`
- files in flight for new batch slice 5:
  - `crates/specforge/src/commands/rescan_plan.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`

## Active N-slice batch
- requested_count: `40`
- completed_count: `4`
- push_policy: defer push until all `40` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add rescan-plan arbitration coverage proving added findings alone produce regression review
  - refresh live docs for the isolated added-finding verdict behavior
- tracker effect:
  - planned live-status row addition: `specforge rescan-plan` added findings force regression review: `Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_arbitration_treats_added_findings_as_regression` passed with `1` test
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `563` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `563` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 5 without pushing, then continue slice 6
