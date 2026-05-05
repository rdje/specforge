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
- latest_commit_hash: `5fb059e4f48473790f3aa8bc4a79ecd9058f1cb3`
- latest_commit_brief_message: `test(adapter): lock fsm graph undriven outputs`
- note: the latest committed baseline regression-locks the structured-FSM branch for graph-backed output inventory entries with no typed FSM-state driving action

## Recent commit chain (last 6)
- `5fb059e` test(adapter): lock fsm graph undriven outputs
- `a2882b4` fix(adapter): block graph-backed undriven outputs
- `706118e` docs: refresh README bootstrap continuity
- `9b425e9` test(adapter): lock mixed top child roots
- `67a4d84` test(adapter): lock reused fsm child roots
- `f2e714d` test(adapter): lock top fsm child roots

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 21` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/validate.rs`
  - `docs/book/src/quality/validation.md`

## Current in-flight slice
- objective:
  - mirror the SemanticIR graph-first compatibility-direction split at the IntentIR validation stage
  - keep `intent_compat_direction_hints_lag_graph` graph-backed only and add `intent_compat_direction_hints_incomplete` for declared signals with neither flat direction nor actor-relative graph coverage
- tracker effect:
  - added a Done row for IntentIR validation distinguishing graph-backed compatibility-direction lag from genuinely unresolved direction coverage
- verification status:
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo test --manifest-path Cargo.toml -p specforge commands::validate::tests::validate_intent_ir_keeps_incomplete_direction_finding_without_graph_coverage -- --exact --nocapture` passed
  - `cargo test --manifest-path Cargo.toml -p specforge commands::validate::tests -- --nocapture` passed with `96` validator tests
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127/127` fixtures
  - `git diff --check` passed
- current known local CI baseline:
  - `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation after this IntentIR validation split slice
  - `127/127` tracked KG fixtures after this IntentIR validation split slice

## Next exact steps
- commit the slice, truncate `git_message_brief.txt` back to `0` bytes, and confirm it remains untracked
- leave the branch unpushed; after this commit it should be `ahead 22`, below the `25`-commit push threshold
