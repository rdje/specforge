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
- outside explicit batch runs, do not push unless the user asks or the branch reaches `25` local commits since the last push
- active batch-run rule: for this `N=10` batch, commit after every slice but defer push until all 10 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `21c4bcf87c2bc1a5ed499a4dbd50325c003f3049`
- latest_commit_brief_message: `test(validation): preserve intent graph replay split`
- note: the latest committed baseline is batch slice 6/10; it locks concurrent IntentIR graph-direction coverage and conflict replay recommendations

## Recent commit chain (last 6)
- `21c4bcf` test(validation): preserve intent graph replay split
- `7beb265` test(validation): preserve graph replay target split
- `741267d` test(kg): lock flat-hint graph conflict boundary
- `d6751ed` fix(validate): treat graph conflicts as unresolved direction
- `3a84112` test(kg): lock mixed direction gap findings
- `b487152` docs(workflow): document batch push policy

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 30` of `origin/main`
- files in flight for slice 7:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/test_data/kg_quality/compat_direction_hints_graph_conflict_incomplete_negative/fixture.json`
  - `docs/book/src/quality/kg-bench.md`

## Active N-slice batch
- requested_count: `10`
- completed_count: `6`
- push_policy: defer push until all `10` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - tighten `compat_direction_hints_graph_conflict_incomplete_negative` so the KG fixture layer asserts the same replay split proven by the project-validation tests
  - require both graph-direction coverage rescan guidance related to `PREADY` and graph-direction conflict rescan guidance related to `graph_direction_conflict:actor_completer:PREADY` at `SemanticIR` and `IntentIR`
- tracker effect:
  - add a Done row for KG fixture coverage of conflicted compatibility-gap rescan guidance
- verification status:
  - focused `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench compat_direction_hints_graph_conflict_incomplete_negative` passed with `1` fixture and `0` failures
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `523` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 7 without pushing
