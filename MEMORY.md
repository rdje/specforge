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
- latest_commit_hash: `ac618356fa8ae9be1c0f4c8d65f9517fddee56a8`
- latest_commit_brief_message: `fix(nlp): normalize alias markdown links`
- note: new local `N=100` batch is active; push remains deferred until all 100 slices complete

## Recent commit chain (last 6)
- `ac61835` fix(nlp): normalize alias markdown links
- `47387b1` fix(nlp): trim alias word punctuation
- `1cafcf9` fix(nlp): reject outline alias markers
- `9c572b2` docs(book): explain adapter graph provenance
- `7e0add5` fix(adapter): preserve top flat graph direction evidence
- `afb2376` test(adapter): lock system contract flat graph disagreement

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `main...origin/main [ahead 26]`
- files in flight for new batch slice 27:
  - `crates/specforge/src/commands/nlp_enrich.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`

## Active N-slice batch
- requested_count: `100`
- completed_count: `26`
- push_policy: defer push until all `100` new-batch slices are committed; do not push at the `25`-commit threshold during this batch
- slice_rule: each slice still receives its own verification, live-doc refresh, commit, message-file truncation, and post-commit checks before the next slice starts

## Current in-flight slice
- objective:
  - harden Form 2 NLP alias learning against reference-style markdown link id pollution
  - normalize reference-style markdown links in subject phrases to their visible labels before storing aliases
  - keep useful implicit noun-phrase alias learning intact
- tracker effect:
  - live-status tracker gains `NLP alias learning now uses reference-style markdown link labels for Form 2 aliases: Done`
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge extract_alias_phrase` passed with `9` tests
  - `cargo test --manifest-path Cargo.toml -p specforge commands::nlp_enrich::tests` passed with `21` tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `bash scripts/run_docs_ci.sh` passed after the live-doc refresh
  - `bash scripts/run_ci.sh` passed with `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `148` fixtures and `0` failures
- current known local CI baseline:
  - `601` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `148/148` tracked KG fixtures

## Next exact steps
- run docs/full CI/KG bench after live-doc edits, run final guards, commit slice 27 without pushing, and continue slice 28
