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
- latest_commit_hash: `5410478`
- latest_commit_brief_message: `docs(memory): sync zero-cycle fixture baseline`
- note: the latest committed baseline records the latest continuity checkpoint after the default-clock zero-cycle slice

## Recent commit chain (last 6)
- `5410478` docs(memory): sync zero-cycle fixture baseline
- `8613510` test(kg-bench): add zero-cycle timing fixture
- `ac2eea8` docs(memory): sync default-clock clock-edge fixture baseline
- `1f30163` test(kg-bench): add default-clock clock-edge timing fixture
- `fee7c02` docs(memory): sync next-tick fixture baseline
- `d90db84` test(kg-bench): add next-tick timing fixture

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 18` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/source.md`
- the new slice is verified locally and ready for the feature commit workflow

## Current in-flight slice
- objective:
  - raise the already-landed generic next-cycle family into the tracked KG-quality benchmark corpus
- tracker effect:
  - move `KG benchmark harness now includes generic next-cycle timing coverage` to `Done`
- current tracked KG-quality suite size in the verified worktree:
  - `119` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the feature commit for the generic next-cycle benchmark fixture slice
- refresh `MEMORY.md` again for the required continuity commit
- create the continuity commit that records the feature-commit baseline
- do not push because the branch remains below the `25`-commit threshold
