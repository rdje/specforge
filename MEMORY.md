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
- latest_commit_hash: `4f1b7af202d074dbee3f00f0d3621d8c541ab4ab`
- latest_commit_brief_message: `test(kg): lock semantic prior source scoping`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `4f1b7af` test(kg): lock semantic prior source scoping
- `767e508` test(kg): lock semantic prior conflict guard
- `33e456b` test(kg): add ready-like semantic prior pair
- `5e9de71` test(kg): add valid-like semantic prior pair
- `6627e06` docs(memory): start N=100 batch
- `f22692c` test(rescan): lock unsupported nlp provider args

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 5]`
- files in flight for new batch slice 6:
  - `crates/specforge/test_data/kg_quality/semantic_prior_phrase_mismatch_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/semantic_prior_phrase_mismatch_negative/source.md`
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
- completed_count: `5`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add a negative KG fixture proving semantic phrase priors require local phrase matching
  - prove an unrelated stored prior cannot resolve a different local prose phrase
  - refresh corpus-KB projections for the expanded KG fixture and prior-candidate surfaces
- tracker effect:
  - live-status tracker gains `KG benchmark harness now locks semantic phrase prior local phrase matching: Done`
- verification status:
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench semantic_prior_phrase_mismatch_negative` passed with `1` fixture and `0` failures
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed with `137` fixtures and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `137` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `137/137` tracked KG fixtures

## Next exact steps
- commit slice 6 without pushing, then continue slice 7
