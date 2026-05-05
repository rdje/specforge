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
- latest_commit_hash: `f22692c33180ebb4ed3c10a2d5d0859d78f8bf6d`
- latest_commit_brief_message: `test(rescan): lock unsupported nlp provider args`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `f22692c` test(rescan): lock unsupported nlp provider args
- `9541a6f` test(rescan): lock duplicate nlp provider hints
- `4a07ae6` test(rescan): lock intent extra args
- `d3a171b` test(rescan): lock command lane mismatches
- `e42361d` test(rescan): lock missing replay path lanes
- `88751c6` fix(rescan): reject current-directory replay paths

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main`
- files in flight for new batch slice 1:
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `0`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - start the new `N=100` batch by refreshing crash-recovery state and deferred-push policy
  - keep the next implementation slice recoverable before any larger code changes
- tracker effect:
  - live-status tracker unchanged; batch policy is already tracked as Done
- verification status:
  - docs-only slice; no focused Rust regression was needed
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130` fixtures and `0` failures
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- validate and commit slice 1 without pushing, then continue slice 2
