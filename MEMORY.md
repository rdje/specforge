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
- latest_commit_hash: `be3b937f2600ff790e6e10842c71f735067f33ca`
- latest_commit_brief_message: `fix(kg): scope visual-motif priors by family`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `be3b937` fix(kg): scope visual-motif priors by family
- `6d38eb1` fix(kg): scope table-shape priors by family
- `87a69b7` test(kg): lock AMBA-generic modality prior fallback
- `7c9951b` fix(kg): scope semantic modality priors by family
- `bae8362` test(kg): lock modality-prior source scoping
- `3eff48b` fix(kg): keep weak semantic modality priors non-decisive

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 13]`
- files in flight for new batch slice 14:
  - `crates/specforge/src/ir/prior_memory.rs`
  - `crates/specforge/test_data/kg_quality/temporal_prior_protocol_family_mismatch_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/temporal_prior_protocol_family_mismatch_negative/axi_temporal_prior_mismatch.md`
  - `corpus_kb/benchmarks/kg-fixtures.md`
  - `corpus_kb/patterns/kg-fixtures.md`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.md`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.json`
  - `corpus_kb/timing/kg-fixtures.md`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `13`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add a KG negative fixture proving APB temporal phrase priors cannot add cycle windows to AXI-local timing rules
  - reuse exact-family plus AMBA-generic lookup for temporal phrase cycle-window prior consumption
  - refresh corpus-KB projections for the expanded KG fixture, timing, truthfulness, and prior-candidate surfaces
- tracker effect:
  - live-status tracker gains `KG benchmark harness now locks temporal-prior protocol-family scoping: Done`
- verification status:
  - pre-fix `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench temporal_prior_protocol_family_mismatch_negative temporal_prior_guided_cycle_window_gold` reproduced APB temporal prior leakage into an AXI fixture
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench temporal_prior_protocol_family_mismatch_negative temporal_prior_guided_cycle_window_gold temporal_prior_guided_cycle_window_without_prior_negative visual_motif_prior_protocol_family_mismatch_negative` passed with `4` fixtures and `0` failures
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed with `145` fixtures and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `145` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `145/145` tracked KG fixtures

## Next exact steps
- commit slice 14 without pushing, then continue slice 15
