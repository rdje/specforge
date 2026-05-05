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
- latest_commit_hash: `f910e116c7e569039efb549cbd4d5e414abd1275`
- latest_commit_brief_message: `test(adapter): lock canonical parametric width provenance`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `f910e11` test(adapter): lock canonical parametric width provenance
- `298911a` test(adapter): lock actor parametric width provenance
- `1456e2d` test(adapter): lock top parametric width provenance
- `01b9fe1` fix(nlp): normalize reference alias links
- `ac61835` fix(nlp): normalize alias markdown links
- `47387b1` fix(nlp): trim alias word punctuation

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 30]`
- files in flight for new batch slice 31:
  - `crates/specforge/src/ir/adapters.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `30`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - regression-lock explicit numeric width precedence over actor-port symbolic width evidence
  - prove renderable `.fsm` signal inventory preserves interface provenance while dropping stale parametric hints
  - keep the numeric-width emission path auditable
- tracker effect:
  - live-status tracker gains `.fsm numeric-over-symbolic width precedence now regression-locks interface provenance while dropping stale parametric hints: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge standalone_dt_keeps_explicit_numeric_width_over_actor_parametric_width` passed with `1` test
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests` passed with `80` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed after the live-doc refresh
  - `bash scripts/run_ci.sh` passed with `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148` fixtures and `0` failures
- current known local CI baseline:
  - `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `148/148` tracked KG fixtures

## Next exact steps
- run docs/full CI/KG bench after live-doc edits, run final guards, commit slice 31 without pushing, and continue slice 32
