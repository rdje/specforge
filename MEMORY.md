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
- latest completed batch-run rule: the `N=200` batch committed each slice independently, synced live docs and mdBook at the end of every slice, deferred push until all 200 new-batch slices were complete, and was pushed after slice 200
- previous batch-run rule used: the `N=100` batch committed each slice independently, deferred push until all 100 slices were complete, and was pushed after slice 100
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `6983c99b34c2b36f424ef7e7c17bbf5ccef07683`
- latest_commit_brief_message: `test(adapter): block duplicate top output`
- note: local `N=200` batch completed all 200 slices and was pushed to `origin/main`

## Recent commit chain (last 6)
- `6983c99` test(adapter): block duplicate top output
- `a5e6fa3` test(adapter): block recovered top output
- `2c71513` test(adapter): block parametric actor output
- `9bf9fb6` test(adapter): block parametric signal output
- `1f01a3e` test(adapter): block selector predicate output
- `90fd192` test(adapter): block reset polarity output

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state after batch push: `main...origin/main`
- files in flight: none

## Latest N-slice batch
- requested_count: `200`
- completed_count: `200`
- push_policy: completed; pushed once after all `200` new-batch slices were committed
- slice_rule: each slice received verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice started
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current batch status
- objective:
  - no active in-flight batch slice remains
  - latest completed slice locked duplicate/conflicting top-port direction blockers against stale top and aggregate renderable output
  - live tracker, roadmap, mdBook, Rust analysis, and continuity docs were synced with the duplicate top-port direction renderable-block guarantee
- tracker effect:
  - live-status tracker marks `.fsm` duplicate top-port direction blockers blocking stale top and aggregate renderable output as `Done`
  - live-status tracker marks the `N=200` batch as completed and pushed
- verification status:
  - implementation and live-doc sync are complete for slice 200
  - focused duplicate top-port direction renderable-block regression passed
  - adapter suite passed
  - formatting passed
  - docs CI passed
  - KG bench passed
  - full CI passed
  - user-requested `cargo sweep --time 1` is deferred until no target-tree process is active because `target/release/tool_matrix` is active
  - final commit guards passed before the slice 200 commit
  - post-commit checks passed after the slice 200 commit
  - completed-batch push to `origin/main` passed
- current known local CI baseline:
  - current slice passed `614` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current slice passed `148/148` tracked KG fixtures
  - current slice validation is complete
  - latest `cargo sweep --time 1` attempt was deferred because `target/release/tool_matrix` is active
  - message file is currently untracked and `0` bytes

## Next exact steps
- keep `cargo sweep --time 1` deferred until the active `target/release/tool_matrix` process exits, then run it when the target tree is idle
