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
- latest_commit_hash: `a23381a2177cc88b94c7bc0a41a1be058785ed15`
- latest_commit_brief_message: `test(adapter): lock structured flat graph guidance`
- note: new local `N=200` batch is active; slice 107 is committed, slice 108 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `a23381a` test(adapter): lock structured flat graph guidance
- `82602cf` test(adapter): lock undeclared-target residual guidance
- `e1efddc` test(adapter): lock missing-initial residual guidance
- `7f7ca69` test(adapter): lock structured undriven residual guidance
- `8d003c2` test(adapter): lock undriven output residual guidance
- `0a4f8bf` test(adapter): lock flat graph residual guidance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 108]`
- files in flight for new batch slice 108:
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
- completed_count: `107`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock signal-inventory residual guidance for explicit-module flat/graph direction disagreement blockers
  - prove `standalone_explicit_module_blocks_flat_graph_direction_disagreement` keeps the `fsm_adapter_signal_inventory` residual packet and upstream enrichment candidate while preserving module interface and actor-port provenance
  - sync the live tracker, roadmap, and mdBook with the explicit-module flat/graph residual-guidance guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` explicit-module flat/graph direction disagreements retaining signal-inventory repair guidance when blocked as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused explicit-module flat/graph residual-guidance regression passed (`1` test)
  - adapter suite passed (`92` tests)
  - formatting, docs CI, KG bench (`148/148` fixtures), and full CI (`613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation) passed
  - user-requested `cargo sweep --time 1` was deferred because another shell still had `target/release/tool_matrix` running from this repository
- current known local CI baseline:
  - last committed slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - last committed slice passed `148/148` tracked KG fixtures
  - latest `cargo sweep --time 1` attempt was deferred because another shell still had `target/release/tool_matrix` running from this repository
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards and commit slice 108 without pushing
