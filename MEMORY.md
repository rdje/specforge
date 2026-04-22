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
- latest_commit_hash: `dc62db6`
- latest_commit_brief_message: `test(kg-bench): add later-phrase timing fixture`
- note: the latest committed baseline records the slice where explicit later-phrase timing entered the tracked KG-quality corpus

## Recent commit chain (last 6)
- `dc62db6` test(kg-bench): add later-phrase timing fixture
- `55df3a3` docs(memory): sync shorthand next-edge fixture baseline
- `d60d1e3` test(kg-bench): add shorthand next-edge timing fixture
- `2804882` docs(memory): sync tick-unit fixture baseline
- `08f3858` test(kg-bench): add tick-unit timing fixture
- `7efb8ce` docs(memory): sync plural edge-of-clock fixture baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 11` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the required continuity refresh remains before the repo returns to a clean post-slice state

## Current in-flight slice
- objective:
  - complete the mandatory post-feature continuity refresh for the later-phrase benchmark slice
- tracker effect:
  - no further tracker change is expected in the continuity commit; the feature commit already moved `KG benchmark harness now includes explicit later-phrase timing coverage` to `Done`
- current tracked KG-quality suite size after validation:
  - `115` fixtures
- verification status:
  - feature commit `dc62db6` already landed cleanly
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
