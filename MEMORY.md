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
- active batch-run rule: for the new `N=40` batch, commit after every slice but defer push until all 40 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `0e108e7cb9093603356ddd78fe44bc9b85c44521`
- latest_commit_brief_message: `test(rescan): lock scoped pending limits`
- note: new local `N=40` batch is active; push remains deferred until all 40 slices complete

## Recent commit chain (last 6)
- `0e108e7` test(rescan): lock scoped pending limits
- `bf7d825` test(rescan): lock no-change promotion gate
- `edde6c1` test(rescan): lock fingerprint delta
- `7febd89` test(rescan): lock missing validation report
- `dfc6b25` test(rescan): lock validation report sidecar
- `16fb481` test(rescan): lock schema version gate

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main`
- files in flight for new batch slice 1:
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `40`
- completed_count: `0`
- push_policy: defer push until all `40` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - start the new `N=40` batch-run continuity record
  - make the deferred-push policy explicit before code/test slices begin
- tracker effect:
  - no live-status row changes; existing batch-run deferred-push policy row remains `Done`
- verification status:
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `559` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130` fixtures and `0` failures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `559` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 1 without pushing, then continue slice 2
