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
- latest_commit_hash: `2de0b3a2f7c22cab1ea1973c7209aa23de09745c`
- latest_commit_brief_message: `test(adapter): align unemitted endpoint guidance`
- note: new local `N=200` batch is active; slice 71 is committed, slice 72 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `2de0b3a` test(adapter): align unemitted endpoint guidance
- `af8584d` test(adapter): lock parametric top residual
- `4890d6b` test(adapter): lock widthless top residual
- `dbd8d2e` test(adapter): lock actor direction residual
- `7a6a7d4` test(adapter): lock child topology direction residual
- `dd44c58` test(adapter): lock child topology width residual

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 72]`
- files in flight for new batch slice 72:
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
- completed_count: `71`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock structured composition residual diagnostics for missing-child top composition blockers
  - prove `keeps_top_composition_blocked_when_child_module_is_missing` keeps child-module reference repair visible in `fsm_adapter_composition_topology`
  - preserve existing child-source declaration guidance plus top-port and unresolved child support IDs
  - sync the live tracker, roadmap, and mdBook with the missing-child residual guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` missing-child top blockers preserving composition topology residual decisions as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused missing-child residual regression passed (`1` test)
  - adapter suite passed (`92` tests)
  - formatting, docs CI, KG bench (`148/148` fixtures), full CI (`613` Rust tests plus warning-deny Clippy/rustdoc and mdBook), and `cargo sweep --time 1` all passed
- current known local CI baseline:
  - current slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - user-requested `cargo sweep --time 1` cleaned nothing after the current full CI gate
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards, commit slice 72 without pushing, then start slice 73 of the active `N=200` batch
