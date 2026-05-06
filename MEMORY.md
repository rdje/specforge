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
- active batch-run rule: for the new `N=100` batch, commit after every slice but defer push until all 100 slices complete; do not push at the `25`-commit threshold during the batch
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `73b0c9620ad839ec660a05af4eb047664d068263`
- latest_commit_brief_message: `test(adapter): lock child top-root confidence support`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `73b0c96` test(adapter): lock child top-root confidence support
- `57e29a6` test(adapter): lock recovered top-root confidence support
- `1074bea` test(adapter): lock transitive child width support
- `aa6d80d` test(adapter): lock child sibling source width support
- `1090d6e` test(adapter): lock child sibling target width support
- `a2d12f4` test(adapter): lock child top-link width support

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 71]`
- files in flight for new batch slice 72:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `71`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - regression-lock baseline renderable top-composition provenance
  - prove selected top candidates preserve support IDs for top ports, child declarations, and explicit topology links
  - keep the canonical producer/consumer top-composition fixture auditable while `.fsm` emission remains renderable
- tracker effect:
  - live-status tracker gains `.fsm baseline top-composition lowering now regression-locks top port, child, and link support IDs: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge builds_renderable_top_composition_fsm_adapter_artifact` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `80` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed before the final live-doc refresh
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` tracked fixtures
  - `bash scripts/run_ci.sh` passed with formatting, warning-deny Clippy, `601` Rust tests, warning-deny Rust docs, and mdBook build
- current known local CI baseline:
  - `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation expected after this slice
  - `148/148` tracked KG fixtures

## Next exact steps
- run final docs CI and guards, commit slice 72 without pushing, and continue slice 73
