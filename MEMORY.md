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
- latest_commit_hash: `ff62d2111ae5ce7f73647b23f557538a137df0c8`
- latest_commit_brief_message: `test(adapter): lock multi-child top-link guidance`
- note: new local `N=200` batch is active; slice 34 is committed, slice 35 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `ff62d21` test(adapter): lock multi-child top-link guidance
- `346e9fc` test(adapter): lock duplicate top width dedup guidance
- `d563389` test(adapter): lock duplicate top dedup guidance
- `5f63f0d` test(adapter): lock duplicate child guidance
- `68f9ef7` test(adapter): lock primary missing child guidance
- `1502073` test(adapter): lock missing child guidance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 35]`
- files in flight for new batch slice 35:
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
- completed_count: `34`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - regression-lock top composition blockers that omit child-module references
  - prove blocked top candidate and aggregate `.fsm` renderability preserve child-module enrichment guidance
  - preserve the declared top port direction, width, support IDs, and high automation confidence while no child is fabricated
  - keep live docs and mdBook aligned with this blocked-artifact guidance surface
- tracker effect:
  - expected live-status tracker gain: `.fsm no-child top blockers now preserve child-module enrichment guidance: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge top_composition_blocks_top_without_child_guidance` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `83` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed after live-doc/mdBook sync
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` tracked fixtures
  - `bash scripts/run_ci.sh` passed with formatting, warning-deny Clippy, `604` Rust tests, warning-deny Rust docs, and mdBook build
- current known local CI baseline:
  - current slice passed `604` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures

## Next exact steps
- run final guards, commit slice 35 without pushing, and continue slice 36
