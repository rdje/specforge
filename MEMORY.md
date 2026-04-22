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
- latest_commit_hash: `2b6d848`
- latest_commit_brief_message: `docs(memory): sync bounded-cycle fixture baseline`
- note: the latest committed baseline records the latest continuity checkpoint after the generic bounded-cycle slice

## Recent commit chain (last 6)
- `2b6d848` docs(memory): sync bounded-cycle fixture baseline
- `769d121` test(kg-bench): add bounded-cycle timing fixture
- `5316b79` docs(memory): sync generic next-cycle fixture baseline
- `575b174` test(kg-bench): add generic next-cycle timing fixture
- `5410478` docs(memory): sync zero-cycle fixture baseline
- `8613510` test(kg-bench): add zero-cycle timing fixture

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 22` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/test_data/kg_quality/generic_range_cycle_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/generic_range_cycle_timing_gold/source.md`
- the new slice is verified locally and ready for the feature commit workflow

## Current in-flight slice
- objective:
  - raise the already-landed generic range and one-sided cycle family into the tracked KG-quality benchmark corpus
- tracker effect:
  - move `KG benchmark harness now includes generic range cycle timing coverage` to `Done`
- current tracked KG-quality suite size in the verified worktree:
  - `121` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the feature commit for the generic range-cycle benchmark fixture slice
- refresh `MEMORY.md` again for the required continuity commit
- create the continuity commit that records the feature-commit baseline
- do not push because the branch remains below the `25`-commit threshold
