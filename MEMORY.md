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
- latest_commit_hash: `7ad8e6d`
- latest_commit_brief_message: `docs(memory): sync named one-cycle lexical baseline`
- note: the latest committed baseline records the continuity refresh immediately after the named one-cycle lexical benchmark hardening checkpoint

## Recent commit chain (last 6)
- `7ad8e6d` docs(memory): sync named one-cycle lexical baseline
- `bce577b` test(kg-bench): harden named one-cycle lexical coverage
- `0175a30` docs(memory): sync default explicit-edge baseline
- `1a8ac65` test(kg-bench): harden default explicit-edge lexical coverage
- `3663e53` docs(memory): sync generic clock-edge baseline
- `1dc4960` test(kg-bench): harden generic clock-edge lexical coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 16` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/source.rs`
- `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/source.md`
- the named local zero-cycle lexical hardening slice is verified and ready for commit

## Current in-flight slice
- objective:
  - harden the named local zero-cycle benchmark family so the tracked corpus proves the supported `same` / `this` / `current` lexical lane instead of only one canonical spelling, while restoring the repo’s full `run_ci.sh` baseline after clippy-rooted verification failures surfaced
- tracker effect:
  - no live-status row change is expected in this reliability-hardening slice
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - fixture, code, and tracked-doc updates are staged in the worktree
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- commit the feature slice
- refresh `MEMORY.md` again so the latest committed baseline points at that new feature commit
- create the continuity commit
- do not push because the branch will remain below the `25`-commit threshold
