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
- latest_commit_hash: `45bb7f5c8339f280f17e80a8fde9a6a8c9d4cfa6`
- latest_commit_brief_message: `test(rescan): lock nlp classify flag rejection`
- note: new local `N=40` batch is active; push remains deferred until all 40 slices complete

## Recent commit chain (last 6)
- `45bb7f5` test(rescan): lock nlp classify flag rejection
- `15500ea` test(rescan): lock command trust boundary
- `0b001d9` test(rescan): lock pending status selection
- `c2f4c96` test(rescan): lock exact document filtering
- `c1a16ca` test(rescan): lock grade removal status
- `1bda030` test(rescan): lock score removal status

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 27]`
- files in flight for new batch slice 28:
  - `crates/specforge/src/commands/rescan_plan.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`

## Active N-slice batch
- requested_count: `40`
- completed_count: `27`
- push_policy: defer push until all `40` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add rescan-plan parser coverage proving unhyphenated lmstudio provider hints parse as LM Studio
  - refresh live docs for LM Studio provider alias behavior
- tracker effect:
  - planned live-status row addition: `specforge rescan-plan` unhyphenated LM Studio provider hints are regression-locked: `Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_parses_unhyphenated_lmstudio_provider_hint` passed with `1` test and `585` filtered out
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `586` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `585` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 28 without pushing, then continue slice 29
