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
- active batch-run rule: for this `N=20` batch, commit after every slice but defer push until all 20 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `679f7d98806ac1ccd91f18153abc0fa28800d1a0`
- latest_commit_brief_message: `test(rescan): lock malformed provider hints`
- note: the latest committed baseline is local batch-20 slice 7; push remains deferred until all `20` slices complete

## Recent commit chain (last 6)
- `679f7d9` test(rescan): lock malformed provider hints
- `c553a16` test(rescan): lock local provider hint variants
- `ec5de3b` test(rescan): lock command hint locality
- `e845035` test(rescan): lock scoped pending selection
- `2aed561` test(rescan): lock empty dry-run preview fields
- `544f326` test(rescan): lock dry-run command preview

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 7]`
- files in flight for batch-20 slice 8:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `README.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/rescan_plan.rs`
  - `docs/book/src/commands/quality-and-learning.md`
  - `docs/book/src/quality/validation.md`
  - `docs/book/src/reference/generated-artifacts.md`

## Active N-slice batch
- requested_count: `20`
- completed_count: `7`
- push_policy: defer push until all `20` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - lock deterministic added/removed finding lists in `rescan-plan` validation deltas
  - ensure execution summaries sort and deduplicate finding IDs before review projection
- tracker effect:
  - add a `Done` live-status row for validation-delta finding list determinism
- verification status:
  - focused test passed: `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_validation_delta_sorts_and_deduplicates_finding_changes`
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `528` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 8 without pushing, then move to slice 9
