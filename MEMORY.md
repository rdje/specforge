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
- latest_commit_hash: `1f01a3eaca4453af146a88a2aec45cfddd979f26`
- latest_commit_brief_message: `test(adapter): block selector predicate output`
- note: new local `N=200` batch is active; slice 196 is committed, slice 197 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `1f01a3e` test(adapter): block selector predicate output
- `90fd192` test(adapter): block reset polarity output
- `637a76c` test(adapter): block missing system output
- `f298b92` test(adapter): block system width output
- `e8e4528` test(adapter): block system direction output
- `b360f63` test(adapter): block system flat graph output

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 197]`
- files in flight for new batch slice 197:
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
- completed_count: `196`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - lock canonical parametric signal-width blockers against stale module and aggregate renderable output
  - prove symbolic interface width evidence leaves no renderable module and no aggregate renderable source document
  - sync the live tracker, roadmap, mdBook, Rust analysis, and continuity docs with the canonical parametric-width renderable-block guarantee
- tracker effect:
  - live-status tracker changed; new row marks `.fsm` canonical parametric-width blockers blocking stale module and aggregate renderable output as `Done`
- verification status:
  - implementation and live-doc sync are complete
  - focused canonical parametric-width renderable-block regression passed (`1` test)
  - adapter suite passed (`92` tests)
  - formatting passed
  - docs CI passed
  - KG bench passed (`148/148` fixtures)
  - full CI passed (`614` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation)
  - user-requested `cargo sweep --time 1` is deferred until no target-tree process is active because `target/release/tool_matrix` is active
  - final commit guards pending
- current known local CI baseline:
  - previous committed slice passed `614` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - previous committed slice passed `148/148` tracked KG fixtures
  - current slice has passed focused canonical parametric-width renderable-block regression, adapter suite, formatting, docs CI, KG bench, and full CI
  - latest `cargo sweep --time 1` attempt was deferred because `target/release/tool_matrix` is active
  - message file is currently untracked and `0` bytes

## Next exact steps
- run final docs CI, final commit guards, then commit slice 197 without pushing
