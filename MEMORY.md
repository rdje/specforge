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
- latest_commit_hash: `575b174`
- latest_commit_brief_message: `test(kg-bench): add generic next-cycle timing fixture`
- note: the latest committed baseline records the latest benchmark checkpoint after the generic next-cycle slice

## Recent commit chain (last 6)
- `575b174` test(kg-bench): add generic next-cycle timing fixture
- `5410478` docs(memory): sync zero-cycle fixture baseline
- `8613510` test(kg-bench): add zero-cycle timing fixture
- `ac2eea8` docs(memory): sync default-clock clock-edge fixture baseline
- `1f30163` test(kg-bench): add default-clock clock-edge timing fixture
- `fee7c02` docs(memory): sync next-tick fixture baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 19` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed and only the continuity refresh remains before the worktree returns to clean

## Current in-flight slice
- objective:
  - preserve the latest committed baseline after landing the generic next-cycle benchmark slice
- tracker effect:
  - no additional tracker change is expected in this continuity-only commit
- current tracked KG-quality suite size in the latest committed baseline:
  - `119` fixtures
- verification status:
  - feature commit `575b174` is complete
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the continuity commit that records the feature-commit baseline
- verify that `git_message_brief.txt` is truncated back to `0` bytes and remains untracked
- verify that the worktree is clean after the continuity commit
- do not push because the branch remains below the `25`-commit threshold
