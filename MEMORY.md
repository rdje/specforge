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
- latest_commit_hash: `26c6486376b2c43789875b091ce33591dcac56d1`
- latest_commit_brief_message: `test(adapter): lock ambiguous actor exclusion`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `26c6486` test(adapter): lock ambiguous actor exclusion
- `27787e8` test(adapter): lock unrelated actor provenance
- `63df237` test(adapter): lock unambiguous actor provenance
- `93e2044` test(adapter): lock direct output actor provenance
- `f95ec63` test(adapter): lock direct width conflict provenance
- `32f7515` test(adapter): lock explicit module width conflict provenance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 42]`
- files in flight for new batch slice 43:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `42`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - regression-lock direct-root actor-port direction-conflict provenance
  - prove blocked `fsm.signal_inventory` preserves both conflicting graph-backed actor-port support IDs and high automation confidence
  - keep same-actor contradictory direction evidence unresolved instead of guessing one side
- tracker effect:
  - live-status tracker gains `.fsm direct-root actor-port direction conflicts now regression-lock support IDs and automation confidence: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge standalone_dt_keeps_conflicting_actor_port_direction_unresolved` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `80` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed before the final `MEMORY.md` verification refresh
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148/148` tracked fixtures
  - `bash scripts/run_ci.sh` passed with formatting, warning-deny Clippy, `601` Rust tests, warning-deny Rust docs, and mdBook build
- current known local CI baseline:
  - `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation expected after this slice
  - `148/148` tracked KG fixtures

## Next exact steps
- rerun docs CI after this final `MEMORY.md` edit, run final guards, commit slice 43 without pushing, and continue slice 44
