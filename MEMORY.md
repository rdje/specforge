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
- latest_commit_hash: `b7274bb`
- latest_commit_brief_message: `fix(adapter): recover child widths from sibling links`
- note: the latest committed baseline records child module port width recovery from connected sibling child endpoints

## Recent commit chain (last 6)
- `b7274bb` fix(adapter): recover child widths from sibling links
- `b462600` fix(adapter): recover top widths from child links
- `e9a91dd` fix(adapter): recover module input widths from actor graph
- `7b266d2` fix(adapter): recover direct input widths from actor graph
- `abc3405` fix(adapter): recover child widths from top links
- `bba3a59` fix(adapter): recover top port widths from actor graph

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 1` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`
  - `crates/specforge/src/ir/source.rs`

## Current in-flight slice
- objective:
  - lock the source-child mirror of sibling child-link width recovery with focused adapter coverage and fix the SourceIR/Docling shared environment-lock gap exposed by full CI
- tracker effect:
  - no live-status row changes; the previous sibling child-link width recovery Done row remains the current tracker state
- current tracked KG-quality suite size in the latest committed baseline:
  - `127` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge top_composition_recovers_source_child_width_from_sibling_child_link_topology` passed on first run
  - `cargo test -p specforge ir::adapters::tests` passed with `55` adapter tests
  - `bash scripts/run_ci.sh` first failed in `pdf_source_ir_materialization_uses_backend_helper_and_writes_manifests` because parallel PATH mutation hid `dirname`, `mkdir`, and `cat` from the stub shell helper
  - `cargo test -p specforge pdf_source_ir_materialization_uses_backend_helper_and_writes_manifests` passed after switching SourceIR PDF tests to the shared `env_var_lock()`
  - `cargo test -p specforge inspect_docling_runtime_prefers` passed after the same shared-lock fix
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed on rerun
  - `git diff --check` passed
- current known local CI baseline:
  - `498` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- commit the sibling child-link source-width coverage and SourceIR test-isolation slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed unless it reaches the `25`-commit threshold
