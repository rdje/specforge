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
- latest_commit_hash: `10fd8f74ca601ef46ecd158d005a5a42897804c1`
- latest_commit_brief_message: `test(adapter): lock duplicate width residual`
- note: new local `N=200` batch is active; slice 60 is committed, slice 61 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `10fd8f7` test(adapter): lock duplicate width residual
- `7aa7d0e` test(adapter): lock duplicate direction residual
- `99f2de3` test(adapter): lock duplicate child residual
- `a7f9a95` test(adapter): lock no-top-port residual
- `ba50656` test(adapter): lock no-child residual
- `d8d4d2d` test(adapter): lock multi-child residual

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 61]`
- files in flight for new batch slice 61:
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
- completed_count: `60`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock structured composition residual diagnostics for top actor-port direction conflicts
  - prove `top_composition_blocks_conflicting_top_actor_port_direction` carries `fsm_adapter_composition_topology`
  - keep explicit top-port repair visible while existing top-boundary direction-conflict guidance and graph-backed provenance remain intact
  - sync the live tracker, roadmap, and mdBook with the top actor-port direction residual guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` top actor-port direction conflicts preserving composition topology residual decisions as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused regression, adapter suite, fmt, docs, KG bench, full CI, and `cargo sweep --time 1` have passed
- current known local CI baseline:
  - current slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - user-requested `cargo sweep --time 1` cleaned nothing after the full CI gate
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards, commit slice 61 without pushing, clear `git_message_brief.txt`, record post-commit checks, then continue slice 62
