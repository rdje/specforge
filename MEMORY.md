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
- latest_commit_hash: `af3db8c5812ec5d3850622338f339a68008a78ad`
- latest_commit_brief_message: `test(adapter): lock fsm graph size entries`
- note: new local `N=200` batch is active; slice 147 is committed, slice 148 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `af3db8c` test(adapter): lock fsm graph size entries
- `c20bb6a` test(adapter): lock graph size-entry directions
- `c8a139c` test(nlp): reject source-label aliases
- `1de996e` test(adapter): lock child source guidance diagnostics
- `3685077` test(adapter): lock child actor direction diagnostics
- `8d4b0f4` test(adapter): lock child topology direction diagnostics

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 148]`
- files in flight for new batch slice 148:
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
- completed_count: `147`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock explicit-module renderable `.fsm` size entries as consumers of graph-recovered module-local input directions when flat module-local `direction_hint` values are absent
  - prove `DATA_IN`, `GO`, and `DONE` lower with expected input roles and widths from explicit-module control-read recovery
  - sync the live tracker, roadmap, mdBook, Rust analysis, and continuity docs with the explicit-module size-entry guarantee
- tracker effect:
  - live-status tracker changed; new row marks explicit-module `.fsm` renderable size entries consuming graph-backed module-control directions as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused explicit-module graph-backed size-entry regression passed (`1` test)
  - adapter suite passed (`92` tests)
  - formatting passed
  - docs CI passed
  - KG bench passed (`148/148` fixtures)
  - full CI passed (`614` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation)
  - user-requested `cargo sweep --time 1` is deferred until no target-tree process is active because `target/release/tool_matrix` is active
  - final commit guards pending
- current known local CI baseline:
  - current in-flight slice passed `614` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current in-flight slice passed `148/148` tracked KG fixtures
  - current slice has passed focused explicit-module graph-backed size-entry regression, adapter suite, and formatting
  - latest `cargo sweep --time 1` attempt was deferred because `target/release/tool_matrix` is active
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards and commit slice 148 without pushing
