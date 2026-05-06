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
- latest_commit_hash: `af8584dd9483e71e61547ae3bb6d6d94f7cd1534`
- latest_commit_brief_message: `test(adapter): lock parametric top residual`
- note: new local `N=200` batch is active; slice 70 is committed, slice 71 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `af8584d` test(adapter): lock parametric top residual
- `4890d6b` test(adapter): lock widthless top residual
- `dbd8d2e` test(adapter): lock actor direction residual
- `7a6a7d4` test(adapter): lock child topology direction residual
- `dd44c58` test(adapter): lock child topology width residual
- `1a03b5b` test(adapter): lock sibling width residual

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 71]`
- files in flight for new batch slice 71:
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
- completed_count: `70`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - align unemitted child endpoint guidance across the legacy source-side regression, roadmap, and mdBook
  - rename `top_composition_blocks_link_to_unemitted_child_port` to the source-side blocker it actually covers
  - assert explicit top-root selection in that legacy regression
  - sync the live tracker, roadmap, and mdBook with source- and target-endpoint repair guidance wording
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` unemitted child endpoint guidance naming source and target repair lanes consistently as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused source-side unemitted child endpoint regression passed (`1` test)
  - adapter suite passed (`92` tests)
  - formatting, docs CI, KG bench (`148/148` fixtures), full CI (`613` Rust tests plus warning-deny Clippy/rustdoc and mdBook), and `cargo sweep --time 1` all passed
- current known local CI baseline:
  - current slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - user-requested `cargo sweep --time 1` cleaned nothing after the current full CI gate
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards, commit slice 71 without pushing, then start slice 72 of the active `N=200` batch
