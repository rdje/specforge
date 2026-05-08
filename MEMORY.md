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
- active batch-run rule: the current batch uses `BWFSC=200`, must commit after every slice, and must defer push until all 200 slices are complete unless the user explicitly instructs otherwise or a real blocker stops the batch
- latest completed batch-run rule: the `BWFSC=100` batch committed each slice independently, synced live docs and mdBook at the end of every slice, deferred push until all 100 slices were complete, and was pushed after slice 100
- previous completed batch-run rule: the `N=200` batch committed each slice independently, synced live docs and mdBook at the end of every slice, deferred push until all 200 new-batch slices were complete, and was pushed after slice 200
- previous batch-run rule used: the `N=100` batch committed each slice independently, deferred push until all 100 slices were complete, and was pushed after slice 100
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `f4ed5ac73aa3f22e64a60d178d7e04cc423789e1`
- latest_commit_brief_message: `test(kg): lock AXI read-data response visual exclusion`
- note: this is the pre-slice-31 baseline for the active `BWFSC=200` batch; local commits are ahead of `origin/main`, and push is deferred until all 200 batch slices complete

## Recent commit chain (last 6)
- `f4ed5ac` test(kg): lock AXI read-data response visual exclusion
- `4b34497` test(kg): lock AXI read-data response alias exclusion
- `8883c38` test(kg): lock AXI read-data response prose exclusion
- `fbeb15a` test(kg): lock AXI write-data VLM exclusion
- `5d47641` test(kg): lock AXI write-data visual exclusion
- `46bc272` test(kg): lock AXI write-data alias exclusion

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before current slice commit: `main` has local work for active `BWFSC=200` slice 31 and push is deferred; branch started this batch one docs-only commit ahead of `origin/main`
- files in flight:
  - `crates/specforge/test_data/kg_quality/axi_read_data_response_stability_gold/fixture.json`
  - live docs and mdBook files synced for the slice

## Latest completed N-slice batch
- requested_count: `100`
- completed_count: `100`
- push_policy: completed; pushed once after all `100` batch slices were committed
- slice_rule: each slice received verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice started
- final_commit: `051e4ee98f6892409551e69828e633b546759a96`

## Current batch status
- objective:
  - active `BWFSC=200` batch is in progress
  - completed_count before this commit: `30`
  - slice 31/200 tightens AXI read-data response VLM timing-annotation semantic-hint exclusion metrics for positive table-backed source-split hardening
  - push is deferred until all `200` slices complete unless explicitly instructed otherwise or a real blocker stops the batch
- tracker effect:
  - live-status tracker now marks AXI read-data response VLM timing-annotation semantic-hint exclusion KG coverage as `Done`
- verification status:
  - implementation and live-doc sync are complete for slice 31 before commit
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench axi_read_data_response_stability_gold` passed with the requested fixture
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed on slice 31 with `150/150` fixtures
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed on slice 31 with `150` fixtures projected, `0` failures, and no tracked projection diff
  - `bash scripts/run_ci.sh` passed on slice 30 as a broader checkpoint gate
  - `bash scripts/run_ci.sh` passed on slice 20 as a broader checkpoint gate
  - `bash scripts/run_ci.sh` passed after the slice 100 live-doc and live-book sync
  - previous batch `git push` completed from `2921ba8e` to `051e4ee9` on `main`
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
  - docs CI passed for the current slice after live-doc sync
  - slice 3 passed `150/150` tracked KG fixtures
  - slice 3 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 7 passed `150/150` tracked KG fixtures
  - slice 7 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 11 passed `150/150` tracked KG fixtures
  - slice 11 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 15 passed `150/150` tracked KG fixtures
  - slice 15 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 19 passed `150/150` tracked KG fixtures
  - slice 19 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 23 passed `150/150` tracked KG fixtures
  - slice 23 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 27 passed `150/150` tracked KG fixtures
  - slice 27 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 28 passed `150/150` tracked KG fixtures
  - slice 28 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 29 passed `150/150` tracked KG fixtures
  - slice 29 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 30 passed `150/150` tracked KG fixtures
  - slice 30 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 30 passed `bash scripts/run_ci.sh`
  - slice 31 passed `150/150` tracked KG fixtures
  - slice 31 passed corpus-KB projection for `150` fixtures with `0` failures and no tracked projection diff
  - slice 20 passed `bash scripts/run_ci.sh`
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
- finish the slice 31 commit workflow, clear and verify `git_message_brief.txt`, then select the next roadmap-aligned source-split hardening fixture
- keep `cargo sweep --time 1` deferred until the active `target/release/tool_matrix` process exits, then run it when the target tree is idle
