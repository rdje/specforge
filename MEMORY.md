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
- active batch-run rule: for the new `N=20` batch, commit after every slice but defer push until all 20 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `1175e509916f8d1f0e98e4366c944d88e613035e`
- latest_commit_brief_message: `test(rescan): lock unknown command intents`
- note: this is the pushed baseline before starting the next local `N=20` batch

## Recent commit chain (last 6)
- `1175e50` test(rescan): lock unknown command intents
- `7b4725d` test(rescan): lock intent lane mismatches
- `4ee80c8` test(rescan): lock unknown provider values
- `be4f608` test(rescan): lock flag-shaped provider values
- `c5c52d1` fix(rescan): reject flag-shaped model values
- `ceac97b` test(rescan): lock explicit provider hints

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main`
- files in flight for new batch slice 1:
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `20`
- completed_count: `0`
- push_policy: defer push until all `20` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - record the new `N=20` batch state after the previous batch was pushed
  - correct the latest committed baseline and push-deferral state before implementation work continues
- tracker effect:
  - no live-status row change expected; this is continuity state only
- verification status:
  - no Rust behavior changed
  - `bash scripts/run_docs_ci.sh` pending
  - `git diff --check` pending
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `540` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 1 batch-state update without pushing, then pick the next focused implementation/test slice
