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
- latest_commit_hash: `f30991f4ae1598ce7af6ff938c1addaf97b26b83`
- latest_commit_brief_message: `test(adapter): lock missing child top port`
- note: final slice of the local `N=100` batch is in flight; push is due after slice 100 is committed and post-commit checks pass

## Recent commit chain (last 6)
- `f30991f` test(adapter): lock missing child top port
- `9c70a20` test(adapter): lock undeclared target FSM states
- `1d6dc59` test(adapter): lock missing initial FSM graph
- `6a18f9d` test(adapter): lock reset FSM renderable graph
- `8d514b3` test(adapter): lock structured FSM candidates
- `8382ee5` test(adapter): lock sequential DT support

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 99]`
- files in flight for new batch slice 100:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `99`
- push_policy: defer push until all `100` new-batch slices are committed; slice 100 is the final slice and should be pushed after commit and post-commit checks
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - regression-lock renderable top-composition top-port provenance
  - prove renderable artifacts keep the declared top port direction, numeric width, high automation confidence, and renderable top root alongside child/link support IDs
  - complete the requested `N=100` batch and push after the slice 100 commit workflow finishes
- tracker effect:
  - live-status tracker gains `.fsm renderable top-composition lowering now preserves top-port shape and renderable-top provenance: Done`
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
- run final docs CI and guards, commit slice 100, run post-commit checks, then push the completed `N=100` batch
