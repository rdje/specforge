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
- latest batch-run rule used: the `N=100` batch committed each slice independently, deferred push until all 100 slices were complete, and was pushed after slice 100
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `b07a28000de1c847dc5495bfd73075a96e08ff23`
- latest_commit_brief_message: `test(adapter): lock renderable top port shape`
- note: local `N=100` batch completed and was pushed to `origin/main`; current task is a documentation sync slice

## Recent commit chain (last 6)
- `b07a280` test(adapter): lock renderable top port shape
- `f30991f` test(adapter): lock missing child top port
- `9c70a20` test(adapter): lock undeclared target FSM states
- `1d6dc59` test(adapter): lock missing initial FSM graph
- `6a18f9d` test(adapter): lock reset FSM renderable graph
- `8d514b3` test(adapter): lock structured FSM candidates

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before this docs-sync commit: `main...origin/main`
- files in flight for documentation sync:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`
  - `docs/book/src/commands/pipeline.md`
  - `docs/book/src/reference/generated-artifacts.md`

## Last N-slice batch
- requested_count: `100`
- completed_count: `100`
- push_status: pushed to `origin/main`
- final_commit_hash: `b07a28000de1c847dc5495bfd73075a96e08ff23`
- slice_rule_observed: each slice received verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice started

## Current in-flight slice
- objective:
  - synchronize live docs and mdBook after the completed `N=100` batch
  - correct the stale slice-100 in-flight continuity state
  - document current `.fsm` adapter provenance behavior in the public mdBook
- tracker effect:
  - no `LIVE_ACHIEVEMENT_STATUS.md` status row change expected; tracker already includes the latest renderable top-composition and blocker provenance rows
- verification status:
  - pending `bash scripts/run_docs_ci.sh`
  - pending `git diff --check`
  - pending markdown absolute-path guard
- current known local CI baseline:
  - last code slice passed `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - last code slice passed `148/148` tracked KG fixtures

## Next exact steps
- run docs CI and guards, commit the docs sync, truncate `git_message_brief.txt`, and report the unchanged live-status snapshot
