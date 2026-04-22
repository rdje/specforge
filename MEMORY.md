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
- latest_commit_hash: `6509fb9`
- latest_commit_brief_message: `docs(memory): sync exact-cycle fixture baseline`
- note: the latest committed baseline records the continuity checkpoint immediately after the generic exact-cycle benchmark slice

## Recent commit chain (last 6)
- `6509fb9` docs(memory): sync exact-cycle fixture baseline
- `81ab3af` test(kg-bench): add exact-cycle timing fixture
- `dd61578` docs(memory): sync range-cycle fixture baseline
- `e14da0c` test(kg-bench): add range-cycle timing fixture
- `2b6d848` docs(memory): sync bounded-cycle fixture baseline
- `769d121` test(kg-bench): add bounded-cycle timing fixture

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `in sync` with `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/source.md`
- the feature slice is verified and ready to commit

## Current in-flight slice
- objective:
  - benchmark-lock the already-supported default-clock explicit next-edge family in the tracked KG corpus
- tracker effect:
  - add one `Done` row for default-clock explicit next-edge timing coverage
- current tracked KG-quality suite size in the latest committed baseline:
  - `122` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- commit the feature slice with synced tracked docs
- refresh `MEMORY.md` again so it points at the new feature commit baseline
- create the continuity commit that records that new baseline
- do not push because the branch will remain below the `25`-commit threshold
