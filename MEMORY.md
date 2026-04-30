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
- latest_commit_hash: `57ac516fcea2eaa295acbe8b29a9df05df0c0ca1`
- latest_commit_brief_message: `test(adapter): lock resolved system-port widths`
- note: the latest committed baseline locks child system-contract recovered top-boundary width support IDs and automation confidence at the resolved top-candidate port surface

## Recent commit chain (last 6)
- `57ac516` test(adapter): lock resolved system-port widths
- `8beb670` test(adapter): lock selected system-port provenance
- `ade21cc` test(adapter): lock system-port renderable widths
- `661f019` test(adapter): lock actor-port renderable width
- `b9bd0a9` test(adapter): lock top-link renderable width
- `fb4f050` test(adapter): lock top-link renderable provenance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 4` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - lock child module system-contract endpoint support IDs and automation confidence before top-link width recovery consumes them
- tracker effect:
  - added a child module system-contract endpoint `Done` row for support/confidence preservation before top-link width recovery
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_system_port_widths_from_child_system_contract -- --exact --nocapture` passed
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` passed with `67` adapter tests
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `git diff --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `511` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- rerun final lightweight formatting, docs, and whitespace checks after validation-doc updates
- commit the child system-contract endpoint provenance regression slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 5`, below the `25`-commit push threshold
