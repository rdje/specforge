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
- latest_commit_hash: `45873250c7ffb387f1418b304f3fe4ad42713c0b`
- latest_commit_brief_message: `chore(fsmgen): refresh submodule contract baseline`
- note: the latest committed baseline fast-forwards `subs/fsmgen` to upstream `32aa318` and documents its bounded machine-readable `.fsm` contract surfaces

## Recent commit chain (last 6)
- `4587325` chore(fsmgen): refresh submodule contract baseline
- `c151bdc` fix(validate): split intent direction gap findings
- `5fb059e` test(adapter): lock fsm graph undriven outputs
- `a2882b4` fix(adapter): block graph-backed undriven outputs
- `706118e` docs: refresh README bootstrap continuity
- `9b425e9` test(adapter): lock mixed top child roots

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 23` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `README.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/test_data/kg_quality/compat_direction_hints_incomplete_negative/fixture.json`
  - `crates/specforge/test_data/kg_quality/compat_direction_hints_incomplete_negative/source.md`
  - `docs/book/src/quality/kg-bench.md`

## Current in-flight slice
- objective:
  - add a tracked KG fixture for the unresolved compatibility-direction state where a declared signal has no flat direction hint and no actor-relative graph direction
  - prove SemanticIR and IntentIR emit `*_compat_direction_hints_incomplete` rather than the graph-backed lag finding ids
- tracker effect:
  - added a Done row for KG benchmark coverage of unresolved compatibility-direction gaps separately from graph-backed lag
- verification status:
  - focused `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench compat_direction_hints_incomplete_negative` passed with `1/1` fixture
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `128/128` fixtures
  - `git diff --check` passed
- current known local CI baseline:
  - `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation after this IntentIR validation split slice
  - `128/128` tracked KG fixtures after this unresolved compatibility-direction fixture slice

## Next exact steps
- commit the KG fixture slice, truncate `git_message_brief.txt` back to `0` bytes, and confirm it remains untracked
- leave branch unpushed unless the next slice reaches the 25-commit threshold
