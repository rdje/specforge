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
- latest_commit_hash: `8783350d20cdcc6efac928f032cb4a721a4752fd`
- latest_commit_brief_message: `fix(adapter): preserve top width provenance`
- note: the latest committed baseline keeps recovered selected top width provenance categories visible for actor-port and top-link topology evidence

## Recent commit chain (last 6)
- `8783350` fix(adapter): preserve top width provenance
- `e92fe35` test(adapter): lock top system contract distribution
- `4846128` fix(adapter): materialize system contract signals
- `7bab72c` fix(adapter): preserve actor parametric widths
- `d22ae1f` fix(adapter): preserve parametric width evidence
- `c52f07a` fix(adapter): preserve flat direction conflicts

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 17` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - preserve selected top signal-inventory direction provenance categories when top-boundary directions are recovered from actor-port graph or top-link topology evidence
- tracker effect:
  - added a `Done` live-status row for recovered selected top direction provenance categories from actor-port and top-link topology evidence
- verification status:
  - `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_actor_ports -- --nocapture` failed before the fix, then passed
  - `cargo test --manifest-path Cargo.toml top_composition_preserves_recovered_top_port_direction_when_still_blocked -- --nocapture` failed before the fix, then passed
  - `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_link_topology -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` passed with `27` top-composition tests
  - `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` passed with `66` adapter tests
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `git diff --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `510` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `510` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- rerun final lightweight formatting, docs, and whitespace checks after validation-doc updates
- commit the selected top direction provenance slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 18`, below the `25`-commit push threshold
