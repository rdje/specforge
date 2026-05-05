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
- latest_commit_hash: `0392d667912984fe969541770b81bb6ae57732c1`
- latest_commit_brief_message: `fix(kg): scope negative knowledge priors by family`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `0392d66` fix(kg): scope negative knowledge priors by family
- `5ecf0ff` fix(kg): scope semantic phrase priors by family
- `8d8aa39` fix(kg): scope temporal priors by family
- `be3b937` fix(kg): scope visual-motif priors by family
- `6d38eb1` fix(kg): scope table-shape priors by family
- `87a69b7` test(kg): lock AMBA-generic modality prior fallback

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 16]`
- files in flight for new batch slice 17:
  - `crates/specforge/src/ir/prior_memory.rs`
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_protocol_family_mismatch_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_protocol_family_mismatch_negative/axi_actor_taxonomy_mismatch.md`
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
- completed_count: `16`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add a KG negative fixture proving APB actor-taxonomy priors cannot recover AXI-local section-heading directions
  - reuse exact-family plus AMBA-generic lookup for actor-taxonomy role matching
  - update AMBA-generic actor-taxonomy unit-test sources so intentional fallback is explicit
  - refresh corpus-KB projections for the expanded KG fixture and prior-candidate surfaces
- tracker effect:
  - live-status tracker gains `KG benchmark harness now locks actor-taxonomy prior protocol-family scoping: Done`
- verification status:
  - pre-fix `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench actor_taxonomy_prior_protocol_family_mismatch_negative actor_taxonomy_prior_guided_section_direction_gold actor_taxonomy_prior_guided_section_direction_without_prior_negative` reproduced APB actor-taxonomy prior leakage into an AXI fixture
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench actor_taxonomy_prior_protocol_family_mismatch_negative actor_taxonomy_prior_guided_section_direction_gold actor_taxonomy_prior_guided_section_direction_without_prior_negative` passed with `3` fixtures and `0` failures
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` passed with `148` fixtures and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - first `bash scripts/run_ci.sh` caught stale unit-test setup for unknown-family actor-taxonomy fallback
  - `cargo test --manifest-path Cargo.toml -p specforge actor_taxonomy_priors_guide` passed with `2` tests
  - final `bash scripts/run_ci.sh` passed with `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148` fixtures and `0` failures
- current known local CI baseline:
  - `593` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `148/148` tracked KG fixtures

## Next exact steps
- run docs after live-doc edits, then final guards, commit slice 17 without pushing, and continue slice 18
