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
- latest_commit_hash: `c9907c3f64b2ebf194fa0230583cb1a733d37958`
- latest_commit_brief_message: `test(adapter): lock same-actor width conflict support`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `c9907c3` test(adapter): lock same-actor width conflict support
- `f1ca312` test(adapter): lock flat conflict provenance
- `e841d7e` test(adapter): lock flat graph disagreement provenance
- `fdfbc91` test(adapter): lock direct direction conflict provenance
- `26c6486` test(adapter): lock ambiguous actor exclusion
- `27787e8` test(adapter): lock unrelated actor provenance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 46]`
- files in flight for new batch slice 47:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `46`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - regression-lock standalone sequential DT system-contract actor-port direction provenance
  - prove renderable `fsm.signal_inventory` preserves `graph_controller_clk` / `graph_controller_rst_n` support IDs and high automation confidence
  - keep clock/reset graph-backed direction recovery auditable when flat direct-interface direction hints lag
- tracker effect:
  - live-status tracker gains `.fsm standalone sequential system-contract actor-port direction recovery now regression-locks support IDs and automation confidence: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge standalone_sequential_dt_recovers_system_directions_from_actor_ports` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `80` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed before the final `MEMORY.md` verification refresh
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` tracked fixtures
  - `bash scripts/run_ci.sh` passed with formatting, warning-deny Clippy, `601` Rust tests, warning-deny Rust docs, and mdBook build
- current known local CI baseline:
  - `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation expected after this slice
  - `148/148` tracked KG fixtures

## Next exact steps
- rerun docs CI after this final `MEMORY.md` edit, run final guards, commit slice 47 without pushing, and continue slice 48
