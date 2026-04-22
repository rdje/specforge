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
- latest_commit_hash: `283148e`
- latest_commit_brief_message: `test(kg-bench): add quantified-edge timing fixture`
- note: the latest committed baseline records the default-clock quantified-edge benchmark checkpoint

## Recent commit chain (last 6)
- `283148e` test(kg-bench): add quantified-edge timing fixture
- `081299b` docs(memory): sync default next-edge fixture baseline
- `ab08b25` test(kg-bench): add default next-edge timing fixture
- `6509fb9` docs(memory): sync exact-cycle fixture baseline
- `81ab3af` test(kg-bench): add exact-cycle timing fixture
- `dd61578` docs(memory): sync range-cycle fixture baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 3` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed and only the continuity refresh remains

## Current in-flight slice
- objective:
  - preserve the latest committed baseline after landing the default-clock quantified-edge benchmark slice
- tracker effect:
  - no additional tracker change is expected in this continuity-only commit
- current tracked KG-quality suite size in the latest committed baseline:
  - `124` fixtures
- verification status:
  - feature commit `283148e` is complete
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the continuity commit that records that new baseline
- do not push because the branch will remain below the `25`-commit threshold
