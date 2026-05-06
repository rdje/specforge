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
- latest_commit_hash: `4fab6ef46af08f56d879b64f56e70b5cb7211bec`
- latest_commit_brief_message: `test(adapter): lock no-child top diagnostics`
- note: new local `N=200` batch is active; slice 133 is committed, slice 134 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `4fab6ef` test(adapter): lock no-child top diagnostics
- `0b2d5a4` test(adapter): lock multi-child no-link diagnostics
- `1abe186` test(adapter): lock duplicate top width diagnostics
- `370789f` test(adapter): lock duplicate top direction diagnostics
- `52796d0` test(adapter): lock missing child diagnostics
- `0a1ba30` test(adapter): lock child target role diagnostics

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 134]`
- files in flight for new batch slice 134:
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
- completed_count: `133`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock composition-topology residual diagnostics for top roots without explicit top-port records
  - prove `top_composition_blocks_top_without_port_guidance` keeps low-confidence `fsm_adapter_composition_topology` diagnostics and topology-detail blocker text while preserving top-port enrichment guidance, child declaration provenance, resolved child root kind, and explicit-top-port repair context
  - sync the live tracker, roadmap, and mdBook with the no-top-port residual-diagnostics guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` no-top-port residuals locking low-confidence topology-detail diagnostics as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused no-top-port residual-diagnostics regression passed (`1` test)
  - adapter suite (`92` tests), formatting, docs CI, KG bench (`148/148` fixtures), and full CI (`613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation) passed
  - user-requested `cargo sweep --time 1` is deferred until no target-tree process is active because `target/release/tool_matrix` is active
  - final commit guards pending
- current known local CI baseline:
  - current in-flight slice passed `613` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - current in-flight slice passed `148/148` tracked KG fixtures
  - latest `cargo sweep --time 1` attempt was deferred because `target/release/tool_matrix` is active
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final commit guards and commit slice 134 without pushing
