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
- no active batch run is currently in progress
- latest completed batch-run rule: the `BWFSC=100` batch committed each slice independently, synced live docs and mdBook at the end of every slice, deferred push until all 100 slices were complete, and was pushed after slice 100
- previous completed batch-run rule: the `N=200` batch committed each slice independently, synced live docs and mdBook at the end of every slice, deferred push until all 200 new-batch slices were complete, and was pushed after slice 200
- previous batch-run rule used: the `N=100` batch committed each slice independently, deferred push until all 100 slices were complete, and was pushed after slice 100
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest completed batch baseline
- latest_batch_final_commit_hash: `051e4ee98f6892409551e69828e633b546759a96`
- latest_batch_final_commit_brief_message: `test(kg): lock APB response prose exclusion`
- note: this is the pushed post-`BWFSC=100` baseline; slices 1-100/100 are committed and `main` is aligned with `origin/main`

## Recent batch commit chain (last 6)
- `051e4ee` test(kg): lock APB response prose exclusion
- `1140fd4` test(kg): lock APB write-control VLM exclusion
- `4aeb62a` test(kg): lock APB write-control visual exclusion
- `f38aee6` test(kg): lock APB write-control alias exclusion
- `78979fb` test(kg): lock APB write-control prose exclusion
- `80bc072` test(kg): lock APB setup access VLM exclusion

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state: `main` is aligned with `origin/main` after the completed `BWFSC=100` push
- files in flight: none

## Latest completed N-slice batch
- requested_count: `100`
- completed_count: `100`
- push_policy: completed; pushed once after all `100` batch slices were committed
- slice_rule: each slice received verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice started
- final_commit: `051e4ee98f6892409551e69828e633b546759a96`

## Current batch status
- objective:
  - no active batch workflow is currently in progress
  - latest `BWFSC=100` batch completed `100/100` slices
  - final slice tightened APB response prose semantic-hint exclusion metrics for positive table-backed source-split hardening
  - push completed after the final slice commit workflow
- tracker effect:
  - live-status tracker now marks APB response prose semantic-hint exclusion KG coverage as `Done`
- verification status:
  - final slice implementation and live-doc sync are complete and pushed
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench apb_response_stability_gold` passed with the requested fixture
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed on slice 99 with `150/150` fixtures
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed on slice 99 with `150` fixtures projected, `0` failures, and no tracked projection diff
  - `bash scripts/run_ci.sh` passed after the slice 100 live-doc and live-book sync
  - `git push` completed from `2921ba8e` to `051e4ee9` on `main`
  - `bash scripts/run_ci.sh` passed on slice 90 with formatting, Clippy warning-deny, Rust tests, rustdoc warning-deny, and mdBook
  - `bash scripts/run_ci.sh` passed on slice 80 with formatting, Clippy warning-deny, Rust tests, rustdoc warning-deny, and mdBook
  - `bash scripts/run_ci.sh` passed on slice 70 with formatting, Clippy warning-deny, Rust tests, rustdoc warning-deny, and mdBook
  - `bash scripts/run_ci.sh` passed on slice 60 with formatting, Clippy warning-deny, `614` Rust tests, rustdoc warning-deny, and mdBook
  - broader `bash scripts/run_ci.sh` passed on slice 50 with formatting, Clippy warning-deny, `614` Rust tests, rustdoc warning-deny, and mdBook
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed on slice 43 with `150/150` fixtures
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed on slice 43 with `150` fixtures projected and `0` failures
  - broader `bash scripts/run_ci.sh` passed on slice 40 with formatting, Clippy warning-deny, `614` Rust tests, rustdoc warning-deny, and mdBook
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed on slice 39 with `150/150` fixtures
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed on slice 39 with `150` fixtures projected and `0` failures
  - source-split metric scan for fixtures with `signal_semantic_hints = 0` produced no remaining fixtures missing split metrics on slice 35
  - broader `bash scripts/run_ci.sh` passed on slice 30 with formatting, Clippy warning-deny, `614` Rust tests, rustdoc warning-deny, and mdBook
  - user-requested `cargo sweep --time 1` is deferred until no target-tree process is active because `target/release/tool_matrix` is active
- current known local CI baseline:
  - focused current-slice KG fixture passed
  - full local CI passed for the current slice after live-doc sync
  - slice 100 passed `bash scripts/run_ci.sh`
  - slice 90 passed `bash scripts/run_ci.sh`
  - slice 80 passed `bash scripts/run_ci.sh`
  - slice 70 passed `bash scripts/run_ci.sh`
  - slice 99 passed `150/150` tracked KG fixtures
  - slice 99 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 95 passed `150/150` tracked KG fixtures
  - slice 95 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 60 passed `bash scripts/run_ci.sh`
  - slice 43 passed `150/150` tracked KG fixtures
  - slice 43 passed corpus-KB projection for `150` fixtures with `0` failures
  - slice 40 passed `bash scripts/run_ci.sh`
  - slice 39 passed `150/150` tracked KG fixtures
  - slice 39 passed corpus-KB projection for `150` fixtures with `0` failures
  - zero-total semantic-hint source-split scan is complete with no missing split metrics
  - latest `cargo sweep --time 1` attempt was deferred because `target/release/tool_matrix` is active
  - message file is currently untracked and `0` bytes

## Next exact steps
- report current roadmap status from `ROADMAP.md` and `LIVE_ACHIEVEMENT_STATUS.md`
- if a new batch is requested, pick the next roadmap-aligned source-split hardening slice and follow `COMMIT.md`
- keep `cargo sweep --time 1` deferred until the active `target/release/tool_matrix` process exits, then run it when the target tree is idle
