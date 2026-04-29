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
- latest_commit_hash: `e9a91dd`
- latest_commit_brief_message: `fix(adapter): recover module input widths from actor graph`
- note: the latest committed baseline records explicit-module control-input width recovery from actor-port numeric shape evidence

## Recent commit chain (last 6)
- `e9a91dd` fix(adapter): recover module input widths from actor graph
- `7b266d2` fix(adapter): recover direct input widths from actor graph
- `abc3405` fix(adapter): recover child widths from top links
- `bba3a59` fix(adapter): recover top port widths from actor graph
- `096438d` fix(adapter): recover top ports from actor graph
- `307f967` fix(doctor): tolerate cold local model load

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 24` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - recover top boundary port numeric widths from connected child module endpoints in explicit `.fsm` top-link topology
- tracker effect:
  - add a Done row for bidirectional top-link width recovery; broader `Actor-relative direction model` remains In Progress
- current tracked KG-quality suite size in the latest committed baseline:
  - `127` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge top_composition_recovers_top_port_width_from_child_link_topology` first failed before the fix, then passed
  - `cargo test -p specforge top_composition_blocks_conflicting_top_port_widths_from_child_links` passed
  - `cargo test -p specforge ir::adapters::tests` passed with `52` adapter tests
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `495` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- commit the top-link top-boundary width recovery slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- push after this commit because the branch will reach the `25`-commit threshold
