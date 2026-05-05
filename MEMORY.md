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
- latest_commit_hash: `c3124f7897c373e5d7e34b535f8228c30a59c0f5`
- latest_commit_brief_message: `fix(rescan): reject absolute replay paths`
- note: new local `N=40` batch is active; push remains deferred until all 40 slices complete

## Recent commit chain (last 6)
- `c3124f7` fix(rescan): reject absolute replay paths
- `9e04bb9` test(rescan): lock skip classify-only replay
- `e22ee13` test(rescan): lock source enrich skip provider
- `251d098` test(rescan): lock open-ai provider rejection
- `914e09c` test(rescan): lock lmstudio provider spelling
- `45bb7f5` test(rescan): lock nlp classify flag rejection

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 32]`
- files in flight for new batch slice 33:
  - `crates/specforge/src/commands/rescan_plan.rs`
  - `docs/book/src/commands/quality-and-learning.md`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`

## Active N-slice batch
- requested_count: `40`
- completed_count: `32`
- push_policy: defer push until all `40` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - harden rescan-plan command-hint parsing so replay source/artifact paths cannot use `..` parent traversal
  - refresh live docs and mdBook reference for non-traversing relative path restrictions
- tracker effect:
  - planned live-status row addition: `specforge rescan-plan` parent-traversal replay artifact paths are rejected: `Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_rejects_parent_traversal_replay_artifact_paths` passed with `1` test and `590` filtered out
  - `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_execute_` passed with `4` tests and `587` filtered out after adding explicit execution-root resolution
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `591` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `590` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 33 without pushing, then continue slice 34
