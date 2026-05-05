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
- active batch-run rule: for this `N=10` batch, commit after every slice but defer push until all 10 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `62d09a8850ad42b3e39e394b974538180de3cbcd`
- latest_commit_brief_message: `test(kg): lock unresolved direction gaps`
- note: the latest committed baseline adds the `compat_direction_hints_incomplete_negative` KG fixture and raises the tracked KG fixture baseline to `128/128`

## Recent commit chain (last 6)
- `62d09a8` test(kg): lock unresolved direction gaps
- `4587325` chore(fsmgen): refresh submodule contract baseline
- `c151bdc` fix(validate): split intent direction gap findings
- `5fb059e` test(adapter): lock fsm graph undriven outputs
- `a2882b4` fix(adapter): block graph-backed undriven outputs
- `706118e` docs: refresh README bootstrap continuity

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 24` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `COMMIT.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `10`
- completed_count: `0`
- push_policy: defer push until all `10` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - document the active batch-run push policy in `COMMIT.md` and `MEMORY.md`
  - keep per-slice commits mandatory while deferring push until the full user-defined `N=10` batch completes
- tracker effect:
  - add a Done row for documented batch-run deferred-push policy
- verification status:
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `128/128` fixtures
  - `git diff --check` passed
- current known local CI baseline:
  - `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `128/128` tracked KG fixtures

## Next exact steps
- verify and commit the batch-policy slice as slice 1 of 10
- continue with slice 2 without pushing
