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
- latest_commit_hash: `e1f9b42f172ed37f2572414baf178a1bbbbbcd2c`
- latest_commit_brief_message: `test(adapter): lock child-link width residual`
- note: new local `N=200` batch is active; slice 64 is committed, slice 65 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `e1f9b42` test(adapter): lock child-link width residual
- `4cd2764` test(adapter): align top-link direction residual
- `1e88286` test(adapter): lock actor width residual
- `779fae2` test(adapter): lock actor direction residual
- `10fd8f7` test(adapter): lock duplicate width residual
- `7aa7d0e` test(adapter): lock duplicate direction residual

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 65]`
- files in flight for new batch slice 65:
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
- completed_count: `64`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock structured composition residual diagnostics for sibling child-link width conflicts
  - prove `top_composition_blocks_conflicting_sibling_child_link_widths` keeps renderable-child repair visible in `fsm_adapter_composition_topology`
  - preserve existing child-module canonical width-conflict guidance, conflicting topology-link support IDs, and high automation confidence
  - sync the live tracker, roadmap, and mdBook with the sibling child-link width residual guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` sibling child-link width conflicts preserving composition topology residual decisions as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused regression, adapter suite, fmt, docs, KG bench, full CI, and `cargo sweep --time 1` passed
  - final docs build, whitespace guard, and markdown absolute-path guard passed; slice 65 is ready to commit
- current known local CI baseline:
  - current slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - user-requested `cargo sweep --time 1` cleaned nothing after the current full CI gate
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards, commit slice 65 without pushing, then start slice 66
