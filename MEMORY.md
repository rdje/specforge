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
- do not push unless the user asks or the branch reaches `25` local commits since the last push
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `ce6d875`
- latest_commit_brief_message: `Docs: refresh bootstrap corpus projections`
- note: the latest committed baseline refreshed README/bootstrap continuity and corpus-KB fixture projections to the executable `127/127` KG fixture baseline

## Recent commit chain (last 6)
- `ce6d875` Docs: refresh bootstrap corpus projections
- `d4f53bb` fix(adapter): recover transitive topology widths
- `3598999` test: harden sibling width and source env isolation
- `b7274bb` fix(adapter): recover child widths from sibling links
- `b462600` fix(adapter): recover top widths from child links
- `e9a91dd` fix(adapter): recover module input widths from actor graph

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 4` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/kg_bench.rs`
  - `crates/specforge/src/commands/validate.rs`
  - `crates/specforge/test_data/kg_quality/compat_direction_hints_lag_graph_negative/fixture.json`

## Current in-flight slice
- objective:
  - split semantic validation compatibility-direction reporting so graph-resolved missing flat hints are `semantic_compat_direction_hints_lag_graph`, while signals with neither flat hint nor non-conflicted graph coverage remain `semantic_compat_direction_hints_incomplete`
- tracker effect:
  - added a `Done` live-status row for semantic validation distinguishing graph-backed compatibility lag from unresolved direction coverage
- verification status:
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo test --manifest-path Cargo.toml validate_semantic_ir_reports_graph_backed_compat_direction_lag_related_ids -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml validate_semantic_ir_keeps_incomplete_direction_finding_without_graph_coverage -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml validate_intent_ir_scores_direction_from_graph_before_compat_hints -- --nocapture` passed
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality compat_direction_hints_lag_graph_negative` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `500` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `500` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- run final `git diff --check` plus status review
- commit the semantic compatibility-direction validation split
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 5`, below the `25`-commit push threshold
