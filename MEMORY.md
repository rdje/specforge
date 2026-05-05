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
- latest_commit_hash: `d6751ed7a13526c04fc766f46799c242e7af8ffb`
- latest_commit_brief_message: `fix(validate): treat graph conflicts as unresolved direction`
- note: the latest committed baseline is batch slice 3/10; it makes same-actor graph conflicts count as unresolved graph coverage while preserving actor-aware conflict diagnostics

## Recent commit chain (last 6)
- `d6751ed` fix(validate): treat graph conflicts as unresolved direction
- `3a84112` test(kg): lock mixed direction gap findings
- `b487152` docs(workflow): document batch push policy
- `62d09a8` test(kg): lock unresolved direction gaps
- `4587325` chore(fsmgen): refresh submodule contract baseline
- `c151bdc` fix(validate): split intent direction gap findings

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 27` of `origin/main`
- files in flight for slice 4:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/test_data/kg_quality/graph_direction_same_actor_conflict_negative/fixture.json`
  - `docs/book/src/quality/kg-bench.md`

## Active N-slice batch
- requested_count: `10`
- completed_count: `3`
- push_policy: defer push until all `10` slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - tighten the existing flat-hint-present graph-conflict fixture so same-actor graph conflict coverage debt does not imply compatibility-direction debt
  - prove graph conflict plus flat hints yields graph conflict and graph-coverage findings, excludes both compatibility-direction finding families, and preserves mixed metrics
- tracker effect:
  - add a Done row for flat-hint-present graph conflicts staying outside compatibility-direction debt
- verification status:
  - focused `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench graph_direction_same_actor_conflict_negative` passed with `1/1` fixture
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `130/130` fixtures
  - `git diff --check` passed
  - checkout-specific absolute path scan across tracked markdown passed
- current known local CI baseline:
  - `521` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `130/130` tracked KG fixtures

## Next exact steps
- commit slice 4 without pushing
