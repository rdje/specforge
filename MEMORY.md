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
- do not push unless the user asks or the branch reaches `25` local commits since the last push
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `3598999`
- latest_commit_brief_message: `test: harden sibling width and source env isolation`
- note: the latest committed baseline locks mirror sibling child-link source-width coverage and fixes SourceIR/Docling shared environment isolation

## Recent commit chain (last 6)
- `3598999` test: harden sibling width and source env isolation
- `b7274bb` fix(adapter): recover child widths from sibling links
- `b462600` fix(adapter): recover top widths from child links
- `e9a91dd` fix(adapter): recover module input widths from actor graph
- `7b266d2` fix(adapter): recover direct input widths from actor graph
- `abc3405` fix(adapter): recover child widths from top links

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 2` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - recover child module port widths through transitive explicit top-link topology by computing a fixed-point endpoint-width graph before module topology overlays
- tracker effect:
  - add a live-status Done row for explicit top-link numeric endpoint-width propagation through the top-link graph
- current tracked KG-quality suite size in the latest committed baseline:
  - `127` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge top_composition_recovers_child_width_through_transitive_topology` first failed before the fix, then passed
  - `cargo test -p specforge top_composition` passed with `23` topology tests
  - `cargo test -p specforge ir::adapters::tests` passed with `56` adapter tests
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `499` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo fmt --all -- --check` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `499` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- run the final stale-doc scan after these doc status updates
- commit the transitive top-link endpoint-width recovery slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 3`, below the `25`-commit push threshold
