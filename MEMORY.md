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
- latest_commit_hash: `65fcf4b`
- latest_commit_brief_message: `fix(validate): split semantic compat direction lag`
- note: the latest committed baseline split semantic compatibility-direction reporting into graph-backed lag vs genuinely unresolved direction coverage

## Recent commit chain (last 6)
- `65fcf4b` fix(validate): split semantic compat direction lag
- `ce6d875` Docs: refresh bootstrap corpus projections
- `d4f53bb` fix(adapter): recover transitive topology widths
- `3598999` test: harden sibling width and source env isolation
- `b7274bb` fix(adapter): recover child widths from sibling links
- `b462600` fix(adapter): recover top widths from child links

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 5` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - preserve recovered `.fsm` top-boundary directions as graph-backed selected signal-inventory hints instead of flattening top-link/top-actor recovery into flat compatibility `direction_hint`
- tracker effect:
  - added a `Done` live-status row for selected top signal inventory preserving recovered top-boundary directions as graph-backed hints
- verification status:
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_link_topology -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_actor_ports -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml top_composition_preserves_recovered_top_port_direction_when_still_blocked -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` passed with `23` top-composition tests
  - `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` passed with `56` adapter tests
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `500` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `500` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- run final `git diff --check` plus status review
- commit the selected top signal-inventory graph-backed direction provenance slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 6`, below the `25`-commit push threshold
