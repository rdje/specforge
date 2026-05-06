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
- latest_commit_hash: `45d3cbe0882b8236aeab4b4ba26b2f5c881afc9a`
- latest_commit_brief_message: `test(adapter): lock unemitted child-target guidance`
- note: new local `N=200` batch is active; slice 39 is committed, slice 40 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `45d3cbe` test(adapter): lock unemitted child-target guidance
- `a21e282` test(adapter): lock undeclared top-source guidance
- `e557330` test(adapter): lock undeclared top-target guidance
- `a99df69` test(adapter): lock no-top-port guidance
- `45e8df4` test(adapter): lock no-child top guidance
- `ff62d21` test(adapter): lock multi-child top-link guidance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 40]`
- files in flight for new batch slice 40:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `docs/book/src/reference/generated-artifacts.md`

## Active N-slice batch
- requested_count: `200`
- completed_count: `39`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - regression-lock top links that originate from child endpoints absent from emitted child modules
  - prove blocked top candidate and aggregate `.fsm` renderability preserve source-endpoint enrichment guidance
  - preserve declared top-port, child, top-link support IDs, and high-confidence topology state while the child remains a resolved DT root
  - keep live docs and mdBook aligned with this blocked-artifact guidance surface
- tracker effect:
  - expected live-status tracker gain: `.fsm links from unemitted child sources now preserve source-endpoint enrichment guidance: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge top_composition_blocks_link_from_unemitted_child_source_guidance` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `88` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed after live-doc/mdBook sync
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` fixtures
  - `bash scripts/run_ci.sh` passed with `609` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- current known local CI baseline:
  - current slice passed `609` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures

## Next exact steps
- commit slice 40 without pushing, run post-commit checks, and continue slice 41
