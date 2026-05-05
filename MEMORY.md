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
- latest_commit_hash: `6d38eb11ad34ab6b3ba6ec66aa2bc01845ee39ca`
- latest_commit_brief_message: `fix(kg): scope table-shape priors by family`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `6d38eb1` fix(kg): scope table-shape priors by family
- `87a69b7` test(kg): lock AMBA-generic modality prior fallback
- `7c9951b` fix(kg): scope semantic modality priors by family
- `bae8362` test(kg): lock modality-prior source scoping
- `3eff48b` fix(kg): keep weak semantic modality priors non-decisive
- `58504ca` fix(kg): reject broad semantic phrase priors

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 12]`
- files in flight for new batch slice 13:
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/src/ir/prior_memory.rs`
  - `crates/specforge/test_data/kg_quality/visual_motif_prior_protocol_family_mismatch_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/visual_motif_prior_protocol_family_mismatch_negative/axi_source.md`
  - `corpus_kb/benchmarks/kg-fixtures.md`
  - `corpus_kb/patterns/kg-fixtures.md`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.md`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.json`
  - `corpus_kb/visuals/kg-fixtures.md`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `12`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add a KG negative fixture proving APB visual-motif priors cannot classify AXI-local unknown visual assets
  - reuse exact-family plus AMBA-generic lookup for visual motif prior consumption
  - preserve AMBA-generic visual motif fallback through an AXI-family unit-test source
  - refresh corpus-KB projections for the expanded KG fixture, visual, truthfulness, and prior-candidate surfaces
- tracker effect:
  - live-status tracker gains `KG benchmark harness now locks visual-motif prior protocol-family scoping: Done`
- verification status:
  - pre-fix `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench visual_motif_prior_protocol_family_mismatch_negative visual_motif_prior_guided_diagram_classification_gold` reproduced APB visual-motif prior leakage into an AXI fixture
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench visual_motif_prior_protocol_family_mismatch_negative visual_motif_prior_guided_diagram_classification_gold table_shape_prior_protocol_family_mismatch_negative` passed with `3` fixtures and `0` failures
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed with `144` fixtures and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - first `bash scripts/run_ci.sh` run caught the stale unknown-family unit-test setup for visual motif fallback; source name was updated to infer AXI
  - `cargo test --manifest-path Cargo.toml -p specforge visual_motif_priors_classify_unknown_captioned_visual_assets` passed with `1` test
  - final `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `144` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `144/144` tracked KG fixtures

## Next exact steps
- commit slice 13 without pushing, then continue slice 14
