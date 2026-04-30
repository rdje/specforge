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
- latest_commit_hash: `67a4d84be709f66fe1b99e4d54143fde34e0f9ac`
- latest_commit_brief_message: `test(adapter): lock reused fsm child roots`
- note: the latest committed baseline locks renderable source-document direct-root de-duplication for reused structured FSM child modules while preserving each `?fsmc` instance

## Recent commit chain (last 6)
- `67a4d84` test(adapter): lock reused fsm child roots
- `f2e714d` test(adapter): lock top fsm child roots
- `17435fd` test(adapter): lock top document root order
- `ec4f991` test(adapter): lock reused child direct roots
- `80c81c5` test(adapter): lock renderable top direct roots
- `68c54a0` test(adapter): lock renderable top link provenance

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
  - lock renderable top source-document order and root kinds for mixed DT/FSM child modules
- tracker effect:
  - added a renderable top source-document `Done` row for mixed DT/FSM child root order and emitted `?dtc`/`?fsmc` spellings
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::renderable_top_document_preserves_mixed_child_root_order_and_kind -- --exact --nocapture` passed
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` passed with `74` adapter tests
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `git diff --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `518` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `518` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- rerun final lightweight formatting, docs, and whitespace checks after validation-doc updates
- commit the mixed DT/FSM child root-order regression slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 18`, below the `25`-commit push threshold
