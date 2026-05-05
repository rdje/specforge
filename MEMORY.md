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
- latest_commit_hash: `146a576d1b6f1a0fd8073c4396ff54dd4ffd1792`
- latest_commit_brief_message: `test(kg): lock flat-hint conflict guidance payloads`
- note: the latest committed baseline is batch slice 9/10; it locks paired graph-coverage and graph-conflict rescan guidance payloads in the flat-hint-present graph conflict KG fixture

## Recent commit chain (last 6)
- `146a576` test(kg): lock flat-hint conflict guidance payloads
- `6531382` test(validate): lock graph conflict guidance pair
- `5d739ee` test(kg): lock graph conflict rescan payloads
- `21c4bcf` test(validation): preserve intent graph replay split
- `7beb265` test(validation): preserve graph replay target split
- `741267d` test(kg): lock flat-hint graph conflict boundary

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 33` of `origin/main`
- files in flight for slice 10:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/project_validation.rs`
  - `docs/book/src/quality/validation.md`

## Active N-slice batch
- requested_count: `10`
- completed_count: `9`
- push_policy: defer push until all `10` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - tighten the combined graph-direction replay preservation tests in `project-validation`
  - assert both coverage and conflict recommendations keep their expected command-intent chains for SemanticIR and IntentIR stages
- tracker effect:
  - add a Done row for graph replay command lanes staying preserved when coverage and conflict targets appear together
- verification status:
  - focused `cargo test --manifest-path Cargo.toml -p specforge project_validation_keeps_graph_direction_conflict_and_coverage_replays` passed with `2` tests
  - `cargo fmt --manifest-path Cargo.toml` applied the helper signature formatting
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- run full slice-10 gates, update validation notes if needed, then commit slice 10 and push the completed batch
