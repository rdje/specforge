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
- latest_commit_hash: `4846128fe7d758fb644727d384471fb4ac40b477`
- latest_commit_brief_message: `fix(adapter): materialize system contract signals`
- note: the latest committed baseline lets canonical `.fsm` system contracts materialize missing clock/reset signal-inventory entries as input, 1-bit `system_contract_signal` evidence

## Recent commit chain (last 6)
- `4846128` fix(adapter): materialize system contract signals
- `7bab72c` fix(adapter): preserve actor parametric widths
- `d22ae1f` fix(adapter): preserve parametric width evidence
- `c52f07a` fix(adapter): preserve flat direction conflicts
- `a9a322e` fix(adapter): require emitted child top-link ports
- `b9f9c27` fix(adapter): gate top public IO widths

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 15` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - regression-lock explicit top-root clock/reset distribution into child system-contract endpoints and top-width recovery from those endpoints
- tracker effect:
  - added a `Done` live-status row for explicit top-root clock/reset top-link distribution into child system-contract endpoints
- verification status:
  - `cargo test --manifest-path Cargo.toml top_composition_recovers_top_system_port_widths_from_child_system_contract -- --nocapture` passed
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
- commit the top system-contract distribution regression slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 16`, below the `25`-commit push threshold
