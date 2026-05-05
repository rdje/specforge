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
- latest_commit_hash: `0f57e18d8d3c88473405a4100a14381af294e011`
- latest_commit_brief_message: `test(rescan): lock stage command parsing`
- note: new local `N=20` batch is active; push remains deferred until all 20 slices complete

## Recent commit chain (last 6)
- `0f57e18` test(rescan): lock stage command parsing
- `f7c55ef` docs(memory): start next N=20 batch
- `1175e50` test(rescan): lock unknown command intents
- `7b4725d` test(rescan): lock intent lane mismatches
- `4ee80c8` test(rescan): lock unknown provider values
- `be4f608` test(rescan): lock flag-shaped provider values

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 2]`
- files in flight for new batch slice 3:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/rescan_plan.rs`

## Active N-slice batch
- requested_count: `20`
- completed_count: `2`
- push_policy: defer push until all `20` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - lock rejection of extra args on non-enrichment `rescan-plan` stage command hints
  - cover `ingest`, `evidence`, `semantic`, and `validate` exact-arity enforcement
- tracker effect:
  - add a `Done` live-status row for extra stage command arg rejection
- verification status:
  - focused test passed: `cargo test --manifest-path Cargo.toml -p specforge rescan_plan_rejects_extra_stage_command_args`
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `542` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `542` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- run slice 3 verification, update validation results, commit without pushing, then continue slice 4
