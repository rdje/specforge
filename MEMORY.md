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
- latest_commit_hash: `3eff48b8e6b6774644dc97ae171dd96c5b329eba`
- latest_commit_brief_message: `fix(kg): keep weak semantic modality priors non-decisive`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `3eff48b` fix(kg): keep weak semantic modality priors non-decisive
- `58504ca` fix(kg): reject broad semantic phrase priors
- `d2d6eb7` test(kg): lock semantic prior phrase matching
- `4f1b7af` test(kg): lock semantic prior source scoping
- `767e508` test(kg): lock semantic prior conflict guard
- `33e456b` test(kg): add ready-like semantic prior pair

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 8]`
- files in flight for new batch slice 9:
  - `crates/specforge/test_data/kg_quality/semantic_modality_reliability_source_kind_mismatch_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/semantic_modality_reliability_source_kind_mismatch_negative/source.md`
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
- completed_count: `8`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add a KG negative fixture proving semantic modality-reliability priors stay source-kind scoped
  - show a visual-caption reliability prior cannot resolve prose/table semantic conflicts
  - refresh corpus-KB projections for the expanded KG fixture and prior-memory surfaces
- tracker effect:
  - live-status tracker gains `KG benchmark harness now locks semantic modality-prior source-kind scoping: Done`
- verification status:
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench semantic_modality_reliability_source_kind_mismatch_negative semantic_modality_reliability_prior_guided_conflict_gold semantic_modality_reliability_weak_prior_negative` passed with `3` fixtures and `0` failures
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed with `140` fixtures and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `140` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `140/140` tracked KG fixtures

## Next exact steps
- commit slice 9 without pushing, then continue slice 10
