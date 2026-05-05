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
- active batch-run rule: for the new `N=100` batch, commit after every slice but defer push until all 100 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `7e0add50b4f05026cd3c20bd2e42d7a27c8d069c`
- latest_commit_brief_message: `fix(adapter): preserve top flat graph direction evidence`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `7e0add5` fix(adapter): preserve top flat graph direction evidence
- `afb2376` test(adapter): lock system contract flat graph disagreement
- `f3f1fbe` test(adapter): lock structured FSM flat graph disagreement
- `50a6661` test(adapter): lock module flat graph disagreement
- `ce867e3` fix(adapter): block flat graph direction disagreement
- `f56f622` fix(kg): scope actor taxonomy priors by family

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 22]`
- files in flight for new batch slice 23:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`
  - `docs/book/src/domain/actor-connectivity.md`
  - `docs/book/src/commands/pipeline.md`
  - `docs/book/src/reference/generated-artifacts.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `22`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - document `.fsm` adapter flat/graph provenance behavior in the mdBook
  - explain that blocked top-root renderability can coexist with inspectable flat and graph signal-inventory evidence
  - keep user-facing generated-artifact guidance aligned with the slice-22 adapter behavior
- tracker effect:
  - live-status tracker gains `mdBook now documents blocked .fsm adapter renderability together with flat/graph signal-inventory provenance: Done`
- verification status:
  - `bash scripts/run_docs_ci.sh` passed for the mdBook edits and must be rerun after the current live-doc refresh
  - `bash scripts/run_ci.sh` passed with `597` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148` fixtures and `0` failures
- current known local CI baseline:
  - `597` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `148/148` tracked KG fixtures

## Next exact steps
- rerun docs after live-doc edits, run final guards, commit slice 23 without pushing, and continue slice 24
