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
- latest_commit_hash: `5e9de711fb00a4154a7946c16bb367cd9f295dac`
- latest_commit_brief_message: `test(kg): add valid-like semantic prior pair`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `5e9de71` test(kg): add valid-like semantic prior pair
- `6627e06` docs(memory): start N=100 batch
- `f22692c` test(rescan): lock unsupported nlp provider args
- `9541a6f` test(rescan): lock duplicate nlp provider hints
- `4a07ae6` test(rescan): lock intent extra args
- `d3a171b` test(rescan): lock command lane mismatches

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 2]`
- files in flight for new batch slice 3:
  - `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_gold/fixture.json`
  - `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_gold/source.md`
  - `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_without_prior_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_without_prior_negative/source.md`
  - `corpus_kb/benchmarks/kg-fixtures.md`
  - `corpus_kb/patterns/kg-fixtures.md`
  - `corpus_kb/prior_memory/kg-fixtures.md`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `2`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add paired KG fixtures for prior-guided ready-like sink phrase recovery
  - prove the ambiguous ready-like phrase resolves only when typed prior memory supports it
  - refresh corpus-KB projections for the expanded KG fixture corpus
- tracker effect:
  - live-status tracker gains `KG benchmark harness now locks prior-guided ready-like sink phrase recovery: Done`
- verification status:
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench semantic_ready_sink_prior_guided_phrase_gold semantic_ready_sink_prior_guided_phrase_without_prior_negative` passed with `2` fixtures and `0` failures
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed with `134` fixtures and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `134` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `134/134` tracked KG fixtures

## Next exact steps
- commit slice 3 without pushing, then continue slice 4
