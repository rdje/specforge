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
- latest_commit_hash: `706118effb5de8b4559b1c08c15fe683d7bd2764`
- latest_commit_brief_message: `docs: refresh README bootstrap continuity`
- note: the latest committed baseline repairs README/bootstrap continuity docs after the mixed DT/FSM child-root slice

## Recent commit chain (last 6)
- `706118e` docs: refresh README bootstrap continuity
- `9b425e9` test(adapter): lock mixed top child roots
- `67a4d84` test(adapter): lock reused fsm child roots
- `f2e714d` test(adapter): lock top fsm child roots
- `17435fd` test(adapter): lock top document root order
- `ec4f991` test(adapter): lock reused child direct roots

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 19` of `origin/main`
- modified tracked files:
  - `README.md`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`
  - `docs/book/src/commands/pipeline.md`

## Current in-flight slice
- objective:
  - block standalone `.fsm` DT/FSM output inventory entries whose output role is recovered from actor-relative graph evidence but has no typed driving action
- tracker effect:
  - added a Done row for graph-backed undriven output inventory blocking
- verification status:
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::standalone_dt_blocks_graph_backed_undriven_output_inventory -- --exact --nocapture` passed
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` passed with `75` adapter tests
  - `bash scripts/run_ci.sh` passed with `519` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
  - `git diff --check` passed
- current known local CI baseline:
  - `519` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build
  - `127/127` tracked KG fixtures

## Next exact steps
- run final docs, formatting-check, full CI, KG-bench, and whitespace validation for this `.fsm` graph-backed output slice
- commit the graph-backed undriven-output renderability slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 20`, below the `25`-commit push threshold
