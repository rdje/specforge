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
- latest_commit_hash: `a2882b4b29bb6f6033aad9bf80c8e988c46a4a0f`
- latest_commit_brief_message: `fix(adapter): block graph-backed undriven outputs`
- note: the latest committed baseline validates DT/FSM output roles across the full signal inventory and keeps top-linked child endpoint diagnostics composition-scoped

## Recent commit chain (last 6)
- `a2882b4` fix(adapter): block graph-backed undriven outputs
- `706118e` docs: refresh README bootstrap continuity
- `9b425e9` test(adapter): lock mixed top child roots
- `67a4d84` test(adapter): lock reused fsm child roots
- `f2e714d` test(adapter): lock top fsm child roots
- `17435fd` test(adapter): lock top document root order

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 20` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - regression-lock the structured-FSM branch for graph-backed output inventory entries whose output role is recovered from actor-relative graph evidence but has no typed FSM-state driving action
- tracker effect:
  - added a Done row for structured FSM graph-backed undriven-output blocking
- verification status:
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::structured_fsm_blocks_graph_backed_undriven_output_inventory -- --exact --nocapture` passed
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` passed with `76` adapter tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `520` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127/127` fixtures
  - `git diff --check` passed
- current known local CI baseline:
  - `520` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build after this regression-lock slice
  - `127/127` tracked KG fixtures after this regression-lock slice

## Next exact steps
- commit the structured-FSM graph-backed undriven-output regression slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 21`, below the `25`-commit push threshold
