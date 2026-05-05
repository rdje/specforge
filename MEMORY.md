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
- active batch-run rule: for the new `N=20` batch, commit after every slice but defer push until all 20 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `410dab184e13829acb97ac3849714b9a5e9fd0cf`
- latest_commit_brief_message: `test(rescan): lock selected dry-run rows`
- note: new local `N=20` batch is active; push remains deferred until all 20 slices complete

## Recent commit chain (last 6)
- `410dab1` test(rescan): lock selected dry-run rows
- `fa44295` test(rescan): lock empty dry-run queue
- `0a8bcbd` test(rescan): lock scoped unlimited queues
- `ee821d1` test(rescan): lock oversized queue limits
- `ba5c8fa` test(rescan): lock dry-run command display
- `1c0b88c` test(rescan): lock display trust boundary

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 11]`
- files in flight for new batch slice 12:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/rescan_plan.rs`

## Active N-slice batch
- requested_count: `20`
- completed_count: `11`
- push_policy: defer push until all `20` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - lock `rescan-plan` dry-run rendering order against the selected index list
  - prove selected recommendations render in caller-provided order
- tracker effect:
  - add a `Done` live-status row for selected-order dry-run rendering
- verification status:
  - focused test passed: `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_dry_run_render_preserves_selected_index_order` (`1` test)
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `551` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` tracked KG fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `551` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 12 without pushing, then continue slice 13
