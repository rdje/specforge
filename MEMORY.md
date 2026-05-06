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
- latest_commit_hash: `7418e58411dc306392c3eeb953eb02426456c28a`
- latest_commit_brief_message: `test(adapter): lock system top-width residual cleanliness`
- note: new local `N=200` batch is active; slice 86 is committed, slice 87 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `7418e58` test(adapter): lock system top-width residual cleanliness
- `b2c34c7` test(adapter): lock child-link top-width residual cleanliness
- `cb20d22` test(adapter): lock actor width residual cleanliness
- `4933d9b` test(adapter): lock actor direction residual cleanliness
- `ac97421` test(adapter): lock top-link direction residual cleanliness
- `c2b50c3` test(adapter): lock mixed-child residual cleanliness

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 87]`
- files in flight for new batch slice 87:
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
- completed_count: `86`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock residual-clean behavior for renderable child actor-port direction recovery
  - prove `top_composition_recovers_child_directions_from_actor_ports` leaves no stale `fsm_adapter_composition_topology` residual once graph-backed child direction recovery renders
  - sync the live tracker, roadmap, and mdBook with the child actor-port direction recovery residual-clean guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` child actor-port direction recovery staying composition-residual clean when renderable as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused child actor-port direction recovery residual-clean regression passed (`1` test)
  - adapter suite passed (`92` tests)
  - formatting, docs CI, KG bench (`148/148` fixtures), full CI (`613` Rust tests plus warning-deny Clippy/rustdoc and mdBook), and `cargo sweep --time 1` all passed
- current known local CI baseline:
  - current slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - user-requested `cargo sweep --time 1` cleaned nothing after the current full CI gate
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards, commit slice 87 without pushing, then start slice 88 of the active `N=200` batch
