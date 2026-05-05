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
- latest_commit_hash: `d2d6eb7ab39037e0f60fc683074285f31f29e9af`
- latest_commit_brief_message: `test(kg): lock semantic prior phrase matching`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `d2d6eb7` test(kg): lock semantic prior phrase matching
- `4f1b7af` test(kg): lock semantic prior source scoping
- `767e508` test(kg): lock semantic prior conflict guard
- `33e456b` test(kg): add ready-like semantic prior pair
- `5e9de71` test(kg): add valid-like semantic prior pair
- `6627e06` docs(memory): start N=100 batch

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 6]`
- files in flight for new batch slice 7:
  - `crates/specforge/src/ir/prior_memory.rs`
  - `crates/specforge/test_data/kg_quality/semantic_prior_broad_phrase_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/semantic_prior_broad_phrase_negative/source.md`
  - `corpus_kb/benchmarks/kg-fixtures.md`
  - `corpus_kb/patterns/kg-fixtures.md`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.md`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.json`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `6`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - harden semantic phrase prior lookup against one-token broad prior records
  - add a negative KG fixture proving broad loaded priors cannot resolve unrelated local prose
  - refresh corpus-KB projections for the expanded KG fixture and prior-candidate surfaces
- tracker effect:
  - live-status tracker gains `Semantic phrase prior lookup rejects one-token broad prior records: Done`
- verification status:
  - pre-fix focused run reproduced the broad-prior false positive in `semantic_prior_broad_phrase_negative`
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench semantic_prior_broad_phrase_negative semantic_prior_guided_phrase_gold semantic_valid_prior_guided_phrase_gold semantic_ready_sink_prior_guided_phrase_gold visual_semantic_prior_guided_caption_gold` passed with `5` fixtures and `0` failures after the fix
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed with `138` fixtures and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `138` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `138/138` tracked KG fixtures

## Next exact steps
- commit slice 7 without pushing, then continue slice 8
