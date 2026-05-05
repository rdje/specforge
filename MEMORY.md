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
- latest_commit_hash: `50a666153ee750ffc76168098985cac1edf915d1`
- latest_commit_brief_message: `test(adapter): lock module flat graph disagreement`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `50a6661` test(adapter): lock module flat graph disagreement
- `ce867e3` fix(adapter): block flat graph direction disagreement
- `f56f622` fix(kg): scope actor taxonomy priors by family
- `0392d66` fix(kg): scope negative knowledge priors by family
- `5ecf0ff` fix(kg): scope semantic phrase priors by family
- `8d8aa39` fix(kg): scope temporal priors by family

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 19]`
- files in flight for new batch slice 20:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `19`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - add structured-FSM coverage for flat-vs-graph direction disagreement
  - prove a single flat `ACC` input declaration cannot render through graph-backed controller output evidence
  - keep the shared slice-18 renderability guard locked through FSM-state signal inventories
- tracker effect:
  - live-status tracker gains `.fsm structured-FSM renderability now regression-locks single flat-vs-graph direction disagreement: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge structured_fsm_blocks_flat_graph_direction_disagreement` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `79` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed before live-doc edits and must be rerun after the current live-doc refresh
  - `bash scripts/run_ci.sh` passed with `596` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148` fixtures and `0` failures
- current known local CI baseline:
  - `596` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `148/148` tracked KG fixtures

## Next exact steps
- rerun docs after live-doc edits, run final guards, commit slice 20 without pushing, and continue slice 21
