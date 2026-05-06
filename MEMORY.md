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
- active batch-run rule: for the new `N=200` batch, commit after every slice, sync live docs and mdBook at the end of every slice, and defer push until all 200 new-batch slices are complete
- previous batch-run rule used: the `N=100` batch committed each slice independently, deferred push until all 100 slices were complete, and was pushed after slice 100
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `7e87a612bbb13db3287151b6e632b5b1faefe810`
- latest_commit_brief_message: `test(adapter): align top target residuals`
- note: new local `N=200` batch is active; slice 53 is committed, slice 54 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `7e87a61` test(adapter): align top target residuals
- `eb66b18` test(adapter): lock child source residual
- `dc64387` test(adapter): lock child target residual
- `28d4899` test(adapter): lock child target link confidence
- `4328bc0` test(adapter): align child role residuals
- `c251a84` test(adapter): lock top-link target residual

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 54]`
- files in flight for new batch slice 54:
  - `crates/specforge/src/ir/adapters.rs`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `docs/book/src/reference/generated-artifacts.md`

## Active N-slice batch
- requested_count: `200`
- completed_count: `53`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - add source-side composition residual assertions to the dedicated top-source tests
  - prove undeclared top-source blockers point at explicit-top-port repair
  - prove unemitted child-source blockers point at renderable-child repair in their named regression
  - sync the live tracker, roadmap, and mdBook with the undeclared top-source residual guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` links from undeclared top sources preserving composition topology residual decisions as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - `cargo test --manifest-path Cargo.toml -p specforge top_composition_blocks_link_from_undeclared_top_source_guidance` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge top_composition_blocks_link_from_unemitted_child_source_guidance` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `92` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` fixtures
  - `bash scripts/run_ci.sh` passed with `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - user-requested `cargo sweep --time 1` completed after full CI and cleaned nothing
- current known local CI baseline:
  - current slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards, then commit slice 54 without pushing
