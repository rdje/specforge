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
- latest_commit_hash: `1867e39`
- latest_commit_brief_message: `docs(memory): sync signal-leading fixture baseline`
- note: the latest committed baseline records the previous slice where the signal-leading local-clock family entered the tracked KG-quality corpus

## Recent commit chain (last 6)
- `1867e39` docs(memory): sync signal-leading fixture baseline
- `26cc2bc` test(kg-bench): add signal-leading clock fixture
- `c55b858` docs(memory): sync signal-leading lexical baseline
- `97b4310` test(temporal): lock signal-leading lexical symmetry
- `d237f12` docs(memory): sync signal-leading clock baseline
- `5de896c` test(temporal): lock signal-leading clock symmetry

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 20` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/source.md`
- the next feature slice is in progress; no new commit has been created yet in this slice
- `LIVE_ACHIEVEMENT_STATUS.md` should gain one new benchmark-coverage row in this slice

## Current in-flight slice
- objective:
  - raise the named local cycle family into the tracked KG-quality corpus without widening the model
- planned code changes:
  - add `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/`
    - lock the already-proved `same ACLK cycle` form through both `SemanticIR` and `IntentIR`
  - update the tracked live docs to reflect the new benchmark fixture and fixture-count growth
- tracker effect:
  - one row should change to `Done`: `KG benchmark harness now includes named local cycle timing coverage`
- current tracked KG-quality suite size after validation:
  - `107` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline from the last completed slice:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked files for the feature commit
- create the feature commit, truncate `git_message_brief.txt`, and verify post-conditions
- refresh `MEMORY.md` to the new committed baseline
- create the required continuity commit and verify post-conditions again
- do not push because the branch will remain below the `25`-commit threshold
