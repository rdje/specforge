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
- latest_commit_hash: `9dbcaef`
- latest_commit_brief_message: `test(kg-bench): add plural edge-of-clock timing fixture`
- note: the latest committed baseline records the slice where plural edge-of-clock timing entered the tracked KG-quality corpus

## Recent commit chain (last 6)
- `9dbcaef` test(kg-bench): add plural edge-of-clock timing fixture
- `2767149` test(kg-bench): add unit-first diagram timing fixture
- `2fd726a` docs(memory): sync named diagram-edge fixture baseline
- `4bd9a16` test(kg-bench): add named diagram-edge timing fixture
- `b40ccbb` docs(memory): sync named quantified-edge fixture baseline
- `6f467e7` test(kg-bench): add named quantified-edge timing fixture

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 5` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the required continuity refresh remains before the repo returns to a clean post-slice state

## Current in-flight slice
- objective:
  - complete the mandatory post-feature continuity refresh for the plural edge-of-clock benchmark slice
- tracker effect:
  - no further tracker change is expected in the continuity commit; the feature commit already moved `KG benchmark harness now includes plural \`edge(s) of <clock>\` timing coverage` to `Done`
- current tracked KG-quality suite size after validation:
  - `112` fixtures
- verification status:
  - feature commit `9dbcaef` already landed cleanly
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- create the required continuity commit, truncate `git_message_brief.txt`, and verify post-conditions again
- do not push because the branch remains below the `25`-commit threshold
