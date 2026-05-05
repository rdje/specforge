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
- latest_commit_hash: `f56f62236f5d6183b1221e3f98414ce966566814`
- latest_commit_brief_message: `fix(kg): scope actor taxonomy priors by family`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `f56f622` fix(kg): scope actor taxonomy priors by family
- `0392d66` fix(kg): scope negative knowledge priors by family
- `5ecf0ff` fix(kg): scope semantic phrase priors by family
- `8d8aa39` fix(kg): scope temporal priors by family
- `be3b937` fix(kg): scope visual-motif priors by family
- `6d38eb1` fix(kg): scope table-shape priors by family

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 17]`
- files in flight for new batch slice 18:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `17`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - block `.fsm` direct-root lowering when unconflicted flat canonical direction evidence disagrees with graph-backed actor-relative direction evidence
  - preserve graph-first recovery for missing flat hints and matching flat/graph directions
  - emit flat-vs-graph disagreement diagnostics rather than generic missing-direction diagnostics
- tracker effect:
  - live-status tracker gains `.fsm adapter renderability now blocks single flat-vs-graph direction disagreement: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge standalone_dt_blocks_flat_graph_direction_disagreement` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `77` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `594` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148` fixtures and `0` failures
- current known local CI baseline:
  - `594` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `148/148` tracked KG fixtures

## Next exact steps
- run docs after live-doc edits, then final guards, commit slice 18 without pushing, and continue slice 19
