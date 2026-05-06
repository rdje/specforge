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
- latest_commit_hash: `8ee672a89a4a5e20835c4f1753d4830ba78976bd`
- latest_commit_brief_message: `test(adapter): lock top target role diagnostics`
- note: new local `N=200` batch is active; slice 126 is committed, slice 127 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `8ee672a` test(adapter): lock top target role diagnostics
- `b4019ce` test(adapter): lock top-link direction diagnostics
- `a211248` test(adapter): lock top actor width diagnostics
- `b857f45` test(adapter): lock top actor direction diagnostics
- `915ff1b` test(adapter): lock recovered direction diagnostics
- `e411bdb` test(adapter): lock unemitted target diagnostics

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 127]`
- files in flight for new batch slice 127:
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
- completed_count: `126`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock composition-topology residual diagnostics for source-side child endpoint direction-role blockers
  - prove `top_composition_blocks_child_source_direction_role_guidance` keeps low-confidence `fsm_adapter_composition_topology` diagnostics and topology-detail blocker text while preserving child-module and top-level endpoint-role guidance
  - sync the live tracker, roadmap, and mdBook with the source-side child role residual-diagnostics guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` source-side child direction-role residuals locking low-confidence topology-detail diagnostics as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused source-side child role residual-diagnostics regression passed (`1` test)
  - adapter suite (`92` tests), formatting, docs CI, KG bench (`148/148` fixtures), and full CI (`613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation) passed
  - final commit guards pending
  - user-requested `cargo sweep --time 1` remains deferred until no target-tree process is active because another shell still has `target/release/tool_matrix` running from this repository
- current known local CI baseline:
  - last committed slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - last committed slice passed `148/148` tracked KG fixtures
  - latest `cargo sweep --time 1` attempt was deferred because another shell still had `target/release/tool_matrix` running from this repository
  - message file is currently untracked and `0` bytes

## Next exact steps
- finish slice 127 docs sync
- run focused regression, adapter suite, fmt, docs CI, KG bench, full CI, and sweep safety gate
- run final commit guards and commit slice 127 without pushing
