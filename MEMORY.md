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
- latest_commit_hash: `b9f9c27`
- latest_commit_brief_message: `fix(adapter): gate top public IO widths`
- note: the latest committed baseline blocks missing or parametric `.fsm` top public IO widths instead of rendering implicit 1-bit top ports

## Recent commit chain (last 6)
- `b9f9c27` fix(adapter): gate top public IO widths
- `af3962a` fix(adapter): surface width conflicts
- `b6255cf` fix(adapter): surface graph direction conflicts
- `72d47f0` fix(adapter): keep top graph conflicts sticky
- `f2618b4` fix(adapter): keep top graph directions in inventory
- `65fcf4b` fix(validate): split semantic compat direction lag

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 10` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - require `.fsm` explicit top links to resolve child endpoints against emitted child module ports, not advisory signal inventory entries
- tracker effect:
  - added a `Done` live-status row for top links requiring child endpoints to resolve to emitted child module ports
- verification status:
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo test --manifest-path Cargo.toml top_composition_blocks_link_to_unemitted_child_port -- --nocapture` failed before the fix, then passed
  - `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` passed with `26` top-composition tests
  - `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` passed with `60` adapter tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `git diff --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `504` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `504` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- rerun final lightweight formatting, docs, and whitespace checks after validation-doc updates
- commit the top-link emitted-child-port gate slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 11`, below the `25`-commit push threshold
