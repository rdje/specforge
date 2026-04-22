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
- latest_commit_hash: `bce577b`
- latest_commit_brief_message: `test(kg-bench): harden named one-cycle lexical coverage`
- note: the latest committed baseline records the named one-cycle lexical benchmark hardening checkpoint

## Recent commit chain (last 6)
- `bce577b` test(kg-bench): harden named one-cycle lexical coverage
- `0175a30` docs(memory): sync default explicit-edge baseline
- `1a8ac65` test(kg-bench): harden default explicit-edge lexical coverage
- `3663e53` docs(memory): sync generic clock-edge baseline
- `1dc4960` test(kg-bench): harden generic clock-edge lexical coverage
- `a7fdb93` test(kg-bench): harden shorthand next-edge lexical coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 15` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed and only the continuity refresh remains

## Current in-flight slice
- objective:
  - preserve the latest committed baseline after landing the named one-cycle lexical benchmark hardening slice
- tracker effect:
  - no live-status row change is expected in this continuity-only commit
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - feature commit `bce577b` is complete
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the continuity commit
- do not push because the branch will remain below the `25`-commit threshold
