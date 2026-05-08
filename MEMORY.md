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
- active batch-run rule: the current batch uses `BWFSC=100`, must commit after every slice, and must defer push until all 100 slices are complete unless the user explicitly instructs otherwise or a real blocker stops the batch
- latest completed batch-run rule: the `N=200` batch committed each slice independently, synced live docs and mdBook at the end of every slice, deferred push until all 200 new-batch slices were complete, and was pushed after slice 200
- previous batch-run rule used: the `N=100` batch committed each slice independently, deferred push until all 100 slices were complete, and was pushed after slice 100
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `b551fda2af40fb4c0398a8e65b8e241d27d86b20`
- latest_commit_brief_message: `test(kg): lock no-prior visual-motif table exclusion`
- note: this is the pre-slice-24 baseline for the current `BWFSC=100` batch; slices 1-23/100 are committed and push remains deferred

## Recent commit chain (last 6)
- `b551fda` test(kg): lock no-prior visual-motif table exclusion
- `3200811` test(kg): lock no-prior visual-caption VLM exclusion
- `69ecd4b` test(kg): lock no-prior visual-caption alias exclusion
- `4d4c95c` test(kg): lock no-prior visual-caption prose exclusion
- `8585595` test(kg): lock no-prior visual-caption table exclusion
- `aa06c51` test(kg): lock field-table visual-hint exclusion

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before current slice commit: `main` has local work for current `BWFSC=100` slice 24 and push is deferred
- files in flight:
  - `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json`
  - live docs and mdBook files synced for the slice

## Previous completed N-slice batch
- requested_count: `200`
- completed_count: `200`
- push_policy: completed; pushed once after all `200` new-batch slices were committed
- slice_rule: each slice received verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice started
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current batch status
- objective:
  - active `BWFSC=100` batch is in progress
  - completed_count before this commit: `23`
  - slice 24/100 tightens no-prior visual-motif ordinary prose semantic-hint exclusion metrics
  - push is deferred until all `100` slices complete unless explicitly instructed otherwise or a real blocker stops the batch
- tracker effect:
  - live-status tracker now marks no-prior visual-motif ordinary prose semantic-hint exclusion KG coverage as `Done`
- verification status:
  - implementation and live-doc sync are complete for slice 24 before commit
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench visual_motif_prior_guided_diagram_classification_without_prior_negative` passed with the requested fixture
  - `bash scripts/run_docs_ci.sh` passed after the live-book sync
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed on slice 22 with `150/150` fixtures
  - broader `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed on slice 22 with `150` fixtures projected and `0` failures
  - broader `bash scripts/run_ci.sh` passed on slice 20 with formatting, Clippy warning-deny, `614` Rust tests, rustdoc warning-deny, and mdBook
  - user-requested `cargo sweep --time 1` is deferred until no target-tree process is active because `target/release/tool_matrix` is active
- current known local CI baseline:
  - focused current-slice KG fixture passed
  - current slice passed docs CI
  - slice 22 passed `150/150` tracked KG fixtures
  - slice 22 passed corpus-KB projection for `150` fixtures with `0` failures
  - slice 20 passed `bash scripts/run_ci.sh`
  - latest `cargo sweep --time 1` attempt was deferred because `target/release/tool_matrix` is active
  - message file is currently untracked and `0` bytes

## Next exact steps
- finish the slice 24 commit workflow, clear and verify `git_message_brief.txt`, then continue to slice 25/100
- keep `cargo sweep --time 1` deferred until the active `target/release/tool_matrix` process exits, then run it when the target tree is idle
