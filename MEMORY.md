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
- latest_commit_hash: `c151bdcac5ed61129be6b2960910381ed9260c18`
- latest_commit_brief_message: `fix(validate): split intent direction gap findings`
- note: the latest committed baseline mirrors the SemanticIR graph-backed compatibility-direction split at the IntentIR validation stage

## Recent commit chain (last 6)
- `c151bdc` fix(validate): split intent direction gap findings
- `5fb059e` test(adapter): lock fsm graph undriven outputs
- `a2882b4` fix(adapter): block graph-backed undriven outputs
- `706118e` docs: refresh README bootstrap continuity
- `9b425e9` test(adapter): lock mixed top child roots
- `67a4d84` test(adapter): lock reused fsm child roots

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 22` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `README.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `docs/FSMGEN_FEEDBACK.md`
  - `docs/book/src/pipeline/intentir.md`
  - `subs/fsmgen`

## Current in-flight slice
- objective:
  - update the pinned FSMGEN submodule before continuing with the next roadmap task
  - document that the new upstream `32aa318` baseline includes bounded machine-readable `.fsm` contract surfaces relevant to future adapter validation
- tracker effect:
  - added a Done row for the FSMGEN submodule now pinning a bounded machine-readable `.fsm` contract baseline for adapter planning
- verification status:
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127/127` fixtures
  - `git -C subs/fsmgen status --short --branch` clean on `main...origin/main`
  - `git submodule status --recursive` reports `32aa318f83361cd0fcfc499312ebf02ccfa0dfe8 subs/fsmgen (heads/main)`
  - `git diff --check` passed
- current known local CI baseline:
  - `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation after this IntentIR validation split slice
  - `127/127` tracked KG fixtures after this IntentIR validation split slice

## Next exact steps
- commit the FSMGEN submodule update slice, truncate `git_message_brief.txt` back to `0` bytes, and confirm it remains untracked
- then pick the next roadmap task and commit that as a separate slice before switching again
