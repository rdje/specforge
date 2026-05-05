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
- active batch-run rule: for this `N=20` batch, commit after every slice but defer push until all 20 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `a15d8613a10541888d16ed198b1bba1c930bd75e`
- latest_commit_brief_message: `test(validation): lock graph replay command lanes`
- note: the latest committed baseline is the pushed final commit from the previous `N=10` batch; this new `N=20` batch starts from a clean local/remote-aligned tree

## Recent commit chain (last 6)
- `a15d861` test(validation): lock graph replay command lanes
- `146a576` test(kg): lock flat-hint conflict guidance payloads
- `6531382` test(validate): lock graph conflict guidance pair
- `5d739ee` test(kg): lock graph conflict rescan payloads
- `21c4bcf` test(validation): preserve intent graph replay split
- `7beb265` test(validation): preserve graph replay target split

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: aligned with `origin/main`
- files in flight for batch-20 slice 1:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `20`
- completed_count: `0`
- push_policy: defer push until all `20` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - record the new `N=20` batch state in continuity docs before functional work resumes
  - replace stale `N=10` in-flight state with the pushed `a15d861` baseline and new deferred-push count
- tracker effect:
  - no live-status row change expected; this is operational continuity state
- verification status:
  - `git status --short --branch` was clean at `main...origin/main`
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit batch-20 slice 1 without pushing, then move to slice 2
