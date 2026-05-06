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
- active batch-run rule: for the new `N=200` batch, commit after every slice, sync live docs and mdBook at the end of every slice, and defer push until all 200 new-batch slices are complete
- previous batch-run rule used: the `N=100` batch committed each slice independently, deferred push until all 100 slices were complete, and was pushed after slice 100
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `166f3c66a244bfaedaaf514c0ab8d82e5c3ba0da`
- latest_commit_brief_message: `test(adapter): lock sibling child-width provenance`
- note: new local `N=200` batch is active; slice 12 is committed, slice 13 is in flight, and push is deferred until all 200 slices complete

## Recent commit chain (last 6)
- `166f3c6` test(adapter): lock sibling child-width provenance
- `ceeb653` test(adapter): lock top-link child-width provenance
- `3ba14b5` test(adapter): lock child-system width provenance
- `aaf4830` test(adapter): lock child-link width provenance
- `049e0f6` test(adapter): lock actor-port width provenance
- `fd12824` test(adapter): lock actor-port direction provenance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 13]`
- files in flight for new batch slice 13:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `docs/book/src/reference/generated-artifacts.md`

## Active N-slice batch
- requested_count: `200`
- completed_count: `12`
- push_policy: defer push until all `200` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice receives verification, live-doc refresh, mdBook sync, commit, message-file truncation, and post-commit checks before the next slice starts
- prior_unpushed_baseline: branch started the batch with one docs-sync commit already ahead of `origin/main`

## Current in-flight slice
- objective:
  - regression-lock source-side sibling-link child-width recovery child-signal declaration provenance
  - prove source-side sibling-link-recovered child signal width keeps original child signal declaration support IDs as well as topology-link support IDs
  - preserve existing child signal-inventory topology provenance and emitted `(output_data 8)` text
  - keep the mdBook adapter artifact description aligned with this provenance surface
- tracker effect:
  - expected live-status tracker gain: `.fsm source-side sibling child-width recovery now preserves child signal declaration support alongside topology support: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge top_composition_recovers_source_child_width_from_sibling_child_link_topology` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `80` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed after live-doc/mdBook sync
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` tracked fixtures
  - `bash scripts/run_ci.sh` passed with formatting, warning-deny Clippy, `601` Rust tests, warning-deny Rust docs, and mdBook build
- current known local CI baseline:
  - `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation expected after this slice
  - `148/148` tracked KG fixtures

## Next exact steps
- run final guards, commit slice 13 without pushing, and continue slice 14
