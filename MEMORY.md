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
- latest_commit_hash: `edb9b433ae9e55147aeaa5e59c6b0d373a89fde3`
- latest_commit_brief_message: `test(adapter): lock direct actor residual cleanliness`
- note: new local `N=200` batch is active; slice 93 is committed, slice 94 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `edb9b43` test(adapter): lock direct actor residual cleanliness
- `6ad1cab` test(adapter): lock transitive width residual cleanliness
- `6e770d9` test(adapter): lock sibling output residual cleanliness
- `33c8f48` test(adapter): lock sibling input residual cleanliness
- `48dc612` test(adapter): lock top-link child-width residual cleanliness
- `202d0f2` test(adapter): lock child link residual cleanliness

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 94]`
- files in flight for new batch slice 94:
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
- completed_count: `93`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock signal-inventory residual-clean behavior for renderable output actor selection
  - prove `standalone_dt_selects_output_actor_when_external_actors_share_signals` leaves no stale `fsm_adapter_signal_inventory` residual once target-actor-backed direct outputs render
  - sync the live tracker, roadmap, and mdBook with the output actor selection residual-clean guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` standalone output actor selection staying signal-inventory-residual clean when renderable as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused output actor selection residual-clean regression passed (`1` test)
  - adapter suite passed (`92` tests)
  - formatting, docs CI, KG bench (`148/148` fixtures), and full CI (`613` Rust tests plus warning-deny Clippy/rustdoc and mdBook) passed
  - user-requested `cargo sweep --time 1` is deferred until no target-tree process is active
- current known local CI baseline:
  - current slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - current `cargo sweep --time 1` was deferred because another shell still had `target/release/tool_matrix` running from this repository
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards, commit slice 94 without pushing, retry `cargo sweep --time 1` when the target tree is idle, then start slice 95 of the active `N=200` batch
