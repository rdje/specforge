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
- latest_commit_hash: `4916c6b690272798922363b87e2660010e8adc43`
- latest_commit_brief_message: `docs(memory): start N=20 batch state`
- note: the latest committed baseline is local batch-20 slice 1; push remains deferred until all `20` slices complete

## Recent commit chain (last 6)
- `4916c6b` docs(memory): start N=20 batch state
- `a15d861` test(validation): lock graph replay command lanes
- `146a576` test(kg): lock flat-hint conflict guidance payloads
- `6531382` test(validate): lock graph conflict guidance pair
- `5d739ee` test(kg): lock graph conflict rescan payloads
- `21c4bcf` test(validation): preserve intent graph replay split

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 1]`
- files in flight for batch-20 slice 2:
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
- completed_count: `1`
- push_policy: defer push until all `20` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - lock the `rescan-plan` dry-run preview so human-visible pending recommendations retain artifact path, extractor lane, replay inputs, action text, related ids, automation status, and command hints
  - keep the preview regression aligned with schema-v2 replay records before moving to the next replay hardening slice
- tracker effect:
  - add a `Done` live-status row for the `rescan-plan` dry-run preview field lock
- verification status:
  - focused test passed: `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_dry_run_render_surfaces_replay_boundary_and_action`
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed after applying `cargo fmt`
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 2 without pushing, then move to slice 3
