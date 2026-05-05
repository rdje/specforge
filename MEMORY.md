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
- latest_commit_hash: `9b425e9250ccd0d5e3b588ef0c49205065492e49`
- latest_commit_brief_message: `test(adapter): lock mixed top child roots`
- note: the latest committed baseline locks mixed DT/FSM child-root order and emitted `?dtc`/`?fsmc` spellings in renderable top source documents

## Recent commit chain (last 6)
- `9b425e9` test(adapter): lock mixed top child roots
- `67a4d84` test(adapter): lock reused fsm child roots
- `f2e714d` test(adapter): lock top fsm child roots
- `17435fd` test(adapter): lock top document root order
- `ec4f991` test(adapter): lock reused child direct roots
- `80c81c5` test(adapter): lock renderable top direct roots

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 18` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`

## Current in-flight slice
- objective:
  - re-execute `README.md`, `SESSION_BOOTSTRAP.md`, and `COMMIT.md` for the current session and refresh continuity docs after the model/session context changed
- tracker effect:
  - no product live-status row changed; this is a continuity/bootstrap synchronization slice
- verification status:
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `518` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
  - `git diff --check` passed
- current known local CI baseline:
  - `518` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build
  - `127/127` tracked KG fixtures

## Next exact steps
- run final docs, formatting, CI, KG-bench, and whitespace validation for this continuity slice
- commit the README/COMMIT bootstrap refresh
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 19`, below the `25`-commit push threshold
