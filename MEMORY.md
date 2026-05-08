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
- latest_commit_hash: `08aee3fe5dc9127c500d33ffcd1b4df8b5c4a08e`
- latest_commit_brief_message: `test(kg): lock APB requester/completer prose exclusion`
- note: this is the pre-slice-5 baseline for the active `BWFSC=200` batch; local commits are ahead of `origin/main`, and push is deferred until all 200 batch slices complete

## Recent commit chain (last 6)
- `08aee3f` test(kg): lock APB requester/completer prose exclusion
- `456a436` test(kg): lock APB response VLM exclusion
- `0238a87` test(kg): lock APB response visual exclusion
- `6359333` test(kg): lock APB response alias exclusion
- `5b47582` docs: reconcile completed BWFSC status
- `051e4ee` test(kg): lock APB response prose exclusion

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before current slice commit: `main` has local work for active `BWFSC=200` slice 5 and push is deferred; branch started this batch one docs-only commit ahead of `origin/main`
- files in flight:
  - `crates/specforge/test_data/kg_quality/apb_requester_completer_handshake_gold/fixture.json`
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
  - completed_count before this commit: `4`
  - slice 5/200 tightens APB requester/completer alias-grounded prose semantic-hint exclusion metrics for positive table-backed source-split hardening
  - push is deferred until all `200` slices complete unless explicitly instructed otherwise or a real blocker stops the batch
- tracker effect:
  - live-status tracker now marks APB requester/completer alias-grounded prose semantic-hint exclusion KG coverage as `Done`
- verification status:
  - implementation and live-doc sync are complete for slice 5 before commit
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench apb_requester_completer_handshake_gold` passed with the requested fixture
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed on slice 3 with `150/150` fixtures
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed on slice 3 with `150` fixtures projected, `0` failures, and no tracked projection diff
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
- finish the slice 5 commit workflow, clear and verify `git_message_brief.txt`, then continue APB requester/completer visual-caption source-split hardening
- keep `cargo sweep --time 1` deferred until the active `target/release/tool_matrix` process exits, then run it when the target tree is idle
